# VITA-002-D — 在界面内完成临时模型接入

Type: Story. Priority: P0. Status: **Refinement**.
Parent: [VITA-002](VITA-002-os-interaction-settings.md). Iteration: 未选择。

## Required Reads

- 本任务及[父级范围/统一门槛](VITA-002-os-interaction-settings.md)。
- [研究证据](../../proposals/OS-INTERACTION-AND-SETTINGS-RESEARCH.md)。
- [Talos嵌入契约](../../reference/TALOS-INTEGRATION.md)、[配置契约](../../reference/CONFIG-CONTRACT.md)。
- [ADR-0002](../../decisions/ADR-0002-governed-capability-broker.md)、[ADR-0005](../../decisions/ADR-0005-rust-first-system-capabilities.md)、[系统设计](../../reference/VITA-OS-DESIGN.md)。
- [测试](../../sop/TESTING.md)、[高风险审查](../../sop/PAIRING-WORKFLOW.md)、[I010现有缺口](../../iterations/I010-vm-model-tryout.md)。

## Outcome / scope

提供协议/endpoint/model/超时/秘密来源表单、测试与应用；默认会话临时存储，复用Talos provider。

Dependencies: VITA-002-B, VITA-002-C。
Unlocks / external gates: 首个配置闭环；外部阻塞：VITA-001-E transport-policy未关闭、I010 UEFI在线失败需解决或定位并完成本任务双固件验证。

## Acceptance

- [ ] Given 无有效模型，When 打开设置并填入配置，Then 可完全离线编辑，密钥遮罩且不进入对话、日志和导出。
- [ ] Given 用户确认测试调用，When DNS/TLS/认证/模型/工具阶段失败，Then 给出可操作且脱敏的分类，不暴露服务响应原文或静默切换端点。
- [ ] Given 新配置通过检查，When 明确应用，Then 在安全回合边界切换；失败保留旧配置，跨服务传历史须另行明确选择。
- [ ] Given BIOS/UEFI真实VM，When 从未配置状态完成设置并请求system_summary，Then 有实际工具执行与真实回复证据；退出清理临时秘密。
- [ ] 更新受影响的用户操作说明、父级子项状态及验证记录；不得只提交内部库而无上述可运行/可测试交付。

## Validation and delivery

表单/协议/脱敏测试、可控HTTPS服务负例、BIOS/UEFI从配置到工具调用的端到端验收。
交付证据需含精确源码版本、环境、命令/日志、通过与失败项；涉及boot必须双固件VM验证，涉及权限/秘密需独立审查。研究/模拟结果不能代替实机或生产验收。

Affected surfaces: Vita会话/设置/受控能力相关模块（按实际方案最小修改）、开发验证脚本、对应reference及用户说明。选型未定的路径不预建抽象。
Documentation: 更新本任务、父级记录和实际运行说明；新增可见操作更新README/用户手册。执行前按NEW-FEATURE与START-ITERATION路由，不自动并入I010。
Exclusions: 生产密钥持久化、自动模型回退、任意skills/MCP导入。

## Readiness / evidence

任务已登记；前置子项验收及上述外部决策完成后，复核范围与测试再升Ready。
Implementation and validation: 未开始。

GitHub: https://github.com/wjhuang88/vita/issues/33
