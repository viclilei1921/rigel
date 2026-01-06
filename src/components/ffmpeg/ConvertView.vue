<script setup lang="ts">
import type { UnlistenFn } from '@tauri-apps/api/event'
import type { ProgressPayloadInterface } from '../../types/listen'
import type { VideoInfoType } from './type'

import { onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/plugin-dialog'
import FluentEmojiHighContrastOpenFileFolder from '~icons/fluent-emoji-high-contrast/open-file-folder'
import MdiFolderOpenOutline from '~icons/mdi/folder-open-outline'
import MiDelete from '~icons/mi/delete'

import { logger } from '../../utils'
import VideoInfo from './common/VideoInfo.vue'

type VideoConvertType = VideoInfoType & {
  outputPath: string
  message: string
  progress: number
}

const videoArr = ref<VideoConvertType[]>([])
const selecting = ref(false)
const converting = ref(-1)

async function handleSelectVideo() {
  if (selecting.value) {
    return
  }

  selecting.value = true
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: 'Video',
        extensions: ['mp4', 'mov', 'avi', 'wmv', 'flv', 'mkv', 'webm', 'mpeg', 'mpg']
      }
    ]
  })

  if (!selected) {
    selecting.value = false
    return
  }

  if (videoArr.value.some(item => item.path === selected)) {
    selecting.value = false
    return
  }

  const info = await invoke('get_video_info', {
    videoPath: selected
  }).catch((e) => {
    logger.error(e)
    return null
  })

  if (!info) {
    selecting.value = false
    return
  }

  videoArr.value.push({
    outputPath: `${selected.split('.').slice(0, -1).join('.')}_converted.mp4`,
    message: '',
    progress: 0,
    ...info as VideoInfoType
  })

  selecting.value = false
  // videoPlayPath.value = convertFileSrc(info.path)
}

async function handleConvert() {
  if (!videoArr.value.length) {
    return
  }

  for (let i = 0; i < videoArr.value.length; i++) {
    const item = videoArr.value[i]

    converting.value = i

    if (item.progress >= 100) {
      continue
    }

    await invoke('convert_video_to_mp4', {
      videoPath: item.path,
      outputPath: item.outputPath
    }).catch((e) => {
      logger.error(e)
    })
  }

  converting.value = -1
}

function handleOpenFolder(path: string) {
  invoke('reveal_in_explorer', {
    path
  }).catch((e) => {
    logger.error(e)
  })
}

let unlistenProgress: UnlistenFn | null = null
let unlistenComplete: UnlistenFn | null = null

async function initEvent() {
  unlistenProgress = await listen('ffmpeg-progress', ({ payload }: { payload: ProgressPayloadInterface }) => {
    const video = videoArr.value[converting.value]
    if (!video) {
      return
    }
    video.progress = payload.progress
    video.message = payload.message
  })

  // 监听完成事件
  unlistenComplete = await listen('ffmpeg-complete', ({ payload }) => {
    const video = videoArr.value[converting.value]
    if (!video) {
      return
    }
    video.progress = 100
    video.message = `完成 code: ${(payload as any).code}`
  })
}

function closeEvent() {
  unlistenProgress?.()
  unlistenComplete?.()
}

initEvent()

onUnmounted(closeEvent)
</script>

<template>
  <v-container>
    <v-card class="pa-4">
      <v-card-title>视频转换</v-card-title>
      <v-card-subtitle>使用 ffmpeg 将视频转换为 mp4 格式</v-card-subtitle>
      <v-card-text>
        <v-btn
          class="me-2" height="40" variant="flat" width="80" :icon="FluentEmojiHighContrastOpenFileFolder"
          :loading="selecting"
          @click="handleSelectVideo"
        />
      </v-card-text>
      <v-card-item v-for="(video, index) of videoArr" :key="index" class="position-relative">
        <v-icon-btn color="error" class="position-absolute top-0 right-0" :icon="MiDelete" @click="videoArr.splice(index, 1)" />
        <VideoInfo :video-info="video" />
        <v-text-field v-model="video.outputPath" label="输出路径 (可选)" placeholder="默认保存在原文件夹" variant="outlined" class="mt-4" />
        <v-container class="mt-0">
          <v-progress-linear v-if="video.progress > 0" color="blue-lighten-3" :model-value="video.progress" rounded />
          <div>{{ video.message }}</div>
          <v-btn v-if="video.progress >= 100" class="me-2" height="40" variant="flat" width="80" :icon="MdiFolderOpenOutline" @click="handleOpenFolder(video.outputPath)" />
        </v-container>
      </v-card-item>
      <v-card-actions>
        <v-spacer />
        <v-btn color="primary" :loading="converting > -1" :disabled="!videoArr.length" @click="handleConvert">
          开始转换
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
