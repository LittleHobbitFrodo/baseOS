//	manage/mod.rs
//	this file originally belonged to baseOS project
//		an OS template on which to build


use core::{arch::asm, hint::spin_loop};
use ministd::{renderer::RENDERER, RwLock, hang};
use core::panic::PanicInfo;
use ministd::{print, println, locked_print, locked_println};
use ministd::convert::strify;

pub mod kernel_state;
pub use kernel_state::*;




static KERNEL_STATE: RwLock<KernelState> = RwLock::new(KernelState::Init(KernelInitState::Base));



/// This is the default panic handler  
/// rewrite this function to change the default behaviour
/// 
/// 
#[unsafe(no_mangle)]
pub extern "C" fn panic_handler(msg: &str) {

    let mut rend = RENDERER.lock();
    if rend.column() > 0 {
        rend.endl();
    }

    rend.set_color(0xff0000);

    locked_print!(rend, "PANIC occured ");

    match &*KERNEL_STATE.read() {
        KernelState::Init(i) => {
            locked_println!(rend, "while initializing {}", strify(i.as_str()) );
        },
        KernelState::Runtime(r) => {
            locked_println!(rend, "at runtime task {}", strify(r.as_str()));
        },
        KernelState::Shutdown(s) => {
            locked_println!(rend, "at shutdown task {}", strify(s.as_str()));
        }
    }

    rend.set_color(0xffffff);
    locked_println!(rend, "error message: {}", msg);


    /*let mut rend = RENDERER.lock();
    if rend.line() > 0 {
        rend.endl();
    }
    rend.set_color(0xff0000);
    rend.print(b"PANIC occured ");
    
    match &*KERNEL_STATE.read() {
        KernelState::Init(i) => {
            rend.print(b"while initializing ");
            rend.println(i.as_str());
        },
        KernelState::Runtime(r) => {
            rend.print(b"at runtime operation");
            rend.println(r.as_str());
        },
        KernelState::Shutdown(s) => {
            rend.print(b"while shudding down");
            rend.println(s.as_str());
        }
    }
    rend.set_color(0xffffff);
    rend.println(msg);
    rend.endl();
    rend.endl();
    rend.endl();
    
    rend.println(b"halting the system");
    
    hang();*/
}

#[panic_handler]
pub fn _panic(_info: &PanicInfo) -> ! {

    ministd::kernel_panic!("panic");

}


#[macro_export]
macro_rules! maccc {
    ($msg:literal) => {
        print($msg);
    }
}