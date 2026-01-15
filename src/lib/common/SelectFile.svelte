<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { logger } from '../../utils';
  import FileVideoOpen from '$lib/icons/FileVideoOpen.svelte';
  import ButtonLoading from '$lib/icons/ButtonLoading.svelte';

  // 1. 定义 Props (Svelte 5 方式)
  interface Props {
    class?: string;
    multiple?: boolean;
    fileName?: string;
    extensions?: string;
    // Svelte 通过回调函数处理 Emit
    onSelect?: (files: string[]) => void;
  }

  let {
    class: className = '',
    multiple = false,
    fileName = 'video',
    extensions = 'mp4,mov,avi,wmv,flv,mkv,webm,mpeg,mpg',
    onSelect
  }: Props = $props();

  // 2. 定义响应式状态
  let selecting = $state(false);

  // 3. 处理选择逻辑
  async function handleSelect() {
    if (selecting) return;

    selecting = true;

    try {
      const selected = await open({
        multiple,
        filters: [
          {
            name: fileName,
            extensions: extensions.split(',')
          }
        ]
      });

      if (selected) {
        // 处理单选和多选的情况
        const files = Array.isArray(selected) ? selected : [selected];
        onSelect?.(files);
      }
    } catch (e) {
      logger.error(e);
    } finally {
      selecting = false;
    }
  }
</script>

<button
  type="button"
  class="w-content me-2 inline-flex h-8 cursor-pointer items-center justify-center gap-2 rounded bg-sky-200 px-5 text-gray-700 transition-colors hover:bg-sky-400 disabled:opacity-50 {className}"
  disabled={selecting}
  onclick={handleSelect}
  aria-label="Open Folder"
>
  {#if selecting}
    <ButtonLoading />
    <span class="w-18">加载中...</span>
  {:else}
    <FileVideoOpen />
    <span class="w-18">选择文件</span>
  {/if}
</button>
