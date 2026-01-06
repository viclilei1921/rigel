<script setup lang="ts">
import type { VideoInfoType } from '../type'

import MdiClockOutline from '~icons/mdi/clock-outline'
import MdiFileVideo from '~icons/mdi/file-video'
import MdiHeadphones from '~icons/mdi/headphones'
import MdiSpeedometer from '~icons/mdi/speedometer'
import MdiTransitConnection from '~icons/mdi/transit-connection'
import MdiVideo from '~icons/mdi/video'
import MdiVideoHomeSystem from '~icons/mdi/video-home-system'

import { ExtractFilename } from '../../../utils'

type DefaultPropsType = {
  canClose?: boolean
  videoInfo?: VideoInfoType
}

withDefaults(defineProps<DefaultPropsType>(), {
  videoInfo: () => ({
    audio_codec: '',
    audio_sample_rate: 0,
    bitrate_kbps: 0,
    duration: 0,
    fps: 0,
    height: 0,
    path: '',
    video_codec: '',
    width: 0
  })
})

function formatTime(time: number) {
  const hours = Math.floor(time / 3600)
  const minutes = Math.floor((time - hours * 3600) / 60)
  const seconds = Math.floor(time - hours * 3600 - minutes * 60)
  return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
}
</script>

<template>
  <v-container dense class="py-1 mt-1 border rounded">
    <v-row class="ma-0">
      <v-col cols="3" class="pa-0">
        <v-list-item>
          <v-list-item-title class="d-flex align-center">
            <v-icon small :icon="MdiFileVideo" />
            <span>文件路径</span>
          </v-list-item-title>
          <v-list-item-subtitle class="text-truncate">
            {{ ExtractFilename(videoInfo.path) }}
          </v-list-item-subtitle>
        </v-list-item>
      </v-col>

      <v-col cols="3" class="pa-0">
        <v-list-item>
          <v-list-item-title class="d-flex align-center">
            <v-icon small :icon="MdiClockOutline" />
            <span>时长</span>
          </v-list-item-title>
          <v-list-item-subtitle>
            {{ formatTime(videoInfo.duration) }}
          </v-list-item-subtitle>
        </v-list-item>
      </v-col>

      <v-col cols="3" class="pa-0">
        <v-list-item>
          <v-list-item-title class="d-flex align-center">
            <v-icon small :icon="MdiVideo" />
            <span>分辨率</span>
          </v-list-item-title>
          <v-list-item-subtitle>
            {{ videoInfo.width }} x {{ videoInfo.height }}
          </v-list-item-subtitle>
        </v-list-item>
      </v-col>

      <v-col cols="3" class="pa-0">
        <v-list-item>
          <v-list-item-title>
            <v-icon small :icon="MdiSpeedometer" />
            <span>帧率</span>
          </v-list-item-title>
          <v-list-item-subtitle>
            {{ videoInfo.fps }} FPS
          </v-list-item-subtitle>
        </v-list-item>
      </v-col>
    </v-row>
    <v-divider />
    <v-row class="ma-0">
      <v-col cols="3" class="pa-0">
        <v-list-item>
          <v-list-item-title>
            <v-icon small :icon="MdiVideoHomeSystem" />
            <span>视频编码</span>
          </v-list-item-title>
          <v-list-item-subtitle>
            {{ videoInfo.video_codec }}
          </v-list-item-subtitle>
        </v-list-item>
      </v-col>

      <v-col cols="3" class="pa-0">
        <v-list-item>
          <v-list-item-title>
            <v-icon small :icon="MdiVideoHomeSystem" />
            <span>音频编码</span>
          </v-list-item-title>
          <v-list-item-subtitle>
            {{ videoInfo.audio_codec }}
          </v-list-item-subtitle>
        </v-list-item>
      </v-col>

      <v-col cols="3" class="pa-0">
        <v-list-item>
          <v-list-item-title>
            <v-icon small :icon="MdiHeadphones" />
            <span>音频采样率</span>
          </v-list-item-title>
          <v-list-item-subtitle>
            {{ videoInfo.audio_sample_rate }}
          </v-list-item-subtitle>
        </v-list-item>
      </v-col>

      <v-col cols="3" class="pa-0">
        <v-list-item>
          <v-list-item-title>
            <v-icon small :icon="MdiTransitConnection" />
            <span>比特率</span>
          </v-list-item-title>
          <v-list-item-subtitle>
            {{ videoInfo.bitrate_kbps }}
          </v-list-item-subtitle>
        </v-list-item>
      </v-col>
    </v-row>
  </v-container>
</template>
