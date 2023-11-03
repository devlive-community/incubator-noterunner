use std::path::PathBuf;
use log::info;
use rusqlite::{Connection, Error};

const DB_FILE: &str = "notepad.db";

fn open_connection() -> Result<Connection, Error> {
    let database = PathBuf::from(&crate::app::APP.lock().app_dir)
        .join(DB_FILE);
    let conn = Connection::open(database)?;
    Ok(conn)
}

pub fn exec<F, T>(func: F) -> Result<T, Error>
    where
        F: FnOnce(&mut Connection) -> Result<T, Error>,
{
    match open_connection() {
        Ok(mut conn) => func(&mut conn),
        Err(e) => Err(e),
    }
}

fn init_table() {
    let res = exec(|conn| {
        // Initialize the article table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes
                (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    title       VARCHAR(2000) NOT NULL,
                    editor VARCHAR(2000) NOT NULL DEFAULT 'Markdown',
                    content     TEXT,
                    is_delete   BOOLEAN                DEFAULT FALSE,
                    create_time timestamp              DEFAULT current_timestamp
                )",
            [],
        )?;
        Ok(true)
    });
    info!("Initialize tables {:?}", res.unwrap());
}

pub fn check() {
    init_table();
}
