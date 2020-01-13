use crate::rgb888_to_rgb565;
use crate::xinlib::{wait, exit_to_main};
use crate::xinlib::vesa::{draw_background, draw_str};

#[no_mangle]
pub extern "C" fn yourGame_main() -> ! {
    draw_background(rgb888_to_rgb565!(0x553333));
    draw_str("Hello xinOS from Rust!", 16, 16, rgb888_to_rgb565!(0xffffff));

    wait(5000);
    exit_to_main();
}
