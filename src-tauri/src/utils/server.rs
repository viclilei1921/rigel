// 基于 Tokio 和 Hyper 构建。负责：定义路由 (/video)、解析 HTTP 请求头（如 Range）、封装 HTTP 响应（状态码、Header、Body）
use axum::{
  body::Body,
  extract::{Request, State},
  http::{header, HeaderMap, HeaderValue, StatusCode},
  response::{IntoResponse, Response},
  routing::get,
  Router,
};
// 提供了 Bytes 和 BytesMut 类型。它们是比 Vec<u8> 更高效的字节容器，支持零拷贝切片。
use bytes::{Bytes, BytesMut};
use std::{io::SeekFrom, path::PathBuf, sync::Arc};
use tauri::async_runtime;
// 负责：监听 TCP 端口 (TcpListener)、异步读取文件 (File)、处理并发任务 (spawn)
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::{fs::File, sync::oneshot};

use crate::utils::crypto::{derive_key, encrypt_decrypt_at_offset, SALT_LEN};
// 提供了 StreamExt trait。Rust 标准库对 Stream（异步流）的支持还很少
use futures_util::StreamExt;
// 它提供了 ReaderStream，把“文件读取器”转换成了“数据流”，这样才能通过 HTTP 发送出去
use tokio_util::io::ReaderStream;

pub struct AppState {
  pub password: String,
  pub video_path: PathBuf,
}

/// 启动内部流媒体服务器 <br>
/// shutdown_rx: 这是一个“遥控器”。当你的主程序（比如 Tauri 窗口关闭时）发送信号，这个服务器就会优雅退出，停止占用资源。
pub async fn start_server(password: String, video_path: PathBuf, shutdown_rx: oneshot::Receiver<()>) -> u16 {
  let state = Arc::new(AppState { password, video_path });

  let app = Router::new().route("/video", get(video_handler)).with_state(state);

  // 端口写 0 是一个系统约定，意思是“操作系统你帮我随便分一个没人在用的端口”
  let addr = "127.0.0.1:0";
  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

  let port = listener.local_addr().unwrap().port();
  log::info!("Internal streaming server running on http://127.0.0.1:{}", port);

  // 使用 with_graceful_shutdown 监听关闭信号
  async_runtime::spawn(async move {
    axum::serve(listener, app)
      .with_graceful_shutdown(async {
        shutdown_rx.await.ok();
        log::info!("Shutting down internal streaming server");
      })
      .await
      .unwrap();
  });

  port
}

/// 视频请求的总入口。当播放器请求 /video 时，进入此函数
async fn video_handler(State(state): State<Arc<AppState>>, req: Request) -> impl IntoResponse {
  let mut file = match File::open(&state.video_path).await {
    Ok(file) => file,
    Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to open file: {}", e)).into_response(),
  };

  let mut salt = [0u8; SALT_LEN];
  if let Err(e) = file.read_exact(&mut salt).await {
    return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read salt: {}", e)).into_response();
  }

  let key = derive_key(&state.password, &salt);
  let metadata = file.metadata().await.unwrap();
  let total_file_size = metadata.len();
  let video_data_size = total_file_size - SALT_LEN as u64;

  // 视频播放器通常不会一次请求整个文件，而是发送 Range: bytes=0-1024 这样的头，以此实现“拖动进度条”和“分段缓冲”。
  let range = req.headers().get(header::RANGE).and_then(|v| v.to_str().ok());

  if let Some(range_str) = range {
    // 如果有 Range 头，进入 handle_range_request（绝大多数情况）
    handle_range_request(file, video_data_size, range_str, key).await
  } else {
    // 否则直接流式传输整个文件
    let mut current_offset = 0u64;
    let stream = ReaderStream::new(file).map(move |chunk: Result<Bytes, std::io::Error>| match chunk {
      Ok(bytes) => {
        let mut data = BytesMut::from(&bytes[..]);
        encrypt_decrypt_at_offset(&mut data, current_offset, &key);
        current_offset += data.len() as u64;
        Ok::<_, std::io::Error>(data.freeze())
      }
      Err(e) => Err(e),
    });

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("video/mp4"));
    headers.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));
    headers.insert(header::CONTENT_LENGTH, HeaderValue::from(video_data_size));

    (StatusCode::OK, headers, Body::from_stream(stream)).into_response()
  }
}

