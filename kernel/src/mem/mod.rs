//  init.rs
//  this file originally belonged to baseOS project
//      on OS template on which to build

use ministd::{alloc::*, mem::Region, MutexGuard};
use bootloader::{MEMMAP, HHDM};
use limine_rs::memory_map::EntryType;
use ministd::mem::{MB, PAGE_ALIGN};
use ministd::oom;

/// `mem::find_heap_region` function is used by the `ministd::init::allocator()` to find suitable place in the memory for heap
/// - returns `ministd::mem::Region` structure that stores
///   - physical and virtual address (must be aligned to `PAGE_ALIGN`)
///   - size: also aligned to `PAGE_SIZE`
/// 
/// 
/// the error value (`Option<&str>`) is used as possible error message
/// 
/// ### `ministd::init::allocator()`
/// 
/// 1. The function will call `find_heap_region` to search for valid region
/// 2. Proceeds to initialize the allocator if the region is found
///     - returns `Err` if not
/// 
/// Feel free to change the behaviour of this function, but do not modify the declaration
/// 
#[unsafe(no_mangle)]
extern "Rust" fn find_heap_region() -> Result<ministd::mem::Region<PAGE_ALIGN>, Option<&'static str>> {

    //  The default behaviour of the function finds valid `usable` MEMMAP entry big enough that is covered by HHDM
    //  - you can change the HHDM request revision in the `bootloader` crate
    //      - make sure that theis function works correctly with the revision
    //  - more about revisions: https://github.com/limine-bootloader/limine/blob/v9.x/PROTOCOL.md#entry-memory-layout


    let hhdm = HHDM.get_response()
        .ok_or(Some("failed to acquire HHDM offset"))?.offset();

    let map = MEMMAP.get_response()
        .ok_or(Some("memory map not found"))?.entries();

    for i in map {
        if i.entry_type == EntryType::USABLE && i.length as usize > MB * 2 {

            let virt = (i.base + hhdm) as usize;
                //  add HHDM offset to create virtual address

            let phys = i.base as usize;
            let size = i.length as usize;

            return Ok(unsafe { Region::new_unchecked(virt, phys, size) })
        
        }
    }

    Err(Some("did not found place for heap"))

}



/// This function is called by the allocator whenever it fails to allocate memory
/// 
/// The purpose of tis function is to add memory to the allocator (if possible)
/// - shall return `Err` upon failure
/// 
/// Feel free to change the behaviour of the function
/// - but keep in ming that it has to have the `#[oom]` attribute
/// 
/// The `#[oom` macro should catch all mistakes in the signature
/// - but you never know: `fn <name>(&mut ministd::HeapRef, &ministd::Allocator) -> Result<(), ()>`
#[oom]
fn out_of_memory(heap: &mut ministd::HeapRef, alloc: &ministd::Allocator) -> Result<(), ()> {
    Err(())
}



