
pub mod exit_code {
    pub const OK: i32 = 0;
    /// invalid parameters, ...
    pub const USER_ERROR: i32 = 1;
    /// Internal failure
    pub const INTERNAL_ERROR: i32 = 2;
}

use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{ffi::os_str::Display, io::{stdin, stdout, Read, Write}};

pub use colored::*;

mod config;
pub use config::{PATH, KernelConfig};


/// Reports error
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use $crate::Colorize;
        print!("{}: ", "error".red());
        print!($($arg)*);
    }};
}

/// Reports error and breaks the line
#[macro_export]
macro_rules! errorln {
    ($($arg:tt)*) => {{
        use $crate::Colorize;
        print!("{}: ", "error".red());
        println!($($arg)*);
    }};
}

/// Reports warning
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        use $crate::Colorize;
        print!("{}: ", "warning".yellow());
        print!($($arg)*);
    }};
}


/// Reports warning and breaks the line
#[macro_export]
macro_rules! warningln {
    ($($arg:tt)*) => {{
        use $crate::Colorize;
        print!("{}: ", "warning".yellow());
        println!($($arg)*);
    }};
}

/// Notifies user
#[macro_export]
macro_rules! note {
    ($($arg:tt)*) => {{
        use $crate::Colorize;
        print!("{}: ", "note".blue());
        print!($($arg)*);
    }};
}

/// Notifies user and breaks the line
#[macro_export]
macro_rules! noteln {
    ($($arg:tt)*) => {{
        use $crate::Colorize;
        print!("{}: ", "note".blue());
        print!($($arg)*);
    }};
}

/// Prints error message and exits the program
/// 
/// usage:
/// - internal error: `fail!(internal, "formatted {}", "message")`
/// - user error: `fail!(userm "formatted {}", "message")`
///   - user error triggers the help menu
#[macro_export]
macro_rules! fail {
    (internal, $($arg:tt)*) => {{
        $crate::error!($($arg)*);
        std::process::exit($crate::exit_code::INTERNAL_ERROR);
    }};
    (user, $($arg:tt)*) => {{
        $crate::error!($($arg)*);
        std::process::exit($crate::exit_code::USER_ERROR);
    }};
}

#[derive(Copy, Clone, Debug)]
pub enum Arch {
    X86_64,
    Arm64,
}

impl Arch {

    /// Converts string to the `Arch` enum
    /// - case sensitive
    pub fn from_str(from: &'_ str) -> Result<Self, ()> {
        Ok(match from {
            "x86_64" => Self::X86_64,
            "amd64" => Self::X86_64,
            "x64" => Self::X86_64,
            "arm" => Self::Arm64,
            "arm64" => Self::Arm64,
            "aarch64" => Self::Arm64,
            _ => { return Err(()) }
        })
    }

    /// Converts the `Arch` enum into `str`
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Arm64 => "arm64",
            Self::X86_64 => "x86_64",
        }
    }

    /// Converts the `Arch` enum to `String`
    pub fn to_string(&self) -> String {
        match self {
            Self::Arm64 => String::from("arm64"),
            Self::X86_64 => String::from("x86_64"),
        }
    }

}

impl std::fmt::Display for Arch {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}


/// Tries to search for a command in `PATH`
pub fn search_path(command: &str) -> Result<String, ()> {

    let path = std::env::var("PATH").map_err(|_| ())?;

    let paths = path.split(':');

    for i in paths {
        let files = match std::fs::read_dir(i) {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        for file in files {
            if let Ok(f) = file {
                let name = f.file_name();
                if name == command {
                    return Ok(name.into_string().map_err(|_| ())?)
                }
            }
        }
    }

    Err(())

}

/// Asks user a question
/// - returns `Err` upon failure
///   - The inner value hits what went wrong
/// - question without question mark
pub fn ask(question: &str) -> Result<bool, Option<String>> {

    let mut ans = [0u8; 1];

    print!("{question}? [y/N]: ");

    stdout().flush().map_err(|_|None)?;

    stdin().read(&mut ans).map_err(|_|None)?;

    match ans[0].to_ascii_lowercase() {
        b'y' => Ok(true),
        b'n' => Ok(false),
        b'\n' => Ok(false),
        _ => Err(Some(String::from("invalid input"))),
    }
}

static PWD: RwLock<String> = RwLock::new(String::new());
static ARGS: RwLock<Vec<String>> = RwLock::new(Vec::new());

/// Returns the current working directory
/// - do not use the `std::env::current_dir()`
pub fn current_dir() -> PathBuf {
    let pwd = PWD.read().expect("failed to acquire lock");
    PathBuf::from_str(&*pwd).expect("failed to parse path")
}

/// Tries to get the current working directory
/// - returns `Err` upon error
pub fn try_current_dir() -> Result<PathBuf, ()> {
    let pwd = PWD.read().map_err(|_| ())?;
    Ok(PathBuf::from_str(&*pwd).map_err(|_| ())?)
}

/// Returns the current working directory as `String`
/// - lower overhead than `current_dir().to_string_lossy()`
pub fn current_dir_as_string() -> String {
    PWD.read().expect("failed to acquire lock").clone()
}

/// Tries to get the current working directory
pub fn try_current_dir_as_string() -> Result<String, ()> {
    Ok(PWD.read().map_err(|_| ())?.clone())
}

/// Returns reference to the collected program arguments
pub fn args<'l>() -> RwLockReadGuard<'l, Vec<String>> {
    ARGS.read().expect("failed to acquire lock")
}

/// Tries to get reference to the collected program arguments
pub fn try_args<'l>() -> Result<RwLockReadGuard<'l, Vec<String>>, ()> {
    ARGS.read().map_err(|_| ())
}

/// Tries to get mutable reference to the collected program arguments
pub fn try_args_mut<'l>() -> Result<RwLockWriteGuard<'l, Vec<String>>, ()> {
    ARGS.write().map_err(|_| ())
}

/// Returns mutable reference to the collected program arguments
pub fn args_mut<'l>() -> RwLockWriteGuard<'l, Vec<String>> {
    ARGS.write().expect("failed to acquire lock")
}


/// Initializes the util module
pub fn initialize() {
    let mut args: Vec<String> = std::env::args().collect();

    let mut index = args.iter().position(|arg| *arg == "--DIR")
        .expect("no DIR specified");

    index += 1;

    if index == args.len() {
        panic!("no path for DIR");
    }

    *PWD.write().expect("failed to acquire lock") = args.remove(index);
    args.remove(index - 1);

    *ARGS.write().expect("failed to acquire lock") = args;


}