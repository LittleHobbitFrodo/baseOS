//	mem/heap.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build


//  this file implements features of the buddy_system_allocator

/// tells the allocator how to align data  
/// 
/// this is also the default align for all allocations  
/// if you change the value:
/// - must be > 0
/// - must be power of 2
/// 
/// otherwise it could break things
//pub const ALLOC_ALIGN: usize = 4;

pub use buddy_system_allocator as allocator;
use spin::MutexGuard;
pub use core::alloc::GlobalAlloc;
pub use core::alloc::Layout;
use core::mem::MaybeUninit;
use core::ptr::{copy_nonoverlapping, drop_in_place, null_mut, NonNull};
use crate::mem::*;

use crate::spin::Mutex;
use crate::Immutable;

pub type LockedHeap = allocator::LockedHeap<32>;
pub type Heap = allocator::Heap<32>;

/// The default Allocator type for BaseOS
/// - has no members on purpose to prevent taking any memory with the use of `Global` allocator in other crates
pub struct Allocator { }


impl Allocator {
    pub(crate) const fn new() -> Self {
        Self { }
    }

    /// allocates data of type T with proper alignment
    /// - layout: `size: size_of::<T>(), align: align_of::<T>()`
    pub unsafe fn allocate<T: Sized>(&self, val: T) -> Result<NonNull<T>, ()> {
        let layout = Layout::new::<T>();

        let data = unsafe { self.alloc(layout) as *mut T };

        if data.is_null() {
            return Err(());
        }

        unsafe {
            *data = val;
        }

        Ok(unsafe { NonNull::new_unchecked(data) })

    }

    /// Allocates uninitialized memory based on layout
    #[inline]
    pub unsafe fn allocate_layout<T: Sized>(&self, layout: Layout) -> Result<NonNull<T>, ()> {
        NonNull::new(unsafe { self.alloc(layout) as *mut T }).ok_or(())
    }

    /// allocates uninitialized data of type T with proper alignment
    /// - layout: `size: size_of::<T>(), align: align_of::<T>()`
    pub unsafe fn allocate_uninit<T: Sized>(&self) -> Result<NonNull<MaybeUninit<T>>, ()> {
        let layout = unsafe { Layout::from_size_align_unchecked(size_of::<T>(), align_of::<T>()) };
        
        let data = unsafe { self.alloc(layout) as *mut MaybeUninit<T> };

        if data.is_null() {
            return Err(());
        }

        Ok(unsafe { NonNull::new_unchecked(data) })
    }

    /// deallocate pointer from heap and run its `drop` if it is needed
    /// - layout: `size: size_of::<T>(), align: align_od::<T>()`
    #[inline]
    pub unsafe fn delete<T: Sized>(&self, ptr: NonNull<T>) {
        unsafe {
            drop_in_place(ptr.as_ptr());
            self.dealloc(ptr.as_ptr() as *mut u8, Layout::from_size_align_unchecked(size_of::<T>(), align_of::<T>()));
        }
    }

}

impl Allocator {


    /// gets immutable reference to regions
    #[inline]
    pub fn get_regions(&self) -> Immutable<MutexGuard<Region<PAGE_ALIGN>>> {
        Immutable::new(REGIONS.lock())
    }
    
    /// try to obtain regions
    #[inline]
    pub fn try_get_regions(&self) -> Option<Immutable<MutexGuard<Region<PAGE_ALIGN>>>> {
        if let Some(guard) = REGIONS.try_lock() {
            Some(Immutable::new(guard))
        } else {
            None
        }
    }

    /// add range of addresses to heap  
    /// also pushes into the regions vector
    #[inline(always)]
    pub unsafe fn add_to_heap(&self, start: usize, end: usize) {
        //  once using vector for regions: push
        unsafe { HEAP.lock().add_to_heap(start, end) };
    }

    #[inline(always)]
    pub unsafe fn add_to_heap_locked(&self, guard: &mut MutexGuard<Heap>, start: usize, end: usize) {
        //  push into vector
        unsafe { guard.add_to_heap(start, end) };
    }

