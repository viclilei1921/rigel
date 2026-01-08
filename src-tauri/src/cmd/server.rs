use std::{path::PathBuf, sync::Mutex};

use tauri::State;
use tokio::sync::oneshot;

use crate::utils::server::start_server;

pub struct ServerState {
  pub shutdown_tx: Mutex<Option<oneshot::Sender<()>>>,
}

#[tauri::command]
pub async fn start_video_stream(
  password: String,
  path: String,
  state: State<'_, ServerState>,
) -> Result<String, String> {
  let video_path = PathBuf::from(path);

  if !video_path.exists() {
    return Err("Video file not found".to_string());
  }

  let (tx, rx) = oneshot::channel();
  {
    let mut tx_guard = state.shutdown_tx.lock().unwrap();
    if let Some(old_tx) = tx_guard.take() {
      let _ = old_tx.send(());
    }
    *tx_guard = Some(tx);
  }

  let port = start_server(password, video_path, rx).await;

  Ok(format!("http://127.0.0.1:{}/video", port))
}

#[tauri::command]
pub async fn stop_video_stream(state: State<'_, ServerState>) -> Result<String, String> {
  let mut tx_guard = state.shutdown_tx.lock().unwrap();
  if let Some(tx) = tx_guard.take() {
    let _ = tx.send(());
    Ok("Server stopped".to_string())
  } else {
    Ok("No server running".to_string())
  }
}
