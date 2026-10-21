//! Installs Microsoft Store products through the Store install service, the
//! service behind `winget install --source msstore`. It installs by product ID,
//! including products whose Store page offers no way to acquire them.
//!
//! The calls block; run them off a thread that must stay responsive.

use std::{collections::HashSet, sync::mpsc};

use snafu::{ResultExt, Snafu};
use strum::FromRepr;
use windows_core::HSTRING;

use crate::{
    bindings::{AppInstallItem, AppInstallManager, AppInstallOptions, GetEntitlementStatus},
    packages,
};

/// The Microsoft Store app, whose policy decides whether the Store may install.
const STORE_FAMILY: &str = "Microsoft.WindowsStore_8wekyb3d8bbwe";
const STORE_CLIENT_NAME: &str = "Microsoft.WindowsStore";
const STORE_CLIENT_PUBLISHER: &str =
    "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US";

/// Why the Store install service could not be queried.
#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub(crate)))]
pub enum StoreError {
    #[snafu(display("could not reach the Microsoft Store install service"))]
    Activate { source: windows_core::Error },
    #[snafu(display("could not find whether the Microsoft Store is registered"))]
    Registration { source: windows_core::Error },
    #[snafu(display("could not read whether policy blocks the Microsoft Store"))]
    Policy { source: windows_core::Error },
}

/// Why a product was not installed.
#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub(crate)))]
pub enum InstallError {
    #[snafu(display("the Microsoft Store did not queue {product_id}"))]
    Queue {
        product_id: String,
        source: windows_core::Error,
    },
    #[snafu(display("the Microsoft Store queued nothing for {product_id}"))]
    NothingQueued { product_id: String },
    #[snafu(display("could not follow the installation of {product_id}"))]
    Watch {
        product_id: String,
        source: windows_core::Error,
    },
    #[snafu(display("the Microsoft Store failed to install {product_id}"))]
    Failed {
        product_id: String,
        source: windows_core::Error,
    },
    #[snafu(display("the installation of {product_id} was canceled"))]
    Canceled { product_id: String },
}

/// The state of one queued product, mirroring `AppInstallState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr)]
#[repr(i32)]
pub enum InstallState {
    Pending = 0,
    Starting = 1,
    AcquiringLicense = 2,
    Downloading = 3,
    RestoringData = 4,
    Installing = 5,
    Completed = 6,
    Canceled = 7,
    Paused = 8,
    Error = 9,
    PausedLowBattery = 10,
    PausedWiFiRecommended = 11,
    PausedWiFiRequired = 12,
    ReadyToDownload = 13,
}

/// A status update of one queued product. Installing a product can queue the
/// products it depends on as well.
#[derive(Debug, Clone, PartialEq)]
pub struct InstallProgress {
    pub product_id: String,
    /// `None` for a state newer than this crate.
    pub state: Option<InstallState>,
    pub percent: f64,
}

/// A client of the Store install service.
pub struct StoreInstaller {
    manager: AppInstallManager,
    client_id: HSTRING,
}

impl StoreInstaller {
    /// `client_id` names the caller in the Store's install log.
    pub fn new(client_id: &str) -> Result<Self, StoreError> {
        Ok(Self {
            manager: AppInstallManager::new().context(store_error::ActivateSnafu)?,
            client_id: client_id.into(),
        })
    }

    /// Whether the Store can install products for the current user: the Store app
    /// is registered and policy allows it. The install service itself is present
    /// without the Store app, as on Windows LTSC, but then finds no product.
    /// Installing can still fail, for example offline, so treat a failed install
    /// as unavailable too.
    pub fn is_available(&self) -> Result<bool, StoreError> {
        if packages::registered(STORE_FAMILY)
            .context(store_error::RegistrationSnafu)?
            .is_empty()
        {
            return Ok(false);
        }
        let blocked = self
            .manager
            .IsStoreBlockedByPolicyAsync(&STORE_CLIENT_NAME.into(), &STORE_CLIENT_PUBLISHER.into())
            .and_then(|operation| operation.join())
            .context(store_error::PolicySnafu)?;
        Ok(!blocked)
    }

