
//! Allows you to use the `forge` spell

use colored::Colorize;

use crate::{Arch, ArchConfig, current_dir, noteln, fail};
use crate::{KERNEL_CONFIG, KernelConfig, ConfigHolder, PATH};
use crate::{debug, verbose};
use std::fs::copy;
use std::path::PathBuf;


/// Set of parameters for cargo to build the kernel in release mode
/// - the `--target` switch must be at the end
/// replaces the `"{target}"` string with the triplet (ex. `x86_64-unknown-none`)
pub const CARGO_RELEASE_PARAMETERS: &[&str] = &["build", "--release", "--target-dir", "bin", "--target", "{target}"];

/// Set of parameters for cargo to build the kernel in debug mode
/// - the `--target` switch must be at the end
/// replaces the `"{target}"` string with the triplet (ex. `x86_64-unknown-none`)
pub const CARGO_DEBUG_PARAMETERS: &[&str] = &["build", "--target-dir", "bin", "--target", "{target}", "--features", "debug_build"];


/// Forges the kernel for target architecture
/// 
/// Parameters:
/// 1. `arch`: The target architecture
/// 2. `path`: Indicates path to the project
///     - The current directory is used when set to `None`
/// 3. `enable_output`: Enables/disables text output (notifications)
/// 
/// Returns:
/// - `cargo_output` output on success
/// - `(cargo_output, error_message)` on error
/// 
pub fn forge_for(arch: Arch, path: Option<PathBuf>, enable_output: bool) -> Result<String, (Option<String>, String)> {

    let project_root = if let Some(path) = path {
        path
    } else {
        current_dir()
    };

    if enable_output {
        noteln!("forging kernel for {} target", arch.triplet().blue())
    }

    //  load kernel configuration for this target
    let cfg = match ArchConfig::load(arch) {
        Ok(o) => o,
        Err(e) => return Err((None, format!("failed to load config for {arch}: {e:?}")))
    };

    //  go to the kernel directory
    std::env::set_current_dir("./kernel/")
        .map_err(|e| (None, format!("failed to go to the kernel directory: {e}")))?;

    
    let (release, params) = cargo_parameters(arch, debug());


    //  cargo build ...
    let output = forge_kernel(arch, &cfg.compiler, params)?;

    //  go back
    std::env::set_current_dir(project_root).expect("failed to go back to the project root");
    

    //  absolute path to the binary built by cargo
    let from = build_path(arch, release);
    let to = PATH.forged.kernel(arch).expect("failed to get path to the forged kernel");

    //  copy the forged kernel into correct location
    match copy(from, to) {
        Ok(_) => Ok(output),
        Err(e) => Err((Some(output), format!("failed to copy the kernel executable: {e}")))
    }


}

/// Builds the kernel, returns cargo output
fn forge_kernel(arch: Arch, cargo: &String, args: Vec<String>) -> Result<String, (Option<String>, String)> {

    use std::process::Command;

    dbg!(&args);

    //  forces cargo to output colored output anyway
    match Command::new(cargo.as_str()).args(args).env("CARGO_TERM_COLOR", "always").output() {
        Ok(output) => {     //  fail if status is error

            let cargo_output = unsafe { String::from_utf8_unchecked(output.stderr) };

            if !output.status.success() {

                return Err((Some(cargo_output), format!("failed to forge the kernel for the {} target", arch.triplet())));
            }

            Ok(cargo_output)

        },
        Err(e) => {
            return Err((None, format!("failed to forge the kernel for {}: {e}", arch.triplet().blue())))
        }
    }

}

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
/// - Uses constants from `tomelib::forge` to construct the parameters
/// - Returns `(release, parameters)`
///   - The `release` field is used to find the kernel binary
pub fn cargo_parameters(arch: Arch, debug: bool) -> (&'static str, Vec<String>) {
    let mut args: Vec<String>;

    if debug {
        args = CARGO_DEBUG_PARAMETERS.iter().map(|p| {
            if *p == "{target}" {
                arch.triplet().to_string()
            } else {
                p.to_string()
            }
        }).collect();

        //if verbose() { args.push("--verbose".into()); }
        if verbose() { args.insert(1, "--verbose".into()) }
        ("debug", args)
    } else {
        args = CARGO_RELEASE_PARAMETERS.iter().map(|p| {
        if *p == "{target}" {
                arch.triplet().to_string()
            } else {
                p.to_string()
            }
        }).collect();
        if verbose() { args.insert(1, "--verbose".into()); }
        ("release", args)
    }
}