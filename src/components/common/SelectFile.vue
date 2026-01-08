<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import MdiFolderOpen from '~icons/mdi/folder-open'

import { logger } from '../../utils'

type EmitsType = {
  (e: 'select', files: string[]): void
}

const props = withDefaults(defineProps<{
  multiple?: boolean
  fileName?: string
  extensions?: string
}>(), {
  multiple: false,
  fileName: 'video',
  extensions: 'mp4,mov,avi,wmv,flv,mkv,webm,mpeg,mpg'
})

const emits = defineEmits<EmitsType>()

const selecting = ref(false)

async function handleSelect() {
  if (selecting.value) {
    return
  }

  selecting.value = true

  const selected = await open({
    multiple: props.multiple,
    filters: [
      {
        name: props.fileName,
        extensions: props.extensions.split(',')
      }
    ]
  }).catch((e) => {
    logger.error(e)
    return null
  })

  selecting.value = false
  if (!selected) {
    return
  }

  if (typeof selected === 'string') {
    emits('select', [selected])
    return
  }

  emits('select', selected as string[])
}
</script>

<template>
  <v-btn
    class="me-2" height="40" variant="flat" width="80" :icon="MdiFolderOpen"
    :loading="selecting" @click="handleSelect"
  />
</template>
