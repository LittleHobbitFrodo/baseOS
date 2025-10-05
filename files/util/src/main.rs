/*  This is a template for your custom subcommand. Simply copy the `main.rs` file and edit it.
    - util subcommands are simply rust executables
    - subcommands are built by running `./util init` or `./util reinit`
      - alternatively you can use the `./util --rebuild-tools` command
    - the new command will carry the name of the file without its `.rs` extension
      - add bin entry with the name and path to the file to the `Cargo.toml` file
      ```toml
      [[bin]]
      name="<subcmd name>"
      path="<path to file>"
      ```
*/

//  This module provides simple API for utility commands
mod util;
use util::*;

use std::fs::File;
use std::io::Read;

fn main() {

    let mut file = File::open("example.toml").map_err(|e| ConfigError::FailedToOpenFile(e))
        .expect("failed to open file");

    //  read the file
    let mut content = String::with_capacity(64);
    file.read_to_string(&mut content).map_err(|_| ConfigError::FailedToReadFile)
        .expect("failed to read file");

    let table: toml::Table = match toml::from_str(content.as_str()) {
        Ok(tab) => tab,
        Err(e) => panic!("failed: {e:?}"),
    };

    for (name, val) in table {
        println!("{name}:\t{val:?}");
    }

}

//  TODO?: move all configs into one
//  TODO: add scripts built attribute into the util config