/// 1. 用户在播放器拖动进度条到 50%。
/// 2. 播放器发送 HTTP 请求：GET /video, Range: bytes=50000-。
/// 3. Axum 收到请求，调用 video_handler。
/// 4. Rust 代码计算：物理文件需要跳过 50000 + SALT_LEN 字节。
/// 5. Tokio file.seek(...) 跳到物理位置。
/// 6. ReaderStream 读取一块 64KB 的加密数据。
/// 7. Map 闭包 拿着密码和 offset 对这 64KB 进行解密。
/// 8. Axum 将解密后的 64KB 发回给播放器。
/// 9. 播放器 以为自己在看普通 MP4，实际上是在看实时解密流。
async fn handle_range_request(mut file: File, video_size: u64, range_str: &str, key: [u8; 32]) -> Response {
  // 1. 解析请求范围
  let range_value = range_str.replace("bytes=", "");
  let parts: Vec<&str> = range_value.split('-').collect();
  // ...解析出 start 和 end ...
  let start = parts[0].parse::<u64>().unwrap_or(0);
  let end = parts.get(1).and_then(|s| s.parse::<u64>().ok()).filter(|&e| e < video_size).unwrap_or(video_size - 1);

  if start >= video_size {
    return (StatusCode::RANGE_NOT_SATISFIABLE, "Range Not Satisfiable").into_response();
  }

  let range_len = end - start + 1;
  // 2. 文件定位（这是关键点！）
  // 物理文件的 offset = 逻辑请求的 start + 头部盐的长度
  file.seek(SeekFrom::Start(start + SALT_LEN as u64)).await.unwrap();

  // 流加密算法（如 CTR 模式或 XOR）通常依赖数据在文件中的位置。第 100 个字节的解密方式和第 200 个字节不同。所以代码里维护了一个 current_offset。
  let mut current_offset = start;
  let mut remaining = range_len;

  // 3. 创建加密流
  // ReaderStream 把文件变成了水流，一块一块地流出来
  // .map() 就像在这个水管上装了一个滤网。每一块数据流过时，都会经过 encrypt_decrypt_at_offset 处理。
  // 处理完的数据直接发给 HTTP 响应，内存中只有这 64KB 的明文，非常安全且节省内存。
  let stream =
    ReaderStream::with_capacity(file, 64 * 1024).map(move |chunk: Result<Bytes, std::io::Error>| match chunk {
      Ok(bytes) => {
        // ... 截取需要的长度 ...
        let take_len = std::cmp::min(bytes.len() as u64, remaining) as usize;
        if take_len == 0 {
          return Ok::<_, std::io::Error>(Bytes::new());
        }
        let mut data = BytesMut::from(&bytes[..take_len]);

        // 4. 实时解密！
        // 拿到这一块密文数据，根据当前的 offset 进行解密
        encrypt_decrypt_at_offset(&mut data, current_offset, &key);

        // 更新 offset，准备解密下一块
        current_offset += take_len as u64;
        remaining -= take_len as u64;

        // 发送给浏览器
        Ok::<_, std::io::Error>(data.freeze())
      }
      Err(e) => Err(e),
    });

  let mut headers = HeaderMap::new();
  headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("video/mp4"));
  headers.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));
  headers.insert(
    header::CONTENT_RANGE,
    HeaderValue::from_str(&format!("bytes {}-{}/{}", start, end, video_size)).unwrap(),
  );
  headers.insert(header::CONTENT_LENGTH, HeaderValue::from(range_len));

  (StatusCode::PARTIAL_CONTENT, headers, Body::from_stream(stream)).into_response()
}
