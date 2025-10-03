pub mod note;
pub mod todo;

pub use note::*;

use anyhow::Context;
use serde_json::Value as JsonValue;

fn get_str(value: &JsonValue, key: &str) -> Result<String, anyhow::Error> {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .context("{key} is required")
        .map(String::from)
}
