//	io/functions.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build

use core::arch::asm;

pub mod int {
    use core::arch::asm;
    
    #[inline(always)]
    pub fn disable() {
        #[cfg(not(feature = "testing"))]
        unsafe { asm!("cli"); }
    }

    #[inline(always)]
    pub fn enable() {
        #[cfg(not(feature = "testing"))]
        unsafe { asm!("sti"); }
    }
}

pub fn outb(port: u16, data: u8) {
    #[cfg(not(feature = "testing"))]
    unsafe {
        asm!("out %0, %1",
        in("al") data,
        in("dx") port,
        options(nostack));
    }
}

pub fn inb(port: u16) -> u8 {
    let mut ret: u8;
    #[cfg(not(feature = "testing"))]
    unsafe {
        asm!("in %1, %0",
        out("al") ret,
        in("dx")port,
        options(nostack));
    }
    ret
}

pub fn outw(port: u16, data: u16) {
    #[cfg(not(feature = "testing"))]
    unsafe {
        asm!("out %0, %1",
        in("ax") data,
        in("dx") port,
        options(nostack));
    }
}

pub fn inw(port: u16) -> u16 {
    let mut ret: u16;
    #[cfg(not(feature = "testing"))]
    unsafe {
        asm!("in %1, %0",
        out("ax") ret,
        in("dx") port,
        options(nostack));
    }
    ret
}

pub fn outd(port: u16, data: u32) {
    #[cfg(not(feature = "testing"))]
    unsafe {
        asm!("out %0, %1",
        in("eax") data,
        in("dx") port,
        options(nostack));
    }
}

pub fn ind(port: u16) -> u32 {
    let mut ret: u32;
    #[cfg(not(feature = "testing"))]
    unsafe {
        asm!("in %1, %0",
        out("eax") ret,
        in("dx") port,
        options(nostack));
    }
    ret
}

pub fn outq(port: u16, data: u64) {
    #[cfg(not(feature = "testing"))]
    unsafe {
        asm!("out %0, %1",
        in("rax") data,
        in("dx") port,
        options(nostack));
    }
}

pub fn inq(port: u16) -> u64 {
    let mut ret: u64;
    #[cfg(not(feature = "testing"))]
    unsafe {
        asm!("in %1, %0",
        out("rax") ret,
        in("dx") port,
        options(nostack));
    }
    ret
}


pub fn wait() {
    //  wait aprox. nanosecond
    #[cfg(not(feature = "testing"))]
    outb(0x80, 0);
}