# Vita

Vita 是一个混合 Java + Rust 的多模块工程，目标是将 Rust 原生库与 Java 库集成。项目使用 Gradle (Kotlin DSL) 进行构建，且包含一个通过 Gradle 插件导入的 native Rust workspace（`native/libvita-sys`）。

## 项目概览

- Group: `zone.hwj`
- Version: `0.0.1`
- 根构建系统: Gradle (Kotlin DSL)
- Java 目标版本（模块 `libs:vita-lib-native`）: Java 24
- Rust workspace 位于: `native/`

模块说明
- `libs:vita-lib-native`：包含 Java API，使用 `fr.stardustenterprises.rust.importer` 插件把 Rust 产物打包到 jar 的 `META-INF/natives` 路径下。
- `native/libvita-sys`：Rust crate（在 workspace 中），负责生成原生库并导出供 Java 使用的本机接口。

## 快速开始

先决条件

- JDK（建议与项目 Java 版本兼容，JDK 17+ 通常可用，但模块使用 Java 24 特性时需安装对应 JDK）
- Gradle Wrapper（项目内提供 `./gradlew`）
- Rust 工具链（rustup、cargo），用于构建 native crate
- 如果需要构建 iOS/Android 或特定平台的原生产物，请安装相应的交叉编译工具链

常用命令

- 构建项目（使用 Gradle Wrapper）

```bash
./gradlew build
```

- 只构建 Java 模块

```bash
./gradlew :libs:vita-lib-native:build
```

- 只构建 native (Rust) crate（在 `native/` 目录）

```bash
cd native
cargo build --release
# 或针对特定 crate
cargo build -p libvita-sys --release
```

注意：仓库使用 Gradle 插件 `fr.stardustenterprises.rust.importer` / `rust.wrapper` 来桥接 Rust 构建产物到 Java 工件。如果想通过 Gradle 触发 native 构建，请使用项目的 Gradle 任务（例如 `./gradlew build` 会在需要时触发导入流程，前提是本地安装并配置好 Rust）。

## 运行测试

项目测试使用 JUnit 5（平台由 `junit-bom` 管理）。运行全部测试：

```bash
./gradlew test
```

或只运行某模块的测试：

```bash
./gradlew :libs:vita-lib-native:test
```

## 项目结构（简要）

- `build.gradle.kts` - 根构建配置（group/version）
- `settings.gradle.kts` - 包含模块定义并配置 Gradle 插件管理
- `libs/vita-lib-native/` - Java 库模块，集成 Rust 原生工件
- `native/` - Rust workspace（含 `libvita-sys`）
- `gradle.properties` - 构建属性（并启用了缓存、并行等）

## 开发者笔记与提示

- 如果遇到 native 导入失败，确保本地安装了 Rust 工具链，且 cargo 能在 PATH 中访问。
- `libs:vita-lib-native` 在 `build.gradle.kts` 中配置了：

  - `rustImport.baseDir = "/META-INF/natives"` 和 `layout = "hierarchical"`，这表示 Rust 产物会被打包到 jar 的 `META-INF/natives` 路径里，按平台/架构分层。

- 编译目标与交叉编译：若要为非本机平台构建产物（例如 Linux-x86_64、aarch64 等），请参考 Rust 官方 cross 文档或使用 `cross` 工具，并在 Gradle 导入流程中提供对应目录。

## 贡献与联系方式

欢迎 PR 与 Issue。主要维护者：Huang.Weijie <wjhuang@live.cn>

贡献时请确保：
- 保持代码风格一致，遵循模块已有的 Gradle / Rust 配置。
- 在变更 native 接口时，同时更新 Java 的 JNI / FFI 封装及对应测试。

## 许可

仓库当前未包含许可文件。如果你希望开源发布，请添加合适的 LICENSE（例如 MIT、Apache-2.0 等）。

## 后续建议

- 添加 `LICENSE` 文件
- 在 `README.md` 中补充更具体的示例：如何从 Java 直接加载 native 库、示例代码片段、以及 CI（GitHub Actions/Gradle CI）配置来在多平台构建和发布原生资产。

---

生成于自动化工具扫描项目的元信息，如需更详细内容（示例代码、CI 示例、发布流程），告诉我你希望包含的细节，我可以继续扩展。 
