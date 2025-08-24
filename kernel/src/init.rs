//  init.rs
//  this file originally belonged to baseOS project
//      on OS template on which to build

//  module for kernel initialization

use ministd::entry;
use ministd::init;
use ministd::RENDERER;
use ministd::{panic_fmt, print, println};
use ministd::{testing, test_only};
use ministd::String;
use ministd::renderer::MinistdRenderer;



/// This function is here to initialize your kernel
/// s
/// you can initialize various kernel functions and submodules here
fn init() -> Result<(), ()> {

    //  initialize screen renderer
    if let Err(_) = init::renderer() {
        panic!("failed to initialize renderer");
    }

    //  initialize allocator
    if let Err(msg) = init::allocator() {
        if let Some(msg) = msg {
            panic_fmt!("failed to initialize heap: {msg}")
        } else {
            panic!("failed to initialize heap");
        }
    }

    println!("hello world!");

    //  run all tests
    ministd::run_tests!("STRING", false);

    let mut rend = RENDERER.lock();
    rend.set_color(0x00ff00);
    println!(rend: "ALL TESTS PASSED");

    Ok(())

}



/// This is the kernel entry point, the function is called by the bootloader
/// 
/// Feel free to change the behaviour of this function
/// - the function needs to be annoted with the `#[ministd::entry]` attribute and must **never** return (`-> !`)
/// 
/// Please do not change the entry point in the `bootloader` crate
/// - the real entry point is the default `_start` symbol
#[entry]
pub fn kernel_entry() -> ! {
    if let Err(_) = init() {
        panic!("failed to initialize kernel");
    }

    ministd::hang();

}



/// # Testing
/// This is your regular unit test to run in the emulator
/// - The whole `ministd` library is available
///
/// # Building the project for tests
/// 1. `./util test <arch>` runs all tests on specified platforms
/// 2. `./util test custom <arch>` runs your OS with test mode enabled
///     - use `ministd::run_tests!()` macro to run tests
#[testing]
fn test_in_emulator() {
    println!("TEST");

    let condition = true;

    assert!(0 == 1);

    if condition {
        success!();
    } else {
        panic!("the test failed :(");
        //  you can also use `fail!()` macro
    }

}

/// This is how you can name a group of tests
/// - the group must be tested with the `run_tests!(<g name>)` macro
#[testing(SOME)]
fn test_some() {
    let something: Option<&'static str> = Some("something");
    let nothing: Option<&'static str> = None;

    if let Some(smth) = something {
        println!("something: {smth}");
    } else {
        println!("something is none");
    }

    if let Some(n) = nothing {
        println!("nothing: {n}")
    } else {
        println!("nothing is none");
    }

}


