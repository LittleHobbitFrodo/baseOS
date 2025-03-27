//  init.rs
//  this file originally belonged to baseOS project
//      on OS template on which to build

use core::cell::RefCell;
use core::panic::PanicInfo;
use crate::bootloader;
//use crate::cell::SyncCell;
use crate::io;
use crate::manage::hang;
use crate::renderer::RENDERER;

#[macro_use]


fn init() {

    let mut rend = unsafe {RENDERER.borrow_mut()};


}

#[unsafe(no_mangle)]
extern "C" fn _start() {

    io::int::disable();

    init();

    hang();
}

pub fn panic(code: PanicCode, msg: &[u8]) -> ! {
    let rend = match RENDERER.acquire_mut() {
        Some(rend) => rend,
        None => {
            hang();
        }
    };
    rend.set_color_rgb(255, 0, 0);

    match code {
        PanicCode::Unknown => {
            rend.printstr(b"UNKNOWN ERROR: PANIC");
        },
        _ => {
            rend.printstr(b"ERROR: PANIC caused by ");
            rend.set_color(0xffffff);
            rend.printstr(msg);
            rend.endl();

            /*match code {
                /// more info
            }*/

            rend.endl();
            rend.endl();
        }
    }

    rend.set_color_rgb(255, 0, 0);
    rend.printstr(b"halting the system");
    hang();
}

pub enum PanicCode {
    Unknown,
}

#[panic_handler]
pub fn _panic(_info: &PanicInfo) -> ! {

    panic(PanicCode::Unknown, b"unknown error");

}
