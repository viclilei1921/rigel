import { createApp } from 'vue'

import App from './App.vue'
import { setupRouter } from './router'
import { platformDetector, startWebVitalObserver } from './utils'
import { setupVuetify } from './vuetify'
import './style/global.less'
import 'unfonts.css'

platformDetector.init()
startWebVitalObserver()

// eslint-disable-next-line no-console
console.log(`Platform detected: ${platformDetector.osType} (${platformDetector.osVersion})`)

const app = createApp(App)

setupVuetify(app)
setupRouter(app)

app.mount('#app')
