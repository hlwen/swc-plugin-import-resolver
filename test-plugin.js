const swc = require('@swc/core');
const path = require('path');

const wasmPath = path.resolve('target/wasm32-wasip1/release/swc_plugin_import_resolver.wasm');

async function test(extension, dirIndex, skip) {
  try {
    const code = `import { foo } from "./foo.ts";
import { bar } from "@/bar";
import { baz } from "lodash";
import { iface } from "./interfaces";
import { json } from "./metadata.json";
import { css } from "./styles.css";
import { skipMe } from "./skip-me";
import { appModule } from "./app.module";
`;
    const config = { aliases: ['@/*'] };
    if (extension !== undefined) config.extension = extension;
    if (dirIndex !== undefined) config.dir_index = dirIndex;
    if (skip !== undefined) config.skip = skip;
    
    const result = await swc.transformSync(code, {
      jsc: {
        experimental: {
          plugins: [[wasmPath, config]]
        }
      }
    });
    console.log(`Result (extension=${extension || 'default'}, dirIndex=${JSON.stringify(dirIndex)}, skip=${JSON.stringify(skip)}):`);
    console.log(result.code);
  } catch (e) {
    console.error('ERROR:', e.message);
    console.error(e);
  }
}

async function main() {
  await test(undefined, undefined, undefined);  // default
  await test(undefined, ['./interfaces'], undefined);  // dirIndex enabled
  await test('.mjs', ['./interfaces'], undefined);  // dirIndex + .mjs
  await test(undefined, undefined, ['./skip-me']);  // skip enabled
  await test(undefined, undefined, ['*.json', '*.css']);  // skip by pattern
}

main();
