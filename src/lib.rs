mod utils;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

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

#[wasm_bindgen]
pub fn __dummy() {
    // TODO: Without this export, the wasm-pack didn't create imports.wbg and
    // failed?
    unsafe {
        log(&"Foo");
    }
}
