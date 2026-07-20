# Autoclicker (Tauri + Rust + Svelte)

Desktop autoclicker + key binder. Rust backend (Tauri 2, `enigo` for input simulation), Svelte 5 / SvelteKit + Tailwind frontend.

## Linux prerequisites

```bash
sudo apt-get install pkg-config libwebkit2gtk-4.1-dev build-essential \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev
```

## Dev

```bash
npm install
npx tauri dev
```

## Build

```bash
npx tauri build
```

Binary lands in `src-tauri/target/release/`.

## Config persistence

Settings (clicker speed/button, key binder keys/interval) autosave to `~/.autoclicker/config.json` on every change and reload on app start.
