<script lang="ts">
  import { logger, WebGPUDrawer } from '../../../utils';
  import SelectFile from '$lib/common/SelectFile.svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';

  let CanvasDom = $state<HTMLCanvasElement | null>(null);

  function handleSelectImage(select: string[]) {
    const imagePath = select[0] || '';

    init(convertFileSrc(imagePath));
  }

  async function init(path: string) {
    if (!CanvasDom) {
      return;
    }

    const image = new Image();
    image.crossOrigin = 'anonymous';
    image.src = path;
    image.onerror = () => {
      logger.error('image load error');
      image.onerror = null;
    };
    await image.decode();
    image.onerror = null;

    CanvasDom.style.width = `${image.width}px`;
    CanvasDom.style.height = `${image.height}px`;

    CanvasDom.width = Math.floor(image.width * window.devicePixelRatio);
    CanvasDom.height = Math.floor(image.height * window.devicePixelRatio);

    const drawer = new WebGPUDrawer(CanvasDom);
    await drawer.init();

    const bitmap = await createImageBitmap(image);

    const texture = await drawer.createTextureFromImage(bitmap);

    if (!texture) {
      return;
    }

    drawer.draw(texture, 0, 0, CanvasDom.width, CanvasDom.height);
  }
</script>

<div class="flex flex-col gap-2 p-2">
  <div class="flex justify-left gap-2">
    <SelectFile
      fileName="image"
      extensions="jpg,jpeg,png,JPG,webp"
      multiple={false}
      onSelect={handleSelectImage}
    />
  </div>
  <canvas bind:this={CanvasDom}></canvas>
</div>
