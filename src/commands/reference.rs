use crate::cli::{CategoriesCommand, CurrenciesCommand, ReferenceSubcommand};
use crate::commands::common::execute_and_print;
use crate::config::Config;
use crate::error::CliError;
use crate::operations::{GET_CATEGORIES, GET_CURRENCIES};

pub async fn run_categories(config: &Config, args: CategoriesCommand) -> Result<(), CliError> {
    match args.command {
        ReferenceSubcommand::List => {
            execute_and_print(config, GET_CATEGORIES, &[], &[], None).await
        }
    }
}

pub async fn run_currencies(config: &Config, args: CurrenciesCommand) -> Result<(), CliError> {
    match args.command {
        ReferenceSubcommand::List => {
            execute_and_print(config, GET_CURRENCIES, &[], &[], None).await
        }
    }
}
