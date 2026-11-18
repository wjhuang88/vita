# VITA-002-H — 配置时区与键盘并诊断系统时间

Type: Story. Priority: P1. Status: **Refinement**.
Parent: [VITA-002](VITA-002-os-interaction-settings.md). Iteration: 未选择。

## Required Reads

- 本任务及[父级范围/统一门槛](VITA-002-os-interaction-settings.md)。
- [研究证据](../../proposals/OS-INTERACTION-AND-SETTINGS-RESEARCH.md)。
- [Talos嵌入契约](../../reference/TALOS-INTEGRATION.md)、[配置契约](../../reference/CONFIG-CONTRACT.md)。
- [ADR-0002](../../decisions/ADR-0002-governed-capability-broker.md)、[ADR-0005](../../decisions/ADR-0005-rust-first-system-capabilities.md)、[系统设计](../../reference/VITA-OS-DESIGN.md)。
- [测试](../../sop/TESTING.md)、[高风险审查](../../sop/PAIRING-WORKFLOW.md)、[I010现有缺口](../../iterations/I010-vm-model-tryout.md)。

## Outcome / scope

提供时区/键盘选择与预览、时间同步只读状态；仅临时设置，切换键盘需确认保留/超时恢复。

Dependencies: VITA-002-E, VITA-002-F。
Unlocks / external gates: 基础本地设置。

## Acceptance

- [ ] Given 无模型，When 更改时区，Then 时间显示改变但系统UTC时钟不被隐式修改。
- [ ] Given 改变键盘布局，When 未确认保留或无法输入，Then 恢复旧布局并保留可用导航。
- [ ] Given 时间服务缺失或时钟导致TLS失败，Then 明确区分未支持/未同步/疑似时钟错误，不冒充诊断确定事实。
- [ ] 更新受影响的用户操作说明、父级子项状态及验证记录；不得只提交内部库而无上述可运行/可测试交付。

## Validation and delivery

时区/布局表验证、VM手动按键与恢复、无服务负例；记录串口布局属于宿主还是客体。
交付证据需含精确源码版本、环境、命令/日志、通过与失败项；涉及boot必须双固件VM验证，涉及权限/秘密需独立审查。研究/模拟结果不能代替实机或生产验收。

Affected surfaces: Vita会话/设置/受控能力相关模块（按实际方案最小修改）、开发验证脚本、对应reference及用户说明。选型未定的路径不预建抽象。
Documentation: 更新本任务、父级记录和实际运行说明；新增可见操作更新README/用户手册。执行前按NEW-FEATURE与START-ITERATION路由，不自动并入I010。
Exclusions: 手动系统校时、NTP配置、字体IME部署、持久化；后续范围。

## Readiness / evidence

任务已登记；前置子项验收及上述外部决策完成后，复核范围与测试再升Ready。
Implementation and validation: 未开始。

GitHub: https://github.com/wjhuang88/vita/issues/37
