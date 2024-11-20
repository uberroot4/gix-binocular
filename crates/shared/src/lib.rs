#![no_std]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod utils {
    pub mod logging;
}
pub use crate::utils::logging;

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
mod signature;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub use signature::{Sig};

#[allow(unused_imports)]
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use std::thread as thread;

#[allow(unused_imports)]
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
use wasm_thread as thread;