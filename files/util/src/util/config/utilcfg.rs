
//! This module indroduces the `UtilConfig` strucure allowing you to simply load and store the util config

use serde::{Serialize, Deserialize};

use std::str::FromStr;

use std::fmt::Debug;
use std::path::PathBuf;

use super::Arch;
use super::ConfigHolder;
use super::UTIL_CONFILG_FILE;



/// Stores the util configuration
/// - The config file is loaded  by `ctor` constructor before the `main` function is called into the `UTIL_CONFIG` static variable
/// - All the fields are directly accessible
/// - use `store()` to save the configuration and `load` to reload it
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