
//! Forges (builds) the kernel for specified architectures

mod util;
use std::{fs::copy, path::PathBuf};
use util::*;


/// Set of parameters for cargo to build the kernel in release mode
/// - the `--target` switch must be at the end
///   - the kernel triplet (`<arch>-unknown-none`) appended to the end in the `cargo_parameters()` function
const CARGO_RELEASE_PARAMETERS: &'static [&'static str] = &["build", "--target-dir", "bin", "--target"];

/// Set of parameters for cargo to build the kernel in debug mode
/// - the `--target` switch must be at the end
///   - the kernel triplet (`<arch>-unknown-none`) appended to the end in the `cargo_parameters()` function
const CARGO_DEBUG_PARAMETERS: &'static [&'static str] = &["build", "--release", "--target-dir", "bin", "--target"];

fn main() {

    let (archs, _, debug, verbose) = collect_args();

    let project_root = current_dir();

    for arch in archs.iter() {

        noteln!("forging kernel for {} target", arch);

        //  load kernel configuration for this target
        let cfg = match ArchConfig::load(*arch) {
            Ok(o) => o,
            Err(e) => fail!(internal: "failed to load config for {arch}: {e:?}")
        };



        //  go to the kernel directory
        std::env::set_current_dir("./kernel/").expect("failed to go to the kernel directory");

        let (release, params) = cargo_parameters(*arch, debug, verbose);

        //  cargo build ...
        forge_for(*arch, &cfg.compiler, params);

        //  go back
        std::env::set_current_dir(&project_root).expect("failed to go back to the project root");
        


        //  absolute path to the binary built by cargo
        let from = build_path(*arch, release);
        let to = PATH.forged.kernel(*arch).expect("failed to get path to the forged kernel");

        //  copy the forged kernel into correct location
        if let Err(e) = copy(from, to) {
            fail!(internal: "failed to copy forged kernel: {e:?}");
        }


        noteln!("the kernel for the {} target was successfully forged", arch.triplet());

    }
    

}

/// Forges the kernel for target architecture
/// - panics upon failure
fn forge_for(arch: Arch, cargo: &String, args: Vec<String>) {

    match cmd::cmd_with_output(cargo.as_str(), Some(args)) {
        Ok((status, _)) => {
            //  fail if status is error, output is printed anyway bruh

            if !status.success() {
                fail!(internal: "failed to forge the kernel for the {} target", arch.triplet().blue());
            }
        },
        Err(e) => {
            fail!(internal: "failed to forge the kernel for the {} target: {e}", arch.triplet().blue());
        }
    }

}

/// Returns absolute path to the built kernel
/// - built by cargo, not the util
/// - panics upon failure
fn build_path(arch: Arch, release: &'_ str) -> PathBuf {
    let guard = KERNEL_CONFIG.write().expect("failed to acquire lock");

    let kernel = if let Some(cfg) = &*guard {
        cfg.clone()
    } else {
        match KernelConfig::load() {
        Ok(cfg) => cfg,
            Err(e) => fail!(internal: "cannot access the kernel configuration, please run {} {}:\t{e:?}", "./util".green(), "reconf".blue()),
        }
    };

    let mut current = current_dir();
    current.push(format!("kernel/bin/{}/{release}/{}", arch.triplet(), kernel.name));
    current

}

/// Returns build mode and list of parameters to pass to cargo
/// - appends the triplet (`<arch>-unknown-none`) at the end
/// - append `--verbose` to the end if `verbose` is `true`
fn cargo_parameters(arch: Arch, debug: bool, verbose: bool) -> (&'static str, Vec<String>) {
    let mut args: Vec<String>;
    if debug == true {
        args = CARGO_DEBUG_PARAMETERS.iter().map(|p| p.to_string()).collect();
        args.push(arch.triplet().to_string());
        if verbose { args.push("--verbose".into()); }
        ("debug", args)
    } else {
        args = CARGO_RELEASE_PARAMETERS.iter().map(|p| p.to_string()).collect();
        args.push(arch.triplet().to_string());
        if verbose { args.push("--verbose".into()); }
        ("release", args)
    }
}
