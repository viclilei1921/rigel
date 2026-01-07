<script setup lang="ts">
import type { UnlistenFn } from '@tauri-apps/api/event'

import { onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/plugin-dialog'
import FluentEmojiHighContrastOpenFileFolder from '~icons/fluent-emoji-high-contrast/open-file-folder'

import { logger } from '../../utils'

const selecting = ref(false)
const encrypting = ref(false)
const encryptPassword = ref('')
const encryptFilePath = ref('')
const encryptOutPath = ref('')
const encryptProgress = ref(0)

async function handleSelectFileToEncrypt() {
  if (selecting.value) {
    return
  }

  selecting.value = true
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: 'file',
        extensions: ['*']
      }
    ]
  })

  if (!selected) {
    selecting.value = false
    return
  }

  encryptFilePath.value = selected
  encryptOutPath.value = `${selected}.enc`

  selecting.value = false
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

  encrypting.value = false
}

let unlistenProgress: UnlistenFn | null = null

async function initEvent() {
  unlistenProgress = await listen('encrypt_progress', (progress) => {
    if (encrypting.value) {
      encryptProgress.value = progress as unknown as number
    }
  })
}

function closeEvent() {
  unlistenProgress?.()
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
        <v-btn
          class="me-2" height="40" variant="flat" width="80" :icon="FluentEmojiHighContrastOpenFileFolder"
          :loading="selecting"
          @click="handleSelectFileToEncrypt"
        />
      </v-card-text>
      <v-card-item>
        <v-text-field
          v-model="encryptPassword"
          class="mt-2"
          type="password"
          label="输入密码"
          counter
        />
        <v-text-field v-model="encryptOutPath" label="输出路径 (可选)" placeholder="默认保存在原文件夹" variant="outlined" class="mt-4" />
        <v-container class="mt-0">
          <v-progress-linear v-if="encryptProgress > 0" color="blue-lighten-3" :model-value="encryptProgress" rounded />
        </v-container>
      </v-card-item>

      <v-card-actions>
        <v-spacer />
        <v-btn color="primary" :loading="encrypting" :disabled="!encryptPassword || !encryptFilePath || !encryptOutPath" @click="handleEncrypt">
          开始转换
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
