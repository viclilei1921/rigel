<script setup lang="ts">
import type { VVideo } from 'vuetify/labs/VVideo'

import { inject, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import MdiBackspaceOutline from '~icons/mdi/backspace-outline'
import MdiFullscreen from '~icons/mdi/fullscreen'
import MdiFullscreenExit from '~icons/mdi/fullscreen-exit'
import MdiPause from '~icons/mdi/pause'
import MdiPlay from '~icons/mdi/play'
import MdiVolumeHigh from '~icons/mdi/volume-high'
import MdiVolumeMedium from '~icons/mdi/volume-medium'
import MdiVolumeOff from '~icons/mdi/volume-off'

import { FormatTime } from '../../utils'
import { VIDEO_PLAYER_KEY } from '../../utils/Constant'

const info = inject(VIDEO_PLAYER_KEY)!

const VideoRef = ref<VVideo | null>(null)

const duration = ref(0)
const volume = ref(1)
const speed = ref(1)

const speedList = [
  { value: 0.5, label: '0.5x' },
  { value: 0.75, label: '0.75x' },
  { value: 1, label: '1.0x' },
  { value: 1.25, label: '1.25x' },
  { value: 1.5, label: '1.5x' },
  { value: 2, label: '2.0x' },
  { value: 2.5, label: '2.5x' },
  { value: 3, label: '3.0x' },
  { value: 4, label: '4.0x' }
]

function setSpeed(val: number) {
  const video = VideoRef.value?.video
  if (!video) {
    return
  }

  video.playbackRate = val
  speed.value = val
}

function changeVolume(value: number) {
  const video = VideoRef.value?.video
  if (!video) {
    return
  }

  video.volume = value
  volume.value = value
}

let preVolume = 1
let hasKeyDown = false

function handleKeyDown(e: KeyboardEvent) {
  if (!info.value.show) {
    return
  }
  e.preventDefault()
  e.stopImmediatePropagation() // 强制停止所有后续监听器

  if (hasKeyDown) {
    return
  }
  hasKeyDown = true

  const video = VideoRef.value?.video
  if (!video) {
    return
  }

  switch (e.key) {
    case ' ':
      video.paused ? video.play() : video.pause()
      break
    case 'ArrowLeft':
      if (video.currentTime - 5 < 0) {
        video.currentTime = 0
        break
      }
      video.currentTime -= 5
      break
    case 'ArrowRight':
      if (video.currentTime + 5 > duration.value) {
        video.currentTime = duration.value
        break
      }
      video.currentTime += 5
      break
    case 'ArrowUp':
      if (volume.value + 0.1 >= 1) {
        volume.value = 1
        video.volume = volume.value
        break
      }
      volume.value += 0.1
      video.volume = volume.value
      break
    case 'ArrowDown':
      if (volume.value - 0.1 <= 0) {
        volume.value = 0
        video.volume = volume.value
        break
      }
      volume.value -= 0.1
      video.volume = volume.value
      break
    case 'm':
    case 'M':
      if (video.muted) {
        video.volume = preVolume
        video.muted = false
        volume.value = preVolume
        break
      }

      preVolume = volume.value
      video.muted = true
      video.volume = 0
      volume.value = 0
      break
    case 'f':
    case 'F':
      video.requestFullscreen()
      break
    case 'Escape':
    case 'q':
    case 'Q':
      info.value.show = false
      break
  }
}

function handleKeyUp(e: KeyboardEvent) {
  if (!info.value.show) {
    return
  }

  e.preventDefault()
  e.stopImmediatePropagation() // 强制停止所有后续监听器
  hasKeyDown = false
}

async function init() {
  await nextTick()

  if (info.value.show) {
    const video = VideoRef.value?.video
    if (!video) {
      return
    }

    video.style.objectFit = 'contain'

    video.onloadedmetadata = () => {
      video.volume = volume.value
      duration.value = video.duration
      speed.value = video.playbackRate
    }
  }
}

function destroy() {
  if (!VideoRef.value) {
    return
  }

  const video = VideoRef.value.video

  if (!video) {
    return
  }

  video.onloadedmetadata = null
}

watch(() => info.value.show, () => {
  if (info.value.show) {
    init()
    return
  }

  destroy()
})

init()

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown, false)
  window.addEventListener('keyup', handleKeyUp, false)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown, false)
  window.removeEventListener('keyup', handleKeyUp, false)
})
</script>

