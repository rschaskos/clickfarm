<script lang="ts">
  import { appState } from "$lib/stores";
  import { startKeyBinder, stopKeyBinder, saveConfig } from "$lib/api";
  import Status from "$lib/components/Status.svelte";

  let error = $state("");
  let startDelay = $state(3);
  let countdown = $state(0);
  let cancelRequested = false;

  async function persist() {
    await saveConfig({
      clicker_speed: $appState.clickerSpeed,
      clicker_button: $appState.clickerButton,
      keys: $appState.keys,
      key_interval: $appState.keyInterval,
    }).catch(() => {});
  }

  async function toggle() {
    error = "";

    if (countdown > 0) {
      cancelRequested = true;
      countdown = 0;
      return;
    }

    if ($appState.keyBinderRunning) {
      try {
        await stopKeyBinder();
      } catch (e) {
        error = String(e);
      }
      return;
    }

    cancelRequested = false;
    for (let i = startDelay; i > 0; i--) {
      countdown = i;
      await new Promise((r) => setTimeout(r, 1000));
      if (cancelRequested) {
        countdown = 0;
        return;
      }
    }
    countdown = 0;

    try {
      await startKeyBinder($appState.keys, $appState.keyInterval);
    } catch (e) {
      error = String(e);
    }
  }
</script>

<section class="rounded-xl border border-neutral-200 dark:border-neutral-700 bg-white dark:bg-neutral-800 p-5 flex flex-col gap-4">
  <div class="flex items-center justify-between">
    <h2 class="text-lg font-semibold text-neutral-900 dark:text-neutral-100">Key Binder</h2>
    <Status running={$appState.keyBinderRunning} label="Keys" />
  </div>

  <label class="flex flex-col gap-1 text-sm text-neutral-600 dark:text-neutral-300">
    Keys to bind
    <input
      type="text"
      bind:value={$appState.keys}
      onchange={persist}
      disabled={$appState.keyBinderRunning}
      placeholder="e.g. bvcxz"
      class="rounded-md border border-neutral-300 dark:border-neutral-600 bg-transparent px-2 py-1"
    />
  </label>

  <label class="flex flex-col gap-1 text-sm text-neutral-600 dark:text-neutral-300">
    Interval: {$appState.keyInterval}ms
    <input
      type="range"
      min="100"
      max="5000"
      step="100"
      bind:value={$appState.keyInterval}
      onchange={persist}
      disabled={$appState.keyBinderRunning}
      class="accent-blue-600"
    />
  </label>

  <label class="flex flex-col gap-1 text-sm text-neutral-600 dark:text-neutral-300">
    Start delay: {startDelay}s
    <input
      type="range"
      min="0"
      max="10"
      step="1"
      bind:value={startDelay}
      disabled={$appState.keyBinderRunning || countdown > 0}
      class="accent-blue-600"
    />
  </label>

  <button
    onclick={toggle}
    class:bg-amber-500={countdown > 0}
    class:bg-red-600={countdown === 0 && $appState.keyBinderRunning}
    class:bg-blue-600={countdown === 0 && !$appState.keyBinderRunning}
    class="mt-auto rounded-lg py-2 font-semibold text-white hover:opacity-90 transition"
  >
    {#if countdown > 0}
      Starting in {countdown}s… (click to cancel)
    {:else}
      {$appState.keyBinderRunning ? "Stop Key Binder" : "Start Key Binder"}
    {/if}
  </button>

  {#if error}
    <p class="text-sm text-red-600">{error}</p>
  {/if}
</section>
