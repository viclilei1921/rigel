<script setup lang="ts">
import { inject, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import MdiClose from '~icons/mdi/close'
import MdiEye from '~icons/mdi/eye'
import MdiEyeOff from '~icons/mdi/eye-off'

import { logger } from '../../utils'
import { VIDEO_PLAYER_KEY } from '../../utils/Constant'
import SelectFile from '../common/SelectFile.vue'

const videoPlayerInfo = inject(VIDEO_PLAYER_KEY)!

const videoPlayPath = ref('')

const password = ref('')
const showPassword = ref(false)

const startServer = ref(false)
const serverPath = ref('')

async function handleSelectVideo(select: string[]) {
  videoPlayPath.value = select[0]
}

function handleCloseVideoPath() {
  videoPlayPath.value = ''
}

async function handleStartServer() {
  if (startServer.value) {
    return
  }

  startServer.value = true
  const res = await invoke('start_video_stream', { password: password.value, path: videoPlayPath.value }).then(p => `${p}`).catch((e) => {
    logger.error(e)
    return null
  })

  if (!res) {
    startServer.value = false
    return
  }

  serverPath.value = res

  videoPlayerInfo.value.url = serverPath.value
  videoPlayerInfo.value.show = true

  startServer.value = false
}

async function handleStopServer() {
  await invoke('stop_video_stream').catch((e) => {
    logger.error(e)
  })

  serverPath.value = ''
}

watch(() => videoPlayerInfo.value.show, () => {
  if (!videoPlayerInfo.value.show) {
    handleStopServer()
  }
})
</script>

<template>
  <v-container>
    <v-card class="pa-4">
      <v-card-title>播放加密视频</v-card-title>
      <v-card-subtitle>使用rust在后端创建一个http服务, 用于播放加密视频</v-card-subtitle>
      <v-card-text>
        <SelectFile v-if="!videoPlayPath" class="mb-2" :multiple="false" extensions="*.*" @select="handleSelectVideo" />
        <div v-else class="d-flex mb-2">
          <span class="text-body-1">{{ videoPlayPath }}</span>
          <v-icon-btn class="ms-2 align-center" size="xl-smell" color="error" :icon="MdiClose" @click="handleCloseVideoPath" />
        </div>
        <v-text-field
          v-model="password"
          :append-inner-icon="showPassword ? MdiEye : MdiEyeOff"
          class="mt-2"
          :type="showPassword ? 'text' : 'password'"
          label="输入密码"
          variant="outlined"
          @click:append-inner="showPassword = !showPassword"
        />
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn color="primary" :loading="startServer" :disabled="!password || !videoPlayPath" @click="handleStartServer">
          开始播放
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
