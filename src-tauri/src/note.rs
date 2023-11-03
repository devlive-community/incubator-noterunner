use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::response::Response;
use crate::storage::exec;

#[derive(Serialize, Deserialize)]
pub struct Note {
    id: i32,
    #[serde(rename = "label")]
    title: String,
    editor_type: String,
    content: String,
    is_delete: bool,
    create_time: i64,
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
                editor_type: row.get("editor_type")?,
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
