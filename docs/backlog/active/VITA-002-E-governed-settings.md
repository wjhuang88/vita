# VITA-002-E — 打通表单与Agent共用的受控配置操作

Type: Story. Priority: P0. Status: **Refinement**.
Parent: [VITA-002](VITA-002-os-interaction-settings.md). Iteration: 未选择。

## Required Reads

- 本任务及[父级范围/统一门槛](VITA-002-os-interaction-settings.md)。
- [研究证据](../../proposals/OS-INTERACTION-AND-SETTINGS-RESEARCH.md)。
- [Talos嵌入契约](../../reference/TALOS-INTEGRATION.md)、[配置契约](../../reference/CONFIG-CONTRACT.md)。
- [ADR-0002](../../decisions/ADR-0002-governed-capability-broker.md)、[ADR-0005](../../decisions/ADR-0005-rust-first-system-capabilities.md)、[系统设计](../../reference/VITA-OS-DESIGN.md)。
- [测试](../../sop/TESTING.md)、[高风险审查](../../sop/PAIRING-WORKFLOW.md)、[I010现有缺口](../../iterations/I010-vm-model-tryout.md)。

## Outcome / scope

以会话级用户偏好作为可运行闭环，另以模拟特权后端验证批准协议；Broker IPC鉴权和风险门槛不得只存在于UI。

Dependencies: VITA-002-B, VITA-002-C。
Unlocks / external gates: 解锁 G/H/I；受控操作公共边界。

## Acceptance

- [ ] Given 用户或Agent提出同一修改，When 应用，Then 进入相同能力/计划/审计路径，权限结果一致。
- [ ] Given 高风险计划，When 无用户确认、确认过期、参数或目标变化、连接断开，Then 后端拒绝；模型文字不能构造有效确认。
- [ ] Given 多个同时编辑者及失败重试，Then revision冲突和操作ID可追踪，不重复执行不可逆步骤。
- [ ] Given UI重启，Then 可查询真实操作状态，不能把“取消等待”显示为“系统已撤销”；UI和日志对秘密均脱敏。
- [ ] 更新受影响的用户操作说明、父级子项状态及验证记录；不得只提交内部库而无上述可运行/可测试交付。

## Validation and delivery

IPC peer身份/权限/确认绑定测试、故障注入、真实VM用户偏好应用闭环及单独高风险技术审查。
交付证据需含精确源码版本、环境、命令/日志、通过与失败项；涉及boot必须双固件VM验证，涉及权限/秘密需独立审查。研究/模拟结果不能代替实机或生产验收。

Affected surfaces: Vita会话/设置/受控能力相关模块（按实际方案最小修改）、开发验证脚本、对应reference及用户说明。选型未定的路径不预建抽象。
Documentation: 更新本任务、父级记录和实际运行说明；新增可见操作更新README/用户手册。执行前按NEW-FEATURE与START-ITERATION路由，不自动并入I010。
Exclusions: 先实现格式化等危险动作、把mock当特权功能交付。

## Readiness / evidence

任务已登记；前置子项验收及上述外部决策完成后，复核范围与测试再升Ready。
Implementation and validation: 未开始。

GitHub: https://github.com/wjhuang88/vita/issues/34
