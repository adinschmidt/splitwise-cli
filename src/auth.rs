use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

use crate::error::CliError;

pub fn bearer_headers(token: &str) -> Result<HeaderMap, CliError> {
    let mut headers = HeaderMap::new();
    let value = HeaderValue::from_str(&format!("Bearer {token}"))
        .map_err(|error| CliError::Config(format!("invalid bearer token: {error}")))?;
    headers.insert(AUTHORIZATION, value);
    Ok(headers)
}
