# VITA-002-J — 明确跨机器配置与秘密持久化

Type: Spike. Priority: P1. Status: **Refinement**.
Parent: [VITA-002](VITA-002-os-interaction-settings.md). Iteration: 未选择。

## Required Reads

- 本任务及[父级范围/统一门槛](VITA-002-os-interaction-settings.md)。
- [研究证据](../../proposals/OS-INTERACTION-AND-SETTINGS-RESEARCH.md)。
- [Talos嵌入契约](../../reference/TALOS-INTEGRATION.md)、[配置契约](../../reference/CONFIG-CONTRACT.md)。
- [ADR-0002](../../decisions/ADR-0002-governed-capability-broker.md)、[ADR-0005](../../decisions/ADR-0005-rust-first-system-capabilities.md)、[系统设计](../../reference/VITA-OS-DESIGN.md)。
- [测试](../../sop/TESTING.md)、[高风险审查](../../sop/PAIRING-WORKFLOW.md)、[I010现有缺口](../../iterations/I010-vm-model-tryout.md)。

## Outcome / scope

提出会话态/用户持久态/设备绑定态边界、加密解锁/清除/轮换与掉电恢复方案；只用合成秘密做原型。

Dependencies: VITA-002-C。
Unlocks / external gates: 解锁后续持久化实现与实体机试验。

## Acceptance

- [ ] 配置迁移、未知schema、空间不足、写入中断有可验证恢复路径；不把只读root改为可写。
- [ ] 区分模型偏好与设备专属网络/磁盘身份；USB换机器不重放危险配置。
- [ ] 验证锁定/解锁/异常退出/导出脱敏，并列出内存与物理访问威胁；未经人工安全批准不得称生产安全存储。
- [ ] 产出ADR候选和独立实现任务，遵守P1/P3系统/用户镜像隔离；不通过此Spike写真实U盘。
- [ ] 更新受影响的用户操作说明、父级子项状态及验证记录；不得只提交内部库而无上述可运行/可测试交付。

## Validation and delivery

临时文件/VM合成秘密与掉电故障测试、威胁模型审查。
交付证据需含精确源码版本、环境、命令/日志、通过与失败项；涉及boot必须双固件VM验证，涉及权限/秘密需独立审查。研究/模拟结果不能代替实机或生产验收。

Affected surfaces: Vita会话/设置/受控能力相关模块（按实际方案最小修改）、开发验证脚本、对应reference及用户说明。选型未定的路径不预建抽象。
Documentation: 更新本任务、父级记录和实际运行说明；新增可见操作更新README/用户手册。执行前按NEW-FEATURE与START-ITERATION路由，不自动并入I010。
Exclusions: 直接交付生产keystore、真实设备迁移。

## Readiness / evidence

任务已登记；前置子项验收及上述外部决策完成后，复核范围与测试再升Ready。
Implementation and validation: 未开始。

GitHub: https://github.com/wjhuang88/vita/issues/39
