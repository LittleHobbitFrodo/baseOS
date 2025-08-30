//  init.rs (ministd crate)
//  this file originally belonged to baseOS project
//      on OS template on which to build


//  declares ministd initialization functions

#[cfg(all(feature="allocator", feature="spin"))]
use crate::mem::alloc;
use crate::mem::kernel;
#[cfg(feature = "renderer")]
use ::renderer::MinistdRenderer;

/// initializes allocator (heap)
#[cfg(all(feature="allocator", feature="spin"))]
#[inline]
pub fn allocator() -> Result<(), Option<&'static str>> {
    alloc::init()
}

/// initializes renderer
/// - needed to print text to the screen
#[cfg(feature="renderer")]
#[inline]
pub fn renderer() -> Result<(), ()> {
    crate::RENDERER.lock().init(&bootloader::FRAMEBUFFER)
}

/// initializes metadata about kernel memory layout
/// - available in the `ministd::mem::kernel` module
pub fn memory() {
    kernel::LAYOUT.write().init();
}

