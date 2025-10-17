use toml::map::Map;
use std::{fmt::{write, Debug}, fs::File, io::{empty, Read, Write}, path::PathBuf, str::FromStr, sync::Arc};
use crate::util::{arch_config_path, KERNEL_CONFIG, KERNEL_CONFIG_FILE, OS_CONFIG, OS_CONFIG_FILE, PATH, UTIL_CONFIG, UTIL_CONFILG_FILE};

use super::{Arch, ConfigError};
use serde::{de::Error, ser, Deserialize, Serialize};


/// Structure used to represent the version in configurations
/// - In format `"X.Y.Z"`
/// - This structure intentionally does **not** implement `serde::{Deserialize, Serialize}`
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Version {
    versions: [u16; 3]
}

impl Version {
    /// Creates new instance and sets all values to `0`
    pub const fn empty() -> Self { Self { versions: [0; 3] } }

    #[inline]
    /// Returns the major version
    pub const fn major(&self) -> u16 { self.versions[0] }

    #[inline]
    /// Returns the minor version
    pub const fn minor(&self) -> u16 { self.versions[1] }

    #[inline]
    /// Returns the path version
    pub const fn patch(&self) -> u16 { self.versions[2] }

    #[inline]
    /// Sets the Major version
    pub fn set_major(&mut self, major: u16) { self.versions[0] = major }

    #[inline]
    /// Sets the Minor version
    pub fn set_minor(&mut self, minor: u16) { self.versions[1] = minor }

    #[inline]
    /// Sets the patch version
    pub fn set_patch(&mut self, patch: u16) { self.versions[2] = patch }

}

impl ToString for Version {
    /// Formaths the `Version` into string of `X.Y.Z` format
    fn to_string(&self) -> String { format!("{}.{}.{}", self.major(), self.minor(), self.patch()) }
}

impl FromStr for Version {

    type Err = ();

    /// Converts string in the `X.Y.Z` format to version
    fn from_str(ver: &str) -> Result<Self, Self::Err> {
        let mut v = ver.split('.');

        let mut next = || { v.next().ok_or(())?.parse::<u16>().map_err(|_| ()) };

        Ok(Self {
            versions: [next()?, next()?, next()?]
        })
    }
}

impl Default for Version {
    fn default() -> Self { Self { versions: [0; 3] } }
}

impl Debug for Version {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}



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

}


/// Reads the toml configuration and stores it in the structure
fn read_toml<'l, T>(path: PathBuf) -> Result<T, ConfigError>
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

fn write_toml<T>(this: &T, path: PathBuf) -> Result<(), ConfigError>
where T: Sized + Serialize {

    let mut file = File::create(path).map_err(|e| ConfigError::FailedToOpenFile(e))?;

    let serialized = toml::to_string_pretty(this).map_err(|e| ConfigError::ParserError(e.to_string()))?;

    file.write(serialized.as_bytes()).map_err(|_| ConfigError::FailedToWriteFile)?;

    Ok(())

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UtilConfig {
    pub configured: bool,
    pub xorriso: String,
    pub iso_name: String,
    pub arch: Vec<Arch>,
}

impl ConfigHolder<'_> for UtilConfig {
    
    #[inline]
    fn default_path() -> PathBuf {
        unsafe { PathBuf::from_str(UTIL_CONFILG_FILE).unwrap_unchecked() }
    }

    #[inline(always)]
    fn default_path_for(&self) -> PathBuf {
        Self::default_path()
    }

    fn empty() -> Self {
        Self {
            configured: false,
            xorriso: String::new(),
            iso_name: String::new(),
            arch: Vec::new(),
        }
    }
}

impl UtilConfig {
    /// Constructs new empty instance
    pub const fn new() -> Self {
        Self {
            configured: false,
            xorriso: String::new(),
            iso_name: String::new(),
            arch: Vec::new(),
        }
    }
}

/// Structure used to load and then process the `name`, `version`, `release` triplet
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConfigTriplet {
    pub name: String,
    pub version: String,
    pub release: String,
}

