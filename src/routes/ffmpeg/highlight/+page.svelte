<script lang="ts">
  import type { ProgressPayloadInterface, VideoInfoType, VideoSegmentType } from '$lib/ffmpeg/type';
  import { invoke } from '@tauri-apps/api/core';
  import { FormatTime, logger } from '../../../utils';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { onDestroy } from 'svelte';
  import SelectFile from '$lib/common/SelectFile.svelte';
  import ButtonLoading from '$lib/icons/ButtonLoading.svelte';
  import SolarVideoFrameCutBold from '$lib/icons/SolarVideoFrameCutBold.svelte';
  import VideoInfo from '$lib/ffmpeg/VideoInfo.svelte';
  import ProgressSlider from '$lib/common/ProgressSlider.svelte';
  import MdiDelete from '$lib/icons/MdiDelete.svelte';
  import StashFolderAltLight from '$lib/icons/StashFolderAltLight.svelte';
  import SolarAddCircleLinear from '$lib/icons/SolarAddCircleLinear.svelte';
  import TimeInput from '$lib/common/TimeInput.svelte';

  let videoInfo = $state<VideoInfoType | null>(null);
  let clips = $state<VideoSegmentType[]>([]);
  let outputPath = $state<string>('');
  let progress = $state(0);
  let message = $state('');
  let highlighting = $state(false);

  async function handleSelectVideo(select: string[]) {
    if (select.length === 0) {
      return;
    }

    const info = await invoke('get_video_info', {
      videoPath: select[0]
    }).catch((e) => {
      logger.error(e);
      return null;
    });

    if (!info) {
      return;
    }

    videoInfo = info as VideoInfoType;
    outputPath = `${select[0].split('.').slice(0, -1).join('.')}_.mp4`;
  }

  function addClip() {
    const segment = { start: 0, end: 5 };
    const len = clips.length;

    if (len > 0) {
      segment.start = clips[len - 1].end + 5;
      segment.end = segment.start + 5;
    }

    clips.push(segment);
  }

  function removeClip(index: number) {
    clips.splice(index, 1);
  }

  async function handleHighlight() {
    if (!videoInfo) {
      return;
    }

    highlighting = true;

    await invoke('create_highlight_video', {
      videoPath: videoInfo.path,
      outputPath: outputPath,
      segments: clips.map(({ start, end }) => ({
        start: FormatTime(start),
        duration: FormatTime(end - start)
      }))
    }).catch((e) => {
      logger.error(e);
    });

    highlighting = false;
  }

  function handleOpenFolder() {
    invoke('reveal_in_explorer', {
      path: outputPath
    }).catch((e) => {
      logger.error(e);
    });
  }

  function handleReset() {
    videoInfo = null;
    clips = [];
    outputPath = '';
    progress = 0;
    message = '';
    highlighting = false;
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
  <div>使用 ffmpeg 将视频中精彩部分提取出来合成为一个视频</div>
  <div class="flex gap-2">
    {#if videoInfo === null}
      <SelectFile
        fileName="video"
        extensions="mp4,mov,avi,wmv,flv,mkv,webm,mpeg,mpg"
        multiple={false}
        onSelect={handleSelectVideo}
      />
    {:else}
      <button
        type="button"
        class="w-content me-2 inline-flex h-8 cursor-pointer items-center justify-center gap-2 rounded bg-sky-200 px-6 text-gray-700 transition-colors hover:bg-sky-300 disabled:opacity-50 disabled:cursor-not-allowed"
        disabled={highlighting}
        onclick={handleHighlight}
      >
        {#if highlighting === false}
          <SolarVideoFrameCutBold />
          <span class="text-align-left w-18">开始剪辑</span>
        {:else}
          <ButtonLoading />
          <span class="text-align-left w-18">剪辑中...</span>
        {/if}
      </button>
    {/if}
  </div>
  {#if videoInfo}
    <div
      class="flex flex-col items-center overflow-hidden rounded-sm border border-gray-400 shadow-sm transition-shadow hover:shadow-md"
    >
      <VideoInfo {videoInfo} />
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
        {#if progress === 0}
          <button
            type="button"
            class="w-content me-2 inline-flex h-6 cursor-pointer items-center justify-center gap-1 rounded bg-sky-200 px-2 text-sm text-red-500 transition-colors hover:bg-sky-300 disabled:opacity-50"
            onclick={handleReset}
          >
            <MdiDelete class="size-4" />
            <span class="text-align-left w-9">删除</span>
          </button>
        {/if}
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
    <div class="justify-left flex items-center gap-5">
      <span class="text-md font-semibold text-gray-800">剪辑片段列表</span>
      <button
        type="button"
        class="w-content me-2 inline-flex h-8 cursor-pointer items-center justify-center gap-2 rounded bg-sky-200 px-6 text-gray-700 transition-colors hover:bg-sky-300"
        onclick={addClip}
      >
        <SolarAddCircleLinear class="text-lg" />
        <span>添加片段</span>
      </button>
    </div>
    <div class="mt-2 grid grid-cols-[repeat(auto-fit,minmax(320px,350px))] justify-center gap-2">
      {#each clips as clip, index}
        <div class="col-span-1 grid grid-cols-5 items-center gap-2">
          <div class="col-span-2">
            <TimeInput label="start" priTime={clip.start} onchange={(t) => (clip.start = t)} />
          </div>

          <div class="col-span-2">
            <TimeInput label="end" priTime={clip.end} onchange={(t) => (clip.end = t)} />
          </div>

          <div class="col-span-1 flex justify-center">
            <button
              type="button"
              onclick={() => removeClip(index)}
              class="cursor-pointer rounded-md bg-sky-200 p-2 text-red-500 transition-colors hover:bg-sky-300"
              aria-label="删除"
            >
              <MdiDelete class="text-xl" />
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
