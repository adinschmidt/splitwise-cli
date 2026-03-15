use comfy_table::{ContentArrangement, Table, presets::UTF8_FULL};
use serde_json::{Map, Value};

use crate::cli::OutputFormat;
use crate::error::CliError;

pub fn print(value: &Value, format: OutputFormat) -> Result<(), CliError> {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string(value)?);
        }
        OutputFormat::Yaml => {
            println!(
                "{}",
                serde_yaml::to_string(value)
                    .map_err(|error| CliError::Config(error.to_string()))?
            );
        }
        OutputFormat::Table => {
            println!("{}", render_human(value)?);
        }
    }

    Ok(())
}

fn render_human(value: &Value) -> Result<String, CliError> {
    match value {
        Value::Array(items) => render_array(items),
        Value::Object(object) => {
            if let Some((_, Value::Array(items))) = single_entry(object) {
                render_array(items)
            } else {
                Ok(serde_json::to_string_pretty(value)?)
            }
        }
        _ => Ok(render_cell(value)),
    }
}

fn single_entry(object: &Map<String, Value>) -> Option<(&String, &Value)> {
    let mut iter = object.iter();
    let first = iter.next()?;
    if iter.next().is_some() {
        None
    } else {
        Some(first)
    }
}

fn render_array(items: &[Value]) -> Result<String, CliError> {
    if items.is_empty() {
        return Ok("(no results)".to_string());
    }

    let headers = collect_headers(items);

    if headers.is_empty() {
        return Ok(serde_json::to_string_pretty(items)?);
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(headers.iter().map(String::as_str));

    for item in items {
        match item {
            Value::Object(object) => {
                let row = headers
                    .iter()
                    .map(|header| render_cell(object.get(header).unwrap_or(&Value::Null)))
                    .collect::<Vec<_>>();
                table.add_row(row);
            }
            _ => {
                table.add_row([render_cell(item)]);
            }
        }
    }

    Ok(table.to_string())
}

fn collect_headers(items: &[Value]) -> Vec<String> {
    let mut headers = Vec::new();

    for item in items {
        let Value::Object(object) = item else {
            return vec!["value".to_string()];
        };

        for key in object.keys() {
            if !headers.contains(key) {
                headers.push(key.clone());
            }
        }
    }

    headers
}

fn render_cell(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number.to_string(),
        Value::String(text) => text.clone(),
        _ => serde_json::to_string(value).unwrap_or_else(|_| "<unprintable json>".to_string()),
    }
}
