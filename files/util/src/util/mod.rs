
pub mod exit_code {
    pub const OK: i32 = 0;
    /// User error - triggers help
    pub const USER_ERROR: i32 = 1;
    /// Internal failure
    pub const INTERNAL_ERROR: i32 = 2;
}

use std::ffi::OsStr;
use std::str::FromStr;
use std::sync::RwLock;
use std::{io::{stdin, stdout, Read, Write}};
use std::process::{Child, ChildStdout, Command, ExitStatus, Stdio};
use std::path::PathBuf;

use ctor::ctor;

pub use colored::*;

mod config;
pub use config::*;
use serde::{Deserialize, Serialize};

/// This enum is used to use One one of the variants
pub enum Either<A: Sized, B: Sized> {
    First(A),
    Second(B),
}


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

    /// Returns the bootloader file name for specific architecture
    /// - `BOOTX64.EFI` for x86_64, `BOOTAA64.EFI` for aarch64, etc.
    pub fn bootloader_name(&self) -> Result<&'static str, ()> {
        Ok(match self {
            Self::None => return Err(()),
            Self::Arm64 => "BOOTAA64.EFI",
            Self::X86_64 => "BOOTX64.EFI",
        })
    }

    /// Returns the triplet (`<arch>-unknown-none`) for this architecture
    /// - returns `Err` if self is `Arch::None`
    pub fn triplet(&self) -> Result<String, ()> {
        if let Self::None = self {
            Err(())
        } else {
            Ok(format!("{}-unknown-none", self.normalize()))
        }
    }

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

/// Returns the current directory, panics upon failure
pub fn current_dir() -> PathBuf { std::env::current_dir().expect("failed to get current dir") }

/// Returns the current directory as string, panics upon failure
pub fn current_dir_as_string() -> String {
    std::env::current_dir().expect("failed to get current directory").to_string_lossy().to_string()
}

/// Runs command/program with or without parameters
pub fn cmd<S, I>(cmd: &'_ str, args: Option<I>) -> Result<ExitStatus, std::io::Error>
where I: IntoIterator<Item = S>, S: AsRef<OsStr> {
    let mut cmd = match args {
        Some(args) => Command::new(cmd).args(args).spawn()?,
        None => Command::new(cmd).spawn()?,
    };

    Ok(cmd.wait()?)
}

/// Runs command/program and collects its output
pub fn cmd_with_output<S, I>(cmd: &'_ str, args: Option<I>) -> Result<(ExitStatus, Option<ChildStdout>), std::io::Error>
where I: IntoIterator<Item = S>, S: AsRef<OsStr> {

    let mut cmd = match args {
        Some(args) => Command::new(cmd).args(args).stdout(Stdio::piped()).spawn()?,
        None => Command::new(cmd).spawn()?,
    };

    let output = cmd.stdout.take();

    Ok((cmd.wait()?, output))

}

/// runs command and leaves its handle
pub fn cmd_async<S, I>(cmd: &'_ str, args: Option<I>) -> Result<Child, std::io::Error>
where I: IntoIterator<Item = S>, S: AsRef<OsStr> {
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



/// Stores the util configuration out of the box
/// - the configuration is loaded before the `main()` function is called
pub static UTIL_CONFIG: RwLock<UtilConfig> = RwLock::new(UtilConfig::new());
/// Stores the kernel configuration out of the box
/// - the configuration is loaded before the `main()` function is called
pub static KERNEL_CONFIG: RwLock<Option<KernelConfig>> = RwLock::new(None);
/// Stores the util configuration out of the box
/// - the configuration is loaded before the `main()` function is called
pub static OS_CONFIG: RwLock<Option<OsConfig>> = RwLock::new(None);


#[cfg(not(feature = "disable_ctor_checks"))]
#[ctor]
fn is_configured() {
    use std::io::{Error, ErrorKind};

    fn not_configured() -> ! {
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


/// Takes out each and all supported architectures
/// - `fail!`s if an architecture is not supported
/// - all other switches has to be in `-<switch>` format
pub fn collect_arch(args: &Vec<String>, allow_empty_arch: bool) -> (Vec<Arch>, Vec<String>) {

    let util = UTIL_CONFIG.read().expect("failed to acquire lock");

    let mut archs: Vec<Arch> = Vec::new();
    let mut other: Vec<String> = Vec::new();

    for arg in args.iter() {
        if arg.is_empty() {
            fail!(user: "argument is empty");
        }

        if arg.as_bytes()[0] == b'-' {
            //  switch
            other.push(arg.clone());
        } else {
            archs.push(match Arch::from_str(arg.as_str()) {
                Ok(a) => {
                    if util.arch.contains(&a) {
                        a
                    } else {
                        fail!(user: "architecture {} is not supported", arg.as_str().blue())
                    }
                },
                Err(_) => fail!(user: "unknown architecture {}", arg.as_str().blue()),
            })
        }
    }

    if !allow_empty_arch && archs.is_empty() {
        fail!(user: "expected at least one architecture");
    }


    (archs, other)
}

#[inline]
/// Path to the arch configuration (`config/util-<arch>.toml`)
pub(super) fn arch_config_path(arch: Arch) -> PathBuf {
    PathBuf::from(format!("config/{}.toml", arch.normalize()))
}