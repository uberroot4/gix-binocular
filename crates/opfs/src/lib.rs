use std::cell::RefCell;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use web_sys::wasm_bindgen::JsCast;
use wasm_thread as thread;

mod wasm32 {
    pub(crate) mod read_dir;
    pub(crate) mod metadata;
    pub(crate) mod file;
}

use crate::wasm32 as fs_imp;

pub use fs_imp::read_dir::{ReadDir};
pub use fs_imp::file::{ThreadSafeFile};
use shared::{debug, trace, warn};
pub use web_fs::{Metadata, DirEntry, File};

mod channel;
mod answer;
mod message;
mod action;

use channel::{Channel};
pub use action::Action;
use crate::action::ActionHandler;

thread_local! {
    static CHANNEL: RefCell<Channel<Action>> = RefCell::new(Channel::new(8*100));
    static CONSUMER_RUNNING : Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

fn terminate_worker() {
    match web_sys::js_sys::eval("self")
        .unwrap()
        .dyn_into::<web_sys::DedicatedWorkerGlobalScope>() {
        Ok(worker_scope) => {
            worker_scope.close();
        }
        Err(_) => {}
    }
}

pub fn read_dir<P: AsRef<Path>>(path: P) -> std::io::Result<crate::ReadDir> {
    fs_imp::read_dir::readdir(path)
}

pub fn metadata<P: AsRef<Path>>(path: P) -> std::io::Result<crate::Metadata> {
    fs_imp::metadata::metadata(path)
}

// pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<ThreadSafeFile> {
//     fs_imp::file::open(path)
// }

pub fn start_webfs_consumer() {
    wasm_thread::spawn({
        let rx = CHANNEL.with_borrow(|cnl| cnl.rx.clone());
        move || {
            wasm_bindgen_futures::spawn_local(async move {
                trace!("Waiting for messages");
                CONSUMER_RUNNING.with(|cr| cr.store(true, Ordering::SeqCst));
                while let Ok(action) = rx.recv() {
                    trace!("recv: {:?}", action);
                    let perform_action_result = match action {
                        Action::Metadata(e) => {
                            // let result = MetadataHandler::handle_message(MetadataCmd::new(e));
                            crate::action::MetadataAction::new(e).handle().await
                        }
                        _ => {
                            unimplemented!("lol")
                        }
                    };
                    // crate::channel::perform_action(action).await;
                    // trace!("perform_action_result: {}", perform_action_result.as_debug());
                }
                drop(rx);
                trace!("No further messages can be processed; the channel is closed.");
                self::terminate_worker();
            });
        }
    });
}

pub fn send_action(action: Action) {
    assert!(!thread::is_web_worker_thread(), "send_action must run in the main thread!");
    trace!("Sending message {:?}", action);

    if matches!(action, Action::Start) {
        debug!("Action::Start");
        if !CONSUMER_RUNNING.with(|cr| cr.load(Ordering::SeqCst)) {
            start_webfs_consumer();
        } else {
            warn!("Cannot start worker, already running! Send Action::Stop required.")
        }
        return;
    }

    CHANNEL.with_borrow_mut(|cnl| {
        if matches!(action, Action::Stop) {
            debug!("Stopping channel due to Action::Stop");
            cnl.close(); // Gracefully stop the channel
            CONSUMER_RUNNING.with(|cr| cr.store(false, Ordering::SeqCst));
            return;
        }

        match cnl.send(action) {
            Ok(_) => {
                debug!("Sending was OK");
            }
            Err(e) => {
                warn!("Error sending {:?}", e.to_string());
            }
        }
    });
}