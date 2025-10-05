mod util;
use std::{fs::File, process::ExitCode, str::FromStr};

use util::*;


const ISO_NAME_DEFAULT: &'static str = "BaseOS";

const KERNEL_NAME_DEFAULT: &'static str = "BaseOS-kernel";
const KERNEL_VERSION_DEFAULT: &'static str = "0.0.1";   //  format X.Y.Z
const KERNEL_RELEASE_DEFAULT: &'static str = "BaseOS-v0.0.1";

const OS_NAME_DEFAULT: &'static str = "BaseOS";
const OS_VERSION_DEFAULT: &'static str = "0.0.1";       //  format X.Y.Z
const OS_RELEASE_DEFAULT: &'static str = "BaseOS-v0.0.1";


static mut ALL_INSTALLED: bool = true;

fn main() {


    let archs = collect_args();

    noteln!("creating utility configuration");
    if let Err(e) = create_util_config(&archs) {
        fail!(internal: "failed to create util config: {e:?}");
    }

    noteln!("adding rustup targets");
    add_targets(&archs);

    build_limine_cmd();
    noteln!("limine cmdline utility build was successfull\n");

    if unsafe { ALL_INSTALLED } {
        println!("{}", "the project is ready to use!".green());
    } else {
        warningln!("some packages are missing, please run {} {} to install them", "./util".green(), "dep install all".blue());
    }

}

/// Collects arguments and returns vector of supported architectures
/// - returned vector does not include duplicit entries
fn collect_args() -> Vec<Arch> {

    let mut args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        fail!(user: "expected at least one architecture");
    }

    args.remove(0);

    let mut archs = Vec::new();

    //  iterate through arcitectures and find duplicates
    for arg in args.iter() {
        if let Ok(arch) = Arch::from_str(arg) {
            if archs.contains(&arch) {
                warningln!("architecture {} is already listed", arch.as_str().blue());
            } else {
                archs.push(arch);
            }
        } else {
            fail!(user: "unknown architecture {}", arg.blue());
        }
    }

    archs

}

/// Creates utility config and arch specific configs
/// - on error returns `Err((<config name>, <error>))`
fn create_util_config(archs: &Vec<Arch>) -> Result<(), (&'static str, ConfigError)> {

    let (xorriso, cargo, qemu) = find_tools(archs);

    //  create util config
    let cfg = UtilConfig {
        configured: true,
        xorriso,
        iso_name: ISO_NAME_DEFAULT.into(),
        arch: archs.clone()
    };

    cfg.store().map_err(|e| ("utility", e))?;


    //  create arch specific configs

    for (arch, emul) in qemu {
        noteln!("creating config for {}", arch.as_str().blue());
        if let Err(e) = ArchConfig::from_parts(arch, cargo.clone(), emul.clone()).store() {
            fail!(internal: "fialed to create config for the {arch} target: {e:?}");
        }
    }


    //  crate kernel configuration
    KernelConfig {
        name: KERNEL_NAME_DEFAULT.into(),
        version: match Version::from_str(KERNEL_VERSION_DEFAULT) {
            Ok(v) => v,
            Err(_) => fail!(internal: "failed to parse the default kernel version ({KERNEL_VERSION_DEFAULT})"),
        },
        release: KERNEL_RELEASE_DEFAULT.into()
    }.store().map_err(|e| ("kernel", e))?;

    //  create OS configuration
    OsConfig {
        name: OS_NAME_DEFAULT.into(),
        version: match Version::from_str(OS_VERSION_DEFAULT) {
            Ok(v) => v,
            Err(_) => fail!(internal: "failed to parse the default OS version ({OS_VERSION_DEFAULT})"),
        },
        release: OS_RELEASE_DEFAULT.into(),
    }.store().map_err(|e| ("OS", e))?;

    Ok(())

}

/// Finds needed tools for `create_util_config`
/// - returns `(xorriso, cargo, Vec<emulator>)`
///   - if emulator for a target is not found, appeds it and leaves the string empty
fn find_tools(archs: &Vec<Arch>) -> (String, String, Vec<(Arch, String)>) {

    fn not_installed_warning(cmd_name: &str) {
        warningln!("{} is not installed, please run {} {}", cmd_name.blue(), "./util".green(), "dep install all + reconf".blue());
    }

    let xorriso = match search_path("xorriso") {
        Ok(x) => x,
        Err(_) => {
            not_installed_warning("xorriso");
            unsafe { ALL_INSTALLED = false };
            String::new()
        }
    };

    let cargo = match search_path("cargo") {
        Ok(c) => c,
        Err(_) => {
            not_installed_warning("cargo");
            unsafe { ALL_INSTALLED = false };
            String::new()
        }
    };

    let mut emuls = Vec::new();

    for a in archs {
        if let Ok(emul) = search_path(format!("qemu-system-{}", a.normalize()).as_str()) {
            emuls.push((*a, emul));
        } else {
            warningln!("{} for {} is not installed, please run {} {}", "qemu".blue(), *a, "./util".green(), "dep install all + reconf".blue());
            unsafe { ALL_INSTALLED = false };
            emuls.push((*a, String::new()));
        }
    }

    (xorriso, cargo, emuls)

}


/// tells rustup to add targets
/// - panics upon failure
fn add_targets(archs: &Vec<Arch>) {

    let rustup = match search_path("rustup") {
        Ok(r) => r,
        Err(_) => {
            fail!(internal: "could not find {}, please install it", "rustup".blue());
        },
    };

    for arch in archs.iter() {
        let triplet = format!("{}-unknown-none", arch.normalize());
        match cmd!(rustup.as_str(), "target", "add", triplet.as_str()) {
            Ok(code) => if code != ExitCode::SUCCESS { fail!(internal: "failed to add target for {arch}"); },
            Err(e) => fail!(internal: "failed to add rustup target: {e:?}"),
        }
    }

}

/// builds the limine commandline utility
fn build_limine_cmd() {

    let limine_cmd = match PATH.limine_path_string(LiminePath::CmdLine) {
        Ok(cmd) => cmd,
        Err(_) => fail!(internal: "failed to get path to limine command line utility"),
    };

    if std::fs::exists(&limine_cmd).expect("unexpected error") {
        if let Err(e) = std::fs::remove_file(&limine_cmd) {
            fail!(internal: "failed to remove limine cmd: {e:?}");
        }
    }


    match cmd!("gcc", "-o", limine_cmd.as_str(), "bootloader/limine.c") {
        Ok(code) => if code != ExitCode::SUCCESS { fail!(internal: "failed to build limine cmdline utility"); },
        Err(e) => fail!(internal: "failed to build limine cmdline utility: {e:?}"),
    }

}