#[macro_use] extern crate nom;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate web_sys;

/// WASM

extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::panic;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{CanvasRenderingContext2d};

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
pub fn test_simulator() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Warn);
    // log::set_max_level(LevelFilter::Off);

    run();

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

pub mod config;
pub mod sim;
pub mod graphics;

fn run() {
    info!("run start");

    let config_data =
r#".model vga
.inputs clk rst en
.outputs pixel[0] pixel[1] pixel[2] pixel[3]
.names hvSync[0] $abc$1349$new_n55_ rst $abc$1164$procdff$37.NEXT_Q[0]
010 1
100 1
.names $abc$1349$new_n58_ $abc$1349$new_n57_ $abc$1349$new_n56_ $abc$1349$new_n55_
111 1
.names en hvSync[18] hvSync[19] $abc$1349$new_n56_
111 1
.names hvSync[15] hvSync[17] hvSync[16] hvSync[14] $abc$1349$new_n57_
1000 1
.names hvSync[12] hvSync[13] hvSync[10] hvSync[11] $abc$1349$new_n58_
0000 1
.names hvSync[1] $abc$1349$new_n60_ hvSync[0] $abc$1349$new_n55_ $abc$1164$procdff$37.NEXT_Q[1]
0011 1
1000 1
1001 1
1010 1
.names rst $abc$1349$new_n67_ $abc$1349$new_n65_ $abc$1349$new_n61_ $abc$1349$new_n60_
0111 1
1000 1
1001 1
1010 1
1011 1
1100 1
1101 1
1110 1
1111 1
.names en $abc$1349$new_n64_ $abc$1349$new_n63_ $abc$1349$new_n62_ $abc$1349$new_n61_
1111 1
.names hvSync[14] hvSync[12] $abc$1349$new_n62_
00 1
.names hvSync[18] hvSync[19] hvSync[17] hvSync[16] $abc$1349$new_n63_
1100 1
.names hvSync[15] hvSync[13] hvSync[10] hvSync[11] $abc$1349$new_n64_
1000 1
.names hvSync[2] hvSync[3] hvSync[0] $abc$1349$new_n66_ $abc$1349$new_n65_
1111 1
.names hvSync[9] hvSync[8] hvSync[7] hvSync[1] $abc$1349$new_n66_
1000 1
.names hvSync[6] hvSync[4] hvSync[5] $abc$1349$new_n67_
000 1
.names $abc$1349$new_n69_ $abc$1349$new_n60_ $abc$1164$procdff$37.NEXT_Q[2]
10 1
.names hvSync[2] hvSync[1] hvSync[0] $abc$1349$new_n61_ $abc$1349$new_n69_
0111 1
1000 1
1001 1
1010 1
1011 1
1100 1
1101 1
1110 1
.names $abc$1349$new_n71_ hvSync[3] $abc$1349$new_n60_ $abc$1164$procdff$37.NEXT_Q[3]
010 1
100 1
.names hvSync[2] hvSync[1] hvSync[0] $abc$1349$new_n55_ $abc$1349$new_n71_
1111 1
.names $abc$1349$new_n73_ $abc$1349$new_n61_ $abc$1349$new_n72_
11 1
.names hvSync[2] hvSync[3] hvSync[1] hvSync[0] $abc$1349$new_n73_
1111 1
.names hvSync[4] $abc$1349$new_n72_ rst $abc$1164$procdff$37.NEXT_Q[4]
010 1
100 1
.names hvSync[5] $abc$1349$new_n76_ rst $abc$1164$procdff$37.NEXT_Q[5]
010 1
100 1
.names hvSync[4] $abc$1349$new_n73_ $abc$1349$new_n61_ $abc$1349$new_n76_
111 1
.names hvSync[6] rst hvSync[5] $abc$1349$new_n76_ $abc$1164$procdff$37.NEXT_Q[6]
0011 1
1000 1
1001 1
1010 1
.names hvSync[7] $abc$1349$new_n79_ rst $abc$1164$procdff$37.NEXT_Q[7]
010 1
100 1
.names hvSync[4] $abc$1349$new_n80_ $abc$1349$new_n73_ $abc$1349$new_n55_ $abc$1349$new_n79_
1111 1
.names hvSync[6] hvSync[5] $abc$1349$new_n80_
11 1
.names hvSync[8] rst $abc$1349$new_n79_ hvSync[7] $abc$1164$procdff$37.NEXT_Q[8]
0011 1
1000 1
1001 1
1010 1
.names rst $abc$1349$new_n58_ $abc$1349$new_n57_ $abc$1349$new_n56_ $abc$1349$new_n83_
0000 1
0001 1
0010 1
0011 1
0100 1
0101 1
0110 1
.names hvSync[8] hvSync[7] hvSync[4] $abc$1349$new_n80_ $abc$1349$new_n84_
1111 1
.names hvSync[9] $abc$1349$new_n86_ $abc$1349$new_n60_ $abc$1164$procdff$37.NEXT_Q[9]
010 1
100 1
.names $abc$1349$new_n73_ $abc$1349$new_n84_ $abc$1349$new_n55_ $abc$1349$new_n86_
111 1
.names $abc$1349$new_n83_ en hvSync[10] $abc$1164$procdff$37.NEXT_Q[10]
101 1
110 1
.names hvSync[11] $abc$1349$new_n89_ rst $abc$1164$procdff$37.NEXT_Q[11]
010 1
100 1
.names en hvSync[10] $abc$1349$new_n89_
11 1
.names hvSync[12] rst hvSync[11] $abc$1349$new_n89_ $abc$1164$procdff$37.NEXT_Q[12]
0011 1
1000 1
1001 1
1010 1
.names hvSync[13] $abc$1349$new_n92_ rst $abc$1164$procdff$37.NEXT_Q[13]
010 1
100 1
.names hvSync[12] hvSync[11] $abc$1349$new_n89_ $abc$1349$new_n92_
111 1
.names hvSync[14] $abc$1349$new_n94_ rst $abc$1164$procdff$37.NEXT_Q[14]
010 1
100 1
.names hvSync[12] hvSync[13] hvSync[11] $abc$1349$new_n89_ $abc$1349$new_n94_
1111 1
.names $abc$1349$new_n83_ hvSync[15] hvSync[14] $abc$1349$new_n94_ $abc$1164$procdff$37.NEXT_Q[15]
1011 1
1100 1
1101 1
1110 1
.names hvSync[16] $abc$1349$new_n97_ rst $abc$1164$procdff$37.NEXT_Q[16]
010 1
100 1
.names hvSync[15] hvSync[14] $abc$1349$new_n94_ $abc$1349$new_n97_
111 1
.names hvSync[17] $abc$1349$new_n99_ rst $abc$1164$procdff$37.NEXT_Q[17]
010 1
100 1
.names hvSync[16] hvSync[15] hvSync[14] $abc$1349$new_n94_ $abc$1349$new_n99_
1111 1
.names $abc$1349$new_n83_ hvSync[18] $abc$1349$new_n101_ $abc$1164$procdff$37.NEXT_Q[18]
101 1
110 1
.names hvSync[15] hvSync[14] $abc$1349$new_n102_ $abc$1349$new_n94_ $abc$1349$new_n101_
1111 1
.names hvSync[17] hvSync[16] $abc$1349$new_n102_
11 1
.names $abc$1349$new_n83_ hvSync[19] hvSync[18] $abc$1349$new_n101_ $abc$1164$procdff$37.NEXT_Q[19]
1011 1
1100 1
1101 1
1110 1
.names en rst $abc$1349$new_n105_ hvSync[20] $abc$1164$procdff$37.NEXT_Q[20]
0001 1
0011 1
1010 1
1011 1
.names hvSync[9] $abc$1349$new_n107_ $abc$1349$new_n106_ $abc$1349$new_n105_
000 1
.names hvSync[8] hvSync[7] $abc$1349$new_n80_ $abc$1349$new_n106_
111 1
.names hvSync[19] hvSync[17] hvSync[18] $abc$1349$new_n107_
101 1
110 1
111 1
.names rst $abc$1349$new_n109_ $abc$1164$procdff$38.NEXT_Q[0]
00 1
.names en result[0] hvSync[16] hvSync[20] $abc$1349$new_n109_
0000 1
0001 1
0010 1
0011 1
1000 1
1001 1
1010 1
1100 1
1101 1
1110 1
.names rst $abc$1349$new_n111_ $abc$1164$procdff$38.NEXT_Q[1]
00 1
.names en result[1] hvSync[17] hvSync[20] $abc$1349$new_n111_
0000 1
0001 1
0010 1
0011 1
1000 1
1001 1
1010 1
1100 1
1101 1
1110 1
.names rst $abc$1349$new_n113_ $abc$1164$procdff$38.NEXT_Q[2]
00 1
.names en result[2] hvSync[18] hvSync[20] $abc$1349$new_n113_
0000 1
0001 1
0010 1
0011 1
1000 1
1001 1
1010 1
1100 1
1101 1
1110 1
.names rst $abc$1349$new_n115_ $abc$1164$procdff$38.NEXT_Q[3]
00 1
.names en result[3] hvSync[19] hvSync[20] $abc$1349$new_n115_
0000 1
0001 1
0010 1
0011 1
1000 1
1001 1
1010 1
1100 1
1101 1
1110 1
.latch $abc$1164$procdff$37.NEXT_Q[0] hvSync[0] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[1] hvSync[1] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[2] hvSync[2] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[3] hvSync[3] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[4] hvSync[4] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[5] hvSync[5] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[6] hvSync[6] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[7] hvSync[7] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[8] hvSync[8] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[9] hvSync[9] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[10] hvSync[10] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[11] hvSync[11] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[12] hvSync[12] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[13] hvSync[13] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[14] hvSync[14] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[15] hvSync[15] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[16] hvSync[16] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[17] hvSync[17] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[18] hvSync[18] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[19] hvSync[19] re clk 0
.latch $abc$1164$procdff$37.NEXT_Q[20] hvSync[20] re clk 0
.latch $abc$1164$procdff$38.NEXT_Q[0] result[0] re clk 0
.latch $abc$1164$procdff$38.NEXT_Q[1] result[1] re clk 0
.latch $abc$1164$procdff$38.NEXT_Q[2] result[2] re clk 0
.latch $abc$1164$procdff$38.NEXT_Q[3] result[3] re clk 0
.names hvSync[16] c$bv[0]
1 1
.names hvSync[17] c$bv[1]
1 1
.names hvSync[18] c$bv[2]
1 1
.names hvSync[19] c$bv[3]
1 1
.names hvSync[10] c$hvSyncState_$jOut_app_arg_selection[0]
1 1
.names hvSync[11] c$hvSyncState_$jOut_app_arg_selection[1]
1 1
.names hvSync[12] c$hvSyncState_$jOut_app_arg_selection[2]
1 1
.names hvSync[13] c$hvSyncState_$jOut_app_arg_selection[3]
1 1
.names hvSync[14] c$hvSyncState_$jOut_app_arg_selection[4]
1 1
.names hvSync[15] c$hvSyncState_$jOut_app_arg_selection[5]
1 1
.names hvSync[16] c$hvSyncState_$jOut_app_arg_selection[6]
1 1
.names hvSync[17] c$hvSyncState_$jOut_app_arg_selection[7]
1 1
.names hvSync[18] c$hvSyncState_$jOut_app_arg_selection[8]
1 1
.names hvSync[19] c$hvSyncState_$jOut_app_arg_selection[9]
1 1
.names $false c$hvSyncState_$jOut_case_alt[10]
1 1
.names $false c$hvSyncState_$jOut_case_alt[11]
1 1
.names $false c$hvSyncState_$jOut_case_alt[12]
1 1
.names $false c$hvSyncState_$jOut_case_alt[13]
1 1
.names $false c$hvSyncState_$jOut_case_alt[14]
1 1
.names $false c$hvSyncState_$jOut_case_alt[15]
1 1
.names $false c$hvSyncState_$jOut_case_alt[16]
1 1
.names $false c$hvSyncState_$jOut_case_alt[17]
1 1
.names $false c$hvSyncState_$jOut_case_alt[18]
1 1
.names $false c$hvSyncState_$jOut_case_alt[19]
1 1
.names hvSync[20] c$hvSyncState_$jOut_case_alt[20]
1 1
.names $false c$hvSyncState_$jOut_case_alt[21]
1 1
.names $false c$hvSyncState_$jOut_case_alt[22]
1 1
.names hvSync[0] c$hvSyncState_$jOut_case_alt_selection[0]
1 1
.names hvSync[1] c$hvSyncState_$jOut_case_alt_selection[1]
1 1
.names hvSync[2] c$hvSyncState_$jOut_case_alt_selection[2]
1 1
.names hvSync[3] c$hvSyncState_$jOut_case_alt_selection[3]
1 1
.names hvSync[4] c$hvSyncState_$jOut_case_alt_selection[4]
1 1
.names hvSync[5] c$hvSyncState_$jOut_case_alt_selection[5]
1 1
.names hvSync[6] c$hvSyncState_$jOut_case_alt_selection[6]
1 1
.names hvSync[7] c$hvSyncState_$jOut_case_alt_selection[7]
1 1
.names hvSync[8] c$hvSyncState_$jOut_case_alt_selection[8]
1 1
.names hvSync[9] c$hvSyncState_$jOut_case_alt_selection[9]
1 1
.names $false hvSync[21]
1 1
.names $false hvSync[22]
1 1
.names result[0] pixel[0]
1 1
.names result[1] pixel[1]
1 1
.names result[2] pixel[2]
1 1
.names result[3] pixel[3]
1 1
.end
"#;

    let config = config::Config::new(&config_data);

    let sim = sim::Simulation::init(config);


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

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // info!("frame request: {:?}", graphics::FRAME);

        // this is safe since buffer size is always within modified bounds
        for _ in 0..801 {
            config::set("clk", 1);
            config::set("en", 1);
            sim.run();
            unsafe {graphics::test_render();}
        }
        graphics::draw(&ctx).expect("failed drawing to screen");

        graphics::request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    graphics::request_animation_frame(g.borrow().as_ref().unwrap());

    // let g = Some(Closure::wrap(Box::new(move || {
    //     request_animation_frame(

    // TODO: Entry into program
}
