//	sync.rs
//	this file originally belonged to baseOS project
//		an OS template on which to build

/// offers locking mechanisms for multithreaded data access

/// RwLock
/// Mutex

use core::cell::UnsafeCell;
use core::sync::atomic::AtomicBool;


pub struct Mutex<T: Sized + Sync> {
    data: UnsafeCell<T>,
    flags: MutexFlags,
    lock: AtomicBool,
}

/// # Mutex is locking wrapper taken from the rust standard library
/// source: https://github.com/rust-lang/rust/blob/master/library/std/src/sync/poison/mutex.rs
/// ### this specific implementation is not using poisoning
/// - if thread crashed the state of variable is set to `Broken` and needs to be fixed with `.repair` function

impl<T> Mutex<T> {
    #[inline]
    pub const fn new(data: T) -> Self {
        Self { data: UnsafeCell::new(data), flags: MutexFlags::new(), lock: AtomicBool::new(false)}
    }
    pub fn get_cloned(&mut self) -> Result<T, ()>
    where T: Clone {
        match self.lock.get_mut() {
            false => Err(()),
            true => {
                Ok(self.data.get().clone())
            }
        }
    }

    pub fn set(&self, value: T) -> Result<(), ()> {

    }

}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

struct MutexGuard<'l, T: Sized + 'l> {
    lock: &'l Mutex<T>,
}

impl<T: ?Sized> !Send for MutexGuard<'_, T> {}
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}



pub struct MutexFlags(u64);
/// flags for Mutex wrapper


impl MutexFlags {
    const RC_MASK: u64 = 0x1FFFFFFFFFFFFFFF;
    pub const fn new() -> Self {
        Self(0)
    }

    /// flag access
    #[inline(always)]
    pub fn flag_set(&mut self, flag: Flags) {
        self.0 |= 1 << flag as u64;
    }
    #[inline(always)]
    pub fn flag_clear(&mut self, flag: Flags) {
        self.0 &= !(1 << flag as u64)
    }

    #[inline(always)]
    pub fn rc(&self) -> u64 {
        self.0 & Self::RC_MASK
    }
    #[inline(always)]
    pub fn rc_set(&mut self, rc: u64) {
        self.0 &= !Self::RC_MASK;
        self.0 |= rc & Self::RC_MASK;
    }
    #[inline(always)]
    pub fn rc_clear(&mut self) {
        self.0 &= !Self::RC_MASK;
    }
}

impl core::ops::AddAssign<u64> for MutexFlags {
    #[inline]
    fn add_assign(&mut self, add: u64) {
        let mut rc = self.rc();
        rc += add;
        self.rc_set(rc);
    }
}

impl core::ops::SubAssign<u64> for MutexFlags {
    #[inline]
    fn sub_assign(&mut self, sub: u64) {
        let mut rc = self.rc();
        rc -= sub;
        self.rc_set(rc);
    }
}

enum Flags {
    MutRef = 61,
    State = 63 | 62,
}



