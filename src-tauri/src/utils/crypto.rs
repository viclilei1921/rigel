use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use chacha20::ChaCha20;

use argon2::Argon2;

use rand::RngCore;

/// NONCE: 12 bytes 这个不能修改
const NONCE: &[u8; 12] = b"unique-vicli";
pub const SALT_LEN: usize = 16;

pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
  let mut key = [0u8; 32];
  let argon2 = Argon2::default();
  // 使用 Argon2 将密码和盐派生出 32 字节密钥
  argon2.hash_password_into(password.as_bytes(), salt, &mut key).expect("Failed to derive key");
  key
}

pub fn generate_salt() -> [u8; SALT_LEN] {
  let mut salt = [0u8; SALT_LEN];
  rand::rng().fill_bytes(&mut salt);
  salt
}

pub fn encrypt_decrypt_at_offset(data: &mut [u8], offset: u64, key: &[u8; 32]) {
  let mut cipher = ChaCha20::new(key.into(), NONCE.into());
  cipher.seek(offset);
  cipher.apply_keystream(data);
}
