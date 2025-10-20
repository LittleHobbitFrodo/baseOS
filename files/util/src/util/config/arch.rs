
use serde::{Deserialize, Serialize};
use super::{Arch, ConfigError, read_toml, write_toml, arch_config_path};
use std::{path::PathBuf};


/// Stores architecture specific config
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

        let path = arch_config_path(arch);

        let mut this: Self = read_toml(path)?;
        this.arch = arch;
        Ok(this)
    }

    /// Loads the configuration from custom path
    pub fn load_from(arch: Arch, path: PathBuf) -> Result<Self, ConfigError> {

        let mut this : Self = read_toml(path)?;
        this.arch = arch;
        Ok(this)
    }

    /// Stores the configuration
    pub fn store(&self) -> Result<(), ConfigError> {

        let path = arch_config_path(self.arch);

        write_toml(self, path)
    }

    /// Stores the configuration to custom location
    pub fn store_to(&self, path: PathBuf) -> Result<(), ConfigError> {

        write_toml(self, path)

    }


}