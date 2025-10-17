

mod util;
use std::{fs::copy, io::Read, path::PathBuf, process::{ChildStdout, ExitCode, ExitStatus}, str::FromStr};


use paste::paste;
use serde::de;
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

    let (archs, debug, verbose) = collect_args();

    let project_root = current_dir();

    for arch in archs.iter() {

        noteln!("forging kernel for {} target", arch.as_str());

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

        if let Err(e) = copy(from, to) {
            fail!(internal: "failed to copy forged kernel: {e:?}");
        }

        noteln!("the core for architecture {} was successfully forged", arch.as_str());

    }
    

}

/// Forges the kernel for target architecture
/// - panics upon failure
/// - prints the cargo output if the `verbose` is set to true
fn forge_for(arch: Arch, cargo: &String, args: Vec<String>) {

    match cmd_with_output(cargo.as_str(), Some(args)) {
        Ok((status, _)) => {
            //  fail if status is error, output is printed anyway bruh

            if !status.success() {
                fail!(internal: "failed to forge the kernel for the {} target", arch.triplet().unwrap().blue());
            }
        },
        Err(e) => {
            fail!(internal: "failed to forge the kernel for the {} target: {e}", arch.triplet().unwrap().blue());
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
    current.push(format!("kernel/bin/{}/{release}/{}", arch.triplet().unwrap(), kernel.name));
    current

}

/// Returns build mode and list of parameters to pass to cargo
/// - appends the triplet (`<arch>-unknown-none`) at the end
/// - append `--verbose` to the end if `verbose` is `true`
fn cargo_parameters(arch: Arch, debug: bool, verbose: bool) -> (&'static str, Vec<String>) {
    let mut args: Vec<String>;
    if debug == true {
        args = CARGO_DEBUG_PARAMETERS.iter().map(|p| p.to_string()).collect();
        args.push(arch.triplet().unwrap());
        if verbose { args.push("--verbose".into()); }
        ("debug", args)
    } else {
        args = CARGO_RELEASE_PARAMETERS.iter().map(|p| p.to_string()).collect();
        args.push(arch.triplet().unwrap());
        if verbose { args.push("--verbose".into()); }
        ("release", args)
    }
}

/// parses commandline arguments
/// - returns `(<architectures>, debug, verbose)`
/// - expects `-<switch>` for each argument that is not target architecture
///   - for example: `./util forge x86_64 -debug`
fn collect_args() -> (Vec<Arch>, bool, bool) {

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    let mut debug = false;

    let mut verbose = false;

    let (mut archs, other) = collect_arch(&args, true);

    if archs.is_empty() {
        archs = UTIL_CONFIG.read().expect("failed to acquire lock").arch.clone();
    }

    for o in other.iter() {
        match o.as_str() {
            "-debug" => {
                debug = true;
                noteln!("forging with debug option");
            },
            "-verbose" => {
                verbose = true;
                noteln!("turning on verbose output");
            }

            _ => {
                fail!(user: "unknown switch {}", o.blue());
            }
        }
    }

    (archs, debug, verbose)

}