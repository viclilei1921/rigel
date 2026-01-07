<script setup lang="ts">
import { ref } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import FluentEmojiHighContrastOpenFileFolder from '~icons/fluent-emoji-high-contrast/open-file-folder'
import MdiWindowClose from '~icons/mdi/window-close'

const selecting = ref(false)
const videoPlayPath = ref('')

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
        extensions: ['*.*']
      }
    ]
  })

  if (!selected) {
    selecting.value = false
    return
  }

  videoPlayPath.value = convertFileSrc(selected)

  selecting.value = false
}
</script>

<template>
  <v-container>
    <v-card class="pa-4">
      <v-card-title>视频转换</v-card-title>
      <v-card-subtitle>使用 ffmpeg 将视频转换为 mp4 格式</v-card-subtitle>
      <v-card-text>
        <v-btn
          class="me-2" height="40" variant="flat" width="80" :icon="FluentEmojiHighContrastOpenFileFolder"
          :loading="selecting" @click="handleSelectVideo"
        />
      </v-card-text>
    </v-card>
    <Teleport v-if="videoPlayPath" to="body">
      <v-sheet class="video-body">
        <v-icon-btn
          color="transparent" class="position-absolute top-0 right-0" :icon="MdiWindowClose"
          @click="videoPlayPath = ''"
        />
        <video :src="videoPlayPath" controls width="100%" height="100%" />
      </v-sheet>
    </Teleport>
  </v-container>
</template>

<style lang="less" scoped>
.video-body {
  position: fixed;
  z-index: 9999;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: black;
}
</style>
