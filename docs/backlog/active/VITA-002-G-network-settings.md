# VITA-002-G — 离线配置有线网络与DNS并可恢复

Type: Story. Priority: P1. Status: **Refinement**.
Parent: [VITA-002](VITA-002-os-interaction-settings.md). Iteration: 未选择。

## Required Reads

- 本任务及[父级范围/统一门槛](VITA-002-os-interaction-settings.md)。
- [研究证据](../../proposals/OS-INTERACTION-AND-SETTINGS-RESEARCH.md)。
- [Talos嵌入契约](../../reference/TALOS-INTEGRATION.md)、[配置契约](../../reference/CONFIG-CONTRACT.md)。
- [ADR-0002](../../decisions/ADR-0002-governed-capability-broker.md)、[ADR-0005](../../decisions/ADR-0005-rust-first-system-capabilities.md)、[系统设计](../../reference/VITA-OS-DESIGN.md)。
- [测试](../../sop/TESTING.md)、[高风险审查](../../sop/PAIRING-WORKFLOW.md)、[I010现有缺口](../../iterations/I010-vm-model-tryout.md)。

## Outcome / scope

首批仅一个明确识别的有线接口、DHCP/静态地址/DNS；无模型依赖，应用与保留操作均在本地完成。

Dependencies: VITA-002-E, VITA-002-F。
Unlocks / external gates: 基本OS配置闭环。

## Acceptance

- [ ] Given 模型不可用，When 打开网络页，Then 显示真实接口/连接/地址/DNS及来源，未提供能力不显示为成功。
- [ ] Given 用户确认计划，When 应用候选配置，Then 后端持有超时恢复点并验证地址/路由/DNS，不仅检查模型端点。
- [ ] Given UI崩溃、断线、超时未保留或设备拔出，Then 恢复或准确报告无法恢复，审计完整，目标变更拒绝执行。
- [ ] Given 用户明确保留，Then 状态读回一致，临时/持久范围明确；宿主网络未改变。
- [ ] 更新受影响的用户操作说明、父级子项状态及验证记录；不得只提交内部库而无上述可运行/可测试交付。

## Validation and delivery

隔离VM网络故障/确认超时/进程崩溃测试、Agent与表单同计划对照；不得用开发机断网测试。
交付证据需含精确源码版本、环境、命令/日志、通过与失败项；涉及boot必须双固件VM验证，涉及权限/秘密需独立审查。研究/模拟结果不能代替实机或生产验收。

Affected surfaces: Vita会话/设置/受控能力相关模块（按实际方案最小修改）、开发验证脚本、对应reference及用户说明。选型未定的路径不预建抽象。
Documentation: 更新本任务、父级记录和实际运行说明；新增可见操作更新README/用户手册。执行前按NEW-FEATURE与START-ITERATION路由，不自动并入I010。
Exclusions: Wi-Fi、VPN、代理、防火墙、复杂路由；由父级后续清单承接。

## Readiness / evidence

任务已登记；前置子项验收及上述外部决策完成后，复核范围与测试再升Ready。
Implementation and validation: 未开始。

GitHub: https://github.com/wjhuang88/vita/issues/36
