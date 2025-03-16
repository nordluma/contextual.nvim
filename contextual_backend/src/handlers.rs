use serde_json::{Value, json};

use crate::database::Store;

pub struct Handler<DB> {
    database: DB,
}

impl<DB: Store> Handler<DB> {
    pub fn new(database: DB) -> Self {
        Self { database }
    }

    pub async fn save_note(&self, params: Value) -> Result<Value, String> {
        let new_note = params.try_into()?;
        match self.database.save_note(new_note).await {
            Ok(note_id) => Ok(json!({"id": note_id})),
            Err(e) => Err(format!("Database error: {e:?}").into()),
        }
    }
}
