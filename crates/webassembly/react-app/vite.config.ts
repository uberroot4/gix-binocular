import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
    server: {
        cors: false,
        headers: {
            'Cross-Origin-Opener-Policy': 'same-origin',
            'Cross-Origin-Embedder-Policy': 'require-corp',
        }
    },
    preview: {
        cors: false
    },
    plugins: [
        wasm(),
        topLevelAwait(),
        react(),
    ],
    worker: {
        format: "es",
        plugins: () => [
            wasm(),
            topLevelAwait()
        ]
    },
    optimizeDeps: {
        exclude: [
            "gix_discover.wasm"
        ]
    }
})
