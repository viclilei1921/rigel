<script setup lang="ts">
import { onUnmounted, ref } from 'vue'
import PhSidebarSimpleLight from '~icons/ph/sidebar-simple-light'

import { trayWrapper } from './utils'

const rail = ref(false)

const drawer = ref(true)

function handleSidebar() {
  rail.value = !rail.value
}

onUnmounted(() => {
  trayWrapper.destroy()
})
</script>

<template>
  <v-app>
    <v-navigation-drawer
      v-model="drawer"
      :rail="rail"
      permanent
    >
      <v-btn size="small" :icon="PhSidebarSimpleLight" @click="handleSidebar" />
      <v-divider />
      <v-btn size="small">
        11
      </v-btn>
    </v-navigation-drawer>
    <RouterView />
  </v-app>
</template>
