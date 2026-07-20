<script lang="ts">
  import { appState } from "$lib/stores";
  import { startKeyBinder, stopKeyBinder, saveConfig } from "$lib/api";
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

  function addBind() {
    $appState.keyBinds = [
      ...$appState.keyBinds,
      { key: "", intervalMs: $appState.keyIntervalMin },
    ];
    persist();
  }

  function removeBind(index: number) {
    $appState.keyBinds = $appState.keyBinds.filter((_, i) => i !== index);
    persist();
  }

  function updateKey(index: number, value: string) {
    const char = value.slice(-1);
    $appState.keyBinds = $appState.keyBinds.map((b, i) =>
      i === index ? { ...b, key: char } : b,
    );
    persist();
  }

  function updateInterval(index: number, seconds: number) {
    $appState.keyBinds = $appState.keyBinds.map((b, i) =>
      i === index ? { ...b, intervalMs: sToMs(seconds) } : b,
    );
    persist();
  }

  function updateMinBound(seconds: number) {
    $appState.keyIntervalMin = sToMs(seconds);
    persist();
  }

  function updateMaxBound(seconds: number) {
    $appState.keyIntervalMax = sToMs(seconds);
    persist();
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
      await startKeyBinder(
        $appState.keyBinds.map((b) => ({ key: b.key, interval_ms: b.intervalMs })),
      );
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

  <div class="flex gap-4">
    <label class="flex flex-col gap-1 text-sm text-neutral-600 dark:text-neutral-300">
      Min interval (s)
      <input
        type="number"
        step="0.1"
        min="0.1"
        value={msToS($appState.keyIntervalMin)}
        onchange={(e) => updateMinBound(+(e.currentTarget as HTMLInputElement).value)}
        disabled={$appState.keyBinderRunning}
        class="w-24 rounded-md border border-neutral-300 dark:border-neutral-600 bg-transparent px-2 py-1"
      />
    </label>
    <label class="flex flex-col gap-1 text-sm text-neutral-600 dark:text-neutral-300">
      Max interval (s)
      <input
        type="number"
        step="0.1"
        min="0.1"
        value={msToS($appState.keyIntervalMax)}
        onchange={(e) => updateMaxBound(+(e.currentTarget as HTMLInputElement).value)}
        disabled={$appState.keyBinderRunning}
        class="w-24 rounded-md border border-neutral-300 dark:border-neutral-600 bg-transparent px-2 py-1"
      />
    </label>
  </div>

  <div class="flex flex-col gap-2">
    {#each $appState.keyBinds as bind, i (i)}
      <div class="flex items-center gap-2">
        <input
          type="text"
          maxlength="1"
          value={bind.key}
          oninput={(e) => updateKey(i, (e.currentTarget as HTMLInputElement).value)}
          disabled={$appState.keyBinderRunning}
          placeholder="key"
          class="w-14 rounded-md border border-neutral-300 dark:border-neutral-600 bg-transparent px-2 py-1 text-center"
        />
        <input
          type="range"
          min={msToS($appState.keyIntervalMin)}
          max={msToS($appState.keyIntervalMax)}
          step="0.1"
          value={msToS(bind.intervalMs)}
          oninput={(e) => updateInterval(i, +(e.currentTarget as HTMLInputElement).value)}
          disabled={$appState.keyBinderRunning}
          class="flex-1 accent-blue-600"
        />
        <input
          type="number"
          step="0.1"
          min={msToS($appState.keyIntervalMin)}
          max={msToS($appState.keyIntervalMax)}
          value={msToS(bind.intervalMs)}
          oninput={(e) => updateInterval(i, +(e.currentTarget as HTMLInputElement).value)}
          disabled={$appState.keyBinderRunning}
          class="w-16 rounded-md border border-neutral-300 dark:border-neutral-600 bg-transparent px-2 py-1 text-right text-sm"
        />
        <button
          onclick={() => removeBind(i)}
          disabled={$appState.keyBinderRunning}
          class="rounded-md px-2 py-1 text-red-600 hover:bg-red-50 dark:hover:bg-red-900/30"
          aria-label="Remove key bind"
        >
          ×
        </button>
      </div>
    {/each}

    <button
      onclick={addBind}
      disabled={$appState.keyBinderRunning}
      class="self-start rounded-md border border-dashed border-neutral-300 dark:border-neutral-600 px-3 py-1 text-sm text-neutral-600 dark:text-neutral-300 hover:bg-neutral-50 dark:hover:bg-neutral-700"
    >
      + Add key
    </button>
  </div>

  <label class="flex flex-col gap-1 text-sm text-neutral-600 dark:text-neutral-300">
    Starting in: {startDelay.toFixed(1)}s
    <input
      type="range"
      min="0"
      max="10"
      step="0.1"
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
      Starting in {countdown.toFixed(1)}s… (click to cancel)
    {:else}
      {$appState.keyBinderRunning ? "Stop Key Binder" : "Start Key Binder"}
    {/if}
  </button>

  {#if error}
    <p class="text-sm text-red-600">{error}</p>
  {/if}
</section>
