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
import {something_async} from "commits-wasm-web";
import {useState} from "react";
import ProducerConsumer from "./ProducerConsumer.tsx";


// const channel = ;

function App() {
    const [count, setCount] = useState(11)
    const [channel, _] = useState(new BroadcastChannel("a-b-c"))
    // const [fileContent, setFileContent] = useState<string>("");
    // const [fileHandle, setFileHandle] = useState<FileSystemFileHandle | null>(null);
    // const [dirHandle, setDirHandle] = useState<WebDir | null>(null);
    // const [tsWorker, _] = useState(new ViteWorker());

    channel.addEventListener("message", (ev: MessageEvent) => {
        console.trace("Received message on port1");
        console.debug(ev);
    });
    channel.addEventListener("messageerror", (ev: MessageEvent) => {
        console.trace("Received messageerror on port1");
        console.error(ev);
    })


    const startWorker = () => {
        // if (!tsWorker) {
        //     var channel = new MessageChannel();
        //     tsWorker = spawn_worker(startup_file_worker());
        //     //tsWorker = new Worker("/webfs-worker.js");
        //     console.log("New Web Worker spawned");
        // } else {
        //     console.info("Worker already running");
        // }
    }
    const terminate = () => {
        // if (tsWorker) {
        //     tsWorker.terminate();
        //     tsWorker = null;
        //     console.info("Worker terminated");
        // } else {
        //     console.warn("Cannot terminate worker (not running)");
        // }
    }
    const runWorker = async () => {
        // console.trace(`runWorker ${count}`);
        // try {
        //     if (tsWorker)
        //         tsWorker.postMessage(count);
        //     setCount(count + 1);
        // } catch (err) {
        //     console.error("Error clearing directory:", err);
        // } finally {
        //     console.log(`counter value = ${count}`)
        // }
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
            something_async(channel)
                .then((e) => {
                    console.log(`something_async returned ${e}`);
                })
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
