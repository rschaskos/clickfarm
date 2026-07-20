<script lang="ts">
  import { appState } from "$lib/stores";
  import { startClicker, stopClicker, saveConfig } from "$lib/api";
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
      if ($appState.clickerRunning) {
        await stopClicker();
      } else {
        await startClicker($appState.clickerSpeed, $appState.clickerButton);
      }
      $appState.clickerRunning = !$appState.clickerRunning;
    } catch (e) {
      error = String(e);
    }
  }
</script>

<section class="rounded-xl border border-neutral-200 dark:border-neutral-700 bg-white dark:bg-neutral-800 p-5 flex flex-col gap-4">
  <div class="flex items-center justify-between">
    <h2 class="text-lg font-semibold text-neutral-900 dark:text-neutral-100">Autoclicker</h2>
    <Status running={$appState.clickerRunning} label="Clicker" />
  </div>

  <label class="flex flex-col gap-1 text-sm text-neutral-600 dark:text-neutral-300">
    Speed: {$appState.clickerSpeed}ms
    <input
      type="range"
      min="10"
      max="1000"
      step="10"
      bind:value={$appState.clickerSpeed}
      onchange={persist}
      disabled={$appState.clickerRunning}
      class="accent-blue-600"
    />
  </label>

  <fieldset class="flex gap-4 text-sm text-neutral-600 dark:text-neutral-300">
    <legend class="sr-only">Mouse button</legend>
    <label class="flex items-center gap-2">
      <input
        type="radio"
        name="clicker-button"
        value="left"
        bind:group={$appState.clickerButton}
        onchange={persist}
        disabled={$appState.clickerRunning}
      />
      Left
    </label>
    <label class="flex items-center gap-2">
      <input
        type="radio"
        name="clicker-button"
        value="right"
        bind:group={$appState.clickerButton}
        onchange={persist}
        disabled={$appState.clickerRunning}
      />
      Right
    </label>
  </fieldset>

  <button
    onclick={toggle}
    class:bg-red-600={$appState.clickerRunning}
    class:bg-blue-600={!$appState.clickerRunning}
    class="rounded-lg py-2 font-semibold text-white hover:opacity-90 transition"
  >
    {$appState.clickerRunning ? "Stop Clicker" : "Start Clicker"}
  </button>

  {#if error}
    <p class="text-sm text-red-600">{error}</p>
  {/if}
</section>
