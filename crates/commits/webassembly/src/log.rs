#[warn(unused_macros)]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (crate::external::log(&format_args!($($t)*).to_string()))
}

#[warn(unused_macros)]
macro_rules! console_err {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (crate::external::error(&format_args!($($t)*).to_string()))
}
#[warn(unused_imports)]
pub(crate) use console_log;
#[warn(unused_imports)]
pub(crate) use console_err;