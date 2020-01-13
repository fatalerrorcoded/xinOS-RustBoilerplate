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
