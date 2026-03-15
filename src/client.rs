use std::collections::BTreeMap;

use reqwest::header::ACCEPT;
use serde_json::Value;

use crate::auth::bearer_headers;
use crate::config::Config;
use crate::error::CliError;
use crate::operations::OperationSpec;

#[derive(Debug, Clone)]
pub struct SplitwiseClient {
    http: reqwest::Client,
    config: Config,
}

impl SplitwiseClient {
    pub fn new(config: Config) -> Result<Self, CliError> {
        let headers = bearer_headers(&config.token)?;
        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self { http, config })
    }

    pub async fn execute(
        &self,
        operation: OperationSpec,
        path_params: &[(&str, String)],
        query_params: &[(&str, String)],
        body: Option<Value>,
    ) -> Result<Value, CliError> {
        if operation.body_required && body.is_none() {
            return Err(CliError::Input(format!(
                "{} requires a request body; pass typed flags or --body",
                operation.name
            )));
        }

        let mut path = operation.path.to_string();
        for (key, value) in path_params {
            path = path.replace(&format!("{{{key}}}"), value);
        }

        let url = format!("{}{}", self.config.base_url.trim_end_matches('/'), path);
        let mut request = self
            .http
            .request(operation.method.clone(), &url)
            .header(ACCEPT, "application/json");

        if !query_params.is_empty() {
            let query: BTreeMap<&str, &str> = query_params
                .iter()
                .map(|(key, value)| (*key, value.as_str()))
                .collect();
            request = request.query(&query);
        }

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;
        let status = response.status();
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let text = response.text().await?;

        if !status.is_success() {
            return Err(CliError::Request(format!("{status}: {text}")));
        }

        let value = parse_response_body(content_type.as_deref(), &text)?;

        operation.evaluate(&value)?;
        Ok(value)
    }
}

fn parse_response_body(content_type: Option<&str>, raw_text: &str) -> Result<Value, CliError> {
    let trimmed = raw_text.trim();
    if trimmed.is_empty() {
        return Ok(Value::Null);
    }

    let looks_like_json = trimmed.starts_with('{') || trimmed.starts_with('[');
    let advertised_json = content_type
        .map(|value| value.contains("application/json"))
        .unwrap_or(false);

    if advertised_json || looks_like_json {
        return Ok(serde_json::from_str(trimmed)?);
    }

    Ok(Value::String(raw_text.to_string()))
}
