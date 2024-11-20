#![feature(async_closure)]

extern crate console_error_panic_hook;

use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use wasm_bindgen::prelude::*;
use opfs::{ThreadSafeFile, Action, start_webfs_consumer};
use shared::{info, trace, warn, debug, error};
use wasm_thread as thread;

mod utils;
mod external;


const WEB_REPO_ROOT: &'static str = "web_repo/.git";

#[wasm_bindgen(start)]
async fn run() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).unwrap();
    web_fs::init_logging();
    info!("run");
    web_fs::create_dir_all(WEB_REPO_ROOT).await.expect("Error creating WEB_REPO_ROOT");
}

#[wasm_bindgen]
pub fn startup_file_worker() {
    trace!(">> startup_file_worker()");
    trace!("<< startup_file_worker()");
}

#[wasm_bindgen]
pub async fn exec() -> Result<JsValue, JsError> {
    // console_error_panic_hook::set_once();

    // let opfs_root = navigator::get_root_storage().await;
    // console_log!("opfsRoot: {:?}", opfs_root);
    // let options: FileSystemGetDirectoryOptions = FileSystemGetDirectoryOptions::new();
    // options.set_create(true);

    // opfs_root.create_dir("web_repo").expect("Should not fail");
    //
    // fn main(fs: WebFS) -> no_std_io::io::Result<()> {
    //     for entry in fs.read_dir(".")? {
    //         let dir = entry?;
    //         console_log!("{:?}", dir.path());
    //     }
    //     Ok(())
    // }
    // main(opfs_root).expect("TODO: panic message");


    // let root_dir = JsFuture::from(opfs_root.get_directory_handle_with_options(WEB_REPO_ROOT, &options)).await
    //     .unwrap()
    //     .dyn_into::<web_sys::FileSystemDirectoryHandle>()
    //     .unwrap();

    // let root_web_dir = WebDir::from(root_dir.clone());
    //
    // let f_options: FileSystemGetFileOptions = FileSystemGetFileOptions::new();
    // f_options.set_create(true);

    // let a = wasm_bindgen_futures::spawn_local(async move {
    //     match opfs::choose().await {
    //         Ok(opfs_dir) => {
    //             match opfs_dir.get_dir_handle(".git").await {
    //                 Ok(git_native_path) => {
    //                     console_log!("git_path: {:?}", opfs_dir);
    //                     // let _ = opfs::read_dir(git_native_path, Some(RecursionType::DepthFirst)).await;
    //                     let _ = opfs::load_into_browser(WEB_REPO_ROOT.to_string(), git_native_path).await;
    //
    //                     let mut paths = web_fs::read_dir(WEB_REPO_ROOT).await?;
    //
    //
    //                     while let Some(path) = paths.next().await {
    //                         // extern crate std;
    //                         // use std::ffi::OsStr;
    //                         // let osstr = OsStr::new("path.unwrap()");
    //                         // console_log!("osstr: {:?}", osstr);
    //                         console_log!("Name: {}", path?.path().display());
    //                     }
    //                     console_log!("++++++++++++++++++++++++++++++++++++++");
    //
    //                     // {
    //                     //     use gix_fs;
    //                     //     for entry in gix_fs::read_dir(WEB_REPO_ROOT.as_ref(), false).await? {
    //                     //         let entry = entry?;
    //                     //         console_log!("Name: {:?}", entry);
    //                     //     }
    //                     // }
    //
    //                     // {
    //                     //     use gix_path;
    //                     //     extern crate std;
    //                     //     let cwd = gix_fs::current_dir(false).map(alloc::borrow::Cow::<std::path::PathBuf>::Owned);
    //                     //     console_log!("current_dir {:?}" , cwd);
    //                     //     let normalized = gix_path::normalize(alloc::borrow::Cow::from(<str as AsRef<std::path::Path>>::as_ref(WEB_REPO_ROOT)), &**cwd?);
    //                     //     console_log!("normalized {:?}" , normalized);
    //                     // }
    //
    //                     // {
    //                     //     use gix;
    //                     //     let repo = gix::open((*WEB_REPO_ROOT).parse())?;
    //                     //     console_log!("repo {:?}" , repo);
    //                     // }
    //                 }
    //                 Err(err) => {
    //                     console_err!("Failed to get .git: {:?}", serde_wasm_bindgen::Error::from(err));
    //                     // Err(JsError::new(move || { err }))
    //                 }
    //             }
    //         }
    //         Err(err) => {
    //             console_err!("Failed to open directory: {:?}", serde_wasm_bindgen::Error::from(err));
    //         }
    //     };
    //     // });
    //
    Ok(JsValue::null())
}

