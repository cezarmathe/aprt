use crate::package::Package;
use crate::result::AprtResult;

use std::convert::Into;
use std::path::Path;

/// An Arch Linux package repository
pub trait Repository {
    /// Add a new package to this repository
    fn add_package<P>(&self, package: P) -> AprtResult<()>
    where
        P: Into<Box<dyn Package>>;

    /// Build packages for this repository
    /// Providing an empty list builds all packages
    fn build_packages<'a, V>(&self, packages: V) -> AprtResult<()>
    where
        V: Into<Vec<&'a dyn Package>>;

    /// Create a new package repository
    fn create<'a, P: Into<&'a Path>>(path: P) -> AprtResult<Self>
    where
        Self: Sized;

    /// Load an already existing package repository
    fn load<'a, P>(path: P) -> AprtResult<Self>
    where
        P: Into<&'a Path>,
        Self: Sized;

    /// Remove a package from this repository
    fn remove_package<P>(&self, package: P) -> AprtResult<()>
    where
        P: Into<Box<dyn Package>>;

    /// Sync the sources of the package repository to a remote
    fn sync_package_sources(&self) -> AprtResult<()>;

    /// Sync the built artifacts of the repository to a remote
    fn sync_package_artifacts(&self) -> AprtResult<()>;

    /// Update packages in this package repository
    /// Providing an empty list updates all packages.
    fn update_packages<'a, V>(packages: V)
    where
        V: Into<Vec<&'a dyn Package>>;
}
