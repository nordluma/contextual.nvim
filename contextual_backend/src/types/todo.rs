use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::types::get_str;

#[allow(unused)]
pub struct NewTodoItems(Vec<NewTodoItem>);

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
        let branch = get_str(&value, "branch")?;
        let file_path = get_str(&value, "file_path")?;
        let line_number = get_str(&value, "line_number")
            .and_then(|l| l.parse().context("failed to parse line_number"))?;
        let content = get_str(&value, "content")?;

        Ok(Self {
            branch,
            file_path,
            line_number,
            content,
        })
    }
}

impl TryFrom<JsonValue> for NewTodoItems {
    type Error = anyhow::Error;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        let todo_items = value
            .as_array()
            .cloned()
            .context("params is expected to be an array")?
            .into_iter()
            .flat_map(NewTodoItem::try_from)
            .collect();

        Ok(NewTodoItems(todo_items))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoItem {
    pub id: Uuid,
    pub hash: String,
    pub branch: String,
    pub file_path: String,
    pub line_number: u64,
    pub content: String,
    pub created_at: chrono::DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TodoItem {
    pub fn new(new_todo: NewTodoItem) -> Self {
        let id = Uuid::new_v4();
        let hash = String::new(); // TODO: compute hash

        Self {
            id,
            hash,
            branch: new_todo.branch,
            file_path: new_todo.file_path,
            line_number: new_todo.line_number,
            content: new_todo.content,
            created_at: Utc::now(),
            deleted_at: None,
        }
    }
}
