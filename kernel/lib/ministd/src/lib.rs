//	lib.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build

#![no_std]

#![no_main]

use core::ops::Deref;
/// # MINISTD crate
/// This crate mimics basic functionalities of the STD crate  
/// Each functionality that provides [`init()`] function is meant to be initialized manually in your kernel [`init()`] function
/// 
/// PS: bootloader requests are done in the [`bootloader`] local crate


pub use core::pin::Pin;
//pub use core::intrinsics::{unlikely, likely};


//  used modules
pub mod mem;
#[cfg(feature = "renderer")]
pub mod renderer;
#[macro_use]
pub mod io;
pub mod convert;
pub mod init;

//  modules
#[cfg(feature="renderer")]
pub use renderer::{RENDERER, Color};

#[cfg(all(feature="string", feature="allocator", feature="spin"))]
pub use mem::string::{self, String};

#[cfg(all(feature="vector", feature="allocator", feature="spin"))]
pub use mem::vec::{self, Vec};

#[cfg(all(feature="box", feature="allocator", feature="spin"))]
pub use mem::boxed::Box;
#[cfg(all(feature="box", feature="allocator", feature="spin"))]
pub use mem::array::Array;

#[cfg(all(feature="allocator", feature="spin"))]
pub use mem::alloc::{self, ALLOCATOR, Allocator};

#[cfg(all(feature="rc", feature="allocator", feature="spin"))]
pub use mem::rc::Rc;

//  local crates
pub use bootloader;
pub use limine_rs as limine;
#[cfg(all(feature="allocator", feature="spin"))]
pub use buddy_system_allocator as allocator;
#[cfg(feature="spin")]
pub use spin;
use proc_macro;

pub use proc_macro::{entry, oom};

//  remote crates
#[cfg(all(feature="allocator", feature="spin", feature="hashmap"))]
pub use hashbrown;
pub mod assert {
    pub use static_assertions::*;
}

#[cfg(feature="spin")]
pub use spin::{Mutex, MutexGuard,
    RwLock, RwLockReadGuard, RwLockWriteGuard, RwLockUpgradableGuard,
    Lazy, Barrier, Once};

#[cfg(all(feature="hashmap", feature="allocator", feature="spin"))]
pub use hashbrown::{HashMap, HashSet, HashTable};

use core::arch::asm;
use core::hint::spin_loop;
pub use core::convert::{Infallible, From, TryFrom, Into, TryInto};



pub type HeapRef<'l> = crate::MutexGuard<'l, crate::alloc::Heap>;

pub fn hang() -> ! {
    loop {
        io::int::disable();
        unsafe { asm!("hlt"); }
        spin_loop();
    }
}


/// Allows cloning if failure is possible
pub trait TryClone {
    type Error;
    fn try_clone(&self) -> Result<Self, Self::Error>
    where Self: Sized;
}

/// # Nothing
/// 
/// This structure represents ..., well ... nothing  
/// 
/// Usage:
/// - No data while returning `Err` but still needs to be constructed
#[derive(Copy, Clone)]
pub struct Nothing();

impl Default for Nothing {
    #[inline(always)]
    fn default() -> Self {
        Nothing()
    }
}



/// structure used for sigle-threaded immutable data access
pub struct Immutable<T: Sized> {
    data: T,
}

impl<T: Sized> Immutable<T> {
    pub const fn new(val: T) -> Self {
        Self {
            data: val,
        }
    }
}

impl<T: Sized> Deref for Immutable<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[cfg(all(feature="allocator", feature="spin", feature="string"))]
pub static PANIC_FMT_MSG: RwLock<Option<&'static str>> = RwLock::new(None);

/// Makes support for formatted panic messages possible
#[cfg(all(feature="allocator", feature="spin", feature="string"))]
#[macro_export]
macro_rules! panic_fmt {
    ($($arg:tt)*) => {{

        use core::fmt::Write;
        use $crate::String;

        let mut msg: String = String::with_capacity(64);

        if let Err(_) = write!(&mut msg, $($arg)*) {
            panic!();
        }

        *$crate::PANIC_FMT_MSG.write() = Some(msg.leak());

        panic!();
        

    }};

    () => {
        panic!();
    }
}