impl ConfigHolder<'_> for ConfigTriplet {

    #[inline(always)]
    fn default_path() -> PathBuf {
        panic!("ConfigTriplet has no path");
    }

    #[inline(always)]
    fn default_path_for(&self) -> PathBuf {
        Self::default_path()
    }

    fn empty() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            release: String::new(),
        }
    }

    /// **Do not use this function**, use the `Self::load_from()` function instead
    #[inline(always)]
    fn load() -> Result<Self, ConfigError> {
        panic!("ConfigTriplet::load() is unsupported, use Self::load_from() instead");
    }

    #[inline(always)]
    fn store(&self) -> Result<(), ConfigError> {
        panic!("ConfigTriplet::load() is unsupported, use Self::laoad_from() instead");
    }

}

/// Used to store the kernel config (`config/kernel.toml`) in memory
/// - this structure can be (de)serialized by using `Self::serialize()` or `Self::deserialize`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KernelConfig {
    #[serde(skip)]
    pub name: String,
    #[serde(skip)]
    pub version: Version,
    #[serde(skip)]
    pub release: String,
}

impl<'l> ConfigHolder<'l> for KernelConfig {

    #[inline]
    fn default_path() -> PathBuf { unsafe { PathBuf::from_str(KERNEL_CONFIG_FILE).unwrap_unchecked() } }

    #[inline]
    fn default_path_for(&self) -> PathBuf { Self::default_path() }

    fn empty() -> Self {
        Self {
            name: String::new(),
            version: Version::empty(),
            release: String::new(),
        }
    }

    fn load_from(path: PathBuf) -> Result<Self, ConfigError>
        where Self: Deserialize<'l> {
        Self::from_triplet(ConfigTriplet::load_from(path)?)
            .map_err(|_| ConfigError::InvalidFormat(("version".into(), "expected version in format X.Y.Z".into())))
    }

    #[inline]
    fn store_to(&self, path: PathBuf) -> Result<(), ConfigError> {
        self.to_triplet_clone().store_to(path)
    }

    #[inline]
    fn load() -> Result<Self, ConfigError>
        where Self: Deserialize<'l> {
        Self::load_from(Self::default_path())
    }

    #[inline]
    fn store(&self) -> Result<(), ConfigError> {
        self.store_to(Self::default_path())
    }

}

impl KernelConfig {

    pub const fn new() -> Self {
        Self {
            name: String::new(),
            version: Version::empty(),
            release: String::new(),
        }
    }

    fn from_triplet(value: ConfigTriplet) -> Result<Self, ()> {
        Ok(Self {
            name: value.name,
            version: Version::from_str(&value.version)?,
            release: value.release,
        })
    }

    fn to_triplet(self) -> ConfigTriplet {
        ConfigTriplet {
            name: self.name,
            version: self.version.to_string(),
            release: self.release
        }
    }

    /// Creates new 
    fn to_triplet_clone(&self) -> ConfigTriplet {
        ConfigTriplet {
            name: self.name.clone(),
            version: self.version.to_string(),
            release: self.release.clone(),
        }
    }

    /// Serializes the structure
    pub fn serialize(&self) -> Result<String, toml::ser::Error> {
        let triplet = Self::to_triplet_clone(&self);
        toml::to_string_pretty(&triplet)
    }

    /// Deserializes the struct
    pub fn deserialize(from: &str) -> Result<Self, toml::de::Error> {
        Self::from_triplet(toml::from_str::<ConfigTriplet>(from)?).map_err(|_| toml::de::Error::custom("unknown error"))
    }

}



/// Used to store the Os config in memory (`config/os.toml`)
/// - this structure can be (de)serialized by using `Self::serialize()` or `Self::deserialize`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsConfig {
    #[serde(skip)]
    pub name: String,
    #[serde(skip)]
    pub version: Version,
    #[serde(skip)]
    pub release: String,
}

impl<'l> ConfigHolder<'l> for OsConfig {

    #[inline]
    fn default_path() -> PathBuf { unsafe { PathBuf::from_str(OS_CONFIG_FILE).unwrap_unchecked() } }

