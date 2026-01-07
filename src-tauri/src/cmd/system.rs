use crate::utils::gpu;

use tauri::{AppHandle, Runtime};

use crate::utils::files::get_cache_dir;

#[tauri::command]
pub async fn get_gpu_info() -> Result<Vec<gpu::GpuInfo>, String> {
  gpu::get_gpu_info().await
}

/// 打开缓存文件夹
#[tauri::command]
pub fn open_cache_folder<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
  let dir = get_cache_dir(app).map_err(|e| e.to_string())?;

  opener::open(&dir).map_err(|e| e.to_string())?;

  Ok(())
}

/// 打开文件夹并选中文件
#[tauri::command]
pub async fn reveal_in_explorer(path: String) {
  #[cfg(target_os = "windows")]
  {
    std::process::Command::new("explorer").args(["/select,", &path]).spawn().ok();
  }
  #[cfg(target_os = "macos")]
  {
    std::process::Command::new("open").args(["-R", &path]).spawn().ok();
  }
}
