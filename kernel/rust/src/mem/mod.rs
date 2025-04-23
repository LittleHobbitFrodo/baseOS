//	memory/mod.rs
//	this file originally belonged to baseOS project
//		an OS template on which to build

/// this file provides basic memory-related functionalities


pub const KB: usize = 1024;
pub const MB: usize = 1024 * 1024;
pub const GB: usize = 1024 * 1024 * 1024;




pub use core::mem::needs_drop;