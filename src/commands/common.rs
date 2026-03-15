use std::io::Read;

use serde_json::{Map, Number, Value};

use crate::client::SplitwiseClient;
use crate::config::Config;
use crate::error::CliError;
use crate::operations::OperationSpec;
use crate::output;

pub async fn execute_and_print(
    config: &Config,
    operation: OperationSpec,
    path_params: &[(&str, String)],
    query_params: &[(&str, String)],
    body: Option<Value>,
) -> Result<(), CliError> {
    let client = SplitwiseClient::new(config.clone())?;
    let value = client
        .execute(operation, path_params, query_params, body)
        .await?;
    output::print(&value, config.output)
}

pub fn parse_body_argument(raw_body: Option<&str>) -> Result<Option<Value>, CliError> {
    raw_body.map(parse_body_value).transpose()
}

pub fn parse_body_value(raw_body: &str) -> Result<Value, CliError> {
    let raw = if let Some(path) = raw_body.strip_prefix('@') {
        if path == "-" {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            buffer
        } else {
            std::fs::read_to_string(path)?
        }
    } else {
        raw_body.to_string()
    };

    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(CliError::Input(
            "request body cannot be empty; pass JSON or @file.json".to_string(),
        ));
    }

    let parsed = serde_json::from_str::<Value>(trimmed).map_err(|error| {
        CliError::Input(format!(
            "invalid JSON body; pass inline JSON or @file.json: {error}"
        ))
    })?;

    if !parsed.is_object() {
        return Err(CliError::Input(
            "request body must decode to a JSON object".to_string(),
        ));
    }

    Ok(parsed)
}

pub fn merge_body_with_fields(
    body: Option<Value>,
    fields: Map<String, Value>,
) -> Result<Option<Value>, CliError> {
    if body.is_none() && fields.is_empty() {
        return Ok(None);
    }

    let mut merged = match body {
        Some(Value::Object(map)) => map,
        Some(_) => {
            return Err(CliError::Input(
                "request body must be a JSON object to merge with typed flags".to_string(),
            ));
        }
        None => Map::new(),
    };

    merged.extend(fields);
    Ok(Some(Value::Object(merged)))
}

pub fn require_body(body: Option<Value>, command_name: &str) -> Result<Value, CliError> {
    body.ok_or_else(|| {
        CliError::Input(format!(
            "{command_name} requires input; pass typed flags or --body"
        ))
    })
}

pub fn parse_key_value_object(input: &str) -> Result<Map<String, Value>, CliError> {
    let mut map = Map::new();

    for entry in input.split(',') {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (key, raw_value) = trimmed.split_once('=').ok_or_else(|| {
            CliError::Input(format!(
                "invalid field specification `{trimmed}`; expected key=value"
            ))
        })?;

        let key = key.trim();
        let raw_value = raw_value.trim();

        if key.is_empty() {
            return Err(CliError::Input(
                "field names in typed specs cannot be empty".to_string(),
            ));
        }

        map.insert(key.to_string(), coerce_scalar(raw_value));
    }

    if map.is_empty() {
        return Err(CliError::Input(
            "typed spec must contain at least one key=value pair".to_string(),
        ));
    }

    Ok(map)
}

pub fn insert_string(target: &mut Map<String, Value>, key: &str, value: Option<String>) {
    if let Some(value) = value {
        target.insert(key.to_string(), Value::String(value));
    }
}

pub fn insert_i64(target: &mut Map<String, Value>, key: &str, value: Option<i64>) {
    if let Some(value) = value {
        target.insert(key.to_string(), Value::Number(value.into()));
    }
}

pub fn insert_bool_if_true(target: &mut Map<String, Value>, key: &str, value: bool) {
    if value {
        target.insert(key.to_string(), Value::Bool(true));
    }
}

pub fn insert_array(target: &mut Map<String, Value>, key: &str, values: Vec<Value>) {
    if !values.is_empty() {
        target.insert(key.to_string(), Value::Array(values));
    }
}

fn coerce_scalar(value: &str) -> Value {
    if value.eq_ignore_ascii_case("true") {
        return Value::Bool(true);
    }

    if value.eq_ignore_ascii_case("false") {
        return Value::Bool(false);
    }

    if value.eq_ignore_ascii_case("null") {
        return Value::Null;
    }

    if let Ok(parsed) = value.parse::<i64>() {
        return Value::Number(parsed.into());
    }

    if let Ok(parsed) = value.parse::<f64>() {
        if let Some(number) = Number::from_f64(parsed) {
            return Value::Number(number);
        }
    }

    Value::String(value.to_string())
}
