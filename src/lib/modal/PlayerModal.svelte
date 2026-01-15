<script lang="ts">
  import { getPlayerContext } from '$lib/state';
  import { fade } from 'svelte/transition';
  import { FormatTime } from '../../utils';
  import LineMdPlayFilled from '$lib/icons/LineMdPlayFilled.svelte';
  import LineMdPlayFilledToPauseTransition from '$lib/icons/LineMdPlayFilledToPauseTransition.svelte';
  import LineMdArrowSmallLeft from '$lib/icons/LineMdArrowSmallLeft.svelte';
  import MdiFullscreenExit from '$lib/icons/MdiFullscreenExit.svelte';
  import MdiFullscreen from '$lib/icons/MdiFullscreen.svelte';
  import LineMdVolumeLowFilled from '$lib/icons/LineMdVolumeLowFilled.svelte';
  import LineMdVolumeMediumFilled from '$lib/icons/LineMdVolumeMediumFilled.svelte';
  import LineMdVolumeHighFilled from '$lib/icons/LineMdVolumeHighFilled.svelte';

  const player = getPlayerContext();

  // 2. 状态定义
  let videoEl = $state<HTMLVideoElement>();
  let paused = $state(true);
  let duration = $state(0);
  let currentTime = $state(0);
  let volume = $state(1);
  let playbackRate = $state(1);
  let isFullscreen = $state(false);
  let showControls = $state(true);
  let showSpeedMenu = $state(false);
  let showVolumeSlider = $state(false);

  const speedList = [0.5, 0.75, 1, 1.25, 1.5, 2, 3, 4];

  // 3. 快捷键处理
  let preVolume = 1;

  function handleKeyDown(e: KeyboardEvent) {
    if (!player.show) return;

    // 阻止某些默认行为，防止空格翻页等
    const target = e.target as HTMLElement;
    if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA') return;

    switch (e.key.toLowerCase()) {
      case ' ':
        e.preventDefault();
        paused = !paused;
        break;
      case 'arrowleft':
        currentTime = Math.max(0, currentTime - 5);
        break;
      case 'arrowright':
        currentTime = Math.min(duration, currentTime + 5);
        break;
      case 'arrowup':
        e.preventDefault();
        volume = Math.min(1, volume + 0.1);
        break;
      case 'arrowdown':
        e.preventDefault();
        volume = Math.max(0, volume - 0.1);
        break;
      case 'm':
        if (volume > 0) {
          preVolume = volume;
          volume = 0;
        } else {
          volume = preVolume;
        }
        break;
      case 'f':
        toggleFullscreen();
        break;
      case 'escape':
      case 'q':
        player.close();
        break;
    }
  }

  function toggleFullscreen() {
    if (!document.fullscreenElement) {
      videoEl?.parentElement?.requestFullscreen();
      isFullscreen = true;
    } else {
      document.exitFullscreen();
      isFullscreen = false;
    }
  }

  // 5. 自动隐藏控制栏
  // let controlsTimeout: number;
  // function resetControlsTimeout() {
  //   showControls = true;
  //   clearTimeout(controlsTimeout);
  //   if (!paused) {
  //     controlsTimeout = window.setTimeout(() => (showControls = false), 3000);
  //   }
  // }
</script>

<svelte:window onkeydown={handleKeyDown} />

