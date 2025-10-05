
pub mod exit_code {
    pub const OK: i32 = 0;
    /// User error - triggers help
    pub const USER_ERROR: i32 = 1;
    /// Internal failure
    pub const INTERNAL_ERROR: i32 = 2;
}

use std::str::FromStr;
use std::{io::{stdin, stdout, Read, Write}};
use std::process::{Child, Command, ExitCode};

pub use colored::*;

mod config;
pub use config::*;
use serde::{Deserialize, Serialize};

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
        println!($($arg)*);
    }};
}

/// Prints error message and exits the program
/// 
/// usage:
/// - internal error: `fail!(internal: "formatted {}", "message")`
/// - user error: `fail!(user: "formatted {}", "message")`
///   - user error triggers the help menu
#[macro_export]
macro_rules! fail {
    (internal: $($arg:tt)*) => {{
        $crate::error!($($arg)*);
        std::process::exit($crate::exit_code::INTERNAL_ERROR);
    }};
    (user: $($arg:tt)*) => {{
        $crate::error!($($arg)*);
        std::process::exit($crate::exit_code::USER_ERROR);
    }};
    (internal) => {{
        std::process::exit($crate::exit_code::INTERNAL_ERROR);
    }};
    (user) => {{
        std::process::exit($crate::exit_code::USER_ERROR);
    }};
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[repr(u32)]
pub enum Arch {
    None,
    X86_64,
    Arm64
}

impl Default for Arch {
    fn default() -> Self { Self::None }
}


impl FromStr for Arch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "x86_64" => Self::X86_64,
            "amd64" => Self::X86_64,
            "x64" => Self::X86_64,
            "arm" => Self::Arm64,
            "arm64" => Self::Arm64,
            "aarch64" => Self::Arm64,
            _ => { return Err(()) }
        })
    }
}


impl Arch {

    /// returns normalized name for target architecture
    pub fn normalize(&self) -> &'static str {
        match self {
            Self::Arm64 => "aarch64",
            Self::None => "none",
            Self::X86_64 => "x86_64",
        }
    }

    /// indicates whether an architecture is supported
    /// - case sensitive
    pub fn is_supported(arch: &'_ str) -> bool {
        match Self::from_str(arch) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Returns list of all supported architectures (as strings)
    pub const fn all_supported() -> &'static [&'static str] {
        &["X86_64", "Arm64"]
    }

    /// Converts the `Arch` enum into `str`
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Arm64 => "Arm64",
            Self::X86_64 => "X86_64",
            Self::None => "None"
        }
    }

    /// Converts the `Arch` enum to `String`
    pub fn to_string(&self) -> String {
        match self {
            Self::Arm64 => String::from("Arm64"),
            Self::X86_64 => String::from("X86_64"),
            Self::None => String::from("None")
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
                    //return Ok(name.into_string().map_err(|_| ())?)
                    return Ok(f.path().to_str().unwrap().to_string());
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

pub fn current_dir_as_string() -> String {
    std::env::current_dir().expect("failed to get current directory").to_string_lossy().to_string()
}

pub fn cmd(cmd: &'_ str, args: Option<&'_ [&'_ str]>) -> Result<ExitCode, std::io::Error> {

    let mut cmd = match args {
        Some(args) => Command::new(cmd).args(args).spawn()?,
        None => Command::new(cmd).spawn()?,
    };


    cmd.wait().map(|stat| ExitCode::from(stat.code().expect("failed to get exit code") as u8))
}

pub fn cmd_async(cmd: &'_ str, args: Option<&'_ [&'_ str]>) -> Result<Child, std::io::Error> {

    Ok(match args {
        Some(args) => Command::new(cmd).args(args).spawn()?,
        None => Command::new(cmd).spawn()?,
    })

}



#[macro_export]
macro_rules! cmd {
    ($cmd:expr) => {
        cmd($cmd, None)
    };
    ($cmd:expr, $($arg:expr),* $(,)?) => {{
        cmd($cmd, Some(&[$($arg),*]))
    }};
}