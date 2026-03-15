use serde_json::Map;

use crate::cli::{CommentCreateArgs, CommentsCommand, CommentsSubcommand};
use crate::commands::common::{
    execute_and_print, insert_i64, insert_string, merge_body_with_fields, parse_body_argument,
    require_body,
};
use crate::config::Config;
use crate::error::CliError;
use crate::operations::{CREATE_COMMENT, DELETE_COMMENT, GET_COMMENTS};

pub async fn run(config: &Config, args: CommentsCommand) -> Result<(), CliError> {
    match args.command {
        CommentsSubcommand::List { expense_id } => {
            execute_and_print(
                config,
                GET_COMMENTS,
                &[],
                &[("expense_id", expense_id.to_string())],
                None,
            )
            .await
        }
        CommentsSubcommand::Create { fields, body } => {
            let body = require_body(
                merge_body_with_fields(
                    parse_body_argument(body.as_deref())?,
                    build_create_fields(fields),
                )?,
                "comments create",
            )?;
            execute_and_print(config, CREATE_COMMENT, &[], &[], Some(body)).await
        }
        CommentsSubcommand::Delete { id } => {
            execute_and_print(config, DELETE_COMMENT, &[("id", id.to_string())], &[], None).await
        }
    }
}

fn build_create_fields(fields: CommentCreateArgs) -> Map<String, serde_json::Value> {
    let mut body = Map::new();
    insert_i64(&mut body, "expense_id", fields.expense_id);
    insert_string(&mut body, "content", fields.content);
    body
}
