import { writable } from "svelte/store";

export interface KeyBind {
  key: string;
  intervalMs: number;
}

export interface AppState {
  clickerRunning: boolean;
  clickerSpeed: number;
  clickerButton: "left" | "right";
  keyBinderRunning: boolean;
  keyBinds: KeyBind[];
  keyIntervalMin: number;
  keyIntervalMax: number;
}

export const appState = writable<AppState>({
  clickerRunning: false,
  clickerSpeed: 100,
  clickerButton: "left",
  keyBinderRunning: false,
  keyBinds: [{ key: "b", intervalMs: 1000 }],
  keyIntervalMin: 100,
  keyIntervalMax: 5000,
});
