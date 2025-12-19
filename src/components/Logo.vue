<script setup lang="ts">
import { computed } from 'vue'

type DefaultPropsType = {
  width?: number
  height?: number
}

const props = withDefaults(defineProps<DefaultPropsType>(), {
  width: 256,
  height: 256
})

const svgStyle = computed(() => {
  return {
    width: `${props.width}px`,
    height: `${props.height}px`
  }
})
</script>

<template>
  <svg :style="svgStyle" width="256" height="256" viewBox="0 0 256 256" fill="none" xmlns="http://www.w3.org/2000/svg">
    <defs>
      <linearGradient id="glass_grad" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#FFFFFF" stop-opacity="0.6" />
        <stop offset="100%" stop-color="#FFFFFF" stop-opacity="0.1" />
      </linearGradient>
      <linearGradient id="core_blue" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#00D2FF" />
        <stop offset="100%" stop-color="#3A7BD5" />
      </linearGradient>
    </defs>

    <rect width="256" height="256" rx="64" fill="#BBDEFB" />

    <g transform="translate(64, 50)">
      <path d="M5 5 L 105 5 L 105 105 L 5 105 Z" fill="#3A7BD5" opacity="0.3" />

      <path d="M25 25 L 130 25 L 130 135 L 25 115 Z" fill="url(#core_blue)" filter="drop-shadow(0 8px 16px rgba(0,0,0,0.4))" />

      <path d="M0 40 L 100 40 L 100 160 L 0 125 Z" fill="url(#glass_grad)" stroke="white" stroke-opacity="0.3" stroke-width="1.5" />

      <line x1="0" y1="40" x2="100" y2="40" stroke="white" stroke-opacity="0.5" stroke-width="2" />
    </g>
  </svg>
</template>
