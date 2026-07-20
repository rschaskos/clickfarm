<script lang="ts">
  import { appState } from "$lib/stores";
  import { startKeyBinder, stopKeyBinder, saveConfig } from "$lib/api";
  import Status from "$lib/components/Status.svelte";

  let error = $state("");

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
    try {
      if ($appState.keyBinderRunning) {
        await stopKeyBinder();
      } else {
        await startKeyBinder($appState.keys, $appState.keyInterval);
      }
      $appState.keyBinderRunning = !$appState.keyBinderRunning;
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

  <button
    onclick={toggle}
    class:bg-red-600={$appState.keyBinderRunning}
    class:bg-blue-600={!$appState.keyBinderRunning}
    class="rounded-lg py-2 font-semibold text-white hover:opacity-90 transition"
  >
    {$appState.keyBinderRunning ? "Stop Key Binder" : "Start Key Binder"}
  </button>

  {#if error}
    <p class="text-sm text-red-600">{error}</p>
  {/if}
</section>
