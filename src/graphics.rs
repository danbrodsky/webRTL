// TODO: drawing can be parallelized, but this is not a priority
// TODO: each sim run should return the next frame to be displayed
// TODO: sim run should be parallelized
// TODO: having to async schedule sim runs from js will result in perf loss

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
use core::sync::atomic::{AtomicU32, Ordering};

const VGA_WIDTH: usize = 640;
const VGA_HEIGHT: usize = 480;
const VGA_BUFFER_SIZE: usize = VGA_WIDTH * VGA_HEIGHT;


pub static FRAME: AtomicU32 = AtomicU32::new(0);
pub static mut BUFFER: [u32; VGA_BUFFER_SIZE] = [0; VGA_BUFFER_SIZE];

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("global window not found")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("requestAnimationFrame failed to register");
}


// this is safe since buffer size is always within modified bounds
pub unsafe fn test_render() {
    let f = FRAME.fetch_add(1, Ordering::Relaxed);

    for y in 0..VGA_HEIGHT {
        for x in 0..VGA_WIDTH {
            BUFFER[y * VGA_WIDTH + x] =
                f.wrapping_add((x^y) as u32) | 0xFF_00_00_00;
        }
    }
}

pub fn draw(
    ctx: &CanvasRenderingContext2d
) -> Result<(), JsValue> {
    // this is always safe since u32 is always u8 aligned
    let (_, u8_buf, _) = unsafe {BUFFER.align_to_mut::<u8>()};
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(u8_buf), VGA_WIDTH as u32, VGA_HEIGHT as u32)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}
