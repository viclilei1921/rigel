use std::io::Write;
use std::{fs::File, path::Path};

use regex::Regex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};

use crate::utils::files::get_cache_temp_dir;
use crate::utils::font::get_default_font_path;
use crate::utils::{
  files::get_cache_dir,
  gpu::{get_gpu_info, GpuInfo},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoInfo {
  pub path: String,
  pub width: u32,
  pub height: u32,
  pub fps: f64,
  pub duration: f64,
  pub video_codec: String,
  pub audio_codec: String,
  pub audio_sample_rate: u32,
  pub bitrate_kbps: u32,
}

/// 进度事件数据结构
#[derive(Clone, Serialize, Deserialize)]
pub struct ProgressPayload {
  pub progress: f64,         // 进度百分比 (0-100)
  pub video_info: VideoInfo, // 视频总时长（秒）
  pub message: String,       // 进度消息
}

/// 完成事件数据结构
#[derive(Clone, Serialize, Deserialize)]
struct CompletionPayload {
  code: Option<i32>,
}

/// 事件数据结构
#[derive(Clone, Serialize, Deserialize)]
pub struct TimeSegment {
  pub start: String,    // 格式 "00:00:10.000" 或 秒数 "10"
  pub duration: String, // 格式 "00:00:20.000" 或 秒数 "20"
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
    // 1. 基础兼容性参数 (所有编码器通用)
    // -pix_fmt yuv420p: 强制 8位 色深，防止转码成 10位 导致浏览器黑屏
    // -tag:v hvc1: 苹果生态 (Safari/Finder) 识别 HEVC 的必要标签
    let common_args = vec!["-c:a", "aac", "-ac", "2", "-pix_fmt", "yuv420p", "-tag:v", "hvc1"];

    // 2. 根据硬件添加特定编码参数
    let encoder_args = match self {
      EncoderPreset::Nvidia(name) => vec![
        "-c:v",
        name.as_str(),
        // -cq: 恒定质量模式 (Constant Quality), 范围 1-51, 越小越清晰
        "-cq",
        "28",
        // -preset: p1(最快)-p7(最慢/质量最好), p4 是平衡点
        "-preset",
        "p4",
      ],
      EncoderPreset::Intel(name) => vec![
        "-c:v",
        name.as_str(),
        // -global_quality: ICQ 模式, 类似 CRF
        "-global_quality",
        "25",
        "-load_plugin",
        "hevc_hw", // 显式加载插件有时能避免报错
      ],
      EncoderPreset::Apple(name) => vec![
        "-c:v",
        name.as_str(),
        // -q:v: 质量控制, 0-100, 这里的 60 大约对应 CRF 26-28
        "-q:v",
        "60",
        // 确保 Apple 编码器不自动用 10bit
        "-profile:v",
        "main",
      ],
      EncoderPreset::Amd(name) => vec![
        "-c:v",
        name.as_str(),
        // AMD AMF 比较特殊，通常用 -rc cqp 来控制质量
        "-usage",
        "transcoding",
        "-rc",
        "cqp",
        "-qp_i",
        "28",
        "-qp_p",
        "28",
      ],
      EncoderPreset::Cpu(name) => vec![
        "-c:v",
        name.as_str(),
        // -crf: 软件编码标准质量控制
        "-crf",
        "28",
        // -preset: medium 是默认, fast 编码更快
        "-preset",
        "medium",
      ],
    };

    // 3. 转换 Vec<&str> 为 Vec<String> 并合并

    [&common_args[..], &encoder_args[..]].concat()
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

    parse_duration_str(time_str)
  } else {
    None
  }
}

/// 用于计算多个视频合并后的目标参数 <br>
/// return: (目标宽度, 目标高度, 目标帧率)
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

