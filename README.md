# swc-plugin-import-resolver

> 中文 | [English](README.en.md)

SWC 插件，用于自动解析和补全 TypeScript/JavaScript 模块导入路径的扩展名。

> 本项目基于 [cffnpwr/swc-plugin-import-extension-resolver](https://github.com/cffnpwr/swc-plugin-import-extension-resolver) 修改而来。

## 功能特性

- **自动补全扩展名**：将 `.ts` 转换为 `.js`（或 `.mjs`、`.cjs` 等）
- **路径别名支持**：支持 `@/*`、`$/*` 等别名路径的扩展名补全
- **目录索引补全**：支持将目录导入自动补全为 `index.js`
- **模块导入保护**：不会修改第三方包（如 `@nestjs/common`、`lodash`）的导入路径

## 安装

```bash
npm install --save-dev swc-plugin-import-resolver
# 或
yarn add -D swc-plugin-import-resolver
# 或
pnpm add -D swc-plugin-import-resolver
```

## 配置

### `.swcrc` 配置示例

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

### 配置选项

| 选项                 | 类型       | 默认值  | 说明                                                    |
| -------------------- | ---------- | ------- | ------------------------------------------------------- |
| `aliases`            | `string[]` | `[]`    | 需要处理的路径别名（如 `@/*`、`$/*`）                   |
| `extension`          | `string`   | `".js"` | 目标扩展名，可选 `.js`、`.mjs`、`.cjs` 等               |
| `dir_index`          | `string[]` | `[]`    | 目录导入列表（精确匹配），自动补全为 `path/index.js`    |
| `dir_index_patterns` | `string[]` | `[]`    | 目录导入模式（支持 glob），匹配则补全为 `path/index.js` |
| `auto_dir_index`     | `boolean`  | `false` | 多段无后缀相对路径自动视为目录导入                      |
| `skip`               | `string[]` | `[]`    | 跳过处理的路径模式（支持 glob）                         |
| `skip_extensions`    | `string[]` | 见下    | 已有扩展名列表，匹配则跳过                              |

## 转换示例

### 1. 基础扩展名补全

```ts
// 转换前
import { AppModule } from "./app.module";
import { Helper } from "./helper.ts";

// 转换后
import { AppModule } from "./app.module.js";
import { Helper } from "./helper.js";
```

### 2. 路径别名处理

```ts
// 转换前
import { Foo } from "@/components/foo";

// 转换后
import { Foo } from "@/components/foo.js";
```

### 3. 模块导入（不受影响）

```ts
// 转换前后保持不变
import { Injectable } from "@nestjs/common";
import lodash from "lodash";
```

### 4. 目录索引补全

```ts
// 转换前
import { Something } from "./interfaces";

// 转换后
import { Something } from "./interfaces/index.js";
```

## 高级配置

### 使用 `.mjs` 扩展名

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-import-resolver",
          {
            "extension": ".mjs"
          }
        ]
      ]
    }
  }
}
```

### 配置目录索引

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-import-resolver",
          {
            "dir_index": ["./interfaces", "./utils", "./types"]
          }
        ]
      ]
    }
  }
}
```

### 配置目录索引模式（glob）

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-import-resolver",
          {
            "dir_index_patterns": ["./modules/*", "./services/*"]
          }
        ]
      ]
    }
  }
}
```

使用 `dir_index_patterns` 可以通过 glob 模式匹配目录导入。匹配的路径会自动补全为 `path/index.js`：

```ts
// 转换前
import { AppLogger } from "./modules/logger";
import { UserService } from "./services/user";

// 转换后
import { AppLogger } from "./modules/logger/index.js";
import { UserService } from "./services/user/index.js";
```

### 自动目录索引

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-import-resolver",
          {
            "auto_dir_index": true
          }
        ]
      ]
    }
  }
}
```

开启 `auto_dir_index` 后，多段无后缀的相对路径会自动视为目录导入。规则：

- `./modules/logger` → `./modules/logger/index.js`（多段 + 无扩展名 → 目录）
- `./constants` → `./constants.js`（单段 → 文件，不受影响）
- `./sdk.service` → `./sdk.service.js`（文件名含 `.` → 文件，不受影响）

适用于使用显式 `.ts` 后缀导入的项目。

### 配置跳过规则

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-import-resolver",
          {
            "skip": ["*.json", "*.css", "*.png"]
          }
        ]
      ]
    }
  }
}
```

使用 `skip` 配置可以指定哪些路径模式应该跳过处理。支持 glob 模式：

- `*.json` — 跳过所有 `.json` 文件
- `*.css` — 跳过所有 `.css` 文件
- `./skip-me` — 跳过特定路径

### 配置跳过扩展名

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-import-resolver",
          {
            "skip_extensions": [".js", ".mjs", ".cjs", ".json", ".css", ".png"]
          }
        ]
      ]
    }
  }
}
```

使用 `skip_extensions` 配置可以指定哪些扩展名应该跳过处理。默认包含常见扩展名：

`.js` `.mjs` `.cjs` `.json` `.css` `.png` `.svg` `.jpg` `.jpeg` `.gif` `.webp` `.woff` `.woff2` `.ttf` `.eot` `.otf` `.mp3` `.mp4` `.wav` `.ogg` `.webm` `.pdf` `.zip` `.tar` `.gz` `.bz2` `.7z` `.rar` `.exe` `.dll` `.so` `.dylib` `.node` `.wasm` `.map`

## 完整 `.swcrc` 示例

```json
{
  "jsc": {
    "parser": {
      "syntax": "typescript",
      "decorators": true
    },
    "transform": {
      "legacyDecorator": true,
      "decoratorMetadata": true
    },
    "baseUrl": "./",
    "paths": {
      "@/*": ["src/*"]
    },
    "experimental": {
      "plugins": [
        [
          "swc-plugin-import-resolver",
          {
            "aliases": ["@/*"],
            "extension": ".js",
            "dir_index": ["./interfaces"],
            "skip": ["*.json", "*.css"]
          }
        ]
      ]
    }
  },
  "module": {
    "type": "es6"
  },
  "sourceMaps": true
}
```

## 开发构建

### 环境要求

- Rust
- wasm32-wasip1 目标

### 构建命令

```bash
# 构建 WASM 插件
cargo build --target wasm32-wasip1 --release

# 或使用别名
cargo build-wasi --release
```

构建产物路径：

```
target/wasm32-wasip1/release/swc_plugin_import_resolver.wasm
```

### 构建配置

`.cargo/config.toml`:

```toml
[target.wasm32-wasip1]
rustflags = ["-C", "link-arg=--allow-undefined"]
```

## 原理解释

### 如何区分本地导入和模块导入？

插件通过正则表达式 `^([./].+)` 匹配路径前缀：

- 以 `./`、`../` 或 `/` 开头的路径 → 本地导入，处理扩展名
- 其他路径（如 `@nestjs/common`、`lodash`）→ 模块导入，跳过处理

### 目录索引如何工作？

由于 SWC 插件运行在 WASM 沙箱中，**无法访问宿主文件系统**。因此 `dir_index` 采用配置化方案，用户预先声明哪些路径是目录导入，插件在编译时自动补全为 `index.js`。

## 许可证

MIT
