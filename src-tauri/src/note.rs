use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::response::Response;
use crate::storage::exec;

#[derive(Serialize, Deserialize)]
pub struct Note {
    id: i32,
    #[serde(rename = "label")]
    title: String,
    editor: String,
    content: String,
    is_delete: Option<bool>,
    create_time: Option<String>,
    children: Option<Vec<Note>>,
}

#[tauri::command]
pub fn get_notes() -> Response<Vec<Note>> {
    match exec(|conn| {
        let mut stmt = conn.prepare("SELECT * FROM notes")?;
        let mut res = stmt.query(params![])?;
        let mut vec = vec![];
        while let Some(row) = res.next()? {
            vec.push(Note {
                id: row.get("id")?,
                title: row.get("title")?,
                editor: row.get("editor")?,
                content: row.get("content")?,
                is_delete: row.get("is_delete")?,
                create_time: row.get("create_time")?,
                children: Some(Vec::new()),
            })
        }
        Ok(vec)
    })
    {
        Ok(records) => Response::new(200, records, "Successful！".to_string()),
        Err(_err) => Response::new(500, Vec::new(), _err.to_string()),
    }
}

#[tauri::command]
pub async fn create_note(note: Note) -> Response<Option<bool>> {
    match exec(|conn| {
        let conn = conn.execute(
            "INSERT INTO notes (title, editor, content, is_delete)\
             VALUES (:title, :editor, :content, :is_delete)",
            params![
                note.title,
                note.editor,
                note.content,
                false
            ],
        )?;
        Ok(conn > 0)
    })
    {
        Ok(_count) => Response::new(200, Some(true), "Successful！".to_string()),
        Err(_err) => Response::new(500, Some(false), _err.to_string()),
    }
}
