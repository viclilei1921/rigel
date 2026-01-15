import { attachConsole, error, info, trace, warn } from '@tauri-apps/plugin-log';

class LoggerWrapper {
  private _initState: 0 | 1 | 2 = 0;
  private _isDev = import.meta.env.DEV;

  public async init() {
    if (this._initState !== 0) {
      return;
    }

    this._initState = 1;
    const detach = await attachConsole();
    detach();

    this._initState = 2;
  }

  public trace(...args: unknown[]) {
    if (this._initState === 2 && !this._isDev) {
      trace(args.map(this._formatArg).join(' '));
      return;
    }

    console.trace(...args);
  }

  public info(...args: unknown[]) {
    if (this._initState === 2 && !this._isDev) {
      info(args.map(this._formatArg).join(' '));
      return;
    }

    console.log(...args);
  }

  public warn(...args: unknown[]) {
    if (this._initState === 2 && !this._isDev) {
      warn(args.map(this._formatArg).join(' '));
      return;
    }

    console.warn(...args);
  }

  public error(...args: unknown[]) {
    if (this._initState === 2 && !this._isDev) {
      error(args.map(this._formatArg).join(' '));
      return;
    }

    console.error(...args);
  }

  private _formatArg(arg: unknown): string {
    if (arg === null) {
      return 'null';
    }
    if (arg === undefined) {
      return 'undefined';
    }

    const type = typeof arg;

    switch (type) {
      case 'string':
        return arg as string;
      case 'number':
      case 'boolean':
      case 'bigint':
      case 'symbol':
        return String(arg);
      case 'object':
        try {
          if (Array.isArray(arg)) {
            return `[${(arg as unknown[]).map((item) => this._formatArg(item)).join(', ')}]`;
          }
          return JSON.stringify(arg, null, 2);
        } catch {
          return `[object ${(arg as object).constructor?.name || 'Object'}]`;
        }
      case 'function':
        // eslint-disable-next-line @typescript-eslint/no-unsafe-function-type
        return `[Function: ${(arg as Function).name || 'anonymous'}]`;
      default:
        return String(arg);
    }
  }
}

/** 日志系统 */
export const logger = new LoggerWrapper();
