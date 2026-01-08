<script setup lang="ts">
import type { UnlistenFn } from '@tauri-apps/api/event'

import { onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import MdiClose from '~icons/mdi/close'
import MdiEye from '~icons/mdi/eye'
import MdiEyeOff from '~icons/mdi/eye-off'

import { logger } from '../../utils'
import SelectFile from '../common/SelectFile.vue'

const encrypting = ref(false)
const encryptPassword = ref('')
const encryptShowPassword = ref(false)
const encryptFilePath = ref('')
const encryptOutPath = ref('')
const encryptProgress = ref(0)

const decrypting = ref(false)
const decryptPassword = ref('')
const decryptShowPassword = ref(false)
const decryptFilePath = ref('')
const decryptOutPath = ref('')
const decryptProgress = ref(0)

function handleSelectFileToEncrypt(select: string[]) {
  if (select.length === 0) {
    return
  }

  encryptFilePath.value = select[0]
  encryptOutPath.value = `${select[0]}.enc`
}

function handleCloseEncrypt() {
  encryptFilePath.value = ''
  encryptOutPath.value = ''
  encryptProgress.value = 0
}

async function handleEncrypt() {
  if (!encryptPassword.value || !encryptFilePath.value || !encryptOutPath.value) {
    return
  }

  encrypting.value = true

  await invoke('encrypt_file', {
    inputPath: encryptFilePath.value,
    outputPath: encryptOutPath.value,
    password: encryptPassword.value
  }).catch((e) => {
    logger.error(e)
  })
}

function handleSelectFileToDecrypt(select: string[]) {
  if (select.length === 0) {
    return
  }

  decryptFilePath.value = select[0]
  decryptOutPath.value = select[0].split('.').slice(0, -1).join('.')
}

function handleCloseDecrypt() {
  decryptFilePath.value = ''
  decryptOutPath.value = ''
  decryptProgress.value = 0
}

function handleDecrypt() {
  if (!decryptPassword.value || !decryptFilePath.value || !decryptOutPath.value) {
    return
  }

  decrypting.value = true

  invoke('decrypt_file', {
    inputPath: decryptFilePath.value,
    outputPath: decryptOutPath.value,
    password: decryptPassword.value
  }).catch((e) => {
    logger.error(e)
  })
}

let unlistenProgress: UnlistenFn | null = null
let unlistenError: UnlistenFn | null = null

async function initEvent() {
  unlistenProgress = await listen('encrypt_progress', ({ payload }) => {
    if (encrypting.value) {
      encryptProgress.value = payload as number
      if (payload === 100) {
        encrypting.value = false
      }
    }

    if (decrypting.value) {
      decryptProgress.value = payload as number

      if (payload === 100) {
        decrypting.value = false
      }
    }
  })

  unlistenError = await listen('encrypt_error', ({ payload }) => {
    logger.error(payload)
  })
}

function closeEvent() {
  unlistenProgress?.()
  unlistenError?.()
}

initEvent()

onUnmounted(closeEvent)
</script>

<template>
  <v-container>
    <v-card class="pa-4">
      <v-card-title>加密文件</v-card-title>
      <v-card-subtitle>使用 chacha20 流式加密文件</v-card-subtitle>
      <v-card-text>
        <SelectFile v-if="!encryptFilePath" :multiple="false" extensions="*.*" @select="handleSelectFileToEncrypt" />
        <div v-else class="d-flex">
          <span class="text-body-1">{{ encryptFilePath }}</span>
          <v-icon-btn class="ms-2 align-center" size="xl-smell" color="error" :icon="MdiClose" @click="handleCloseEncrypt" />
        </div>
      </v-card-text>
      <v-card-item>
        <v-text-field
          v-model="encryptPassword"
          :append-inner-icon="encryptShowPassword ? MdiEye : MdiEyeOff"
          :type="encryptShowPassword ? 'text' : 'password'"
          label="输入密码"
          variant="outlined"
          @click:append-inner="encryptShowPassword = !encryptShowPassword"
        />
        <v-text-field v-model="encryptOutPath" label="输出路径 (可选)" placeholder="默认保存在原文件夹" variant="outlined" class="mt-4" />
        <v-container class="mt-0">
          <v-progress-linear v-if="encryptProgress > 0" color="blue-lighten-3" :model-value="encryptProgress" rounded />
        </v-container>
      </v-card-item>

      <v-card-actions>
        <v-spacer />
        <v-btn color="primary" :loading="encrypting" :disabled="!encryptPassword || !encryptFilePath || !encryptOutPath" @click="handleEncrypt">
          开始加密
        </v-btn>
      </v-card-actions>
    </v-card>

    <v-card class="pa-4 mt-4">
      <v-card-title>解密文件</v-card-title>
      <v-card-subtitle>使用 chacha20 流式解密文件</v-card-subtitle>
      <v-card-text>
        <SelectFile v-if="!decryptFilePath" :multiple="false" extensions="*.*" @select="handleSelectFileToDecrypt" />
        <div v-else class="d-flex">
          <span class="text-body-1">{{ decryptFilePath }}</span>
          <v-icon-btn class="ms-2 align-center" size="xl-smell" color="error" :icon="MdiClose" @click="handleCloseDecrypt" />
        </div>
      </v-card-text>
      <v-card-item>
        <v-text-field
          v-model="decryptPassword"
          :append-inner-icon="decryptShowPassword ? MdiEye : MdiEyeOff"
          :type="decryptShowPassword ? 'text' : 'password'"
          label="输入密码"
          variant="outlined"
          @click:append-inner="decryptShowPassword = !decryptShowPassword"
        />
        <v-text-field v-model="decryptOutPath" label="输出路径 (可选)" placeholder="默认保存在原文件夹" variant="outlined" class="mt-4" />
        <v-container class="mt-0">
          <v-progress-linear v-if="decryptProgress > 0" color="blue-lighten-3" :model-value="decryptProgress" rounded />
        </v-container>
      </v-card-item>

      <v-card-actions>
        <v-spacer />
        <v-btn color="primary" :loading="decrypting" :disabled="!decryptFilePath || !decryptPassword || !decryptOutPath" @click="handleDecrypt">
          开始解密
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
