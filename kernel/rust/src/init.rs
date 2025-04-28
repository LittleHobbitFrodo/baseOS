//  init.rs
//  this file originally belonged to baseOS project
//      on OS template on which to build


use ministd::{io, renderer::RENDERER};
use crate::manage::panic;
use ministd::String;
use ministd::{print, println};



fn init() -> Result<(), ()> {

    if let Err(_) = ministd::renderer::init() {
        panic(b"failed to initialize renderer");
    }

    if let Err(_) = ministd::mem::heap::init() {
        panic(b"failed to initialize heap");
    }

    

    let string = match String::from_str(b"hello world!") {
        Ok(s) => s,
        Err(_) => {
            return Err(());
        },
    };

    println!("string: {}", string);

    Ok(())

}

#[unsafe(no_mangle)]
extern "C" fn _start() {

    io::int::disable();

    if let Err(_) = init() {
        panic(b"failed to initialize the kernel");
    }

    ministd::hang();
}
