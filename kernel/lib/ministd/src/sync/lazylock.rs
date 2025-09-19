//  mem/sync/lazylock.rs (ministd crate)
//  this file originally belonged to baseOS project
//      an OS template on which to build

use core::{cell::Cell, mem::{ManuallyDrop, MaybeUninit}, ptr::drop_in_place};
use super::Once;

union Data<T: Sized, F: Fn() -> T> {
    value: ManuallyDrop<T>,
    f: ManuallyDrop<F>,
}

/// A value which is initialized on the first access.
pub struct LazyLock<T: Sized, F: Fn() -> T> {
    once: Once,
    //data: UnsafeCell<MaybeUninit<T>>,
    data: Cell<Data<T, F>>,
}

impl<T: Sized, F: Fn() -> T> LazyLock<T, F> {

    /// Creates a new lazy value with the given initializing function
    pub const fn new(f: F) -> Self {
        Self {
            once: Once::new(),
            data: Cell::new(Data { f: ManuallyDrop::new(f) })
        }
    }

    /// Consumes this `LazyLock` returning the stored value.
    /// Returns `Ok(value)` if `Lazy` is initialized and `Err(f)` otherwise
    pub fn into_inner(self) -> Result<T, F> {

        let mut this = ManuallyDrop::new(self);

        unsafe {
            if this.once.is_completed() {
                Ok(ManuallyDrop::take(&mut this.data.get_mut().value))
            } else {
                Err(ManuallyDrop::take(&mut this.data.get_mut().f))
            }
        }
    }

    /// Forces the evaluation of this lazy value and returns a mutable reference to the result
    pub fn force_mut(&self) -> &mut T {

        let data = self.data.as_ptr();

        unsafe {

            let f = ManuallyDrop::take(&mut (*data).f);

            if self.once.is_completed() {
                ManuallyDrop::drop(&mut (*data).value);
            }

            (*data).value = ManuallyDrop::new(f());

            &mut (*data).value
        }
    }


}