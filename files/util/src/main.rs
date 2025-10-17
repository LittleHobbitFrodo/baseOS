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

    println!("hello world!");
    println!("limine:");
    println!("\tcmdline:    {}", PATH.limine.cmdline_string());
    println!("\tconf dir:   {}", PATH.limine.config_dir_string());
    println!("\tconfig:     {}", PATH.limine.config_string());
    println!("\tdebug conf: {}", PATH.limine.config_debug_string());
    println!("\tdata dir:   {}", PATH.limine.data_dir_string());
    println!("\tbootloader: {}", PATH.limine.bootloader_for_string(Arch::X86_64).expect("failure"));

}