<script setup lang="ts">
import { ref } from 'vue'
import { convertFileSrc, invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import MaterialIconThemeFolderSvgOpen from '~icons/material-icon-theme/folder-svg-open'

const urls = ref<[string, string][]>([])

const selectSvg = ref('')
const svgPath = ref('')

const width = ref(600)
const height = ref(600)

async function handleSelect() {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: 'SVG',
        extensions: ['svg']
      }
    ]
  })

  if (!selected) {
    return
  }

  // eslint-disable-next-line no-console
  console.log(selected)
  selectSvg.value = selected
  svgPath.value = convertFileSrc(selected)
}

async function handleConvert() {
  if (selectSvg.value === '') {
    return
  }

  const path: string = await invoke('svg_to_png', {
    svgPath: selectSvg.value,
    width: width.value,
    height: height.value
  })

  urls.value.push([path, convertFileSrc(path)])
}
</script>

<template>
  <v-app-bar
    color="grey-lighten-4"
    height="72"
    flat
  >
    <v-btn
      class="me-2"
      height="40"
      variant="flat"
      width="80"
      :icon="MaterialIconThemeFolderSvgOpen"
      @click="handleSelect"
    />
    <v-number-input
      v-model="width"
      class="w-25 h-25"
      :reverse="false"
      control-variant="stacked"
      label="width"
      :hide-input="false"
      :inset="false"
      density="compact"
      variant="outlined"
    />
    <v-number-input
      v-model="height"
      class="w-25 h-25"
      :reverse="false"
      control-variant="stacked"
      label="height"
      :hide-input="false"
      :inset="false"
      density="compact"
      variant="outlined"
    />
    <v-btn
      class="ms-2"
      height="40"
      variant="flat"
      width="80"
      :icon="MaterialIconThemeFolderSvgOpen"
      @click="handleConvert"
    />
    <v-spacer />
  </v-app-bar>

  <v-main>
    <v-img :src="svgPath" :width="300" />
    <v-container>
      <v-row>
        <v-col
          v-for="(item, index) in urls"
          :key="index"
          cols="12"
          sm="6"
          md="4"
          lg="3"
        >
          <v-card>
            <v-img :src="item[1]" height="200px" />
            <v-card-title>{{ item[0] }}</v-card-title>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </v-main>
  <v-footer
    color="grey"
    height="44"
    app
  />
</template>
