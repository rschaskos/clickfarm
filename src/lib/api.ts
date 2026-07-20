import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export interface KeyBind {
  key: string;
  interval_ms: number;
}

export interface AppConfig {
  clicker_speed: number;
  clicker_button: string;
  key_binds: KeyBind[];
  key_interval_min_ms: number;
  key_interval_max_ms: number;
}

export interface AppStatus {
  clicker_running: boolean;
  keys_running: boolean;
}

export const startClicker = (speedMs: number, button: string) =>
  invoke<void>("start_clicker", { speedMs, button });

export const stopClicker = () => invoke<void>("stop_clicker");

export const startKeyBinder = (keyBinds: KeyBind[]) =>
  invoke<void>("start_key_binder", { keyBinds });

export const stopKeyBinder = () => invoke<void>("stop_key_binder");

export const getStatus = () => invoke<AppStatus>("get_status");

export const loadConfig = () => invoke<AppConfig>("load_config");

export const saveConfig = (config: AppConfig) =>
  invoke<void>("save_config", { config });

export const onStatusChanged = (
  cb: (status: AppStatus) => void,
): Promise<UnlistenFn> => listen<AppStatus>("status-changed", (e) => cb(e.payload));
