#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod storage;
mod note;
mod response;

fn main() {
    storage::check();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            note::get_notes,
            note::create_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
