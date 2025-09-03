use uuid::Uuid;

use crate::types::{NewNote, Note};

pub mod file;

pub trait Store: Sized {
    fn save_note(&self, new_note: NewNote) -> impl Future<Output = Result<Uuid, anyhow::Error>>;
    fn get_note(&self, note_id: Uuid) -> impl Future<Output = Result<Note, anyhow::Error>>;
    fn get_notes(&self) -> impl Future<Output = Result<Vec<String>, anyhow::Error>>;
    fn update_note(
        &self,
        note_id: u64,
        updated_note: String,
    ) -> impl Future<Output = Result<(), anyhow::Error>>;
    fn delete_note(&self, note_id: u64) -> impl Future<Output = Result<(), anyhow::Error>>;

    fn save_todo(&self) -> impl Future<Output = Result<Uuid, anyhow::Error>>;
}
