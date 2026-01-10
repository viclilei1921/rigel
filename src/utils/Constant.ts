import type { InjectionKey, Ref } from 'vue'
import type { VideoPlayerInfoType } from '../types/video'

export const VIDEO_PLAYER_KEY: InjectionKey<Ref<VideoPlayerInfoType>> = Symbol('video_player')
