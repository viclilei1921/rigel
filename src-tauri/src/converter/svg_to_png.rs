use std::{fs, path};

use resvg::usvg;
use tauri::{AppHandle, Runtime};

use crate::utils::files::get_cache_dir;

#[tauri::command]
pub fn svg_to_png<R: Runtime>(app: AppHandle<R>, svg_path: String, width: u32, height: u32) -> Result<String, String> {
  let cache_dir = get_cache_dir(app)?;
  let filename = path::Path::new(&svg_path).file_prefix().expect("file path error");

  println!("cache dir: {}", cache_dir.display());

  let out_file_name = format!("{}_{}x{}.png", filename.to_str().unwrap(), width, height);
  let output_path = cache_dir.join(out_file_name);

  let svg_data = fs::read(&svg_path).map_err(|e| e.to_string())?;
  let opt = usvg::Options::default();

  let tree = usvg::Tree::from_data(&svg_data, &opt).map_err(|e| format!("parse svg failed: {}", e))?;
  let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or("create canvas error")?;

  let svg_size = tree.size();

  let scale_x = width as f32 / svg_size.width();
  let scale_y = height as f32 / svg_size.height();
  let transform = tiny_skia::Transform::from_scale(scale_x, scale_y);

  resvg::render(&tree, transform, &mut pixmap.as_mut());

  pixmap.save_png(&output_path).map_err(|e| e.to_string())?;

  Ok(output_path.to_string_lossy().into_owned())
}
