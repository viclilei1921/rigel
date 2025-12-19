import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

import { platformDetector } from '../utils'

export function useWindow() {
  async function createWebviewWindow(
    title: string,
    label: string,
    url: string,
    width: number,
    height: number,
    visible = false,
    transparent = false,
    resizable = false,
    wantCloseWindow?: string,
    minWidth = 330,
    minHeight = 495
  ) {
    if (!platformDetector.isDesktop) {
      return null
    }

    const webview = new WebviewWindow(label, {
      title,
      url,
      fullscreen: false,
      center: true,
      width,
      height,
      resizable,
      minHeight,
      minWidth,
      transparent: transparent || platformDetector.isCompatibility,
      visible,
      skipTaskbar: false,
      decorations: !platformDetector.isCompatibility,
      titleBarStyle: 'overlay',
      hiddenTitle: true,
      ...platformDetector.isWindows10 ? { shadow: false } : {}
    })

    webview.once('tauri://webview-created', async () => {
      // eslint-disable-next-line no-console
      console.log('webview created')
    })

    webview.once('tauri://window-created', async () => {
      // eslint-disable-next-line no-console
      console.log('window created')
      if (wantCloseWindow) {
        const win = await WebviewWindow.getByLabel(wantCloseWindow)
        win?.close()
      }
    })

    webview.once('tauri://destroyed', async () => {
      // eslint-disable-next-line no-console
      console.log('webview destroyed')
    })

    // webview.show()

    return webview
  }

  return {
    createWebviewWindow
  }
}
