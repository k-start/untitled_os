#![no_std]
#![no_main]
#![feature(llvm_asm)]

mod print;

use bootloader::boot_info::FrameBuffer;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        println!("{:?}", framebuffer.info());

        for x in 0..info.horizontal_resolution {
            for y in 0..info.vertical_resolution {
                put_pixel(x, y, framebuffer, Rgb { r: 0, g: 0, b: 0 });
            }
        }
    }

    loop {}
}

struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn put_pixel(x: usize, y: usize, framebuffer: &mut FrameBuffer, colour: Rgb) {
    let info = framebuffer.info();

    let location = ((y * info.stride) + x) * info.bytes_per_pixel;

    framebuffer.buffer_mut()[location] = colour.b;
    framebuffer.buffer_mut()[location + 1] = colour.g;
    framebuffer.buffer_mut()[location + 2] = colour.r;
    framebuffer.buffer_mut()[location + 3] = 0;
}