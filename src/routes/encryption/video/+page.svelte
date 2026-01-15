<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import MdiEye from '$lib/icons/MdiEye.svelte';
  import MdiEyeOff from '$lib/icons/MdiEyeOff.svelte';
  import SelectFile from '$lib/common/SelectFile.svelte';
  import MdiClose from '$lib/icons/MdiClose.svelte';
  import { logger } from '../../../utils';
  import ButtonLoading from '$lib/icons/ButtonLoading.svelte';
  import MdiPlayCircleOutline from '$lib/icons/MdiPlayCircleOutline.svelte';
  import { getPlayerContext } from '$lib/state';

  const playerState = getPlayerContext();

  // 2. 响应式状态
  let videoPlayPath = $state('');
  let password = $state('');
  let showPassword = $state(false);
  let startServer = $state(false);
  let serverPath = $state('');

  // 3. 逻辑函数
  async function handleSelectVideo(select: string[]) {
    if (select.length > 0) {
      videoPlayPath = select[0];
    }
  }

  function handleCloseVideoPath() {
    videoPlayPath = '';
  }

  async function handleStartServer() {
    if (startServer) return;

    startServer = true;
    try {
      const res = await invoke<string>('start_video_stream', {
        password: password,
        path: videoPlayPath
      });

      if (res) {
        serverPath = res;
        // 更新全局播放器状态
        playerState.open(serverPath);
      }
    } catch (e) {
      logger.error(e);
    } finally {
      startServer = false;
    }
  }

  async function handleStopServer() {
    try {
      await invoke('stop_video_stream');
      serverPath = '';
    } catch (e) {
      logger.error(e);
    }
  }

  // 4. 侦听器 (对应 watch)
  // 当播放器窗口关闭时，自动停止后端服务
  $effect(() => {
    if (playerState.show === false) {
      handleStopServer();
    }
  });
</script>

<div class="container mx-auto max-w-2xl p-4">
  <div class="rounded-lg border border-gray-200 bg-white p-6 shadow-md">
    <h2 class="text-xl font-bold text-gray-800">播放加密视频</h2>
    <p class="mb-6 text-sm text-gray-500">使用 Rust 在后端创建一个 HTTP 服务，用于播放加密视频</p>

    <div class="space-y-4">
      <div class="min-h-10">
        {#if !videoPlayPath}
          <SelectFile class="mb-2" multiple={false} extensions="*.*" onSelect={handleSelectVideo} />
        {:else}
          <div
            class="mb-2 flex items-center justify-between rounded-md border border-gray-200 bg-gray-50 p-3"
          >
            <span class="mr-4 truncate text-sm text-gray-700">{videoPlayPath}</span>
            <button
              onclick={handleCloseVideoPath}
              class="rounded-full p-1 text-red-500 transition-colors hover:bg-red-50"
            >
              <MdiClose class="text-lg" />
            </button>
          </div>
        {/if}
      </div>

      <div class="relative">
        <div class="relative">
          <input
            type={showPassword ? 'text' : 'password'}
            bind:value={password}
            class="peer w-full rounded-md border border-gray-300 bg-transparent p-2 focus:border-blue-600 focus:outline-none"
          />
          <label
            for="password"
            class="absolute -top-2 left-2 bg-white px-1 text-xs text-gray-500 transition-all duration-200
            peer-focus:-top-2 peer-focus:text-xs peer-focus:text-blue-600"
          >
            输入密码
          </label>
          <button
            type="button"
            onclick={() => (showPassword = !showPassword)}
            class="absolute top-1/2 right-3 -translate-y-1/2 p-1 text-gray-400 hover:text-gray-600"
          >
            {#if showPassword}
              <MdiEye class="text-xl" />
            {:else}
              <MdiEyeOff class="text-xl" />
            {/if}
          </button>
        </div>
      </div>
    </div>

    <div class="mt-8 flex justify-end">
      <button
        onclick={handleStartServer}
        disabled={startServer || !password || !videoPlayPath}
        class="w-content me-2 inline-flex h-8 cursor-pointer items-center justify-center gap-2 rounded bg-sky-200 px-6 text-gray-700 transition-colors hover:bg-sky-300 disabled:cursor-not-allowed disabled:opacity-50"
      >
        {#if startServer}
          <ButtonLoading />
          <span class="text-align-left w-18">启动中...</span>
        {:else}
          <MdiPlayCircleOutline />
          <span class="text-align-left w-18">开始播放</span>
        {/if}
      </button>
    </div>
  </div>
</div>
