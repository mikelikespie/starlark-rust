
import * as starlark_wasm_bindgen from "./starlark_wasm_bindgen";
import { default as  init } from "./starlark_wasm_bindgen";


// main
async function main(): Promise<void> {
    await init()
    await starlark_wasm_bindgen.do_stuff();
}

main();