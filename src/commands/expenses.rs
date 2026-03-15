use serde_json::{Map, Value};

use crate::cli::{ExpenseWriteArgs, ExpensesCommand, ExpensesSubcommand};
use crate::commands::common::{
    execute_and_print, insert_array, insert_bool_if_true, insert_i64, insert_string,
    merge_body_with_fields, parse_body_argument, parse_key_value_object, require_body,
};
use crate::config::Config;
use crate::error::CliError;
use crate::operations::{
    CREATE_EXPENSE, DELETE_EXPENSE, GET_EXPENSE, GET_EXPENSES, UNDELETE_EXPENSE, UPDATE_EXPENSE,
};

pub async fn run(config: &Config, args: ExpensesCommand) -> Result<(), CliError> {
    match args.command {
        ExpensesSubcommand::List {
            group_id,
            friend_id,
            dated_after,
            dated_before,
            updated_after,
            updated_before,
            limit,
            offset,
        } => {
            let mut query = Vec::new();
            if let Some(value) = group_id {
                query.push(("group_id", value.to_string()));
            }
            if let Some(value) = friend_id {
                query.push(("friend_id", value.to_string()));
            }
            if let Some(value) = dated_after {
                query.push(("dated_after", value));
            }
            if let Some(value) = dated_before {
                query.push(("dated_before", value));
            }
            if let Some(value) = updated_after {
                query.push(("updated_after", value));
            }
            if let Some(value) = updated_before {
                query.push(("updated_before", value));
            }
            if let Some(value) = limit {
                query.push(("limit", value.to_string()));
            }
            if let Some(value) = offset {
                query.push(("offset", value.to_string()));
            }
            execute_and_print(config, GET_EXPENSES, &[], &query, None).await
        }
        ExpensesSubcommand::Get { id } => {
            execute_and_print(config, GET_EXPENSE, &[("id", id.to_string())], &[], None).await
        }
        ExpensesSubcommand::Create { fields, body } => {
            let body = require_body(
                merge_body_with_fields(
                    parse_body_argument(body.as_deref())?,
                    build_write_fields(fields)?,
                )?,
                "expenses create",
            )?;
            execute_and_print(config, CREATE_EXPENSE, &[], &[], Some(body)).await
        }
        ExpensesSubcommand::Update { id, fields, body } => {
            let body = require_body(
                merge_body_with_fields(
                    parse_body_argument(body.as_deref())?,
                    build_write_fields(fields)?,
                )?,
                "expenses update",
            )?;
            execute_and_print(
                config,
                UPDATE_EXPENSE,
                &[("id", id.to_string())],
                &[],
                Some(body),
            )
            .await
        }
        ExpensesSubcommand::Delete { id } => {
            execute_and_print(config, DELETE_EXPENSE, &[("id", id.to_string())], &[], None).await
        }
        ExpensesSubcommand::Undelete { id } => {
            execute_and_print(
                config,
                UNDELETE_EXPENSE,
                &[("id", id.to_string())],
                &[],
                None,
            )
            .await
        }
    }
}

fn build_write_fields(fields: ExpenseWriteArgs) -> Result<Map<String, Value>, CliError> {
    let mut body = Map::new();
    insert_string(&mut body, "cost", fields.cost);
    insert_string(&mut body, "description", fields.description);
    insert_string(&mut body, "details", fields.details);
    insert_i64(&mut body, "group_id", fields.group_id);
    insert_string(&mut body, "currency_code", fields.currency_code);
    insert_string(&mut body, "date", fields.date);
    insert_i64(&mut body, "category_id", fields.category_id);
    insert_bool_if_true(&mut body, "payment", fields.payment);

    let shares = fields
        .shares
        .into_iter()
        .map(|share| parse_key_value_object(&share).map(Value::Object))
        .collect::<Result<Vec<_>, _>>()?;
    insert_array(&mut body, "users", shares);

    Ok(body)
}
