//  init.rs (ministd crate)
//  this file originally belonged to baseOS project
//      on OS template on which to build


//  declares ministd initialization functions

use crate::mem::alloc;
use crate::renderer;

/// initializes allocator
#[inline]
pub fn allocator() -> Result<(), Option<&'static str>> {
    alloc::init()
}

#[inline]
pub fn renderer() -> Result<(), ()> {
    renderer::init()
}