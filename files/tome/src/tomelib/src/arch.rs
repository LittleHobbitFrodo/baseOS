
//! Provides the `Arch` enum which is used to identify 

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use super::{fail, TOME_CONFIG};


/// Identifies all architectures supported by the `util` and `ministd`
/// 
/// Architecture is deemed **supported by util** when all built-in `util` subcommands are supporting it and is also supported by ministd.
/// - **supported by OS** means that the architecture is supported by the util, ministd and is listed in the util config file
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[repr(u32)]
pub enum Arch {
    X86_64,
    Arm64
}

impl Default for Arch {
    /// Returns `Arch::X86_64`
    fn default() -> Self { Arch::X86_64 }
}


impl FromStr for Arch {
    type Err = ();
    /// lowercase only
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
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

    #[inline]
    /// Case unsensitive `Arch::from_str`
    pub fn from_str_insensitive(s: &str) -> Result<Self, ()> {
        Self::from_str(s.to_ascii_lowercase().as_str())
    }

    /// Returns the bootloader file name for specific architecture
    /// - `BOOTX64.EFI` for x86_64, `BOOTAA64.EFI` for aarch64, etc.
    pub fn bootloader_name(&self) -> &'static str {
        match self {
            Self::Arm64 => "BOOTAA64.EFI",
            Self::X86_64 => "BOOTX64.EFI",
        }
    }

    /// Returns the triplet (`<arch>-unknown-none`) for this architecture
    pub fn triplet(&self) -> &'static str {
        match self {
            Self::X86_64 => "x86_64-unknown-none",
            Self::Arm64 => "aarch64-unknown-none"
        }
    }

    /// returns normalized name for target architecture
    /// - `x86_64` for x86_64, `aarch64` for arm64, etc.
    pub fn normalize(&self) -> &'static str {
        match self {
            Self::Arm64 => "aarch64",
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
        &["x86_64", "aarch64"]
    }

    /// Converts the `Arch` enum into `str`
    /// - `arm64` for arm64, `x86_64` for x86_64
    /// - use the `normalize()` function to get the normalized names
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

    /// Sepataters all architectures and switches in given commandline arguments
    /// - returns `(archs, other)`
    /// - `fail`s if given architecture is not supported
    /// - All switches has to start with `-`: `-<switch>` to be recognized as switches
    /// - Relies in the `ctor` constructor
    /// - if `allow_empty_arch` is `false`, function will panic if the `archs` vector is empty
    pub fn collect(args: &Vec<String>, allow_empty_arch: bool) -> (Vec<Arch>, Vec<String>) {

        let util = TOME_CONFIG.read().expect("failed to acquire lock");

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
                            if !archs.contains(&a) {
                                a
                            } else {
                                continue;
                            }
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

    /// Septarates all architectures, switches and actions
    /// - action is considered string that does not start with '-' and does not match any arch
    /// - returns `(archs, switches, actions)`
    pub fn collect_with_actions(args: &Vec<String>, allow_empty_arch: bool) -> (Vec<Arch>, Vec<String>, Vec<String>) {

        let util = TOME_CONFIG.read().expect("failed to acquire lock");

        let mut archs: Vec<Arch> = Vec::new();
        let mut switches: Vec<String> = Vec::new();
        let mut actions: Vec<String> = Vec::new();

        for arg in args.iter() {
            if arg.is_empty() {
                fail!(user: "argument is empty");
            }

            if arg.as_bytes()[0] == b'-' {
                //  switch
                switches.push(arg.clone());
            } else {
                if let Ok(a) = Arch::from_str(arg.as_str()) {
                    //  arch
                    if util.arch.contains(&a) {
                            if !archs.contains(&a) {
                                archs.push(a);
                            } else {
                                continue;
                            }
                        } else {
                            fail!(user: "architecture {} is not supported", arg.as_str().blue())
                        }
                } else {
                    //  action
                    actions.push(arg.clone())
                }
            }

        }

        if !allow_empty_arch && archs.is_empty() {
            fail!(user: "expected at least one architecture");
        }

        (archs, switches, actions)

    }

}

impl std::fmt::Display for Arch {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}