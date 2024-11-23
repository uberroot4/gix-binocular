#![feature(async_closure)]

use wasm_bindgen::prelude::*;
mod worker;
use async_channel::RecvError;
use futures_lite::stream::StreamExt;
use log::{info, trace, Level};
use std::path::PathBuf;
use std::time::Duration;
use wasm_bindgen_futures::JsFuture;
use web_sys::MessageEvent;

pub mod file_worker;


#[wasm_bindgen]
pub fn fib(n: i32) -> u64 {
    info!("received n = {}", n);
    worker::fib(n)
}

#[cfg(feature = "standalone")]
#[wasm_bindgen(start)]
fn start() {
    trace!(">> Running Web Worker (trace)");
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Trace).unwrap();
    trace!("<< Running Web Worker (trace)");
}


//#[wasm_bindgen]
pub async fn read_dir(path: String) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut dir = web_fs::read_dir(path).await.unwrap();
    let dwgs = web_sys::js_sys::global()
        .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
        .expect("worker global scope is not available");

    let root_dir = JsFuture::from(dwgs.navigator().storage().get_directory()).await;
    info!("root_dir {:?}", root_dir);

    let output = "fs:\n".to_owned();
    info!("{}", output);
    let level: usize = 0;
    while let Some(entry) = dir.next().await {
        let entry = entry.unwrap();
        let msg = format!("{}{:?}", " ".repeat(level * 4), entry);
        dwgs.post_message(&msg.clone().into()).expect("TODO: panic message");
        info!("{}", msg);
    }
    info!("spawn_local end");

    info!("end of test");
}

//#[wasm_bindgen]
pub fn lib_discover(path: String) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    info!("discovering @ {}", path);
    trace!("discovering @ {}", path);
    let path_buf = PathBuf::from(path);
    info!("path_buf @ {:?}", path_buf);

    // let (thread_tx, main_rx) = async_channel::unbounded::<String>();

    // let x = wasm_thread::scope(|s| {
    //     let thread_tx = thread_tx.clone();
    //     s.spawn(move || async {
    //         info!("(1) {:?}", wasm_thread::current().id());
    //         async move || {
    //             // async_std::task::sleep(Duration::from_millis(1500)).await;
    //             thread_tx.send("hello from thread_tx".parse().unwrap()).await.expect("thread_tx.send failed");
    //             // for _ in 0..2 {
    //             //     info!("loop");
    //             //     wasm_thread::spawn(|| {
    //             //         for i in 1..5 {
    //             //             futures::executor::block_on(async move {
    //             //                 info!("hi number {} from the spawned thread {:?}!", i, wasm_thread::current().id());
    //             //                 wasm_thread::sleep(Duration::from_millis(1500))
    //             //             });
    //             //         }
    //             //     });
    //             // }
    //             "asdf"
    //         }
    //     })
    // });
    // // let xval = futures::executor::block_on(x);
    // // info!("xval {}", xval);
    // // wasm_thread::sleep(Duration::from_millis(1500));
    //
    // info!("pre recv()");
    // let msg = futures::executor::block_on(main_rx.recv());
    // // while let Ok(msg) = main_rx.try_recv() {
    // info!("received message: {:?}", msg);
    // match msg {
    //     Ok(_) => {}
    //     Err(e) => {
    //         info!("{}", e);
    //     }
    // }
    // }

    // wasm_thread::spawn(|| {
    //     trace!("{:?}", wasm_thread::current().id());
    // }).join_async().await;

    // let _ = wasm_thread::spawn(|| {
    //     // console_log::init().unwrap();
    //     log::info!("after spawn");
    //     // wasm_thread::scope(|s| {
    //     //     futures::executor::block_on(async {
    //     //         info!("(1) hi number from the spawned thread {:?}!", wasm_thread::current().id());
    //     //         // worker::discover(path_buf).await;
    //     //         web_fs::init_logging();
    //     //         // let res = web_fs::metadata(path_buf).await;
    //     //         // info!("metadata {:?}", res);
    //     //         info!("(2) hi number from the spawned thread {:?}!", wasm_thread::current().id());
    //     //     });
    //     // });
    // }).join();

    // for _ in 0..2 {
    //     info!("loop");
    //     wasm_thread::spawn(|| {
    //         for i in 1..5 {
    //             futures::executor::block_on(async move {
    //                 info!("hi number {} from the spawned thread {:?}!", i, wasm_thread::current().id());
    //                 wasm_thread::sleep(Duration::from_millis(1500))
    //             });
    //         }
    //     });
    // }

    info!("end of discovering");
}