    /// Installs `product_id` for the current user and waits until every product it
    /// queued is installed, reporting each status change on this thread. A product
    /// that is already installed completes without downloading. The Store finishes
    /// an installation even if the caller stops waiting.
    pub fn install(
        &self,
        product_id: &str,
        mut on_progress: impl FnMut(&InstallProgress),
    ) -> Result<(), InstallError> {
        let id = HSTRING::from(product_id);
        self.acquire_free_entitlement(&id);

        let items: Vec<AppInstallItem> = AppInstallOptions::new()
            .and_then(|options| {
                self.manager.StartProductInstallWithOptionsAsync(
                    &id,
                    &HSTRING::new(),
                    &self.client_id,
                    &HSTRING::new(),
                    &options,
                )
            })
            .and_then(|operation| operation.join())
            .context(install_error::QueueSnafu { product_id })?
            .into_iter()
            .collect();
        snafu::ensure!(
            !items.is_empty(),
            install_error::NothingQueuedSnafu { product_id }
        );

        let (sender, receiver) = mpsc::channel();
        let mut revokers = Vec::with_capacity(items.len());
        let mut pending = HashSet::with_capacity(items.len());
        for item in &items {
            let changed = sender.clone();
            let revoker = item
                .StatusChanged(move |item, _| {
                    if let Some(item) = item.as_ref() {
                        // The receiver is gone once the installation has finished.
                        let _ = changed.send(item.clone());
                    }
                })
                .context(install_error::WatchSnafu { product_id })?;
            revokers.push(revoker);
            pending.insert(
                item.ProductId()
                    .context(install_error::WatchSnafu { product_id })?
                    .to_string_lossy(),
            );
            // A status reached before the handler was registered raises no event,
            // so read each item once after registering.
            sender
                .send(item.clone())
                .expect("the receiver lives in this function");
        }

        loop {
            let item = receiver.recv().expect("the status handlers keep a sender");
            let progress = progress(&item).context(install_error::WatchSnafu { product_id })?;
            on_progress(&progress);
            match progress.state {
                Some(InstallState::Completed) => {
                    pending.remove(&progress.product_id);
                    if pending.is_empty() {
                        return Ok(());
                    }
                }
                Some(InstallState::Error) => {
                    let code = item
                        .GetCurrentStatus()
                        .and_then(|status| status.ErrorCode())
                        .context(install_error::WatchSnafu { product_id })?;
                    return Err(windows_core::Error::from(code)).context(
                        install_error::FailedSnafu {
                            product_id: progress.product_id,
                        },
                    );
                }
                Some(InstallState::Canceled) => {
                    return install_error::CanceledSnafu {
                        product_id: progress.product_id,
                    }
                    .fail();
                }
                _ => {}
            }
        }
    }

    /// WinGet requests a free entitlement before it installs a product and
    /// installs even when the request fails; this follows it.
    fn acquire_free_entitlement(&self, id: &HSTRING) {
        let empty = HSTRING::new();
        let request = |device: bool| {
            let operation = if device {
                self.manager
                    .GetFreeDeviceEntitlementAsync(id, &empty, &empty)
            } else {
                self.manager.GetFreeUserEntitlementAsync(id, &empty, &empty)
            };
            operation?.join()?.Status()
        };
        let status = request(false).and_then(|status| {
            if status == GetEntitlementStatus::NoStoreAccount {
                request(true)
            } else {
                Ok(status)
            }
        });
        tracing::debug!(product_id = %id.to_string_lossy(), ?status, "requested a free entitlement");
    }
}

fn progress(item: &AppInstallItem) -> windows_core::Result<InstallProgress> {
    let status = item.GetCurrentStatus()?;
    Ok(InstallProgress {
        product_id: item.ProductId()?.to_string_lossy(),
        state: InstallState::from_repr(status.InstallState()?.0),
        percent: status.PercentComplete()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FrameworkDependency, bindings::AppInstallState};

    #[test]
    fn install_states_match_the_windows_values() {
        for (raw, state) in [
            (AppInstallState::Pending, InstallState::Pending),
            (AppInstallState::Completed, InstallState::Completed),
            (AppInstallState::Error, InstallState::Error),
            (
                AppInstallState::ReadyToDownload,
                InstallState::ReadyToDownload,
            ),
        ] {
            assert_eq!(InstallState::from_repr(raw.0), Some(state));
        }
    }

    /// Needs the Microsoft Store and changes the installed packages.
    #[test]
    #[ignore = "installs Windows App Runtime 2 from the Microsoft Store"]
    fn installs_windows_app_runtime_2() {
        let installer = StoreInstaller::new("NyanpasuTest").unwrap();
        assert!(installer.is_available().unwrap());
        let product = FrameworkDependency::WindowsAppRuntime2
            .store_product_id()
            .unwrap();
        installer
            .install(product, |progress| println!("{progress:?}"))
            .unwrap();
        assert!(
            FrameworkDependency::WindowsAppRuntime2
                .diagnose()
                .unwrap()
                .is_some()
        );
    }
}
