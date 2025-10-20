use std::str::FromStr;
use std::fmt::Debug;



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
    /// Returns the patch version
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

    /// Returns reference to the inner array
    pub const fn as_array(&self) -> &[u16; 3] { &self.versions }

    /// Returns mutable reference to the inner array
    pub const fn as_array_mut(&mut self) -> &mut [u16; 3] { &mut self.versions }

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
    fn default() -> Self { Self::empty() }
}

impl Debug for Version {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}