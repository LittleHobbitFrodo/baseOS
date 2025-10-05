use crate::{util::Arch};
use std::{path::PathBuf, str::FromStr};

mod holders;
pub use holders::*;


/// tells you where are things in the project
pub static PATH: Paths = Paths::new();


pub(super) const QEMU_PARAMETERS: &'static str = "config/qemu-parameters.toml";
/// Path to config directory
pub(super) const CONFIG_PATH: &'static str = "config/";
/// Path to the kernel config file
pub(super) const KERNEL_CONFIG: &'static str = "config/kernel.toml";
/// Path to the OS config file
pub(super) const OS_CONFIG: &'static str = "config/os.toml";
/// Path to the limine config directory
pub(super) const LIMINE_CONFIG_PATH: &'static str = "bootloader/configs/";
/// Path to limine data
pub(super) const LIMINE_DATA_PATH: &'static str = "bootloader/";
/// Path to the utility config file
pub(super) const UTIL_CONFIG: &'static str = "config/util.toml";
/// Path to the files directory
pub(super) const FILES_PATH: &'static str = "files/";
/// Path to the limine commandline utility
pub(super) const LIMINE_CMDLINE: &'static str = "bootloader/limine-cmdline";

#[inline]
/// Path to the arch configuration (`config/util-<arch>.toml`)
pub(super) fn arch_config_path(arch: Arch) -> PathBuf {
    PathBuf::from(format!("config/{}.toml", arch.normalize()))
}

/// Used to describe which config file do you want to access
pub enum ConfigFile {
    /// The kernel config file (`kernel.toml`)
    Kernel,
    /// The OS config file (`os.toml`)
    Os,
    /// The util config file (`util.toml`)
    Util,
    /// Target specific config file (`<arch>.toml`)
    Target(Arch),
    /// The `qemu-parameters.toml` config file
    Qemu,
}

pub enum LiminePath {
    /// Path to the config directory
    Config,
    /// Path to the data directory
    Data,
    /// Path to the limine commandline utility
    CmdLine,
}

/// List of all needed directories and files
pub struct Paths {}

impl Paths {
    pub const fn new() -> Self { Self {} }

    /// Returns path to the given config fil
    pub fn config_path(&self, file: ConfigFile) -> Result<PathBuf, ()> {

        Ok(match file {
            ConfigFile::Kernel => PathBuf::from_str(KERNEL_CONFIG),
            ConfigFile::Os => PathBuf::from_str(OS_CONFIG),
            ConfigFile::Qemu => PathBuf::from_str(QEMU_PARAMETERS),
            ConfigFile::Util => PathBuf::from_str(UTIL_CONFIG),
            ConfigFile::Target(a) => PathBuf::from_str(match a {
                Arch::None => return Err(()),
                _ => a.as_str()
            }),
        }.map_err(|_| ())?)

    }

    /// Returns path to the given config file as string
    pub fn config_path_string(&self, file: ConfigFile) -> Result<String, ()> {

        Ok(match file {
            ConfigFile::Kernel => KERNEL_CONFIG.into(),
            ConfigFile::Os => OS_CONFIG.into(),
            ConfigFile::Qemu => QEMU_PARAMETERS.into(),
            ConfigFile::Util => UTIL_CONFIG.into(),
            ConfigFile::Target(a) => match a {
                Arch::None => return Err(()),
                _ => a.as_str(),
            }.into(),
        })

    }

    /// Returns path to the given limine attribute
    pub fn limine_path(&self, path: LiminePath) -> Result<PathBuf, ()> {

        Ok(match path {
            LiminePath::CmdLine => PathBuf::from_str(LIMINE_CMDLINE),
            LiminePath::Config => PathBuf::from_str(LIMINE_CONFIG_PATH),
            LiminePath::Data => PathBuf::from_str(LIMINE_DATA_PATH)
        }.map_err(|_| ())?)

    }

    /// Returns path to the given limine attribute as string
    pub fn limine_path_string(&self, path: LiminePath) -> Result<String, ()> {

        Ok(match path {
            LiminePath::CmdLine => LIMINE_CMDLINE.into(),
            LiminePath::Config => LIMINE_CONFIG_PATH.into(),
            LiminePath::Data => LIMINE_DATA_PATH.into(),
        })

    }

}


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
    /// Indicates invalid format for an attribute (holds `(attr_name, message)`)
    InvalidFormat((String, String)),
    /// Indicates error in architecture specification
    InvalidArch
}