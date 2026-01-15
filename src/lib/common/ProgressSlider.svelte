<script>
  // 定义进度值，范围 0-100
  export let progress = 45;
  export let message = '';
  export let color = 'bg-blue-600';
</script>

<div class="mx-auto w-full p-4">
  <div class="mb-2 flex justify-between">
    <span class="truncate text-sm font-medium text-gray-700 dark:text-white" title={message}
      >{message}</span
    >
    <span class="text-sm font-medium text-gray-700 dark:text-white">{Math.floor(progress)}%</span>
  </div>

  <div class="h-1 w-full overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700">
    <div
      class="{color} h-1 rounded-full transition-all duration-500 ease-out"
      style="width: {progress}%"
    >
      <div
        class="h-full w-full animate-[progress-stripe_1s_linear_infinite] bg-sky-400 opacity-20"
      ></div>
    </div>
  </div>
</div>

<style>
  @keyframes progress-stripe {
    from {
      background-position: 1rem 0;
    }
    to {
      background-position: 0 0;
    }
  }
</style>
