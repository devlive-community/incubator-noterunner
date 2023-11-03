use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
  pub static ref APP: Mutex<App> = Mutex::new(App::new());
}

pub struct App {
    pub app_dir: PathBuf,
}

impl App {
    pub fn new() -> App {
        let cfg = tauri::Config::default();
        match tauri::api::path::app_config_dir(&cfg) {
            None => App {
                app_dir: PathBuf::new(),
            },
            Some(p) => App {
                app_dir: p.join("org.devlive.notepad"),
            },
        }
    }
}


pub fn init_app_dir() -> bool {
    if !Path::new(&crate::app::APP.lock().app_dir).exists() {
        if let Ok(_) = create_dir_all(&crate::app::APP.lock().app_dir) {
            return true;
        }
        return false;
    }
    true
}
