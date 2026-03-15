use serde_json::{Map, Value};

use crate::cli::{FriendCreateArgs, FriendsCommand, FriendsSubcommand};
use crate::commands::common::{
    execute_and_print, insert_string, merge_body_with_fields, parse_body_argument,
    parse_key_value_object, require_body,
};
use crate::config::Config;
use crate::error::CliError;
use crate::operations::{CREATE_FRIEND, CREATE_FRIENDS, DELETE_FRIEND, GET_FRIEND, GET_FRIENDS};

pub async fn run(config: &Config, args: FriendsCommand) -> Result<(), CliError> {
    match args.command {
        FriendsSubcommand::List => execute_and_print(config, GET_FRIENDS, &[], &[], None).await,
        FriendsSubcommand::Get { id } => {
            execute_and_print(config, GET_FRIEND, &[("id", id.to_string())], &[], None).await
        }
        FriendsSubcommand::Create { fields, body } => {
            let body = require_body(
                merge_body_with_fields(
                    parse_body_argument(body.as_deref())?,
                    build_create_fields(fields),
                )?,
                "friends create",
            )?;
            execute_and_print(config, CREATE_FRIEND, &[], &[], Some(body)).await
        }
        FriendsSubcommand::CreateMany { friends, body } => {
            let body = require_body(
                merge_body_with_fields(
                    parse_body_argument(body.as_deref())?,
                    build_create_many_fields(friends)?,
                )?,
                "friends create-many",
            )?;
            execute_and_print(config, CREATE_FRIENDS, &[], &[], Some(body)).await
        }
        FriendsSubcommand::Delete { id } => {
            execute_and_print(config, DELETE_FRIEND, &[("id", id.to_string())], &[], None).await
        }
    }
}

fn build_create_fields(fields: FriendCreateArgs) -> Map<String, Value> {
    let mut body = Map::new();
    insert_string(&mut body, "user_email", fields.email);
    insert_string(&mut body, "user_first_name", fields.first_name);
    insert_string(&mut body, "user_last_name", fields.last_name);
    body
}

fn build_create_many_fields(friends: Vec<String>) -> Result<Map<String, Value>, CliError> {
    let mut body = Map::new();

    for (index, friend) in friends.into_iter().enumerate() {
        let parsed = parse_key_value_object(&friend)?;
        for (key, value) in parsed {
            body.insert(format!("users__{index}__{key}"), value);
        }
    }

    Ok(body)
}
