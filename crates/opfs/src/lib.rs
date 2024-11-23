#![feature(box_into_inner)]

use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool};
use web_sys::wasm_bindgen::JsCast;
use wasm_thread as thread;

mod wasm32 {
    pub(crate) mod metadata;
    pub(crate) mod file;
}

mod opfs {
    pub mod opfs;
}
pub use opfs::opfs::Opfs;
use crate::wasm32 as fs_imp;

pub use fs_imp::file::{ThreadSafeFile};
pub use web_fs::{Metadata, DirEntry, File};

mod channel;
mod action;

use channel::{Channel};
pub use action::{Action, Answer, ReadDir};
use crate::action::{ActionHandler};

thread_local! {
    static ACTION_CHANNEL: RefCell<Channel<Action>> = RefCell::new(Channel::new(8*100));
    static ANSWER_CHANNEL: RefCell<Channel<Box<dyn Answer>>> = RefCell::new(Channel::new(8*100));
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


#[deprecated]
pub fn start_webfs_consumer() -> anyhow::Result<()> {
    unimplemented!()
//     wasm_thread::spawn({
//         let action_rx = ACTION_CHANNEL.with_borrow(|cnl| cnl.rx.clone());
//         let answer_cnl = ANSWER_CHANNEL.with_borrow(|cnl| cnl.tx.clone());
//         move || {
//             let answer_tx;
//             match answer_cnl.clone() {
//                 Some(e) => {
//                     answer_tx = e;
//                     drop(answer_cnl);
//                 }
//                 None => {
//                     panic!("Answer Channel is not there")
//                 }
//             }
//
//             wasm_bindgen_futures::spawn_local(async move {
//                 trace!("Waiting for messages");
//                 CONSUMER_RUNNING.with(|cr| cr.store(true, Ordering::SeqCst));
//                 while let Ok(action) = action_rx.recv() {
//                     trace!("recv: {:?}", action);
//                     match action {
//                         Action::Metadata(e) => {
//                             trace!("Action::Metadata({})", e);
//                             // let result = MetadataHandler::handle_message(MetadataCmd::new(e));
//                             crate::action::MetadataAction::new(e).handle().await;
//                         }
//                         Action::ReadDir(dir) => {
//                             trace!("Action::ReadDir({:?})", dir);
//                             // let read_dir = opfs::read_dir::<&Path>(dir.as_ref()).unwrap();
//                             //  in read_dir {
//                             // for d in crate::action::ReadDirAction::new(dir.clone()).handle().await {
//                             //     debug!("d = {:?}", d)
//                             // }
//                             crate::action::ReadDirAction::new(dir).handle().await;
//                         }
//                         _ => {
//                             unimplemented!("lol")
//                         }
//                     };
//                     // crate::channel::perform_action(action).await;
//                     if let Ok(val) = perform_action_result {
//                         trace!("perform_action_result: {:?}", val);
//
//                         match answer_tx.send(val) {
//                             Ok(_) => {
//                                 debug!("Answer sent successfully");
//                             }
//                             Err(e) => {
//                                 error!("Error sending answer {:?}", e.to_string())
//                             }
//                         }
//                     } else {
//                         error!("perform_action_result did not return successfully");
//                     }
//                     // trace!("perform_action_result");
//                 }
//                 drop(action_rx);
//                 trace!("No further messages can be processed; the channel is closed.");
//                 self::terminate_worker();
//             });
//         }
//     });
//     Ok(())
}

// fn recv() -> anyhow::Result<Box<dyn Answer>> {
//     // assert!(thread::is_web_worker_thread(), "recv must not run in the main thread!");
//     trace!("Waiting for Answer in recv()");
//     match ANSWER_CHANNEL.with_borrow(|cnl| cnl.rx.recv()) {
//         Ok(val) => {
//             debug!("Answer in recv(): {:?}", val);
//             anyhow::Ok(val)
//         }
//         Err(e) => {
//             error!("Error receiving content, {:?}", e);
//             Err(Error::from(e))
//         }
//     }
// }

// pub fn init_channels() -> (Option<Sender<Action>>, Receiver<Box<dyn Answer>>) {
//     let action_tx = ACTION_CHANNEL.with_borrow(|cnl| cnl.tx.clone());
//     let answer_rx = ANSWER_CHANNEL.with_borrow(|cnl| cnl.rx.clone());
//
//     (action_tx, answer_rx)
// }

pub fn send_action(action: Action) -> Option<crossbeam::channel::Receiver<Box<dyn Answer>>> {
    unimplemented!()
//     // assert!(!thread::is_web_worker_thread(), "send_action must run in the main thread!"); // (js_name = 'sendAction')
//     trace!("Sending message {:?}", action);
//
//     if matches!(action, Action::Start) {
//         debug!("Action::Start");
//         if !CONSUMER_RUNNING.with(|cr| cr.load(Ordering::SeqCst)) {
//             match start_webfs_consumer() {
//                 Ok(_) => {
//                     trace!("Web-FS Consumer started OK");
//                 }
//                 Err(e) => {
//                     log::error!(target: "Error starting Web-FS Consumer: {:?}", "{}", e.to_string());
//                     // return Ok(Box::new(AnswerResult::Error(anyhow::Error::from(e))));
//                     return None;
//                 }
//             }
//         } else {
//             warn!("Cannot start worker, already running! Send Action::Stop required.")
//         }
//         // return Ok(Box::new(AnswerResult::Success(true)));
//         return None;
//     }
//
//     ACTION_CHANNEL.with_borrow_mut(|cnl| {
//         if matches!(action, Action::Stop) {
//             debug!("Stopping channel due to Action::Stop");
//             cnl.close(); // Gracefully stop the channel
//             CONSUMER_RUNNING.with(|cr| cr.store(false, Ordering::SeqCst));
//             return;
//         }
//
//         match cnl.send(action) {
//             Ok(_) => {
//                 debug!("Sending was OK");
//             }
//             Err(e) => {
//                 warn!("Error sending {:?}", e.to_string());
//             }
//         }
//     });
//     // Some(ANSWER_CHANNEL.with_borrow(|cnl| cnl.rx.clone()))
//     None
}