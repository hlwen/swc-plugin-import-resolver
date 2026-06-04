const swc = require('@swc/core');
const path = require('path');

const wasmPath = path.resolve('target/wasm32-wasip1/release/swc_plugin_import_extension_resolver.wasm');

async function test(extension, dirIndex) {
  try {
    const code = `import { foo } from "./foo.ts";
import { bar } from "@/bar";
import { baz } from "lodash";
import { iface } from "./interfaces";
`;
    const config = { aliases: ['@/*'] };
    if (extension !== undefined) config.extension = extension;
    if (dirIndex !== undefined) config.dir_index = dirIndex;
    
    const result = await swc.transformSync(code, {
      jsc: {
        experimental: {
          plugins: [[wasmPath, config]]
        }
      }
    });
    console.log(`Result (extension=${extension || 'default'}, dirIndex=${JSON.stringify(dirIndex)}):`);
    console.log(result.code);
  } catch (e) {
    console.error('ERROR:', e.message);
    console.error(e);
  }
}

async function main() {
  await test(undefined, undefined);  // default
  await test(undefined, ['./interfaces']);  // dirIndex enabled
  await test('.mjs', ['./interfaces']);  // dirIndex + .mjs
}

main();
