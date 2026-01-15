<script lang="ts">
  import type { VideoInfoType } from '$lib/ffmpeg/type';
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import { logger } from '../utils';
  import SelectFile from '$lib/common/SelectFile.svelte';
  import { getPlayerContext } from '$lib/state';

  const playerState = getPlayerContext();

  async function handleSelectVideo(select: string[]) {
    if (select.length === 0) {
      return;
    }

    const info = (await invoke('get_video_info', {
      videoPath: select[0]
    }).catch((e) => {
      logger.error(e);
      return null;
    })) as VideoInfoType | null;

    if (!info) {
      return;
    }

    if (
      info.video_codec === 'h264' ||
      info.video_codec === 'h265' ||
      info.video_codec === 'hevc' ||
      info.video_codec === 'av1' ||
      info.video_codec === 'vp9' ||
      info.video_codec === 'vp8'
    ) {
      playerState.open(convertFileSrc(select[0]));
      return
    }

    logger.error('不支持的视频格式', info.video_codec);
  }
</script>

<div class="flex flex-col gap-2 p-2">
  <div>使用webview播放视频</div>
  <div class="flex gap-2">
    <SelectFile
      fileName="video"
      extensions="mp4,mov,avi,wmv,flv,mkv,webm,mpeg,mpg"
      multiple={false}
      onSelect={handleSelectVideo}
    />
  </div>
</div>
