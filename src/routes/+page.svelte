<script lang="ts">
  import { onMount } from "svelte";
  import { appState } from "$lib/stores";
  import { loadConfig, getStatus, onStatusChanged } from "$lib/api";
  import ClickerControl from "$lib/components/ClickerControl.svelte";
  import KeyBinderControl from "$lib/components/KeyBinderControl.svelte";

  onMount(() => {
    let unlisten: (() => void) | undefined;

    (async () => {
      try {
        const config = await loadConfig();
        appState.update((s) => ({
          ...s,
          clickerSpeed: config.clicker_speed,
          clickerButton: config.clicker_button === "right" ? "right" : "left",
          keys: config.keys,
          keyInterval: config.key_interval,
        }));

        const status = await getStatus();
        appState.update((s) => ({
          ...s,
          clickerRunning: status.clicker_running,
          keyBinderRunning: status.keys_running,
        }));

        unlisten = await onStatusChanged((status) => {
          appState.update((s) => ({
            ...s,
            clickerRunning: status.clicker_running,
            keyBinderRunning: status.keys_running,
          }));
        });
      } catch (e) {
        console.error(e);
      }
    })();

    return () => unlisten?.();
  });
</script>

<main class="min-h-screen bg-neutral-100 dark:bg-neutral-900 flex items-center justify-center p-8">
  <div class="w-full max-w-3xl grid gap-6 sm:grid-cols-2">
    <h1 class="sm:col-span-2 text-2xl font-bold text-center text-neutral-900 dark:text-neutral-100">
      Autoclicker
    </h1>
    <ClickerControl />
    <KeyBinderControl />
  </div>
</main>
