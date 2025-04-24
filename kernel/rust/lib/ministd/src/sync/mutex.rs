//	mutex.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build

/// offers locking mechanisms for multithreaded data access

/// RwLock
/// Mutex => std::sync::Mutex<T> like structure

use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::mem;
use core::ops::BitAnd;
use core::ops::Deref;
use core::ops::DerefMut;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;



pub struct Mutex<T: Sized> {
    data: UnsafeCell<T>,
    //flags: MutexFlags,
    lock: AtomicBool,
}

//unsafe impl<T: Sized> Send for Mutex<T> {}
//unsafe impl<T: Sized> Sync for Mutex<T> {}
unsafe impl<T: Sized + Send> Send for Mutex<T> {}
unsafe impl<T: Sized + Send> Sync for Mutex<T> {}

/// # Mutex is locking wrapper taken from the rust standard library
/// source: https://github.com/rust-lang/rust/blob/master/library/std/src/sync/poison/mutex.rs
/// ### this specific implementation is not using poisoning



impl<T: Sized> Mutex<T> {
    #[inline]
    pub const fn new(data: T) -> Self {
        Self { data: UnsafeCell::new(data), lock: AtomicBool::new(false), /*flags: MutexFlags::new()*/}
    }
    pub fn get_cloned(&mut self) -> Result<T, ()>
    where T: Copy {
        match self.lock.get_mut() {
            false => Err(()),
            true => {
                Ok(unsafe {*self.data.get()}.clone())
            }
        }
    }



    pub fn set(&mut self, value: T) -> Result<(), ()> {
        if mem::needs_drop::<T>() {
            self.replace(value).map(drop);
            Ok(())
        } else {
            match self.inner_lock_mut() {
                Ok(data) => {
                    *data = value;
                    self.unlock();
                    Ok(())
                },
                Err(_) => Err(())
            }
        }
    }

    pub fn replace(&mut self, value: T) -> Result<(), ()> {
        match self.inner_lock_mut() {
            Ok(data) => {
                *data = value;
                Ok(())
            },
            Err(_) => Err(())
        }
    }

    /*#[inline(always)]
    pub fn is_poisoned(&self) -> bool {
        self.flags.is_poisoned()
    }*/

    //  implementation functions

    #[inline(always)]
    fn is_locked(&mut self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }
    
    fn unlock(&mut self) -> Result<(), ()> {
        match self.lock.compare_exchange(true, false, Ordering::Release, Ordering::Relaxed) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }

    fn inner_lock(&mut self) -> Result<&T, ()> {
        match self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => Ok(unsafe {self.data.get().as_ref().unwrap()}),
            Err(_) => Err(())
        }
    }
    fn inner_lock_mut(&mut self) -> Result<&mut T, ()> {
        match self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => Ok(unsafe {self.data.get().as_mut().unwrap()}),
            Err(_) => Err(())
        }
    }
    const fn get_ptr<'l>(&'l self) -> *const T {
        self.data.get()
    }
    fn inner_get_ref(&self) -> &T {
        unsafe {self.data.get().as_ref().unwrap()}
    }
    fn inner_get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }

}

impl<T: Sized + Sync> Mutex<T> {
    /// # lock
    /// - blocks thread until the mutex is unlocked
    #[inline]
    pub fn lock(&self) -> MutexGuard<'_, T>
    where T: Send {
        
        while self.lock.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        
        MutexGuard::new(self)
    }

    /// # try_lock
    /// - attempts to acquire lock
    #[inline]
    pub fn try_lock(&self) -> Result<MutexGuard<'_, T>, ()> {

        match self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => Ok(MutexGuard::new(self)),
            Err(_) => Err(()),
        }
    }
}


impl<T: Sized> Drop for Mutex<T> {
    fn drop(&mut self) {
        if mem::needs_drop::<T>() {
            //  drop T here
            unsafe {core::ptr::drop_in_place(self.data.get());}
        }
    }
}







#[must_use="if MutexGuard is unused its Mutex will immediately unlock"]
pub struct MutexGuard<'mux, T: Sized + Sync + 'mux> {
    lock: &'mux Mutex<T>,
    data: &'mux UnsafeCell<T>,
}

impl<'mux, T: Sized + Sync> MutexGuard<'mux, T> {
    const fn new(mutex:  &'mux Mutex<T>) -> Self {
        Self {
            lock: mutex,
            data: &mutex.data,
        }
    }

    unsafe fn get_mutex_mut(&self) -> &'mux mut Mutex<T> {
        unsafe { {self.lock as *const Mutex<T> as *mut Mutex<T>}.as_mut().unwrap()}
    }
}

impl<'l, T: Sized + Sync> Drop for MutexGuard<'_, T> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {self.get_mutex_mut()}.unlock();
    }
}


impl<'mux, T: Sized + Sync> Deref for MutexGuard<'mux, T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe {self.data.get().as_ref()}.unwrap()
    }
}

impl<'mux, T: Sized + Sync> DerefMut for MutexGuard<'mux, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {self.data.get().as_mut()}.unwrap()
    }
}





/*

/// # Bits
/// 7th = borrowed as mut  
/// 6th + 5th = state
///   - 0 = OK
///   - 1 = broken
///   - 2 = poisoned
struct MutexFlags(u8);
enum MutexFlagState {
    Ok,
    Broken,
    Poisoned,
    Unknown
}

impl MutexFlags {
    pub const FLAG_STATE: u8 = 0b11 << 5;
    pub const fn new() -> Self {
        Self { 0: 0 }
    }


    #[inline(always)]
    pub fn state(&self) -> MutexFlagState {
        match self.0.bitand(Self::FLAG_STATE) {
            0 => MutexFlagState::Ok,
            1 => MutexFlagState::Broken,
            2 => MutexFlagState::Poisoned,
            _ => MutexFlagState::Unknown
        }
    }

    #[inline(always)]
    pub fn make_broken(&mut self) -> Result<(),()>{
        if self.is_poisoned() {
            return Err(());
        }
        self.0 &= !Self::FLAG_STATE;
        self.0 |= 1 << 5;
        Ok(())
    }

    #[inline(always)]
    pub fn fix(&mut self) -> Result<(),()> {
        if (self.is_broken()) {
            return Err(());
        }
        self.0 &= !Self::FLAG_STATE;
        Ok(())
    }

    #[inline(always)]
    pub fn is_poisoned(&self) -> bool {
        if let MutexFlagState::Poisoned = self.state() {
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn is_broken(&self) -> bool {
        if let MutexFlagState::Broken = self.state() {
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        if let MutexFlagState::Ok = self.state() {
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn make_poisoned(&mut self) {
        self.0 &= !Self::FLAG_STATE;
        self.0 |= 2 << 5;
    }

}*/