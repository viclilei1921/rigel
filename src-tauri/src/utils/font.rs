use std::path::Path;

/// --- 自动获取系统默认字体路径 ---
pub fn get_default_font_path() -> String {
  // 候选列表：根据不同系统，按优先级尝试字体
  let candidates = if cfg!(target_os = "windows") {
    vec![
      // 优先级 1: 微软雅黑 (Windows 默认中文 UI 字体，通常是 .ttc)
      r"C:\Windows\Fonts\msyh.ttc",
      // 优先级 2: 微软雅黑 (旧版 Windows 可能是 .ttf)
      r"C:\Windows\Fonts\msyh.ttf",
      // 优先级 3: 黑体 (备用中文)
      r"C:\Windows\Fonts\simhei.ttf",
      // 优先级 4: Segoe UI (英文 UI 字体，可能含少量中文，但不全)
      r"C:\Windows\Fonts\segoeui.ttf",
      // 优先级 5: Arial (仅英文，最后兜底)
      r"C:\Windows\Fonts\arial.ttf",
    ]
  } else if cfg!(target_os = "macos") {
    vec![
      "/System/Library/Fonts/PingFang.ttc", // 苹方 (中文首选)
      "/Library/Fonts/Arial Unicode.ttf",   // Arial Unicode (含中文)
      "/System/Library/Fonts/STHeiti Light.ttc",
      "/System/Library/Fonts/Helvetica.ttc",
    ]
  } else {
    // Linux (建议安装 google-noto-cjk)
    vec![
      "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
      "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
      "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc", // 文泉驿微米黑
      "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
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
