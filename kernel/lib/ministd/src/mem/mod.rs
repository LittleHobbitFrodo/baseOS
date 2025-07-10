//	mem/mod.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build

/// this file provides basic memory-related functionalities



pub const KB: usize = 1024;
pub const MB: usize = 1024 * 1024;
pub const GB: usize = 1024 * 1024 * 1024;

#[cfg(target_arch = "x86_64")]
/// Constant that shows the size of one page
/// - target specific
pub const PAGE_SIZE: usize = 4096;

#[cfg(target_arch = "x86_64")]
/// Constant that shows the align of one page
/// - target specific
pub const PAGE_ALIGN: usize = 4096;

pub use core::mem::needs_drop;

pub mod readonly;
pub use readonly::ReadOnly;
pub mod alloc;
pub mod boxed;
pub mod array;
pub mod dynamic_buffer;

pub mod string;
pub mod vec;
pub mod rc;

pub use dynamic_buffer::DynamicBuffer;

pub use crate::convert::Align;



/// Region represents memory region
/// 
/// It holds virtual address, physical address and size of the memory region
/// 
/// Generic parameter: ALIGN
/// - **forces the align** of addresses and size
///   - the `ministd::mem::Align` trait is used to align values
#[derive(Copy, Clone)]
pub struct Region<const ALIGN: usize = PAGE_ALIGN> {
    virt: usize,    //  usize is only inner representation
    phys: usize,
    size: usize,
}

const fn const_align_usize(val: usize, align: usize) -> usize {
    (val + align-1) & !(align-1)
}


impl<const ALIGN: usize> Region<ALIGN> {

    

    /// Constructs Region with given information
    /// - `virt: usize` is used because of `*const u8` cannot be aligned in `const fn`
    pub const fn new(virt: usize, phys: usize, size: usize) -> Self {
        Self {
            virt: const_align_usize(virt, ALIGN),
            phys: const_align_usize(phys, ALIGN),
            size: const_align_usize(size, ALIGN),
        }
    }

    /// Constructs Region with given information, does not align values
    /// - `virt: usize` is used because of `*const u8` cannot be aligned in `const fn`
    pub const unsafe fn new_unchecked(virt: usize, phys: usize, size: usize) -> Self {
        Self {
            virt: virt,
            phys,
            size,
        }
    }

    /// Constructs empty Region
    /// - addresses are set to `null`
    /// - `size = 0`
    pub const fn empty() -> Self {
        Self {
            virt: 0,
            phys: 0,
            size: 0,
        }
    }

    /// Moves the virtual address to specified value
    /// - the new address is aligned to `ALIGN`
    #[inline(always)]
    pub fn move_to(&mut self, virt: *const u8) {
        self.virt = const_align_usize(virt.addr(), ALIGN);
    }

    /// Moves the virtual address to specified value
    /// - does not align the new address
    #[inline(always)]
    pub unsafe fn move_to_unchecked(&mut self, virt: *const u8) {
        self.virt = virt as usize;
    }

    /// Moves the virtual address by specified amount of bytes
    /// - returns `Err` on overflow
    /// - the address is calculated and then aligned to `ALIGN`
    #[inline]
    pub fn move_by(&mut self, by: isize) -> Result<(), ()> {

        self.virt = (  self.virt.checked_add_signed(by).ok_or(())?  ).align(ALIGN);

        Ok(())

    }

    /// Moves the virtual address by specified amount of bytes
    /// - does not check for overflow
    /// - the address is calculated and then aligned to `ALIGN`
    #[inline]
    pub fn move_by_unchecked(&mut self, by: isize) {
        self.virt = unsafe { (self.virt as *const u8).offset(by) as usize }.align(ALIGN);
    }

    /// Moves the virtual address by specified amount of bytes
    /// - does not check for overflow
    /// - does not align thw calculated address
    pub unsafe fn move_by_unckecked_unaligned(&mut self, by: isize) {
        self.virt = unsafe { (self.virt as *const u8).offset(by) } as usize
    }

    /// Moves the physical address to specified place
    /// - the physical address is aligned to `ALIGN`
    pub const fn reallocate(&mut self, phys: usize) {
        self.phys = const_align_usize(phys, ALIGN);
    }

    /// Moves the physical address to specified place
    /// - does not align the address
    pub const unsafe fn reallocate_unchecked(&mut self, phys: usize) {
        self.phys = phys;
    }

    /// Resizes the region
    pub const fn resize(&mut self, size: usize) {
        self.size = const_align_usize(size, ALIGN);
    }

    /// Adds some contignous memory to the Region
    /// - the calculated size is the aligned to `ALIGN`
    pub const fn enlarge(&mut self, by: usize) {
        self.size = const_align_usize(self.size + by, ALIGN);
    }

    /// Adds some contignous memory to the region
    /// - does not align the size
    pub const fn enlarge_unckecked(&mut self, by: usize) {
        self.size += by;
    }

    /// Shrinks the region by specified value
    /// - returns `Err` if resulted size is less than or equal to zero
    #[inline]
    pub fn shrink(&mut self, by: usize) -> Result<(), ()> {

        self.size = (self.size.checked_sub(by).ok_or(())?).align(ALIGN);

        Ok(())

    }

    /// Shrinks the region by specified value
    /// - does not check for overflow
    /// - the calculated size is aligned to `ALIGN`
    #[inline]
    pub unsafe fn shrink_unchecked(&mut self, by: usize) {
        self.size = (self.size - by).align(ALIGN);
    }

    /// Shrinks the region by specified value
    /// - does not check for overflow
    /// - does not align the size
    pub unsafe fn shrink_unchecked_unaligned(&mut self, by: usize) {
        self.size -= by;
    }

    /// Returns the starting virtual address
    pub const fn virt(&self) -> *const u8 {
        self.virt as *const u8
    }

    /// Returns the starting physical address
    pub const fn phys(&self) -> usize {
        self.phys
    }

    /// Returns the size of the region
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Returns the forced align for this instance
    pub const fn align(&self) -> usize {
        ALIGN
    }

}
