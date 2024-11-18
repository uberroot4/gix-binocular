use wasm_bindgen::prelude::wasm_bindgen;

// struct DirectoryPickerOptions {
//     id: Option<alloc::string::String>,
//     mode:
// }

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    // Use `js_namespace` here to bind `console.trace(..)` instead of just
    // `trace(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn trace(s: &str);

    // Use `js_namespace` here to bind `console.debug(..)` instead of just
    // `trace(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn debug(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn error(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);

    // #[wasm_bindgen(js_namespace = window)]
    // pub fn showDirectoryPicker(/*options: Option<DirectoryPickerOptions>*/) -> js_sys::Promise;
}