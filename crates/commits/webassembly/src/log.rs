macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (crate::external::log(&format_args!($($t)*).to_string()))
}

macro_rules! console_err {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (crate::external::error(&format_args!($($t)*).to_string()))
}
pub(crate) use console_log;
pub(crate) use console_err;