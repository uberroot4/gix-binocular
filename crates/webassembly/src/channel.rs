use std::path::Path;
use async_std::stream::StreamExt;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;
use shared::{info, trace};
use crate::thread;

pub struct Channel<T> {
    pub(crate) rx: crossbeam::channel::Receiver<T>,
    pub(crate) tx: crossbeam::channel::Sender<T>,
}

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[tsify(namespace)]
pub enum Action {
    Stop,
    ReadDir(String),
}

pub trait Answer {}

impl Answer for crate::Action {}

pub(crate) async fn perform_action(action: Action) {
    match action {
        Action::Stop => {
            trace!("Closing channel! (doing nothing actually)");
            crate::utils::terminate_worker();
            //     let is_closed = channel.close();
            //     debug!("Channel closed? {:?}", is_closed);
        }
        Action::ReadDir(dir) => {
            trace!("calling read_dir with {:?}", dir);
            // let read_dir = opfs::read_dir::<&Path>(dir.as_ref()).unwrap();
            // for d in read_dir {
            
            let mut read_dir = web_fs::read_dir::<&Path>(dir.as_ref()).await.unwrap();
            while let Some(d) = read_dir.next().await {
                info!("d = {:?}", d)
            }
        }
    }
}