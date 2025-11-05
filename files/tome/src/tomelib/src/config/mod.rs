
//! Provides structures and functions allowing user to load/store configurations in a simple way

use crate::{current_dir_as_string, Arch, KERNEL_CONFIG, OS_CONFIG, TOME_CONFIG};
use std::{path::PathBuf, str::FromStr, fs::File, io::{Read, Write}};

use super::current_dir;

use serde::{Deserialize, Serialize};

mod triplet;
pub use triplet::*;

use paste::paste;

mod version;
pub use version::*;
mod tome;
pub use tome::*;
mod kernel;
pub use kernel::*;
mod os;
pub use os::*;
mod arch;
pub use arch::*;


/// Tells you where things are in the project
pub static PATH: Paths = Paths::new();

/// Path to the `qemu-parameters.toml` config file
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
/// Path to the tome config file
pub(super) const TOME_CONFILG_FILE: &'static str = "config/tome.toml";
/// Path to the files directory
pub(super) const FILES_PATH: &'static str = "files/";
/// Path to the limine commandline utility
pub(super) const LIMINE_CMDLINE: &'static str = "bootloader/limine-cmdline";

/// Path to the kernel
pub(super) const FORGE_KERNEL_PATH: &'static str = "files/forged/";
/// Path to the iso image
pub(super) const FORGE_ISO_PATH: &'static str = "files/forged/";


/// `ConfigHolder` standardizes implementation of config files
pub trait ConfigHolder<'l>
where Self: Sized + Serialize {

    /// Marks the default path to the config file (from PWD)
    /// - used in the `Self::load()` function
    fn default_path() -> PathBuf;

    fn default_path_for(&self) -> PathBuf;

    /// creates empty config instance
    fn empty() -> Self;

    /// Loads the configuration from the `Self::DEFAULT_PATH`
    fn load() -> Result<Self, ConfigError>
    where Self: Deserialize<'l> {
        let path = Self::default_path();
        read_toml(path)
    }

    /// Loads the configuration from custom location
    fn load_from(path: PathBuf) -> Result<Self, ConfigError>
    where Self: Deserialize<'l> {
        read_toml(path)
    }


    /// Stores the configuration to the `Self::DEFAULT_PATH`
    fn store(&self) -> Result<(), ConfigError> {
        write_toml(self, Self::default_path())
    }

    /// Stores the configuration to custom location
    fn store_to(&self, path: PathBuf) -> Result<(), ConfigError> {
        write_toml(self, path)
    }

    /// Holds the default configuration (including comments)
    fn default() -> &'static str;

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

    path_duo!(config, "bootloader/configs/limine.conf", "the limine config file");

    /// Returns name of the limine bootloader config file
    pub fn config_name(&self) -> &'static str { "limine.conf" }

    /// Returns name of the limine bootloader debug config file 
    pub fn debug_config_name(&self) -> &'static str { "limine-debug.conf" }

    path_duo!(config_debug,  "bootloader/configs/limine-debug.conf", "the limine debug config");

    path_duo!(data_dir,  LIMINE_DATA_PATH, "the limine data directory");

    path_duo!(bios_cd, "bootloader/limine-bios-cd.bin", "the limine bios CD");

    path_duo!(uefi_cd, "bootloader/limine-uefi-cd.bin", "the limine uefi CD");

    /// Returns name of the bios CD
    pub const fn bios_cd_name(&self) -> &'static str { "limine-bios-cd.bin" }

    /// Returns name of the uefi CD
    pub const fn uefi_cd_name(&self) -> &'static str { "limine-uefi-cd.bin" }

    /// Returns absolute path to the bootloader file for specific architecture
    /// - `BOOTX64.EFI` for x86_64, `BOOTAA64.EFI` for aarch64, etc.
    pub fn bootloader_for(&self, arch: Arch) -> Result<PathBuf, ()> {
        let name = arch.bootloader_name();
        let mut path = self.data_dir();
        path.push(name); Ok(path)
    }


    /// Returns absolute path to the bootloader file fo specific architecture
    /// - `BOOTX64.EFI` for x86_64, `BOOTAA64.EFI` for aarch64, etc.
    pub fn bootloader_for_string(&self, arch: Arch) -> Result<String, ()> {
        let name = arch.bootloader_name();
        Ok(format!("{}/{name}", self.data_dir_string()))
    }

}

