use std::io::Write;
use std::{fs::File, path::Path};

use regex::Regex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};

#[cfg(target_os = "windows")]
use crate::utils::font::get_default_font_path;
use crate::utils::{
  files::get_cache_dir,
  gpu::{get_gpu_info, GpuInfo},
};

#[derive(Debug, Clone)]
pub struct VideoInfo {
  pub width: u32,
  pub height: u32,
  pub fps: f64,
  pub duration: f64,
}

/// 进度事件数据结构
#[derive(Clone, Serialize, Deserialize)]
struct ProgressPayload {
  progress: f64,       // 进度百分比 (0-100)
  current_time: f64,   // 当前处理到的时间点（秒）
  total_duration: f64, // 视频总时长（秒）
  message: String,     // 进度消息
}

/// 完成事件数据结构
#[derive(Clone, Serialize, Deserialize)]
struct CompletionPayload {
  code: Option<i32>,
}

/// 事件数据结构
#[derive(Clone, Serialize, Deserialize)]
pub struct TimeSegment {
  pub start: String, // 格式 "00:00:10" 或 秒数 "10"
  pub end: String,   // 格式 "00:00:20" 或 秒数 "20"
}

/// 编码器预设
#[derive(Debug)]
enum EncoderPreset {
  Nvidia(String), // hevc_nvenc
  Intel(String),  // hevc_qsv
  Amd(String),    // hevc_amf
  Apple(String),  // hevc_videotoolbox
  Cpu(String),    // libx265
}

impl EncoderPreset {
  /// 将编码器预设转换为 FFmpeg 参数
  fn to_ffmpeg_args(&self) -> Vec<&str> {
    match self {
      EncoderPreset::Nvidia(name) => vec!["-c:v", name.as_str(), "-cq", "28", "-c:a", "aac"], // N卡参数
      EncoderPreset::Intel(name) => vec!["-c:v", name.as_str(), "-global_quality", "25", "-c:a", "aac"], // Intel参数
      EncoderPreset::Apple(name) => vec!["-c:v", name.as_str(), "-q:v", "60", "-c:a", "aac"], // Apple参数
      EncoderPreset::Amd(name) => vec!["-c:v", name.as_str(), "-c:a", "aac"],                 // A卡参数
      EncoderPreset::Cpu(name) => vec!["-c:v", name.as_str(), "-crf", "28", "-c:a", "aac"],   // CPU参数
    }
  }
}

/// 匹配厂商到编码器预设
fn match_vendor_to_encoder(name: &str) -> EncoderPreset {
  let name_lower = name.to_lowercase();

  if name_lower.contains("nvidia") {
    EncoderPreset::Nvidia("hevc_nvenc".to_string())
  } else if name_lower.contains("intel") {
    EncoderPreset::Intel("hevc_qsv".to_string())
  } else if name_lower.contains("amd") || name_lower.contains("radeon") {
    EncoderPreset::Amd("hevc_amf".to_string())
  } else if name_lower.contains("apple") {
    EncoderPreset::Apple("hevc_videotoolbox".to_string())
  } else {
    // 未知厂商，兜底回 CPU
    EncoderPreset::Cpu("libx265".to_string())
  }
}

/// 选择最佳编码器
fn select_best_encoder(gpus: &[GpuInfo]) -> EncoderPreset {
  // 1. 优先寻找独立显卡 (DiscreteGpu)
  if let Some(gpu) = gpus.iter().find(|g| g.device_type == "DiscreteGpu") {
    return match_vendor_to_encoder(&gpu.name);
  }

  // 2. 如果没有独显，寻找集成显卡 (IntegratedGpu)
  if let Some(gpu) = gpus.iter().find(|g| g.device_type == "IntegratedGpu") {
    return match_vendor_to_encoder(&gpu.name);
  }

  // 3. 都没有，返回 CPU 软解
  EncoderPreset::Cpu("libx265".to_string())
}

/// 从 FFmpeg 输出解析当前时间（秒）
fn parse_time_from_ffmpeg_output(line: &Vec<u8>) -> Option<f64> {
  let line_str = String::from_utf8_lossy(&line);

  // 查找 time= 格式的时间（如 time=00:01:23.45）
  if let Some(start) = line_str.find("time=") {
    let time_part = &line_str[start + 5..];
    let end = time_part.find(' ').unwrap_or(time_part.len());
    let time_str = &time_part[..end];

    // 将 HH:MM:SS.ms 转换为秒
    let parts: Vec<&str> = time_str.split(':').collect();
    match parts.len() {
      3 => {
        // 小时:分钟:秒
        let hours: f64 = parts[0].parse().ok()?;
        let minutes: f64 = parts[1].parse().ok()?;
        let seconds: f64 = parts[2].parse().ok()?;
        Some(hours * 3600.0 + minutes * 60.0 + seconds)
      }
      2 => {
        // 分钟:秒
        let minutes: f64 = parts[0].parse().ok()?;
        let seconds: f64 = parts[1].parse().ok()?;
        Some(minutes * 60.0 + seconds)
      }
      _ => None,
    }
  } else {
    None
  }
}

