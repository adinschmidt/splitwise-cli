use directories::ProjectDirs;
use std::path::{Path, PathBuf};

use crate::cli::{Cli, OutputFormat};
use crate::error::CliError;

const DEFAULT_BASE_URL: &str = "https://secure.splitwise.com/api/v3.0";
const TOKEN_ENV_KEYS: [&str; 4] = [
    "SPLITWISE_API_KEY",
    "SPLITWISE_ACCESS_TOKEN",
    "SPLITWISE_OAUTH_ACCESS_TOKEN",
    "SPLITWISE_BEARER_TOKEN",
];

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub token: String,
    pub output: OutputFormat,
    pub verbose: bool,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
struct FileConfig {
    pub base_url: Option<String>,
    pub token: Option<String>,
    pub output: Option<String>,
}

impl Config {
    pub fn from_cli(cli: &Cli) -> Result<Self, CliError> {
        let env = EnvConfig {
            base_url: std::env::var("SPLITWISE_BASE_URL").ok(),
            token: load_env_token(),
        };
        let file_config = load_file_config(cli.config.as_deref())?.unwrap_or_default();

        Self::resolve(cli, env, file_config)
    }

    fn resolve(cli: &Cli, env: EnvConfig, file_config: FileConfig) -> Result<Self, CliError> {
        let output = if cli.json {
            OutputFormat::Json
        } else if cli.yaml {
            OutputFormat::Yaml
        } else if let Some(output) = cli.output {
            output
        } else if let Some(file_output) = file_config.output.as_deref() {
            parse_output_format(file_output)?
        } else {
            OutputFormat::Table
        };

        let base_url = cli
            .base_url
            .clone()
            .or(env.base_url)
            .or(file_config.base_url)
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());

        let token = cli
            .token
            .clone()
            .or(env.token)
            .or(file_config.token)
            .ok_or(CliError::MissingToken)?;

        Ok(Self {
            base_url,
            token,
            output,
            verbose: cli.verbose,
        })
    }
}

#[derive(Debug, Clone, Default)]
struct EnvConfig {
    base_url: Option<String>,
    token: Option<String>,
}

fn load_env_token() -> Option<String> {
    TOKEN_ENV_KEYS
        .into_iter()
        .find_map(|key| {
            std::env::var(key)
                .ok()
                .map(|value| value.trim().to_string())
        })
        .filter(|value| !value.is_empty())
}

fn load_file_config(path_override: Option<&Path>) -> Result<Option<FileConfig>, CliError> {
    let Some(path) = resolve_config_path(path_override)? else {
        return Ok(None);
    };

    if !path.exists() {
        return Ok(None);
    }

    let contents = std::fs::read_to_string(path)?;
    let parsed = toml::from_str::<FileConfig>(&contents)?;
    Ok(Some(parsed))
}

fn resolve_config_path(path_override: Option<&Path>) -> Result<Option<PathBuf>, CliError> {
    if let Some(path) = path_override {
        return Ok(Some(path.to_path_buf()));
    }

    let Some(project_dirs) = ProjectDirs::from("", "", "splitwise") else {
        return Ok(None);
    };

    Ok(Some(project_dirs.config_dir().join("config.toml")))
}

fn parse_output_format(value: &str) -> Result<OutputFormat, CliError> {
    match value {
        "table" => Ok(OutputFormat::Table),
        "json" => Ok(OutputFormat::Json),
        "yaml" => Ok(OutputFormat::Yaml),
        _ => Err(CliError::Config(format!(
            "invalid output format in config file: {value}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, EnvConfig, FileConfig};
    use crate::cli::{Cli, CommandGroup, OutputFormat, ReferenceSubcommand};

    fn base_cli() -> Cli {
        Cli {
            output: None,
            json: false,
            yaml: false,
            base_url: None,
            token: None,
            config: None,
            verbose: false,
            command: CommandGroup::Categories(crate::cli::CategoriesCommand {
                command: ReferenceSubcommand::List,
            }),
        }
    }

    #[test]
    fn config_precedence_is_cli_then_env_then_file() {
        let mut cli = base_cli();
        cli.output = Some(OutputFormat::Yaml);
        cli.base_url = Some("https://cli.example".to_string());
        cli.token = Some("cli-token".to_string());

        let config = Config::resolve(
            &cli,
            EnvConfig {
                base_url: Some("https://env.example".to_string()),
                token: Some("env-token".to_string()),
            },
            FileConfig {
                base_url: Some("https://file.example".to_string()),
                token: Some("file-token".to_string()),
                output: Some("json".to_string()),
            },
        )
        .expect("config should resolve");

        assert_eq!(config.base_url, "https://cli.example");
        assert_eq!(config.token, "cli-token");
        assert_eq!(config.output, OutputFormat::Yaml);
    }

    #[test]
    fn config_uses_env_before_file() {
        let cli = base_cli();

        let config = Config::resolve(
            &cli,
            EnvConfig {
                base_url: Some("https://env.example".to_string()),
                token: Some("env-token".to_string()),
            },
            FileConfig {
                base_url: Some("https://file.example".to_string()),
                token: Some("file-token".to_string()),
                output: Some("yaml".to_string()),
            },
        )
        .expect("config should resolve");

        assert_eq!(config.base_url, "https://env.example");
        assert_eq!(config.token, "env-token");
        assert_eq!(config.output, OutputFormat::Yaml);
    }

    #[test]
    fn json_flag_overrides_other_output_sources() {
        let mut cli = base_cli();
        cli.json = true;
        cli.output = Some(OutputFormat::Yaml);

        let config = Config::resolve(
            &cli,
            EnvConfig::default(),
            FileConfig {
                base_url: Some("https://file.example".to_string()),
                token: Some("file-token".to_string()),
                output: Some("table".to_string()),
            },
        )
        .expect("config should resolve");

        assert_eq!(config.output, OutputFormat::Json);
    }
}
