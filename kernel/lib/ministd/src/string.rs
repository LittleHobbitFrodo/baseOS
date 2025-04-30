//	string.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build


//  simple [`String`] implementation that uses ascii

use core::{fmt::{Display, Error}, ops::{Bound, Deref, RangeBounds}, ptr, slice};
use crate::{convert::{strify, Align}, mem::heap::{self, ALLOC_ALIGN}};
use crate::renderer::{RENDERER, Render};

pub const DATA_ALIGN: u32 = ALLOC_ALIGN as u32;

/// next multiple of [`DATA_ALIGN`]
#[inline(always)]
fn cap_next_up(len: u32) -> u32 {
    ((len >> 2) + 1) << 2
}


/// Implementation of the std [`String`] suited for kernel  
/// 
/// Differences from the std:
/// - Uses ASCII and [`u8`] as characters
/// - This implementation should cause panic in most functions
/// - Some functions are not implemented ([`remove_matches`], any utf related stuff, ...)
/// - Some custom functions are added ([`optimize`])
pub struct String {
    data: Option<*mut u8>,
    cap: u32,
    len: u32,
}

impl String {
    pub const fn new() -> Self {
        Self {
            data: None,
            cap: 0,
            len: 0,
        }
    }


    /// returns data as bytes  
    /// does exactly the same as [`as_str`] in this implementation
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.data.unwrap(), self.len as usize) }
    }

    /// shortens the string to specified [`size`]
    /// this has no effect on the allocated data ([`capacity`])
    #[inline]
    pub fn truncate(&mut self, size: u32) {
        if size < self.len {
            self.len = size;
        }
    }

    /// pops the last character and returns it  
    /// this does not take any operation on the buffer
    #[inline]
    pub fn pop(&mut self) -> Option<u8> {
        if let Some(data) = self.data {
            self.len -= 1;
            Some(unsafe { *data.add(self.len as usize) })
        } else {
            None
        }
    }

    /// removes character at some index  
    /// unlike in the [`std`] this implementation does not panic
    /// - instead it returns [`b'\0'`]  
    /// 
    /// This is an O(n) operation, as it requires copying every element in the buffer.
    pub fn remove(&mut self, index: u32) -> u8 {
        if let Some(data) = self.data {
            if index < self.len {
                let ret = unsafe { *data.add(index as usize) };
                self.len -= 1;
                unsafe {
                    for i in 0..self.len as usize {
                        *data.add(i) = *data.add(i+1);
                    }
                }
                return ret;
            }
        }
        b'\0'
    }

    /// inserts character into this string at [`index`]  
    /// This is an O(n) operation as it requires copying every element in the buffer.
    /// 
    /// behaviour:
    /// - [`index >= self.len()`]: pushes character
    /// - string is empty: no operation involved
    pub fn insert(&mut self, index: u32, ch: u8){
        if let Some(mut data) = self.data {
            if index < self.len {
                self.push(b'\0');
                data = self.data.unwrap();      //  in case of reallocation

                unsafe {
                    for i in index..self.len-2 {
                        *data.add(i as usize + 1) = *data.add(i as usize);
                    }
                    *data.add(index as usize) = ch;
                }
            } else {
                self.push(ch);
            }
        }
    }

    /// Inserts substring into string  
    /// This is an O(n) operation as it requires copying every element in the buffer.
    /// 
    /// behaviour:
    /// - [`index >= self.len()`]: pushes string
    /// - string is empty: no operation
    pub fn insert_str(&mut self, index: u32, string: &[u8]) {
        if let Some(mut data) = self.data {
            if index < self.len {
                if let Ok(_) = self.enlarge(string.len() as u32) {
                    data = self.data.unwrap();

                    unsafe {
                        for i in index..self.len-1-string.len() as u32 {
                            *data.add(i as usize+string.len()) = *data.add(i as usize);
                        }
                        core::ptr::copy(string.as_ptr(), data.add(index as usize), string.len());
                    }
                }

            } else {
                //  self.push_str(string);
            }
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }
    #[inline]
    pub fn capacity(&self) -> usize {
        self.cap as usize
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    /// splits string at [`index`], returning newly allocated string
    /// 
    /// behaviour:
    /// - should not panic
    /// - [`index >= self.len()`]: no operation
    /// - is empty: no operation
    /// - [`self.capacity()`] does not change
    pub fn split_off(&mut self, index: u32) -> String {
        if let Some(data) = self.data {
            if index < self.len {
                let new = match String::from_str(unsafe { slice::from_raw_parts(data.add(index as usize), (self.len-index) as usize) }) {
                    Ok(s) => s,
                    Err(_) => String::new(),
                };

                return new;
            }
        }
        String::new()
    }

    /// clears all data in the string  
    /// although it sets the [`self.len()`] to 0, buffer is not modified
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// replaces range of characteers with substring  
    /// - the substring does not need to be the same length
    /// 
    /// behaviour:
    /// - should not panic
    /// - string is empty: no operation
    pub fn replace_range<R>(&mut self, range: R, with: &[u8])
    where R: RangeBounds<usize> {
        if let Some(data) = self.data {
            let start = match range.start_bound() {
                Bound::Excluded(start) => *start,   //  -1?
                Bound::Included(start) => *start,
                Bound::Unbounded => return,
            };
            let end = match range.end_bound() {
                Bound::Excluded(end) => *end,   //  -1?
                Bound::Included(end) => *end,
                Bound::Unbounded => return,
            };

            for i in start..end {
                unsafe { *data.add(i) = with[i-start] }
            }
        }
    }

    #[inline]
    pub fn leak<'l>(&self) -> &'l mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.data.unwrap(), self.len()) }
    }



    
    pub fn from_str(str: &[u8]) -> Result<String, ()> {
        let data = match heap::alloc(str.len() as u32) {
            Ok(d) => d.as_ptr(),
            Err(_) => return Err(())
        };
        unsafe { core::ptr::copy_nonoverlapping(str.as_ptr(), data, str.len()); }
        Ok(String {
            data: Some(data),
            cap: str.len().align(ALLOC_ALIGN) as u32,
            len: str.len() as u32,
        })
    }


    pub fn push(&mut self, ch: u8) {
        if let Some(data) = self.data {
            if self.len < self.cap {
                unsafe { *(data.add(self.len as usize)) = ch };
                self.len += 1;
            } else {
                self.cap = cap_next_up(self.cap);
                self.data = match heap::realloc(data as *const _, self.len, self.cap) {
                    Ok(d) => Some(d.as_ptr()),
                    Err(_) => return,
                };
                unsafe { *self.data.unwrap().offset(self.len as isize) = ch; };
            }
        } else {
            self.data = match heap::alloc(DATA_ALIGN) {
                Ok(data) => {
                    unsafe { *data.as_ptr() = ch;}
                    self.len = 1;
                    Some(data.as_ptr())
                },
                Err(_) => return,
            };
            self.cap = DATA_ALIGN;
            self.len = 1;
        }
    }


    pub fn push_str(&mut self, str: &[u8]) {
        let ln = self.len;
        if let Ok(_) = self.enlarge(str.len() as u32) {
            let data = self.data.unwrap();
            for i in 0..str.len() as u32 {
                unsafe { *data.add((ln+i) as usize) = str[i as usize] };
            }
        }
    }

}


impl String {
    /// by > 0
    fn enlarge(&mut self, by: u32) -> Result<(), ()> {
        if let Some(data) = self.data {
            if self.len+by < self.cap {
                self.len += by;
                return Ok(());
            } else {
                self.len += by;
                let cap = self.len.align(DATA_ALIGN);
                self.data = match heap::realloc(data as *const _, self.cap, cap) {
                    Ok(d) => Some(d.as_ptr()),
                    Err(_) => return Err(()),
                };
                self.cap = cap;
                return Ok(());
            }
        } else {
            self.data = match heap::alloc(by as u32) {
                Ok(d) => Some(d.as_ptr()),
                Err(_) => return Err(()),
            };
            self.len = by;
            self.cap = by.align(DATA_ALIGN);
            return Ok(());
        }

    }
}

impl Drop for String {
    #[inline]
    fn drop(&mut self) {
        if let Some(data) = self.data {
            heap::dealloc(data, self.cap);
        }
    }
}

impl<'l> Deref for String {
    type Target=[u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.data.unwrap(), self.len as usize) }
    }
}

impl Display for String {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(data) = self.data {
            return write!(f, "{}", strify(unsafe { slice::from_raw_parts(data, self.len as usize) }));
        }
        Err(Error::default())
    }
}

