//  init.rs
//  this file originally belonged to baseOS project
//      on OS template on which to build

use ministd::mem::Region;
use bootloader::{MEMMAP, HHDM};
use limine_rs::memory_map::EntryType;
use ministd::mem::{MB, GB};

/// Use this function to find an valid spot for heap
/// Fell free to rewrite it!
/// - but do not change the declaration
#[unsafe(no_mangle)]
extern "C" fn find_heap_region() -> Result<Region, ()> {

    let hhdm = match HHDM.get_response() {
        Some(res) => res.offset(),
        None => return Err(()),
    } as usize;

    if let Some(res) = MEMMAP.get_response() {
        for i in res.entries() {
            match i.entry_type {
                EntryType::USABLE => {
                    if (i.length as usize) > MB && (i.base as usize) < 4*GB {   //  4GB should be the boundary for HHDM
                        return Ok(Region::new(i.base as usize + hhdm, core::cmp::min(i.length as usize, 2*MB)));
                        //  add HHDM offset to be in virtual address space
                    }
                },
                _ => continue,
            }
        }
    }

    Err(())

}