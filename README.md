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
  <a href="#english">English</a> &nbsp;|&nbsp; <a href="#中文">中文</a>
</p>

---

<a id="english"></a>

## What Is This?

A Rust-native rewrite of the backend for [fathah/hermes-desktop](https://github.com/fathah/hermes-desktop), a desktop app for [Hermes Agent](https://github.com/NousResearch/hermes-agent).

- **Front-end** (React, Preload, i18n, assets) — reused from the original project
- **Back-end** — rewritten in Rust via [napi-rs](https://napi.rs/), covering SSE parsing, SQLite, SSH tunneling, session management, and config I/O

## Why Rust?

The original Electron/TypeScript app processes SSE streams, SQLite queries, and SSH tunnels on the Node.js main thread. This project moves those hot paths into a Rust native addon:

| Component | Original (TypeScript) | Rust Edition |
|-----------|----------------------|--------------|
| SSE Parsing | JS string split + regex | Native zero-copy parser |
| SQLite | `better-sqlite3` (sync, blocks event loop) | `rusqlite` (off-thread) |
| SSH Tunnel | Node.js `ssh2` | System SSH with native health check |
| Config I/O | `fs.readFileSync` | Rust `std::fs` |
| Code Structure | Single 1800-line file | 8 focused modules |

### Performance

> Based on local testing on Apple M-series. Results may vary by hardware and provider.

| Metric | Original (TypeScript) | Rust Edition |
|--------|----------------------|--------------|
| SSE streaming | Noticeable latency between chunks | Smooth, near-instant rendering |
| Session cache sync | Blocks UI during sync | Non-blocking, off-thread |
| Memory (idle) | ~180 MB | ~145 MB (~20% less) |
| Startup time | Standard | Slightly faster (native module init) |

The "typing speed" in chat is visibly faster — chunks arrive at the same rate from the API, but the local processing pipeline no longer introduces GC pauses or event-loop contention.

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

## Supported Providers

### LLM Providers

| Provider | Endpoint | Notes |
|----------|----------|-------|
| **OpenRouter** | openrouter.ai | 200+ models via single API (recommended) |
| **Anthropic** | api.anthropic.com | Direct Claude access |
| **OpenAI** | api.openai.com | Direct GPT access |
| **DeepSeek** | api.deepseek.com | DeepSeek models |
| **Google (Gemini)** | via OpenRouter | Google AI Studio |
| **Groq** | api.groq.com | Fast inference |
| **Mistral** | api.mistral.ai | Mistral models |
| **Together AI** | api.together.xyz | Open model hosting |
| **Fireworks AI** | api.fireworks.ai | Fast inference |
| **Cerebras** | api.cerebras.ai | Ultra-fast inference |
| **Perplexity** | api.perplexity.ai | Search-augmented models |
| **Hugging Face** | router.huggingface.co | 20+ open models |
| **xAI (Grok)** | via OpenRouter | Grok models |
| **Nous Portal** | Free tier available | |
| **Local/Custom** | Any OpenAI-compatible endpoint | LM Studio, Ollama, vLLM, llama.cpp |

### Messaging Gateways

Telegram, Discord, Slack, WhatsApp, Signal, Matrix, Mattermost, Email (IMAP/SMTP), SMS (Twilio/Vonage), iMessage (BlueBubbles), DingTalk, Feishu/Lark, WeCom, WeChat, Webhooks, Home Assistant.

### Tool Integrations

Web search, browser automation (Camoufox anti-detection), terminal execution, file operations, code execution, vision analysis, image generation (FAL.ai), video generation, TTS, memory providers, skills management, and more.

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

### Release

```bash
./release.sh patch    # 0.5.2 → 0.5.3
./release.sh minor    # 0.5.2 → 0.6.0
./release.sh major    # 0.5.2 → 1.0.0
```

This automatically bumps `package.json` version, creates a git tag, and pushes to trigger multi-platform CI builds.

## Features

All features from the original [hermes-desktop](https://github.com/fathah/hermes-desktop) are preserved:

- **Guided install** for Hermes Agent with dependency resolution
- **Streaming chat UI** with SSE, tool progress, markdown, syntax highlighting
- **Session management** — full-text search (FTS5), date-grouped history
- **Profile switching** — isolated Hermes environments
- **14 toolsets** — web, browser, terminal, file, code, vision, image gen, TTS, and more
- **Memory system** — view/edit entries, discoverable providers
- **16 messaging gateways**
- **Scheduled tasks** — cron job builder with delivery targets
- **Kanban board** — task management for multi-agent workflows
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

## Contributing

Contributions are welcome! Here's how you can help:

- **Bug reports** — [Open an issue](https://github.com/shaoliang123456/hermes-desktop-rust/issues) with steps to reproduce
- **Feature requests** — Describe the use case and expected behavior
- **Pull requests** — Fork, create a branch, submit a PR. Run `npm run lint` and `npm run typecheck` first
- **Translations** — Add or improve i18n strings in `src/shared/i18n/locales/`
- **Rust backend** — Improve or add native modules in `native/src/`

Areas where help is especially appreciated:

- Writing automated benchmarks for SSE/streaming performance
- Adding more i18n translations (Kanban screen and others)
- Improving SSH tunnel reliability and error recovery
- Cross-platform testing (Windows, Linux)

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

## 为什么用 Rust？

原版 Electron/TypeScript 应用在 Node.js 主线程上处理 SSE 流、SQLite 查询和 SSH 隧道。本项目将这些热路径移入 Rust 原生插件：

| 组件 | 原版 (TypeScript) | Rust 版 |
|------|------------------|---------|
| SSE 解析 | JS 字符串 split + 正则 | 原生零拷贝解析器 |
| SQLite | `better-sqlite3`（同步，阻塞事件循环） | `rusqlite`（独立线程） |
| SSH 隧道 | Node.js `ssh2` | 系统 SSH + 原生健康检查 |
| 配置读写 | `fs.readFileSync` | Rust `std::fs` |
| 代码架构 | 单一 1800 行文件 | 8 个职责清晰的模块 |

### 性能对比

> 基于 Apple M 系列芯片本地测试。实际结果可能因硬件和供应商而异。

| 指标 | 原版 (TypeScript) | Rust 版 |
|------|------------------|---------|
| SSE 流式输出 | chunk 之间有明显延迟 | 流畅，接近即时渲染 |
| 会话缓存同步 | 同步期间阻塞 UI | 非阻塞，独立线程 |
| 内存占用（空闲） | ~180 MB | ~145 MB（少约 20%） |
| 启动时间 | 标准 | 略快（原生模块初始化） |

聊天框里的"打字速度"明显更快——API 返回数据的速度不变，但本地处理管道不再引入 GC 停顿或事件循环争用。

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

## 支持的供应商

### LLM 供应商

| 供应商 | 端点 | 备注 |
|--------|------|------|
| **OpenRouter** | openrouter.ai | 单 API 访问 200+ 模型（推荐） |
| **Anthropic** | api.anthropic.com | 直连 Claude |
| **OpenAI** | api.openai.com | 直连 GPT |
| **DeepSeek** | api.deepseek.com | DeepSeek 模型 |
| **Google (Gemini)** | 通过 OpenRouter | Google AI Studio |
| **Groq** | api.groq.com | 快速推理 |
| **Mistral** | api.mistral.ai | Mistral 模型 |
| **Together AI** | api.together.xyz | 开放模型托管 |
| **Fireworks AI** | api.fireworks.ai | 快速推理 |
| **Cerebras** | api.cerebras.ai | 超快推理 |
| **Perplexity** | api.perplexity.ai | 搜索增强模型 |
| **Hugging Face** | router.huggingface.co | 20+ 开放模型 |
| **xAI (Grok)** | 通过 OpenRouter | Grok 模型 |
| **Nous Portal** | 免费套餐可用 | |
| **本地/自定义** | 任何 OpenAI 兼容端点 | LM Studio、Ollama、vLLM、llama.cpp |

### 消息网关

Telegram、Discord、Slack、WhatsApp、Signal、Matrix、Mattermost、邮件（IMAP/SMTP）、短信（Twilio/Vonage）、iMessage（BlueBubbles）、钉钉、飞书、企业微信、微信、Webhooks、Home Assistant。

### 工具集成

网页搜索、浏览器自动化（Camoufox 反检测）、终端执行、文件操作、代码执行、视觉分析、图像生成（FAL.ai）、视频生成、TTS、记忆提供者、技能管理等。

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

### 发版

```bash
./release.sh patch    # 0.5.2 → 0.5.3
./release.sh minor    # 0.5.2 → 0.6.0
./release.sh major    # 0.5.2 → 1.0.0
```

自动更新 `package.json` 版本号、创建 git tag、推送触发多平台 CI 构建。

## 功能特性

原版 [hermes-desktop](https://github.com/fathah/hermes-desktop) 的所有功能完整保留：

- **引导式安装** — 自动安装 Hermes Agent 及依赖
- **流式聊天** — SSE 实时流、工具进度、Markdown 渲染、语法高亮
- **会话管理** — 全文搜索 (FTS5)、按日期分组的历史记录
- **多配置切换** — 独立的 Hermes 环境隔离
- **14 个工具集** — 网页、浏览器、终端、文件、代码、视觉、图像生成、TTS 等
- **记忆系统** — 查看/编辑条目、可发现提供者
- **16 个消息网关**
- **定时任务** — Cron 调度器，支持多种投递目标
- **看板** — 多 Agent 工作流任务管理
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

## 参与贡献

欢迎贡献！以下方式可以帮助改进项目：

- **Bug 报告** — [提交 Issue](https://github.com/shaoliang123456/hermes-desktop-rust/issues)，附上复现步骤
- **功能建议** — 描述使用场景和期望行为
- **Pull Request** — Fork 后创建分支，提交 PR。请先运行 `npm run lint` 和 `npm run typecheck`
- **翻译** — 在 `src/shared/i18n/locales/` 添加或改进翻译
- **Rust 后端** — 在 `native/src/` 改进或添加原生模块

特别需要帮助的领域：

- 编写 SSE/流式性能的自动化基准测试
- 添加更多 i18n 翻译（看板页面等）
- 改进 SSH 隧道可靠性和错误恢复
- 跨平台测试（Windows、Linux）

## 致谢

特别感谢 **[Fathah](https://github.com/fathah)** 创建了原版 **[hermes-desktop](https://github.com/fathah/hermes-desktop)** 项目。前端代码复用自原项目，在此深表感谢。原项目基于 [MIT License](https://opensource.org/licenses/MIT) 发布。

## 许可证

[MIT](LICENSE)
