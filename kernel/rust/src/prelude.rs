//	prelude.rs
//	this file originally belonged to baseOS project
//		an OS template on which to build

//  this file is used as target in the util script

#![no_std]
#![no_main]
#![allow(static_mut_refs)]

//  used traits
pub use core::clone::Clone;
pub use core::marker::Copy;
//pub use core::convert::{Into, From, AsMut, AsRef};


use core::cell::UnsafeCell;
pub use core::mem::{align_of, size_of};
use core::ops::*;

pub mod bootloader;
pub mod font;
pub mod init;
pub mod io;
pub mod limine;
pub mod manage;
pub mod renderer;
pub mod mem;

pub mod convert;
pub mod sync;




use core::ops::{Deref, DerefMut};

pub struct Sync<T>(UnsafeCell<T>);
unsafe impl<T> core::marker::Sync for Sync<T> {}

impl<T> Sync<T> {
    pub const fn new(data: T) -> Self {
        Self(UnsafeCell::new(data))
    }
    #[inline(always)]
    pub fn borrow(&self) -> &T {
        unsafe { &*self.0.get() }
    }
    #[inline(always)]
    pub fn borrow_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}

pub type Stringy = [u8];

