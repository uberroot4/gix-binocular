use std::sync::{
    Arc,
    atomic::{
        AtomicBool,
        Ordering,
    },
};
use shared::{debug};
use crate::{thread};

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

    pub fn recv(&self) -> Result<T, crossbeam::channel::RecvError> {
        self.rx.recv()
    }
}