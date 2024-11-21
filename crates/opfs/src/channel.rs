use std::io::{Error, ErrorKind};
use std::path::Path;
use std::sync::{
    Arc,
    atomic::{
        AtomicBool,
        Ordering,
    },
};
use async_std::{
    stream::StreamExt,
    io::ReadExt,
};
use shared::{debug, error, info, trace};
use crate::{thread, action::Action, answer::Answer};
use crate::answer::AnswerResult;

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

pub(crate) async fn perform_action(action: Action) -> Box<AnswerResult> {
    let result = match action.clone() {
        Action::ReadDir(dir) => {
            trace!("Action::ReadDir({:?})", dir);
            // let read_dir = opfs::read_dir::<&Path>(dir.as_ref()).unwrap();
            // for d in read_dir {

            let mut entries = vec![];
            if let Ok(mut read_dir) = web_fs::read_dir::<&Path>(dir.as_ref()).await {
                while let Some(d) = read_dir.next().await {
                    info!("d = {:?}", d);
                    entries.push(d.unwrap().path().as_os_str().to_os_string().into_string().unwrap());
                }
                Box::new(AnswerResult::DirectoryContents(entries))
            } else {
                Box::new(AnswerResult::Error(
                    Error::new(ErrorKind::Other, format!("Action::ReadDir({})", dir))
                ))
            }
        }
        Action::OpenFile(file) => {
            trace!("Action::OpenFile({:?})", file);
            match web_fs::File::open::<&Path>(file.as_ref()).await {
                Ok(file) => {
                    info!("file = {:?}", file);
                    Box::new(AnswerResult::Success(true))
                }
                Err(e) => {
                    Box::new(AnswerResult::Error(
                        Error::new(e.kind(), format!("Action::OpenFile({})", file))
                    ))
                }
            }
        }
        Action::ReadFile(file) => {
            trace!("Action::ReadFile({:?})", file);
            // let content = web_fs::File::open("/web_repo/.git/HEAD").await.unwrap().read_to_end(&mut output).await.unwrap();
            match web_fs::File::open(file).await {
                Ok(mut opened) => {
                    let mut output = Vec::new();
                    match opened.read_to_end(&mut output).await {
                        Ok(size) => {
                            debug!("size: {:?}", size);
                            info!("content: {:?}", String::from_utf8(output.clone()));
                            // Ok(output)
                            Box::new(AnswerResult::FileContents(output))
                        }
                        Err(e) => {
                            Box::new(AnswerResult::Error(
                                Error::new(e.kind(), "file.read_to_end")
                            ))
                        }
                    }
                }
                Err(e) => {
                    Box::new(AnswerResult::Error(
                        Error::new(e.kind(), format!("Action::ReadFile({})", e))
                    ))
                }
            }
        }
        // Action::Metadata(file) => {
        //     trace!("Action::Metadata({})", file);
        //     match web_fs::metadata::<&Path>(file.as_ref()).await {
        //         Ok(metadata) => {
        //             debug!("metadata: {:?}", metadata);
        //             // Ok(metadata)
        //             Box::new(AnswerResult::Metadata(metadata))
        //         }
        //         Err(e) => {
        //             Box::new(AnswerResult::Error(
        //                 Error::new(e.kind(), format!("Action::Metadata({})", e))
        //             ))
        //         }
        //     }
        // }
        _ => {
            unimplemented!("Action not implemented for performing in WebWorker: '{:?}'", action);
        }
    };
    // let x = match result {
    //     Ok(e) => {
    //         Ok(e)
    //     }
    //     Err(e) => {
    //         error!("Error {:?}:\t{:?}", action, e);
    //         Err(e)
    //     }
    // };
    result
}