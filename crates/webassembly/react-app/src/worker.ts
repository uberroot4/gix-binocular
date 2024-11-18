//import init, {lib_discover, read_dir} from 'commits-wasm-worker'
import {default as web_init, startup_file_worker} from "commits-wasm-web";

async function initialize() {
    await web_init();
    console.debug("Worker init successful");
}

await initialize().catch((reason: any) => {
    console.error(reason);
});

self.addEventListener('error', (error: ErrorEvent) => {
    console.error('error (worker)', error)
});

self.addEventListener('message', (e: MessageEvent) => {
    try {
        console.debug('received a message (worker)', e.data, e)

        // read_dir("web_repo/.git").then(() => {
        //     console.log("read_dir (then)")
        // }).finally(() => {
        //     console.log("read_dir (finally)")
        // }) // working

        startup_file_worker();
        self.postMessage(e.data);

        // something_async()
        //     .then(() => {
        //         console.log("something_async returned");
        //     }).finally(() => {
        //     console.log("something_async finally");
        // });

        //lib_discover("web_repo/.git")
        //     .then(() => {
        //     console.log("lib_discover (then)")
        // }).finally(() => {
        //     console.log("discover has returned (message)")
        // })

        // const data = fib(e.data);
        // console.log(`worker_init#fib = ${data}`);
        // postMessage(data);
    } catch (err) {
        console.error('error processing message (worker)', e.data, err)
    }
});

export {};