/// Stores paths to all configurations of the util tool (mostly files stored in the `config/` directory)
pub struct ConfigPath {}

impl ConfigPath {

    path_duo!(kernel, KERNEL_CONFIG_FILE, "the kernel config file");

    path_duo!(os, OS_CONFIG_FILE, "the OS config file");

    path_duo!(qemu_parameters, QEMU_PARAMETERS, "the qemu parameters file");

    path_duo!(tome, TOME_CONFILG_FILE, "the util config file");

    /// Returns path to the arch configuration (`<arch>.toml`)
    pub fn arch(&self, arch: Arch) -> PathBuf {
        let s = format!("{}.toml", arch.normalize());
        let mut path = current_dir();
        path.push(CONFIG_PATH);
        path.push(s); path
    }

    /// Returs path to the arch configurataion (`<arch>.toml`) as string
    pub fn arch_str(&self, arch: Arch) -> String {
        format!("{}{CONFIG_PATH}/{}.toml", current_dir().to_string_lossy(), arch.normalize())
    }

}


/// Stores all paths to forged objects (forged kernel, ISO image, etc.).
pub struct ForgedPath {}

impl ForgedPath {

    /// Returns absolute path to the forged kernel for target architecture
    pub fn kernel(&self, arch: Arch) -> Result<PathBuf, ()> {
        let kernel = KERNEL_CONFIG.read().map_err(|_| ())?;
        Ok(PathBuf::from(format!("{}/{}/{}-{}.bin", current_dir().to_string_lossy(), FORGE_KERNEL_PATH, kernel.as_ref().ok_or(())?.name, arch.normalize())))
    }

    /// Returns relative path to the forged kernel for target architecture
    pub fn kernel_relative(&self, arch: Arch) -> Result<PathBuf, ()> {
        let kernel = KERNEL_CONFIG.read().map_err(|_| ())?;
        Ok(PathBuf::from(format!("{}/{}-{}.bin", FORGE_KERNEL_PATH, kernel.as_ref().ok_or(())?.name, arch.normalize())))
    }


    /// Returns absolute path to the forged kernel for target architecture as string
    pub fn kernel_string(&self, arch: Arch) -> Result<String, ()> {
        let kernel = KERNEL_CONFIG.read().map_err(|_| ())?;
        Ok(format!("{}/{}/{}-{}.bin", current_dir().to_string_lossy(), FORGE_KERNEL_PATH, kernel.as_ref().ok_or(())?.name, arch.normalize()))
    }

    /// Returns relative path to the forged kernel for target architecture as string
    pub fn kernel_string_relative(&self, arch: Arch) -> Result<String, ()> {
        let kernel = KERNEL_CONFIG.read().map_err(|_| ())?;
        Ok(format!("{}/{}-{}.bin", FORGE_KERNEL_PATH, kernel.as_ref().ok_or(())?.name, arch.normalize()))
    }

    /// Returns absolute path to the forged ISO image for target architecture
    pub fn iso(&self, arch: Arch) -> Result<PathBuf, ()> {
        let os = TOME_CONFIG.read().map_err(|_| ())?;
        Ok(PathBuf::from(format!("{}/{}/{}-{}.iso", current_dir().to_string_lossy(), FORGE_ISO_PATH, os.iso_name, arch.normalize())))
    }

    /// Returns relative path to the forged ISO image for target architecture
    pub fn iso_relative(&self, arch: Arch) -> Result<PathBuf, ()> {
        let os = TOME_CONFIG.read().map_err(|_| ())?;
        Ok(PathBuf::from(format!("{}/{}-{}.iso", FORGE_ISO_PATH, os.iso_name, arch.normalize())))
    }

