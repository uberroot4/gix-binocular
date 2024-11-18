pub fn location() -> web_sys::Location {
    web_sys::window().unwrap().location()
}