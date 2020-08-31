use wasm_bindgen::prelude::*;
use web_sys::console;
use serde::{Serialize, Deserialize};
use rand::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn util () {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
#[derive(Serialize)]
#[allow(dead_code)]
pub struct Whatever {
    msg: &'static str,
}

#[wasm_bindgen]
#[derive(Serialize)]
#[allow(dead_code)]
pub struct GameState {
    yeet: i32,
    yote: &'static str,
    yeeted: Vec<i32>,
    whatevs: Whatever,
}

#[wasm_bindgen]
pub fn tick () -> Result<JsValue, JsValue> {
    let mut yeeted: Vec<i32> = vec!();
    let mut i = 0;
    while i < 500 {
        let x: i32 = random();
        yeeted.push(x);
        i += 1;
    }

    let game_state = GameState {
        yeet: 42,
        yote: "i have yoted",
        yeeted: yeeted,
        whatevs: Whatever {
            msg: "hello world"
        }
    };

    serde_wasm_bindgen::to_value(&game_state)
    .map_err(|err| err.into())
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main() -> () {
    util();

    console::log_1(&JsValue::from_str("Hello world!"));
}
