use crate::action::ActionHandler;
use crate::channel::Channel;
use crate::thread;
use crate::{Action, Answer};
use shared::{debug, error, trace};
use std::cell::RefCell;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

thread_local! {
    static CONSUMER_RUNNING : Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

pub struct Opfs {
    pub action_channel: RefCell<Channel<Action>>,
    pub answer_channel: RefCell<Channel<Box<dyn Answer>>>,
}

impl Opfs {
    fn new(
        action_channel: RefCell<Channel<Action>>,
        answer_channel: RefCell<Channel<Box<dyn Answer>>>,
    ) -> Self {
        Self {
            action_channel,
            answer_channel,
        }
    }

    pub fn start_webfs_consumer(&self) -> anyhow::Result<bool> {
        trace!("Starting webfs consumer");
        let (active_tx, active_rx) = crossbeam::channel::bounded::<bool>(1);
        wasm_thread::spawn({
            let action_rx = self.action_channel.borrow().rx.clone();
            let answer_cnl = self.answer_channel.borrow().tx.clone();
            move || {
                let answer_tx;
                match answer_cnl.clone() {
                    Some(e) => {
                        answer_tx = e;
                        drop(answer_cnl);
                    }
                    None => {
                        panic!("Answer Channel is not there")
                    }
                }

                wasm_bindgen_futures::spawn_local(async move {
                    active_tx.send(true).expect("Sending should not block");
                    drop(active_tx);
                    trace!("Waiting for messages");
                    while let Ok(action) = action_rx.recv() {
                        trace!("recv: {:?}", action);
                        let perform_action_result: std::io::Result<Box<dyn Answer>> = match action {
                            Action::Metadata(e) => {
                                trace!("Action::Metadata({})", e);
                                crate::action::MetadataAction::new(e).handle().await
                            }
                            Action::ReadDir(dir) => {
                                trace!("Action::ReadDir({:?})", dir);
                                crate::action::ReadDirAction::new(dir).handle().await
                            }
                            _ => {
                                unimplemented!("lol")
                            }
                        };
                        // crate::channel::perform_action(action).await;
                        if let Ok(val) = perform_action_result {
                            trace!("perform_action_result: {:?}", val);

                            match answer_tx.send(val) {
                                Ok(_) => {
                                    debug!("Answer sent successfully");
                                }
                                Err(e) => {
                                    error!("Error sending answer {:?}", e.to_string())
                                }
                            }
                        } else {
                            error!("perform_action_result did not return successfully");
                        }
                        // trace!("perform_action_result");
                    }
                    drop(action_rx);
                    trace!("No further messages can be processed; the channel is closed.");
                    crate::terminate_worker();
                });
            }
        });
        // .join()
        // .expect("TODO: panic message");
        let started = active_rx.recv()?;
        assert!(started);
        drop(active_rx);
        CONSUMER_RUNNING.with(|cr| {
            cr.store(started, Ordering::SeqCst);
        });
        Ok(CONSUMER_RUNNING.with(|cr| cr.load(Ordering::SeqCst)))
    }

    pub fn read_dir<P: AsRef<Path>>(&self, path: P) -> std::io::Result<crate::ReadDir> {
        trace!("Opfs::read_dir({:?})", path.as_ref());
        Self::check_running_consumer();
        let action = Action::ReadDir(
            path.as_ref()
                .as_os_str()
                .to_os_string()
                .into_string()
                .unwrap(),
        );
        debug!("action {:?}", action);
        match self.action_channel.borrow().send(action.clone()) {
            Ok(_) => {
                trace!("Action sent successfully: {:?}", action)
            }
            Err(e) => {
                error!("Error sending Action: {:?}", e.to_string());
                return Err(Error::new(ErrorKind::Other, e.to_string()));
            }
        }
        trace!("Waiting for answer");
        let x = match self.answer_channel.borrow().recv() {
            Ok(recv) => {
                debug!("Received: {:?}", recv);
                let result = &*recv
                    .as_any()
                    .downcast_ref::<crate::ReadDir>()
                    .expect("Failed to downcast to crate::ReadDir");
                Ok(result.clone())
            }
            Err(e) => {
                error!("Error receiving Answer: {:?}", e.to_string());
                Err(Error::new(ErrorKind::Other, e.to_string()))
            }
        };
        x
    }

    pub fn metadata<P: AsRef<Path>>(&self, path: P) -> std::io::Result<crate::Metadata> {
        trace!("Opfs::read_dir({:?})", path.as_ref());
        Self::check_running_consumer();
        let action = Action::Metadata(
            path.as_ref()
                .as_os_str()
                .to_os_string()
                .into_string()
                .unwrap(),
        );
        debug!("action {:?}", action);
        match self.action_channel.borrow().send(action.clone()) {
            Ok(_) => {
                trace!("Action sent successfully: {:?}", action)
            }
            Err(e) => {
                error!("Error sending Action: {:?}", e.to_string());
                return Err(Error::new(ErrorKind::Other, e.to_string()));
            }
        }
        trace!("Waiting for answer");
        let x = match self.answer_channel.borrow().recv() {
            Ok(recv) => {
                debug!("Received: {:?}", recv);
                let result = &*recv
                    .as_any()
                    .downcast_ref::<crate::Metadata>()
                    .expect("Failed to downcast to crate::ReadDir");
                Ok(result.clone())
            }
            Err(e) => {
                error!("Error receiving Answer: {:?}", e.to_string());
                Err(Error::new(ErrorKind::Other, e.to_string()))
            }
        };
        x
    }

    fn check_running_consumer() {
        if !CONSUMER_RUNNING.with(|cr| cr.load(Ordering::SeqCst)) {
            let msg = "WebFS consumer not running. Call start_webfs_consumer() prior!";
            error!("{}", msg);
            panic!("{}", msg);
        }
    }

    // pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<ThreadSafeFile> {
    //     fs_imp::file::open(path)
    // }
}

impl Default for Opfs {
    fn default() -> Self {
        let action_channel: RefCell<Channel<Action>> = RefCell::new(Channel::new(8 * 100));
        let answer_channel: RefCell<Channel<Box<dyn Answer>>> = RefCell::new(Channel::new(8 * 100));
        Self::new(action_channel, answer_channel)
    }
}
