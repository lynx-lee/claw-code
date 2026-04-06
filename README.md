# Claw Code

<p align="center">
  <a href="https://github.com/ultraworkers/claw-code">ultraworkers/claw-code</a>
  ·
  <a href="./USAGE.md">Usage</a>
  ·
  <a href="./rust/README.md">Rust workspace</a>
  ·
  <a href="./PARITY.md">Parity</a>
  ·
  <a href="./ROADMAP.md">Roadmap</a>
  ·
  <a href="https://discord.gg/5TUQKqFWd">UltraWorkers Discord</a>
</p>

<p align="center">
  <a href="https://star-history.com/#ultraworkers/claw-code&Date">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=ultraworkers/claw-code&type=Date&theme=dark" />
      <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=ultraworkers/claw-code&type=Date" />
      <img alt="Star history for ultraworkers/claw-code" src="https://api.star-history.com/svg?repos=ultraworkers/claw-code&type=Date" width="600" />
    </picture>
  </a>
</p>

<p align="center">
  <img src="assets/claw-hero.jpeg" alt="Claw Code" width="300" />
</p>

Claw Code is the public Rust implementation of the `claw` CLI agent harness.
The canonical implementation lives in [`rust/`](./rust), and the current source of truth for this repository is **ultraworkers/claw-code**.

> [!IMPORTANT]
> Start with [`USAGE.md`](./USAGE.md) for build, auth, CLI, session, and parity-harness workflows. Make `claw doctor` your first health check after building, use [`rust/README.md`](./rust/README.md) for crate-level details, read [`PARITY.md`](./PARITY.md) for the current Rust-port checkpoint, and see [`docs/container.md`](./docs/container.md) for the container-first workflow.

## Current repository shape

- **`rust/`** — canonical Rust workspace and the `claw` CLI binary
- **`USAGE.md`** — task-oriented usage guide for the current product surface
- **`PARITY.md`** — Rust-port parity status and migration notes
- **`ROADMAP.md`** — active roadmap and cleanup backlog
- **`PHILOSOPHY.md`** — project intent and system-design framing
- **`src/` + `tests/`** — companion Python/reference workspace and audit helpers; not the primary runtime surface

## Quick start

```bash
cd rust
cargo build --workspace
./target/debug/claw --help
./target/debug/claw prompt "summarize this repository"
```

Authenticate with either an API key or the built-in OAuth flow:

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
# or
cd rust
./target/debug/claw login
```

Run the workspace test suite:

```bash
cd rust
cargo test --workspace
```

## Documentation map

- [`USAGE.md`](./USAGE.md) — quick commands, auth, sessions, config, parity harness
- [`rust/README.md`](./rust/README.md) — crate map, CLI surface, features, workspace layout
- [`PARITY.md`](./PARITY.md) — parity status for the Rust port
- [`rust/MOCK_PARITY_HARNESS.md`](./rust/MOCK_PARITY_HARNESS.md) — deterministic mock-service harness details
- [`ROADMAP.md`](./ROADMAP.md) — active roadmap and open cleanup work
- [`PHILOSOPHY.md`](./PHILOSOPHY.md) — why the project exists and how it is operated

## Ecosystem

Claw Code is built in the open alongside the broader UltraWorkers toolchain:

- [clawhip](https://github.com/Yeachan-Heo/clawhip)
- [oh-my-openagent](https://github.com/code-yeongyu/oh-my-openagent)
- [oh-my-claudecode](https://github.com/Yeachan-Heo/oh-my-claudecode)
- [oh-my-codex](https://github.com/Yeachan-Heo/oh-my-codex)
- [UltraWorkers Discord](https://discord.gg/5TUQKqFWd)

## Ownership / affiliation disclaimer

- This repository does **not** claim ownership of the original Claude Code source material.
- This repository is **not affiliated with, endorsed by, or maintained by Anthropic**.

---

## 中文说明

### 项目简介

Claw Code 是 `claw` CLI Agent 的公开 Rust 实现。本项目支持多模型供应商，可通过配置文件灵活扩展。

### 多模型供应商支持

项目内置支持以下供应商：

| 供应商 | API 类型 | 环境变量 |
|--------|----------|----------|
| Anthropic | anthropic | `ANTHROPIC_API_KEY` |
| OpenAI | openai_compat | `OPENAI_API_KEY` |
| xAI | openai_compat | `XAI_API_KEY` |
| DeepSeek | openai_compat | `DEEPSEEK_API_KEY` |

### 添加新供应商

创建配置文件 `~/.config/claw-code/providers.toml`：

```toml
[[providers]]
name = "ollama"
api_type = "openai_compat"
api_key_env = "OLLAMA_API_KEY"
base_url_env = "OLLAMA_BASE_URL"
default_base_url = "http://localhost:11434/v1"

[[models]]
alias = "llama3"
canonical = "llama3.1:70b"
provider = "ollama"
max_output_tokens = 4096
context_window_tokens = 128000
```

配置完成后，即可使用 `claw --model llama3 "你的问题"` 调用新模型。

### 快速开始

```bash
# 构建项目
cd rust
cargo build --workspace

# 查看帮助
./target/debug/claw --help

# 使用默认模型（Anthropic）
export ANTHROPIC_API_KEY="sk-ant-..."
./target/debug/claw prompt "总结这个仓库"

# 使用其他模型
export XAI_API_KEY="your-xai-key"
./target/debug/claw --model grok "你好"

# 运行测试
cargo test --workspace
```

### 目录结构

```
rust/
├── crates/
│   ├── api/          # API 客户端，支持多供应商
│   ├── commands/     # 命令解析
│   ├── runtime/      # 运行时核心
│   └── rusty-claude-cli/  # CLI 入口
└── Cargo.toml
```

### 文档

- [USAGE.md](./USAGE.md) — 使用指南
- [rust/README.md](./rust/README.md) — Rust 工作区详情
- [PARITY.md](./PARITY.md) — Rust 移植状态
- [ROADMAP.md](./ROADMAP.md) — 开发路线图
