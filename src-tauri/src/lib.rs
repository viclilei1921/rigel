// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod converter;
pub mod utils;

use std::sync::OnceLock;

static FILE_PATH: OnceLock<String> = OnceLock::new();

fn init_file_path(file_path: String) {
  FILE_PATH.set(file_path).unwrap_or_else(|_| {
    panic!("FILE_PATH already initialized")
  });
}

pub fn get_file_path() -> &'static String {
  FILE_PATH.get().expect("FILE_PATH not initialized")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  init_file_path("rigel".to_string());
  
  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_os::init())
    .invoke_handler(tauri::generate_handler![converter::svg_to_png::svg_to_png, utils::files::open_cache_folder])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
