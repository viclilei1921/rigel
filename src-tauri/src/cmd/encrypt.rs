use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

use tauri::{async_runtime, AppHandle, Emitter};

use crate::utils::crypto::{derive_key, encrypt_decrypt_at_offset, generate_salt, SALT_LEN};

/// 加密文件
#[tauri::command]
pub fn encrypt_file(app: AppHandle, input_path: String, output_path: String, password: String) -> Result<(), String> {
  let app_handle = app.clone();
  async_runtime::spawn(async move {
    let process = async move {
      let input_file = File::open(&input_path).await.map_err(|e| e.to_string())?;
      let mut output_file = File::create(&output_path).await.map_err(|e| e.to_string())?;
      let file_size = input_file.metadata().await.map_err(|e| e.to_string())?.len();

      // 1. 生成随机盐值并写入文件头
      let salt = generate_salt();
      output_file.write_all(&salt).await.map_err(|e| e.to_string())?;

      // 2. 派生密钥
      let key = derive_key(&password, &salt);

      let mut reader = BufReader::new(input_file);
      let mut writer = BufWriter::new(output_file);

      let mut buffer = [0u8; 64 * 1024];
      let mut offset = 0u64;

      while let Ok(n) = reader.read(&mut buffer).await {
        if n == 0 {
          break;
        }

        let chunk = &mut buffer[..n];
        encrypt_decrypt_at_offset(chunk, offset, &key);

        writer.write_all(chunk).await.map_err(|e| e.to_string())?;
        tokio::task::yield_now().await;

        offset += n as u64;

        if offset % (1024 * 1024) < 64 * 1024 || offset == file_size {
          let progress = (offset as f64 / file_size as f64 * 100.0).round();
          let _ = app_handle.emit("encrypt_progress", progress);
        }
      }

      writer.flush().await.map_err(|e| e.to_string())?;

      let _ = app_handle.emit("encrypt_progress", 100.0);

      Ok::<(), String>(())
    };

    if let Err(e) = process.await {
      log::error!("Error encrypting file: {}", e);
      let _ = app.emit("encrypt_error", e.to_string());
    }
  });

  Ok(())
}

/// 解密文件
#[tauri::command]
pub async fn decrypt_file(
  app: AppHandle,
  input_path: String,
  output_path: String,
  password: String,
) -> Result<(), String> {
  let app_handle = app.clone();

  async_runtime::spawn(async move {
    let process = async move {
      let mut input_file = File::open(&input_path).await.map_err(|e| e.to_string())?;
      let output_file = File::create(&output_path).await.map_err(|e| e.to_string())?;
      let file_size = input_file.metadata().await.map_err(|e| e.to_string())?.len();

      // 1. 读取盐值
      let mut salt = [0u8; SALT_LEN];
      input_file.read_exact(&mut salt).await.map_err(|e: std::io::Error| e.to_string())?;

      // 2. 派生密钥
      let key = derive_key(&password, &salt);

      let mut reader = BufReader::new(input_file);
      let mut writer = BufWriter::new(output_file);

      let mut buffer = [0u8; 64 * 1024];
      let mut offset = 0u64;

      while let Ok(n) = reader.read(&mut buffer).await {
        if n == 0 {
          break;
        }

        let chunk = &mut buffer[..n];
        encrypt_decrypt_at_offset(chunk, offset, &key);

        writer.write_all(chunk).await.map_err(|e| e.to_string())?;

        offset += n as u64;

        tokio::task::yield_now().await;
        if offset % (1024 * 1024) < 64 * 1024 || offset == file_size {
          let progress = (offset as f64 / file_size as f64 * 100.0).round();
          let _ = app_handle.emit("encrypt_progress", progress);
        }
      }

      writer.flush().await.map_err(|e| e.to_string())?;
      let _ = app_handle.emit("encrypt_progress", 100.0);

      Ok::<(), String>(())
    };

    if let Err(e) = process.await {
      log::error!("Error decrypting file: {}", e);
      let _ = app.emit("encrypt_error", e.to_string());
    }
  });

  Ok(())
}
