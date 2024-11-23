use std::cell::RefCell;
use std::rc::Rc;
use log::{info, trace};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlElement, MessageEvent, Worker};


#[no_mangle]
pub fn startup() {
    trace!(">> startup()");
    // Here, we create our worker. In a larger app, multiple callbacks should be
    // able to interact with the code in the worker. Therefore, we wrap it in
    // `Rc<RefCell>` following the interior mutability pattern. Here, it would
    // not be needed but we include the wrapping anyway as example.
    let worker_handle: Rc<RefCell<Worker>> = Rc::new(RefCell::new(Worker::new("/webfs-worker.js").unwrap()));
    //let worker_handle: Worker = Worker::new("/webfs-worker.js")?;
    //let worker_handle = Rc::new(RefCell::new(Worker::new("/src/file-worker.ts").unwrap()));
    info!("Created a new webfs-worker from within Wasm");

    // Pass the worker to the function which sets up the `oninput` callback.
    //setup_input_oninput_callback(Rc::new(RefCell::new(worker_handle.clone())));
    setup_input_oninput_callback(worker_handle);
    trace!("<< startup()");
    // Ok(&*worker_handle)
}

//fn setup_input_oninput_callback(worker: Rc<RefCell<web_sys::Worker>>) {
fn setup_input_oninput_callback(worker: Rc<RefCell<web_sys::Worker>>) {
    let document = web_sys::window().unwrap().document().unwrap();

    /// Create a closure to act on the message returned by the worker
    #[allow(unused_assignments)]
    let mut persistent_callback_handle = get_on_msg_callback();

    let callback = Closure::new(move || {
        trace!("oninput callback triggered");
        let worker_handle = &*worker.borrow();
        let _ = worker_handle.post_message(&"hello from startup".into()).expect("TODO: panic message");
        persistent_callback_handle = get_on_msg_callback();
        worker_handle.set_onmessage(Some(persistent_callback_handle.as_ref().unchecked_ref()));
    });

    // Attach the closure as `oninput` callback to the input field.
    document
        .get_element_by_id("startupBtn")
        .expect("#startupBtn should exist")
        .dyn_ref::<HtmlElement>()
        .expect("#startupBtn should be a HtmlElement")
        .set_onclick(Some(callback.as_ref().unchecked_ref()));
    trace!("callback attached");

    // Leaks memory.
    callback.forget();
}

fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
    Closure::new(move |event: MessageEvent| {
        trace!("Received response: {:?}", &event.data());
    })
}