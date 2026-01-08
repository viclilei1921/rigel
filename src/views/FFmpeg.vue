<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const activeTab = ref('convert')

watch(() => route.path, (path) => {
  const parts = path.split('/')
  if (parts.length > 2) {
    activeTab.value = parts[2]
  }
}, { immediate: true })
</script>

<template>
  <v-container fluid class="ffmpeg-body pa-0 fill-height flex-column align-stretch">
    <v-toolbar density="compact" color="surface" elevation="1">
      <v-tabs v-model="activeTab" color="primary" align-tabs="start">
        <v-tab value="convert" to="/ffmpeg/convert" rounded="sm">
          视频转换
        </v-tab>
        <v-tab value="highlight" to="/ffmpeg/highlight" rounded="sm">
          精彩剪辑
        </v-tab>
        <v-tab value="merge" to="/ffmpeg/merge" rounded="sm">
          视频合并
        </v-tab>
        <v-tab value="edit" to="/ffmpeg/append" rounded="sm">
          视频追加
        </v-tab>
      </v-tabs>
    </v-toolbar>

    <v-window v-model="activeTab" class="flex-grow-1 overflow-y-auto">
      <router-view />
    </v-window>
  </v-container>
</template>

<style lang="less" scoped>
.ffmpeg-body {
  gap: 2px;
  height: 100%;
}

.overflow-y-auto {
  height: calc(100% - 50px);
}
</style>
