import { writable } from "svelte/store";

export interface AppState {
  clickerRunning: boolean;
  clickerSpeed: number;
  clickerButton: "left" | "right";
  keyBinderRunning: boolean;
  keys: string;
  keyInterval: number;
}

export const appState = writable<AppState>({
  clickerRunning: false,
  clickerSpeed: 100,
  clickerButton: "left",
  keyBinderRunning: false,
  keys: "bvcxz",
  keyInterval: 1000,
});
