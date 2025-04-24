//	mutex.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build

use core::cell::UnsafeCell;
use core::hint::spin_loop;
use core::mem;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, AtomicUsize};
use core::ops::{Deref, DerefMut};
use core::sync::atomic::Ordering;

pub struct RwLock<T: Sized> {
    data: UnsafeCell<T>,
    writer: AtomicBool,
    readers: AtomicUsize,
}

unsafe impl<T: Sized + Send> Send for RwLock<T> {}
unsafe impl<T: Sized + Send + Sync> Sync for RwLock<T> {} 



impl<T: Sized> RwLock<T> {
    pub const fn new(t: T) -> Self {
        Self {
            data: UnsafeCell::new(t),
            writer: AtomicBool::new(false),
            readers: AtomicUsize::new(0)
        }
    }


    #[inline]
    pub fn get_cloned(&self) -> T
    where T: Clone {
        self.read().clone()
    }

    pub fn set(&self, value: T) -> Result<(), ()> {
        if mem::needs_drop::<T>() {
            drop(self.replace(value));
            Ok(())
        } else {
            *self.write() = value;
            Ok(())
        }
    }

    #[inline]
    pub fn replace(&self, value: T) -> T {
        mem::replace(&mut *self.write(), value)
    }





    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.add_reader();
        RwLockReadGuard::new(self)
    }


    pub fn try_read(&self) -> Result<RwLockReadGuard<'_, T>, ()> {
        if let Ok(_) = self.try_add_reader() {
            Ok(RwLockReadGuard::new(self))
        } else {
            Err(())
        }
    }


    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.acquire_writer();
        RwLockWriteGuard::new(self)
    }

    pub fn try_write(&self) -> Result<RwLockWriteGuard<T>, ()> {
        if let Ok(_) = self.try_acquire_writer() {
            return Ok(RwLockWriteGuard::new(self));
        }
        Err(())
    }

}

impl<T: Sized> RwLock<T> {
    #[inline(always)]
    fn writer(&self) -> bool {
        self.writer.load(Ordering::Relaxed)
    }
    #[inline(always)]
    fn rc(&self) -> usize {
        self.readers.load(Ordering::Relaxed)
    }

    /// will block this thread
    #[inline]
    fn acquire_writer(&self) {
        loop {
            if !self.writer.load(Ordering::Acquire) && self.readers.load(Ordering::Acquire) == 0 {
                if self.writer.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                    break;
                }
            }
            spin_loop();
        }
    }

    #[inline]
    fn try_acquire_writer(&self) -> Result<(), ()> {
        if !self.writer.load(Ordering::Acquire) && self.readers.load(Ordering::Acquire) == 0 {
            if let Ok(_) = self.writer.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
                return Ok(());
            }
        }
        Err(())
    }


    ///  add_reader will block this thread
    #[inline]
    fn add_reader(&self) {
        loop {
            if !self.writer.load(Ordering::Acquire) {
                self.readers.fetch_add(1, Ordering::Acquire);
                if !self.writer.load(Ordering::Acquire) {
                    break;
                }
                self.readers.fetch_sub(1, Ordering::Release);
            }
            spin_loop();
        }
    }

    #[inline]
    fn try_add_reader(&self) -> Result<(), ()> {
        if !self.writer.load(Ordering::Acquire) {
            self.readers.fetch_add(1, Ordering::Acquire);
            if !self.writer.load(Ordering::Acquire) {
                return Err(());
            }
            self.readers.fetch_sub(1, Ordering::Release);
        }
        Ok(())
    }

}









pub struct RwLockReadGuard<'rl, T: Sized + 'rl> {
    lock: &'rl RwLock<T>,
    data: NonNull<T>,
}

impl<'rl, T: Sized + 'rl> Deref for RwLockReadGuard<'rl, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {self.data.as_ref() }
    }
}

impl<'rl, T: Sized> RwLockReadGuard<'rl, T> {
    const fn new(lock: &'rl RwLock<T>) -> Self {
        Self {
            lock,
            data: unsafe { NonNull::new_unchecked(lock.data.get()) }
        }
    }
}


pub struct RwLockWriteGuard<'rl, T: Sized + 'rl> {
    lock: &'rl RwLock<T>,
    data: NonNull<T>,
}

impl<'rl, T: Sized + 'rl> Deref for RwLockWriteGuard<'rl, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ref() }
    }
}

impl<'rl, T: Sized + 'rl> DerefMut for RwLockWriteGuard<'rl, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.data.as_mut() }
    }
}

impl<'rl, T: Sized> RwLockWriteGuard<'rl, T> {
    const fn new(lock: &'rl RwLock<T>) -> Self {
        Self {
            lock,
            data: unsafe { NonNull::new_unchecked(lock.data.get()) }
        }
    }
}