# Prompt Completo Tauri One-Shot

---

## Criar Autoclicker Desktop - Tauri + Rust + Svelte

Desenvolver aplicação desktop completa e funcional. Executável ao sair.

### Tech Stack
- **Backend:** Rust + Tauri 1.x
- **Frontend:** Svelte + SvelteKit + Tailwind
- **Target:** Ubuntu, Windows, macOS

---

## Arquitetura

### Estrutura Projeto

```
autoclicker-tauri/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs              (Tauri setup + commands)
│   │   ├── clicker.rs           (lógica autoclicker)
│   │   ├── key_binder.rs        (lógica key binding)
│   │   └── state.rs             (estado compartilhado)
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/
│   ├── App.svelte               (root component)
│   ├── components/
│   │   ├── ClickerControl.svelte
│   │   ├── KeyBinderControl.svelte
│   │   └── Status.svelte
│   ├── stores.ts
│   └── main.ts
├── package.json
└── README.md
```

### Fluxo Comunicação

```
Frontend (Svelte)
    ↓ invoke command
Tauri IPC
    ↓
Backend (Rust)
    ↓ execute
Threads + enigo
    ↓ events
Tauri Emit
    ↓
Frontend (Svelte)
```

---

## Setup & Pré-requisitos

### Linux/Ubuntu
```bash
sudo apt-get install libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

### macOS
```bash
brew install webkit2gtk
```

### Windows
- Visual Studio Build Tools ou MSVC
- WebView2 (geralmente já instalado)

### Instalar Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Criar Projeto
```bash
npm create tauri-app@latest

