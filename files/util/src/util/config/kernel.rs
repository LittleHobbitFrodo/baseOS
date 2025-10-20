

use serde::{Serialize, Deserialize};
use std::{path::PathBuf, str::FromStr};
use super::{KERNEL_CONFIG_FILE, ConfigError, ConfigHolder, Version, ConfigTriplet};

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
            .map_err(|_| ConfigError::InvalidValueFormat(("version".into(), "expected version in format X.Y.Z".into())))
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
    pub fn deserialize(from: &str) -> Result<Self, ()> {
        Self::from_triplet(toml::from_str::<ConfigTriplet>(from).map_err(|_| ())?).map_err(|_| ())
    }

}