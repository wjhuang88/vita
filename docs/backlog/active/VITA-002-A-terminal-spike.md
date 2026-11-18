# VITA-002-A — 验证 VM 全屏终端与组件边界

Type: Spike. Priority: P0. Status: **Ready**.
Parent: [VITA-002](VITA-002-os-interaction-settings.md). Iteration: 未选择。

## Required Reads

- 本任务及[父级范围/统一门槛](VITA-002-os-interaction-settings.md)。
- [研究证据](../../proposals/OS-INTERACTION-AND-SETTINGS-RESEARCH.md)。
- [Talos嵌入契约](../../reference/TALOS-INTEGRATION.md)、[配置契约](../../reference/CONFIG-CONTRACT.md)。
- [ADR-0002](../../decisions/ADR-0002-governed-capability-broker.md)、[ADR-0005](../../decisions/ADR-0005-rust-first-system-capabilities.md)、[系统设计](../../reference/VITA-OS-DESIGN.md)。
- [测试](../../sop/TESTING.md)、[高风险审查](../../sop/PAIRING-WORKFLOW.md)、[I010现有缺口](../../iterations/I010-vm-model-tryout.md)。

## Outcome / scope

以确定性模型构建非 root 可运行终端原型，比较 Vita 自有 ratatui/crossterm 与可提取 Talos 组件；不直接依赖 Talos CLI/TUI 作为产品契约。

Dependencies: 无未完成的硬前置；使用已有VM/SDK基线。
Unlocks / external gates: 解锁 B；原型与组件选择证据。

## Acceptance

- [ ] Given BIOS/UEFI VM 已启动，When 从宿主 PTY 和 QEMU 串口进入原型，Then 可输入、滚动、取消并回到可用提示符，记录两种连接差异。
- [ ] Given 窗口变化、中文粘贴、组合/宽字符、80×24与更小尺寸，When 操作界面，Then 无丢输入/永久错位；无法同步尺寸时给出固定布局降级，不伪称支持。
- [ ] Given UI异常退出、持续日志和快速流式输出，When 重新进入会话，Then 终端恢复且只有一个渲染者；记录RSS、输入延迟、待机CPU及依赖/许可证。
- [ ] 产出选型对比及建议ADR；失败结果也须有可复现案例和后续任务，不能仅给截图。
- [ ] 更新受影响的用户操作说明、父级子项状态及验证记录；不得只提交内部库而无上述可运行/可测试交付。

## Validation and delivery

PTY输入/resize/termios测试、BIOS/UEFI实际会话、中文人工操作记录；无需真实模型密钥。
交付证据需含精确源码版本、环境、命令/日志、通过与失败项；涉及boot必须双固件VM验证，涉及权限/秘密需独立审查。研究/模拟结果不能代替实机或生产验收。

Affected surfaces: Vita会话/设置/受控能力相关模块（按实际方案最小修改）、开发验证脚本、对应reference及用户说明。选型未定的路径不预建抽象。
Documentation: 更新本任务、父级记录和实际运行说明；新增可见操作更新README/用户手册。执行前按NEW-FEATURE与START-ITERATION路由，不自动并入I010。
Exclusions: 实体硬件中文IME、生产UI、网络写操作。

## Readiness / evidence

范围和验证已明确，可选择进入迭代；Ready不表示已实现或已批准任何真实设备操作。
Implementation and validation: 未开始。

GitHub: https://github.com/wjhuang88/vita/issues/30
