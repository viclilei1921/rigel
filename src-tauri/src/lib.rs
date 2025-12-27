// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod converter;
pub mod shell;
pub mod utils;

use chrono::{FixedOffset, Utc};
use std::sync::OnceLock;

use tauri_plugin_log::{Target, TargetKind};

static FILE_PATH: OnceLock<String> = OnceLock::new();

fn init_file_path(file_path: String) {
  FILE_PATH.set(file_path).unwrap_or_else(|_| panic!("FILE_PATH already initialized"));
}

pub fn get_file_path() -> &'static String {
  FILE_PATH.get().expect("FILE_PATH not initialized")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  init_file_path("rigel".to_string());

  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(
      tauri_plugin_log::Builder::new()
        .targets([
          Target::new(TargetKind::Stdout),                                     // 输出到控制台
          Target::new(TargetKind::LogDir { file_name: Some("rigel".into()) }), // 自定义文件名
          Target::new(TargetKind::Webview),
        ])
        .format(|out, message, record| {
          // 获取东八区时间 (UTC+8)
          let offset = FixedOffset::east_opt(8 * 3600).unwrap();
          let now = Utc::now().with_timezone(&offset);

          let target = if record.target().starts_with("webview") { "frontend" } else { record.target() };

          out.finish(format_args!(
            "[{}] [{}] [{}] {}",
            now.format("%Y-%m-%d %H:%M:%S"), // 格式化时间
            record.level(),
            target,
            message
          ))
        })
        .level(log::LevelFilter::Info)
        .build(),
    )
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![
      converter::svg_to_png::svg_to_png,
      utils::files::open_cache_folder,
      utils::gpu::get_gpu_info,
      shell::ffmpeg::convert_video_to_mp4,
      shell::ffmpeg::create_highlight_video,
      shell::ffmpeg::merge_smart
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
