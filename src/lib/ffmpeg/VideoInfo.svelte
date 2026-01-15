<script lang="ts">
  import type { VideoInfoType } from './type';
  import { ExtractFilename, FormatTime } from '../../utils';
  import SolarFileSmileLinear from '$lib/icons/SolarFileSmileLinear.svelte';
  import SolarClockSquareLinear from '$lib/icons/SolarClockSquareLinear.svelte';
  import RaphaelVideo from '$lib/icons/RaphaelVideo.svelte';
  import ArcticonsFps from '$lib/icons/ArcticonsFps.svelte';
  import MdiVideoHomeSystem from '$lib/icons/MdiVideoHomeSystem.svelte';
  import MdiHeadphones from '$lib/icons/MdiHeadphones.svelte';
  import MdiTransitConnectionHorizontal from '$lib/icons/MdiTransitConnectionHorizontal.svelte';

  // 1. 定义 Props 和默认值 (Svelte 5 方式)
  interface Props {
    canClose?: boolean;
    videoInfo?: VideoInfoType;
    class?: string; // 允许外部透传 class
  }

  let {
    videoInfo = {
      audio_codec: '',
      audio_sample_rate: 0,
      bitrate_kbps: 0,
      duration: 0,
      fps: 0,
      height: 0,
      path: '',
      video_codec: '',
      width: 0
    },
    class: className = ''
  }: Props = $props();
</script>

<div class="mt-1 rounded py-1 {className}">
  <div class="grid grid-cols-4 gap-0">
    <div class="p-2">
      <div class="flex items-center justify-center gap-1 text-sm font-medium text-gray-600">
        <SolarFileSmileLinear class="text-base" />
        <span>文件路径</span>
      </div>
      <div class="truncate text-xs text-center text-gray-500" title={videoInfo.path}>
        {ExtractFilename(videoInfo.path)}
      </div>
    </div>

    <div class="p-2">
      <div class="flex items-center justify-center gap-1 text-sm font-medium text-gray-600">
        <SolarClockSquareLinear class="text-base" />
        <span>时长</span>
      </div>
      <div class="text-xs text-center text-gray-500">
        {FormatTime(videoInfo.duration)}
      </div>
    </div>

    <div class="p-2">
      <div class="flex items-center justify-center gap-1 text-sm font-medium text-gray-600">
        <RaphaelVideo class="text-base" />
        <span>分辨率</span>
      </div>
      <div class="text-xs text-center text-gray-500">
        {videoInfo.width} x {videoInfo.height}
      </div>
    </div>

    <div class="p-2">
      <div class="flex items-center justify-center gap-1 text-sm font-medium text-gray-600">
        <ArcticonsFps class="text-base" />
        <span>帧率</span>
      </div>
      <div class="text-xs text-center text-gray-500">
        {videoInfo.fps} FPS
      </div>
    </div>
  </div>

  <hr class="border-t border-gray-200" />

  <div class="grid grid-cols-4 gap-0">
    <div class="p-2">
      <div class="flex items-center justify-center gap-1 text-sm font-medium text-gray-600">
        <MdiVideoHomeSystem class="text-base" />
        <span>视频编码</span>
      </div>
      <div class="text-xs text-center text-gray-500">
        {videoInfo.video_codec}
      </div>
    </div>

    <div class="p-2">
      <div class="flex items-center justify-center gap-1 text-sm font-medium text-gray-600">
        <MdiVideoHomeSystem class="text-base" />
        <span>音频编码</span>
      </div>
      <div class="text-xs text-center text-gray-500">
        {videoInfo.audio_codec}
      </div>
    </div>

    <div class="p-2">
      <div class="flex items-center justify-center gap-1 text-sm font-medium text-gray-600">
        <MdiHeadphones class="text-base" />
        <span>音频采样率</span>
      </div>
      <div class="text-xs text-center text-gray-500">
        {videoInfo.audio_sample_rate}
      </div>
    </div>

    <div class="p-2">
      <div class="flex items-center justify-center gap-1 text-sm font-medium text-gray-600">
        <MdiTransitConnectionHorizontal class="text-base" />
        <span>比特率</span>
      </div>
      <div class="text-xs text-center text-gray-500">
        {videoInfo.bitrate_kbps}
      </div>
    </div>
  </div>
</div>
