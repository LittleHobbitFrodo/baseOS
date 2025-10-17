use crate::util::{Arch, KERNEL_CONFIG, OS_CONFIG, UTIL_CONFIG};
use std::{path::{self, Path, PathBuf}, str::FromStr};

mod holders;
pub use holders::*;

use paste::paste;


/// Tells you where are things in the project
pub static PATH: Paths = Paths::new();


pub(super) const QEMU_PARAMETERS: &'static str = "config/qemu-parameters.toml";
/// Path to config directory
pub(super) const CONFIG_PATH: &'static str = "config/";
/// Path to the kernel config file
pub(super) const KERNEL_CONFIG_FILE: &'static str = "config/kernel.toml";
/// Path to the OS config file
pub(super) const OS_CONFIG_FILE: &'static str = "config/os.toml";
/// Path to the limine config directory
pub(super) const LIMINE_CONFIG_PATH: &'static str = "bootloader/configs/";
/// Path to limine data
pub(super) const LIMINE_DATA_PATH: &'static str = "bootloader/";
/// Path to the utility config file
pub(super) const UTIL_CONFILG_FILE: &'static str = "config/util.toml";
/// Path to the files directory
pub(super) const FILES_PATH: &'static str = "files/";
/// Path to the limine commandline utility
pub(super) const LIMINE_CMDLINE: &'static str = "bootloader/limine-cmdline";

/// Path to the kernel
pub(super) const FORGE_KERNEL_PATH: &'static str = "files/forged/";
/// Path to the iso image
pub(super) const FORGE_ISO_PATH: &'static str = "files/forged/";


/// Shortens and unify the process of getting the current working directory
#[inline]
fn current_dir() -> PathBuf {
    std::env::current_dir().expect("failed to get current dir")
}

/// generates boilerplate code for path functions:
/// ```rust
/// path_duo!(cmdline, cmdline_string, "file", "<path destination>")
/// ```
/// generates this code:
/// ```rust
/// /// Returns absolute path to <path destination>
/// pub fn cmdline(&self) -> PathBuf { ... }
/// /// Returns absolute path to <path destination> as string
/// pub fn cmdline_string(&self) -> String { ... }
/// ```
macro_rules! path_duo {
    ($name:tt, $path:expr, $doc:expr) => {

        paste! {
            #[doc = "Returns absolute path to "]
            #[doc = $doc]
            pub fn $name(&self) -> PathBuf {
                let mut path = current_dir();
                path.push($path); path
            }

            #[doc = "Returns relative path to "]
            #[doc = $doc]
            pub fn [<$name _relative>](&self) -> PathBuf {
                unsafe { PathBuf::from_str($path).unwrap_unchecked() }
            }


            #[doc = "Returns absolute path to "]
            #[doc = $doc]
            #[doc = " as string"]
            pub fn [<$name _string>](&self) -> String {
                format!("{}/{}", current_dir().to_string_lossy(), $path)
            }

            #[doc = "Returns relative path to "]
            #[doc = $doc]
            #[doc = " as string"]
            pub fn [<$name _string_relative>](&self) -> String {
                $path.into()
            }

        }
    };
}


/// Stores paths to files and directories used for working with the limine bootloader
pub struct LiminePaths {}

impl LiminePaths {

    pub const fn new() -> Self { Self {} }

    path_duo!(cmdline, LIMINE_CMDLINE, "the limine commandline utility");

    path_duo!(config_dir,  LIMINE_CONFIG_PATH, "limine config directory");

    path_duo!(config, "limine.conf", "the limine config file");

    path_duo!(config_debug,  "limine-debug.conf", "the limine debug config");

    path_duo!(data_dir,  LIMINE_DATA_PATH, "the limine data directory");

    /// Returns absolute path to the bootloader file for specific architecture
    /// - `BOOTX64.EFI` for x86_64, `BOOTAA64.EFI` for aarch64, etc.
    pub fn bootloader_for(&self, arch: Arch) -> Result<PathBuf, ()> {
        let name = arch.bootloader_name()?;
        let mut path = self.data_dir();
        path.push(name); Ok(path)
    }


