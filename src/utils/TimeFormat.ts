/**
 * 工具函数 - 补零
 * @param num 数字
 * @param length 补零长度
 * @returns 补零后的字符串
 */
export function Completion(num: number, length = 2) {
  return num.toString().padStart(length, '0')
}

/**
 * 格式化时间
 * @param t 时间 - 单位(s)
 * @returns [h, m, s, ms]
 */
export function TimeToFormat(t: number): [string, string, string, string] {
  const h = Math.floor(t / 3600)
  const m = Math.floor((t - 3600 * h) / 60)
  const s = Math.floor(t - 3600 * h - 60 * m)
  const ms = Math.floor((t - 3600 * h - 60 * m - s) * 1000)

  return [Completion(h), Completion(m), Completion(s), Completion(ms, 3)]
}

/**
 * 格式化时间
 * @param t 时间 - 单位(s)
 * @returns 格式化后的时间
 */
export function FormatTime(t: number) {
  const [h, m, s, ms] = TimeToFormat(t)
  return `${h}:${m}:${s}.${ms}`
}
