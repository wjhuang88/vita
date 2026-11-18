# VITA-002-I — 查看服务并安全重启服务或关闭VM

Type: Story. Priority: P1. Status: **Refinement**.
Parent: [VITA-002](VITA-002-os-interaction-settings.md). Iteration: 未选择。

## Required Reads

- 本任务及[父级范围/统一门槛](VITA-002-os-interaction-settings.md)。
- [研究证据](../../proposals/OS-INTERACTION-AND-SETTINGS-RESEARCH.md)。
- [Talos嵌入契约](../../reference/TALOS-INTEGRATION.md)、[配置契约](../../reference/CONFIG-CONTRACT.md)。
- [ADR-0002](../../decisions/ADR-0002-governed-capability-broker.md)、[ADR-0005](../../decisions/ADR-0005-rust-first-system-capabilities.md)、[系统设计](../../reference/VITA-OS-DESIGN.md)。
- [测试](../../sop/TESTING.md)、[高风险审查](../../sop/PAIRING-WORKFLOW.md)、[I010现有缺口](../../iterations/I010-vm-model-tryout.md)。

## Outcome / scope

仅选型后监督器支持的一个非关键测试服务，以及正常关机/重启；提供任务影响和确认。

Dependencies: VITA-002-E, VITA-002-F。
Unlocks / external gates: 基础系统管理。

## Acceptance

- [ ] Given 服务不存在/无后端/异常，Then 显示真实不支持或失败，不依赖systemctl命令假设。
- [ ] Given 重启明确服务，When 用户确认，Then 验证新服务实例/健康并审计；不授予任意进程信号权限。
- [ ] Given 存在任务或配置事务，When 请求关机/重启，Then 展示影响并确认，flush受支持状态后走监督器关闭路径。
- [ ] 更新受影响的用户操作说明、父级子项状态及验证记录；不得只提交内部库而无上述可运行/可测试交付。

## Validation and delivery

VM服务故障、拒绝授权、关机/重启与日志证据；验证UI退出不等于关机。
交付证据需含精确源码版本、环境、命令/日志、通过与失败项；涉及boot必须双固件VM验证，涉及权限/秘密需独立审查。研究/模拟结果不能代替实机或生产验收。

Affected surfaces: Vita会话/设置/受控能力相关模块（按实际方案最小修改）、开发验证脚本、对应reference及用户说明。选型未定的路径不预建抽象。
Documentation: 更新本任务、父级记录和实际运行说明；新增可见操作更新README/用户手册。执行前按NEW-FEATURE与START-ITERATION路由，不自动并入I010。
Exclusions: 休眠、任意服务编辑、修改PID1选型。

## Readiness / evidence

任务已登记；前置子项验收及上述外部决策完成后，复核范围与测试再升Ready。
Implementation and validation: 未开始。

GitHub: https://github.com/wjhuang88/vita/issues/38