    /// Returns absolute path to the bootloader file fo specific architecture
    /// - `BOOTX64.EFI` for x86_64, `BOOTAA64.EFI` for aarch64, etc.
    pub fn bootloader_for_string(&self, arch: Arch) -> Result<String, ()> {
        let name = arch.bootloader_name()?;
        Ok(format!("{}/{name}", self.data_dir_string()))
    }

}

/// Stores paths to all configurations of the util tool (mostly files stored in the `config/` directory)
pub struct ConfigPath {}

impl ConfigPath {

    path_duo!(kernel, KERNEL_CONFIG_FILE, "the kernel config file");

    path_duo!(os, OS_CONFIG_FILE, "the OS config file");

    path_duo!(qemu_parameters, QEMU_PARAMETERS, "the qemu parameters file");

    path_duo!(util, UTIL_CONFILG_FILE, "the util config file");

    /// Returns path to the arch configuration (`<arch>.toml`)
    pub fn arch(&self, arch: Arch) -> Result<PathBuf, ()> {
        let s = match arch {
            Arch::None => return Err(()),
            _ => format!("{}.toml", arch.normalize()),
        };
        let mut path = current_dir();
        path.push(CONFIG_PATH);
        path.push(s); Ok(path)
    }

    /// Returs path to the arch configurataion (`<arch>.toml`) as string
    pub fn arch_str(&self, arch: Arch) -> Result<String, ()> {
        match arch {
            Arch::None => Err(()),
            _ => Ok(format!("{}{CONFIG_PATH}/{}.toml", current_dir().to_string_lossy(), arch.normalize()))
        }
    }

}


/// Stores all paths to forged objects (forged kernel, ISO image, etc.).
pub struct ForgedPath {}

impl ForgedPath {

    /// Returns absolute path to the forged kernel for target architecture
    pub fn kernel(&self, arch: Arch) -> Result<PathBuf, ()> {
        match arch {
            Arch::None => return Err(()),
            _ => {
                let kernel = KERNEL_CONFIG.read().expect("failed to acquire lock");
                Ok(PathBuf::from(format!("{}/{}/{}-{}.bin", current_dir().to_string_lossy(), FORGE_KERNEL_PATH, kernel.as_ref().ok_or(())?.name, arch.normalize())))
            }
        }
    }

    /// Returns relative path to the forged kernel for target architecture
    pub fn kernel_relative(&self, arch: Arch) -> Result<PathBuf, ()> {
        match arch {
            Arch::None => return Err(()),
            _ => {
                let kernel = KERNEL_CONFIG.read().expect("failed to acquire lock");
                Ok(PathBuf::from(format!("{}/{}-{}.bin", FORGE_KERNEL_PATH, kernel.as_ref().ok_or(())?.name, arch.normalize())))
            }
        }
    }


    /// Returns absolute path to the forged kernel for target architecture as string
    pub fn kernel_string(&self, arch: Arch) -> Result<String, ()> {
        match arch {
            Arch::None => return Err(()),
            _ => {
                let kernel = KERNEL_CONFIG.read().expect("failed to acquire lock");
                Ok(format!("{}/{}/{}-{}.bin", current_dir().to_string_lossy(), FORGE_KERNEL_PATH, kernel.as_ref().ok_or(())?.name, arch.normalize()))
            }
        }
    }

    /// Returns relative path to the forged kernel for target architecture as string
    pub fn kernel_string_relative(&self, arch: Arch) -> Result<String, ()> {
        match arch {
            Arch::None => return Err(()),
            _ => {
                let kernel = KERNEL_CONFIG.read().expect("failed to acquire lock");
                Ok(format!("{}/{}-{}.bin", FORGE_KERNEL_PATH, kernel.as_ref().ok_or(())?.name, arch.normalize()))
            }
        }
    }

