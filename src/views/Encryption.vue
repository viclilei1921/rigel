<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const activeTab = ref('file')

watch(() => route.path, (path) => {
  const parts = path.split('/')
  if (parts.length > 2) {
    activeTab.value = parts[2]
  }
}, { immediate: true })
</script>

<template>
  <v-container fluid class="encryption-body pa-0 fill-height flex-column align-stretch">
    <v-toolbar density="compact" color="surface" elevation="1">
      <v-tabs v-model="activeTab" color="primary" align-tabs="start">
        <v-tab value="file" to="/encryption/file" rounded="sm">
          文件加解密
        </v-tab>
        <v-tab value="video" to="/encryption/video" rounded="sm">
          播放加密视频
        </v-tab>
      </v-tabs>
    </v-toolbar>

    <v-window v-model="activeTab" class="flex-grow-1 overflow-y-auto">
      <router-view />
    </v-window>
  </v-container>
</template>

<style lang="less" scoped>
.encryption-body {
  gap: 2px;
  height: 100%;
}

.overflow-y-auto {
  height: calc(100% - 50px);
}
</style>