    /// Returns absolute path to the forged ISO image for target architecture as string
    pub fn iso_string(&self, arch: Arch) -> Result<String, ()> {
        let os = TOME_CONFIG.read().map_err(|_| ())?;
        Ok(format!("{}/{}/{}-{}.iso", current_dir().to_string_lossy(), FORGE_ISO_PATH, os.iso_name, arch.normalize()))
    }

    /// Returns relative path to the forged ISO image for target architecture as string
    pub fn iso_string_relative(&self, arch: Arch) -> Result<String, ()> {
        let os = TOME_CONFIG.read().map_err(|_| ())?;
        Ok(format!("{}/{}-{}.iso", FORGE_ISO_PATH, os.iso_name, arch.normalize()))
    }

    /// Returns absolute path to the iso frame (structure)
    pub fn iso_frame(&self, arch: Arch) -> PathBuf {
        PathBuf::from(format!("{}/files/iso-{}/", current_dir_as_string(), arch.normalize()))
    }

    /// Returns absolute path to the iso frame (structure) as string
    pub fn iso_frame_string(&self, arch: Arch) -> String {
        format!("{}/files/iso-{}/", current_dir_as_string(), arch.normalize())
    }

    /// Returns relative path to the iso frame (structure)
    pub fn iso_frame_relative(&self, arch: Arch) -> PathBuf {
        PathBuf::from(format!("files/iso-{}/", arch.normalize()))
    }

    /// Returns relative path to the iso frame (structure) as string
    pub fn iso_frame_string_relative(&self, arch: Arch) -> String {
        format!("files/iso-{}/", arch.normalize())
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
    DuplicitKey(String),
    /// Some attributes are missing
    MissingKeys(Vec<String>),
    /// Invalid type for an attribute (holds `(name, expected type)`)
    InvalidValueType((String, String)),
    /// Indicates invalid format for an attribute (holds `(name, message)`)
    InvalidValueFormat((String, String)),
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
            Self::DuplicitKey(key) => write!(f, "found duplicit key \"{key}\""),
            Self::MissingKeys(keys) => {
                let _ = write!(f, "found duplicit attributes: ");
                for i in keys {
                    let _ = write!(f, "\"{i}\", ");
                }
                Ok(())
            },
            Self::InvalidValueType((name, expected)) => write!(f, "attributkey \"{name}\" has unsupported type, expected {expected}"),
            Self::InvalidValueFormat((name, message)) => write!(f, "invalid format for key \"{name}\": {message}"),
            Self::InvalidArch => write!(f, "unsupported architecture")
        }
    }
}

impl std::error::Error for ConfigError {}


#[inline]
/// Path to the arch configuration (`config/util-<arch>.toml`)
fn arch_config_path(arch: Arch) -> PathBuf {
    PathBuf::from(format!("config/{}.toml", arch.normalize()))
}



/// Reads the toml configuration and stores it in the structure
pub fn read_toml<'l, T>(path: PathBuf) -> Result<T, ConfigError>
where T: Sized + Deserialize<'l> {

    let mut file = File::open(path).map_err(|e| ConfigError::FailedToOpenFile(e))?;

    let mut loaded = String::with_capacity(64);

    file.read_to_string(&mut loaded).map_err(|_| ConfigError::FailedToReadFile)?;


    //  tell the borrow checker to stfu
    let ptr = (loaded.len(), loaded.capacity(), loaded.leak().as_ptr() as *mut u8);

    let r = unsafe {
        str::from_utf8_unchecked(std::slice::from_raw_parts(ptr.2, ptr.0))
    };

    let ret = toml::from_str(r).map_err(|e| ConfigError::ParserError(e.message().to_string()));

    drop(unsafe { String::from_raw_parts(ptr.2, ptr.0, ptr.1) });

    ret
}

pub fn write_toml<T>(this: &T, path: PathBuf) -> Result<(), ConfigError>
where T: Sized + Serialize {

    let mut file = File::create(path).map_err(|e| ConfigError::FailedToOpenFile(e))?;

    let serialized = toml::to_string_pretty(this).map_err(|e| ConfigError::ParserError(e.to_string()))?;

    file.write(serialized.as_bytes()).map_err(|_| ConfigError::FailedToWriteFile)?;

    Ok(())

}
