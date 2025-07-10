//  init.rs
//  this file originally belonged to baseOS project
//      on OS template on which to build

//  module for kernel initialization

use ministd::{dbg, io, panic_fmt, Rc};
use ministd::{println, print, locked_println, eprintln, init};
use ministd::{Box, Array, Vec, String, HashMap};

use crate::manage::*;


/// This function is here to initialize your kernel
/// 
/// you can initialize various kernel functions and submodules here
fn init() -> Result<(), ()> {

    if let Err(_) = init::renderer() {
        panic!("failed to initialize renderer");
    }

    if let Err(msg) = init::allocator() {
        if let Some(msg) = msg {
            panic_fmt!("failed to initialize heap: {msg}")
        } else {
            panic!("failed to initialize heap");
        }
    }

    println!("hello world!");

    //let rc = Rc::new(69 as usize);

    Ok(())

}

/// This is the first function that is called by the bootloader
/// 
/// Make sure to put your initialization function here :)
#[unsafe(no_mangle)]
extern "C" fn _start() {

    io::int::disable();

    if let Err(_) = init() {
        panic!("failed to initialize the kernel");
    }

    ministd::hang();
}
