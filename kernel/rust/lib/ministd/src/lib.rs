//	lib.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build

#![no_std]
#![no_main]

pub mod mem;

pub mod sync;
pub mod convert;

pub mod renderer;

pub use bootloader;
pub use limine_rs;
