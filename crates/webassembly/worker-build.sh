#!/bin/sh

#wasm-pack build --dev --out-dir ../../react-app/src/pkg/worker --target web ./crates/worker --features wasm_thread/es_modules

wasm-pack build -t no-modules --out-dir ../../react-app/public/webfs --profiling --no-pack ./crates/worker