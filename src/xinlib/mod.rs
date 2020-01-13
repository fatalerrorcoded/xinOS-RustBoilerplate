pub mod vesa;

mod c {
    extern "C" {
        pub fn wait(ms: u32);
        pub fn exit_to_main();
    }
}

pub fn wait(ms: u32) { unsafe { c::wait(ms); } }
pub fn exit_to_main() -> ! {
    unsafe { c::exit_to_main(); }
    loop {}
}

#[macro_export]
macro_rules! rgb888_to_rgb565 {
    ($x:expr) => {
        // The as u32 and as u16 are required as otherwise, the code complains about the input values not fitting in a u16
        ((($x as u32 & 0xf80000) >> 8) + (($x as u32 & 0xfc00) >> 5) + (($x as u32 & 0xf8) >> 3)) as u16
    };
}
