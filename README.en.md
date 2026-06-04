# swc-plugin-import-resolver

> [中文](README.md) | English

An SWC plugin to automatically resolve and append extensions for TypeScript/JavaScript module import paths.

> This project is based on [cffnpwr/swc-plugin-import-extension-resolver](https://github.com/cffnpwr/swc-plugin-import-extension-resolver).

## Features

- **Automatic Extension Resolution**: Converts `.ts` to `.js` (or `.mjs`, `.cjs`, etc.)
- **Path Alias Support**: Supports aliases like `@/*`, `$/*` for extension resolution
- **Directory Index Resolution**: Automatically resolves directory imports to `index.js`
- **Module Import Protection**: Does not modify third-party package imports (e.g., `@nestjs/common`, `lodash`)

## Installation

```bash
npm install --save-dev swc-plugin-import-resolver
# or
yarn add -D swc-plugin-import-resolver
# or
pnpm add -D swc-plugin-import-resolver
```

## Configuration

### `.swcrc` Example

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

### Options

| Option      | Type       | Default | Description                                          |
| ----------- | ---------- | ------- | ---------------------------------------------------- |
| `aliases`   | `string[]` | `[]`    | Path aliases to resolve (e.g., `@/*`, `$/*`)       |
| `extension` | `string`   | `".js"` | Target extension, e.g., `.js`, `.mjs`, `.cjs`        |
| `dir_index` | `string[]` | `[]`    | Directory imports to auto-resolve as `path/index.js` |

## Examples

### 1. Basic Extension Resolution

```ts
// Before
import { AppModule } from "./app.module";
import { Helper } from "./helper.ts";

// After
import { AppModule } from "./app.module.js";
import { Helper } from "./helper.js";
```

### 2. Path Alias Handling

```ts
// Before
import { Foo } from "@/components/foo";

// After
import { Foo } from "@/components/foo.js";
```

### 3. Module Imports (Unchanged)

```ts
// Unchanged before and after transformation
import { Injectable } from "@nestjs/common";
import lodash from "lodash";
```

### 4. Directory Index Resolution

```ts
// Before
import { Something } from "./interfaces";

// After
import { Something } from "./interfaces/index.js";
```

## Advanced Configuration

### Using `.mjs` Extension

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

### Configuring Directory Index

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

## Complete `.swcrc` Example

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
            "dir_index": ["./interfaces"]
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

## Development Build

### Requirements

- Rust
- wasm32-wasip1 target

### Build Commands

```bash
# Build WASM plugin
cargo build --target wasm32-wasip1 --release

# Or use alias
cargo build-wasi --release
```

Build output path:

```
target/wasm32-wasip1/release/swc_plugin_import_resolver.wasm
```

### Build Configuration

`.cargo/config.toml`:

```toml
[target.wasm32-wasip1]
rustflags = ["-C", "link-arg=--allow-undefined"]
```

## How It Works

### How Does It Distinguish Between Local and Module Imports?

The plugin uses the regular expression `^([./].+)` to match path prefixes:

- Paths starting with `./`, `../`, or `/` → local imports, extension is resolved
- Other paths (e.g., `@nestjs/common`, `lodash`) → module imports, skipped

### How Does Directory Index Resolution Work?

Since SWC plugins run in a WASM sandbox, **they cannot access the host file system**. Therefore, `dir_index` uses a configuration-based approach: users declare which paths are directory imports in advance, and the plugin automatically resolves them to `index.js` during compilation.

## License

MIT
