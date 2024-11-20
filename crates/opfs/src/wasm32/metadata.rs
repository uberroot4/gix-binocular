use std::path::Path;
use crate::thread;

pub fn metadata<P: AsRef<Path>>(path: P) -> std::io::Result<crate::Metadata> {
    shared::trace!("metadata(path={:?})", path.as_ref());
    let (tx, rx) = futures_channel::oneshot::channel();
    let dir_ = std::path::PathBuf::from(path.as_ref());
    shared::debug!("dir_: {:?}", dir_);
    let _wasm_thread_handle = thread::spawn(|| {
        wasm_bindgen_futures::spawn_local(async move { // required as FileSystemDirectoryHandle is not Send
            shared::debug!("dir_: {:?}", dir_);
            let metadata = web_fs::metadata(dir_.as_path()).await;
            shared::trace!("metadata {:?}", metadata);
            drop(tx.send(metadata));
        });
        // terminate_self(); // not working rn, as web-fs/open_options has spawn_local
    }).join_async();

    match futures::executor::block_on(rx) {
        Ok(_data) => {
            shared::debug!("received _data: {:?}", _data);
            crate::terminate_worker();
            Ok(_data?)
        }
        Err(e) => {
            crate::terminate_worker();
            use std::io::{Error, ErrorKind};
            shared::error!("Error within threads: {:?}", e);
            Err(Error::new(ErrorKind::Interrupted, e.to_string()))
        }
    }
}
