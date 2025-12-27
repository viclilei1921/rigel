<script setup lang="ts">
import type { UnlistenFn } from '@tauri-apps/api/event'

import { onUnmounted, ref } from 'vue'
import { convertFileSrc, invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/plugin-dialog'
import FluentScreenCut20Filled from '~icons/fluent/screen-cut-20-filled'
import HugeiconsFolderTransfer from '~icons/hugeicons/folder-transfer'
import MaterialIconThemeFolderSvgOpen from '~icons/material-icon-theme/folder-svg-open'
import RiMergeCellsHorizontal from '~icons/ri/merge-cells-horizontal'

import { logger } from '../utils'

const urls = ref<[string, string][]>([])

const selectSvg = ref('')
const svgPath = ref('')

const width = ref(600)
const height = ref(600)

const time = ref('')

const segments = ref<{
  start: string
  end: string
}[]>([])

async function handleSelect() {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: 'SVG',
        extensions: ['svg']
      }
    ]
  })

  if (!selected) {
    return
  }

  logger.info(selected)
  selectSvg.value = selected
  svgPath.value = convertFileSrc(selected)
}

async function handleConvert() {
  if (selectSvg.value === '') {
    return
  }

  const path: string = await invoke('svg_to_png', {
    svgPath: selectSvg.value,
    width: width.value,
    height: height.value
  })

  urls.value.push([path, convertFileSrc(path)])
}

async function handleFfmpeg() {
  const result = await invoke('start_ffmpeg_task')

  // eslint-disable-next-line no-console
  console.log(result)
}

async function handleConvertVideo() {
  // const result = await invoke('get_video_duration', {
  //   videoPath: 'C:\\Users\\lilei\\Downloads\\123.mkv'
  // })

  const result = await invoke('convert_video_to_mp4', {
    videoPath: 'C:\\Users\\lilei\\Downloads\\123.mkv',
    outputPath: 'C:\\Users\\lilei\\Downloads\\output.mp4'
  })

  logger.info(result)
}

async function handleCutVideo() {
  const len = segments.value.length

  if (len === 0 || segments.value[len - 1].end === '') {
    logger.warn('请先添加分段')
    return
  }

  const result = await invoke('create_highlight_video', {
    videoPath: 'C:\\Users\\lilei\\Downloads\\rabbit.mp4',
    outputPath: 'C:\\Users\\lilei\\Downloads\\output.mp4',
    segments: [...segments.value]
  })

  segments.value.length = 0

  logger.info(result)
}

function handleSave() {
  const len = segments.value.length

  if (len === 0 || segments.value[len - 1].end !== '') {
    segments.value.push({ start: time.value, end: '' })
    return
  }

  segments.value[len - 1].end = time.value
}

async function handleConcatVideo() {
  const result = await invoke('merge_smart', {
    inputs: ['C:\\Users\\lilei\\Downloads\\123.mkv', 'C:\\Users\\lilei\\Downloads\\124.mkv', 'C:\\Users\\lilei\\Downloads\\125.mkv'],
    outputPath: 'C:\\Users\\lilei\\Downloads\\output.mp4'
  })

  logger.info(result)
}

let unlistenProgress: UnlistenFn | null = null
let unlistenComplete: UnlistenFn | null = null

async function initEvent() {
  unlistenProgress = await listen('ffmpeg-progress', (event) => {
    logger.info(event)
  })

  // 监听完成事件
  unlistenComplete = await listen('ffmpeg-complete', (event) => {
    logger.info(event)
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
  <v-main>
    <v-container>
      <v-btn
        class="me-2" height="40" variant="flat" width="80" :icon="MaterialIconThemeFolderSvgOpen"
        @click="handleSelect"
      />
      <v-number-input
        v-model="width" class="w-25 h-25" :reverse="false" control-variant="stacked" label="width"
        :hide-input="false" :inset="false" density="compact" variant="outlined"
      />
      <v-number-input
        v-model="height" class="w-25 h-25" :reverse="false" control-variant="stacked" label="height"
        :hide-input="false" :inset="false" density="compact" variant="outlined"
      />
      <v-btn
        class="ms-2" height="40" variant="flat" width="80" :icon="MaterialIconThemeFolderSvgOpen"
        @click="handleConvert"
      />
    </v-container>

    <v-btn
      class="ms-2" height="40" variant="flat" width="80" :icon="MaterialIconThemeFolderSvgOpen"
      @click="handleFfmpeg"
    />
    <v-btn
      class="ms-2" height="40" variant="flat" width="80" :icon="HugeiconsFolderTransfer"
      @click="handleConvertVideo"
    />
    <v-img :src="svgPath" :width="300" />
    <v-container>
      <v-time-picker
        v-model="time" width="10" height="60" :landscape="$vuetify.display.smAndUp" format="24hr" divided
        hide-title use-seconds density="compact"
      />
      <v-container>
        <p v-for="(item, index) in segments" :key="index">
          {{ item.start }} - {{ item.end }}
        </p>
      </v-container>
      <v-btn @click="handleSave">
        save
      </v-btn>
      <v-btn
        class="ms-2" height="40" variant="flat" width="80" :icon="FluentScreenCut20Filled"
        @click="handleCutVideo"
      />
      <v-btn
        class="ms-2" height="40" variant="flat" width="80" :icon="RiMergeCellsHorizontal"
        @click="handleConcatVideo"
      />
    </v-container>
  </v-main>
</template>
