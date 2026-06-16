# SWC Plugin Import Resolver 开发规范

> 本文档用于指导 AI 助手和开发者进行持续开发和功能迭代。

## 项目概述

**名称**: swc-plugin-import-resolver  
**类型**: SWC WASM 插件（Rust）  
**功能**: 在 SWC 编译时自动解析和补全 TypeScript/JavaScript 模块导入路径的扩展名  
**目标平台**: `wasm32-wasip1`

### 核心能力

- 自动补全扩展名（`.ts` → `.js`/`.mjs`/`.cjs`）
- 路径别名支持（`@/*`、`$/*`）
- 目录索引补全（`./interfaces` → `./interfaces/index.js`）
- 模块导入保护（跳过第三方包）
- 跳过规则（`skip` 配置，支持 glob 模式）

---

## 技术栈

| 技术          | 版本     | 说明          |
| ------------- | -------- | ------------- |
| Rust          | stable   | 核心开发语言  |
| swc_core      | 68.0.\*  | SWC 插件框架  |
| wasm32-wasip1 | -        | WASM 目标平台 |
| serde         | 1.0.225+ | JSON 序列化   |
| globset       | 0.4.10   | glob 模式匹配 |
| regex         | 1.8.4    | 正则表达式    |

### 关键依赖约束

- `swc_core` 版本必须与 `@swc/core` 的 ABI 版本严格匹配
- 当前兼容：`swc_core 68.0.*` ↔ `@swc/core 1.15.40`

---

## 项目结构

```
.
├── src/
│   └── lib.rs              # 核心插件逻辑（唯一 Rust 源文件）
├── .cargo/
│   └── config.toml         # WASM 链接参数（--allow-undefined）
├── Cargo.toml              # Rust 依赖配置
├── package.json            # npm 包配置
├── test-plugin.js          # Node.js 集成测试脚本
├── README.md               # 中文文档
├── README.en.md            # 英文文档
└── .github/workflows/
    └── publish.yml         # GitHub Actions 自动发布
```

---

## 开发规范

### 1. 代码风格

- **语言**: Rust（目标编译为 WASM）
- **命名**: 使用 snake_case（Rust 标准）
- **注释**: 关键逻辑用中文注释，接口文档用英文
- **错误处理**: 使用 `Result` 和 `Option`，避免 unwrap/panic（测试代码除外）

### 2. 配置项设计原则

所有功能必须通过 `Config` 结构体配置化：

```rust
#[derive(Debug, Default, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
  // 新功能添加新字段时：
  // 1. 使用 #[serde(default)] 提供默认值
  // 2. 在 TransformVisitor 中添加对应字段
  // 3. 在 transform_extension 中处理新逻辑
}
```

**添加新配置项步骤**：

1. 在 `Config` 结构体中添加字段
2. 在 `TransformVisitor` 中添加对应字段
3. 在 `set_config` 中赋值
4. 在 `transform_extension` 中使用
5. 更新 `process_transform` 中的调用
6. 更新文档和测试

### 3. 功能实现原则

- **配置驱动**: 所有功能必须可配置，禁止硬编码行为
- **默认安全**: 默认值应该保持向后兼容
- **本地导入优先**: 只处理以 `./`、`../`、配置的别名开头的路径
- **模块导入跳过**: 不处理第三方包（`lodash`、`@nestjs/common` 等）
- **已有扩展名跳过**: 如果路径已有非 `.ts` 扩展名，不做处理

### 4. 测试规范

- **单元测试**: 在 `lib.rs` 底部使用 `#[cfg(test)]` 模块
- **集成测试**: 通过 `test-plugin.js` 进行 Node.js 环境测试
- **测试覆盖**: 每个新功能必须包含测试用例

**添加测试的步骤**：

1. 在 `test-plugin.js` 中添加新的测试场景
2. 运行 `node test-plugin.js` 验证
3. 在 `#[cfg(test)]` 模块中添加 Rust 单元测试
4. 运行 `cargo test` 验证

### 5. 构建规范

```bash
# 开发构建
cargo build --target wasm32-wasip1

# Release 构建（用于发布）
cargo build --target wasm32-wasip1 --release

# 运行 Rust 单元测试
cargo test

# 运行集成测试
node test-plugin.js
```

**构建配置**（`.cargo/config.toml`）：

```toml
[target.wasm32-wasip1]
rustflags = ["-C", "link-arg=--allow-undefined"]
```

### 6. 版本发布流程

1. 修改 `Cargo.toml` 中的 `version`
2. 修改 `package.json` 中的 `version`
3. 构建 Release: `cargo build --target wasm32-wasip1 --release`
4. 测试：`node test-plugin.js`
5. 提交代码：`git commit -m "chore: bump version to x.x.x"`，不要自动提交代码，代码需要人工审核
6. 打 Tag：`git tag vx.x.x`
7. 推送触发 Trusted Publishing

---

## 常见问题处理

### 编译问题

| 错误                         | 原因                     | 解决                                |
| ---------------------------- | ------------------------ | ----------------------------------- |
| `serde::__private` not found | serde 版本过低           | 升级 serde 到 1.0.185+              |
| WASM linking error           | 缺少 `--allow-undefined` | 检查 `.cargo/config.toml`           |
| `failed to invoke plugin`    | ABI 版本不匹配           | 检查 `swc_core` 与 `@swc/core` 版本 |

### 运行时问题

| 错误                               | 原因                     | 解决                         |
| ---------------------------------- | ------------------------ | ---------------------------- |
| `Cannot find module 'xxx.json.js'` | 对已有扩展名路径错误处理 | 添加已有扩展名检查逻辑       |
| `Cannot find module 'xxx'`         | 路径未被处理             | 检查路径是否匹配本地导入规则 |
| 目录导入失败                       | 未配置 `dir_index`       | 在配置中添加目录路径         |

---

## 功能开发模板

### 添加新配置项

```rust
// 1. Config 结构体
#[derive(Debug, Default, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
  aliases: Option<Vec<String>>,
  extension: String,
  dir_index: Option<Vec<String>>,
  // 新增字段
  #[serde(default)]
  new_feature: Option<String>,
}

// 2. TransformVisitor
pub struct TransformVisitor {
  aliases: Option<Vec<String>>,
  extension: String,
  dir_index: Option<Vec<String>>,
  new_feature: Option<String>,
}

// 3. set_config
pub fn set_config(..., new_feature: Option<String>) {
  // ...
  self.new_feature = new_feature;
}

// 4. transform_extension
fn transform_extension(..., new_feature: &Option<String>) -> String {
  if let Some(feature) = new_feature {
    // 实现新功能逻辑
  }
  // ...
}
```

### 添加新测试

```rust
#[cfg(test)]
mod transform_tests {
  // ...

  test!(
    Default::default(),
    |_| test_visitor_with_new_feature(),
    new_feature_test,
    r#"
    import { Foo } from "./foo";
    "#,
    r#"
    import { Foo } from "./foo.js";
    "#
  );
}
```

---

## 文档更新检查清单

- [ ] 修改 README.md 中文文档
- [ ] 修改 README.en.md 英文文档
- [ ] 更新配置选项表格
- [ ] 添加功能示例
- [ ] 更新完整 `.swcrc` 示例
- [ ] 同步更新 package.json 描述（如有必要）

---

## 联系方式

- **GitHub**: https://github.com/hlwen/swc-plugin-import-resolver
- **npm**: https://www.npmjs.com/package/swc-plugin-import-resolver
- **原作者**: https://github.com/cffnpwr/swc-plugin-import-extension-resolver
