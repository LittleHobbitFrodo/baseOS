
//! The standard for all `util` subcommands, all needed subsystems are here

pub mod exit_code {
    pub const OK: i32 = 0;
    /// User error - triggers help
    pub const USER_ERROR: i32 = 1;
    /// Internal failure
    pub const INTERNAL_ERROR: i32 = 2;
}

use std::ffi::OsStr;
use std::sync::RwLock;
use std::{io::{stdin, stdout, Read, Write}};
use std::path::PathBuf;

use ctor::ctor;

pub use colored::*;

mod config;
pub use config::*;

mod arch;
pub use arch::Arch;

mod output;
pub use super::{error, errorln, warning, warningln, note, noteln, fail};

pub mod cmd;

/// This enum is used to use one of the variants
pub enum Either<A: Sized, B: Sized> {
    First(A),
    Second(B),
}


/// Asks user a question
/// - returns `Err` upon failure
///   - The inner value hits what went wrong
pub fn ask(question: &str) -> Result<bool, Option<&'static str>> {

    let mut ans = [0u8; 1];

    print!("{question} [y/N]: ");

    stdout().flush().map_err(|_| "failed to flust standard output")?;

    stdin().read(&mut ans).map_err(|_| "failed to read from standard input")?;

    match ans[0].to_ascii_lowercase() {
        b'y' => Ok(true),
        b'n' => Ok(false),
        b'\n' => Ok(false),
        _ => Err(Some("invalid input")),
    }
}

/// Returns the current directory, panics upon failure
pub fn current_dir() -> PathBuf { std::env::current_dir().expect("failed to get current dir") }

/// Returns the current directory as string, panics upon failure
pub fn current_dir_as_string() -> String {
    std::env::current_dir().expect("failed to get current directory").to_string_lossy().to_string()
}


/// Stores the util configuration out of the box
/// - the configuration is loaded before the `main()` function is called
pub static UTIL_CONFIG: RwLock<UtilConfig> = RwLock::new(UtilConfig::new());
/// Stores the kernel configuration out of the box
/// - the configuration is loaded before the `main()` function is called
pub static KERNEL_CONFIG: RwLock<Option<KernelConfig>> = RwLock::new(None);
/// Stores the util configuration out of the box
/// - the configuration is loaded before the `main()` function is called
pub static OS_CONFIG: RwLock<Option<OsConfig>> = RwLock::new(None);


/// Checks if the project is configured, **panics** if not
/// - if the `disable_ctor_checks` feature is set for this executable, panic does not occur
/// 
/// Loads the `UTIL_CONFIG`, `KERNEL_CONFIG`, `OS_CONFIG` into memory
/// - if one of the files does not exit, or there is any other error, sets it to `None`
/// 
/// You can **disable** this function by adding `features=["disable_config_preloading"]` to the `Cargo.toml` configuration for your subcommand
#[cfg(not(feature = "disable_config_preloading"))]
#[ctor]
fn is_configured() {
    use std::io::{Error, ErrorKind};

    fn not_configured() -> ! {
        #[cfg(not(feature = "disable_ctor_checks"))]
        fail!(internal: "the project is not configured, please run {} {} to configure it", "./util".green(), "conf".blue());
    }

    let mut configured = false;

    *UTIL_CONFIG.write().expect("failed to acquire lock") = match UtilConfig::load() {
        Ok(cfg) => {
            configured = cfg.configured;
            cfg
        },
        Err(_) => not_configured(),
    };

    *KERNEL_CONFIG.write().expect("failed to acquire lock") = match KernelConfig::load() {
        Ok(cfg) => Some(cfg),
        Err(e) => {
            errorln!("failed to load kernel configuration: {e:?}");
            None
        },
    };

    *OS_CONFIG.write().expect("failed to acquire lock") = match OsConfig::load() {
        Ok(cfg) => Some(cfg),
        Err(e) => {
            errorln!("failed to load OS configuration: {e:?}");
            None
        },
    };

    if configured == false {
        not_configured();
    }

}



/// parses commandline arguments
/// - returns `(<architectures>, <switches>, debug, verbose)`
///   - `<switches>` returns all switches except `-debug` and `-verbose`
/// - expects `-<switch>` for each argument that is not target architecture
pub fn collect_args() -> (Vec<Arch>, Vec<String>, bool, bool) {

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    let mut debug = false;

    let mut verbose = false;

    let mut switches: Vec<String> = Vec::new();

    let (mut archs, other) = Arch::collect(&args, true);

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
                switches.push(o.clone());
            }
        }
    }

    (archs, switches, debug, verbose)

}