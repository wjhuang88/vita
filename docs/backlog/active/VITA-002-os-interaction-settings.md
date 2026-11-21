# VITA-002 — Agent-native OS interaction and settings

Type: Epic. Priority: P0. Status: **Refinement / planning-ready, not executable as one task**.
Owner: Vita maintainer. Created: 2026-11-18, user authorized promotion from research to development records.
Related: VITA-001-C (OS services), D (Talos session), E (model/config), F (broker), G (recovery). This Epic coordinates bounded UI/settings slices; it does not duplicate or close their underlying OS responsibilities.

## Required Reads

- [Research](../../proposals/OS-INTERACTION-AND-SETTINGS-RESEARCH.md) and [D1 discovery disposition](VITA-001-D1-os-interface-research.md).
- [System design](../../reference/VITA-OS-DESIGN.md), [Talos integration](../../reference/TALOS-INTEGRATION.md), [config contract](../../reference/CONFIG-CONTRACT.md).
- [ADR-0002 broker](../../decisions/ADR-0002-governed-capability-broker.md), [ADR-0005 Rust-first](../../decisions/ADR-0005-rust-first-system-capabilities.md), [I010](../../iterations/I010-vm-model-tryout.md).
- Every selected child record below is mandatory context; it names its additional dependencies and acceptance.

## Product outcome and boundary

Vita启动后进入非root原生Agent主会话，用户能聊天、看系统状态，并在模型/网络不可用时使用本地设置修复模型与基本系统配置。自然语言与表单调用同一受控能力，不能以设置按钮绕过权限。

本记录确立开发需求范围，不把研究中的 ratatui、终端承载、网络管理器或加密方案直接批准为最终架构。会影响Soft/Assumption的实际技术选择仍需ADR与验证。

Hard: Talos SDK为主交互；模型无root shell；高风险操作有能力分级、目标、计划、显式确认、审计；秘密不进Git/ISO/对话；无模型仍可基本设置；真实设备写入另行明确授权和VM前置。
Soft: 轻量Rust TUI、复用可维护组件、最小服务依赖。
Assumptions: 终端中文/resize、网络恢复后端、持久化安全可满足需求；由A/F/J验证，不提前写为已实现。

## Children and dependency order

| Child / Required Reads | Outcome | Type | State | Depends on | Iteration |
| --- | --- | --- | --- | --- | --- |
| [VITA-002-A](VITA-002-A-terminal-spike.md) | 验证 VM 全屏终端与组件边界 | Spike | Ready | — | — |
| [VITA-002-B](VITA-002-B-tui-session.md) | 提供可用的全屏 Agent 会话与离线系统首页 | Story | Refinement | VITA-002-A | — |
| [VITA-002-C](VITA-002-C-config-contract.md) | 冻结最小配置事务与秘密引用契约 | Technical Story | Ready | — | — |
| [VITA-002-D](VITA-002-D-model-settings.md) | 在界面内完成临时模型接入 | Story | Refinement | VITA-002-B, VITA-002-C | — |
| [VITA-002-E](VITA-002-E-governed-settings.md) | 打通表单与Agent共用的受控配置操作 | Story | Refinement | VITA-002-B, VITA-002-C | — |
| [VITA-002-F](VITA-002-F-os-backends.md) | 选择首批网络、时间与服务后端 | Spike | Ready | — | — |
| [VITA-002-G](VITA-002-G-network-settings.md) | 离线配置有线网络与DNS并可恢复 | Story | Refinement | VITA-002-E, VITA-002-F | — |
| [VITA-002-H](VITA-002-H-locale-time-settings.md) | 配置时区与键盘并诊断系统时间 | Story | Refinement | VITA-002-E, VITA-002-F | — |
| [VITA-002-I](VITA-002-I-services-power.md) | 查看服务并安全重启服务或关闭VM | Story | Refinement | VITA-002-E, VITA-002-F | — |
| [VITA-002-J](VITA-002-J-persistence-discovery.md) | 明确跨机器配置与秘密持久化 | Spike | Refinement | VITA-002-C | — |
| [VITA-002-K](VITA-002-K-chinese-ime.md) | 纯 Rust 拼音与无桌面本机中文控制台分阶段验证（#40） | Spike | Active Stage 1 | 独立 PTY 原型可启动；最终集成依赖 VITA-002-A/B 与 VM 展示验证 | I011 Stage 1 |

第一步建议 A；C、F也已达到各自Spike/技术验证的准备条件，可后续选择。无并行执行授权，也未创建新迭代。B→D形成首个可配置模型的产品闭环；E→G/H/I逐项接入OS变更。
D额外受上游transport-policy和I010 UEFI在线问题制约；G/H/I受F与真实后端可用性制约。依赖图无环。不得选择父Epic替代子任务。

## 2026-11-21 用户授权纯 TUI 中文输入专项

单独 [#40](https://github.com/wjhuang88/vita/issues/40) / [I011](../../iterations/I011-chinese-ime-tui-spike.md) 已启动 Stage 1：独立 Rust `inputx-pinyin` + Ratatui 本机键入状态机与宿主 PTY 可运行验证。其第 2 阶段必须是 **BIOS/UEFI VM 内真实 Kmscon/DRM/KMS CJK 渲染+键盘**，第 3 阶段才允许架构选择与集成；不是对 Kmscon 或发行级中文支持的预先批准。它为 #30 的终端研究提供补充实验，不取代 #30 全量验收，也不提前把 B 提升为 Ready。默认无模型、无凭据、无磁盘写入，终端及 IME 故障不影响独立 ASCII 恢复。

## Milestones / completion

M1: A/B/C/D通过：可运行TUI、离线首页、临时模型接入及实际工具回复。
M2: E/F/G/H/I通过：统一受控设置、有线网络/DNS、时区/键盘、服务和电源操作。
M3: J完成持久化发现和后续任务；它只完成研究，不宣称产品持久化完成。
Epic完成要求上述子项全部验收，或经明确范围变更移出并留记录；未来功能不隐藏为“已支持”。默认不承诺生产安全/完整硬件OS。

## Deferred scope with owners

- Wi-Fi/VPN/代理/防火墙：VITA-001-C/F，F产出后再拆垂直任务；不塞入G。
- 手动系统校时/NTP写配置、字体/IME与实体终端：VITA-001-C/D，A/F验收后拆分。
- 加密持久化实现、跨机配置迁移：VITA-001-E，J后拆分，不止停留建议。
- 存储/用户镜像管理、P1/P3系统更新：VITA-001-B/C/F/H，保留#27前置和高风险门槛。
- 认证恢复、锁屏/身份管理：VITA-001-G/F，独立安全需求和人工验收。
- 全桌面、远程接管、通用软件商店：本Epic范围外。

## Validation/status

仅完成任务规划，代码未开发；I010仍按原记录Partial，不自动关闭或改变其基线。研究提案保留历史建议，本Epic及子项成为该新增开发范围的状态与验收owner。
规划验证：治理/链接检查、子项完整性与依赖无环检查、diff检查；结果见收尾记录。

GitHub: https://github.com/wjhuang88/vita/issues/29

2026-11-18规划校验：治理/链接检查PASS，diff检查PASS，10个子项字段完整且依赖无环。GitHub总任务#29，子项#30–#39；可启动项#30/#32/#35，其余按依赖细化。仅发布开发记录，未启动实现。
