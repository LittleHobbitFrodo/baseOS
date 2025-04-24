//	sync/mod.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build


pub mod mutex;
pub use mutex::Mutex;

pub mod rwlock;
pub use rwlock::RwLock;

pub mod rosync;
pub use rosync::RoSync;