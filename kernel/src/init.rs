//  init.rs
//  this file originally belonged to baseOS project
//      on OS template on which to build


use ministd::{io, renderer::RENDERER};
use ministd::String;
use ministd::{print, println, kernel_panic};



fn init() -> Result<(), ()> {

    if let Err(_) = ministd::renderer::init() {
        kernel_panic!("failed to initialize renderer");
    }

    if let Err(_) = ministd::mem::heap::init() {
        kernel_panic!("failed to initialize heap");
    }

    println!("hello world!");

    Ok(())

}

#[unsafe(no_mangle)]
extern "C" fn _start() {

    io::int::disable();

    if let Err(_) = init() {
        ministd::kernel_panic!("failed to initialize the kernel");
    }

    ministd::hang();
}