// --- 3. 核心计算逻辑：最大分辨率 & 中位数帧率 ---
fn calculate_target_params(videos_info: &[VideoInfo]) -> (u32, u32, f64) {
  // A. 计算最大分辨率 (画布能够包容所有视频)
  let max_w = videos_info.iter().map(|m| m.width).max().unwrap_or(1920);
  let max_h = videos_info.iter().map(|m| m.height).max().unwrap_or(1080);

  // B. 计算帧率中位数 (取大者)
  let mut fps_list: Vec<f64> = videos_info.iter().map(|m| m.fps).collect();
  // 排序
  fps_list.sort_by(|a, b| a.partial_cmp(b).unwrap());

  let len = fps_list.len();
  let median_fps = if len == 0 {
    30.0
  } else {
    // 如果是偶数长度 (例如 [24, 25, 30, 60])，len/2 索引是 2 (数值30)。
    // 这符合你要求的 "取最大者" (Upper Median)
    fps_list[len / 2]
  };

  (max_w, max_h, median_fps)
}

/// 解析 HH:MM:SS.ms 格式的时长
fn parse_duration_str(duration_str: &str) -> Result<f64, String> {
  let parts: Vec<&str> = duration_str.split(':').collect();
  if parts.len() != 3 {
    return Err(format!("not a valid duration: {}", duration_str));
  }

  let hours: f64 = parts[0].parse().map_err(|_| "parse hours failed".to_string())?;
  let minutes: f64 = parts[1].parse().map_err(|_| "parse minutes failed".to_string())?;
  let seconds: f64 = parts[2].parse().map_err(|_| "parse seconds failed".to_string())?;

  Ok(hours * 3600.0 + minutes * 60.0 + seconds)
}

/// 获取视频信息
#[tauri::command]
pub async fn get_video_info(app: AppHandle, video_path: &str) -> Result<VideoInfo, String> {
  let shell = app.shell();
  let (mut rx, _child) = shell.command("ffmpeg").args(&["-i", video_path]).spawn().map_err(|e| e.to_string())?;
  let mut video_info = VideoInfo { width: 0, height: 0, duration: 0.0, fps: 0.0 };

  // 正则表达式
  // 匹配分辨率: 1920x1080
  let re_res = Regex::new(r"Video:.*?\s(\d{3,})x(\d{3,})").unwrap();
  // 匹配帧率: 30 fps
  let re_fps = Regex::new(r",\s*(\d+(?:\.\d+)?)\s*fps").unwrap();
  // 匹配时长: Duration: 00:00:10.50
  let re_dur = Regex::new(r"Duration:\s*(\d{2}:\d{2}:\d{2}\.\d{2})").unwrap();

  // 完全异步处理，不使用 block_on
  while let Some(event) = rx.recv().await {
    match event {
      CommandEvent::Stderr(data) => {
        let line = String::from_utf8_lossy(&data).trim().to_string();

        if let Some(caps) = re_res.captures(&line) {
          video_info.width = caps[1].parse::<u32>().unwrap_or(1920);
          video_info.height = caps[2].parse::<u32>().unwrap_or(1080);
        }

        if let Some(caps) = re_fps.captures(&line) {
          video_info.fps = caps[1].parse::<f64>().unwrap_or(30.0);
        }

        if let Some(caps) = re_dur.captures(&line) {
          if let Ok(duration) = parse_duration_str(&caps[1]) {
            video_info.duration = duration;
          }
        }
      }
      CommandEvent::Terminated(status) => {
        if status.code != Some(0) {
          log::info!("get video duration terminated: {:?}", status.code);
        }
        break;
      }
      _ => {}
    }
  }

  if video_info.duration > 0.0 {
    return Ok(video_info);
  }

  Err("cannot get video duration".to_string())
}

