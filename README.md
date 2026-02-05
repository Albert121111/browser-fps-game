# Mintcraft Launcher

## Roadmap (milestones)

1. **MVP instances + install**: instance CRUD, version/loader install, basic downloads.
2. **Auth + launch**: Microsoft device flow, offline profile fallback, launch args generation.
3. **Content**: Modrinth search/install/update, .mrpack import, CurseForge adapter stub.
4. **Downloads**: queue, resume, concurrency, error retries.
5. **Stabilization**: tests, logging, updater, i18n, release pipelines.

## Структура проекта

```
.
├── app.js
├── index.html
├── styles.css
├── package.json
├── vite.config.js
├── .eslintrc.cjs
├── src-tauri
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   └── src
│       ├── main.rs
│       ├── state.rs
│       └── commands
│           ├── auth.rs
│           ├── downloads.rs
│           ├── installers.rs
│           ├── instances.rs
│           ├── java.rs
│           ├── launch.rs
│           ├── manifests.rs
│           ├── modrinth.rs
│           ├── settings.rs
│           ├── utils.rs
│           └── mod.rs
```

## Prerequisites

- **Rust** (stable) + Cargo
- **Node.js** (18+)
- **Tauri CLI** (`cargo install tauri-cli`)

## Dev run

```bash
npm install
npm run dev
# in another terminal
npm run tauri dev
```

## Build release

```bash
npm install
npm run build
npm run tauri build
```

## Data dirs

- **Launcher data**: `~/.config/mintcraft` (Linux), `~/Library/Application Support/mintcraft` (macOS), `%APPDATA%\mintcraft` (Windows)
- **Instances**: `~/Games/Mintcraft/Instances` (default, configurable)

## Авторизация Microsoft

- Используется device code flow (или PKCE при веб-логине в будущем).
- Токены сохраняются через системное secure storage (keyring).
- Если авторизация недоступна, используется offline профиль (ник без входа).

## CurseForge адаптер

Официальный API требует ключ. В проекте оставлен адаптер-заглушка и UI-уведомление. После добавления ключа
можно подключить реальные запросы.

## Обновления

Tauri updater включен в `tauri.conf.json`. Добавьте endpoint в список `endpoints`, когда будет настроен
сервер обновлений.

## Проверки качества

```bash
npm run lint
cargo fmt
cargo clippy
```

## Возможности прототипа UI

- Переключение страниц: Library, Instance, Browse, Downloads, Settings.
- Инстансы с настройками версии/лоадера/Java и управлением папками.
- Очередь загрузок и простая визуализация прогресса.
- Установщик сборки как отдельная панель.
