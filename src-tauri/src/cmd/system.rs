use crate::utils::gpu;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

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
    // 将路径转换为 Windows 标准格式 (C:\Path\To\File)
    let win_path = path.replace("/", "\\");

    // 2. 转义单引号，防止 PowerShell 字符串截断
    let safe_path = win_path.replace("'", "''");

    let script = format!(
      r#"
      $target = '{safe_path}'
      $parent = Split-Path $target
      $file = Split-Path $target -Leaf
      $shell = New-Object -ComObject Shell.Application
      $found = $false

      # 尝试在已打开的窗口中查找
      foreach ($win in $shell.Windows()) {{
          try {{
              $winPath = $win.Document.Folder.Self.Path
              if ($winPath -eq $parent) {{
                  $win.Visible = $true
                  if ($win.WindowState -eq 1) {{ $win.WindowState = 0 }}

                  $item = $win.Document.Folder.ParseName($file)
                  if ($item) {{
                      # 聚焦并选中 (17 = Select + Focus)
                      $win.Document.SelectItem($item, 17)
                  }}

                  $wscript = New-Object -ComObject WScript.Shell
                  $wscript.AppActivate($win.LocationName)

                  $found = $true
                  break
              }}
          }} catch {{}}
      }}

      # 如果没找到，使用 explorer.exe 打开
      if (-not $found) {{
          # 【核心修复】
          # 使用 Start-Process 显式传递 ArgumentList
          # 注意这里的三层转义：
          # 1. Rust 字符串格式化
          # 2. PowerShell 字符串
          # 3. Explorer 参数引号
          # 最终生成的命令类似于：explorer.exe "/select,C:\Path\File.txt"
          $arg = "/select,`"$target`""
          Start-Process explorer.exe -ArgumentList $arg
      }}
      "#
    );

    std::process::Command::new("powershell")
      .args(["-NoProfile", "-Command", &script])
      .creation_flags(0x08000000) // CREATE_NO_WINDOW
      .spawn()
      .ok();
    // std::process::Command::new("explorer").args(["/select,", &path]).spawn().ok();
  }
  #[cfg(target_os = "macos")]
  {
    std::process::Command::new("open").args(["-R", &path]).spawn().ok();
  }
}
