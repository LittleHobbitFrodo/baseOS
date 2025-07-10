//	mem/weak.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build


use core::{ptr::NonNull, marker::PhantomData, cell::Cell, ptr::drop_in_place};
use crate::ALLOCATOR;

use crate::{mem::rc::rc_inner::RcInner, Rc};

/// A weak single-threaded reference-counting pointer
pub struct Weak<T: Sized> {
    data: NonNull<RcInner<T>>,
    _not_sync_not_send: PhantomData<Cell<()>>,    //  for !Send
}

impl<T> Weak<T> {

    pub(crate) fn new(inner: &mut RcInner<T>) -> Self {
        inner.inc_weak();
        Self {
            data: unsafe { NonNull::new_unchecked(inner.as_ptr() as *mut RcInner<T>) },
            _not_sync_not_send: PhantomData,
        }
    }

    const fn inner(&self) -> &RcInner<T> {
        unsafe { self.data.as_ref() }
    }

    const fn inner_mut(&mut self) -> &mut RcInner<T> {
        unsafe { self.data.as_mut() }
    }

    /// Returs pointer to allocated data
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.inner().data_as_ptr() as *const T
    }

    /// Consumes the weak pointer and returns pointer to its data
    #[inline]
    pub fn into_raw(self) -> *const T {
        self.inner().data_as_ptr()
    }

    /// Upgrades the weak pointer to `Rc`
    pub fn upgrade(&self) -> Option<Rc<T>> {
        let inner = self.inner();
        if inner.strong() == 0 {
            None
        } else {
            inner.inc_strong();
            inner.dec_weak();
            Some(Rc::from_raw(inner.data_as_ptr()))
        }
    }

    #[inline]
    pub fn strong_count(&self) -> usize {
        self.inner().strong() as usize
    }

    #[inline]
    pub fn weak_count(&self) -> usize {
        self.inner().weak() as usize
    }

    /// Checks if the two Rcs are pointing to the same allocation
    #[inline]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.data == other.data
    }

    /// Checks if the two Rcs are pointing to the same allocation
    /// - does exaclty the same as `Rc::ptr_eq`
    #[inline]
    pub fn is_same(&self, other: &Self) -> bool {
        self.data == other.data
    }



}


impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        let inner = self.inner_mut();

        inner.dec_weak();
        
        if inner.strong() == 0 {
            unsafe {
                drop_in_place(inner.data_as_ptr());
                ALLOCATOR.delete(NonNull::new_unchecked(inner.as_mut_ptr()));
            }
        }
    }
}