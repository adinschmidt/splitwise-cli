#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error(
        "missing Splitwise token; set --token or one of SPLITWISE_API_KEY, SPLITWISE_ACCESS_TOKEN, SPLITWISE_OAUTH_ACCESS_TOKEN, SPLITWISE_BEARER_TOKEN"
    )]
    MissingToken,

    #[error("config error: {0}")]
    Config(String),

    #[error("input error: {0}")]
    Input(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    #[error("request failed: {0}")]
    Request(String),

    #[error("Splitwise reported a semantic failure: {0}")]
    SemanticFailure(String),
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::SemanticFailure(_) => 2,
            _ => 1,
        }
    }
}
