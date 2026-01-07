use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

use tauri::{AppHandle, Emitter};

use crate::utils::crypto::{derive_key, encrypt_decrypt_at_offset, generate_salt, SALT_LEN};
use crate::utils::files::get_file_size;

/// 加密文件
#[tauri::command]
pub fn encrypt_file(app: AppHandle, input_path: &str, output_path: &str, password: &str) -> Result<(), String> {
  let file_size = get_file_size(input_path).unwrap();

  let input_file = File::open(input_path).unwrap();
  let mut output_file = File::create(output_path).unwrap();

  // 1. 生成随机盐值并写入文件头
  let salt = generate_salt();
  output_file.write_all(&salt).unwrap();

  // 2. 派生密钥
  let key = derive_key(password, &salt);

  let mut reader = BufReader::new(input_file);
  let mut writer = BufWriter::new(output_file);

  let mut buffer = [0u8; 64 * 1024];
  let mut offset = 0u64;

  log::info!("Starting streaming encryption...");

  while let Ok(n) = reader.read(&mut buffer) {
    if n == 0 {
      break;
    }

    let chunk = &mut buffer[..n];
    encrypt_decrypt_at_offset(chunk, offset, &key);

    writer.write_all(chunk).unwrap();

    offset += n as u64;

    let _ = app.emit("encrypt_progress", offset as f64 / file_size as f64 * 100.0);
  }

  writer.flush().unwrap();

  let _ = app.emit("encrypt_progress", 100);

  Ok(())
}

#[tauri::command]
pub async fn decrypt_file(app: AppHandle, input_path: &str, output_path: &str, password: &str) -> Result<(), String> {
  let file_size = get_file_size(input_path).unwrap();

  let mut input_file = File::open(&input_path).unwrap();

  // 1. 读取盐值
  let mut salt = [0u8; SALT_LEN];
  input_file.read_exact(&mut salt).unwrap();

  // 2. 派生密钥
  let key = derive_key(&password, &salt);

  let output_file = File::create(&output_path).unwrap();

  let mut reader = BufReader::new(input_file);
  let mut writer = BufWriter::new(output_file);

  let mut buffer = [0u8; 64 * 1024];
  let mut offset = 0u64;

  while let Ok(n) = reader.read(&mut buffer) {
    if n == 0 {
      break;
    }
    let chunk = &mut buffer[..n];
    // 解密和加密是同一个 XOR 过程
    encrypt_decrypt_at_offset(chunk, offset, &key);
    writer.write_all(chunk).unwrap();
    offset += n as u64;

    let _ = app.emit("encrypt_progress", offset as f64 / file_size as f64 * 100.0);
  }

  writer.flush().unwrap();

  let _ = app.emit("encrypt_progress", 100);

  Ok(())
}
