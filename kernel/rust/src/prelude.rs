//	prelude.rs
//	this file originally belonged to baseOS project
//		an OS template on which to build

//  this file is used as target in the util script

#![no_std]
#![no_main]

//  used traits
pub use core::clone::Clone;
pub use core::marker::Copy;


pub use core::mem::{align_of, size_of};

pub mod font;
pub mod init;
pub mod io;
pub mod manage;


//  local crates
pub use ministd;
pub use limine_rs as limine;
pub use bootloader;


