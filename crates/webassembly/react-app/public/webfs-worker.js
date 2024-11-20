console.log("Importing WASM module in worker");
importScripts("/webfs/webfs_worker.js");
(async function () {
    console.log("Initializing WASM module in worker (start)");
    await wasm_bindgen({module_or_path: "/webfs/webfs_worker_bg.wasm"});
    //wasm_bindgen.startup();

    // self.onmessage = function ({ data }) {
    //     console.log(`data ${data}`);
    //     wasm_bindgen["fib"](4);
    //     self.postMessage(`data ${data}`);
    // };
    // Set callback to handle messages passed to the worker.
    // self.onmessage = async event => {
    //     console.log("JavaScript onmessage");
    //     // By using methods of a struct as reaction to messages passed to the
    //     // worker, we can preserve our state between messages.
    //     var worker_result = wasm_bindgen["fib"](4);
    //     console.log(`data ${event.data}`);
    //
    //     // Send response back to be handled by callback in main thread.
    //     self.postMessage(`${worker_result}`);
    // };

    //self.onmessage = async event => on_message_func(event);

    console.info("Initializing WASM module in worker (done)")
})();

async function on_message_func(event) {
    console.log("JavaScript onmessage");
    // By using methods of a struct as reaction to messages passed to the
    // worker, we can preserve our state between messages.
    var worker_result = wasm_bindgen["fib"](4);
    console.log(`data: ${event.data}`);

    // Send response back to be handled by callback in main thread.
    self.postMessage(`${worker_result}`);
}