import type { App } from 'vue'

import { createVuetify } from 'vuetify'
import { md3 } from 'vuetify/blueprints'
import {
  VApp,
  VBtn,
  VContainer,
  VDivider,
  VImg,
  VMain,
  VNavigationDrawer,
  VNumberInput,
  VTimePicker
} from 'vuetify/components'
import { Ripple } from 'vuetify/directives'

import 'vuetify/styles'

export function setupVuetify(app: App) {
  const vuetify = createVuetify({
    components: {
      VApp,

      VNavigationDrawer,
      VDivider,

      VMain,
      VContainer,

      VBtn,

      VImg,

      VNumberInput,

      VTimePicker
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
