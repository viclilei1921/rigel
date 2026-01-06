import type { App } from 'vue'

import { createVuetify } from 'vuetify'
import { md3 } from 'vuetify/blueprints'
import {
  VAlert,
  VApp,
  VBtn,
  VCard,
  VCardActions,
  VCardItem,
  VCardSubtitle,
  VCardText,
  VCardTitle,
  VCol,
  VContainer,
  VDefaultsProvider,
  VDivider,
  VFileInput,
  VIcon,
  VImg,
  VList,
  VListItem,
  VListItemSubtitle,
  VListItemTitle,
  VListSubheader,
  VMain,
  VNavigationDrawer,
  VNumberInput,
  VProgressLinear,
  VRow,
  VSlider,
  VSpacer,
  VTab,
  VTabs,
  VTextField,
  VTimePicker,
  VToolbar,
  VTooltip,
  VWindow
} from 'vuetify/components'
import { Ripple, Tooltip } from 'vuetify/directives'
import { VIconBtn } from 'vuetify/labs/VIconBtn'
import { VVideo, VVideoVolume } from 'vuetify/labs/VVideo'

import 'vuetify/styles'

export function setupVuetify(app: App) {
  const vuetify = createVuetify({
    components: {
      VApp,

      VNavigationDrawer,
      VList,
      VListItem,
      VListItemTitle,
      VListItemSubtitle,
      VListSubheader,
      VDivider,

      VSpacer,

      VMain,
      VContainer,
      VCol,
      VRow,

      VTab,
      VTabs,

      VToolbar,

      VWindow,

      VCard,
      VCardTitle,
      VCardText,
      VCardActions,
      VCardSubtitle,
      VCardItem,

      VBtn,
      VIconBtn,

      VIcon,
      VImg,
      VVideo,
      VDefaultsProvider,
      VVideoVolume,

      VTextField,
      VNumberInput,
      VFileInput,

      VTimePicker,

      VSlider,

      VProgressLinear,

      VAlert,
      VTooltip
    },
    directives: {
      Ripple,
      Tooltip
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
