<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import MdiEncryptionQuestion from '$lib/icons/MdiEncryptionQuestion.svelte';
  import SolarVideoFrame2Broken from '$lib/icons/SolarVideoFrame2Broken.svelte';
  let { children } = $props();

  const menus = [
    { label: '播放加密视频', path: '/encryption/video', icon: SolarVideoFrame2Broken},
    { label: '文件加解密', path: '/encryption/file', icon: MdiEncryptionQuestion},
  ];
</script>

<div class="flex size-full shrink-0 flex-col bg-amber-200">
  <div
    class="flex h-12 border-b border-gray-500 bg-blue-100 text-gray-700 transition-all duration-200"
  >
    {#each menus as { label, path, icon: Icon }}
      <button
        class="flex cursor-pointer items-center justify-center gap-2 rounded-t-sm border-b-2 border-blue-100 px-5 transition-all duration-200 hover:border-blue-500 hover:bg-green-500 hover:text-white {page
          .url.pathname === path && 'border-blue-500 bg-green-300'}"
        onclick={() => goto(path)}
      >
        <Icon />
        <span>{label}</span>
      </button>
    {/each}
  </div>
  <div class="scrollbar-custom w-full flex-1 overflow-auto bg-slate-100">
    {@render children()}
  </div>
</div>