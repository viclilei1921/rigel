<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import MdiClose from '~icons/mdi/close'
import MdiExitToApp from '~icons/mdi/exit-to-app'
import MdiEye from '~icons/mdi/eye'
import MdiEyeOff from '~icons/mdi/eye-off'

import { logger } from '../../utils'
import SelectFile from '../common/SelectFile.vue'

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
  logger.info(res)

  startServer.value = false
}

async function handleStopServer() {
  await invoke('stop_video_stream').catch((e) => {
    logger.error(e)
  })

  serverPath.value = ''
}
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
    <Teleport v-if="serverPath" to="body">
      <v-sheet class="video-body">
        <v-icon-btn
          color="rgba(255, 255, 255, 0.3)" class="video-body-close" :icon="MdiExitToApp"
          rounded="sm"
          size="sm"
          @click="handleStopServer"
        />
        <video :src="serverPath" controls width="100%" height="100%" />
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

  .video-body-close {
    position: absolute;
    z-index: 10;
    top: 5px;
    left: 5px;
    display: none;
    padding: 3px;
    color: rgb(180 102 252);
  }

  &:hover {
    .video-body-close {
      display: flex;
    }
  }
}
</style>