    /// Returns absolute path to the forged ISO image for target architecture
    pub fn iso(&self, arch: Arch) -> Result<PathBuf, ()> {
        match arch {
            Arch::None => Err(()),
            _ => {
                let os = UTIL_CONFIG.read().expect("failed to acquire lock");
                
                Ok(PathBuf::from(format!("{}/{}/{}-{}.iso", current_dir().to_string_lossy(), FORGE_ISO_PATH, os.iso_name, arch.normalize())))
            },
        }
    }

    /// Returns relative path to the forged ISO image for target architecture
    pub fn iso_relative(&self, arch: Arch) -> Result<PathBuf, ()> {
        match arch {
            Arch::None => Err(()),
            _ => {
                let os = UTIL_CONFIG.read().expect("failed to acquire lock");
                
                Ok(PathBuf::from(format!("{}/{}-{}.iso", FORGE_ISO_PATH, os.iso_name, arch.normalize())))
            },
        }
    }

    /// Returns absolute path to the forged ISO image for target architecture as string
    pub fn iso_string(&self, arch: Arch) -> Result<String, ()> {
        match arch {
            Arch::None => Err(()),
            _ => {
                let os = UTIL_CONFIG.read().expect("failed to acquire lock");
                Ok(format!("{}/{}/{}-{}.iso", current_dir().to_string_lossy(), FORGE_ISO_PATH, os.iso_name, arch.normalize()))
            }
        }
    }

    /// Returns relative path to the forged ISO image for target architecture as string
    pub fn iso_string_relative(&self, arch: Arch) -> Result<String, ()> {
        match arch {
            Arch::None => Err(()),
            _ => {
                let os = UTIL_CONFIG.read().expect("failed to acquire lock");
                Ok(format!("{}/{}-{}.iso", FORGE_ISO_PATH, os.iso_name, arch.normalize()))
            }
        }
    }


}


/// List of all needed directories and files
pub struct Paths {
    pub limine: LiminePaths,
    pub config: ConfigPath,
    pub forged: ForgedPath
}

impl Paths {

    const fn new() -> Self { Self { limine: LiminePaths {}, config: ConfigPath {}, forged: ForgedPath {} } }

}

/// Describes most of the possible errors that can occur while loading/storing util configurations
#[derive(Debug)]
pub enum ConfigError {
    FailedToOpenFile(std::io::Error),
    FailedToReadFile,
    FailedToWriteFile,
    /// The `toml` crate failed to parse the file
    ParserError(String),

    /// Found one attribute multiple times (holds name of the attribute)
    DuplicitAttribute(String),
    /// Some attributes are missing
    MissingAttributes(Vec<String>),
    /// Invalid type for an attribute (holds `(name, expected type)`)
    InvalidType((String, String)),
    /// Indicates invalid format for an attribute (holds `(name, message)`)
    InvalidFormat((String, String)),
    /// Indicates error in architecture specification (`None` or unsupported)
    InvalidArch
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToOpenFile(e) => write!(f, "failed to open file: {e}"),
            Self::FailedToReadFile => write!(f, "failed to read file: unknown error"),
            Self::FailedToWriteFile => write!(f, "failed to write into file: unknown error"),
            Self::ParserError(e) => write!(f, "failed to parse config: {e}"),
            Self::DuplicitAttribute(attr) => write!(f, "found duplicit attribute \"{attr}\""),
            Self::MissingAttributes(attrs) => {
                let _ = write!(f, "found duplicit attributes: ");
                for i in attrs {
                    let _ = write!(f, "\"{i}\", ");
                }
                Ok(())
            },
            Self::InvalidType((name, expected)) => write!(f, "attribute \"{name}\" has unsupported type, expected {expected}"),
            Self::InvalidFormat((name, message)) => write!(f, "invalid format for attribute \"{name}\": {message}"),
            Self::InvalidArch => write!(f, "unsupported architecture")
        }
    }
}

impl std::error::Error for ConfigError {}