# Respostas:
# Project name: autoclicker-tauri
# Package manager: npm
# UI template: Svelte
# TypeScript: Yes
```

### Dependências Cargo (Cargo.toml)
```toml
[dependencies]
tauri = { version = "1.x", features = ["shell-open"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
enigo = "0.1"
```

### Dependências NPM
```bash
npm install
npm install -D tailwindcss postcss autoprefixer
npm install -D @tauri-apps/api
```

### Development
```bash
# Terminal 1
npm run dev

# Terminal 2
cargo tauri dev
```

### Build
```bash
npm run build
cargo tauri build
```

---

## Requisitos Funcionais

### 1. Autoclicker
- Start/stop clicker
- Slider velocidade (10-1000ms)
- Radio button: botão esquerdo/direito
- Indicador status (running/stopped)

### 2. Key Binder
- Input texto: teclas a bindar (ex: "bvcxz")
- Slider intervalo (100-5000ms)
- Start/stop key binding
- Status indicator

### 3. Estado Global
- Usar Svelte store para compartilhar estado
- Salvar configs em JSON (`~/.autoclicker/config.json`)
- Carregar configs ao iniciar app

### 4. UI/UX
- Layout: 2 seções (Clicker | Key Binder)
- Buttons: start/stop bem visíveis
- Sliders: feedback visual tempo real
- Responsive design (desktop first)
- Tema claro/escuro (system default)

---

## Backend Commands (Rust)

```rust
#[tauri::command]
start_clicker(speed_ms: u64, button: String) → Result<(), String>

#[tauri::command]
stop_clicker() → Result<(), String>

#[tauri::command]
start_key_binder(keys: String, interval_ms: u64) → Result<(), String>

#[tauri::command]
stop_key_binder() → Result<(), String>

#[tauri::command]
get_status() → Result<AppStatus, String>
// AppStatus = { clicker_running: bool, keys_running: bool }
```

---

## Detalhes Implementação

### Autoclicker (Rust)
- Thread separada com tokio::spawn
- Usar enigo::Mouse para clicks
- Stop via atomic flag (Arc<AtomicBool>)
- Sleep configurável
- Botão esquerdo/direito via parâmetro

### Key Binder (Rust)
- Thread separada
- Usar enigo::Key para cada tecla
- Press + sleep pequeno + release
- Intervalo entre sequência
- Suportar múltiplas teclas (string parsing)

### Svelte Store (stores.ts)
```typescript
export let appState = writable({
  clickerRunning: false,
  clickerSpeed: 100,
  clickerButton: 'left',
  keyBinderRunning: false,
  keys: 'bvcxz',
  keyInterval: 1000
})
```

### Config JSON (~/.autoclicker/config.json)
```json
{
  "clicker_speed": 100,
  "clicker_button": "left",
  "keys": "bvcxz",
  "key_interval": 1000
}
```

### Exemplo main.rs
```rust
#[tauri::command]
fn start_clicker(speed_ms: u64, button: String) -> Result<String, String> {
    // implementar lógica com enigo e tokio
    Ok(format!("Clicker started at {} ms", speed_ms))
}

#[tauri::command]
fn stop_clicker() -> Result<String, String> {
    // implementar parada
    Ok("Clicker stopped".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_clicker, stop_clicker])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Exemplo Svelte Component
```svelte
<script>
  import { invoke } from '@tauri-apps/api/tauri';
  
  let speed = 100;
  let button = 'left';
  
  async function startClicker() {
    try {
      const result = await invoke('start_clicker', {
        speedMs: speed,
        button: button
      });
      console.log(result);
    } catch (error) {
      console.error(error);
    }
  }
</script>

<button on:click={startClicker}>
  Start Clicker
</button>
```

---

## Comportamento Expected

1. App inicia → carrega config salva → UI reflete estado
2. User muda slider velocidade → update visual time real
3. User clica Start Clicker → invoke Rust command → threads iniciam → Status muda "Running"
4. User clica Stop → threads param → Status muda "Stopped"
5. Fechar app → salva config atual em JSON
6. Reabrir → carrega config anterior

---

## Roadmap (Fases)

### Fase 1: Setup Base
- [ ] Criar projeto Tauri com Svelte
- [ ] Configurar Rust backend com enigo
- [ ] Setup comunicação Tauri IPC
- [ ] Estrutura pastas completa

### Fase 2: Core Autoclicker
- [ ] Comando `start_clicker` em Rust
- [ ] Comando `stop_clicker`
- [ ] Thread autoclicker com enigo
- [ ] Parâmetros: velocidade, botão (esquerdo/direito)
- [ ] Status event emitter (running/stopped)

### Fase 3: Key Binder
- [ ] Comando `start_key_binder`
- [ ] Comando `stop_key_binder`
- [ ] Thread key binder
- [ ] Input dinâmico de teclas
- [ ] Intervalo configurável

### Fase 4: Frontend UI
- [ ] Component ClickerControl (slider velocidade, radio botão, start/stop)
- [ ] Component KeyBinderControl (input teclas, slider intervalo, start/stop)
- [ ] Component Status (indicador visual)
- [ ] Styling Tailwind (tema claro/escuro)

### Fase 5: Estado & Persistência
- [ ] Svelte store (estado global)
- [ ] Salvar configs em arquivo JSON
- [ ] Carregar configs ao iniciar
- [ ] Settings page (salvar preferências)

### Fase 6: Build & Distribuição
- [ ] Configurar tauri.conf.json para cada OS
- [ ] Build Ubuntu (AppImage ou .deb)
- [ ] Build Windows (.msi)
- [ ] Build macOS (.dmg)
- [ ] Testes em cada plataforma

### Fase 7: Polish
- [ ] Ícone aplicação
- [ ] Logs/debug console
- [ ] Tratamento erros
- [ ] UX refinement
- [ ] Documentação usuário

---

## Testing
- Testar cada comando Rust (start/stop clicker, start/stop keybinder)
- Testar comunicação IPC (frontend → backend)
- Testar config save/load
- Testar UI responsiveness em 3 plataformas

---

## Entregar
- Código completo pronto build
- Instrução: `npm install && cargo tauri dev` deve abrir app funcional
- Sem erros compilação/runtime
- UI responsiva e funcional
- Todas fases 1-5 completas (fase 6-7 opcionais)

---

## Notas Gerais
- Usar padrões Rust idiomáticos
- Svelte components granulares + reutilizáveis
- Sem comments desnecessários, código auto-explicativo
- Error handling robusto (não panic em user input)
- Cross-platform ready (usar paths corretos por OS)
- Code pronto para publicar/distribuir
