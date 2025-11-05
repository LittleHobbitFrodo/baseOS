
//! Forges (builds) the kernel for specified architectures

use colored::*;
use tomelib as tome;
use tome::*;
use tome::forge;

fn main() {

    let (archs, switches) = collect_args();

    if !switches.is_empty() {
        failln!(internal: "unknown switch {}", switches[0].blue());
    }

    for arch in archs {

        match forge::forge_for(arch, None, true) {
            Ok(cargo_output) => {
                if verbose() {
                    println!("{}\n", cargo_output);
                }
                noteln!("forging for {} completed", arch.triplet().blue());
            },
            Err((cargo_output, msg)) => {
                if let Some(out) = cargo_output {
                    println!("{}\n", out);
                }
                fail!(internal: "failed to forge kernel for {}:\t{msg}", arch.triplet().blue())
            }
        }

    }

    noteln!("forging completed");
    

}