<template>
  <Teleport to="body">
    <v-sheet v-if="info.show" class="video-body">
      <v-video
        ref="VideoRef"
        color="blue" tabindex="0" playsinline :start-at="0" height="100vh" width="100vw" autoplay
        :src="info.url" eager hide-overlay pills
      >
        <template #header>
          <div class="d-flex pa-1 pointer-pass-through">
            <v-icon-btn
              class="ml-auto video-header-element"
              :icon="MdiBackspaceOutline"
              color="blue"
              size="sm"
              :icon-size="20"
              variant="text"
              @click="info.show = false"
            />
          </div>
        </template>
        <template #controls="{ play, pause, playing, progress, skipTo, fullscreen, toggleFullscreen, labels }">
          <v-defaults-provider
            :defaults="{ VIconBtn: { color: 'blue', rounded: 'sm', size: 'sm' }, VSlider: { color: 'blue' } }"
          >
            <div class="d-flex align-center ga-3 w-100">
              <v-icon-btn
                :icon-size="20" :icon="playing ? MdiPause : MdiPlay"
                @click="() => playing ? pause() : play()"
              />
              <v-slider
                track-size="2" :aria-label="labels.seek" :model-value="progress" width="60%" no-keyboard
                thumb-size="10" @update:model-value="skipTo"
              />
              <span class="duration-content text-caption text-blue">
                {{ FormatTime(progress * duration / 100) }} / {{ FormatTime(duration) }}
              </span>
              <v-menu location="top center" offset="5">
                <template #activator="{ props }">
                  <v-btn class="pa-0" variant="text" v-bind="props" size="small" color="blue">
                    {{ speed }}x
                  </v-btn>
                </template>
                <v-list class="speed-list-content" bg-color="rgba(0, 0, 0, 0.6)">
                  <v-list-item
                    v-for="item in speedList" :key="item.value" class="text-blue"
                    :active="item.value === speed" density="compact" min-height="20px" @click="setSpeed(item.value)"
                  >
                    {{ item.label }}
                  </v-list-item>
                </v-list>
              </v-menu>
              <v-menu :close-on-content-click="false" location="top center">
                <template #activator="{ props }">
                  <v-icon-btn
                    :icon-size="20" v-bind="props"
                    :icon="volume === 0 ? MdiVolumeOff : volume < 0.5 ? MdiVolumeMedium : MdiVolumeHigh"
                  />
                </template>

                <v-slider
                  class="volume-slider" track-size="2" :model-value="volume" direction="vertical" min="0"
                  max="1" step="0.01" thumb-size="10" hide-details color="blue"
                  @update:model-value="changeVolume"
                />
              </v-menu>
              <v-icon-btn
                :icon-size="20" :icon="fullscreen ? MdiFullscreenExit : MdiFullscreen"
                @click="toggleFullscreen"
              />
            </div>
          </v-defaults-provider>
        </template>
      </v-video>
    </v-sheet>
  </Teleport>
</template>

<style lang="less" scoped>
.video-body {
  position: fixed;
  z-index: 1010;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: black;

  .video-body-close {
    position: absolute;
    z-index: 10;
    top: 5px;
    left: 5px;
    display: none;
    padding: 3px;
    color: rgb(180 102 252);
  }

  &:hover {
    .video-body-close {
      display: flex;
    }
  }

  .duration-content {
    width: 162px;
    line-height: 10px;
  }
}

.speed-list-content {
  backdrop-filter: blur(4px);
  font-size: 0.75rem;
}
</style>

<style lang="less">
.volume-slider {
  height: 100px;
  min-height: 100px;

  &.v-slider.v-input--vertical>.v-input__control {
    height: 100px;
    min-height: 100px;
  }
}
</style>
