//	convert.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build

use core::{fmt::Write, mem::MaybeUninit};

use crate::renderer::{Render, RENDERER};


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
        (self + align as u32 -1) & !(align as u32-1)
    }
}




impl Render for usize {
    fn render(&self) {

        let mut rend = RENDERER.lock();
        if *self == 0 {
            rend.render(b'0');
        } else {
            let mut buf = crate::array::uninit::<u8, 20>();
            let mut i = 19;

            let mut num = *self;
            while num != 0 {
                buf[i] = ((num % 10) as u8) + b'0';
                i -= 1;
                num /= 10;
            }

            rend.print(&buf[i..]);
        }

    }

    fn render_locked<'l>(&self, guard: &'l mut spin::MutexGuard<crate::renderer::renderer::Renderer>) {
        
        if *self == 0 {
            guard.render(b'0');
        } else {
            let mut buf = crate::array::uninit::<u8, 20>();
            let mut i = 19;

            let mut num = *self;
            while num != 0 {
                buf[i] = ((num % 10) as u8) + b'0';
                i -= 1;
                num /= 10;
            }

            guard.print(&buf[i..]);
        }

    }

}


impl Render for isize {
    fn render(&self) {

        let mut rend = RENDERER.lock();
        if *self == 0 {
            rend.render(b'0');
        } else {
            let mut buf = crate::array::uninit::<u8, 21>();
            let mut num = *self;
            let negative = num < 0;
            let mut i = 19;

            if negative {
                num = -num;
                rend.render(b'-');

                while num != 0 {
                    buf[i] = ((num % 10) as u8) + b'0';
                    num /= 10;
                    i -= 1;
                }
                rend.print(&buf[i..]);
            }
        }
    }

    fn render_locked<'l>(&self, guard: &'l mut spin::MutexGuard<crate::renderer::renderer::Renderer>) {
        if *self == 0 {
            guard.render(b'0');
        } else {
            let mut buf = crate::array::uninit::<u8, 21>();
            let mut num = *self;
            let negative = num < 0;
            let mut i = 19;

            if negative {
                num = -num;
                guard.render(b'-');

                while num != 0 {
                    buf[i] = ((num % 10) as u8) + b'0';
                    num /= 10;
                    i -= 1;
                }
                guard.print(&buf[i..]);
            } else {
                while num != 0 {
                    buf[i] = ((num % 10) as u8) + b'0';
                    num /= 10;
                    i -= 1;
                }
                guard.print(&buf[i..]);
            }
        }
    }

}
