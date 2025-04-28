//	array.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build

// this file simplyfies array declaration and management

use core::mem::MaybeUninit;

pub const fn uninit<T: Sized, const LEN: usize>() -> [T; LEN] {
    unsafe { MaybeUninit::uninit().assume_init() }
}