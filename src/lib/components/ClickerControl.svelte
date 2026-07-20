<script lang="ts">
  import { appState } from "$lib/stores";
  import { startClicker, stopClicker, saveConfig } from "$lib/api";
  import Status from "$lib/components/Status.svelte";

  let error = $state("");
  let startDelay = $state(3);
  let countdown = $state(0);
  let cancelRequested = false;

  function msToS(ms: number) {
    return +(ms / 1000).toFixed(1);
  }
  function sToMs(s: number) {
    return Math.round(s * 1000);
  }

  async function persist() {
    await saveConfig({
      clicker_speed: $appState.clickerSpeed,
      clicker_button: $appState.clickerButton,
      key_binds: $appState.keyBinds.map((b) => ({ key: b.key, interval_ms: b.intervalMs })),
      key_interval_min_ms: $appState.keyIntervalMin,
      key_interval_max_ms: $appState.keyIntervalMax,
    }).catch(() => {});
  }

  function onSpeedInput(e: Event) {
    $appState.clickerSpeed = sToMs(+(e.currentTarget as HTMLInputElement).value);
    persist();
  }

  async function toggle() {
    error = "";

    if (countdown > 0) {
      cancelRequested = true;
      countdown = 0;
      return;
    }

    if ($appState.clickerRunning) {
      try {
        await stopClicker();
      } catch (e) {
        error = String(e);
      }
      return;
    }

    cancelRequested = false;
    let remainingMs = sToMs(startDelay);
    while (remainingMs > 0) {
      countdown = msToS(remainingMs);
      await new Promise((r) => setTimeout(r, 100));
      remainingMs -= 100;
      if (cancelRequested) {
        countdown = 0;
        return;
      }
    }
    countdown = 0;

    try {
      await startClicker($appState.clickerSpeed, $appState.clickerButton);
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
    Speed: {msToS($appState.clickerSpeed).toFixed(1)}s
    <input
      type="range"
      min="0.1"
      max="1"
      step="0.1"
      value={msToS($appState.clickerSpeed)}
      oninput={onSpeedInput}
      disabled={$appState.clickerRunning}
      class="accent-blue-600"
    />
  </label>

  <label class="flex flex-col gap-1 text-sm text-neutral-600 dark:text-neutral-300">
    Starting in: {startDelay.toFixed(1)}s
    <input
      type="range"
      min="0"
      max="10"
      step="0.1"
      bind:value={startDelay}
      disabled={$appState.clickerRunning || countdown > 0}
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
    class:bg-amber-500={countdown > 0}
    class:bg-red-600={countdown === 0 && $appState.clickerRunning}
    class:bg-blue-600={countdown === 0 && !$appState.clickerRunning}
    class="mt-auto rounded-lg py-2 font-semibold text-white hover:opacity-90 transition"
  >
    {#if countdown > 0}
      Starting in {countdown.toFixed(1)}s… (click to cancel)
    {:else}
      {$appState.clickerRunning ? "Stop Clicker" : "Start Clicker"}
    {/if}
  </button>

  {#if error}
    <p class="text-sm text-red-600">{error}</p>
  {/if}
</section>
