mod util;
use std::{fs::File, str::FromStr};

use util::*;

fn main() {


    let cfg = match ArchConfig::load(Arch::X86_64) {
        Ok(c) => c,
        Err(e) => fail!(internal: "{e:?}"),
    };

    dbg!(cfg);

}