/// 将视频转换成 mp4 格式
#[tauri::command]
pub async fn convert_video_to_mp4(app: AppHandle, video_path: String, output_path: String) -> Result<(), String> {
  let video_info = get_video_info(app.clone(), &video_path).await.unwrap();

  let gpus = get_gpu_info().await.unwrap();
  let best = select_best_encoder(&gpus);
  let best_args = best.to_ffmpeg_args();
  let mut args = Vec::with_capacity(4 + best_args.len());

  args.push("-i");
  args.push(&video_path);
  args.extend_from_slice(&best_args);
  args.push("-y");
  args.push(&output_path);

  // 获取 shell 实例
  let shell = app.shell();

  // 创建命令（注意：这里的 "ffmpeg" 必须在 capabilities 中配置）
  let (mut rx, _child) = shell.command("ffmpeg").args(args).spawn().map_err(|e| e.to_string())?;

  // 异步处理输出流，不要使用 block_on
  while let Some(event) = rx.recv().await {
    match event {
      CommandEvent::Stderr(line) => {
        if let Some(current_time) = parse_time_from_ffmpeg_output(&line) {
          let progress = if video_info.duration > 0.0 { (current_time / video_info.duration) * 100.0 } else { 0.0 };

          // 发射进度事件到前端
          let _ = app.emit(
            "ffmpeg-progress",
            ProgressPayload {
              progress,
              current_time,
              total_duration: video_info.duration,
              message: format!("正在转换: {:.1}%", progress),
            },
          );
        }
      }
      CommandEvent::Terminated(status) => {
        let _ = app.emit("ffmpeg-complete", CompletionPayload { code: status.code });
        break;
      }
      _ => {}
    }
  }

  return Ok(());
}

#[tauri::command]
pub async fn create_highlight_video(
  app: AppHandle,
  video_path: &str,
  output_path: &str,
  segments: Vec<TimeSegment>,
) -> Result<(), String> {
  let cache_dir = get_cache_dir(app.clone())?;
  let mut temp_files = Vec::new();
  let temp_dir = cache_dir.join("temp"); // 建议创建一个临时文件夹
  let _ = std::fs::create_dir(&temp_dir); // 忽略已存在的错误

  log::info!("--- first stage: get gpu info ---");

  let gpus = get_gpu_info().await.unwrap();
  let best = select_best_encoder(&gpus);
  let best_args = best.to_ffmpeg_args();

  log::info!("--- second stage: split video ---");

  for (i, seg) in segments.iter().enumerate() {
    let temp_dir = temp_dir.join(format!("part_{}.mp4", i));
    let temp_name = temp_dir.to_string_lossy().into_owned();

    let mut args = Vec::with_capacity(4 + best_args.len());
    args.push("-i");
    args.push(video_path);
    args.push("-ss");
    args.push(&seg.start);
    args.push("-to");
    args.push(&seg.end);
    args.extend_from_slice(&best_args);
    args.push("-y");
    args.push(&temp_name);

    log::info!("start split segment {}: {} - {} ...", i, seg.start, seg.end);

    let shell = app.shell();
    let (mut rx, _child) = shell.command("ffmpeg").args(args).spawn().map_err(|e| e.to_string())?;
    while let Some(event) = rx.recv().await {
      match event {
        CommandEvent::Stderr(data) => {
          let line = String::from_utf8_lossy(&data).trim().to_string();
          log::info!("{}", line);
        }
        CommandEvent::Terminated(status) => {
          if status.code == Some(0) {
            log::info!("end split segment {}: {} - {}.", i, seg.start, seg.end);
            temp_files.push(temp_name.clone());
            break;
          }
          log::info!("end split segment {}: {} - {} with error.", i, seg.start, seg.end);
          break;
        }
        _ => {}
      }
    }
  }

  if temp_files.is_empty() {
    return Err("not cut video".to_string());
  }

  log::info!("--- third stage: create concat list ---");

  // 2. 创建 concat 列表文件
  let list_file_name = temp_dir.join("concat_list.txt");
  let mut list_file = File::create(&list_file_name).map_err(|e| e.to_string())?;

  for path in &temp_files {
    // FFmpeg concat 列表格式: file '文件名'
    // 注意 Windows 路径转义，这里简单处理
    writeln!(list_file, "file '{}'", path).map_err(|e: std::io::Error| e.to_string())?;
  }
  list_file.flush().map_err(|e| e.to_string())?;

  log::info!("--- fourth stage: concat video ---");

  let shell = app.shell();
  let (mut rx, _child) = shell
    .command("ffmpeg")
    .args(["-f", "concat", "-safe", "0", "-i", &list_file_name.to_string_lossy(), "-c", "copy", "-y", output_path])
    .spawn()
    .map_err(|e| e.to_string())?;

  while let Some(event) = rx.recv().await {
    match event {
      CommandEvent::Stderr(data) => {
        let line = String::from_utf8_lossy(&data).trim().to_string();
        log::info!("{}", line);
      }
      CommandEvent::Terminated(status) => {
        log::info!("clear temp files ...");
        let _ = std::fs::remove_file(list_file_name);
        let _ = std::fs::remove_dir_all(temp_dir); // 删除临时文件夹

        if status.code == Some(0) {
          log::info!("concat video success");
          break;
        }
        log::info!("concat video failed, exit code: {}", status.code.unwrap());
        return Err("concat video failed".to_string());
      }
      _ => {}
    }
  }

  Ok(())
}

