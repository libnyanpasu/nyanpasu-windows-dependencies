use snafu::{OptionExt, ResultExt, Snafu};
use windows_registry::{CURRENT_USER, Key, LOCAL_MACHINE};

use crate::{
    Version,
    packages::{self, PROCESS_ARCHITECTURE},
};

/// A runtime that Windows does not ship with every edition, so a program
/// depending on it has to check for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameworkDependency {
    /// The Visual C++ v14 Redistributable (`vcruntime140.dll` and friends).
    VisualCppRuntime,
    /// The Evergreen Microsoft Edge WebView2 Runtime.
    WebView2Runtime,
    /// The `Microsoft.WindowsAppRuntime.2` framework package of Windows App SDK 2.x.
    WindowsAppRuntime2,
}

/// Why a dependency could not be diagnosed. A dependency that is simply absent
/// is not an error.
#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub(crate)))]
pub enum DiagnoseError {
    #[snafu(display("could not read the registry key {path}"))]
    Registry {
        path: String,
        source: windows_core::Error,
    },
    #[snafu(display("the registry key {path} holds the invalid version {value:?}"))]
    InvalidVersion { path: String, value: String },
    #[snafu(display("could not list the packages of {family_name}"))]
    Packages {
        family_name: &'static str,
        source: windows_core::Error,
    },
}

const VISUAL_CPP_RUNTIMES: &str = r"SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes";
#[cfg(target_arch = "x86_64")]
const VISUAL_CPP_ARCHITECTURE: &str = "x64";
#[cfg(target_arch = "aarch64")]
const VISUAL_CPP_ARCHITECTURE: &str = "arm64";
#[cfg(target_arch = "x86")]
const VISUAL_CPP_ARCHITECTURE: &str = "x86";

/// The Evergreen runtime's client under EdgeUpdate, per machine and per user.
const WEBVIEW2_MACHINE: &str =
    r"SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}";
const WEBVIEW2_USER: &str =
    r"Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}";

const WINDOWS_APP_RUNTIME_2: &str = "Microsoft.WindowsAppRuntime.2_8wekyb3d8bbwe";

/// `HRESULT_FROM_WIN32(ERROR_FILE_NOT_FOUND)`: a missing key or value.
const NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8007_0002_u32 as i32);

impl FrameworkDependency {
    /// The highest version installed for this process's architecture, or `None`
    /// when the dependency is not installed.
    pub fn diagnose(self) -> Result<Option<Version>, DiagnoseError> {
        match self {
            Self::VisualCppRuntime => visual_cpp_runtime(),
            Self::WebView2Runtime => webview2_runtime(),
            Self::WindowsAppRuntime2 => windows_app_runtime(WINDOWS_APP_RUNTIME_2),
        }
    }

    /// The Microsoft Store product that installs this dependency, when the Store
    /// has one: install it with [`crate::store::StoreInstaller`]. Its Store page
    /// may still offer no way to acquire it.
    pub const fn store_product_id(self) -> Option<&'static str> {
        match self {
            Self::WindowsAppRuntime2 => Some("9NRZT3Q9R3DL"),
            Self::VisualCppRuntime | Self::WebView2Runtime => None,
        }
    }
}

/// The Redistributable records itself in the 32-bit registry view for every
/// architecture.
fn visual_cpp_runtime() -> Result<Option<Version>, DiagnoseError> {
    let path = format!(r"{VISUAL_CPP_RUNTIMES}\{VISUAL_CPP_ARCHITECTURE}");
    let Some(key) = open(LOCAL_MACHINE.options().read().wow64_32(), &path)? else {
        return Ok(None);
    };
    let read = |name: &str| {
        optional(key.get_u32(name)).context(diagnose_error::RegistrySnafu { path: &path })
    };
    if read("Installed")? != Some(1) {
        return Ok(None);
    }
    let mut parts = [0; 4];
    for (part, name) in parts.iter_mut().zip(["Major", "Minor", "Bld", "Rbld"]) {
        let value = read(name)?.context(diagnose_error::InvalidVersionSnafu {
            path: &path,
            value: format!("no {name}"),
        })?;
        *part = u16::try_from(value)
            .ok()
            .context(diagnose_error::InvalidVersionSnafu {
                path: &path,
                value: value.to_string(),
            })?;
    }
    let [major, minor, build, revision] = parts;
    Ok(Some(Version::new(major, minor, build, revision)))
}

/// The runtime is installed per machine or per user; either counts.
fn webview2_runtime() -> Result<Option<Version>, DiagnoseError> {
    let mut machine = LOCAL_MACHINE.options();
    machine.read().wow64_32();
    let mut user = CURRENT_USER.options();
    user.read();
    let mut found = None;
    for (options, path) in [(&machine, WEBVIEW2_MACHINE), (&user, WEBVIEW2_USER)] {
        let Some(key) = open(options, path)? else {
            continue;
        };
        let value = optional(key.get_string("pv"))
            .context(diagnose_error::RegistrySnafu { path })?
            .unwrap_or_default();
        found = found.max(webview2_version(path, &value)?);
    }
    Ok(found)
}

/// An empty `pv`, or `0.0.0.0`, means the runtime is not installed.
fn webview2_version(path: &str, value: &str) -> Result<Option<Version>, DiagnoseError> {
    if value.is_empty() {
        return Ok(None);
    }
    let version: Version = value
        .parse()
        .ok()
        .context(diagnose_error::InvalidVersionSnafu { path, value })?;
    Ok((version != Version::new(0, 0, 0, 0)).then_some(version))
}

fn windows_app_runtime(family_name: &'static str) -> Result<Option<Version>, DiagnoseError> {
    let packages =
        packages::registered(family_name).context(diagnose_error::PackagesSnafu { family_name })?;
    Ok(packages
        .into_iter()
        .filter(|(_, architecture)| *architecture == PROCESS_ARCHITECTURE)
        .map(|(version, _)| version)
        .max())
}

fn open(
    options: &windows_registry::OpenOptions<'_>,
    path: &str,
) -> Result<Option<Key>, DiagnoseError> {
    optional(options.open(path)).context(diagnose_error::RegistrySnafu { path })
}

fn optional<T>(result: windows_core::Result<T>) -> windows_core::Result<Option<T>> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(error) if error.code() == NOT_FOUND => Ok(None),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_or_zero_webview2_version_is_absent() {
        assert_eq!(webview2_version("pv", "").unwrap(), None);
        assert_eq!(webview2_version("pv", "0.0.0.0").unwrap(), None);
        assert_eq!(
            webview2_version("pv", "155.0.4283.62").unwrap(),
            Some(Version::new(155, 0, 4283, 62))
        );
        assert!(webview2_version("pv", "garbage").is_err());
    }

    #[test]
    fn only_windows_app_runtime_has_a_store_product() {
        assert_eq!(
            FrameworkDependency::WindowsAppRuntime2.store_product_id(),
            Some("9NRZT3Q9R3DL")
        );
        assert_eq!(
            FrameworkDependency::VisualCppRuntime.store_product_id(),
            None
        );
        assert_eq!(
            FrameworkDependency::WebView2Runtime.store_product_id(),
            None
        );
    }

    #[test]
    fn diagnoses_every_dependency_on_this_machine() {
        for dependency in [
            FrameworkDependency::VisualCppRuntime,
            FrameworkDependency::WebView2Runtime,
            FrameworkDependency::WindowsAppRuntime2,
        ] {
            dependency.diagnose().unwrap();
        }
    }
}
