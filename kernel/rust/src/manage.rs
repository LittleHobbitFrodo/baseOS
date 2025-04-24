//	manage.rs
//	this file originally belonged to baseOS project
//		an OS template on which to build

use core::{arch::asm, hint::spin_loop};
use ministd::renderer::RENDERER;
use core::panic::PanicInfo;
use ministd::sync::rwlock::RwLock;

pub extern "C" fn hang() -> ! {
    crate::io::int::disable();
    loop {
        unsafe { asm!("hlt"); }
        spin_loop();
    }
}


/// [`KernelInitState`] is enumerator ised to check what init phase is the kernel in
pub enum KernelInitState {
    Base,
    Memory,
    //  TODO!
}


/// [`KernelState`] is enumerator used to check what phase is kernel in
/// - you are supposed to edit this enum for your specific use
pub enum KernelState {
    Init(KernelInitState),
    Runtime,
    Shutdown,
}

static KERNEL_STATE: RwLock<KernelState> = RwLock::new(KernelState::Init(KernelInitState::Base));


pub fn panic(msg: &[u8]) -> ! {

    let mut rend = RENDERER.lock();
    if rend.line() > 0 {
        rend.endl();
    }
    rend.set_color(0xff0000);



    hang();

    /*let mut rend = RENDERER.lock();
    rend.set_color(0xff0000);
    rend.printstr(b"PANIC");
    rend.set_color(0xffffff);
    rend.printstr(b": ");
    rend.printstr(msg);
    rend.endl();

    rend.printstr(b"\n\nhalting the system");
    hang();*/
}

#[panic_handler]
pub fn _panic(_info: &PanicInfo) -> ! {

    panic(b"panic");

}


#[macro_export]
macro_rules! maccc {
    ($msg:literal) => {
        print($msg);
    }
}