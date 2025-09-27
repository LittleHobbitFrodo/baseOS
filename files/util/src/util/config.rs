use crate::{util::Arch};
use std::{fs::File, io::Read, path::PathBuf, str::FromStr};


pub static PATH: Config = Config::new();

/// Path to config directory
pub(super) const CONFIG_PATH: &'static str = "config/";
/// Path to the kernel config file
pub(super) const KERNEL_CONFIG: &'static str = "config/kernel.conf";
/// Path to the OS config file
pub(super) const OS_CONFIG: &'static str = "config/os.conf";
/// Path to the limine config directory
pub(super) const LIMINE_CONFIG_PATH: &'static str = "bootloader/configs/";
/// Path to limine data
pub(super) const LIMINE_DATA_PATH: &'static str = "bootloader/";
/// Path to the utility config file
pub(super) const UTIL_CONFIG: &'static str = "config/util.conf";
/// Path to the files directory
pub(super) const FILES_PATH: &'static str = "files/";
/// Path to the limine commandline utility
pub(super) const LIMINE_CMDLINE: &'static str = "bootloader/line-cmdline";

/// Tries to retrieve the current working directory
fn try_current() -> Result<PathBuf, ()> {
    std::env::current_dir().map_err(|_| ())
}


/// List of all needed directories and files
pub struct Config {}

impl Config {
    pub const fn new() -> Self {
        Self {}
    }


    /// Returns relative path to the config directory
    pub fn config_path(&self) -> PathBuf {
        PathBuf::from_str(CONFIG_PATH).expect("failed to parse path")
    }

    /// Returns absolute path to the config directory
    pub fn config_path_full(&self) -> PathBuf {
        let mut path = super::current_dir();
        path.push(CONFIG_PATH); path
    }



    /// Returns relative path to the kernel config file
    pub fn kernel_config(&self) -> PathBuf {
        PathBuf::from_str(KERNEL_CONFIG).expect("failed to parse path")
    }

    /// Returns absolute path to the kernel config file
    pub fn kernel_config_full(&self) -> PathBuf {
        let mut path = super::current_dir();
        path.push(KERNEL_CONFIG); path
    }

    /// Returns relative path to the OS config file
    pub fn os_config(&self) -> PathBuf {
        PathBuf::from_str(OS_CONFIG).expect("failed to parse path")
    }

    /// Returns absolute path to the OS config file
    pub fn os_config_full(&self) -> PathBuf {
        let mut path = super::current_dir();
        path.push(OS_CONFIG); path
    }


    /// Returns relative path to the utility config file
    pub fn util_config(&self) -> PathBuf {
        PathBuf::from_str(UTIL_CONFIG).expect("failed to parse path")
    }

    /// Returns absolute path to the utility config file
    pub fn util_config_full(&self) -> PathBuf {
        let mut path = super::current_dir();
        path.push(UTIL_CONFIG); path
    }

    /// Returns relative path to the limine config directory
    pub fn limine_config_path(&self) -> PathBuf {
        PathBuf::from_str(LIMINE_CONFIG_PATH).expect("failed to parse path")
    }

    /// Returns absolute path to the limine config directory
    pub fn limine_config_path_full(&self) -> PathBuf {
        let mut path = super::current_dir();
        path.push(LIMINE_CONFIG_PATH); path
    }


    /// Returns relative path to limine data directory
    pub fn limine_data_path(&self) -> PathBuf {
        PathBuf::from_str(LIMINE_DATA_PATH).expect("failed to parse path")
    }

    /// returns absolute path to limine data directory
    pub fn limine_data_path_full(&self) -> PathBuf {
        let mut path = super::current_dir();
        path.push(LIMINE_DATA_PATH); path
    }


    /// Returns relative path to files directory
    pub fn files_path(&self) -> PathBuf {
        PathBuf::from_str(FILES_PATH).expect("failed to parse path")
    }

    /// Returns absolute path to files directory
    pub fn files_path_full(&self) -> PathBuf {
        let mut path = super::current_dir();
        path.push(FILES_PATH); path
    }

    
    /// Returns relative path to the limine utility
    pub fn limine_cmdline(&self) -> PathBuf {
        PathBuf::from_str(LIMINE_CMDLINE).expect("failed to parse path")
    }

    /// Returns absolute path to the limine utility
    pub fn limine_cmdline_full(&self) -> PathBuf {
        let mut path = super::current_dir();
        path.push(LIMINE_CMDLINE); path
    }



    //  target specific

