//	prelude.rs
//	this file originally belonged to baseOS project
//		an OS template on which to build

//  this file is used as target in the util script

#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#[macro_use]

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
pub mod memory;

pub mod convert;
pub mod sync;

/// integer traits
trait Integer: Sized + Copy + PartialEq + PartialOrd + Add<Output=Self>
+ AddAssign + Sub<Output=Self> + Not<Output=Self> + BitAnd<Output=Self> + BitOr<Output=Self>
+ BitXor<Output=Self> + Div<Output=Self> + Mul<Output=Self> + Shl<Output=Self> + Shr<Output=Self> {}

impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for usize {}
impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for isize {}


trait Unsigned: Integer {}
impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for usize {}


trait Signed: Integer {}
impl Signed for i8 {}
impl Signed for i16 {}
impl Signed for i32 {}
impl Signed for i64 {}
impl Signed for isize {}



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

