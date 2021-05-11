mod utils;

use wasm_bindgen::prelude::*;

use cli::note::Note;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-wasm!");
}

#[wasm_bindgen]
pub fn fact(n: u32) -> u32 {
    if (n <= 1) {
        1
    } else {
        n * fact(n - 1)
    }
}

#[wasm_bindgen]
pub fn pi(n: u32) -> f32 {
    let mut v = 0f32;
    for i in 1..=n {
        let i = i as f32;
        v += 1. / i - 1. / (i + 2.)
    }
    4. * v
}

#[wasm_bindgen]
pub fn napier(n: u32) -> f32 {
    let mut e = 0f32;
    for i in 1..=n {
        e += 1. / fact(i) as f32
    }
    e
}
