#!/bin/sh

wasm-pack build --dev --out-dir ./react-app/src/pkg/web --target web ./ --features wasm_thread/es_modules

#wasm-pack build -t no-modules --out-dir ./react-app/public/pkg --profiling --no-pack ./ #--features wasm_thread/es_modules
