use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use async_std::stream::StreamExt;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;
use shared::{debug, info, trace, warn};
use crate::thread;

pub struct Channel<T: Send + 'static> {
    pub(crate) rx: crossbeam::channel::Receiver<T>,
    pub(crate) tx: Option<crossbeam::channel::Sender<T>>,
    pub(crate) closed: Arc<AtomicBool>, // To track if the channel is closed
}

impl<T: Send + 'static> Channel<T> {
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = crossbeam::channel::bounded(capacity);
        Channel {
            rx,
            tx: Some(tx),
            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn reset(&mut self) {
        let (tx, rx) = crossbeam::channel::bounded(self.rx.capacity().unwrap());
        self.rx = rx;
        self.tx = Some(tx);
        self.closed = Arc::new(AtomicBool::new(false))
    }

    pub fn close(&mut self) -> bool {
        self.closed.store(true, Ordering::SeqCst);
        if let Some(s) = self.tx.take() {
            assert!(self.is_closed());
            debug!("Channel is now closed, and sender has been dropped.");
            drop(s);
        }
        self.reset();

        self.is_closed()
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    pub fn send(&self, message: T) -> Result<(), crossbeam::channel::SendError<T>> {
        // if self.is_closed() {
        //     Err(crossbeam::channel::SendError(message))
        // } else {
        if let Some(ref tx) = self.tx {
            tx.send(message)
        } else {
            Err(crossbeam::channel::SendError(message))
        }
        // }
    }
}

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[tsify(namespace)]
pub enum Action {
    Stop,
    Start,
    ReadDir(String),
}

pub trait Answer {}

impl Answer for crate::Action {}

pub(crate) async fn perform_action(action: Action) {
    match action {
        Action::ReadDir(dir) => {
            trace!("calling read_dir with {:?}", dir);
            // let read_dir = opfs::read_dir::<&Path>(dir.as_ref()).unwrap();
            // for d in read_dir {

            let mut read_dir = web_fs::read_dir::<&Path>(dir.as_ref()).await.unwrap();
            while let Some(d) = read_dir.next().await {
                info!("d = {:?}", d)
            }
        },
        _ => {
            warn!("Action not implemented for performing in WebWorker: {:?}", action)
        }
    }
}