use serde_json::{Value, json};

use crate::database::Store;

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

    pub async fn save_todo_item(&self, params: Value) -> Result<Value, anyhow::Error> {
        let new_todo_item = params.try_into()?;

        todo!()
    }
}
