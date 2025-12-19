import type { App } from 'vue'

import { createVuetify } from 'vuetify'
import { md3 } from 'vuetify/blueprints'
import {
  VApp,
  VAppBar,
  VAvatar,
  VBtn,
  VCard,
  VCardTitle,
  VCol,
  VContainer,
  VFooter,
  VImg,
  VMain,
  VNumberInput,
  VRow,
  VSpacer,
  VSystemBar
} from 'vuetify/components'
import { Ripple } from 'vuetify/directives'

import 'vuetify/styles'

export function setupVuetify(app: App) {
  const vuetify = createVuetify({
    components: {
      VBtn,
      VApp,
      VSpacer,
      VSystemBar,
      VAppBar,
      VAvatar,
      VContainer,
      VImg,
      VFooter,
      VMain,
      VNumberInput,
      VRow,
      VCol,
      VCard,
      VCardTitle
    },
    directives: {
      Ripple
    },
    blueprint: md3,
    theme: {
      defaultTheme: 'light'
    },
    display: {
      mobileBreakpoint: 'sm',
      thresholds: {
        xs: 0,
        sm: 340,
        md: 540,
        lg: 800,
        xl: 1280
      }
    }
  })

  app.use(vuetify)
}
