# swc-plugin-import-extension-resolver

A SWC plugin to resolve import extensions.

TypeScriptのトランスパイル時にローカルの`.ts`ファイルを`.js`に変換するためのSWCプラグインです。

## Install

```sh
npm install --save-dev swc-plugin-import-extension-resolver
```

or

```sh
yarn add -D swc-plugin-import-extension-resolver
```

or

```sh
pnpm add -D swc-plugin-import-extension-resolver
```

## Usage

### .swcrc

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-import-extension-resolver",
          {
            "aliases": ["@/*", "$/*"],
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

| Option      | Type       | Default | Description                                                    |
| ----------- | ---------- | ------- | -------------------------------------------------------------- |
| `aliases`   | `string[]` | `[]`    | Path aliases to apply extension resolution (e.g. `@/*`, `$/*`) |
| `extension` | `string`   | `".js"` | Target file extension to append (e.g. `.js`, `.mjs`, `.cjs`)   |
| `dir_index` | `string[]` | `[]`    | Directory paths that should be resolved to `index.js`          |

### Options Detail

#### `aliases`

`jsc.paths`で指定したエイリアスと同じものを指定することで、エイリアスに対しても変換を行なうことができます。

```json
{
  "jsc": {
    "paths": {
      "@/*": ["src/*"]
    },
    "experimental": {
      "plugins": [
        [
          "swc-plugin-import-extension-resolver",
          {
            "aliases": ["@/*"]
          }
        ]
      ]
    }
  }
}
```

#### `extension`

変換後の拡張子を指定します。デフォルトは `.js` です。

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-import-extension-resolver",
          {
            "extension": ".mjs"
          }
        ]
      ]
    }
  }
}
```

#### `dir_index`

ディレクトリをモジュールとしてインポートする場合、`index.js` を自動で付与します。

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-import-extension-resolver",
          {
            "dir_index": ["./interfaces", "./utils"]
          }
        ]
      ]
    }
  }
}
```

Before:

```ts
import { Something } from "./interfaces";
```

After:

```ts
import { Something } from "./interfaces/index.js";
```

## Transform Examples

### Local imports

```ts
// Before
import { AppModule } from "./app.module";
import { Helper } from "./helper.ts";
import { Utils } from "../utils";

// After
import { AppModule } from "./app.module.js";
import { Helper } from "./helper.js";
import { Utils } from "../utils.js";
```

### Alias imports

```ts
// Before
import { Foo } from "@/components/foo";
import { Bar } from "$/lib/bar";

// After
import { Foo } from "@/components/foo.js";
import { Bar } from "$/lib/bar.js";
```

### Module imports (unchanged)

```ts
// Before & After
import { Injectable } from "@nestjs/common";
import lodash from "lodash";
```

### Directory imports with dir_index

```ts
// Before
import { Something } from "./interfaces";

// After
import { Something } from "./interfaces/index.js";
```

## Development

### Prerequisites

- Rust
- wasm32-wasip1 target

### Build

```bash
cargo build --target wasm32-wasip1 --release
```

The compiled plugin will be at:

```
target/wasm32-wasip1/release/swc_plugin_import_extension_resolver.wasm
```

### Configuration for WASM build

Add to `.cargo/config.toml`:

```toml
[target.wasm32-wasip1]
rustflags = ["-C", "link-arg=--allow-undefined"]
```
