//	convert.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build



pub trait Align {
    fn align(&self, align: Self) -> Self;
}


impl Align for usize {
    #[inline]
    /// aligns up to X if isn't already aligned
    fn align(&self, align: Self) -> Self {
        (self + align - 1) & !(align-1)
    }
}

impl Align for u32 {
    #[inline]
    fn align(&self, align: Self) -> Self {
        (self + align - 1) & !(align-1)
    }
}

/// converts [`&[u8]`] to [`&str`]
#[inline(always)]
pub fn strify(s: &[u8]) -> &str {
    unsafe { core::str::from_utf8_unchecked(s) }
}