import type { App } from 'vue'
import type {
  NavigationGuardNext,
  RouteLocationNormalized,
  RouteRecordRaw
} from 'vue-router'

import { createRouter, createWebHistory } from 'vue-router'

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
      path: '/ffmpeg',
      component: () => import('../views/FFmpeg.vue'),
      redirect: '/ffmpeg/convert',
      children: [
        {
          path: 'convert',
          component: () => import('../components/ffmpeg/ConvertView.vue')
        },
        {
          path: 'highlight',
          component: () => import('../components/ffmpeg/HighlightView.vue')
        },
        {
          path: 'merge',
          component: () => import('../components/ffmpeg/MergeView.vue')
        },
        {
          path: 'edit',
          component: () => import('../components/ffmpeg/EditView.vue')
        }
      ]
    },
    {
      path: '/encryption',
      component: () => import('../views/Encryption.vue'),
      redirect: '/encryption/file',
      children: [
        {
          path: 'file',
          component: () => import('../components/encryption/FileView.vue')
        },
        {
          path: 'video',
          component: () => import('../components/encryption/VideoView.vue')
        }
      ]
    },
    {
      path: '/settings',
      component: () => import('../views/Settings.vue')
    }
  ]
}

const router = createRouter({
  history: createWebHistory(BASE_URL),
  routes: getDesktopRoutes()
})

export function setupRouter(app: App) {
  router.beforeEach(
    async (
      to: RouteLocationNormalized,
      _from: RouteLocationNormalized,
      next: NavigationGuardNext
    ) => {
      if (to.matched.length === 0) {
        return { path: '/' }
      }

      next()
    }
  )

  app.use(router)
}
