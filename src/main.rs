#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

extern crate alloc;

use bootloader::boot_info::FrameBuffer;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use untitled_os_lib::{memory, memory::allocator, println};
use x86_64::VirtAddr;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    untitled_os_lib::init();

    println!(" _    _       _   _ _   _          _  ____   _____ ");
    println!("| |  | |     | | (_) | | |        | |/ __ \\ / ____|");
    println!("| |  | |_ __ | |_ _| |_| | ___  __| | |  | | (___  ");
    println!("| |  | | '_ \\| __| | __| |/ _ \\/ _` | |  | |\\___ \\ ");
    println!("| |__| | | | | |_| | |_| |  __/ (_| | |__| |____) |");
    println!(" \\____/|_| |_|\\__|_|\\__|_|\\___|\\__,_|\\____/|_____/ \n");

    let mut total_mem = 0;

    for i in 0..boot_info.memory_regions.len() {
        total_mem += (*boot_info.memory_regions)[i].end - (*boot_info.memory_regions)[i].start;
    }

    println!("Ram detected: {}MB", total_mem / 1024 / 1024 + 1);

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();

        for x in 0..info.horizontal_resolution {
            for y in 0..info.vertical_resolution {
                put_pixel(x, y, framebuffer, Rgb { r: 0, g: 0, b: 0 });
            }
        }
    }

    untitled_os_lib::hlt_loop();
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

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
