#![feature(new_uninit)]
#[macro_use] extern crate nom;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate flate2;


#[macro_use]
mod util;

mod config;
mod sim;
mod graphics;

/// WASM

struct SimpleLogger;

/// LOGGING

use log::{Record, Level, Metadata, Log, LevelFilter};
impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

/// ENTRY

use clap::{crate_version, App, Arg};

use std::fs::{File, remove_file};
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;

use crate::util::*;
use crate::graphics::{VGA_BUFFER_SIZE,
                      FRAME_CACHE_SIZE,
                      FrameBuffer};
use crate::config::STATE;

fn main() {
    print!("allocating frame buffer");

    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);

    let args = App::new("webrtl")
        .version(crate_version!())
        .author("Daniel Brodsky <dnbrdsky@gmail.com>")
        .arg(
            Arg::with_name("blif")
                .help("flattened blif config")
                .required(true),
        )
        .get_matches();

    let config_data = std::fs::read_to_string(args.value_of("blif").unwrap()).expect("blif file being read");
    let config = config::Config::new(&config_data);

    let sim = sim::Simulation::init(config);



    sim.run();
    info!("allocating frame buffer");
    let frames = Box::<[FrameBuffer; FRAME_CACHE_SIZE]>::new_zeroed();
    let mut frames = unsafe { frames.assume_init() };
    info!("allocated frame buffer");
    for i in 0..FRAME_CACHE_SIZE {
        for j in 0..VGA_BUFFER_SIZE {
            set("clk", 1);
            set("en", 1);
            sim.run();

            // TODO: change to match pixel format
            let px = get_n_to_m("pixel", 0, 32);

            // let mut color = 0xFF_00_00_00;
            // for i in 0..3 {
            //     if px[i] == 1 {
            //         color |= 0xFF << (i*8);
            //     }
            // }
            // if px == 1 {
            //     color = 0xFF_FF_FF_FF;
            // }
            info!("pixel {}, {} added", i, j);
            frames[i][j] = to_u32(px);
        }
    }
    let (_, u8_frames, _) = unsafe { frames.align_to_mut::<u8>() };
    let mut enc = GzEncoder::new(Vec::new(), Compression::default());
    enc.write_all(u8_frames).unwrap();
    let res = enc.finish().unwrap();
    let mut file = File::create("frames.gz").expect("failed to create file");
    file.write_all(&res[..]).expect("saving to file failed");

}
