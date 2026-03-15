use serde_json::{Map, Value};

use crate::cli::{
    GroupAddUserArgs, GroupCreateArgs, GroupRemoveUserArgs, GroupsCommand, GroupsSubcommand,
};
use crate::commands::common::{
    execute_and_print, insert_bool_if_true, insert_i64, insert_string, merge_body_with_fields,
    parse_body_argument, parse_key_value_object, require_body,
};
use crate::config::Config;
use crate::error::CliError;
use crate::operations::{
    ADD_USER_TO_GROUP, CREATE_GROUP, DELETE_GROUP, GET_GROUP, GET_GROUPS, REMOVE_USER_FROM_GROUP,
    UNDELETE_GROUP,
};

pub async fn run(config: &Config, args: GroupsCommand) -> Result<(), CliError> {
    match args.command {
        GroupsSubcommand::List => execute_and_print(config, GET_GROUPS, &[], &[], None).await,
        GroupsSubcommand::Get { id } => {
            execute_and_print(config, GET_GROUP, &[("id", id.to_string())], &[], None).await
        }
        GroupsSubcommand::Create { fields, body } => {
            let body = require_body(
                merge_body_with_fields(
                    parse_body_argument(body.as_deref())?,
                    build_create_fields(fields)?,
                )?,
                "groups create",
            )?;
            execute_and_print(config, CREATE_GROUP, &[], &[], Some(body)).await
        }
        GroupsSubcommand::Delete { id } => {
            execute_and_print(config, DELETE_GROUP, &[("id", id.to_string())], &[], None).await
        }
        GroupsSubcommand::Undelete { id } => {
            execute_and_print(config, UNDELETE_GROUP, &[("id", id.to_string())], &[], None).await
        }
        GroupsSubcommand::AddUser { fields, body } => {
            let body = require_body(
                merge_body_with_fields(
                    parse_body_argument(body.as_deref())?,
                    build_add_user_fields(fields),
                )?,
                "groups add-user",
            )?;
            execute_and_print(config, ADD_USER_TO_GROUP, &[], &[], Some(body)).await
        }
        GroupsSubcommand::RemoveUser { fields, body } => {
            let body = require_body(
                merge_body_with_fields(
                    parse_body_argument(body.as_deref())?,
                    build_remove_user_fields(fields),
                )?,
                "groups remove-user",
            )?;
            execute_and_print(config, REMOVE_USER_FROM_GROUP, &[], &[], Some(body)).await
        }
    }
}

fn build_create_fields(fields: GroupCreateArgs) -> Result<Map<String, Value>, CliError> {
    let mut body = Map::new();
    insert_string(&mut body, "name", fields.name);
    insert_string(
        &mut body,
        "group_type",
        fields
            .group_type
            .map(|value| value.as_api_value().to_string()),
    );
    insert_bool_if_true(&mut body, "simplify_by_default", fields.simplify_by_default);

    for (index, member) in fields.members.into_iter().enumerate() {
        let parsed = parse_key_value_object(&member)?;
        for (key, value) in parsed {
            let key = if key == "user_id" {
                "id".to_string()
            } else {
                key
            };
            body.insert(format!("users__{index}__{key}"), value);
        }
    }

    Ok(body)
}

fn build_add_user_fields(fields: GroupAddUserArgs) -> Map<String, Value> {
    let mut body = Map::new();
    insert_i64(&mut body, "group_id", fields.group_id);
    insert_i64(&mut body, "user_id", fields.user_id);
    insert_string(&mut body, "first_name", fields.first_name);
    insert_string(&mut body, "last_name", fields.last_name);
    insert_string(&mut body, "email", fields.email);
    body
}

fn build_remove_user_fields(fields: GroupRemoveUserArgs) -> Map<String, Value> {
    let mut body = Map::new();
    insert_i64(&mut body, "group_id", fields.group_id);
    insert_i64(&mut body, "user_id", fields.user_id);
    body
}
