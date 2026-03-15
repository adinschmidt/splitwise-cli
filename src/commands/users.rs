use serde_json::Map;

use crate::cli::{UserUpdateArgs, UsersCommand, UsersSubcommand};
use crate::commands::common::{
    execute_and_print, insert_string, merge_body_with_fields, parse_body_argument, require_body,
};
use crate::config::Config;
use crate::error::CliError;
use crate::operations::{GET_CURRENT_USER, GET_USER, UPDATE_USER};

pub async fn run(config: &Config, args: UsersCommand) -> Result<(), CliError> {
    match args.command {
        UsersSubcommand::Me => execute_and_print(config, GET_CURRENT_USER, &[], &[], None).await,
        UsersSubcommand::Get { id } => {
            execute_and_print(config, GET_USER, &[("id", id.to_string())], &[], None).await
        }
        UsersSubcommand::Update { id, fields, body } => {
            let body = require_body(
                merge_body_with_fields(
                    parse_body_argument(body.as_deref())?,
                    build_update_fields(fields),
                )?,
                "users update",
            )?;
            execute_and_print(
                config,
                UPDATE_USER,
                &[("id", id.to_string())],
                &[],
                Some(body),
            )
            .await
        }
    }
}

fn build_update_fields(fields: UserUpdateArgs) -> Map<String, serde_json::Value> {
    let mut body = Map::new();
    insert_string(&mut body, "first_name", fields.first_name);
    insert_string(&mut body, "last_name", fields.last_name);
    insert_string(&mut body, "email", fields.email);
    insert_string(&mut body, "password", fields.password);
    insert_string(&mut body, "locale", fields.locale);
    insert_string(&mut body, "default_currency", fields.default_currency);
    body
}
