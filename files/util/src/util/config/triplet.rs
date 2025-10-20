

use std::{fmt::{write, Debug}, fs::File, io::{empty, Read, Write}, path::PathBuf, str::FromStr, sync::Arc};
use crate::util::{KERNEL_CONFIG, KERNEL_CONFIG_FILE, OS_CONFIG, OS_CONFIG_FILE, PATH, UTIL_CONFIG, UTIL_CONFILG_FILE};

use super::{Arch, ConfigError, Version};
use serde::{de::Error, Deserialize, Serialize};

use super::ConfigHolder;


/// Structure used to load and then process the `name`, `version`, `release` triplet
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super)struct ConfigTriplet {
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