/// 将 HH:MM:SS.ms 转换为秒
fn parse_duration_str(duration_str: &str) -> Option<f64> {
  let parts: Vec<&str> = duration_str.split(':').collect();
  match parts.len() {
    4 => {
      let hours: f64 = parts[0].parse().ok()?;
      let minutes: f64 = parts[1].parse().ok()?;
      let seconds: f64 = parts[2].parse().ok()?;
      let millisecond: f64 = parts[3].parse().ok()?;
      Some(hours * 3600.0 + minutes * 60.0 + seconds + millisecond / 1000.0)
    }
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
}

/// 获取视频信息
#[tauri::command]
pub async fn get_video_info(app: AppHandle, video_path: &str) -> Result<VideoInfo, String> {
  let shell = app.shell();
  let (mut rx, _child) =
    shell.command("ffmpeg").args(&["-i", video_path, "-hide_banner"]).spawn().map_err(|e| e.to_string())?;
  let mut video_info = VideoInfo { path: video_path.to_string(), ..Default::default() };

  // 正则表达式
  // 匹配分辨率: 寻找 "Video:" 行后面的数字x数字
  let re_res = Regex::new(r"Video:.*?\s(\d{3,})x(\d{3,})").unwrap();

  // 匹配 FPS: 寻找 "23.98 fps" 或 "60 fps"
  let re_fps = Regex::new(r",\s*(\d+(?:\.\d+)?)\s*fps").unwrap();

  // 匹配时长和总码率: "Duration: 00:00:10.50, start: 0.000000, bitrate: 5466 kb/s"
  let re_dur_bitrate = Regex::new(r"Duration:\s*(\d{2}:\d{2}:\d{2}\.\d{2}).*?bitrate:\s*(\d+)\s*kb/s").unwrap();

  // 匹配视频编码: "Stream #0:0(eng): Video: hevc (Main)..." -> 捕获 "hevc"
  // 逻辑：匹配 "Video:" 后面的第一个单词
  let re_vcodec = Regex::new(r"Video:\s*([a-zA-Z0-9_]+)").unwrap();

  // 匹配音频编码和采样率: "Stream #0:1: Audio: aac (LC), 48000 Hz, stereo..."
  // 逻辑：匹配 "Audio:" 后面的单词，以及后面的 Hz 数值
  let re_acodec = Regex::new(r"Audio:\s*([a-zA-Z0-9_]+).*?(\d+)\s*Hz").unwrap();

  // 完全异步处理，不使用 block_on
  while let Some(event) = rx.recv().await {
    match event {
      CommandEvent::Stderr(data) => {
        let line = String::from_utf8_lossy(&data).trim().to_string();

        // 1. 解析基础视频信息 (分辨率, 编码)
        if line.contains("Video:") {
          if let Some(caps) = re_res.captures(&line) {
            video_info.width = caps[1].parse().unwrap_or(0);
            video_info.height = caps[2].parse().unwrap_or(0);
          }
          if let Some(caps) = re_fps.captures(&line) {
            video_info.fps = caps[1].parse().unwrap_or(0.0);
          }
          // 防止覆盖，只获取第一个视频流的编码
          if video_info.video_codec.is_empty() {
            if let Some(caps) = re_vcodec.captures(&line) {
              video_info.video_codec = caps[1].to_string();
            }
          }
        }

        // 2. 解析音频信息
        if line.contains("Audio:") && video_info.audio_codec.is_empty() {
          if let Some(caps) = re_acodec.captures(&line) {
            video_info.audio_codec = caps[1].to_string();
            video_info.audio_sample_rate = caps[2].parse().unwrap_or(0);
          }
        }

        // 3. 解析时长和码率
        if line.contains("Duration:") {
          if let Some(caps) = re_dur_bitrate.captures(&line) {
            // 解析时长字符串 (HH:MM:SS.mm)
            if let Some(duration) = parse_duration_str(&caps[1]) {
              video_info.duration = duration;
            }
            // 解析码率
            video_info.bitrate_kbps = caps[2].parse().unwrap_or(0);
          }
        }
      }
      CommandEvent::Terminated(status) => {
        if let Some(code) = status.code {
          if code == 0 || code == 1 {
            break;
          }
          return Err(format!("ffmpeg exited with status {}", code));
        }
        break;
      }
      _ => {}
    }
  }

  // 以视频时长为准
  if video_info.duration > 0.0 {
    return Ok(video_info);
  }

  Err("cannot get video duration".to_string())
}

/// 将视频转换成 mp4 格式
#[tauri::command]
pub async fn convert_video_to_mp4(app: AppHandle, video_path: &str, output_path: &str) -> Result<(), String> {
  let video_info = get_video_info(app.clone(), video_path).await.unwrap();

  let gpus = get_gpu_info().await.unwrap();
  let best = select_best_encoder(&gpus);
  let best_args = best.to_ffmpeg_args();

  let mut args = Vec::with_capacity(5 + best_args.len());

  args.push("-i");
  args.push(video_path);
  args.extend_from_slice(&best_args);
  args.push("-y");
  args.push(output_path);
  args.push("-hide_banner");

  // 获取 shell 实例
  let shell = app.shell();

  // 创建命令（注意：这里的 "ffmpeg" 必须在 capabilities 中配置）
  log::info!("ffmpeg {}", args.join(" "));
  let (mut rx, _child) = shell.command("ffmpeg").args(args).spawn().map_err(|e| e.to_string())?;

  // 异步处理输出流，不要使用 block_on
  while let Some(event) = rx.recv().await {
    match event {
      CommandEvent::Stderr(line) => {
        if let Some(current_time) = parse_time_from_ffmpeg_output(&line) {
          // 发射进度事件到前端
          let _ = app.emit(
            "ffmpeg-progress",
            ProgressPayload {
              progress: if video_info.duration > 0.0 { (current_time / video_info.duration) * 100.0 } else { 0.0 },
              video_info: video_info.clone(),
              message: format!("transcoding from {} to {}", video_path, output_path),
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

/// 裁剪和合并视频，来截取精彩的片段
#[tauri::command]
pub async fn create_highlight_video(
  app: AppHandle,
  video_path: &str,
  output_path: &str,
  segments: Vec<TimeSegment>,
) -> Result<(), String> {
  let video_info = get_video_info(app.clone(), video_path).await.unwrap();

  let mut temp_files = Vec::new();
  let temp_dir = get_cache_temp_dir(app.clone())?;

  let gpus = get_gpu_info().await.unwrap();
  let best = select_best_encoder(&gpus);
  let best_args = best.to_ffmpeg_args();

  for (i, seg) in segments.iter().enumerate() {
    let mut temp_dir = temp_dir.clone();
    temp_dir.push(format!("part_{}.mp4", i));
    let temp_name = temp_dir.to_string_lossy().into_owned();

    let mut args = Vec::with_capacity(9 + best_args.len());
    args.push("-ss");
    args.push(&seg.start);
    args.push("-i");
    args.push(video_path);
    args.push("-t");
    args.push(&seg.duration);
    args.extend_from_slice(&best_args);
    args.push("-y");
    args.push(&temp_name);
    args.push("-hide_banner");

    let duration = parse_duration_str(&seg.duration).unwrap();

    log::info!("ffmpeg {}", args.join(" "));
    let shell = app.shell();
    let (mut rx, _child) = shell.command("ffmpeg").args(args).spawn().map_err(|e| e.to_string())?;
    while let Some(event) = rx.recv().await {
      match event {
        CommandEvent::Stderr(data) => {
          if let Some(current_time) = parse_time_from_ffmpeg_output(&data) {
            // 发射进度事件到前端
            let _ = app.emit(
              "ffmpeg-progress",
              ProgressPayload {
                progress: if duration > 0.0 { (current_time / duration) * 100.0 } else { 0.0 },
                video_info: video_info.clone(),
                message: format!("split segment {}: {}", seg.start, seg.duration),
              },
            );
          }
        }
        CommandEvent::Terminated(status) => {
          if status.code == Some(0) {
            temp_files.push(temp_name);
          }
          let _ = app.emit("ffmpeg-complete", CompletionPayload { code: status.code });
          break;
        }
        _ => {}
      }
    }
  }

  if temp_files.is_empty() {
    return Err("not cut video".to_string());
  }

  // 2. 创建 concat 列表文件
  let list_file_name = temp_dir.clone().join("concat_list.txt");
  let mut list_file = File::create(&list_file_name).map_err(|e| e.to_string())?;

  for path in &temp_files {
    // FFmpeg concat 列表格式: file '文件名'
    // 注意 Windows 路径转义，这里简单处理
    writeln!(list_file, "file '{}'", path).map_err(|e: std::io::Error| e.to_string())?;
  }
  list_file.flush().map_err(|e| e.to_string())?;

  let shell = app.shell();
  let file_path = list_file_name.to_string_lossy().into_owned();
  let args =
    Vec::from(["-f", "concat", "-safe", "0", "-i", &file_path, "-c", "copy", "-y", output_path, "-hide_banner"]);
  let (mut rx, _child) = shell.command("ffmpeg").args(args).spawn().map_err(|e| e.to_string())?;

  while let Some(event) = rx.recv().await {
    match event {
      CommandEvent::Terminated(status) => {
        let _ = std::fs::remove_file(list_file_name);
        let _ = std::fs::remove_dir_all(temp_dir); // 删除临时文件夹

        let _ = app.emit("ffmpeg-complete", CompletionPayload { code: status.code });
        break;
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

  for input in &inputs {
    match get_video_info(app.clone(), &input).await {
      Ok(meta) => {
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
  let target_duration = videos_info.iter().map(|m| m.duration).sum::<f64>();

  log::info!(" target resolute: {}x{} | fps: {}", target_w, target_h, target_fps);

  let gpus = get_gpu_info().await.unwrap();
  let best = select_best_encoder(&gpus);
  let mut best_args = best.to_ffmpeg_args();

  best_args.pop();
  best_args.pop();
  best_args.push("-an");

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
    args.push("-i");
    args.push(input_path);

    // 获取文件名
    let filename = Path::new(input_path).file_name().unwrap().to_string_lossy();
    // 清洗文件名，防止破坏 FFmpeg 语法
    let clean_name = filename.replace(":", "\\:").replace("'", "");
    let target_w_f: f64 = target_w as f64;
    let target_h_f: f64 = target_h as f64;

    let font_size = 24.0 * target_w_f / 1920.0;
    let x = 10.0 * target_w_f / 1920.0;
    let y = 10.0 * target_h_f / 1080.0;
    // 绘制文件名
    let drawtext_part = format!(
      ",drawtext=fontfile={}:text={}:fontcolor=white:fontsize={}:x={}:y={}:box=1:boxcolor=black@0.0",
      font_path, clean_name, font_size as u32, x as u32, y as u32
    );

    // 滤镜链:
    // 1. fps: 统一帧率
    // 2. scale='trunc(iw*sar/2)*2':'trunc(ih/2)*2' -> 将视频按照自身的 SAR 物理缩放为正方形像素分辨率
    // 3. setsar=1 -> 标记现在的像素是正方形了
    // 4. scale={w}:{h}:force_original_aspect_ratio=decrease -> 原本的目标尺寸缩放
    // 5. pad={w}:{h}:(ow-iw)/2:(oh-ih)/2 -> 填充至最大分辨率
    // [v{i}];[{i}:a]aresample=48000[a{i}]; -> 音频
    // 8. aresample=48000 -> 音频采样率
    let chain = format!(
      "[{i}:v]fps={fps},scale='trunc(iw*sar/2)*2':'trunc(ih/2)*2',setsar=1,scale={w}:{h}:force_original_aspect_ratio=decrease,pad={w}:{h}:(ow-iw)/2:(oh-ih)/2,setsar=1{text_filter}[v{i}];[{i}:a]aresample=48000,aformat=sample_fmts=fltp:channel_layouts=stereo[a{i}];",
      i = i,
      fps = target_fps,
      w = target_w,
      h = target_h,
      text_filter = drawtext_part
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

  let shell = app.shell();
  let (mut rx, _child) = shell.command("ffmpeg").args(args).spawn().map_err(|e| e.to_string())?;

  while let Some(event) = rx.recv().await {
    match event {
      CommandEvent::Stderr(line) => {
        // log::info!("{}", String::from_utf8_lossy(&line));
        if let Some(current_time) = parse_time_from_ffmpeg_output(&line) {
          let progress = if target_duration > 0.0 { (current_time / target_duration) * 100.0 } else { 0.0 };

          let mut dur = 0.0;
          let mut video_info_target = valid_tasks.get(0).unwrap().1.clone();
          let mut video_name = Path::new(&video_info_target.path).file_name().unwrap().to_string_lossy().into_owned();

          for (_, video_info) in &valid_tasks {
            dur += video_info.duration;
            if current_time < dur {
              video_info_target = video_info.clone();
              video_name = Path::new(&video_info_target.path).file_name().unwrap().to_string_lossy().into_owned();
              break;
            }
          }

          // 发射进度事件到前端
          let _ = app.emit(
            "ffmpeg-progress",
            ProgressPayload {
              progress,
              video_info: video_info_target,
              message: format!("concat video: {}", video_name),
            },
          );
        }
      }
      CommandEvent::Terminated(status) => {
        let _ = app.emit("ffmpeg-complete", CompletionPayload { code: status.code });
        let _ = std::fs::remove_dir_all(temp_dir);
        break;
      }
      _ => {}
    }
  }

  Ok(())
}

/// 智能追加视频 (防卡顿优化版)
/// 策略：MP4 -> TS -> Concat -> MP4
/// 1. Base Video -> Remux to .ts (不重编码，超快)
/// 2. New Videos -> Transcode to .ts (统一参数)
/// 3. Concat all .ts files -> Remux to .mp4
#[tauri::command]
pub async fn append_smart(
  app: AppHandle,
  base_path: &str,
  new_inputs: Vec<&str>,
  output_path: &str,
) -> Result<(), String> {
  if new_inputs.is_empty() {
    return Err("no new videos to append".to_string());
  }

  let base_info = get_video_info(app.clone(), base_path).await?;
  let temp_dir = get_cache_temp_dir(app.clone())?;
  let mut ts_files: Vec<String> = Vec::new();

  // ==========================================
  // 步骤 1: 将基准视频无损封装为 TS (Remux)
  // ==========================================
  let base_ts_path = temp_dir.join("part_base.ts").to_string_lossy().into_owned();

  // 确定比特流过滤器 (Bitstream Filter)
  // MP4 转 TS 需要将数据从 AVCC/HVCC 转换为 Annex-B 格式，否则会黑屏或报错
  let bsf_filter = if base_info.video_codec.contains("hevc") || base_info.video_codec.contains("h265") {
    "hevc_mp4toannexb"
  } else {
    "h264_mp4toannexb"
  };

  let remux_args = vec![
    "-i",
    base_path,
    "-c",
    "copy", // 核心：只复制流，不重新编码
    "-bsf:v",
    bsf_filter, // 关键：解决卡顿和兼容性的核心参数
    "-f",
    "mpegts",
    "-y",
    &base_ts_path,
    "-hide_banner",
  ];

  log::info!("Remuxing base to TS...");
  let shell = app.shell();
  let (mut rx, _) = shell.command("ffmpeg").args(remux_args).spawn().map_err(|e| e.to_string())?;

  // 等待基准视频处理完成
  while let Some(event) = rx.recv().await {
    if let CommandEvent::Terminated(status) = event {
      if status.code != Some(0) {
        return Err("Failed to remux base video".to_string());
      }
      break;
    }
  }
  ts_files.push(base_ts_path);

  // ==========================================
  // 步骤 2: 处理新视频并转码为 TS
  // ==========================================
  let gpus = get_gpu_info().await.unwrap();
  let best = select_best_encoder(&gpus);
  let best_args = best.to_ffmpeg_args();
  let font_path = get_default_font_path();

  for (i, input_path) in new_inputs.iter().enumerate() {
    let current_ts_path = temp_dir.join(format!("part_new_{}.ts", i)).to_string_lossy().into_owned();
    let input_info = get_video_info(app.clone(), input_path).await.unwrap_or_default();

    // 画面处理滤镜 (同之前逻辑)
    let filename = Path::new(input_path).file_name().unwrap().to_string_lossy();
    let clean_name = filename.replace(":", "\\:").replace("'", "");
    let target_w_f = base_info.width as f64;
    let target_h_f = base_info.height as f64;
    let font_size = 24.0 * target_w_f / 1920.0;
    let x = 10.0 * target_w_f / 1920.0;
    let y = 10.0 * target_h_f / 1080.0;
    let sample_rate = if base_info.audio_sample_rate > 0 { base_info.audio_sample_rate } else { 48000 };

    let drawtext = format!(
      ",drawtext=fontfile={}:text={}:fontcolor=white:fontsize={}:x={}:y={}:box=1:boxcolor=black@0.0",
      font_path, clean_name, font_size as u32, x as u32, y as u32
    );

    let filter_complex = format!(
            "[0:v]fps={fps},scale='trunc(iw*sar/2)*2':'trunc(ih/2)*2',setsar=1,scale={w}:{h}:force_original_aspect_ratio=decrease,pad={w}:{h}:(ow-iw)/2:(oh-ih)/2,setsar=1{text}[v];[0:a]aresample={ar},aformat=sample_fmts=fltp:channel_layouts=stereo[a]",
            fps = base_info.fps, w = base_info.width, h = base_info.height, text = drawtext, ar = sample_rate
        );

    let mut args = vec!["-i", input_path, "-filter_complex", &filter_complex, "-map", "[v]", "-map", "[a]"];

    // 编码参数
    args.extend_from_slice(&best_args);

    // TS 特定参数
    args.push("-bsf:v");
    args.push(bsf_filter); // 同样应用 Annex-B 滤镜
    args.push("-f");
    args.push("mpegts");
    args.push("-y");
    args.push(&current_ts_path);
    args.push("-hide_banner");

    log::info!("Transcoding part {} to TS...", i);
    let shell = app.shell();
    let (mut rx, _) = shell.command("ffmpeg").args(args).spawn().map_err(|e| e.to_string())?;

    while let Some(event) = rx.recv().await {
      match event {
        CommandEvent::Stderr(line) => {
          if let Some(current_time) = parse_time_from_ffmpeg_output(&line) {
            let progress = if input_info.duration > 0.0 { (current_time / input_info.duration) * 100.0 } else { 0.0 };
            let _ = app.emit(
              "ffmpeg-progress",
              ProgressPayload {
                progress,
                video_info: input_info.clone(),
                message: format!("Processing part {}/{}", i + 1, new_inputs.len()),
              },
            );
          }
        }
        CommandEvent::Terminated(status) => {
          if status.code == Some(0) {
            ts_files.push(current_ts_path.clone());
          }
          break;
        }
        _ => {}
      }
    }
  }

  // ==========================================
  // 步骤 3: Concat 协议合并所有 TS 并转回 MP4
  // ==========================================
  // 这里的 concat 字符串格式为: "concat:part1.ts|part2.ts|part3.ts"
  // 注意：TS 文件可以直接用 concat: 协议，不需要创建列表文件，但文件多时列表文件更安全
  // 既然我们已经有文件系统操作，还是用列表文件最稳妥 (-f concat)

  let list_file_name = temp_dir.join("ts_concat_list.txt");
  let mut list_file = File::create(&list_file_name).map_err(|e| e.to_string())?;
  for path in &ts_files {
    writeln!(list_file, "file '{}'", path.replace("'", "'\\''")).map_err(|e| e.to_string())?;
  }
  list_file.flush().map_err(|e| e.to_string())?;

  let list_file_path_str = list_file_name.to_string_lossy().into_owned();

  // 合并并转回 MP4
  let concat_args = vec![
    "-f",
    "concat",
    "-safe",
    "0",
    "-i",
    &list_file_path_str,
    "-c",
    "copy", // 此时 TS 到 MP4 也是流复制，不重编码
    "-tag:v",
    "hvc1",
    "-bsf:a",
    "aac_adtstoasc", // 关键：TS 中的 AAC 转换回 MP4 需要这个滤镜修复音频头
    "-y",
    output_path,
    "-hide_banner",
  ];

  log::info!("Final merge (TS -> MP4)...");
  let shell = app.shell();
  let (mut rx, _) =
    shell.command("ffmpeg").args(concat_args).spawn().map_err(|e: tauri_plugin_shell::Error| e.to_string())?;

  while let Some(event) = rx.recv().await {
    if let CommandEvent::Terminated(status) = event {
      // 清理
      let _ = std::fs::remove_file(list_file_name);
      for tmp in ts_files {
        let _ = std::fs::remove_file(tmp);
      }
      let _ = std::fs::remove_dir(temp_dir);

      let _ = app.emit("ffmpeg-complete", CompletionPayload { code: status.code });
      break;
    }
  }

  Ok(())
}
