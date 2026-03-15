pub mod auth;
pub mod cli;
pub mod client;
pub mod commands;
pub mod config;
pub mod error;
pub mod operations;
pub mod output;

use cli::{Cli, CommandGroup};
use config::Config;
use error::CliError;

pub async fn dispatch(cli: Cli, config: Config) -> Result<(), CliError> {
    match cli.command {
        CommandGroup::Users(command) => commands::users::run(&config, command).await,
        CommandGroup::Groups(command) => commands::groups::run(&config, command).await,
        CommandGroup::Friends(command) => commands::friends::run(&config, command).await,
        CommandGroup::Expenses(command) => commands::expenses::run(&config, command).await,
        CommandGroup::Comments(command) => commands::comments::run(&config, command).await,
        CommandGroup::Notifications(command) => {
            commands::notifications::run(&config, command).await
        }
        CommandGroup::Categories(command) => {
            commands::reference::run_categories(&config, command).await
        }
        CommandGroup::Currencies(command) => {
            commands::reference::run_currencies(&config, command).await
        }
    }
}

pub fn run(
    cli: Cli,
    config: Config,
) -> Result<impl std::future::Future<Output = Result<(), CliError>>, CliError> {
    Ok(dispatch(cli, config))
}
