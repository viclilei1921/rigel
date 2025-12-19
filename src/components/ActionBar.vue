<script setup lang="ts">
import type { UnlistenFn } from '@tauri-apps/api/event'

import { onMounted, onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import PhFolderThin from '~icons/ph/folder-thin'
import QlementineIconsWindowsClose16 from '~icons/qlementine-icons/windows-close-16'
import QlementineIconsWindowsMaximize16 from '~icons/qlementine-icons/windows-maximize-16'
import QlementineIconsWindowsMinimize16 from '~icons/qlementine-icons/windows-minimize-16'
import QlementineIconsWindowsUnmaximize16 from '~icons/qlementine-icons/windows-unmaximize-16'

import { platformDetector } from '../utils'
import Logo from './Logo.vue'

type DefaultPropsType = {
  minW?: boolean
  maxW?: boolean
  closeW?: boolean
  isDrag?: boolean
}

const {
  minW = true,
  maxW = true,
  closeW = true,
  isDrag = true
} = defineProps<DefaultPropsType>()

/** 窗口是否最大化状态 */
const windowMaximized = ref(false)

const appWindow = WebviewWindow.getCurrent()

let offSizeFn: UnlistenFn | undefined

async function updateWindowMaximized() {
  const maximized = await appWindow.isMaximized()

  if (platformDetector.isMac) {
    const fullscreen = await appWindow.isFullscreen()
    windowMaximized.value = maximized || fullscreen
    return
  }

  windowMaximized.value = maximized
}

async function minimizeWindow() {
  await appWindow.minimize()
}

async function restoreWindow() {
  if (windowMaximized.value) {
    await appWindow.unmaximize()
    return
  }

  await appWindow.maximize()
}

async function closeWindow() {
  await appWindow.close()
}

async function openCacheFolder() {
  await invoke('open_cache_folder')
}

onMounted(async () => {
  await updateWindowMaximized()

  offSizeFn = await appWindow.onResized(() => updateWindowMaximized())
})

onUnmounted(async () => {
  offSizeFn?.()
})
</script>

<template>
  <v-system-bar :data-tauri-drag-region="isDrag" class="pa-1 h-30">
    <Logo :width="18" :height="18" />
    <v-spacer />
    <v-btn
      class="mr-2" rounded="sm" size="small-x" variant="text" :icon="PhFolderThin"
      color="gray" @click="openCacheFolder"
    />
    <v-btn
      v-if="minW" class="mr-2" rounded="sm" size="small-x" variant="text" :icon="QlementineIconsWindowsMinimize16"
      color="gray" @click="minimizeWindow"
    />
    <v-btn
      v-if="maxW" class="mr-2" rounded="sm" size="small-x" variant="text"
      :icon="windowMaximized ? QlementineIconsWindowsUnmaximize16 : QlementineIconsWindowsMaximize16" color="gray"
      @click="restoreWindow"
    />
    <v-btn
      v-if="closeW" active-color="bg-error" rounded="sm" size="small-x" variant="text" :icon="QlementineIconsWindowsClose16" color="gray"
      @click="closeWindow"
    />
  </v-system-bar>
</template>
