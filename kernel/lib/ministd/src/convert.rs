//	convert.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build



pub trait Align {
    /// returns `Self` aligned to `align`
    fn align(&self, align: usize) -> Self;

    /// aligns `self` to `align`
    fn align_mut(&mut self, align: usize);
}

pub trait IsAligned {
    fn is_aligned(&self, align: usize) -> bool;

    fn is_not_aligned(&self, align: usize) -> bool;
}

macro_rules! impl_align_for {
    ($i:ty) => {
        impl Align for $i {
            #[inline]
            /// aligns up to X if isn't already aligned
            fn align(&self, align: usize) -> Self {
                (self + align as Self - 1) & !(align as Self-1)
            }

            #[inline]
            fn align_mut(&mut self, align: usize) {
                *self = (*self + align as Self - 1) & !(align as Self-1);
            }
        }

        impl IsAligned for $i {
            #[inline]
            fn is_aligned(&self, align: usize) -> bool {
                *self == *self & !(align as Self-1)
            }

            #[inline]
            fn is_not_aligned(&self, align: usize) -> bool {
                *self != *self * !(align as Self-1)
            }
        }
    };
}

impl_align_for!(usize);
impl_align_for!(u32);
impl_align_for!(u16);
impl_align_for!(u8);

impl Align for *const u8 {
    #[inline]
    /// aligns up to X if isn't already aligned
    fn align(&self, align: usize) -> Self {
        ((self.addr() + align - 1) & !(align-1)) as Self
    }

    #[inline]
    fn align_mut(&mut self, align: usize) {
        *self = ((self.addr() + align - 1) & !(align-1)) as Self
    }
}


/// converts `&[u8]` to [`&str`]
pub const fn strify(s: &[u8]) -> &str {
    unsafe { core::str::from_utf8_unchecked(s) }
}

/// converts `&mut [u8]` to `&mut str`
pub const fn strify_mut(s: &mut [u8]) -> &mut str {
    unsafe { core::str::from_utf8_unchecked_mut(s) }
}