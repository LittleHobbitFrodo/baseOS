//  init.rs
//  this file originally belonged to baseOS project
//      on OS template on which to build

use core::alloc::Layout;

use crate::bootloader;
//use crate::cell::SyncCell;
use ministd::{io, renderer::RENDERER};
use crate::manage::panic;
use ministd::mem::heap;


fn init() -> Result<(), ()> {

    if let Err(_) = ministd::renderer::init() {
        panic(b"failed to initialize renderer");
    }

    if let Err(_) = ministd::mem::heap::init() {
        panic(b"failed to initialize heap");
    }

    let mut rend = RENDERER.lock();
    rend.println(b"testing heap: ");
    
    //  test allocation
    if let Ok(alloc) = heap::HEAP.lock().alloc(Layout::from_size_align(8, 8).expect("heehee")) {
        let mut arr: &mut [u8] = unsafe { core::slice::from_raw_parts_mut(alloc.as_ptr(), 8) };
        let data = "deadbeef".as_bytes();
        for i in 0..8 {
            arr[i] = data[i];
        }

        rend.print(b"array: ");
        rend.println(arr);
        heap::HEAP.lock().dealloc(alloc, Layout::from_size_align(8, 8).expect("heehee"));
    }



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
