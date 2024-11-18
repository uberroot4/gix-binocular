import {StrictMode} from 'react'
import {createRoot} from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import {default as wasm_web} from 'commits-wasm-web'

// import wasmUrl from '../public/commits_wasm_worker.wasm?url';
// import wasmUrl from './pkg/worker/commits_wasm_worker.wasm?url';

async function initialize() {
    // await wasm_init();
    await wasm_web();

    // const singleThread = await import('./pkg/worker/commits_wasm_worker.js');
    // let x = await singleThread.default();
    // const worker = new Worker(
    //     // './pkg/worker/commits_wasm_worker.js',
    //     new URL('./pkg/worker/commits_wasm_worker.js', import.meta.url),
    //     {
    //         type: 'module'
    //     }
    // );
    // worker.postMessage("lol");

    // WebAssembly.compileStreaming(fetch(wasmUrl)).then((r) => {
    //     console.log(r)
    //     new Worker();
    // }).catch((err) => {
    //     console.error(err);
    // }).finally(() => {
    //     console.log("compileStreaming done")
    // });

    // gix_init().then((inst) => {
    //     console.log(inst);
    // });
    // return WebAssembly.compileStreaming(fetch(gixDiscoverUrl));
    // await wasm_init_wb();
    // await wasip2_init();
    // test()
    // console.log(import.meta.url)
    // // const responsePromise = fetch(new URL("commits_wasm_worker.wasm?url", import.meta.url))
    // const responsePromise = fetch(wasmUrl)
    // const { module, instance } =
    //     await WebAssembly.instantiateStreaming(responsePromise)
    // console.log(instance);
}

// wasi_init().then((instance) => {
//     console.log("wasi instance")
//     console.log(instance);
//     // instance.test();
//     test();
// }).catch(err => {
//     console.error(err);
// })

initialize().then(() => {
    console.log("Wasm init successful");
    console.log(import.meta.url);
    // console.log(module);
    // let wasi = new WASI
    // runWasix(module).then((instance) => {
    //     console.log(instance);
    // });

    // readFile(gixDiscoverUrl).then((bytes) => {
    //     WebAssembly.compile(bytes).then((lib) => {
    //         console.log(lib);
    //     })
    // });
    // Wasmer.fromFile("./gix_discover.wasm").then((instance) => {
    //     console.log(instance)
    // });

    // const tsWorker = new ViteWorker();
    // const worker = new Worker(new URL('./worker.ts', import.meta.url), {
    //     type: "module"
    // });
    // worker.onmessage = (event: MessageEvent) => {
    //     console.debug(event)
    // }
    // tsWorker.postMessage("test");
    // console.log(tsWorker)

    const isFileSystemAccessSupported = (): boolean => {
        return "showOpenFilePicker" in window && "showSaveFilePicker" in window;
    };

    if (!isFileSystemAccessSupported()) {
        console.error("File System Access API is not supported in your browser.");
        //return;
    }

    createRoot(document.getElementById('root')!).render(
        <StrictMode>
            <App/>
        </StrictMode>,
    )
});
// .catch((reason: any) => {
//     console.error(reason)
// })

// readFile fetches the file and returns its contents as an Uint8Array.
// async function readFile(url) {
//     const response = await fetch(url);
//     if (!response.ok) {
//         throw new Error(`${url}: http status ${response.status}`);
//     }
//     const buffer = await response.arrayBuffer();
//     return new Uint8Array(buffer);
// }