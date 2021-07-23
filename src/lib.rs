mod utils;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn __dummy() {
    // TODO: Without this export, the wasm-pack didn't create imports.wbg and
    // failed?
    unsafe {
        log(&"Foo");
    }
}

// Shared hash map
// ----------------------------------------------------------------------------

static CACHE: Lazy<Arc<RwLock<HashMap<i32, String>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::<i32, String>::new())));

#[wasm_bindgen]
pub fn add_to_map(key: i32, value: String) {
    let mut c = CACHE.write().unwrap();
    c.insert(key, value);
}

#[wasm_bindgen]
pub fn get_from_map(key: i32) -> Option<String> {
    let c = CACHE.read().unwrap();
    c.get(&key).map(Clone::clone)
}

// Shared Channel
// ----------------------------------------------------------------------------

static CHANNEL: Lazy<(Sender<String>, Receiver<String>)> = Lazy::new(|| unbounded());

#[wasm_bindgen]
pub fn send_to_channel(str: &str) {
    let _ = CHANNEL.0.send(str.into());
}

#[wasm_bindgen]
pub fn receive_from_channel() -> String {
    // Using timeout is not possible:
    // let value = CHANNEL.1.recv_timeout(Duration::from_millis(3000)).unwrap();

    // We must receive infinitely
    let value = CHANNEL.1.recv().unwrap();
    return value;
}