    #[inline]
    fn default_path_for(&self) -> PathBuf { Self::default_path() }

    fn empty() -> Self {
        Self {
            name: String::new(),
            version: Version::empty(),
            release: String::new(),
        }
    }

    fn load_from(path: PathBuf) -> Result<Self, ConfigError>
        where Self: Deserialize<'l> {
        Self::from_triplet(ConfigTriplet::load_from(path)?)
            .map_err(|_| ConfigError::InvalidFormat(("version".into(), "expected version in format X.Y.Z".into())))
    }

    #[inline]
    fn store_to(&self, path: PathBuf) -> Result<(), ConfigError> {
        self.to_triplet_clone().store_to(path)
    }

    #[inline]
    fn load() -> Result<Self, ConfigError>
        where Self: Deserialize<'l> {
        Self::load_from(Self::default_path())
    }

    #[inline]
    fn store(&self) -> Result<(), ConfigError> {
        self.store_to(Self::default_path())
    }


}

impl OsConfig {


    pub const fn new() -> Self {
        Self {
            name: String::new(),
            version: Version::empty(),
            release: String::new(),
        }
    }

    fn from_triplet(value: ConfigTriplet) -> Result<Self, ()> {
        Ok(Self {
            name: value.name,
            version: Version::from_str(&value.version)?,
            release: value.release,
        })
    }

    fn to_triplet(self) -> ConfigTriplet {
        ConfigTriplet {
            name: self.name,
            version: self.version.to_string(),
            release: self.release
        }
    }

    /// Creates new 
    fn to_triplet_clone(&self) -> ConfigTriplet {
        ConfigTriplet {
            name: self.name.clone(),
            version: self.version.to_string(),
            release: self.release.clone(),
        }
    }

    /// Serializes the structure
    pub fn serialize(&self) -> Result<String, toml::ser::Error> {
        let triplet = Self::to_triplet_clone(&self);
        toml::to_string_pretty(&triplet)
    }

    /// Deserializes the struct
    pub fn deserialize(from: &str) -> Result<Self, toml::de::Error> {
        Self::from_triplet(toml::from_str::<ConfigTriplet>(from)?).map_err(|_| toml::de::Error::custom("unknown error"))
    }

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArchConfig {
    #[serde(skip)]
    arch: Arch,
    pub compiler: String,
    pub emulator: String,
}

impl ArchConfig {

    /// Returns target architecture 
    pub const fn arch(&self) -> Arch {
        self.arch
    }

    pub const fn from_parts(arch: Arch, compiler: String, emulator: String) -> Self {
        Self { arch, compiler, emulator }
    }

    /// Creates new empty configuration
    pub const fn empty(arch: Arch) -> Self {
        Self {
            arch,
            compiler: String::new(),
            emulator: String::new(),
        }
    }

    /// Loads the configuration
    pub fn load(arch: Arch) -> Result<Self, ConfigError> {

        if let Arch::None = arch {
            return Err(ConfigError::InvalidArch)
        }

        let path = arch_config_path(arch);

        let mut this: Self = read_toml(path)?;
        this.arch = arch;
        Ok(this)
    }

    /// Loads the configuration from custom path
    pub fn load_from(arch: Arch, path: PathBuf) -> Result<Self, ConfigError> {

        if let Arch::None = arch {
            return Err(ConfigError::InvalidArch)
        }

        let mut this : Self = read_toml(path)?;
        this.arch = arch;
        Ok(this)
    }

    /// Stores the configuration
    pub fn store(&self) -> Result<(), ConfigError> {

        if let Arch::None = self.arch {
            return Err(ConfigError::InvalidArch)
        }

        let path = arch_config_path(self.arch);

        write_toml(self, path)
    }

    /// Stores the configuration to custom location
    pub fn store_to(&self, path: PathBuf) -> Result<(), ConfigError> {

        if let Arch::None = self.arch {
            return Err(ConfigError::InvalidArch)
        }

        write_toml(self, path)

    }


}