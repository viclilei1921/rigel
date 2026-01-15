/**
 * 通过路径获取文件名
 * @param path 路径
 * @returns 文件名
 */
export function ExtractFilename(path: string): string {
  // 匹配最后一个斜杠后的内容
  const match = path.match(/[^\\/]+$/);
  return match ? match[0] : '';
}
