#![no_std]
#![no_main]

extern crate efw;
extern crate font8x8;

mod framebuffer;
mod console;

use efw::*;

// Trait imports.
use efw::efi::Protocol;
use core::fmt::Write;

#[no_mangle]
unsafe fn efw_main() {
    // Get access to the framebuffer.
    let mut framebuffer = {
        let protocol = &mut efi::protocols::GraphicsOutput::find_instances().unwrap()[0];
        let mode = protocol.mode();
        let info = &mut *mode.info;
        let addr = mode.frame_buffer_base;
        framebuffer::Framebuffer::new(
            addr as _,
            info.horizontal_resolution as _,
            info.vertical_resolution as _,
            info.pixels_per_scan_line as _
        )
    };

    println!("Exiting boot services");
    // Exit boot services.
    let boot_services = efi::SystemTable::get().boot_services();
    let mem_map = boot_services.get_memory_map().unwrap();
    boot_services.exit_boot_services(mem_map.key()).unwrap();

    framebuffer.fill(0, 0, 0);
    let mut console = console::Console::new(framebuffer);

    writeln!(console, "Hello World!").unwrap();
    writeln!(console, "Next line!").unwrap();

    loop {}
}
