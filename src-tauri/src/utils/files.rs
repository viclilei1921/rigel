use std::{fs, path::PathBuf};

use tauri::{AppHandle, Manager, Runtime};

use crate::get_file_path;

pub fn get_cache_dir<R: Runtime>(app: AppHandle<R>) -> Result<PathBuf, String> {
  let cache_dir = app.path().cache_dir().map_err(|e| e.to_string())?;
  let file_path = get_file_path();

  let dir = cache_dir.join(file_path);
  if !dir.exists() {
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
  }

  Ok(cache_dir.join(file_path))
}

#[tauri::command]
pub fn open_cache_folder<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
  let dir = get_cache_dir(app).map_err(|e| e.to_string())?;

  opener::open(&dir).map_err(|e| e.to_string())?;

  Ok(())
}
