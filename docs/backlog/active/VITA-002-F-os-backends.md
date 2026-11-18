# VITA-002-F — 选择首批网络、时间与服务后端

Type: Spike. Priority: P1. Status: **Ready**.
Parent: [VITA-002](VITA-002-os-interaction-settings.md). Iteration: 未选择。

## Required Reads

- 本任务及[父级范围/统一门槛](VITA-002-os-interaction-settings.md)。
- [研究证据](../../proposals/OS-INTERACTION-AND-SETTINGS-RESEARCH.md)。
- [Talos嵌入契约](../../reference/TALOS-INTEGRATION.md)、[配置契约](../../reference/CONFIG-CONTRACT.md)。
- [ADR-0002](../../decisions/ADR-0002-governed-capability-broker.md)、[ADR-0005](../../decisions/ADR-0005-rust-first-system-capabilities.md)、[系统设计](../../reference/VITA-OS-DESIGN.md)。
- [测试](../../sop/TESTING.md)、[高风险审查](../../sop/PAIRING-WORKFLOW.md)、[I010现有缺口](../../iterations/I010-vm-model-tryout.md)。

## Outcome / scope

评估现有内核/Rust接口与成熟IPC服务；比较NetworkManager/iwd等网络组合及当前监督器候选，输出适配器矩阵而非提前指定systemd。

Dependencies: 无未完成的硬前置；使用已有VM/SDK基线。
Unlocks / external gates: 解锁 G/H/I；与init选型#25协调，最终服务选型未完成时对应任务保持Refinement。

## Acceptance

- [ ] 用可丢弃VM验证有线地址/DNS查询与可恢复修改候选；提供网络恢复计时由后端持有的证据或明确缺口。
- [ ] 分别标明时区/系统时间/同步、键盘、服务查询/重启/关机的实现接口、所需服务、权限、失败语义及启动依赖。
- [ ] 记录镜像增量、RSS、启动时长、二进制/库/许可证；正常路径不新增解释器或shell命令链，例外需ADR。
- [ ] 形成有依据的后端推荐和开放决策；不以宿主机API存在替代Vita镜像验证。
- [ ] 更新受影响的用户操作说明、父级子项状态及验证记录；不得只提交内部库而无上述可运行/可测试交付。

## Validation and delivery

候选VM对比、依赖清单、接口契约与选型评审；网络修改须显式确认并限VM。
交付证据需含精确源码版本、环境、命令/日志、通过与失败项；涉及boot必须双固件VM验证，涉及权限/秘密需独立审查。研究/模拟结果不能代替实机或生产验收。

Affected surfaces: Vita会话/设置/受控能力相关模块（按实际方案最小修改）、开发验证脚本、对应reference及用户说明。选型未定的路径不预建抽象。
Documentation: 更新本任务、父级记录和实际运行说明；新增可见操作更新README/用户手册。执行前按NEW-FEATURE与START-ITERATION路由，不自动并入I010。
Exclusions: 最终全服务管理器、物理Wi-Fi支持、生产网络配置。

## Readiness / evidence

范围和验证已明确，可选择进入迭代；Ready不表示已实现或已批准任何真实设备操作。
Implementation and validation: 未开始。

GitHub: https://github.com/wjhuang88/vita/issues/35
