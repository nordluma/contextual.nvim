use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};
use uuid::Uuid;

type GenError = Box<dyn std::error::Error>;

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteContext {
    pub filename: String,
    pub project_dir: String,
    pub selection: String,
}

impl TryFrom<JsonMap<String, JsonValue>> for NoteContext {
    type Error = String;

    fn try_from(value: JsonMap<String, JsonValue>) -> Result<Self, Self::Error> {
        let filename = value
            .get("filename")
            .and_then(|f| f.as_str())
            .ok_or("Filename required")?
            .to_owned();
        let project_dir = value
            .get("project_dir")
            .and_then(|f| f.as_str())
            .ok_or("Project directory required")?
            .to_owned();
        let selection = value
            .get("selection")
            .and_then(|s| s.as_str())
            .ok_or("Selection required")?
            .to_owned();

        Ok(Self {
            filename,
            project_dir,
            selection,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Note {
    pub id: Uuid,
    pub context: NoteContext,
    pub content: String,
}

impl Note {
    pub fn new(new_note: NewNote) -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            context: new_note.context,
            content: new_note.content,
        }
    }
}

#[derive(Debug)]
pub struct NewNote {
    pub context: NoteContext,
    pub content: String,
}

impl TryFrom<JsonValue> for NewNote {
    type Error = String;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        let context = value
            .get("context")
            .and_then(|ctx| ctx.as_object().cloned())
            .ok_or("Invalid context")?
            .try_into()?;

        let content = value
            .get("content")
            .and_then(|c| c.as_str())
            .ok_or("content is required")?
            .to_owned();

        Ok(Self { context, content })
    }
}
