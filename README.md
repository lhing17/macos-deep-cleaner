# macOS Deep Cleaner (mdc)

一个用于深度清理 macOS 系统的命令行工具，专门解决卸载开发工具、IDE 和库后系统空间没有明显恢复的问题。它能扫描 `~/Library`、`/Library` 以及用户目录中遗留的缓存、日志、配置文件和开发环境残留，并提供安全的预览与清理能力。

> **初版定位**：CLI 命令行工具，底层使用 Rust 编写，未来可无缝升级为 Tauri 桌面应用。

## 主要特性

- 🔍 **多维度扫描**：覆盖系统缓存、日志、应用残留、以及 Homebrew / Node.js / Docker / Python / Xcode / JetBrains 等常见开发环境垃圾。
- 📊 **可视化报告**：终端表格输出，支持按分类、安全等级汇总，快速定位空间占用大户。
- 🛡️ **安全优先**：所有清理操作默认进入 **dry-run** 模式；删除时使用系统废纸篓（`trash`），提供"后悔药"。
- ⚡ **Rust 高性能**：并发文件遍历，快速完成全盘扫描。
- 🔧 **Tauri 预留**：核心逻辑与 CLI 解耦，未来可直接作为 Tauri 后端复用。

## 安装

### 从源码编译

需要安装 [Rust](https://rustup.rs/)（1.70+）。

```bash
git clone https://github.com/yourusername/macos-deep-cleaner.git
cd macos-deep-cleaner
cargo install --path crates/mdc-cli --bin mdc
```

编译完成后，`mdc` 命令即可使用。

## 使用示例

### 1. 扫描系统垃圾

```bash
# 默认扫描全部并输出终端表格报告
mdc scan

# 只扫描开发环境相关残留
mdc scan --category "Dev"

# 输出 JSON 格式（方便脚本集成）
mdc scan --report json
```

### 2. 预览将要清理的内容

```bash
mdc clean --dry-run
```

### 3. 执行清理（移入废纸篓）

```bash
# 先 dry-run 确认无误后，加上 --confirm 执行真实清理
mdc clean --confirm

# 只清理某个分类
mdc clean --confirm --category "Dev - Node"
```

## 项目结构

```
macos-deep-cleaner/
├── Cargo.toml
├── crates/
│   ├── mdc-core/     # 核心引擎：扫描器、分析器、清理器
│   ├── mdc-cli/      # 命令行入口
│   └── mdc-rules/    # 清理规则与内置 macOS 规则集
```

## 支持清理的类别

| 分类 | 说明 | 示例路径 |
|------|------|----------|
| System Cache | 用户/系统级应用缓存 | `~/Library/Caches`, `/Library/Caches` |
| System Logs | 应用与系统日志 | `~/Library/Logs`, `/var/log` |
| App Leftovers | 已卸载应用的残留数据 | `~/Library/Application Support`, `~/Library/Preferences` |
| Dev - Homebrew | Homebrew 下载缓存 | `~/Library/Caches/Homebrew` |
| Dev - Node.js | npm / yarn / pnpm 缓存 | `~/.npm`, `~/.yarn/cache` |
| Dev - Docker | Docker 镜像与构建缓存 | 通过 Docker CLI/API 检测 |
| Dev - Python | pip 缓存与 `__pycache__` | `~/.cache/pip` |
| Dev - Xcode | 派生数据、归档、模拟器 | `~/Library/Developer/Xcode/DerivedData` |
| Dev - JetBrains | IDE 缓存与日志 | `~/Library/Caches/JetBrains` |
| Dev - General | 通用开发缓存 | `~/.gradle/caches`, `~/.cargo/registry/cache` |
| .DS_Store | 散落的 macOS 元数据文件 | 常见目录下的 `.DS_Store` |
| Trash | 废纸篓内容 | `~/.Trash` |

## 安全说明

- **Dry-run 是默认行为**：`mdc clean` 不带 `--confirm` 不会删除任何文件。
- **废纸篓机制**：真实清理时文件会被移入系统 Trash，而非直接 `rm -rf`。
- **安全等级标记**：
  - 🟢 **Safe**：低风险缓存/日志
  - 🟡 **Caution**：配置文件或应用数据
  - 🔴 **Danger**：可能仍在使用（当前版本以 Caution 为主，后续将增强启发式检测）
- **权限不足优雅降级**：受 SIP 保护或需要 `sudo` 的目录会自动跳过并提示。

## 开发计划

| 阶段 | 内容 | 状态 |
|------|------|------|
| Phase 1 | Workspace 搭建、并发扫描框架 | ✅ |
| Phase 2 | 规则引擎与各类扫描器实现 | 🚧 |
| Phase 3 | 分析与报告（表格 / JSON） | 🚧 |
| Phase 4 | 安全清理与 dry-run 机制 | 🚧 |
| Phase 5 | 集成测试、文档与发布 | 🚧 |

## 贡献

欢迎提交 Issue 和 PR！如果你有特定的开发工具残留清理需求，可以在 `crates/mdc-rules/src/builtins.rs` 中添加新的规则。

## License

MIT
