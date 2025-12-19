import { type, version } from '@tauri-apps/plugin-os'

/**
 * 平台类型枚举
 */
export type PlatformType = 'desktop' | 'mobile'

/**
 * 操作系统类型
 */
export type OSType = 'windows' | 'macos' | 'linux' | 'android' | 'ios'

/**
 * 平台检测结果 - 应用启动时执行一次，全局共享
 */
class PlatformDetector {
  private _osType: OSType = 'windows'
  private _platformType: PlatformType = 'desktop'
  private _osVersion: string | undefined
  private _isWindows10: boolean = false
  private _initialized: boolean = false

  public init() {
    if (this._initialized) {
      return
    }

    try {
      const detectedType = type()
      this._osType = this._normalizeOSType(detectedType)
      this._platformType = this._isDesktopOS(this._osType) ? 'desktop' : 'mobile'
      this._osVersion = this._detectVersion()
      this._isWindows10 = this._isWindows10Version()
    } catch (error) {
      console.warn('Failed to detect platform:', error)
    }

    this._initialized = true
  }

  private _normalizeOSType(osType: OSType): OSType {
    switch (osType) {
      case 'windows':
        return 'windows'
      case 'macos':
        return 'macos'
      case 'linux':
        return 'linux'
      case 'android':
        return 'android'
      case 'ios':
        return 'ios'
      default:
        throw new Error(`Unsupported OS type: ${osType}`)
    }
  }

  private _isDesktopOS(osType: OSType): boolean {
    return osType === 'windows' || osType === 'linux' || osType === 'macos'
  }

  private _detectVersion(): string | undefined {
    try {
      return version()
    } catch (error) {
      console.warn('Failed to detect platform version:', error)
      return undefined
    }
  }

  private _isWindows10Version(osVersion?: string): boolean {
    if (!osVersion) {
      return false
    }

    const numbers = osVersion
      .match(/\d+/g)
      ?.map(num => Number.parseInt(num, 10))
      .filter(num => !Number.isNaN(num))
    if (!numbers || numbers.length === 0) {
      return false
    }

    const [major, minor, patch] = numbers
    const buildNumber = typeof patch === 'number' ? patch : typeof minor === 'number' ? minor : undefined

    return major === 10 && typeof buildNumber === 'number' && buildNumber < 22000
  }

  /** 获取操作系统类型 - 'windows' | 'macos' | 'linux' | 'android' | 'ios' */
  get osType(): OSType {
    return this._osType
  }

  /** 获取平台类型 - 'desktop' | 'mobile' */
  get platformType(): PlatformType {
    return this._platformType
  }

  /** 获取系统版本号 */
  get osVersion(): string | undefined {
    return this._osVersion
  }

  /** 获取是否桌面端 */
  get isDesktop(): boolean {
    return this._platformType === 'desktop'
  }

  /** 获取是否移动端 */
  get isMobile(): boolean {
    return this._platformType === 'mobile'
  }

  /** 获取是否 Windows 10 */
  get isWindows10(): boolean {
    return this._isWindows10
  }

  /** 获取是否 Windows */
  get isWindows(): boolean {
    return this._osType === 'windows'
  }

  /** 获取是否 macOS */
  get isMac(): boolean {
    return this._osType === 'macos'
  }

  /** 获取是否 Linux */
  get isLinux(): boolean {
    return this._osType === 'linux'
  }

  /** 获取是否 Android */
  get isAndroid(): boolean {
    return this._osType === 'android'
  }

  /** 获取是否 iOS */
  get isIOS(): boolean {
    return this._osType === 'ios'
  }

  /** 是否为兼容平台（Windows 或 Linux） */
  get isCompatibility(): boolean {
    return this.isWindows || this.isLinux
  }
}

export const platformDetector = new PlatformDetector()
