use std::path::Path;

/// --- 自动获取系统默认字体路径 ---
pub fn get_default_font_path() -> String {
  // 候选列表：根据不同系统，按优先级尝试字体
  let candidates = if cfg!(target_os = "windows") {
    vec![
      r"C:\Windows\Fonts\arial.ttf",   // 首选 Arial
      r"C:\Windows\Fonts\segoeui.ttf", // 备选 Segoe UI (Win10/11 默认)
      r"C:\Windows\Fonts\tahoma.ttf",
      r"C:\Windows\Fonts\msyh.ttf", // 微软雅黑 (如果想要中文支持首选这个)
    ]
  } else if cfg!(target_os = "macos") {
    vec![
      "/System/Library/Fonts/Helvetica.ttc",
      "/Library/Fonts/Arial.ttf",
      "/System/Library/Fonts/PingFang.ttc", // 中文支持
    ]
  } else {
    // Linux
    vec![
      "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
      "/usr/share/fonts/TTF/DejaVuSans.ttf",
      "/usr/share/fonts/gnu-free/FreeSans.ttf",
      "/usr/share/fonts/noto/NotoSans-Regular.ttf",
    ]
  };

  // 检查哪个文件实际存在
  for path in candidates {
    if Path::new(path).exists() {
      // Windows 下 FFmpeg 路径需要转义: C:\ -> C\\:
      if cfg!(target_os = "windows") {
        let clean_font_path = path.replace("\\", "/");
        return clean_font_path.replace(":", "\\\\:");
      }
      return path.to_string();
    }
  }

  // 如果都没找到，返回一个空字符串，或者抛出警告
  // 此时 FFmpeg 可能会报错，但这是最后的兜底
  log::warn!("warn: no default font found, ffmpeg may fail.");
  "arial.ttf".to_string()
}
