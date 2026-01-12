<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const activeTab = ref('player')

watch(() => route.path, (path) => {
  const parts = path.split('/')
  if (parts.length > 2) {
    activeTab.value = parts[2]
  }
}, { immediate: true })
</script>

<template>
  <v-container fluid class="image-body pa-0">
    <v-toolbar density="compact" color="surface" elevation="1">
      <v-tabs v-model="activeTab" color="primary" align-tabs="start">
        <v-tab value="player" to="/image/player" rounded="sm">
          播放图片
        </v-tab>
      </v-tabs>
    </v-toolbar>
    <v-window v-model="activeTab" class="flex-grow-1 overflow-auto">
      <router-view />
    </v-window>
  </v-container>
</template>

<style lang="less" scoped>
.image-body {
  gap: 2px;
  height: 100%;
}
</style>
