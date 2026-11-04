# Vita

**机器是工具，意图是交互界面。**

Vita 是一个开源的、**Rust-first 的 Agent 原生便携操作环境**。我们希望让一个 U 盘既能启动用户自带的各种 ISO，也能进入 Vita 自己的 Linux Live 系统；在这个系统中，[Talos Core](https://github.com/wjhuang88/talos) 是理解和操作机器的主要交互入口。

**Vita 不是“预装了 AI CLI 的 Linux 发行版”。** 它将围绕第一等公民的 Agent 会话运行时、结构化系统能力、可审计的权限边界与独立恢复路径来构建。

[![Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![I001 Foundation](https://github.com/wjhuang88/vita/actions/workflows/i001-foundation.yml/badge.svg)](https://github.com/wjhuang88/vita/actions/workflows/i001-foundation.yml)

[**English**](README.md) · [架构](docs/reference/SYSTEM-ARCHITECTURE.md) · [路线图](docs/roadmap/IMPLEMENTATION-ROADMAP.md) · [参与贡献](CONTRIBUTING.md)

> [!IMPORTANT]
> **当前为 Pre-alpha 工程验证阶段。** Vita 尚未发布供终端用户使用的 Live OS ISO 或 U 盘安装器。当前仓库已经通过的是启动链 **测试 ISO** 和独立的 Talos Runtime 嵌入原型，并非完整操作系统。

## 我们希望实现什么

- **一个 U 盘，多种启动选择**：保留用户自己的 ISO，不必每次重刷整个介质。
- **让 Talos 成为系统交互界面**：默认进入 Vita 专属 Agent 会话，而不是先启动 bash 再运行 Talos CLI。
- **以 Rust 原生系统能力为底座**：通过 Linux 的结构化接口实现系统状态获取与操作，减少额外语言运行时及命令行包装。
- **智能但受控**：权限分级、目标校验、显式确认、审计与离线恢复，是设计约束而非可选装饰。

## 目标架构

```text
               BIOS / UEFI
                    │
          Ventoy 启动兼容底座
            ┌───────┴────────┐
            │                │
        用户 ISO         Vita Live OS（规划中）
                             │
                   Linux + 最小用户空间
                             │
                    Vita Rust 会话宿主
                             │
                       Talos Runtime
                             │
                      Capability Broker
                             │
                    Rust 原生系统 Providers
                             │
                  /proc · /sys · netlink · ioctl

               独立恢复 Shell（规划中）
```

第一阶段选择 **Ventoy 负责 ISO 启动兼容，Vita 负责自己的 Live OS 与全部系统交互体验**。Talos 通过 `talos-runtime` SDK 嵌入，不依赖 `talos-cli` 包装。详见 [ADR-0004](docs/decisions/ADR-0004-hybrid-ventoy-boot-substrate.md)、[ADR-0001](docs/decisions/ADR-0001-talos-primary-interaction-runtime.md) 与 [ADR-0005](docs/decisions/ADR-0005-rust-first-system-capabilities.md)。

## 实际进度

| 能力 | 当前状态 |
| --- | --- |
| Ventoy 1.1.18 安装到经过目标验证的临时镜像 | **CI 已通过** |
| BIOS 启动 Vita 测试 ISO | **QEMU 已通过**：`VITA_BOOT_OK` |
| x86_64 UEFI / OVMF 启动 Vita 测试 ISO | **QEMU 已通过**：`VITA_BOOT_OK` |
| BIOS 启动第二个用户测试 ISO | **QEMU 已通过**：`VITA_USER_ISO_BOOT_OK` |
| Rust 直接嵌入 Talos Runtime，使用独立的 [vita-system-facts](crates/vita-system-facts/) Rust 模块执行只读 `system_summary` | **CI 已通过**：`VITA_I001_TALOS_RUNTIME_OK` |
| 真实第三方 Linux ISO 启动 | **待实现** |
| Vita Linux Live OS（内核、rootfs、会话） | **待实现** |
| 完整权限代理、配置持久化、恢复机制 | **待实现** |
| Secure Boot、真实 U 盘兼容性、可发布镜像 | **尚未验证** |

I001 基础阶段已经闭环。可查看 [迭代记录](docs/iterations/I001-foundation.md)、[启动原型](prototypes/boot/) 和 [交互原型](prototypes/interaction-runtime/)；CI 证据见 [GitHub Actions](https://github.com/wjhuang88/vita/actions/workflows/i001-foundation.yml)。

## 开发者快速验证

以下是**开发环境验证，不是 U 盘安装说明**。

```sh
git clone https://github.com/wjhuang88/vita.git
cd vita
sh scripts/validate_project_governance.sh .
```

在安装 Rust 的 Linux 主机上验证 Talos 嵌入：

```sh
rustup toolchain install 1.99.0 --profile minimal
cargo +1.99.0 run --manifest-path prototypes/interaction-runtime/Cargo.toml
```

成功时输出 `VITA_I001_TALOS_RUNTIME_OK`。该原型使用确定性模拟模型，不需要模型 API Key，也不需要访问在线模型。

I004 新增可在 Linux 宿主机上运行的 **Rust 原生离线控制台测试模式**：

```sh
cargo +1.99.0 run --manifest-path prototypes/interaction-runtime/Cargo.toml -- --console-fixture
```

该模式通过实际 Talos Runtime 调用只读系统信息能力，但使用确定性模拟模型，不是正式 Live OS。真人操作的 Ctrl+C、TTY/PTY、恢复路径等检查集中记录在 [I004 人工验收清单](docs/validation/I004-HUMAN-ACCEPTANCE.md)，供独立会话统一验收。

在**可丢弃的 Linux VM / CI Runner** 中验证 Ventoy 启动链：

```sh
bash scripts/ci/validate_i001_boot.sh
```

> [!CAUTION]
> 这段脚本会通过 `sudo` 安装开发依赖，使用 loop 设备和 Ventoy 安装器。但目标被限制为本次创建、经 backing-file 核验的临时 raw 镜像。**不要把脚本改成传入真实磁盘路径，也不要把它当作物理 U 盘写盘工具。**

## 规划中的 U 盘目录

```text
/iso/
  vita/vita-live.iso       # Vita 默认系统，尚未发布
  other-rescue.iso         # 用户自己的镜像
/talos/
  config/models.toml       # 模型/Provider 配置
  config/agent.toml        # Agent 和权限配置
  skills/
  workspace/
  logs/
/ventoy/ventoy.json
```

这是 [Phase 1 布局契约](docs/reference/USB-LAYOUT.md)，并非已发布介质。

## 参与和开源许可

欢迎贡献 Rust 原生 Linux 系统 API、极简 Live OS、ISO/固件兼容性、权限安全审查以及自动化测试。请先阅读 [CONTRIBUTING.md](CONTRIBUTING.md) 和 [AGENTS.md](AGENTS.md)。涉及写盘、引导和权限的变更必须遵循 [高风险审查流程](docs/sop/PAIRING-WORKFLOW.md)。

**Vita 自有代码采用 [Apache License 2.0](LICENSE)**。Ventoy 等第三方组件仍然遵守各自的许可证；其中 Ventoy 自有代码为 GPLv3+。未来发布包含第三方组件的镜像时需要独立完成许可证、NOTICE 与对应源码等合规检查。详见 [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)。

项目计划请参阅 [BOARD](docs/BOARD.md)、[Product Backlog](docs/backlog/PRODUCT-BACKLOG.md) 和 [Implementation Roadmap](docs/roadmap/IMPLEMENTATION-ROADMAP.md)。

---

**Vita 希望让机器自然地理解意图，同时始终把系统级控制权交还给使用者。**

## I005：Linux 内核与 Rust 用户空间工程验证

I005 正在实现 [QEMU 直接启动 Linux 内核并进入 Vita Rust 进程](prototypes/live-init/) 的最小验证路径，使用校验过的 Alpine 内核和静态 Rust 程序。它**不是**完整 Live OS、Talos 默认交互环境或实体 U 盘安装器，也不代表 Ventoy 真实 ISO 兼容性问题已经解决。参见 [ADR-0006](docs/decisions/ADR-0006-alpine-live-base.md) 及 [I005 人工验收/评审清单](docs/validation/I005-HUMAN-ACCEPTANCE.md)。
