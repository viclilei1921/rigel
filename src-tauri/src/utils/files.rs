use std::{fs, path::PathBuf};

use tauri::{AppHandle, Manager, Runtime};

use crate::get_file_path;

/// 获取缓存文件夹
pub fn get_cache_dir<R: Runtime>(app: AppHandle<R>) -> Result<PathBuf, String> {
  let cache_dir = app.path().cache_dir().map_err(|e| e.to_string())?;
  let file_path = get_file_path();

  let dir = cache_dir.join(file_path);
  if !dir.exists() {
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
  }

  Ok(cache_dir.join(file_path))
}

/// 获取临时文件夹
pub fn get_cache_temp_dir<R: Runtime>(app: AppHandle<R>) -> Result<PathBuf, String> {
  let cache_dir = get_cache_dir(app)?;

  let dir = cache_dir.join("temp");

  if !dir.exists() {
    fs::create_dir(&dir).map_err(|e| e.to_string())?;
  }

  Ok(dir)
}

pub fn get_file_size(path: String) -> Result<u64, std::io::Error> {
  let metadata = fs::metadata(path).unwrap();
  Ok(metadata.len())
}
