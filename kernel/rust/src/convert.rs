//	convert.rs
//	this file originally belonged to baseOS project
//		an OS template on which to build

use crate::renderer::{Render, RENDERER};


pub trait Convert: Sized {
    type Type;
    fn into(&self) -> Self::Type;
    fn from(_: Self::Type) -> Self;
    fn from_ref(_: &Self::Type) -> Self;
}

pub trait ConvertMut<T>: Sized {
    type Type;
    type Error;
    fn mut_into(&self, data: &mut Self::Type) -> Result<(), Self::Error>;
    fn mut_from(&mut self, data: &Self::Type) -> Result<(), Self::Error>;
}

impl Render for u8 {
    #[inline(always)]
    fn render(&self) {
        unsafe {RENDERER.acquire_mut().acquire_mut().render(*self)};
    }
}


macro_rules! implement_render_for_unsigned {
    ($type:ty, $ln:expr) => {
        impl Render for $type {
            fn render(&self) {
                let rend = unsafe {crate::renderer::RENDERER.acquire_mut().acquire_mut()};
                if *self == 0 {
                    rend.render(b'0');
                }
                let mut num = *self;
                let mut index: isize = 0;
                let mut arr = [0u8; $ln];

                loop {
                    arr[index as usize] = (num % 10) as u8 + b'0';
                    num /= 10;
                    if num == 0 { break; }
                    index += 1;
                }

                while index >= 0 {
                    rend.render(arr[index as usize]);
                    index -= 1;
                }
            }
        }
    };
}

macro_rules! implement_render_for_signed {
    ($type:ty, $ln:expr) => {
        impl Render for $type {
            fn render(&self) {
                let rend = unsafe {RENDERER.acquire_mut().acquire_mut()};
                if *self == 0 {
                    rend.render(b'0');
                }
                let mut num = *self;
                if num < 0 {
                    num = -num;
                    rend.render(b'-');
                }
                let mut index: isize = 0;
                let mut arr = [0u8; $ln];

                loop {
                    arr[index as usize] = (num % 10) as u8 + b'0';
                    num /= 10;
                    if num == 0 { break; }
                    index += 1;
                }

                while index >= 0 {
                    rend.render(arr[index as usize]);
                    index -= 1;
                }
            }
        }
    };
}


implement_render_for_signed!(i8, 4);

implement_render_for_unsigned!(u16, 5);
implement_render_for_signed!(i16, 6);

implement_render_for_unsigned!(u32, 10);
implement_render_for_signed!(i32, 11);

implement_render_for_unsigned!(u64, 19);
implement_render_for_signed!(i64, 20);
implement_render_for_unsigned!(usize, 19);
implement_render_for_signed!(isize, 20);