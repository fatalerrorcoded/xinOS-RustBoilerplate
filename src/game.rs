use crate::xinlib::{wait, exit_to_main};

#[no_mangle]
pub extern "C" fn yourGame_main() -> ! {
    wait(5000);
    exit_to_main();
}