/// 智能合并
#[tauri::command]
pub async fn merge_smart(app: AppHandle, inputs: Vec<&str>, output_path: &str) -> Result<(), String> {
  if inputs.is_empty() {
    return Err("not find video".to_string());
  }

  // 关键步骤：构建一个包含 (路径, 元数据) 的有效列表
  // 这样可以确保后续处理时，文件和元数据是一一对应的
  let mut valid_tasks: Vec<(&str, VideoInfo)> = Vec::new();

  log::info!("--- first stage: get videos info ---");

  for input in &inputs {
    match get_video_info(app.clone(), &input).await {
      Ok(meta) => {
        log::info!("file: {} | resolute: {}x{} | fps: {}", input, meta.width, meta.height, meta.fps);
        valid_tasks.push((input, meta));
      }
      Err(e) => log::error!("file: {} | error: {}", input, e),
    }
  }

  if valid_tasks.is_empty() {
    return Err("no valid video".to_string());
  }

  // 提取纯 meta 列表用于计算
  let videos_info: Vec<VideoInfo> = valid_tasks.iter().map(|(_, m)| m.clone()).collect();
  let (target_w, target_h, target_fps) = calculate_target_params(&videos_info);

  log::info!(" target resolute: {}x{} | fps: {}", target_w, target_h, target_fps);

  log::info!("--- second stage: get gpu info ---");
  let gpus = get_gpu_info().await.unwrap();
  let best = select_best_encoder(&gpus);
  let best_args = best.to_ffmpeg_args();

  log::info!("--- third stage: build filter complex ---");
  // 构建 Filter Complex
  let mut filter_complex = String::new();
  // 2. 创建 concat 列表文件
  let cache_dir = get_cache_dir(app.clone())?;
  let temp_dir = cache_dir.join("temp");
  let _ = std::fs::create_dir(&temp_dir);
  let filter_file_name = temp_dir.join("filter.txt");
  let mut filter_file = File::create(&filter_file_name).map_err(|e| e.to_string())?;

  let font_path = get_default_font_path();

  let mut args = Vec::with_capacity(8 + best_args.len() + inputs.len() * 2);

  for (i, input_path) in inputs.iter().enumerate() {
    // 获取文件名
    let filename = Path::new(input_path).file_name().unwrap().to_string_lossy();
    // 清洗文件名，防止破坏 FFmpeg 语法
    let clean_name = filename.replace(":", "\\:").replace("'", "");

    args.push("-i");
    args.push(input_path);

    // 滤镜链:
    // 1. fps: 统一帧率
    // 2. scale & pad: 缩放并填充至最大分辨率
    // 3. draw text: 绘制文件名
    let chain = format!(
      "[{i}:v]fps={fps},scale={w}:{h}:force_original_aspect_ratio=decrease,pad={w}:{h}:(ow-iw)/2:(oh-ih)/2,setsar=1,drawtext=fontfile={font}:text={text}:fontcolor=white:fontsize=24:x=10:y=10:box=1:boxcolor=black@0.0[v{i}];[{i}:a]aresample=48000[a{i}];",
      i = i,
      fps = target_fps,
      w = target_w,
      h = target_h,
      font = font_path,
      text = clean_name
    );
    filter_complex.push_str(&chain);
  }

  // 拼接
  for i in 0..valid_tasks.len() {
    filter_complex.push_str(&format!("[v{i}][a{i}]", i = i));
  }
  filter_complex.push_str(&format!("concat=n={}:v=1:a=1[outv][outa]", valid_tasks.len()));

  writeln!(filter_file, "{}", filter_complex).map_err(|e: std::io::Error| e.to_string())?;
  filter_file.flush().map_err(|e| e.to_string())?;

  let filter_file_name_str = filter_file_name.to_string_lossy();
  args.push("-/filter_complex");
  args.push(filter_file_name_str.as_ref());
  args.push("-map");
  args.push("[outv]");
  args.push("-map");
  args.push("[outa]");
  args.extend_from_slice(&best_args);
  args.push("-y");
  args.push(output_path);

  log::info!("ffmpeg {}", args.join(" "));

  log::info!("--- fourth stage: concat video ---");

  let shell = app.shell();
  let (mut rx, _child) = shell.command("ffmpeg").args(args).spawn().map_err(|e| e.to_string())?;

  while let Some(event) = rx.recv().await {
    match event {
      CommandEvent::Stderr(data) => {
        let line = String::from_utf8_lossy(&data).trim().to_string();
        log::info!("{}", line);
      }
      CommandEvent::Terminated(status) => {
        if status.code == Some(0) {
          let _ = std::fs::remove_dir_all(temp_dir);
          log::info!("concat video success");
          break;
        }
        log::info!("concat video failed, exit code: {}", status.code.unwrap());
        return Err("concat video failed".to_string());
      }
      _ => {}
    }
  }

  Ok(())
}
