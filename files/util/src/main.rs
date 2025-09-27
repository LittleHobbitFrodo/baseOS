//! Dependencies can be specified in the script file itself as follows:
//! - and should always be on top of the file
//!
//! ```cargo
//! [dependencies]
//! colored = "3.0.0"
//! ```

/* This is a template for your new commands. Simply copy it before editing
    - all util commands are run by `rust-script` - a rust code interpretter
    - all commands are in this directory and are named <cmd name>.rs
*/


//  This module provides simple API for utility commands
mod util;
use util::*;

fn main() {

    //  always call this function on start of the script
    initialize();

    //  use the `util::current_dir()` function instead of the `std::env::current_dir()`
    //      `rust-script` modifies the path of the working directory
    println!("pwd: {}", current_dir().to_string_lossy());

    //  report error
    errorln!("some error occured!");

    if let Ok(args) = try_args() {
        print!("arguments:\t");
        for arg in args.iter() {
            print!(" {arg}");
        }
    } else {
        //  reports error and exits the script
        fail!(internal, "failed to get program arguments");
            //  user errors triggers the help menu
    }

}