{#if player.show}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    transition:fade={{ duration: 200 }}
    class="fixed inset-0 z-999 flex items-center justify-center bg-black/50 p-4 backdrop-blur-sm"
  >
    <div
      class="absolute top-0 right-0 left-0 z-20 p-4 transition-opacity duration-300 {showControls
        ? 'opacity-100'
        : 'opacity-0'}"
    >
      <button
        onclick={() => player.close()}
        class="rounded-full bg-black/20 p-2 text-blue-500 transition-colors hover:bg-black/40"
        aria-label="返回"
      >
        <LineMdArrowSmallLeft class="text-2xl" />
      </button>
    </div>

    <video
      bind:this={videoEl}
      bind:paused
      bind:currentTime
      bind:duration
      bind:volume
      bind:playbackRate
      src={player.url}
      autoplay
      class="pointer-events-auto h-full w-full object-contain"
      onclick={() => (paused = !paused)}
    >
      <track kind="captions" />
    </video>

    <div
      class="absolute right-0 bottom-0 left-0 z-20 bg-linear-to-t from-black/80 to-transparent px-6 pt-10 pb-6 transition-opacity duration-300 {showControls
        ? 'opacity-100'
        : 'opacity-0'}"
    >
      <div class="group relative mb-4 flex items-center">
        <input
          type="range"
          min="0"
          max={duration}
          step="0.01"
          bind:value={currentTime}
          class="h-1.5 w-full cursor-pointer appearance-none rounded-lg bg-gray-600 accent-blue-500 transition-all group-hover:h-2"
        />
      </div>

      <div class="flex items-center gap-4 text-blue-500">
        <button onclick={() => (paused = !paused)} class="transition-transform hover:scale-110">
          {#if paused}<LineMdPlayFilled class="text-3xl" />{:else}<LineMdPlayFilledToPauseTransition
              class="text-3xl"
            />{/if}
        </button>

        <span class="min-w-30 font-mono text-sm">
          {FormatTime(currentTime)} / {FormatTime(duration)}
        </span>

        <div class="flex-1"></div>

        <div class="relative">
          <button
            onclick={() => (showSpeedMenu = !showSpeedMenu)}
            class="rounded border border-blue-500/50 px-2 py-0.5 text-sm font-bold hover:bg-blue-500/10"
          >
            {playbackRate}x
          </button>
          {#if showSpeedMenu}
            <div
              class="absolute bottom-full left-1/2 mb-2 -translate-x-1/2 overflow-hidden rounded border border-white/10 bg-black/80 py-1 backdrop-blur-md"
            >
              {#each speedList as s}
                <button
                  onclick={() => {
                    playbackRate = s;
                    showSpeedMenu = false;
                  }}
                  class="block w-full px-4 py-1 text-xs transition-colors hover:bg-blue-500 hover:text-white {playbackRate ===
                  s
                    ? 'bg-blue-500/30 text-white'
                    : ''}"
                >
                  {s}x
                </button>
              {/each}
            </div>
          {/if}
        </div>

        <div
          role="contentinfo"
          class="group relative flex items-center gap-2"
          onmouseenter={() => (showVolumeSlider = true)}
          onmouseleave={() => (showVolumeSlider = false)}
        >
          <button onclick={() => (volume = volume === 0 ? 0.5 : 0)}>
            {#if volume === 0}<LineMdVolumeLowFilled class="text-2xl" />
            {:else if volume < 0.5}<LineMdVolumeMediumFilled class="text-2xl" />
            {:else}<LineMdVolumeHighFilled class="text-2xl" />{/if}
          </button>

          <div
            class="overflow-hidden transition-all duration-300 {showVolumeSlider
              ? 'w-24 opacity-100'
              : 'w-0 opacity-0'}"
          >
            <input
              type="range"
              min="0"
              max="1"
              step="0.01"
              bind:value={volume}
              class="h-1 w-20 accent-blue-500"
            />
          </div>
        </div>

        <button onclick={toggleFullscreen}>
          {#if isFullscreen}<MdiFullscreenExit class="text-2xl" />{:else}<MdiFullscreen
              class="text-2xl"
            />{/if}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  /* 隐藏原生进度条样式，美化 Input Range */
  input[type='range'] {
    appearance: none;
    background: rgba(255, 255, 255, 0.2);
  }
  input[type='range']::-webkit-slider-thumb {
    -webkit-appearance: none;
    height: 12px;
    width: 12px;
    border-radius: 50%;
    background: #3b82f6; /* blue-500 */
    cursor: pointer;
  }
</style>
