import {StrictMode} from 'react'
import {createRoot} from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import init from 'commits-wasm'

async function initialize() {
    await init();
}

initialize().then(() => {
    console.log("Wasm init successful");
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
}).catch((reason: any) => {
    console.error(reason)
})
