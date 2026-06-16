# 告别手动改 import：发布 swc-plugin-import-resolver，一键解决 TypeScript ESM 扩展名难题

> 项目地址：https://github.com/hlwen/swc-plugin-import-resolver

如果你在用 TypeScript + ESM 开发，一定遇到过这个烦人的问题：

```typescript
// 源码
import { foo } from "./foo";

// 编译后 ❌ 报错
import { foo } from "./foo"; // Error: Cannot find module

// 必须手动改成
import { foo } from "./foo.js"; // ✅
```

在 ESM 规范中，**导入路径必须包含完整的文件扩展名**（`.js`、`.mjs` 等），这与 CommonJS 的灵活解析完全不同。对于大型项目来说，手动维护这些扩展名既繁琐又容易出错。

今天，我正式开源 **`swc-plugin-import-resolver`** —— 一个基于 SWC 的编译时插件，帮你自动解决这个问题。

---

## 为什么需要这个插件？

### 背景：Node.js ESM 的"坑"

2022 年 Node.js 全面支持 ESM 后，越来越多的项目开始从 CommonJS 迁移到 ESM。但 ESM 有一个严格的要求：

| CommonJS                        | ESM                                      |
| ------------------------------- | ---------------------------------------- |
| `require('./foo')`              | `import { foo } from './foo.js'`         |
| `require('./utils')`            | `import { ... } from './utils/index.js'` |
| 自动解析 `.js`、目录 `index.js` | **必须写完整路径**                       |

TypeScript 编译器（`tsc`）本身不会处理这个问题，它直接保留原始导入路径。这就导致编译后的代码在运行时直接报错。

### 现有方案的局限

- **手动改代码**：痛苦，容易遗漏，维护成本高
- **构建工具处理**（Vite/Webpack）：增加了构建复杂度，且不是所有项目都想用打包工具
- **tsc-alias 等后处理工具**：需要额外的构建步骤，不够优雅

### 最佳方案：编译时自动处理

SWC 是 Rust 编写的高性能 JavaScript/TypeScript 编译器，比 tsc 快 10~20 倍。如果在 SWC 编译阶段就能自动补全扩展名，既保持了源码的简洁，又不需要额外的构建步骤。

这就是 `swc-plugin-import-resolver` 的设计初衷。

---

## swc-plugin-import-resolver 是什么？

这是一个 **SWC 插件**，在编译时自动处理 import 路径的扩展名补全。

### 核心能力

| 功能               | 说明                                                |
| ------------------ | --------------------------------------------------- |
| **自动扩展名补全** | `./foo` → `./foo.js`，`./helper.ts` → `./helper.js` |
| **路径别名支持**   | `@/components/foo` → `@/components/foo.js`          |
| **目录索引补全**   | `./interfaces` → `./interfaces/index.js`            |
| **模块导入保护**   | `@nestjs/common`、`lodash` 等第三方包不受影响       |
| **自定义扩展名**   | 支持 `.js`、`.mjs`、`.cjs` 等                       |

### 使用示例

```typescript
// 源码（.ts）
import { AppModule } from "./app.module";
import { Helper } from "./helper.ts";
import { Foo } from "@/components/foo";
import { Something } from "./interfaces";
import { Injectable } from "@nestjs/common";

// 编译后（SWC 自动转换）
import { AppModule } from "./app.module.js";
import { Helper } from "./helper.js";
import { Foo } from "@/components/foo.js";
import { Something } from "./interfaces/index.js";
import { Injectable } from "@nestjs/common"; // 第三方包不变
```

---

## 快速开始

### 安装

```bash
npm install --save-dev swc-plugin-import-resolver
```

### 配置 `.swcrc`

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-import-resolver",
          {
            "aliases": ["@/*"],
            "extension": ".js",
            "dir_index": ["./interfaces", "./utils"]
          }
        ]
      ]
    }
  }
}
```

配置说明：

| 选项        | 类型       | 默认值  | 说明                                |
| ----------- | ---------- | ------- | ----------------------------------- |
| `aliases`   | `string[]` | `[]`    | 路径别名规则，如 `@/*`、`$/*`       |
| `extension` | `string`   | `".js"` | 目标扩展名，可选 `.mjs`、`.cjs`     |
| `dir_index` | `string[]` | `[]`    | 目录导入列表，自动补全为 `index.js` |

---

## 技术实现亮点

### 1. 基于 Rust + WASM

插件使用 Rust 编写，编译为 WASM 模块运行在 SWC 中：

- **零运行时开销**：编译时处理，不增加构建产物体积
- **跨平台**：WASM 格式，Windows/macOS/Linux 通用
- **安全隔离**：运行在沙箱中，不会影响宿主进程

### 2. 智能路径识别

通过正则表达式 `^([./].+)` 精确区分本地导入和模块导入：

```
./foo.ts      → 本地导入 → 处理 ✅
../bar.ts     → 本地导入 → 处理 ✅
@/baz.ts      → 本地导入 → 处理 ✅
@nestjs/core   → 模块导入 → 跳过 ❌
lodash        → 模块导入 → 跳过 ❌
```

### 3. 完全可配置

所有功能都通过 JSON 配置控制，无需修改源码：

```json
{
  "extension": ".mjs",
  "aliases": ["@/*", "#/*"],
  "dir_index": ["./types", "./utils"]
}
```

---

## 对比其他方案

| 方案       | 性能           | 额外步骤         | 路径别名 | 目录索引 |
| ---------- | -------------- | ---------------- | -------- | -------- |
| 手动修改   | ⭐             | 无               | 支持     | 支持     |
| tsc-alias  | ⭐⭐           | 需要后处理       | 支持     | 需配置   |
| Vite 插件  | ⭐⭐⭐         | 依赖 Vite        | 支持     | 需配置   |
| **本插件** | **⭐⭐⭐⭐⭐** | **无（编译时）** | **支持** | **支持** |

---

## 适用场景

- **Node.js + ESM 项目**：需要 `.js` 扩展名
- **Monorepo**：大量使用路径别名（`@/`）
- **纯 TypeScript 库**：不使用打包工具，直接 tsc/swc 编译输出
- **NestJS 项目**：大量目录导入（`./interfaces`、`./dto`）

---

## 开源与贡献

项目完全开源，基于 MIT 协议。

- **GitHub**: https://github.com/hlwen/swc-plugin-import-resolver
- **npm**: https://www.npmjs.com/package/swc-plugin-import-resolver

本项目基于 [cffnpwr/swc-plugin-import-extension-resolver](https://github.com/cffnpwr/swc-plugin-import-extension-resolver) 修改而来，感谢原作者的出色工作！

如果你在使用过程中遇到问题，欢迎提交 Issue 或 PR。也欢迎 Star ⭐ 支持一下！

---

## 总结

TypeScript + ESM 是未来的趋势，但模块解析的细节往往让人头疼。`swc-plugin-import-resolver` 通过编译时自动处理，让你专注于业务代码，不用关心扩展名细节。

如果你正在使用 SWC 编译 TypeScript 项目，强烈推荐试试这个插件，相信能帮你省不少事。

```bash
npm install --save-dev swc-plugin-import-resolver
```

**让 import 路径回归简洁，让扩展名补全自动完成。**
