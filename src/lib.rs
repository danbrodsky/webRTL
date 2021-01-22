#[macro_use] extern crate nom;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate web_sys;
extern crate js_sys;

/// WASM

extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::panic;
use std::rc::Rc;
use std::cell::RefCell;
use js_sys::Uint8Array;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_use]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn test_simulator(frames: JsValue) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
    // log::set_max_level(LevelFilter::Off);

    run(frames)

}


/// LOGGING

struct WebLogger;

use log::{Record, Level, Metadata, Log, LevelFilter};
impl Log for WebLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            console_log!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: WebLogger = WebLogger;


/// ENTRY

#[macro_use]
pub mod util;
pub mod config;
pub mod sim;
pub mod graphics;

use crate::graphics::*;

fn run(frames: JsValue) {

    let arr: Uint8Array = frames.into();
    let mut data = vec![0; arr.length() as usize];
    arr.copy_to(&mut data[..]);

    let (head, body, _tail) = unsafe { data.align_to::<FrameBuffer>()};
    assert!(head.len() == 0, "mistake when decompressing");

    for i in 0..FRAME_CACHE_SIZE {
        unsafe { FRAME_CACHE[i] = body[i] };
    }

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();


    let t = Rc::new(RefCell::new(None));
    let tc = f.clone();

    let w = web_sys::window().unwrap();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // info!("frame request: {:?}", graphics::FRAME);

        // this is safe since buffer size is always within modified bounds
        // for _ in 0..801 {
        //     set("clk", 1);
        //     set("en", 1);
        //     sim.run();
        //     unsafe {graphics::test_render();}
        // }


        graphics::request_animation_frame(&w, t.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    let w2 = web_sys::window().unwrap();
    *tc.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        render_next();
        draw(&ctx).expect("failed drawing to screen");

        set_timeout(&w2,
            &f.borrow().as_ref().unwrap(),
            1000 / 60);

    }) as Box<dyn FnMut()>));

        request_animation_frame(&window(), tc.borrow().as_ref().unwrap());
    }