    /// returns the actual number of bytes in the heap
    #[inline(always)]
    pub fn total_bytes(&self) -> usize {
        HEAP.lock().stats_total_bytes()
    }

    /// returns the number of bytes that are allocated
    #[inline(always)]
    pub fn allocated_bytes(&self) -> usize {
        HEAP.lock().stats_alloc_actual()
    }

    /// reallocates memory to an new layout
    pub unsafe fn realloc_layout(&self, old: *mut u8, old_l: Layout, new_l: Layout) -> *mut u8 {
        let new = unsafe { self.alloc(new_l) };

        if new.is_null() {
            return core::ptr::null_mut();
        }

        unsafe {
            core::ptr::copy_nonoverlapping(old, new, core::cmp::min(old_l.size(), new_l.size()));
            ALLOCATOR.dealloc(old, old_l);
        }

        new

    }
    
}



unsafe impl GlobalAlloc for Allocator {

    /// allocates new data on the heap
    /// 
    /// if allocation fails:
    /// - runs the `out_of_memory_handler` routine (defined in main crate)
    ///   - success: try allocation again
    ///   - failure: returns null
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {

        match HEAP.lock().alloc(layout) {
            Ok(data) => data.as_ptr(),
            Err(_) => {
                //  run out_of_memory routine and try again
                
                let mut alloc = HEAP.lock();
                if let Ok(_) = unsafe { __oom_handler(&mut alloc, &self) }{
                    match alloc.alloc(layout) {
                        Ok(data) => data.as_ptr(),
                        Err(_) => null_mut(),
                    }
                } else {
                    null_mut()
                }

            },
        }
    }

    /// same as `alloc` but zeroes the allocated buffer
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {

        let data = unsafe { self.alloc(layout) };

        if data.is_null() {
            null_mut()
        } else {
            unsafe {
                core::ptr::write_bytes(data, 0, layout.size())
            }
            data
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        HEAP.lock().dealloc(unsafe { NonNull::new_unchecked(ptr) }, layout);
    }

    /// reallocates memory
    /// - does not deallocate the old buffer if allocation fails
    /// 
    /// used layout: `Layout::from_size_unchecked(new_size, layout.align())`
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        
        let new = unsafe {
            self.alloc(Layout::from_size_align_unchecked(new_size, layout.align()))
        };

        if new.is_null() {
            return null_mut();
        }

        let count = core::cmp::min(new_size, layout.size());
        unsafe {
            copy_nonoverlapping(ptr, new, count);
            self.dealloc(ptr, layout);
        }
        new
    }

}


/// This is the global allocator for BaseOS
/// 
/// It is used by all structures that are working with heap as the default allocator
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::new();

/// This structure takes care of mapping all the memory regions of the heap
pub(crate) static REGIONS: Mutex<Region<PAGE_ALIGN>> = Mutex::new(Region::empty());
/// This is the Heap used by the `ALLOCATOR`
pub(crate) static HEAP: LockedHeap = allocator::LockedHeap::new();
    // use Vec later


unsafe extern "Rust" {

    //  functions defined by the developer in the main crate

    pub(crate) fn __region_finder() -> Result<Region, Option<&'static str>>;
    //pub(crate) fn out_of_memory_handler(heap: &mut MutexGuard<Heap>, allocator: &Allocator) -> Result<(), ()>;
    pub(crate) fn __oom_handler(heap: &mut crate::HeapRef, alloc: &crate::Allocator) -> Result<(), ()>;
}



//  TODO: use mutex<Vec<Region>> for heap mapping



/// `init` initializes heap  
/// 
/// **IMPORTANT**
/// - this function uses the [`mem::find_heap_region`] function from the main crate
///   - rewrite this function to change the default behaviour
/// 
/// You can check where the heap is with the [`ministd::mem::heap::REGION`] variable
/// - please do not change it
pub(crate) fn init() -> Result<(), Option<&'static str>> {

    let reg = unsafe { __region_finder() }?;

    *REGIONS.lock() = reg;

    let mut alloc = HEAP.lock();

    unsafe { alloc.init(reg.virt() as usize, reg.size); }

    Ok(())

}