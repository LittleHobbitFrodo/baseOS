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
//  - the `tome` module HAS to be imported this way to ensure its functionality

use tomelib as tome;

fn main() {



}