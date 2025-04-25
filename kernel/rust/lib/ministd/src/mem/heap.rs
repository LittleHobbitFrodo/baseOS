//	mem/heap.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build


//  this file implements features of the buddy_system_allocator

pub use buddy_system_allocator as allocator;
pub use core::alloc::Layout;

#[global_allocator]
pub static HEAP: allocator::LockedHeap<32> = allocator::LockedHeap::new();

use crate::mem::Region;

pub static REGION: crate::Mutex<Region> = crate::Mutex::new(Region::empty());


unsafe extern "C" {
     fn find_heap_region() -> Result<Region, ()>;
}

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

