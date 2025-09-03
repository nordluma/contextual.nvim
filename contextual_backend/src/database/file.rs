use std::{env::current_dir, path::PathBuf};

use uuid::Uuid;

use crate::types::{NewNote, Note};

use super::Store;

pub struct FileDatabase {
    dir: PathBuf,
}

impl FileDatabase {
    pub fn init() -> Self {
        let dir = current_dir().unwrap();

        Self { dir }
    }
}

impl Store for FileDatabase {
    async fn save_note(&self, new_note: NewNote) -> Result<Uuid, anyhow::Error> {
        let note = Note::new(new_note);

        if !std::fs::exists(&self.dir)? {
            std::fs::create_dir_all(&self.dir)?;
        }

        let note_file = self.dir.join(note.id.to_string());
        let f = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(note_file)?;

        serde_json::to_writer(f, &note)?;

        Ok(note.id)
    }

    async fn get_note(&self, note_id: Uuid) -> Result<Note, anyhow::Error> {
        let note_file = self.dir.join(note_id.to_string());
        let file = std::fs::File::open(note_file)?;
        let note = serde_json::from_reader(file)?;

        Ok(note)
    }

    async fn get_notes(&self) -> Result<Vec<String>, anyhow::Error> {
        todo!()
    }

    async fn update_note(&self, note_id: u64, updated_note: String) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn delete_note(&self, note_id: u64) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn save_todo(&self) -> Result<Uuid, anyhow::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        database::{Store, file::FileDatabase},
        types::{NewNote, NoteContext},
    };

    #[tokio::test]
    async fn new_note_is_saved_to_disk() {
        let new_note = NewNote {
            context: NoteContext {
                filename: "test_file.rs".to_string(),
                project_dir: "/test_user/projects/test_project".to_string(),
                selection: "fn test_function() {".to_string(),
            },
            content: "A new test note".to_string(),
        };
        let file_db = FileDatabase::init();
        println!("saving files to: {}", file_db.dir.display());
        let id = file_db.save_note(new_note).await.unwrap();

        assert!(std::fs::exists(id.to_string()).unwrap());

        std::fs::remove_file(id.to_string()).unwrap();
    }
}
