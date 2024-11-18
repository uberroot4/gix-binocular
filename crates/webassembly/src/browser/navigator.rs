use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

pub async fn get_root_storage() -> web_sys::FileSystemDirectoryHandle {
    let navigator = web_sys::window().unwrap().navigator();
    let opfs_root = JsFuture::from(navigator.storage().get_directory())
        .await
        .unwrap()
        .dyn_into::<web_sys::FileSystemDirectoryHandle>()
        .unwrap();
    opfs_root
}