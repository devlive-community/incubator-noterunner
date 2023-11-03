#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod storage;
mod note;
mod response;
mod app;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            if !app::init_app_dir() {
                panic!("Failed to initialize app dir");
            }
            storage::check();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            note::get_notes,
            note::create_note
        ])
        .run(tauri::generate_context!())
        .expect("Failed running tauri application");
}
