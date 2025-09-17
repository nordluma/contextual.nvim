use std::path::PathBuf;

use anyhow::Context;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    database::{NoteStorage, TodoStorage},
    types::{
        NewNote, Note,
        todo::{NewTodoItem, TodoItem},
    },
};

pub struct FileDatabase {
    dir: PathBuf,
}

impl FileDatabase {
    pub async fn init() -> Self {
        // TODO: error handling
        let dir = dirs::data_dir()
            .expect("unable to determine 'data' directory")
            .join("contextual");

        if !tokio::fs::try_exists(&dir)
            .await
            .expect("could not check for existing directory")
        {
            tokio::fs::create_dir_all(&dir)
                .await
                .expect("failed to create directory for file based storage");
        }

        Self { dir }
    }
}

#[async_trait::async_trait]
impl NoteStorage for FileDatabase {
    async fn save_note(&self, new_note: NewNote) -> Result<Uuid, anyhow::Error> {
        let note_dir = self.dir.join("notes");
        if !tokio::fs::try_exists(&note_dir).await? {
            tokio::fs::create_dir_all(&note_dir).await?;
        }

        let note = Note::new(new_note);
        let note_file = note_dir.join(note.id.to_string());
        let note_id = note.id;
        write_file(note_file, note).await?;

        Ok(note_id)
    }

    async fn get_note(&self, note_id: Uuid) -> Result<Note, anyhow::Error> {
        let note_file = self.dir.join("notes").join(note_id.to_string());
        let file = std::fs::File::open(note_file)?;
        let note = serde_json::from_reader(file)?;

        Ok(note)
    }

    async fn get_notes(&self) -> Result<Vec<String>, anyhow::Error> {
        todo!()
    }

    async fn update_note(&self, _note_id: u64, _updated_note: String) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn delete_note(&self, _note_id: u64) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[async_trait::async_trait]
impl TodoStorage for FileDatabase {
    async fn save_todo(&self, new_todo: NewTodoItem) -> Result<Uuid, anyhow::Error> {
        if !tokio::fs::try_exists(&self.dir).await? {
            tokio::fs::create_dir_all(&self.dir).await?;
        }

        let todo_item = TodoItem::new(new_todo);
        let todo_file = self.dir.join(todo_item.id.to_string());
        let todo_id = todo_item.id;
        write_file(todo_file, todo_item).await?;

        Ok(todo_id)
    }

    async fn get_todos(&self) -> Result<Vec<TodoItem>, anyhow::Error> {
        todo!()
    }
}

async fn write_file<S: Serialize + Send + 'static>(
    path: PathBuf,
    content: S,
) -> Result<(), anyhow::Error> {
    tokio::task::spawn_blocking(move || {
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .context("failed to open file for writing")
            .and_then(|f| serde_json::to_writer(f, &content).context("failed to write to file"))
    })
    .await?
}

#[cfg(test)]
mod tests {
    use crate::{
        database::{NoteStorage, file::FileDatabase},
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
        let file_db = FileDatabase::init().await;
        println!("saving files to: {}", file_db.dir.display());
        let id = file_db.save_note(new_note).await.unwrap();

        assert!(std::fs::exists(id.to_string()).unwrap());

        std::fs::remove_file(id.to_string()).unwrap();
    }
}
