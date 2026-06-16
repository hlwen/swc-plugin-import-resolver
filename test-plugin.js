const swc = require('@swc/core');
const path = require('path');

const wasmPath = path.resolve('target/wasm32-wasip1/release/swc_plugin_import_resolver.wasm');

async function test(extension, dirIndex, skip, dirIndexPatterns, autoDirIndex) {
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
import { SdkService } from "./sdk.service";
import { SDK_OPTIONS } from "./constants";
`;
    const config = { aliases: ['@/*'] };
    if (extension !== undefined) config.extension = extension;
    if (dirIndex !== undefined) config.dir_index = dirIndex;
    if (skip !== undefined) config.skip = skip;
    if (dirIndexPatterns !== undefined) config.dir_index_patterns = dirIndexPatterns;
    if (autoDirIndex !== undefined) config.auto_dir_index = autoDirIndex;
    
    const result = await swc.transformSync(code, {
      jsc: {
        experimental: {
          plugins: [[wasmPath, config]]
        }
      }
    });
    console.log(`Result (extension=${extension || 'default'}, dirIndex=${JSON.stringify(dirIndex)}, skip=${JSON.stringify(skip)}, dirIndexPatterns=${JSON.stringify(dirIndexPatterns)}, autoDirIndex=${autoDirIndex}):`);
    console.log(result.code);
  } catch (e) {
    console.error('ERROR:', e.message);
    console.error(e);
  }
}

async function main() {
  await test(undefined, undefined, undefined, undefined, undefined);  // default
  await test(undefined, ['./interfaces'], undefined, undefined, undefined);  // dirIndex enabled
  await test('.mjs', ['./interfaces'], undefined, undefined, undefined);  // dirIndex + .mjs
  await test(undefined, undefined, ['./skip-me'], undefined, undefined);  // skip enabled
  await test(undefined, undefined, ['*.json', '*.css'], undefined, undefined);  // skip by pattern
  await test(undefined, undefined, undefined, ['./modules/*'], undefined);  // dir_index_patterns
  await test(undefined, undefined, undefined, undefined, true);  // auto_dir_index
}

main();
