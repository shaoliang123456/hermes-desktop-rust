# Hermes Desktop Rust

> A high-performance desktop companion for [Hermes Agent](https://github.com/NousResearch/hermes-agent), built with **Electron + Rust native addon**.

## Why Rust?

The original [hermes-desktop](https://github.com/fathah/hermes-desktop) is a pure Electron/TypeScript application. This project replaces the performance-critical backend with **Rust native addons** (via [napi-rs](https://napi.rs/)):

- **SQLite operations**: Synchronous `better-sqlite3` calls that could block the Electron main process are now handled by `rusqlite` in Rust.
- **SSE parsing**: High-throughput character stream parsing at native speed.
- **SSH tunneling**: Connection stability powered by the `ssh2` crate.
- **Modular architecture**: Each domain is a separate Rust module instead of a 1,800-line God File.

## Architecture

```
┌─────────────────────────────────────────────────────┐
│  Electron Renderer (React)                          │
│  ─ 100% reused from original project, zero changes  │
└───────────────────┬─────────────────────────────────┘
                    │ ipcMain / ipcRenderer (unchanged)
┌───────────────────┴─────────────────────────────────┐
│  Electron Main Process (thin shell ~70 lines)       │
│  ┌─────────────────────────────────────────────┐    │
│  │  Rust Native Addon (.node)                   │    │
│  │  sessions · kanban · config · hermes         │    │
│  │  profiles · models · ssh_tunnel · sse        │    │
│  │  rusqlite · ssh2 · serde · tokio             │    │
│  └─────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────┘
```

## Getting Started

```bash
git clone https://github.com/shaoliang123456/hermes-desktop-rust.git
cd hermes-desktop-rust
npm install
cd native && npm install && npm run build && cd ..
npm run dev
```

## Acknowledgments

This project was **inspired by and incorporates front-end code from** [fathah/hermes-desktop](https://github.com/fathah/hermes-desktop), an Electron-based Hermes desktop client by Fathah, released under the [MIT License](https://opensource.org/licenses/MIT).

The Rust native addon back-end is a complete rewrite with a modular architecture.

## License

[MIT](LICENSE)
