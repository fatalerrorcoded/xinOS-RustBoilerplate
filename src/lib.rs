#![no_std]

pub mod xinlib;
mod game;

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
