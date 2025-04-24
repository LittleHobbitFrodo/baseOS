//  init.rs
//  this file originally belonged to baseOS project
//      on OS template on which to build

use crate::bootloader;
//use crate::cell::SyncCell;
use crate::io;
use crate::manage::{hang};
use ministd::renderer::RENDERER;



fn init() {

    if let Ok(mut rend) = RENDERER.try_lock() {
        rend.init(&bootloader::FRAMEBUFFER);
        rend.printstr(b"hello world!\n");
    }

    RENDERER.lock().printstr(b"hello world!\n");

    panic!("panic");

}

#[unsafe(no_mangle)]
extern "C" fn _start() {

    io::int::disable();

    init();

    hang();
}
