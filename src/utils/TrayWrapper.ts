import type { TrayIconEvent, TrayIconOptions } from '@tauri-apps/api/tray'

import { defaultWindowIcon } from '@tauri-apps/api/app'
import { Menu } from '@tauri-apps/api/menu'
import { TrayIcon } from '@tauri-apps/api/tray'

import { logger } from './LoggerWrapper'

class TrayWrapper {
  private _menu: Menu | null = null
  private _tray: TrayIcon | null = null
  private _initState: 0 | 1 | 2 = 0

  get menu() {
    return this._menu
  }

  get tray() {
    return this._tray
  }

  get initState() {
    return this._initState
  }

  public async init() {
    if (this._initState !== 0) {
      return
    }
    this._initState = 1

    const menu = await Menu.new({
      items: [
        {
          id: 'quit',
          text: 'Quit',
          action: this._onTrayMenuClick
        }
      ]
    })

    const options: TrayIconOptions = {
      menu,
      icon: await defaultWindowIcon() || undefined,
      action: (event: TrayIconEvent) => {
        switch (event.type) {
          case 'Click':
            logger.info(
              `mouse ${event.button} button pressed, state: ${event.buttonState}`
            )
            break
          case 'DoubleClick':
            logger.info(`mouse ${event.button} button pressed`)
            break
          case 'Enter':
            logger.info(
              `mouse hovered tray at ${event.rect.position.x}, ${event.rect.position.y}`
            )
            break
          case 'Move':
            logger.info(
              `mouse moved on tray at ${event.rect.position.x}, ${event.rect.position.y}`
            )
            break
          case 'Leave':
            logger.info(
              `mouse left tray at ${event.rect.position.x}, ${event.rect.position.y}`
            )
            break
        }
      }
    }

    this._tray = await TrayIcon.new(options)
    this._menu = menu
    this._initState = 2
  }

  /** 不起作用 */
  public destroy() {
    this._tray?.close()
    this._menu?.close()
  }

  private _onTrayMenuClick(itemId: string) {
    logger.info('Tray menu item clicked:', itemId)
    if (itemId === 'quit') {
      // TODO
    }
  }
}

export const trayWrapper = new TrayWrapper()
