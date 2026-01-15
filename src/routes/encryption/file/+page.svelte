<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import MdiClose from '$lib/icons/MdiClose.svelte';
  import SelectFile from '$lib/common/SelectFile.svelte';
  import MdiEye from '$lib/icons/MdiEye.svelte';
  import MdiEyeOff from '$lib/icons/MdiEyeOff.svelte';
  import { logger } from '../../../utils';
  import MdiDecryptedVariant from '$lib/icons/MdiDecryptedVariant.svelte';
  import MdiEncryption from '$lib/icons/MdiEncryption.svelte';
  import ButtonLoading from '$lib/icons/ButtonLoading.svelte';
  import ProgressSlider from '$lib/common/ProgressSlider.svelte';

  // --- 加密状态 ---
  let encrypt = $state({
    ing: false,
    password: '',
    showPassword: false,
    filePath: '',
    outPath: '',
    progress: 0
  });

  // --- 解密状态 ---
  let decrypt = $state({
    ing: false,
    password: '',
    showPassword: false,
    filePath: '',
    outPath: '',
    progress: 0
  });

  // --- 事件处理 ---
  function handleSelectFileToEncrypt(select: string[]) {
    if (select.length === 0) return;
    encrypt.filePath = select[0];
    encrypt.outPath = `${select[0]}.enc`;
  }

  function handleCloseEncrypt() {
    encrypt.filePath = '';
    encrypt.outPath = '';
    encrypt.progress = 0;
  }

  async function handleEncrypt() {
    if (!encrypt.password || !encrypt.filePath || !encrypt.outPath) return;
    encrypt.ing = true;
    try {
      await invoke('encrypt_file', {
        inputPath: encrypt.filePath,
        outputPath: encrypt.outPath,
        password: encrypt.password
      });
    } catch (e) {
      logger.error(e);
      encrypt.ing = false;
    }
  }

  function handleSelectFileToDecrypt(select: string[]) {
    if (select.length === 0) return;
    decrypt.filePath = select[0];
    decrypt.outPath = select[0].split('.').slice(0, -1).join('.');
  }

  function handleCloseDecrypt() {
    decrypt.filePath = '';
    decrypt.outPath = '';
    decrypt.progress = 0;
  }

  async function handleDecrypt() {
    if (!decrypt.password || !decrypt.filePath || !decrypt.outPath) return;
    decrypt.ing = true;
    try {
      await invoke('decrypt_file', {
        inputPath: decrypt.filePath,
        outputPath: decrypt.outPath,
        password: decrypt.password
      });
    } catch (e) {
      logger.error(e);
      decrypt.ing = false;
    }
  }

  // --- 生命周期与 Tauri 事件 ---
  let unlistenProgress: UnlistenFn;
  let unlistenError: UnlistenFn;

  onMount(async () => {
    unlistenProgress = await listen('encrypt_progress', ({ payload }) => {
      const p = payload as number;
      if (encrypt.ing) {
        encrypt.progress = p;
        if (p === 100) encrypt.ing = false;
      }
      if (decrypt.ing) {
        decrypt.progress = p;
        if (p === 100) decrypt.ing = false;
      }
    });

    unlistenError = await listen('encrypt_error', ({ payload }) => {
      logger.error(payload);
      encrypt.ing = false;
      decrypt.ing = false;
    });
  });

  onDestroy(() => {
    unlistenProgress?.();
    unlistenError?.();
  });
</script>

