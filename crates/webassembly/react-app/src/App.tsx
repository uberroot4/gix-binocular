// import {useState} from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
// import {
//     // get_location,
//     // WebDir,
//     exec,
//     // clear_dir
// } from 'commits-wasm'
import {something_async, startup_file_worker} from "commits-wasm-web";
import {useState} from "react";
import ProducerConsumer from "./ProducerConsumer.tsx";
//import ViteWorker from "./worker?worker";
//import FileWorker from "./file-worker?worker"
// import ViteWorker from "./pkg/worker/commits_wasm_worker.js?worker"

// let tsWorker: Worker | null = new ViteWorker();
let tsWorker: Worker | null = null;

function spawn_worker(worker: Worker): Worker {
    worker.addEventListener('message', (e: MessageEvent) => {
        try {
            console.debug(`received a message (main) ${e.data}`, e)
        } catch (err) {
            console.error('error processing message (main)', e.data, err)
        }
    });
    return worker
}

function App() {
    const [count, setCount] = useState(11)
    // const [fileContent, setFileContent] = useState<string>("");
    // const [fileHandle, setFileHandle] = useState<FileSystemFileHandle | null>(null);
    // const [dirHandle, setDirHandle] = useState<WebDir | null>(null);
    // const [tsWorker, _] = useState(new ViteWorker());


    // Open File
    const clearDir = async () => {
        try {
            // await clear_dir()
        } catch (err) {
            console.error("Error clearing directory:", err);
        }
    };


    const startWorker = () => {
        if (!tsWorker) {
            var channel = new MessageChannel();
            tsWorker = spawn_worker(startup_file_worker());
            //tsWorker = new Worker("/webfs-worker.js");
            console.log("New Web Worker spawned");
        } else {
            console.info("Worker already running");
        }
    }
    const terminate = () => {
        if (tsWorker) {
            tsWorker.terminate();
            tsWorker = null;
            console.info("Worker terminated");
        } else {
            console.warn("Cannot terminate worker (not running)");
        }
    }
    const runWorker = async () => {
        console.trace(`runWorker ${count}`);
        try {
            if (tsWorker)
                tsWorker.postMessage(count);
            setCount(count + 1);
        } catch (err) {
            console.error("Error clearing directory:", err);
        } finally {
            console.log(`counter value = ${count}`)
        }
    };

    const openDir = async () => {
        try {
            // await exec();
        } catch (err) {
            console.error("Error opening directory:", err);
        }
    };

    const something = async () => {
        try {
            something_async()
                // .then((e) => {
                //     console.log(`something_async returned ${e}`);
                // })
        } catch (err) {
            console.error("Error opening directory:", err);
        } finally {
            console.log("finally of something")
        }
    };

    return (
        <>
            <div>
                <a href="https://vite.dev" target="_blank">
                    <img src={viteLogo} className="logo" alt="Vite logo"/>
                </a>
                <a href="https://react.dev" target="_blank">
                    <img src={reactLogo} className="logo react" alt="React logo"/>
                </a>
            </div>
            <h1>Vite + React</h1>
            <div className="card">
                {/*<button onClick={clearDir}>*/}
                {/*    Delete OPFS*/}
                {/*</button>*/}
                <button onClick={startWorker}>Start</button>
                <button id="startupBtn" onClick={runWorker}>Run Worker</button>
                <button onClick={terminate}>Terminate</button>
            </div>
            <div className="card">
                <button onClick={openDir}>Open Dir</button>
                <button onClick={something}>Something Async</button>
                <p>
                    Edit <code>src/App.tsx</code> and save to test HMR
                </p>
            </div>
            <div className="card">
                <ProducerConsumer/>
            </div>
        </>
    )
}

export default App
