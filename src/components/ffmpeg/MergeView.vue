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

const videoArr = ref<VideoInfoType[]>([])
const selecting = ref(false)
const outputPath = ref('')
const progress = ref(0)
const message = ref('')

async function handleSelectVideo() {
  if (selecting.value) {
    return
  }

  selecting.value = true
  const selectedArr = await open({
    multiple: true,
    filters: [
      {
        name: 'Video',
        extensions: ['mp4', 'mov', 'avi', 'wmv', 'flv', 'mkv', 'webm', 'mpeg', 'mpg']
      }
    ]
  })

  if (!selectedArr || selectedArr.length === 0) {
    selecting.value = false
    return
  }

  for (let i = 0; i < selectedArr.length; i++) {
    const item = selectedArr[i]

    const info = await invoke('get_video_info', {
      videoPath: item
    }).catch((e) => {
      logger.error(e)
      return null
    })

    if (info) {
      videoArr.value.push(info as VideoInfoType)
    }
  }

  if (videoArr.value.length) {
    outputPath.value = `${videoArr.value[0].path.split('.').slice(0, -1).join('.')}_.mp4`
  }

  selecting.value = false
}

async function handleMerge() {
  if (!videoArr.value.length) {
    return
  }

  await invoke('merge_smart', {
    inputs: videoArr.value.map(item => item.path),
    outputPath: outputPath.value
  }).catch((e) => {
    logger.error(e)
  })
}

function handleOpenFolder() {
  invoke('reveal_in_explorer', {
    path: outputPath.value
  }).catch((e) => {
    logger.error(e)
  })
}

let unlistenProgress: UnlistenFn | null = null
let unlistenComplete: UnlistenFn | null = null

async function initEvent() {
  unlistenProgress = await listen('ffmpeg-progress', ({ payload }: { payload: ProgressPayloadInterface }) => {
    progress.value = payload.progress
    message.value = payload.message
  })

  // 监听完成事件
  unlistenComplete = await listen('ffmpeg-complete', ({ payload }) => {
    progress.value = 100
    message.value = `完成 code: ${(payload as any).code}`
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
      <v-card-title>视频合成</v-card-title>
      <v-card-subtitle>使用 ffmpeg 将多个视频合成为一个视频, 并将视频名称绘制在左上角, 帧率使用中位数, 尺寸用最大尺寸</v-card-subtitle>
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
      </v-card-item>
      <v-container v-if="videoArr.length" class="mt-0">
        <v-text-field v-model="outputPath" label="输出路径 (可选)" placeholder="默认保存在原文件夹" variant="outlined" class="mt-4" />
        <v-progress-linear v-show="progress > 0" color="blue-lighten-3" :model-value="progress" rounded />
        <div>{{ message }}</div>
        <v-btn v-if="progress >= 100" class="me-2" height="40" variant="flat" width="80" :icon="MdiFolderOpenOutline" @click="handleOpenFolder()" />
      </v-container>
      <v-card-actions>
        <v-spacer />
        <v-btn
          color="primary"
          :loading="progress > 0 && progress < 100"
          :disabled="!videoArr.length"
          @click="handleMerge"
        >
          开始合成
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
