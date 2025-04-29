//	lib.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build

#![no_std]
#![no_main]
//#![deny(static_mut_refs)]


/// # MINISTD crate
/// This crate mimics basic functionalities of the STD crate  
/// Each functionality that provides [`init()`] function is meant to be initialized manually in your kernel [`init()`] function
/// 
/// PS: bootloader requests are done in the [`bootloader`] local crate



//  used modules
pub mod mem;
pub mod renderer;
#[macro_use]
pub mod io;
pub mod string;
pub mod convert;
pub mod array;


use renderer::RENDERER;
pub use string::String;
pub use array::*;


//  local crates
pub use bootloader;
pub use limine_rs as limine;
pub use buddy_system_allocator as allocator;
pub use spin;


pub use spin::{Mutex, MutexGuard,
    RwLock, RwLockReadGuard, RwLockWriteGuard, RwLockUpgradableGuard,
    Lazy, Barrier, Once};

use core::arch::asm;
use core::hint::spin_loop;


pub fn hang() -> ! {
    loop {
        io::int::disable();
        unsafe { asm!("hlt"); }
        spin_loop();
    }
}

unsafe extern "C" {
    fn panic_handler(msg: &str);
}

pub fn panic(msg: &str) -> ! {

    unsafe { RENDERER.force_unlock() };
    unsafe { panic_handler(msg); }

    hang();
}



#[macro_export]
macro_rules! kernel_panic {
    ($msg:literal) => {{
        use super::ministd::{panic, print, println};
        unsafe { RENDERER.force_unlock() };
        println!("Panic occured at {}:{}", file!(), line!());
        panic($msg)
        }};
}