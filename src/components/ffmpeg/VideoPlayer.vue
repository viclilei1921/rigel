<script setup lang="ts">
import PhPause from '~icons/ph/pause'
import PhPlay from '~icons/ph/play'

type DefaultPropsType = {
  src?: string
  image?: string
}

withDefaults(defineProps<DefaultPropsType>(), {
  src: '',
  image: ''
})
</script>

<template>
  <v-video
    :start-at="10" :volume-props="{ inline: true }" class="mx-auto" controls-variant="mini" height="300"
    :image="image" max-width="500" rounded="lg" :src="src" eager hide-overlay pills
  >
    <template #controls="{ play, pause, playing, progress, skipTo, volume, toggleMuted, labels }">
      <v-defaults-provider
        :defaults="{ VIconBtn: { color: 'blue', rounded: 'lg', size: '36', variant: 'flat' }, VSlider: { color: 'blue', trackColor: 'white' } }"
      >
        <div class="d-flex ga-3 w-100 px-2">
          <v-icon-btn
            v-tooltip:top="labels.playAction" :aria-label="labels.playAction"
            :icon="playing ? PhPause : PhPlay" @click="() => playing ? pause() : play()"
          />
          <v-slider
            :aria-label="labels.seek" :model-value="progress" width="75%" no-keyboard
            @update:model-value="skipTo"
          />
          <v-video-volume
            v-model="volume.value" :label="labels.volumeAction"
            :slider-props="{ maxWidth: 100, width: '25%' }" class="ga-3" inline @click="toggleMuted"
          />
        </div>
      </v-defaults-provider>
    </template>
  </v-video>
</template>