#[wasm_bindgen(js_name = "sendAction")]
pub fn send_action(action: Action) {
    opfs::send_action(action);
}

#[wasm_bindgen]
pub async fn something_async() {
    use futures_channel::oneshot;
    let (tx, rx) = oneshot::channel();

    // use gix_fs;
    let _ = wasm_thread::spawn(|| {
        // wasm_bindgen_futures::spawn_local(async {
        //     let read_dir = gix_fs::read_dir(WEB_REPO_ROOT.as_ref(), false).unwrap();

        /// opfs::read_dir EXAMPLE
        // let read_dir = opfs::read_dir::<&Path>(WEB_REPO_ROOT.as_ref()).unwrap();
        // for d in read_dir {
        //     info!("d = {:?}", d)
        // }

        /// File Example
        let file = crate::ThreadSafeFile::open("web_repo/.git/HEAD").unwrap();
        info!("web_fs::File::open_sync: {:?}", file);

        /// METADATA Example
        // let metadata = opfs::metadata::<&Path>("web_repo/.git/HEAD".as_ref()).unwrap();
        // info!("metadata {:?}", metadata);

        drop(tx.send(std::io::Result::Ok("file")));
        // web_sys::js_sys::eval("self")
        //     .unwrap()
        //     .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
        //     .unwrap()
        //     .close();
        // });
    }).join_async().await;


    // use gix_discover;
    // info!("pre thread spawned");
    // let _gix_discover_thread_handle = wasm_thread::spawn(|| {
    //     info!("thread spawned");
    //     let is_git = gix_discover::is_git(WEB_REPO_ROOT.as_ref());
    //     drop(tx.send(is_git));
    //     web_sys::js_sys::eval("self")
    //         .unwrap()
    //         .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
    //         .unwrap()
    //         .close();
    // }).join_async();

    // trace!("Within done_handle");
    let is_git = match rx.await.unwrap() {
        Ok(_data) => {
            info!("_data: {:?}", _data);
            // let mut output = Vec::with_capacity(4000);
            // let content = web_fs::File::open("/web_repo/.git/HEAD").await.unwrap().read_to_end(&mut output).await.unwrap();

            // info!("web_fs::File::content: {:?}", content );
            // info!("web_fs::File::output: {:?}", String::from_utf8(output) );
            Ok(JsValue::from_str("If you see this, everything is OK"))
        }
        Err(e) => {
            log::error!("{}", e);
            Err(JsError::new(&e.to_string()))
        }
    };

    info!("is_git {:?}" , is_git);
}
// pub fn something_async_old() {
//     use wasm_thread;
//
//     info!("something_async");
//     info!("(0) {}", thread_id());
//
//     let (thread_tx, main_rx) = async_channel::unbounded::<String>();
//     async_std::task::block_on(async move {
//         //info!("(0.1) {}", thread_id());
//         let join_handle = wasm_thread::spawn_async({
//             //info!("(1) {}", thread_id());
//             let thread_tx = thread_tx.clone();
//             async move || {
//                 //info!("(2) {}", thread_id());
//                 let storage = match window() {
//                     None => {
//                         info!("window not found, using navigator");
//                         wasm_bindgen_futures::js_sys::global()
//                             .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
//                             .expect("worker global scope is not available")
//                             .navigator()
//                             .storage()
//                     }
//                     Some(window) => {
//                         trace!("window found");
//                         window.navigator().storage()
//                     }
//                 };
//                 trace!("storage = {:?}", storage);
//                 let promise = storage.estimate().unwrap();
//                 let future = JsFuture::from(promise);
//                 let estimate = future.await.unwrap();
//                 //thread_tx.send(format!("estimate {:?}", estimate)).await.expect("thread_tx.send failed");
//
//
//                 //thread_tx.send(format!("is_git {:?}", is_git)).await.expect("thread_tx.send failed (2)");
//
//                 info!("end of spawn");
//             }
//         }).join_async();
//         return join_handle;
//     });
//
//     info!("pre recv()");
//     let msg = async_std::task::block_on(async move {
//         let main_rx = main_rx.clone();
//         let msg = main_rx.recv().await.unwrap();
//         info!("received message: {:?}", msg);
//         msg
//     });
//
//     info!("end something_async {:?}", msg);
// }