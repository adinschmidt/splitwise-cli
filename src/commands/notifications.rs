use crate::cli::{NotificationsCommand, NotificationsSubcommand};
use crate::commands::common::execute_and_print;
use crate::config::Config;
use crate::error::CliError;
use crate::operations::GET_NOTIFICATIONS;

pub async fn run(config: &Config, args: NotificationsCommand) -> Result<(), CliError> {
    match args.command {
        NotificationsSubcommand::List {
            limit,
            updated_after,
        } => {
            let mut query = Vec::new();
            if let Some(value) = limit {
                query.push(("limit", value.to_string()));
            }
            if let Some(value) = updated_after {
                query.push(("updated_after", value));
            }
            execute_and_print(config, GET_NOTIFICATIONS, &[], &query, None).await
        }
    }
}
