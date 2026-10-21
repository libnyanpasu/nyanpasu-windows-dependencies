//! MSIX packages registered for the current user.

use crate::{
    Version,
    bindings::{Package, PackageManager, ProcessorArchitecture},
};

/// The architecture of packages this process can load.
#[cfg(target_arch = "x86_64")]
pub(crate) const PROCESS_ARCHITECTURE: ProcessorArchitecture = ProcessorArchitecture::X64;
#[cfg(target_arch = "aarch64")]
pub(crate) const PROCESS_ARCHITECTURE: ProcessorArchitecture = ProcessorArchitecture::Arm64;
#[cfg(target_arch = "x86")]
pub(crate) const PROCESS_ARCHITECTURE: ProcessorArchitecture = ProcessorArchitecture::X86;

/// The packages of `family_name` registered for the current user; a family can
/// have several versions and architectures side by side.
pub(crate) fn registered(
    family_name: &str,
) -> windows_core::Result<Vec<(Version, ProcessorArchitecture)>> {
    // An empty security identifier selects the current user.
    PackageManager::new()?
        .FindPackagesByUserSecurityIdPackageFamilyName(&Default::default(), &family_name.into())?
        .into_iter()
        .map(|package| identity(&package))
        .collect()
}

fn identity(package: &Package) -> windows_core::Result<(Version, ProcessorArchitecture)> {
    let id = package.Id()?;
    let version = id.Version()?;
    Ok((
        Version::new(
            version.Major,
            version.Minor,
            version.Build,
            version.Revision,
        ),
        id.Architecture()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_absent_family_has_no_packages() {
        assert!(
            registered("Nyanpasu.NoSuchFramework_0000000000000")
                .unwrap()
                .is_empty()
        );
    }
}
