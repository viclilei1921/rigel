<script setup lang="ts">
import type { UnlistenFn } from '@tauri-apps/api/event'
import type { ProgressPayloadInterface } from '../../types/listen'
import type { VideoInfoType } from './type'

import { onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/plugin-dialog'
import FluentEmojiHighContrastOpenFileFolder from '~icons/fluent-emoji-high-contrast/open-file-folder'
import LineMdPlus from '~icons/line-md/plus'
import MdiFolderOpenOutline from '~icons/mdi/folder-open-outline'
import MiDelete from '~icons/mi/delete'

import { FormatTime, logger } from '../../utils'
import TimeInput from '../common/TimeInput.vue'
import VideoInfo from './common/VideoInfo.vue'

type VideoSegmentType = {
  start: number
  end: number
}

const videoInfo = ref<VideoInfoType | null>(null)
const selecting = ref(false)
const clips = ref<VideoSegmentType[]>([])
const outputPath = ref('')
const progress = ref(0)
const message = ref('')
const highlighting = ref(false)

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

  videoInfo.value = info as VideoInfoType
  outputPath.value = `${selected.split('.').slice(0, -1).join('.')}_.mp4`
  selecting.value = false
}

function addClip() {
  const segment = { start: 0, end: 5 }
  const len = clips.value.length

  if (len > 0) {
    segment.start = clips.value[len - 1].end + 5
    segment.end = segment.start + 5
  }

  clips.value.push(segment)
}

function removeClip(index: number) {
  clips.value.splice(index, 1)
}

async function handleHighlight() {
  if (!videoInfo.value) {
    return
  }

  highlighting.value = true

  await invoke('create_highlight_video', {
    videoPath: videoInfo.value.path,
    outputPath: outputPath.value,
    segments: clips.value.map(({ start, end }) => ({ start: FormatTime(start), duration: FormatTime(end - start) }))
  }).catch((e) => {
    logger.error(e)
  })

  highlighting.value = false
}

function handleOpenFolder() {
  invoke('reveal_in_explorer', {
    path: outputPath.value
  }).catch((e) => {
    logger.error(e)
  })
}

function handleReset() {
  videoInfo.value = null
  clips.value = []
  outputPath.value = ''
  progress.value = 0
  message.value = ''
  highlighting.value = false
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
      <v-card-title>精彩剪辑合成</v-card-title>
      <v-card-subtitle>使用 ffmpeg 将视频中精彩部分提取出来合成为一个视频</v-card-subtitle>
      <v-card-text v-if="!videoInfo">
        <v-btn
          class="me-2" height="40" variant="flat" width="80" :icon="FluentEmojiHighContrastOpenFileFolder"
          :loading="selecting"
          @click="handleSelectVideo"
        />
      </v-card-text>
      <v-card-text v-if="videoInfo" class="position-relative">
        <v-icon-btn color="error" class="position-absolute top-0 right-0" :icon="MiDelete" @click="handleReset" />
        <v-card-item>
          <VideoInfo :video-info="videoInfo" />
          <v-text-field v-model="outputPath" label="输出路径 (可选)" placeholder="默认保存在原文件夹" variant="outlined" class="mt-4" />
          <v-container class="mt-0">
            <v-progress-linear v-show="progress > 0" color="blue-lighten-3" :model-value="progress" rounded />
            <div>{{ message }}</div>
            <v-btn v-if="progress >= 100" class="me-2" height="40" variant="flat" width="80" :icon="MdiFolderOpenOutline" @click="handleOpenFolder()" />
          </v-container>
        </v-card-item>

        <div class="d-flex justify-space-between align-center">
          <span class="text-h6">剪辑片段列表</span>
          <v-btn :prepend-icon="LineMdPlus" size="small" @click="addClip">
            添加片段
          </v-btn>
        </div>

        <v-row v-for="(clip, index) in clips" :key="index" class="mt-0 align-center">
          <v-col cols="4">
            <TimeInput label="开始时间 HH:MM:SS" :pri-time="clip.start" density="compact" hide-details @change="(t) => clip.start = t" />
          </v-col>
          <v-col cols="4">
            <TimeInput label="结束时间 HH:MM:SS" :pri-time="clip.end" density="compact" hide-details @change="(t) => clip.end = t" />
          </v-col>
          <v-col cols="2">
            <v-btn :icon="MiDelete" variant="text" color="error" @click="removeClip(index)" />
          </v-col>
        </v-row>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn color="primary" :loading="highlighting" :disabled="!videoInfo || clips.length === 0" @click="handleHighlight">
          生成精彩合集
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
