<script setup lang="ts">
import type { VideoPlayerInfoType } from './types/video'

import { provide, ref } from 'vue'
import AntDesignSettingOutlined from '~icons/ant-design/setting-outlined'
import FileIconsFfmpeg from '~icons/file-icons/ffmpeg'
import MdiEncryptionOutline from '~icons/mdi/encryption-outline'
import MdiFileImage from '~icons/mdi/file-image'
import MdiHome from '~icons/mdi/home'
import PhSidebarSimpleLight from '~icons/ph/sidebar-simple-light'

import VideoPlay from './components/common/VideoPlay.vue'
import { VIDEO_PLAYER_KEY } from './utils/Constant'

const videoPlayerInfo = ref<VideoPlayerInfoType>({
  url: '',
  show: false
})

provide(VIDEO_PLAYER_KEY, videoPlayerInfo)

const rail = ref(false)

const drawer = ref(true)

function handleSidebar() {
  rail.value = !rail.value
}
</script>

<template>
  <v-app>
    <v-navigation-drawer
      v-model="drawer"
      :rail="rail"
      permanent
    >
      <v-btn class=" ma-1" size="small" :icon="PhSidebarSimpleLight" @click="handleSidebar" />
      <v-divider />
      <v-list density="compact" nav>
        <v-list-item
          :prepend-icon="MdiHome"
          title="Home"
          value="home"
          to="/"
        />
        <v-list-item
          :prepend-icon="FileIconsFfmpeg"
          title="FFmpeg"
          value="ffmpeg"
          to="/ffmpeg"
        />
        <v-list-item
          :prepend-icon="MdiEncryptionOutline"
          title="Encryption"
          value="encryption"
          to="/encryption"
        />
        <v-list-item
          :prepend-icon="MdiFileImage"
          title="Image"
          value="image"
          to="/image"
        />
        <v-list-item
          :prepend-icon="AntDesignSettingOutlined"
          title="Settings"
          value="settings"
          to="/settings"
        />
      </v-list>
    </v-navigation-drawer>
    <v-main class="fill-height">
      <router-view />
    </v-main>
  </v-app>
  <VideoPlay />
</template>

<style lang="less">
#app {
  width: 100vw;
  height: 100vh;
}
</style>
