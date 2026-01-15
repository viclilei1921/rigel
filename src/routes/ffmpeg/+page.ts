import { redirect } from '@sveltejs/kit';

export function load() {
  // 状态码 307 是临时重定向，308 是永久重定向
  throw redirect(307, '/ffmpeg/convert');
}
