# Claw CLI 使用指南

> Claw v0.1.0 - 一个强大的交互式 AI 助手命令行工具

## 目录

- [快速开始](#快速开始)
- [命令模式](#命令模式)
- [命令行参数](#命令行参数)
- [Slash 命令大全](#slash-命令大全)
- [子命令速查](#子命令速查)
- [实用示例](#实用示例)
- [会话管理](#会话管理)
- [权限与安全](#权限与安全)

---

## 快速开始

### 基础用法

```bash
# 进入交互式 REPL 模式
claw

# 发送单条提示并退出
claw "总结这个仓库"

# 使用指定模型
claw --model claude-opus "解释 src/main.rs 的代码"

# JSON 格式输出
claw --output-format json prompt "分析项目结构"
```

### 第一步：登录

```bash
claw login
```

首次使用前需要先登录以连接服务。

---

## 命令模式

Claw 支持三种主要使用模式：

### 1. 交互式 REPL 模式
```bash
claw [--model MODEL] [--allowedTools TOOL[,TOOL...]]
# 进入交互式会话，可使用 / 开头的 slash 命令
```

### 2. 单次提示模式
```bash
claw [--model MODEL] [--output-format text|json] prompt TEXT
# 发送提示后立即退出，返回结果
```

### 3. 快捷提示模式
```bash
claw [--model MODEL] [--output-format text|json] TEXT
# 单次提示的简写形式
```

---

## 命令行参数

### 通用标志

| 参数 | 说明 | 示例 |
|------|------|------|
| `--model MODEL` | 覆盖当前使用的模型 | `--model claude-opus` |
| `--output-format FORMAT` | 非交互模式输出格式: `text` 或 `json` | `--output-format json` |
| `--permission-mode MODE` | 权限模式：`read-only`, `workspace-write`, `danger-full-access` | `--permission-mode read-only` |
| `--allowedTools TOOLS` | 限制可用的工具（可重复，支持逗号分隔别名） | `--allowedTools read,glob` |
| `--dangerously-skip-permissions` | 跳过所有权限检查（危险！） | `--dangerously-skip-permissions` |
| `--version, -V` | 打印版本和构建信息 | `--version` |
| `--help` | 显示帮助信息 | `--help` |

### 会话相关

| 参数 | 说明 |
|------|------|
| `--resume [SESSION.jsonl\|session-id\|latest]` | 恢复指定的会话，支持 `/status`, `/compact` 等后附命令 |

---

## Slash 命令大全

在交互式 REPL 模式中使用 `/` 开头的命令。

### 🚀 快速启动命令

```
/status      显示当前会话状态
/diff        显示工作区 git 差异
/agents      列出配置的代理
/skills      列出可用技能
/commit      生成 commit 消息并提交
```

---

### 📊 会话与可见性

| 命令 | 说明 | 可恢复 |
|------|------|--------|
| `/help` | 显示可用 slash 命令列表 | ✅ |
| `/status` | 显示当前会话状态 | ✅ |
| `/sandbox` | 显示沙箱隔离状态 | ✅ |
| `/model [model]` | 显示或切换当前模型 | ❌ |
| `/permissions [模式]` | 显示或切换权限模式 | ❌ |
| `/cost` | 显示当前会话累计 token 使用量 | ✅ |
| `/resume <session-path>` | 将保存的会话加载到 REPL | ❌ |
| `/version` | 显示 CLI 版本信息 | ✅ |
| `/session [list\|switch <id>\|fork [name]]` | 列表/切换/分支会话 | ❌ |
| `/login` | 登录到服务 | ❌ |
| `/logout` | 退出登录 | ❌ |
| `/usage` | 显示详细的 API 使用统计 | ✅ |
| `/stats` | 显示工作区和会话统计 | ✅ |
| `/rename <name>` | 重命名当前会话 | ❌ |
| `/privacy-settings` | 查看或修改隐私设置 | ✅ |

---

### 🗂️ 工作区与 Git

| 命令 | 说明 | 可恢复 |
|------|------|--------|
| `/compact` | 压缩本地会话历史 | ✅ |
| `/clear [--confirm]` | 创建新的本地会话 | ✅ |
| `/config [env\|hooks\|model\|plugins]` | 检查 Claude 配置文件 | ✅ |
| `/memory` | 检查加载的 Claude 指令内存文件 | ✅ |
| `/init` | 为当前仓库创建 starter CLAUDE.md | ✅ |
| `/diff` | 显示当前工作区变更的 git diff | ✅ |
| `/commit` | 生成 commit 消息并创建 git commit | ❌ |
| `/pr [context]` | 从对话中起草或创建 pull request | ❌ |
| `/issue [context]` | 从对话中起草或创建 GitHub issue | ❌ |
| `/export [file]` | 导出当前对话到文件 | ✅ |
| `/plugin [list\|install\|enable\|disable\|uninstall\|update]` | 管理插件 | ❌ |
| `/hooks [list\|run <hook>]` | 列出和管理生命周期钩子 | ✅ |
| `/files` | 列出当前上下文窗口中的文件 | ✅ |
| `/branch [name]` | 创建或切换 git 分支 | ❌ |
| `/release-notes` | 从最近变更生成发布说明 | ❌ |
| `/add-dir <path>` | 添加额外目录到上下文 | ❌ |

---

### 🔍 发现与调试

| 命令 | 说明 | 可恢复 |
|------|------|--------|
| `/mcp [list\|show <server>\|help]` | 检查配置的 MCP 服务器 | ✅ |
| `/teleport <symbol-or-path>` | 通过搜索工作区跳转到文件或符号 | ❌ |
| `/debug-tool-call` | 用调试详情重放上次工具调用 | ❌ |
| `/agents [list\|help]` | 列出配置的代理 | ✅ |
| `/skills [list\|install\|help\|<skill> [args]]` | 列出/安装/调用技能 | ✅ |
| `/doctor` | 诊断设置问题和环境健康状态 | ✅ |
| `/tasks [list\|get <id>\|stop <id>]` | 列出和管理后台任务 | ✅ |
| `/context [show\|clear]` | 检查或管理对话上下文 | ✅ |
| `/desktop` | 打开或管理桌面应用集成 | ❌ |
| `/ide [vscode\|cursor]` | 打开或配置 IDE 集成 | ❌ |

---

### 🤖 分析与自动化

| 命令 | 说明 | 可恢复 |
|------|------|--------|
| `/bughunter [scope]` | 检查工作区中的潜在 bug | ❌ |
| `/ultraplan [task]` | 运行深度规划提示，多步骤推理 | ❌ |
| `/review [scope]` | 对当前变更运行代码审查 | ❌ |
| `/advisor` | 切换顾问模式，仅提供指导 | ✅ |
| `/insights` | 显示会话的 AI 生成洞察 | ✅ |
| `/security-review [scope]` | 对工作区运行安全审查 | ❌ |

---

### ⚙️ 会话快捷命令（可恢复）

以下命令支持在 `--resume` 模式下使用：

```
/help, /status, /sandbox, /compact, /clear [--confirm], /cost, 
/config [env\|hooks\|model\|plugins], /mcp [list\|show\|help], /memory, 
/init, /diff, /version, /export [file], /agents [list\|help], 
/skills [list\|install\|help\|<skill> [args]], /doctor, /plan [on\|off], 
/tasks [list\|get\|stop], /theme [theme-name], /vim, /usage, /stats, 
/copy [last\|all], /hooks [list\|run <hook>], /files, /context [show\|clear], 
/color [scheme], /effort [low\|medium\|high], /fast, /summary, /tag [label], 
/brief, /advisor, /stickers, /insights, /thinkback, /keybindings, 
/privacy-settings, /output-style [style], /allowed-tools [add\|remove\|list] [tool], 
/terminal-setup, /language [language], /max-tokens [count], /temperature [value], 
/system-prompt, /tool-details <tool-name>, /bookmarks [add\|remove\|list], 
/workspace [path], /history [count], /tokens, /cache, /providers, 
/notifications [on\|off\|status], /changelog [count], /blame <file> [line], 
/log [count], /cron [list\|add\|remove], /team [list\|create\|delete], 
/telemetry [on\|off\|status], /env, /project, /map [depth], 
/symbols <path>, /hover <symbol>, /diagnostics [path], 
/alias <name> <command>, /agent [list\|spawn\|kill], 
/subagent [list\|steer <target> <msg>\|kill <id>], 
/reasoning [on\|off\|stream], /budget [show\|set <limit>], 
/rate-limit [status\|set <rpm>], /metrics
```

---

## 子命令速查

以下命令可在 CLI 中直接以子命令形式使用（非交互模式）：

| 子命令 | 说明 |
|--------|------|
| `claw version` | 显示版本信息 |
| `claw status` | 显示当前本地工作区状态快照 |
| `claw sandbox` | 显示当前沙箱隔离状态快照 |
| `claw doctor` | 诊断本地认证、配置、工作区和沙箱健康状态 |
| `claw dump-manifests` | 转储清单文件 |
| `claw bootstrap-plan` | 显示引导计划 |
| `claw agents` | 显示配置的代理 |
| `claw mcp` | 管理 MCP 服务器 |
| `claw skills` | 管理技能 |
| `claw system-prompt [--cwd PATH] [--date YYYY-MM-DD]` | 显示或生成系统提示 |
| `claw login` | 登录 |
| `claw logout` | 登出 |
| `claw init` | 初始化 |
| `claw help` | 显示帮助 |

---

## 实用示例

### 基础使用

```bash
# 快速提问
claw "解释这个项目的架构"

# 使用指定模型
claw --model claude-opus "优化这段代码的性能"
```

### 输出格式控制

```bash
# JSON 格式输出（适合脚本集成）
claw --output-format json prompt "列出所有 Rust 源文件" > output.json

# 文本格式（默认）
claw --output-format text "总结变更"
```

### 权限控制

```bash
# 限制只读工具
claw --allowedTools read,glob "分析项目结构"

# 设置权限模式为只读
claw --permission-mode read-only "尝试修改文件"
```

### 会话管理

```bash
# 恢复最近的会话
claw --resume latest

# 恢复特定会话并执行命令
claw --resume session.jsonl /status /diff /export notes.txt

# 在 REPL 中列出所有会话
claw
> /session list

# 切换到另一个会话
claw
> /session switch <session-id>

# 分支当前会话
claw
> /session fork feature-branch
```

### 工作区操作

```bash
# 查看当前变更
claw
> /diff

# 生成 commit
claw
> /commit

# 创建 PR
claw
> /pr "添加新功能 X"

# 创建分支
claw
> /branch feature/new-feature
```

### 调试与分析

```bash
# 诊断环境问题
claw doctor

# 检查安全漏洞
claw
> /security-review

# 代码审查
claw
> /review src/

# 查找潜在 bug
claw
> /bughunter
```

### 技能与工具

```bash
# 列出可用技能
claw skills
claw
> /skills list

# 使用技能
claw
> /skills <skill-name> [args]

# 列出 MCP 服务器
claw mcp
claw
> /mcp list

# 查看特定 MCP 服务器详情
claw mcp show my-server
```

### 后台任务管理

```bash
# 列出所有后台任务
claw
> /tasks list

# 获取任务详情
claw
> /tasks get <task-id>

# 停止任务
claw
> /tasks stop <task-id>
```

### 插件管理

```bash
# 列出已安装插件
claw
> /plugin list

# 安装插件
claw
> /plugin install /path/to/plugin

# 启用/禁用插件
claw
> /plugin enable <name>
claw
> /plugin disable <name>

# 更新插件
claw
> /plugin update <id>
```

---

## 会话管理

### 会话文件位置

自动保存的会话存储在：
```
.claw/sessions/<session-id>.jsonl
```

### 会话操作

| 操作 | 命令 |
|------|------|
| 列出会话 | `/session list` |
| 切换会话 | `/session switch <session-id>` |
| 分支会话 | `/session fork [branch-name]` |
| 重命名会话 | `/rename <name>` |
| 导出会话 | `/export [file]` |
| 恢复会话 | `--resume <path>` 或 `/resume <path>` |

### 快捷引用

- `latest` - 引用最新保存的会话
- 可在 `--resume` 后附加命令，如 `claw --resume latest /status /diff`

---

## 权限与安全

### 权限模式

| 模式 | 描述 |
|------|------|
| `read-only` | 只允许读取操作 |
| `workspace-write` | 允许在工作区内写入 |
| `danger-full-access` | 允许所有操作（危险） |

### 工具限制

使用 `--allowedTools` 限制可用工具：

```bash
# 只允许 read 和 glob 工具
claw --allowedTools read,glob "分析项目"

# 多个工具可用逗号分隔
claw --allowedTools read,glob,write,execute "执行任务"
```

### 跳过权限检查（不推荐）

```bash
--dangerously-skip-permissions
```

⚠️ **警告**：此选项会绕过所有安全检查，可能导致意外操作！

---

## 在 macOS 上安装和构建

### 前置条件

在开始之前，请确保您已经安装了以下工具：

#### 1. 安装 Rust 工具链

如果尚未安装 Rust，请使用官方安装脚本：

```bash
# 下载并运行 Rustup 安装脚本
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 使 cargo 在当前 shell 中可用
source $HOME/.cargo/env

# 验证安装
rustc --version
cargo --version
```

#### 2. 克隆仓库

```bash
# 克隆项目
git clone <repository-url>
cd claw-code
```

### 构建项目

```bash
# 进入 Rust 工作区目录
cd rust

# 构建整个工作区（调试模式）
cargo build --workspace

# 构建优化版本（发布模式，速度更快但编译时间更长）
cargo build --workspace --release
# 或者使用 cargo install 直接构建安装
cargo install --path crates/rusty-claude-cli
```

构建完成后，二进制文件位于 `rust/target/debug/claw` 或 `rust/target/release/claw`。

### 可选：将 claw 添加到 PATH

为方便全局使用，可以创建符号链接：

```bash
# 创建符号链接到 /usr/local/bin（可能需要 sudo）
sudo ln -s /path/to/claw-code/rust/target/debug/claw /usr/local/bin/claw

# 或者添加到 ~/.zshrc
echo 'export PATH="$HOME/gitX/other/claw-code/rust/target/debug:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### 第一次运行：健康检查

构建完成后，先运行诊断检查：

```bash
cd rust
./target/debug/claw
# 在 REPL 中运行
/doctor
```

`/doctor` 命令会诊断设置问题和环境健康状态，是首次使用的首选命令。

### 配置 API 凭证

有两种认证方式：

#### 方式 1：API 密钥（简单）

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
```

将环境变量添加到 `~/.zshrc` 以永久生效：

```bash
echo 'export ANTHROPIC_API_KEY="sk-ant-..."' >> ~/.zshrc
source ~/.zshrc
```

#### 方式 2：OAuth 登录（推荐）

```bash
cd rust
./target/debug/claw login
```

OAuth 方式会将凭证安全地存储在本地。

### 多模型供应商配置

Claw 支持多种 AI 模型供应商，可通过配置文件灵活配置。

#### 配置文件路径

| 操作系统 | 路径 |
|----------|------|
| macOS | `~/Library/Application Support/claw-code/providers.toml` |
| Linux | `~/.config/claw-code/providers.toml` |

#### 配置文件格式

```toml
# 方式一：使用环境变量
[[providers]]
name = "anthropic"
api_type = "anthropic"
api_key_env = "ANTHROPIC_API_KEY"
base_url_env = "ANTHROPIC_BASE_URL"
default_base_url = "https://api.anthropic.com"

# 方式二：直接在配置文件中指定 API key（推荐）
[[providers]]
name = "ollama"
api_type = "open_ai_compat"
api_key_env = "OLLAMA_API_KEY"
base_url_env = "OLLAMA_BASE_URL"
default_base_url = "http://localhost:11434/v1"
api_key = "your-api-key-here"
base_url = "http://192.168.124.168:8080/v1"

# 方式三：混合使用（配置文件优先，环境变量作为后备）
[[providers]]
name = "xai"
api_type = "open_ai_compat"
api_key_env = "XAI_API_KEY"
base_url_env = "XAI_BASE_URL"
default_base_url = "https://api.x.ai/v1"
# api_key = "sk-xxx"  # 可选：直接指定，优先于环境变量

# 模型别名配置
[[models]]
alias = "llama3"
canonical = "llama3"
provider = "ollama"
max_output_tokens = 4096
context_window_tokens = 128000
```

#### 内置支持的供应商

| 供应商 | api_type | 环境变量 |
|--------|----------|----------|
| Anthropic | `anthropic` | `ANTHROPIC_API_KEY` |
| OpenAI | `open_ai_compat` | `OPENAI_API_KEY` |
| xAI | `open_ai_compat` | `XAI_API_KEY` |
| DeepSeek | `open_ai_compat` | `DEEPSEEK_API_KEY` |
| Ollama | `open_ai_compat` | `OLLAMA_API_KEY` |

#### 使用示例

```bash
# 使用配置文件中的模型（无需设置环境变量）
./target/debug/claw --model llama3 "你好"

# 使用内置别名
./target/debug/claw --model grok "解释这段代码"
./target/debug/claw --model deepseek "帮我写一个函数"

# 查看可用模型
./target/debug/claw
> /model
```

#### 配置优先级

1. 配置文件中的 `api_key` 和 `base_url`（最高优先级）
2. 环境变量（`api_key_env` 和 `base_url_env` 指定的变量）
3. `default_base_url`（仅作为 base_url 的默认值）

#### 注意事项

- `api_type` 必须是 `anthropic` 或 `open_ai_compat`（带下划线）
- 配置文件修改后立即生效，无需重启
- API key 存储在配置文件中，请注意文件权限安全

### 沙箱支持（macOS）

macOS 的沙箱支持有限：

- **命名空间隔离**：不可用（需要 Linux 的 `unshare`）
- **文件系统限制**：可用（workspace-only 模式）
- **网络隔离**：需要额外配置

在 macOS 上，文件系统模式默认限制为 `workspace-only`，这意味着 claw 只能访问工作区内的文件。

---

## 环境要求

### 首次使用

1. **登录**：`claw login`
2. **配置**：检查 `.claude.json` 和 `.claude/settings.local.json`
3. **初始化**（可选）：`claw init` 创建 CLAUDE.md

### 系统要求

- **操作系统**：macOS 10.15+、Linux、Windows
- **Rust**：1.70+（推荐最新稳定版）
- **磁盘空间**：至少 1GB（用于构建和依赖）
- **内存**：推荐 4GB+

### 常用 macOS 问题

**问题：找不到 `cargo` 命令**
```bash
# 确保 Rust 已正确安装并添加到 PATH
source $HOME/.cargo/env

# 或者重新安装 Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**问题：编译失败**
```bash
# 更新 Rust 工具链
rustup update

# 清理并重新构建
cargo clean
cargo build --workspace
```

**问题：权限错误**
```bash
# 检查权限模式
claw
> /permissions

# 切换到适当的模式
claw --permission-mode workspace-write
```

---

## 提示技巧

### 高效使用

1. **使用 slash 命令快速导航**
   - `/status` - 快速查看当前状态
   - `/diff` - 查看变更
   - `/compact` - 压缩历史，节省 token

2. **会话复用**
   - 使用 `--resume latest` 继续上次工作
   - 使用 `/session fork` 创建分支试验

3. **输出格式**
   - 脚本集成使用 `--output-format json`
   - 人类阅读使用 `--output-format text`

4. **权限控制**
   - 日常开发使用 `workspace-write`
   - 敏感操作使用 `read-only`

---

## 故障排除

### 常见问题

**问题：`claw doctor` 显示配置问题**
```bash
# 运行诊断
claw doctor
```

**问题：无法访问某些文件**
```bash
# 检查权限模式
claw
> /permissions

# 切换模式（谨慎）
claw --permission-mode workspace-write
```

**问题：会话无法恢复**
```bash
# 列出可用会话
claw
> /session list

# 尝试恢复特定的
claw --resume <session-id>
```

---

## 更多信息

- **帮助**：`claw help` 或 `claw --help`
- **版本**：`claw version` 或 `claw --version`
- **状态检查**：`claw status`
- **诊断**：`claw doctor`

---

*文档生成自 claw v0.1.0 | 最后更新：2026-04-07*
