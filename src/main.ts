import { createApp } from 'vue'

import App from './App.vue'
import { setupRouter } from './router'
import { logger, platformDetector, startWebVitalObserver } from './utils'
import { setupVuetify } from './vuetify'
import './style/global.less'
import 'unfonts.css'

async function init() {
  // 初始化日志系统
  await logger.init()

  // web性能检查
  startWebVitalObserver()

  // 获取平台
  platformDetector.init()

  const app = createApp(App)

  setupVuetify(app)
  setupRouter(app)

  app.mount('#app')
}

init()
