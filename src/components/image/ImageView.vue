<script setup lang="ts">
import { onMounted, ref } from 'vue'

import { logger, WebGPUDrawer } from '../../utils'

const CanvasDom = ref<HTMLCanvasElement | null>(null)

async function init() {
  if (!CanvasDom.value) {
    return
  }

  const image = new Image()
  image.crossOrigin = 'anonymous'
  // TODO
  image.src = './images/1.jpg'
  image.onerror = () => {
    logger.error('image load error')
    image.onerror = null
  }
  await image.decode()
  image.onerror = null

  CanvasDom.value.style.width = `${image.width}px`
  CanvasDom.value.style.height = `${image.height}px`

  CanvasDom.value.width = Math.floor(image.width * 1.25)
  CanvasDom.value.height = Math.floor(image.height * 1.25)

  const drawer = new WebGPUDrawer(CanvasDom.value)
  await drawer.init()

  const bitmap = await createImageBitmap(image)

  const texture = await drawer.createTextureFromImage(bitmap)

  if (!texture) {
    return
  }

  drawer.draw(texture, 0, 0, CanvasDom.value.width, CanvasDom.value.height)
}

onMounted(init)
</script>

<template>
  <v-container>
    <canvas ref="CanvasDom" />
  </v-container>
</template>
