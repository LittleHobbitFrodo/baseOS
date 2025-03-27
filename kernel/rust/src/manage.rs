//	manage.rs
//	this file originally belonged to baseOS project
//		an OS template on which to build

use core::arch::asm;
use crate::io;

pub extern "C" fn hang() -> !{
    io::int::disable();
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}


#[inline(always)]
pub fn wait() {
    /// spin-loop: tells cpu that it is in loop
    core::hint::spin_loop();
}