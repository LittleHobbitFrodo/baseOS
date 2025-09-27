//  mem/sync/mod.rs (ministd crate)
//  this file originally belonged to baseOS project
//      an OS template on which to build

#[cfg(all(feature="rc", feature="allocator"))]
mod arc;
#[cfg(all(feature="rc", feature="allocator"))]
pub use arc::Arc;

pub use spin::{Once, Lazy, Mutex, RwLock};