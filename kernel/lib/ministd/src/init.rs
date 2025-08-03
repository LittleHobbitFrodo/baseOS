//  init.rs (ministd crate)
//  this file originally belonged to baseOS project
//      on OS template on which to build


//  declares ministd initialization functions

#[cfg(all(feature="allocator", feature="spin"))]
use crate::mem::alloc;
#[cfg(feature = "renderer")]
use ::renderer::MinistdRenderer;

/// initializes allocator
#[cfg(all(feature="allocator", feature="spin"))]
#[inline]
pub fn allocator() -> Result<(), Option<&'static str>> {
    alloc::init()
}

#[cfg(feature="renderer")]
#[inline]
pub fn renderer() -> Result<(), ()> {
    crate::RENDERER.lock().init(&bootloader::FRAMEBUFFER)
}

