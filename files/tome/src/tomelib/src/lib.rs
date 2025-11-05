
//! The standard for all `util` subcommands, all needed subsystems are here

pub mod exit_code {
    pub const OK: i32 = 0;
    /// User error - triggers help
    pub const USER_ERROR: i32 = 1;
    /// Internal failure
    pub const INTERNAL_ERROR: i32 = 2;
}

use std::str::FromStr;
use std::sync::RwLock;
use std::{io::{stdin, stdout, Read, Write}};
use std::path::PathBuf;

use ctor::ctor;

pub mod config;
pub use config::*;

mod arch;
pub use arch::Arch;

mod output;

pub mod cmd;
pub mod forge;

/// This enum is used to use one of the variants
pub enum Either<A: Sized, B: Sized> {
    First(A),
    Second(B),
}

#[inline]
/// Tells you if debug mode is enabled
pub fn debug() -> bool { unsafe { DEBUG } }

#[inline]
/// Sets debug mode
pub fn set_debug(value: bool) { unsafe { DEBUG = value } }

#[inline]
/// Tells you if verbose mode is enabled
pub fn verbose() -> bool { unsafe { VERBOSE } }

#[inline]
/// Sets verbose mode
pub fn set_verbose(value: bool) { unsafe { VERBOSE = value } }

/// Used to indicate debug mode
/// - Defaults to `false`
/// - Can be modified by `collect_args()` and `collect_args_with_subcmd()` functions
static mut DEBUG: bool = false;
/// Used to indicate verbose mode
/// - defaults to `false`
/// - Can be modified by `collect_args()` and `collect_args_with_subcmd()` functions
static mut VERBOSE: bool = false;


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
pub static TOME_CONFIG: RwLock<TomeConfig> = RwLock::new(TomeConfig::new());
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
fn preload_configs() {
    use std::io::{Error, ErrorKind};

    fn not_configured() -> ! {
        #[cfg(not(feature = "disable_ctor_checks"))]
        fail!(internal: "the project is not configured, please run {} {} to configure it", "./util".green(), "conf".blue());
    }

    let mut configured = false;

    *TOME_CONFIG.write().expect("failed to acquire lock") = match TomeConfig::load() {
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



/// Parses commandline arguments
/// - Returns `(<architectures>, <switches>)`
///   - `<switches>` returns all switches except `-debug` and `-verbose`
/// - Expects `-<switch>` for each argument that is not target architecture
/// 
/// Uses the `-debug` switch to set the util debug mode
/// - You can access or modify the value using `util::debug()` or `util::set_debug()` functions
/// 
/// Uses the `-verbose` switch to set verbose mode
/// - You can access or modify the value using `util::verbose()` or `util::set_verbose()` functions
pub fn collect_args() -> (Vec<Arch>, Vec<String>) {

    let mut args: Vec<String> = std::env::args().collect();

    if args.is_empty() { return (Vec::new(), Vec::new()) }

    //  first argument is exe path given by bash
    args.remove(0);

    let mut switches: Vec<String> = Vec::new();

    let (mut archs, other) = Arch::collect(&args, true);

    if archs.is_empty() {
        archs = TOME_CONFIG.read().expect("failed to acquire lock").arch.clone();
    }

    for o in other.iter() {
        match o.as_str() {
            "-debug" => {
                set_debug(true);
                noteln!("debug mode");
            },
            "-verbose" => {
                set_verbose(true);
                noteln!("turning on verbose output");
            }
            _ => {
                switches.push(o.clone());
            }
        }
    }

    (archs, switches)

}

/// Parses commandline arguments with actions
/// 
/// If argument does not match switch description and does not match arch, it is considered arction
/// - The `A::from_str()` function is used to verify the Action
pub fn collect_args_with_actions<A>() -> (Vec<A>, Vec<Arch>, Vec<String>)
where A: Sized + FromStr, A::Err: std::fmt::Debug {

    let mut args: Vec<String> = std::env::args().collect();

    if args.is_empty() { return (Vec::new(), Vec::new(), Vec::new()) }

    //  first argument is exe path given by bash
    args.remove(0);

    let (mut archs, switches, acts) = Arch::collect_with_actions(&args, true);

    if archs.is_empty() {
        archs = TOME_CONFIG.read().expect("failed to acquire lock").arch.clone();
    }

    let actions = acts.iter().map(|a| {
        match A::from_str(a.as_str()) {
            Ok(a) => a,
            Err(e) => fail!(internal: "unknown action {a}: {e:?}"),
        }
    } ).collect();


    (actions, archs, switches)

}