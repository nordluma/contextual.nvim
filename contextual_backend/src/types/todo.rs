use anyhow::Context;
use serde_json::Value as JsonValue;

#[derive(Debug)]
pub struct NewTodoItem {
    pub branch: String,
    pub file_path: String,
    pub line_number: u64,
    pub content: String,
}

impl TryFrom<JsonValue> for NewTodoItem {
    type Error = anyhow::Error;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        let branch = value
            .get("branch")
            .and_then(|b| b.as_str())
            .context("branch required")?
            .to_owned();

        let file_path = value
            .get("file_path")
            .and_then(|f| f.as_str())
            .context("filename required")?
            .to_owned();

        let line_number = value
            .get("line_number")
            .and_then(|l| l.as_u64())
            .context("line number required")?
            .to_owned();

        let content = value
            .get("content")
            .and_then(|c| c.as_str())
            .context("content required")?
            .to_owned();

        Ok(Self {
            branch,
            file_path,
            line_number,
            content,
        })
    }
}
