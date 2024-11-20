use std::path::Path;

#[derive(Debug)]
pub struct ThreadSafeFile {
    // pub inner: std::fs::File
}
impl ThreadSafeFile {
    // pub fn new(name: String) -> Self {
    //     Self { name }
    // }

    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<ThreadSafeFile> {
        shared::trace!("File::open_sync({:?})", path.as_ref());
        use wasm_thread as thread;
        let (tx, rx) = futures_channel::oneshot::channel();
        let dir_ = std::path::PathBuf::from(path.as_ref());
        shared::debug!("dir_: {:?}", dir_);
        let _wasm_thread_handle = thread::spawn(move || {
            wasm_bindgen_futures::spawn_local(async move {
                shared::debug!("dir_: {:?}", dir_);

                let file = crate::File::open(dir_.as_path()).await.unwrap();
                // file.
                let ts_file = ThreadSafeFile {};

                shared::trace!("file");
                drop(tx.send(ts_file));
                // crate::terminate_self(); // not working rn, as web-fs/open_options has spawn_local
            });
        }).join_async();

        // let rx_handle = thread::spawn(move || {
        shared::trace!("rx_handle");
        match futures::executor::block_on(rx) {
            Ok(_data) => {
                shared::debug!("received _data: {:?}", _data);
                crate::terminate_worker();
                Ok(_data)
            }
            Err(e) => {
                use std::io::{Error, ErrorKind};
                crate::terminate_worker();
                shared::error!("Error within threads: {:?}", e);
                Err(Error::new(ErrorKind::Interrupted, e.to_string()))
            }
        }
        // }).join(); // blocking!
        // match rx_handle {
        //     Ok(r) => {
        //         shared::debug!("Result rx_handle: {:?}", r);
        //         // let val = FS.with_borrow(|fs| fs.open(r, options, inner_clone));
        //         Ok(r)
        //     }
        //     Err(e) => {
        //
        //     }
        // }
    }
}

// impl From<crate::File> for ThreadSafeFile {
//     fn from(value: crate::File) -> Self {
//         Self {
//             fd: value.,
//             size: value.size,
//         }
//     }
// }


// impl From<ThreadSafeFile> for crate::File {
//
//     fn from(value: ThreadSafeFile) -> Self {
//         File::new(value.fd, value.size)
//     }
// }

// impl Read for ThreadSafeFile {
//     fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
//         todo!()
//     }
// }

