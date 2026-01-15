/** 视频信息 */
export interface VideoInfoInterface {
  path: string;
  width: number;
  height: number;
  duration: number;
  fps: number;
}

/**
 * ffmpeg 进度事件
 */
export interface ProgressPayloadInterface {
  progress: number;
  message: string;
  videoInfo: VideoInfoInterface;
}

/**
 * 视频信息
 */
export type VideoInfoType = {
  audio_codec: string;
  audio_sample_rate: number;
  bitrate_kbps: number;
  duration: number;
  fps: number;
  height: number;
  path: string;
  video_codec: string;
  width: number;
};

/**
 * 视频片段
 */
export type VideoSegmentType = {
  start: number;
  end: number;
};
