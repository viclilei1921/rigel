<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import FFmpegAppend from '$lib/icons/FFmpegAppend.svelte';
  import FFmpegConvert from '$lib/icons/FFmpegConvert.svelte';
  import FFmpegHighlight from '$lib/icons/FFmpegHighlight.svelte';
  import FFmpegMerge from '$lib/icons/FFmpegMerge.svelte';
  let { children } = $props();

  const menus = [
    { label: '视频转换', path: '/ffmpeg/convert', icon: FFmpegConvert },
    { label: '精彩剪辑', path: '/ffmpeg/highlight', icon: FFmpegHighlight },
    { label: '视频合并', path: '/ffmpeg/merge', icon: FFmpegMerge },
    { label: '视频追加', path: '/ffmpeg/append', icon: FFmpegAppend }
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
