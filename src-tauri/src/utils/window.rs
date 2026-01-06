use tauri::WebviewWindow;

/// 窗口聚焦
pub fn focus_window(window: WebviewWindow) {
  if window.is_minimized().unwrap_or(false) {
    let _ = window.unminimize();
  }
  let _ = window.show();
  let _ = window.set_focus();
}