<div class="container mx-auto max-w-2xl space-y-6 p-4">
  <section class="rounded-lg border border-gray-200 bg-white p-6 shadow-md">
    <h2 class="text-xl font-bold text-gray-800">加密文件</h2>
    <p class="mb-4 text-sm text-gray-500">使用 chacha20 流式加密文件</p>

    <div class="mb-4">
      {#if !encrypt.filePath}
        <SelectFile multiple={false} extensions="*.*" onSelect={handleSelectFileToEncrypt} />
      {:else}
        <div class="flex items-center rounded border bg-gray-50 p-2">
          <span class="flex-1 truncate text-sm text-gray-700">{encrypt.filePath}</span>
          <button
            onclick={handleCloseEncrypt}
            class="ml-2 rounded p-1 text-red-500 hover:bg-red-50"
          >
            <MdiClose />
          </button>
        </div>
      {/if}
    </div>

    <div class="space-y-4">
      <div class="relative">
        <input
          type={encrypt.showPassword ? 'text' : 'password'}
          bind:value={encrypt.password}
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
          class="absolute top-2.5 right-3 text-gray-400"
          onclick={() => (encrypt.showPassword = !encrypt.showPassword)}
        >
          {#if encrypt.showPassword}<MdiEye />{:else}<MdiEyeOff />{/if}
        </button>
      </div>
      <div class="relative">
        <input
          type="text"
          bind:value={encrypt.outPath}
          class="peer w-full rounded-md border border-gray-300 bg-transparent p-2 focus:border-blue-600 focus:outline-none"
        />
        <label
          for="outPath"
          class="absolute -top-2 left-2 bg-white px-1 text-xs text-gray-500 transition-all duration-200
            peer-focus:-top-2 peer-focus:text-xs peer-focus:text-blue-600"
        >
          输出路径 (默认保存在原文件夹)
        </label>
      </div>

      {#if encrypt.progress > 0}
        <ProgressSlider progress={encrypt.progress} color="bg-indigo-500" />
      {/if}
    </div>

    <div class="mt-6 flex justify-end">
      <button
        onclick={handleEncrypt}
        disabled={encrypt.ing || !encrypt.password || !encrypt.filePath}
        class="w-content me-2 inline-flex h-8 cursor-pointer items-center justify-center gap-2 rounded bg-sky-200 px-6 text-gray-700 transition-colors hover:bg-sky-300 disabled:cursor-not-allowed disabled:opacity-50"
      >
        {#if encrypt.ing}
          <ButtonLoading />
          <span class="text-align-left w-18">加密中...</span>
        {:else}
          <MdiEncryption />
          <span class="text-align-left w-18">开始加密</span>
        {/if}
      </button>
    </div>
  </section>

  <section class="rounded-lg border border-gray-200 bg-white p-6 shadow-md">
    <h2 class="text-xl font-bold text-gray-800">解密文件</h2>
    <p class="mb-4 text-sm text-gray-500">使用 chacha20 流式解密文件</p>

    <div class="mb-4">
      {#if !decrypt.filePath}
        <SelectFile multiple={false} extensions="*.*" onSelect={handleSelectFileToDecrypt} />
      {:else}
        <div class="flex items-center rounded border bg-gray-50 p-2">
          <span class="flex-1 truncate text-sm text-gray-700">{decrypt.filePath}</span>
          <button
            onclick={handleCloseDecrypt}
            class="ml-2 rounded p-1 text-red-500 hover:bg-red-50"
          >
            <MdiClose />
          </button>
        </div>
      {/if}
    </div>

    <div class="space-y-4">
      <div class="relative">
        <input
          type={decrypt.showPassword ? 'text' : 'password'}
          bind:value={decrypt.password}
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
          class="absolute top-2.5 right-3 text-gray-400"
          onclick={() => (decrypt.showPassword = !decrypt.showPassword)}
        >
          {#if decrypt.showPassword}<MdiEye />{:else}<MdiEyeOff />{/if}
        </button>
      </div>
      <div class="relative">
        <input
          type="text"
          bind:value={decrypt.outPath}
          class="peer w-full rounded-md border border-gray-300 bg-transparent p-2 focus:border-blue-600 focus:outline-none"
        />
        <label
          for="outPath"
          class="absolute -top-2 left-2 bg-white px-1 text-xs text-gray-500 transition-all duration-200
            peer-focus:-top-2 peer-focus:text-xs peer-focus:text-blue-600"
        >
          输出路径 (默认保存在原文件夹)
        </label>
      </div>

      {#if decrypt.progress > 0}
        <ProgressSlider progress={decrypt.progress} color="bg-indigo-500" />
      {/if}
    </div>

    <div class="mt-6 flex justify-end">
      <button
        onclick={handleDecrypt}
        disabled={decrypt.ing || !decrypt.password || !decrypt.filePath}
        class="w-content me-2 inline-flex h-8 cursor-pointer items-center justify-center gap-2 rounded bg-sky-200 px-6 text-gray-700 transition-colors hover:bg-sky-300 disabled:cursor-not-allowed disabled:opacity-50"
      >
        {#if encrypt.ing}
          <ButtonLoading />
          <span class="text-align-left w-18">解密中...</span>
        {:else}
          <MdiDecryptedVariant />
          <span class="text-align-left w-18">开始解密</span>
        {/if}
      </button>
    </div>
  </section>
</div>
