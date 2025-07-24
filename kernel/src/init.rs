//  init.rs
//  this file originally belonged to baseOS project
//      on OS template on which to build

//  module for kernel initialization

/*use ministd::mem::string::searcher::{StrSearcher, SearchStep};
use ministd::mem::string::Searcher;
use ministd::renderer::RENDERER;
use ministd::string::searcher::CharPredicateSearcher;
use ministd::string::ReverseSearcher;
use ministd::{dbg, io, locked_eprintln, panic_fmt, Color, Rc};
use ministd::{println, print, locked_print, locked_println, eprintln, init};
use ministd::{Box, Array, Vec, String, HashMap, vec};*/

//use crate::manage::*;

use ministd::entry;
use ministd::init;
use ministd::{panic_fmt, print, println};


/// This is the kernel entry point, the point that is called by the `ministd` library after boot
/// 
/// Feel free to change the behaviour of this function
/// - the function needs to have the `#[ministd::init]` attribute and must **never** return (`-> !`)
/// 
/// Note that the entry point can be changed via the `bootloader` crate
/// - use the `EntryPointRequest` structure
#[entry]
pub fn kernel_entry() -> ! {

    if let Err(_) = init() {
        panic!("failed to initialize kernel");
    }

    ministd::hang();

}

/// This function is here to initialize your kernel
/// 
/// you can initialize various kernel functions and submodules here
fn init() -> Result<(), ()> {

    if let Err(_) = init::renderer() {
        panic!("failed to initialize renderer");
    }

    if let Err(msg) = init::allocator() {
        if let Some(msg) = msg {
            panic_fmt!("failed to initialize heap: {msg}")
        } else {
            panic!("failed to initialize heap");
        }
    }

    println!("hello world!");

    /*let f = |x: u8| x == b'f';


    let arr = [
        ("The quick brown fox jumps over the lazy dog"),
        ("The quick brown fox"),
        ("match this string"),
        ("end with this"),
        ("aaaabaaaab"),
        ("abababab"),
        ("fhello world!")
    ];

    let x = "";

    for i in arr.iter() {
        check_searcher(i, x, f);
    }*/

    Ok(())

}

/*
fn check_searcher<'h, 'n, F>(haystack: &'h str, needle: &'n str, predicate: F)
where F: FnMut(u8) -> bool + Clone {

    //let mut s = StrSearcher::new(haystack, needle);
    let mut s = CharPredicateSearcher::new(haystack, predicate);

    //RENDERER.lock().set_color(0x99ff99);
    let mut rend = ministd::RENDERER.lock();
    rend.set_color(RED);
    locked_print!(rend, "{haystack}");
    rend.set_color(WHITE);
    locked_print!(rend, " | ");
    rend.set_color(GREEN);
    locked_println!(rend, "{needle}");

    rend.set_color(0xff8888);

    let mut result: SearchStep;
    for _ in 0..4 {

        result = s.next();

        match result {
            SearchStep::Match(start, end) => {
                rend.set_color(GREEN);
                locked_print!(rend, "match");
                rend.set_color(WHITE);
                locked_println!(rend, ": {start}..{end} = \"{}\"", &haystack[start..end]);
            },
            SearchStep::Reject(start, end) => {
                rend.set_color(RED);
                locked_print!(rend, "reject");
                rend.set_color(WHITE);
                locked_println!(rend, ": {start}..{end} = \"{}\"", &haystack[start..end]);
            },
            SearchStep::LastMatch(start, end) => {
                rend.set_color(GREEN);
                locked_print!(rend, "last match");
                rend.set_color(WHITE);
                locked_println!(rend, ": {start}..{end} = \"{}\"", &haystack[start..end]);
                break;
            },
            SearchStep::LastReject(start, end) => {
                rend.set_color(RED);
                locked_print!(rend, "last reject");
                rend.set_color(WHITE);
                locked_println!(rend, ": {start}..{end} = \"{}\"", &haystack[start..end]);
                break;
            },
            SearchStep::Done => {
                rend.set_color(WHITE);
                locked_println!(rend, "DONE");
                break;
            }
        }
    }

    locked_eprintln!(rend, "\n---------------------------\n");

}

const GREEN: u32 = 0x88ff88;
const RED: u32 = 0xff8888;
const WHITE: u32 = 0xffffff;*/