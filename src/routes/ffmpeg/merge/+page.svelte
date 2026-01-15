<script lang="ts">
  import type { ProgressPayloadInterface, VideoInfoType } from '$lib/ffmpeg/type';
  import { onDestroy } from 'svelte';
  import { logger } from '../../../utils';
  import { invoke } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import ButtonLoading from '$lib/icons/ButtonLoading.svelte';
  import SelectFile from '$lib/common/SelectFile.svelte';
  import ProgressSlider from '$lib/common/ProgressSlider.svelte';
  import MdiDelete from '$lib/icons/MdiDelete.svelte';
  import StashFolderAltLight from '$lib/icons/StashFolderAltLight.svelte';
  import ArrowSwitch from '$lib/icons/ArrowSwitch.svelte';
  import VideoInfo from '$lib/ffmpeg/VideoInfo.svelte';

  const videoArr = $state<VideoInfoType[]>([]);
  let outputPath = $state<string>('');
  let merging = $state(false);
  let progress = $state(0);
  let message = $state('');

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
        videoArr.push(info as VideoInfoType);
      }
    }

    if (videoArr.length) {
      outputPath = `${videoArr[0].path.split('.').slice(0, -1).join('.')}_.mp4`;
    }
  }

  async function handleMerge() {
    if (!videoArr.length) {
      return;
    }

    if (merging) {
      return;
    }

    merging = true;

    await invoke('merge_smart', {
      inputs: videoArr.map((item) => item.path),
      outputPath: outputPath
    }).catch((e) => {
      logger.error(e);
    });

    merging = false;
  }

  function handleOpenFolder() {
    invoke('reveal_in_explorer', {
      path: outputPath
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
        progress = payload.progress;
        message = payload.message;
      }
    );

    // 监听完成事件
    unlistenComplete = await listen('ffmpeg-complete', ({ payload }) => {
      progress = 100;
      message = `完成 code: ${(payload as any).code}`;
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
  <div>
    使用 ffmpeg 将多个视频合成为一个视频, 并将视频名称绘制在左上角, 帧率使用中位数, 尺寸用最大尺寸
  </div>
  <div class="flex gap-2">
    {#if progress <= 0 || progress >= 100}
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
        disabled={merging}
        onclick={handleMerge}
      >
        {#if !merging}
          <ArrowSwitch />
          <span class="text-align-left w-18">开始合并</span>
        {:else}
          <ButtonLoading />
          <span class="text-align-left w-18">合并中...</span>
        {/if}
      </button>
    {/if}
  </div>
  {#if videoArr.length > 0}
    <div
      class="flex flex-col pt-2 items-center overflow-hidden rounded-sm border border-gray-400 shadow-sm transition-shadow hover:shadow-md"
    >
      <div class="relative w-full max-w-md">
        <input
          bind:value={outputPath}
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
      <div class="flex w-full items-center justify-center gap-2">
        <ProgressSlider {progress} {message} color="bg-indigo-500" />
        {#if progress === 100}
          <button
            type="button"
            class="w-content me-2 inline-flex h-6 cursor-pointer items-center justify-center gap-1 rounded bg-sky-200 px-2 text-sm text-gray-700 transition-colors hover:bg-sky-300 disabled:opacity-50"
            onclick={handleOpenFolder}
          >
            <StashFolderAltLight class="size-4" />
            <span class="text-align-left w-9">打开</span>
          </button>
        {/if}
      </div>
    </div>
  {/if}
  <div class="grid grid-cols-[repeat(auto-fit,minmax(450px,500px))] justify-center gap-2">
    {#each videoArr as video, i}
      <div
        class="flex flex-col items-center overflow-hidden rounded-sm border border-gray-400 shadow-sm transition-shadow hover:shadow-md"
      >
        <VideoInfo videoInfo={video} />
        <button
          type="button"
          class="w-content me-2 mb-2 inline-flex h-6 cursor-pointer items-center justify-center gap-1 rounded bg-sky-200 px-2 text-sm text-red-500 transition-colors hover:bg-sky-300 disabled:opacity-50"
          onclick={() => videoArr.splice(i, 1)}
        >
          <MdiDelete class="size-4" />
          <span class="text-align-left w-9">删除</span>
        </button>
      </div>
    {/each}
  </div>
</div>
