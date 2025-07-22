//  init.rs
//  this file originally belonged to baseOS project
//      on OS template on which to build

//  module for kernel initialization

use ministd::mem::string::searcher::{StrSearcher, SearchStep};
use ministd::mem::string::Searcher;
use ministd::renderer::RENDERER;
use ministd::string::ReverseSearcher;
use ministd::{dbg, io, locked_eprintln, panic_fmt, Color, Rc};
use ministd::{println, print, locked_print, locked_println, eprintln, init};
use ministd::{Box, Array, Vec, String, HashMap, vec};

use crate::manage::*;

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

    let arr = &[
        ("The quick brown fox jumps over the lazy dog", "fox"),
        ("The quick brown fox", "cat"),
        ("match this string", "match"),
        ("end with this", "this"),
        ("aaaabaaaab", "aaab"),
        ("abababab", "ab"),
        ("hello world!", "hello world!")
    ];

    for i in arr.iter() {
        check_searcher(i.0, i.1);
    }

    Ok(())

}

/// This is the first function that is called by the bootloader
/// 
/// Make sure to put your initialization function here :)
#[unsafe(no_mangle)]
extern "C" fn _start() {

    io::int::disable();

    if let Err(_) = init() {
        panic!("failed to initialize the kernel");
    }

    ministd::hang();
}

fn check_searcher<'h, 'n>(haystack: &'h str, needle: &'n str) {

    let mut s = StrSearcher::new(haystack, needle);

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

        result = s.next_back();

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
const WHITE: u32 = 0xffffff;