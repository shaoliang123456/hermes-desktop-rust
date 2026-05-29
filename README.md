<h1 align="center">Hermes Desktop · Rust Edition</h1>

<p align="center">
  <strong>Desktop companion for <a href="https://github.com/NousResearch/hermes-agent">Hermes Agent</a></strong><br>
  Built with <strong>Electron + Rust Native Addon</strong>
</p>

<p align="center">
  <a href="https://github.com/shaoliang123456/hermes-desktop-rust/releases">
    <img src="https://img.shields.io/badge/Download-Releases-FF6600?style=for-the-badge" alt="Releases">
  </a>
  <a href="https://github.com/shaoliang123456/hermes-desktop-rust/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="License: MIT">
  </a>
  <a href="https://github.com/fathah/hermes-desktop">
    <img src="https://img.shields.io/badge/Based%20on-fathah%2Fhermes--desktop-blue?style=for-the-badge" alt="Original Project">
  </a>
</p>

<p align="center">
  🌐 <a href="#english">English</a> &nbsp;|&nbsp; 🇨🇳 <a href="#中文">中文</a>
</p>

---

<a id="english"></a>

## What Is This?

A Rust-native rewrite of the backend for [fathah/hermes-desktop](https://github.com/fathah/hermes-desktop), a desktop app for [Hermes Agent](https://github.com/NousResearch/hermes-agent).

- **Front-end** (React, Preload, i18n, assets) — reused from the original project
- **Back-end** — rewritten in Rust via [napi-rs](https://napi.rs/), covering SSE parsing, SQLite, SSH tunneling, session management, and config I/O

## Architecture

```
┌─────────────────────────────────────────────────┐
│  Electron Renderer (React)                      │
│  Reused from original project                   │
└───────────────────┬─────────────────────────────┘
                    │ IPC
┌───────────────────┴─────────────────────────────┐
│  Electron Main Process                           │
│  ┌─────────────────────────────────────────┐    │
│  │  Rust Native Addon (.node)               │    │
│  │  sessions · kanban · config · hermes     │    │
│  │  profiles · models · ssh_tunnel · sse    │    │
│  └─────────────────────────────────────────┘    │
└─────────────────────────────────────────────────┘
```

## Download

Go to the [**Releases page**](https://github.com/shaoliang123456/hermes-desktop-rust/releases).

| Platform | File |
|----------|------|
| **macOS** (Apple Silicon) | `.dmg` or `-arm64-mac.zip` |
| **macOS** (Intel) | `-x64-mac.zip` |
| **Windows** | `-setup.exe` |
| **Linux** | `.AppImage` |

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) 20+ and npm
- [Rust](https://rustup.rs/) toolchain (stable)

### Build & Run

```bash
git clone https://github.com/shaoliang123456/hermes-desktop-rust.git
cd hermes-desktop-rust
npm install
cd native && npm install && npm run build && cd ..
npm run dev
```

### Build Packages

```bash
npm run build:mac      # macOS .dmg
npm run build:win      # Windows .exe
npm run build:linux    # Linux AppImage / .deb
```

## Features

All features from the original [hermes-desktop](https://github.com/fathah/hermes-desktop) are preserved:

- **Guided install** for Hermes Agent
- **Streaming chat UI** with SSE, tool progress, markdown, syntax highlighting
- **Multi-provider support** — OpenRouter, Anthropic, OpenAI, Google, xAI, local endpoints, and more
- **Session management** — full-text search (FTS5), date-grouped history
- **Profile switching** — isolated Hermes environments
- **14 toolsets** — web, browser, terminal, file, code, vision, image gen, TTS, and more
- **Memory system** — view/edit entries, discoverable providers
- **16 messaging gateways** — Telegram, Discord, Slack, WhatsApp, Signal, Matrix, and more
- **Scheduled tasks** — cron job builder with delivery targets
- **i18n** — EN, ZH-CN, ZH-TW, JA, KO, ES, PT-BR, PT-PT, ID
- **Auto-updater** via electron-updater

## Tech Stack

| Layer | Technology |
|-------|-----------|
| UI | React 19 + Tailwind CSS 4 |
| Desktop Shell | Electron 39 |
| Native Backend | Rust (napi-rs 2) |
| Database | rusqlite (SQLite 3) |
| Build | electron-vite + Vite 7 |

## Acknowledgments

Special thanks to **[Fathah](https://github.com/fathah)** for creating the original **[hermes-desktop](https://github.com/fathah/hermes-desktop)** project. Front-end code is reused from the original with gratitude. The original project is released under the [MIT License](https://opensource.org/licenses/MIT).

## License

[MIT](LICENSE)

---

<a id="中文"></a>

## 这是什么？

基于 [fathah/hermes-desktop](https://github.com/fathah/hermes-desktop) 后端用 Rust 重写的桌面客户端，用于 [Hermes Agent](https://github.com/NousResearch/hermes-agent)。

- **前端**（React、Preload、i18n、资源文件）— 复用原项目
- **后端** — 通过 [napi-rs](https://napi.rs/) 用 Rust 重写，覆盖 SSE 解析、SQLite、SSH 隧道、会话管理、配置读写

## 架构

```
┌─────────────────────────────────────────────────┐
│  Electron 渲染器 (React)                        │
│  复用自原项目                                    │
└───────────────────┬─────────────────────────────┘
                    │ IPC
┌───────────────────┴─────────────────────────────┐
│  Electron 主进程                                  │
│  ┌─────────────────────────────────────────┐    │
│  │  Rust 原生插件 (.node)                    │    │
│  │  sessions · kanban · config · hermes     │    │
│  │  profiles · models · ssh_tunnel · sse    │    │
│  └─────────────────────────────────────────┘    │
└─────────────────────────────────────────────────┘
```

## 下载

前往 [**Releases 页面**](https://github.com/shaoliang123456/hermes-desktop-rust/releases)。

| 平台 | 文件 |
|------|------|
| **macOS**（Apple 芯片） | `.dmg` 或 `-arm64-mac.zip` |
| **macOS**（Intel） | `-x64-mac.zip` |
| **Windows** | `-setup.exe` |
| **Linux** | `.AppImage` |

## 开发

### 前置条件

- [Node.js](https://nodejs.org/) 20+ 和 npm
- [Rust](https://rustup.rs/) 工具链（stable）

### 构建与运行

```bash
git clone https://github.com/shaoliang123456/hermes-desktop-rust.git
cd hermes-desktop-rust
npm install
cd native && npm install && npm run build && cd ..
npm run dev
```

### 打包发布

```bash
npm run build:mac      # macOS .dmg
npm run build:win      # Windows .exe
npm run build:linux    # Linux AppImage / .deb
```

## 功能特性

原版 [hermes-desktop](https://github.com/fathah/hermes-desktop) 的所有功能完整保留：

- **引导式安装** — 自动安装 Hermes Agent 及依赖
- **流式聊天** — SSE 实时流、工具进度、Markdown 渲染、语法高亮
- **多模型支持** — OpenRouter、Anthropic、OpenAI、Google、xAI、本地端点等
- **会话管理** — 全文搜索 (FTS5)、按日期分组的历史记录
- **多配置切换** — 独立的 Hermes 环境隔离
- **14 个工具集** — 网页、浏览器、终端、文件、代码、视觉、图像生成、TTS 等
- **记忆系统** — 查看/编辑条目、可发现提供者
- **16 个消息网关** — Telegram、Discord、Slack、WhatsApp、Signal、Matrix 等
- **定时任务** — Cron 调度器，支持多种投递目标
- **国际化** — EN、ZH-CN、ZH-TW、JA、KO、ES、PT-BR、PT-PT、ID
- **自动更新** — 通过 electron-updater

## 技术栈

| 层级 | 技术 |
|------|------|
| UI | React 19 + Tailwind CSS 4 |
| 桌面壳 | Electron 39 |
| 原生后端 | Rust (napi-rs 2) |
| 数据库 | rusqlite (SQLite 3) |
| 构建 | electron-vite + Vite 7 |

## 致谢

特别感谢 **[Fathah](https://github.com/fathah)** 创建了原版 **[hermes-desktop](https://github.com/fathah/hermes-desktop)** 项目。前端代码复用自原项目，在此深表感谢。原项目基于 [MIT License](https://opensource.org/licenses/MIT) 发布。

## 许可证

[MIT](LICENSE)
