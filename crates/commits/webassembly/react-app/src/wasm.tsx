import init from "./wasm/commits_wasm";
import * as wasm from "./wasm/commits_wasm";
import wasmData from "./wasm/commits_wasm_bg.wasm";

console.log(wasm);
await init(wasmData);
console.log("Wasm init successful");

export default wasm;