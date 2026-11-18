# VITA-002-C — 冻结最小配置事务与秘密引用契约

Type: Technical Story. Priority: P0. Status: **Ready**.
Parent: [VITA-002](VITA-002-os-interaction-settings.md). Iteration: 未选择。

## Required Reads

- 本任务及[父级范围/统一门槛](VITA-002-os-interaction-settings.md)。
- [研究证据](../../proposals/OS-INTERACTION-AND-SETTINGS-RESEARCH.md)。
- [Talos嵌入契约](../../reference/TALOS-INTEGRATION.md)、[配置契约](../../reference/CONFIG-CONTRACT.md)。
- [ADR-0002](../../decisions/ADR-0002-governed-capability-broker.md)、[ADR-0005](../../decisions/ADR-0005-rust-first-system-capabilities.md)、[系统设计](../../reference/VITA-OS-DESIGN.md)。
- [测试](../../sop/TESTING.md)、[高风险审查](../../sop/PAIRING-WORKFLOW.md)、[I010现有缺口](../../iterations/I010-vm-model-tryout.md)。

## Outcome / scope

先交付可测试的配置状态机和能力协议原型：schema/revision/scope/source、草稿/校验/计划/确认/应用/读回；以内存后端验证，不连接真实系统修改。

Dependencies: 无未完成的硬前置；使用已有VM/SDK基线。
Unlocks / external gates: 解锁 D/E；最小契约和可执行测试。

## Acceptance

- [ ] 给定相同revision和字段，表单与Agent输入生成相同类型化计划；revision冲突、未知字段、过期批准、批准后参数改变均拒绝。
- [ ] 秘密只通过SecretRef和本地秘密输入路径流动；序列化、错误、审计、模型上下文中均不含合成canary。
- [ ] 明确临时与持久、当前值与期望值、可逆与不可逆；用模拟后端验证apply失败/读回失败/崩溃后状态，不盲目重放。
- [ ] 产出更新后的CONFIG-CONTRACT及需要的ADR，列出每个接口风险/权限与后端认证；不能用mock成功宣称Broker或持久化完成。
- [ ] 更新受影响的用户操作说明、父级子项状态及验证记录；不得只提交内部库而无上述可运行/可测试交付。

## Validation and delivery

纯状态机、冲突、脱敏、故障注入测试与独立协议审查；无需真实密钥/系统写入。
交付证据需含精确源码版本、环境、命令/日志、通过与失败项；涉及boot必须双固件VM验证，涉及权限/秘密需独立审查。研究/模拟结果不能代替实机或生产验收。

Affected surfaces: Vita会话/设置/受控能力相关模块（按实际方案最小修改）、开发验证脚本、对应reference及用户说明。选型未定的路径不预建抽象。
Documentation: 更新本任务、父级记录和实际运行说明；新增可见操作更新README/用户手册。执行前按NEW-FEATURE与START-ITERATION路由，不自动并入I010。
Exclusions: 实现通用配置框架、真实提权、加密持久化、冻结所有OS字段。

## Readiness / evidence

范围和验证已明确，可选择进入迭代；Ready不表示已实现或已批准任何真实设备操作。
Implementation and validation: 未开始。

GitHub: https://github.com/wjhuang88/vita/issues/32
