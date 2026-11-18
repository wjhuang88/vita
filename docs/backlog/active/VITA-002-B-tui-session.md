# VITA-002-B — 提供可用的全屏 Agent 会话与离线系统首页

Type: Story. Priority: P0. Status: **Refinement**.
Parent: [VITA-002](VITA-002-os-interaction-settings.md). Iteration: 未选择。

## Required Reads

- 本任务及[父级范围/统一门槛](VITA-002-os-interaction-settings.md)。
- [研究证据](../../proposals/OS-INTERACTION-AND-SETTINGS-RESEARCH.md)。
- [Talos嵌入契约](../../reference/TALOS-INTEGRATION.md)、[配置契约](../../reference/CONFIG-CONTRACT.md)。
- [ADR-0002](../../decisions/ADR-0002-governed-capability-broker.md)、[ADR-0005](../../decisions/ADR-0005-rust-first-system-capabilities.md)、[系统设计](../../reference/VITA-OS-DESIGN.md)。
- [测试](../../sop/TESTING.md)、[高风险审查](../../sop/PAIRING-WORKFLOW.md)、[I010现有缺口](../../iterations/I010-vm-model-tryout.md)。

## Outcome / scope

实现会话/系统/设置入口、键盘导航、多行输入、工具进度及只读facts；模型未配置时仍可进入首页。

Dependencies: VITA-002-A。
Unlocks / external gates: 解锁 D/E/H/I；可交互产品骨架。

## Acceptance

- [ ] Given 无网络、无密钥，When 启动，Then 首页/帮助/只读系统状态仍可使用，模型状态明确为未配置。
- [ ] Given 模型流式响应或慢工具，When 输入、切换页面、取消，Then UI保持响应，取消状态真实，不将退出误报为任务回滚。
- [ ] Given 模型/日志含ANSI或OSC控制序列，When 渲染，Then 不修改标题/剪贴板或伪造系统审批界面。
- [ ] Given 小窗口、历史增长和输入过长，Then 有受限缓冲、分页及降级，不发生无界内存增长。
- [ ] 更新受影响的用户操作说明、父级子项状态及验证记录；不得只提交内部库而无上述可运行/可测试交付。

## Validation and delivery

渲染状态/转义过滤测试、PTY集成、VM开机进入UI与故障恢复。
交付证据需含精确源码版本、环境、命令/日志、通过与失败项；涉及boot必须双固件VM验证，涉及权限/秘密需独立审查。研究/模拟结果不能代替实机或生产验收。

Affected surfaces: Vita会话/设置/受控能力相关模块（按实际方案最小修改）、开发验证脚本、对应reference及用户说明。选型未定的路径不预建抽象。
Documentation: 更新本任务、父级记录和实际运行说明；新增可见操作更新README/用户手册。执行前按NEW-FEATURE与START-ITERATION路由，不自动并入I010。
Exclusions: 持久会话、远程接管、特权写操作、全桌面。

## Readiness / evidence

任务已登记；前置子项验收及上述外部决策完成后，复核范围与测试再升Ready。
Implementation and validation: 未开始。

GitHub: https://github.com/wjhuang88/vita/issues/31
