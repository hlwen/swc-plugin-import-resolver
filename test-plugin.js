const swc = require('@swc/core');
const path = require('path');

const wasmPath = path.resolve('target/wasm32-wasip1/release/swc_plugin_import_resolver.wasm');

async function test(extension, dirIndex, skip, dirIndexPatterns) {
  try {
    const code = `import { foo } from "./foo.ts";
import { bar } from "@/bar";
import { baz } from "lodash";
import { iface } from "./interfaces";
import { json } from "./metadata.json";
import { css } from "./styles.css";
import { skipMe } from "./skip-me";
import { appModule } from "./app.module";
import { AppLogger } from "./modules/logger";
import { Utils } from "./modules/utils";
`;
    const config = { aliases: ['@/*'] };
    if (extension !== undefined) config.extension = extension;
    if (dirIndex !== undefined) config.dir_index = dirIndex;
    if (skip !== undefined) config.skip = skip;
    if (dirIndexPatterns !== undefined) config.dir_index_patterns = dirIndexPatterns;
    
    const result = await swc.transformSync(code, {
      jsc: {
        experimental: {
          plugins: [[wasmPath, config]]
        }
      }
    });
    console.log(`Result (extension=${extension || 'default'}, dirIndex=${JSON.stringify(dirIndex)}, skip=${JSON.stringify(skip)}, dirIndexPatterns=${JSON.stringify(dirIndexPatterns)}):`);
    console.log(result.code);
  } catch (e) {
    console.error('ERROR:', e.message);
    console.error(e);
  }
}

async function main() {
  await test(undefined, undefined, undefined, undefined);  // default
  await test(undefined, ['./interfaces'], undefined, undefined);  // dirIndex enabled
  await test('.mjs', ['./interfaces'], undefined, undefined);  // dirIndex + .mjs
  await test(undefined, undefined, ['./skip-me'], undefined);  // skip enabled
  await test(undefined, undefined, ['*.json', '*.css'], undefined);  // skip by pattern
  await test(undefined, undefined, undefined, ['./modules/*']);  // dir_index_patterns
}

main();
