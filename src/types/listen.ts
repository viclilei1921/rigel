/** 视频信息 */
export interface VideoInfoInterface {
  path: string
  width: number
  height: number
  duration: number
  fps: number
}

/**
 * ffmpeg 进度事件
 */
export interface ProgressPayloadInterface {
  progress: number
  message: string
  videoInfo: VideoInfoInterface
}
