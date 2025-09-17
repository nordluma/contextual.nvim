use uuid::Uuid;

use crate::types::{
    NewNote, Note,
    todo::{NewTodoItem, TodoItem},
};

pub mod file;

pub trait Storage: NoteStorage + TodoStorage {}
impl<T: NoteStorage + TodoStorage> Storage for T {}

#[async_trait::async_trait]
pub trait NoteStorage: Send + Sync {
    async fn save_note(&self, new_note: NewNote) -> Result<Uuid, anyhow::Error>;
    async fn get_note(&self, note_id: Uuid) -> Result<Note, anyhow::Error>;
    async fn get_notes(&self) -> Result<Vec<String>, anyhow::Error>;
    async fn update_note(&self, note_id: u64, updated_note: String) -> Result<(), anyhow::Error>;
    async fn delete_note(&self, note_id: u64) -> Result<(), anyhow::Error>;
}

#[async_trait::async_trait]
pub trait TodoStorage: Send + Sync {
    async fn save_todo(&self, new_todo: NewTodoItem) -> Result<Uuid, anyhow::Error>;
    async fn get_todos(&self) -> Result<Vec<TodoItem>, anyhow::Error>;
}
