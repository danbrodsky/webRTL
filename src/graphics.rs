// TODO: drawing can be parallelized, but this is not a priority
// TODO: each sim run should return the next frame to be displayed
// TODO: sim run should be parallelized
// TODO: having to async schedule sim runs from js will result in perf loss

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
use core::sync::atomic::{ AtomicUsize, Ordering};
use crate::util::*;

const VGA_WIDTH: usize = 320;
const VGA_HEIGHT: usize = 10;
pub const VGA_BUFFER_SIZE: usize = VGA_WIDTH * VGA_HEIGHT;
pub const FRAME_CACHE_SIZE: usize = 5;

pub type FrameBuffer = [u32; VGA_BUFFER_SIZE];

pub static FRAME: AtomicUsize = AtomicUsize::new(0);
pub static mut BUFFER: FrameBuffer = [0; VGA_BUFFER_SIZE];
pub static mut FRAME_CACHE: [FrameBuffer; FRAME_CACHE_SIZE] = [[0; VGA_BUFFER_SIZE]; FRAME_CACHE_SIZE];

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(window: &web_sys::Window, f: &Closure<dyn FnMut()>) -> i32 {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

pub fn set_timeout(window: &web_sys::Window, f: &Closure<dyn FnMut()>, timeout_ms: i32) -> i32 {
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            timeout_ms,
        )
        .expect("should register `setTimeout` OK")
}


// TODO: change buffer to be passed in so no need for unsafe
pub unsafe fn test_render() {
    let f = FRAME.fetch_add(1, Ordering::Relaxed) as usize;
    let px = get_n_to_m("pixel", 0, 4);

    let mut color = 0xFF_00_00_00;
    for i in 0..3 {
        if px[i] == 1 {
            color |= 0xFF << (i*8);
        }
    }
    // warn!("{:#?}", color);
    BUFFER[f] = color;
    BUFFER[f+1] = color;

    FRAME.compare_and_swap(VGA_BUFFER_SIZE, 0, Ordering::Relaxed);

    // for y in 0..vga_height {
    //     for x in 0..vga_width {
    //         buffer[y * vga_width + x] = color
    //             // f.wrapping_add((x^y) as u32) | 0xff_00_00_00;
    //     }
    // }
}

pub fn render_next() {
    let f = FRAME.fetch_add(1, Ordering::Relaxed) as usize;
    for y in 0..VGA_HEIGHT {
        for x in 0..VGA_WIDTH {
            unsafe {
            BUFFER[y * VGA_WIDTH + x] = FRAME_CACHE[f][y * VGA_WIDTH + x]
            };
        }
    }

    FRAME.compare_and_swap(FRAME_CACHE_SIZE, 0, Ordering::Relaxed);

}

pub fn draw(
    ctx: &CanvasRenderingContext2d
) -> Result<(), JsValue> {
    // this is always safe since u32 is always u8 aligned
    let (_, u8_buf, _) = unsafe {BUFFER.align_to_mut::<u8>()};
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(u8_buf), VGA_WIDTH as u32, VGA_HEIGHT as u32)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}
