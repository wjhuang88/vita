# Vita — 高性能 HTTP 服务器实验

Vita 是一个以高性能 HTTP 服务为目标的多模块工程。核心目标是通过更紧密、更高效的 Java ↔ native 交互（利用 JDK 的新原生接口）在性能关键路径中使用经优化的 native 代码（目前以 Rust 为主），从而实现可扩展、高吞吐、低延迟的 HTTP 服务器实现。

本项目并不是一个通用的 Web 框架，而是围绕高性能 HTTP 服务和高效本地交互的研究与工程实现。后续计划包含对 JAX-RS 的适配层，以便提供更友好的 Java REST 风格编程模型。

## 为什么是 Vita

- 性能为先：将网络 I/O、序列化/反序列化、协议处理等性能敏感部分放到经过优化的 native 代码路径。
- 低开销交互：探索并使用 JDK 的新原生接口（例如 Project Panama / 外部函数与内存 API 等概念性的能力）来减少 JNI 带来的开销，实现更直接的内存与函数调用交互。
- 可扩展性：保留 Java 层用于路由、生命周期与扩展点，native 层用于热点路径。

## 项目概览

- Group: `zone.hwj`
- Version: `0.0.1`
- 构建: Gradle (Kotlin DSL) + Rust workspace
- Java 目标版本（`libs:vita-lib-native`）：Java 24（请根据本地环境选择合适的 JDK）
- Rust workspace 位于：`native/`（含 `libvita-sys`）

模块简述

- `libs:vita-lib-native`：Java 层的 API / 桥接逻辑，负责把 native 产物按约定嵌入到 Jar（`META-INF/natives`），并暴露 Java 侧的调用入口。
- `native/libvita-sys`：Rust crate，包含性能关键路径实现与对外导出的本机接口。

## 快速开始

先决条件

- 安装合适的 JDK（若要尝试 JDK 新原生接口功能，请使用支持相关 API 的 JDK 版本）
- 项目内包含 Gradle Wrapper：`./gradlew`
- 本地安装 Rust 工具链（rustup、cargo），用于构建 native crate

构建命令（常用）

```bash
# 全量构建（Gradle 会在需要时触发 native 导入，前提是本地配置好 Rust）
./gradlew build

# 仅构建 Java 模块
./gradlew :libs:vita-lib-native:build

# 在 native 目录直接用 cargo 构建（快速迭代 native 代码时有用）
cd native
cargo build --release
```

运行测试

```bash
./gradlew test
```

## 设计与实现要点

- Java ↔ Native 边界：优先使用低开销的调用机制、尽量避免不必要的内存复制。项目会探索 JDK 新引入的原生接口以替代传统 JNI 的部分场景。
- 本机产物管理：`libs:vita-lib-native` 使用 `fr.stardustenterprises.rust.importer` 插件将 Rust 构建产物打包到 Jar 的 `META-INF/natives` 路径，按平台/架构分层存放，运行时可动态加载对应平台的 native 库。
- 关注点：网络 I/O 调度、零拷贝缓冲区、二进制协议/HTTP 解析器、异步与阻塞模型的权衡。

## 未来计划

- 提供 JAX-RS 兼容层（可选模块），让开发者能用熟悉的注解式 API 编写 REST 接口，内部通过高效的路由和 native 实现落地。
- 增加跨平台构建与 CI：自动构建并打包适用于 Linux/macOS/Windows 的 native 二进制，便于发布。
- 性能基准与对比测试：与现有常见 Java HTTP 服务器（如 Netty、Undertow 等）在吞吐和延迟上做对比。

## 项目结构（简要）

- `build.gradle.kts` - 根构建配置
- `settings.gradle.kts` - 模块与插件声明
- `libs/vita-lib-native/` - Java API 层与 native 导入配置
- `native/` - Rust workspace（`libvita-sys`）
- `gradle.properties` - 构建属性

## 开发者提示

- 若 native 导入失败：确认 `cargo` 在 PATH 中且已安装目标工具链。
- 若要尝试 JDK 原生接口：请确保使用支持相应 API 的 JDK 版本，并在本地测试不同调用方式的延迟与开销。
- 本地开发建议：先单独在 `native/` 使用 `cargo` 编译并运行基准，然后再用 Gradle 打包到 Java 模块进行集成测试。

## 联系与贡献

欢迎 Issue 与 PR。主要维护者：Huang.Weijie <wjhuang@live.cn>

在贡献时，请尽量包含性能回归基准或说明，以便评估改动对关键路径的影响。

## 许可

本项目使用 Apache License 2.0（详见仓库根目录 `LICENSE` 文件）。

---

如果你希望我把 README 再补充：

- 添加示例代码片段（Java 侧加载 native 的样例、简单的 HTTP handler 示例）
- 提供一个 GitHub Actions CI 模板，用于多平台构建与发布 native 资产

告诉我你优先需要的扩展项，我会继续实现。 
