<script lang="ts">
  import './layout.css';
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import MenuLeftIcon from '../lib/icons/MenuLeftIcon.svelte';
  import HomeIcon from '../lib/icons/HomeIcon.svelte';
  import FFmpegIcon from '../lib/icons/FFmpegIcon.svelte';
  import MenuRightIcon from '../lib/icons/MenuRightIcon.svelte';
  import EncryptionIcon from '../lib/icons/EncryptionIcon.svelte';
  import ImageIcon from '../lib/icons/ImageIcon.svelte';
  import SettingIcon from '../lib/icons/SettingIcon.svelte';
  import { logger, platformDetector, startWebVitalObserver } from '../utils';
  import { setPlayerContext } from '$lib/state';
  import PlayerModal from '$lib/modal/PlayerModal.svelte';

  let collapsed = $state(false);

  let { children } = $props();

  const menus = [
    { label: 'Home', path: '/', icon: HomeIcon },
    { label: 'ffmpeg', path: '/ffmpeg', icon: FFmpegIcon },
    { label: 'encryption', path: '/encryption', icon: EncryptionIcon },
    { label: 'image', path: '/image', icon: ImageIcon },
    { label: 'settings', path: '/settings', icon: SettingIcon }
  ];

  function handleToggle() {
    collapsed = !collapsed;
  }

  function currentMenu() {
    if (page.url.pathname === '/') {
      return 'Home';
    }

    return page.url.pathname.split('/')[1];
  }

  async function init() {
    // 初始化context
    setPlayerContext();

    await logger.init();

    startWebVitalObserver();

    platformDetector.init();
  }

  init();
</script>

<div class="flex h-screen w-screen">
  <aside
    class="border-r border-gray-500 bg-green-50 text-gray-700 transition-all duration-200"
    class:w-12={collapsed}
    class:w-50={!collapsed}
  >
    <div class="flex h-12 items-center justify-center gap-20 border-b border-gray-500">
      {#if !collapsed}
        <span class="font-semibold">Rigel</span>
      {/if}
      <button class="cursor-pointer rounded-sm p-2 hover:bg-gray-200" onclick={handleToggle}>
        {#if collapsed}
          <MenuRightIcon />
        {:else}
          <MenuLeftIcon />
        {/if}
      </button>
    </div>

    <nav class="flex flex-col justify-center">
      {#each menus as { label, path, icon: Icon }}
        <button
          class="justify-left flex cursor-pointer items-center gap-2 p-2 pl-3 hover:bg-green-500 hover:text-white transition-all duration-200"
          class:bg-green-300={currentMenu() === label}
          onclick={() => goto(path)}
        >
          <Icon />
          {#if !collapsed}
            <span class="text-left">{label}</span>
          {/if}
        </button>
      {/each}
    </nav>
  </aside>

  <main class="flex-1 overflow-hidden bg-slate-100">
    {@render children()}
  </main>

  <PlayerModal />
</div>
