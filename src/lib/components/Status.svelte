<script lang="ts">
  let { running, label }: { running: boolean; label: string } = $props();
</script>

<span class="inline-flex items-center gap-2 text-xs font-semibold text-neutral-600 dark:text-neutral-300">
  <span
    class="h-2.5 w-2.5 rounded-full"
    class:bg-green-500={running}
    class:bg-neutral-400={!running}
  ></span>
  {label}: {running ? "Running" : "Stopped"}
</span>
