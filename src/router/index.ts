import type { App } from 'vue'
import type { NavigationGuardNext, RouteLocationNormalized, RouteRecordRaw } from 'vue-router'

import { createRouter, createWebHistory } from 'vue-router'

import AboutPage from '../views/About.vue'
import HomePage from '../views/Home.vue'

/** ! 创建窗口后再跳转页面就会导致样式没有生效所以不能使用懒加载路由的方式，有些页面需要快速响应的就不需要懒加载 */
const { BASE_URL } = import.meta.env

function getDesktopRoutes(): Array<RouteRecordRaw> {
  return [
    {
      path: '/',
      name: 'home',
      component: HomePage
    },
    {
      path: '/about',
      name: 'about',
      component: AboutPage
    }
  ]
}

const router = createRouter({
  history: createWebHistory(BASE_URL),
  routes: getDesktopRoutes()
})

export function setupRouter(app: App) {
  router.beforeEach(async (to: RouteLocationNormalized, _from: RouteLocationNormalized, next: NavigationGuardNext) => {
    if (to.matched.length === 0) {
      return { path: '/' }
    }

    next()
  })

  app.use(router)
}
