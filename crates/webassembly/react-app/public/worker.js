console.log("Importing WASM module in worker");
importScripts("/pkg/commits_wasm.js");
(async function () {
    console.log("Initializing WASM module in worker");
    await wasm_bindgen({ module_or_path: "/pkg/commits_wasm_bg.wasm" });
    await wasm_bindgen.init_wasm_module();

    self.onmessage = function ({ data }) {
        console.log(`data in js ${data}`);
        //wasm_bindgen["something_async"]();
    };
})();