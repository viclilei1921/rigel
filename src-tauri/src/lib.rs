// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod converter;
pub mod shell;
pub mod utils;

use chrono::{FixedOffset, Utc};
use std::sync::OnceLock;
use tauri::{
  Manager, WindowEvent, menu::{Menu, MenuItem}, tray::{MouseButton, TrayIconBuilder, TrayIconEvent}
};

use tauri_plugin_log::{Target, TargetKind};

/// 程序文件缓存路径
static FILE_PATH: OnceLock<String> = OnceLock::new();

/// 初始化文件缓存路径
fn init_file_path(file_path: String) {
  FILE_PATH.set(file_path).unwrap_or_else(|_| panic!("FILE_PATH already initialized"));
}

/// 获取文件缓存路径
pub fn get_file_path() -> &'static String {
  FILE_PATH.get().expect("FILE_PATH not initialized")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  init_file_path("rigel".to_string());

  tauri::Builder::default()
    .on_window_event(|window, event| {
      if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        window.hide().unwrap();
      }
    })
    .setup(|app| {
      // 1. 创建菜单项
      // 参数: manager, id, text, enabled, accelerator
      let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
      let show_i = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;

      // 2. 创建菜单
      let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

      let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .show_menu_on_left_click(false)
        .menu(&menu)
        // 4. 处理菜单点击事件 (右键菜单)
        .on_menu_event(|app, event| match event.id.as_ref() {
          "quit" => {
            app.exit(0);
          }
          "show" => {
            if let Some(window) = app.get_webview_window("main") {
              utils::window::focus_window(window);
            }
          }
          _ => {}
        })
        // 5. 处理托盘图标本身的点击事件 (左键点击)
        .on_tray_icon_event(|tray, event| {
          if let TrayIconEvent::Click { button: MouseButton::Left, .. } = event {
            let app = tray.app_handle();
            if let Some(window) = app.get_webview_window("main") {
              utils::window::focus_window(window);
            }
          }

          
        })
        .build(app)?;
      Ok(())
    })
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
        .filter(|metadata| {
          let target = metadata.target();

          // 屏蔽 wgpu 相关模块
          !target.starts_with("wgpu") && !target.starts_with("wgpu_hal") && !target.starts_with("wgpu_core")
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
      utils::files::reveal_in_explorer,
      utils::gpu::get_gpu_info,
      shell::ffmpeg::convert_video_to_mp4,
      shell::ffmpeg::create_highlight_video,
      shell::ffmpeg::merge_smart,
      shell::ffmpeg::get_video_info
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
