<script lang="ts">
  import type { ProgressPayloadInterface, VideoInfoType } from '$lib/ffmpeg/type';
  import { invoke } from '@tauri-apps/api/core';
  import { logger } from '../../../utils';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { onDestroy } from 'svelte';
  import SelectFile from '$lib/common/SelectFile.svelte';
  import ButtonLoading from '$lib/icons/ButtonLoading.svelte';
  import ArrowSwitch from '$lib/icons/ArrowSwitch.svelte';
  import VideoInfo from '$lib/ffmpeg/VideoInfo.svelte';
  import ProgressSlider from '$lib/common/ProgressSlider.svelte';
  import MdiDelete from '$lib/icons/MdiDelete.svelte';
  import StashFolderAltLight from '$lib/icons/StashFolderAltLight.svelte';

  type VideoConvertType = VideoInfoType & {
    outputPath: string;
    message: string;
    progress: number;
  };

  const videoArr = $state<VideoConvertType[]>([]);
  let converting = $state(-1);

  async function handleSelectVideo(select: string[]) {
    for (let i = 0; i < select.length; i++) {
      const item = select[i];

      const info = await invoke('get_video_info', {
        videoPath: item
      }).catch((e) => {
        logger.error(e);
        return null;
      });

      if (info) {
        videoArr.push({
          outputPath: `${item.split('.').slice(0, -1).join('.')}_converted.mp4`,
          message: 'wait',
          progress: 0,
          ...(info as VideoInfoType)
        });
      }
    }
  }

  async function handleConvert() {
    if (!videoArr.length) {
      return;
    }

    for (let i = 0; i < videoArr.length; i++) {
      const item = videoArr[i];

      converting = i;

      if (item.progress >= 100) {
        continue;
      }

      await invoke('convert_video_to_mp4', {
        videoPath: item.path,
        outputPath: item.outputPath
      }).catch((e) => {
        logger.error(e);
      });
    }

    converting = -1;
  }

  function handleOpenFolder(path: string) {
    invoke('reveal_in_explorer', {
      path
    }).catch((e) => {
      logger.error(e);
    });
  }

  let unlistenProgress: UnlistenFn | null = null;
  let unlistenComplete: UnlistenFn | null = null;

  async function initEvent() {
    unlistenProgress = await listen(
      'ffmpeg-progress',
      ({ payload }: { payload: ProgressPayloadInterface }) => {
        const video = videoArr[converting];
        if (!video) {
          return;
        }
        video.progress = payload.progress;
        video.message = payload.message;
      }
    );

    // 监听完成事件
    unlistenComplete = await listen('ffmpeg-complete', ({ payload }) => {
      const video = videoArr[converting];
      if (!video) {
        return;
      }
      video.progress = 100;
      video.message = `完成 code: ${(payload as any).code}`;
    });
  }

  function closeEvent() {
    unlistenProgress?.();
    unlistenComplete?.();
  }

  initEvent();

  onDestroy(closeEvent);
</script>

<div class="flex flex-col gap-2 p-2">
  <div>使用 ffmpeg 将视频转换为 mp4 格式</div>
  <div class="flex gap-2">
    {#if converting < 0}
      <SelectFile
        fileName="video"
        extensions="mp4,mov,avi,wmv,flv,mkv,webm,mpeg,mpg"
        multiple
        onSelect={handleSelectVideo}
      />
    {/if}
    {#if videoArr.length > 0}
      <button
        type="button"
        class="w-content me-2 inline-flex h-8 cursor-pointer items-center justify-center gap-2 rounded bg-sky-200 px-6 text-gray-700 transition-colors hover:bg-sky-300 disabled:opacity-50 disabled:cursor-not-allowed"
        disabled={converting >= 0}
        onclick={handleConvert}
      >
        {#if converting < 0}
          <ArrowSwitch />
          <span class="text-align-left w-18">开始转换</span>
        {:else}
          <ButtonLoading />
          <span class="text-align-left w-18">转换中...</span>
        {/if}
      </button>
    {/if}
  </div>
  <div class="grid grid-cols-[repeat(auto-fit,minmax(450px,500px))] justify-center gap-2">
    {#each videoArr as video, i}
      <div
        class="flex flex-col items-center overflow-hidden rounded-sm border border-gray-400 shadow-sm transition-shadow hover:shadow-md"
      >
        <VideoInfo videoInfo={video} />
        <div class="relative w-full max-w-md">
          <input
            bind:value={video.outputPath}
            placeholder="默认保存在原文件夹"
            class="peer w-full rounded-md border border-gray-300 bg-transparent p-2 focus:border-blue-600 focus:outline-none"
          />
          <label
            for="outputPath"
            class="absolute -top-2 left-2 bg-slate-100 px-1 text-xs text-gray-500 transition-all duration-200
            peer-focus:-top-2 peer-focus:text-xs peer-focus:text-blue-600"
          >
            输出路径 (可选)
          </label>
        </div>
        <div class="flex w-full items-center gap-2">
          <ProgressSlider progress={video.progress} message={video.message} color="bg-indigo-500" />
          {#if video.progress === 0}
            <button
              type="button"
              class="w-content text-sm me-2 inline-flex h-6 cursor-pointer items-center justify-center gap-1 rounded bg-sky-200 px-2 text-red-500 transition-colors hover:bg-sky-300 disabled:opacity-50"
              onclick={() => videoArr.splice(i, 1)}
            >
              <MdiDelete class="size-4" />
              <span class="text-align-left w-9">删除</span>
            </button>
          {/if}
          {#if video.progress === 100}
            <button
              type="button"
              class="w-content text-sm me-2 inline-flex h-6 cursor-pointer items-center justify-center gap-1 rounded bg-sky-200 px-2 text-gray-700 transition-colors hover:bg-sky-300 disabled:opacity-50"
              onclick={() => handleOpenFolder(video.outputPath)}
            >
              <StashFolderAltLight class="size-4" />
              <span class="text-align-left w-9">打开</span>
            </button>
          {/if}
        </div>
      </div>
    {/each}
  </div>
</div>
