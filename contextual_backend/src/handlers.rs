use serde_json::{Value, json};

use crate::{database::Store, types::todo::NewTodoItems};

pub struct Handler<DB> {
    database: DB,
}

impl<DB: Store> Handler<DB> {
    pub fn new(database: DB) -> Self {
        Self { database }
    }

    pub async fn save_note(&self, params: Value) -> Result<Value, anyhow::Error> {
        let new_note = params.try_into()?;
        let note_id = self.database.save_note(new_note).await?;

        Ok(json!({"id": note_id}))
    }

    pub async fn sync_todos(&self, params: Value) -> Result<Value, anyhow::Error> {
        let _project_todos: NewTodoItems = params.try_into()?;
        let _saved_todos = self.database.get_todos().await?;

        todo!()
    }

    pub async fn save_todo_item(&self, params: Value) -> Result<Value, anyhow::Error> {
        let new_todo = params.try_into()?;
        let todo_id = self.database.save_todo(new_todo).await?;

        Ok(json!({"id": todo_id}))
    }
}
