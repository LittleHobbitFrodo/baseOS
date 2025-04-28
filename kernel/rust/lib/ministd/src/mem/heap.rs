//	mem/heap.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build


//  this file implements features of the buddy_system_allocator

/// tells the allocator how to align data  
/// if you change the value:
/// - must be > 0
/// - must be power of 2
/// 
/// otherwise it could break things
pub const ALLOC_ALIGN: usize = 4;

pub use buddy_system_allocator as allocator;
pub use core::alloc::Layout;
use core::ptr::NonNull;
use crate::mem::Region;
use crate::convert::Align;


#[global_allocator]
static HEAP: allocator::LockedHeap<32> = allocator::LockedHeap::new();

pub static REGION: crate::Mutex<Region> = crate::Mutex::new(Region::empty());


unsafe extern "C" {
     fn find_heap_region() -> Result<Region, ()>;
}

//  TODO: use mutex<Vec<Region>> for heap mapping


/// [`init`] initializes heap  
/// **IMPORTANT**
/// - this function uses the [`mem::find_heap_region`] function from the main crate
///   - rewrite this function to change the default behaviour
/// 
/// You can check where the heap is with the [`ministd::mem::heap::REGION`] variable
/// - please do not change it
pub fn init() -> Result<(), ()> {

    if let Ok(reg) = unsafe { find_heap_region() } {
        unsafe {
            HEAP.lock().init(reg.start, reg.size);
            *REGION.lock() = reg;
            return Ok(());
        }
    }
    Err(())

}


////    TODO!



/// Allocates new data
/// - Use [`ministd::mem::heap::dealloc`] to deallocate the data
/// - Allocated data is aligned to [`ALLOC_ALIGN`] constant
///   - 4 by default  
pub fn alloc(size: u32) -> Result<NonNull<u8>, ()> {
    let layout = unsafe { Layout::from_size_align_unchecked(size.align(ALLOC_ALIGN as u32) as usize , ALLOC_ALIGN) };
    HEAP.lock().alloc(layout)
}

pub fn aligned_alloc(size: u32, align: u32) -> Result<NonNull<u8>, ()> {
    let layout = match Layout:: from_size_align(size.align(ALLOC_ALIGN as u32) as usize, align as usize) {
        Ok(l) => l,
        Err(_) => return Err(()),
    };
    HEAP.lock().alloc(layout)
}

/// Reallocates data
/// - Allocated data is aligned to [`ALLOC_ALIGN`] constant
pub fn realloc(ptr: *const u8, old_size: u32, new_size: u32) -> Result<NonNull<u8>, ()> {
    let new = match alloc(new_size) {
        Ok(data) => data.as_ptr(),
        Err(_) => return Err(()),
    };

    let len = core::cmp::min(old_size.align(ALLOC_ALIGN as u32), new_size.align(ALLOC_ALIGN as u32));
    unsafe { core::ptr::copy_nonoverlapping(ptr, new, len as usize) };

    dealloc(ptr, old_size);

    Ok(NonNull::from( unsafe {new.as_ref().unwrap()} ))
}



#[inline]
pub fn dealloc(ptr: *const u8, size: u32) {
    let layout = unsafe { Layout::from_size_align_unchecked(size.align(ALLOC_ALIGN as u32) as usize, ALLOC_ALIGN) };
    HEAP.lock().dealloc(NonNull::from(unsafe { ptr.as_ref().unwrap() }), layout);
}

#[inline]
pub fn aligned_dealloc(ptr: *const u8, size: u32, align: u32) -> Result<(), ()> {
    let layout = match Layout::from_size_align(size.align(ALLOC_ALIGN as u32) as usize, align as usize) {
        Ok(l) => l,
        Err(_) => return Err(()),
    };
    HEAP.lock().dealloc(NonNull::from(unsafe { ptr.as_ref().unwrap() }), layout);
    Ok(())
}



/// returns size of allocated data in bytes
#[inline]
pub fn bytes_allocated() -> usize {
    HEAP.lock().stats_alloc_actual()
}

/// adds region into the heap
#[inline]
pub fn expand_region(reg: Region) {
    unsafe { HEAP.lock().add_to_heap(reg.start(), reg.size()) };
}


/// returns the size of heap in bytes
#[inline]
pub fn size() -> usize {
    HEAP.lock().stats_total_bytes()
}