    /// Returns relative path to the util config file for target arch
    pub fn util_config_for(&self, arch: Arch) -> PathBuf {
        let mut path = PathBuf::from_str(CONFIG_PATH).expect("failed to parse path");
        path.push(format!("util-{}.conf", arch.as_str())); path
    }

    /// Returns absolute path to the util config file for target arch
    pub fn util_config_full_for(&self, arch: Arch) -> PathBuf {
        let mut path = super::current_dir();
        path.push(CONFIG_PATH); path.push(format!("config-{}.conf", arch.as_str()));
        path
    }

}


#[derive(Copy, Clone, Debug)]
/// This struct is used to store version information
pub struct Version {
    v: [u16; 3],
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

impl Version {

    #[inline(always)]
    /// Returns the major version number
    pub const fn major(&self) -> u16 { self.v[0] }

    #[inline(always)]
    /// Returns the minor version number
    pub const fn minor(&self) -> u16 { self.v[1] }

    #[inline(always)]
    /// Returns the patch version number
    pub const fn patch(&self) -> u16 { self.v[2] }



    #[inline]
    /// Sets the major number
    pub fn set_major(&mut self, major: u16) { self.v[0] = major }

    #[inline]
    /// Sets the minor version
    pub fn set_minor(&mut self, minor: u16) { self.v[1] = minor }

    #[inline]
    /// Sets the patch version
    pub fn set_patch(&mut self, patch: u16) { self.v[2] = patch }
    

    /// Creates empty instance
    pub const fn empty() -> Self {
        Self { v: [0u16; 3] }
    }

    /// Formats the version into a string
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major(), self.minor(), self.patch())
    }

    #[inline]
    /// Returns the inner value
    fn as_arr(&self) -> &[u16; 3] { &self.v }

    #[inline]
    /// Returns the inner value as mutable
    fn as_mut_arr(&mut self) -> &mut [u16; 3] { &mut self.v }

}

#[derive(Debug)]
pub enum ConfigLoadError {
    /// Indicates that config file cannot be opened
    FailedToOpenFile(std::io::Error),
    /// Reading the file failed
    FailedToReadFile,
    /// Config has invalid format (indicates whats wrong)
    InvalidFormat(String),
    /// There are duplicit attributes in the config (holds attribute name)
    ConflictingAttribue(String),
    /// Found attribute that is not known (stores its name)
    UnknownAttribute(String),
    /// Invalid value for attribute (holds (attribute, value))
    InvalidValue((String, String)),
}

/// `KernelConfig` is used to load, store and modify kernel configuration
#[derive(Clone, Debug)]
pub struct KernelConfig {
    pub name: String,
    pub version: Version,
    pub release: String,
}

impl KernelConfig {

    /// Creates new empty instance
    pub const fn empty() -> Self {
        Self {
            name: String::new(),
            version: Version::empty(),
            release: String::new(),
        }
    }


    pub fn load() -> Result<Self, ConfigLoadError> {

        //  open file
        let mut file = match File::open(PATH.kernel_config_full()) {
            Ok(f) => f,
            Err(e) => return Err(ConfigLoadError::FailedToOpenFile(e)),
        };

        let mut cfg = Self::empty();

        //  read content
        let mut content = String::with_capacity(64);
        if let Err(_) = file.read_to_string(&mut content) {
            return Err(ConfigLoadError::FailedToReadFile);
        }

        //  iterate through lines
        for line in content.lines() {
            //  split attribute and value
            let split = match line.find('=') {
                Some(pos) => pos,
                None => return Err(ConfigLoadError::InvalidFormat(String::from("expected this format: attribute=value")))
            };

            let attribute = &line[..split];
            let value = &line[split+1..];

            match attribute {
                "name" => {
                    if !cfg.name.is_empty() {
                        return Err(ConfigLoadError::ConflictingAttribue(String::from("name")))
                    }
                    cfg.name = String::from(value)
                },
                "release" => cfg.release = String::from(value),
                "version" => {

                    let mut versions = value.split('.');

                    let arr = cfg.version.as_mut_arr();

                    //  iterate through the major, minor and patch versions
                    for i in arr {
                        *i = match versions.next() {
                            Some(v) => v.parse().map_err(|_| {
                                ConfigLoadError::InvalidValue((attribute.into(), "version number".into()))
                            })?,
                            None => return Err(ConfigLoadError::InvalidValue((attribute.into(), "version number".into()))),
                        }
                    }
                },
                _ => return Err(ConfigLoadError::UnknownAttribute(String::from(attribute)))
            }
        }

        Ok(cfg)

    }

}
