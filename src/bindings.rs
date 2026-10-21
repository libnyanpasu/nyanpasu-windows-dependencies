#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppInstallItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppInstallItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl AppInstallItem {
    pub fn ProductId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetCurrentStatus(&self) -> windows_core::Result<AppInstallStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentStatus)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StatusChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>)
            + Send
            + 'static,
    {
        let handler = <TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).StatusChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveStatusChanged,
            ))
        }
    }
}
impl windows_core::RuntimeType for AppInstallItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppInstallItem>();
}
unsafe impl windows_core::Interface for AppInstallItem {
    type Vtable = <IAppInstallItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppInstallItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppInstallItem {
    const NAME: &'static str =
        "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem";
}
unsafe impl Send for AppInstallItem {}
unsafe impl Sync for AppInstallItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppInstallManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppInstallManager,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl AppInstallManager {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            AppInstallManager,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsStoreBlockedByPolicyAsync(
        &self,
        storeclientname: &windows_core::HSTRING,
        storeclientpublisher: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsStoreBlockedByPolicyAsync)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(storeclientname),
                core::mem::transmute_copy(storeclientpublisher),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetFreeUserEntitlementAsync(
        &self,
        storeid: &windows_core::HSTRING,
        campaignid: &windows_core::HSTRING,
        correlationvector: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_future::IAsyncOperation<GetEntitlementResult>> {
        let this = &windows_core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFreeUserEntitlementAsync)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(storeid),
                core::mem::transmute_copy(campaignid),
                core::mem::transmute_copy(correlationvector),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetFreeDeviceEntitlementAsync(
        &self,
        storeid: &windows_core::HSTRING,
        campaignid: &windows_core::HSTRING,
        correlationvector: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_future::IAsyncOperation<GetEntitlementResult>> {
        let this = &windows_core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFreeDeviceEntitlementAsync)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(storeid),
                core::mem::transmute_copy(campaignid),
                core::mem::transmute_copy(correlationvector),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StartProductInstallWithOptionsAsync<P4>(
        &self,
        productid: &windows_core::HSTRING,
        flightid: &windows_core::HSTRING,
        clientid: &windows_core::HSTRING,
        correlationvector: &windows_core::HSTRING,
        installoptions: P4,
    ) -> windows_core::Result<
        windows_future::IAsyncOperation<windows_collections::IVectorView<AppInstallItem>>,
    >
    where
        P4: windows_core::Param<AppInstallOptions>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartProductInstallWithOptionsAsync)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(productid),
                core::mem::transmute_copy(flightid),
                core::mem::transmute_copy(clientid),
                core::mem::transmute_copy(correlationvector),
                installoptions.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppInstallManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppInstallManager>();
}
unsafe impl windows_core::Interface for AppInstallManager {
    type Vtable = <IAppInstallManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppInstallManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppInstallManager {
    const NAME: &'static str =
        "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager";
}
unsafe impl Send for AppInstallManager {}
unsafe impl Sync for AppInstallManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppInstallOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppInstallOptions,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl AppInstallOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            AppInstallOptions,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppInstallOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppInstallOptions>();
}
unsafe impl windows_core::Interface for AppInstallOptions {
    type Vtable = <IAppInstallOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppInstallOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppInstallOptions {
    const NAME: &'static str =
        "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions";
}
unsafe impl Send for AppInstallOptions {}
unsafe impl Sync for AppInstallOptions {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AppInstallState(pub i32);
impl AppInstallState {
    pub const Pending: Self = Self(0);
    pub const Starting: Self = Self(1);
    pub const AcquiringLicense: Self = Self(2);
    pub const Downloading: Self = Self(3);
    pub const RestoringData: Self = Self(4);
    pub const Installing: Self = Self(5);
    pub const Completed: Self = Self(6);
    pub const Canceled: Self = Self(7);
    pub const Paused: Self = Self(8);
    pub const Error: Self = Self(9);
    pub const PausedLowBattery: Self = Self(10);
    pub const PausedWiFiRecommended: Self = Self(11);
    pub const PausedWiFiRequired: Self = Self(12);
    pub const ReadyToDownload: Self = Self(13);
}
impl windows_core::imp::TypeKind for AppInstallState {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for AppInstallState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallState;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallState",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppInstallStatus(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppInstallStatus,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl AppInstallStatus {
    pub fn InstallState(&self) -> windows_core::Result<AppInstallState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstallState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PercentComplete(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PercentComplete)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ErrorCode)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppInstallStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppInstallStatus>();
}
unsafe impl windows_core::Interface for AppInstallStatus {
    type Vtable = <IAppInstallStatus as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppInstallStatus as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppInstallStatus {
    const NAME: &'static str =
        "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus";
}
unsafe impl Send for AppInstallStatus {}
unsafe impl Sync for AppInstallStatus {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GetEntitlementResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    GetEntitlementResult,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl GetEntitlementResult {
    pub fn Status(&self) -> windows_core::Result<GetEntitlementStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GetEntitlementResult {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IGetEntitlementResult>();
}
unsafe impl windows_core::Interface for GetEntitlementResult {
    type Vtable = <IGetEntitlementResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGetEntitlementResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GetEntitlementResult {
    const NAME: &'static str =
        "Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult";
}
unsafe impl Send for GetEntitlementResult {}
unsafe impl Sync for GetEntitlementResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GetEntitlementStatus(pub i32);
impl GetEntitlementStatus {
    pub const Succeeded: Self = Self(0);
    pub const NoStoreAccount: Self = Self(1);
    pub const NetworkError: Self = Self(2);
    pub const ServerError: Self = Self(3);
}
impl windows_core::imp::TypeKind for GetEntitlementStatus {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for GetEntitlementStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementStatus;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementStatus",
    );
}
windows_core::imp::define_interface!(
    IAppInstallItem,
    IAppInstallItem_Vtbl,
    0x49d3dfab_168a_4cbf_a93a_9e448c82737d
);
impl windows_core::RuntimeType for IAppInstallItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem",
    );
}
#[repr(C)]
pub struct IAppInstallItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    PackageFamilyName: usize,
    InstallType: usize,
    IsUserInitiated: usize,
    pub GetCurrentStatus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Cancel: usize,
    Pause: usize,
    Restart: usize,
    Completed: usize,
    RemoveCompleted: usize,
    pub StatusChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveStatusChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppInstallManager,
    IAppInstallManager_Vtbl,
    0x9353e170_8441_4b45_bd72_7c2fa925beee
);
impl windows_core::RuntimeType for IAppInstallManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager",
    );
}
#[repr(C)]
pub struct IAppInstallManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    AppInstallItems: usize,
    Cancel: usize,
    Pause: usize,
    Restart: usize,
    ItemCompleted: usize,
    RemoveItemCompleted: usize,
    ItemStatusChanged: usize,
    RemoveItemStatusChanged: usize,
    AutoUpdateSetting: usize,
    SetAutoUpdateSetting: usize,
    AcquisitionIdentity: usize,
    SetAcquisitionIdentity: usize,
    GetIsApplicableAsync: usize,
    StartAppInstallAsync: usize,
    UpdateAppByPackageFamilyNameAsync: usize,
    SearchForUpdatesAsync: usize,
    SearchForAllUpdatesAsync: usize,
    pub IsStoreBlockedByPolicyAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppInstallManager4,
    IAppInstallManager4_Vtbl,
    0x260a2a16_5a9e_4ebd_b944_f2ba75c31159
);
impl windows_core::RuntimeType for IAppInstallManager4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager4",
    );
}
#[repr(C)]
pub struct IAppInstallManager4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFreeUserEntitlementAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetFreeUserEntitlementForUserAsync: usize,
    pub GetFreeDeviceEntitlementAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppInstallManager6,
    IAppInstallManager6_Vtbl,
    0xc9e7d408_f27a_4471_b2f4_e76efcbebcca
);
impl windows_core::RuntimeType for IAppInstallManager6 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager6",
    );
}
#[repr(C)]
pub struct IAppInstallManager6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    SearchForAllUpdatesWithUpdateOptionsAsync: usize,
    SearchForAllUpdatesWithUpdateOptionsForUserAsync: usize,
    SearchForUpdatesWithUpdateOptionsAsync: usize,
    SearchForUpdatesWithUpdateOptionsForUserAsync: usize,
    pub StartProductInstallWithOptionsAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppInstallOptions,
    IAppInstallOptions_Vtbl,
    0xc9808300_1cb8_4eb6_8c9f_6a30c64a5b51
);
impl windows_core::RuntimeType for IAppInstallOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions",
    );
}
#[repr(C)]
pub struct IAppInstallOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IAppInstallStatus,
    IAppInstallStatus_Vtbl,
    0x936dccfa_2450_4126_88b1_6127a644dd5c
);
impl windows_core::RuntimeType for IAppInstallStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus",
    );
}
#[repr(C)]
pub struct IAppInstallStatus_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InstallState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AppInstallState,
    ) -> windows_core::HRESULT,
    DownloadSizeInBytes: usize,
    BytesDownloaded: usize,
    pub PercentComplete:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::HRESULT,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IGetEntitlementResult,
    IGetEntitlementResult_Vtbl,
    0x74fc843f_1a9e_4609_8e4d_819086d08a3d
);
impl windows_core::RuntimeType for IGetEntitlementResult {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult",
    );
}
#[repr(C)]
pub struct IGetEntitlementResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut GetEntitlementStatus,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPackage,
    IPackage_Vtbl,
    0x163c792f_bd75_413c_bf23_b1fe7b95d825
);
impl windows_core::RuntimeType for IPackage {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.IPackage");
}
#[repr(C)]
pub struct IPackage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPackageId,
    IPackageId_Vtbl,
    0x1adb665e_37c7_4790_9980_dd7ae74e8bb2
);
impl windows_core::RuntimeType for IPackageId {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.IPackageId");
}
#[repr(C)]
pub struct IPackageId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Name: usize,
    pub Version: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PackageVersion,
    ) -> windows_core::HRESULT,
    pub Architecture: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ProcessorArchitecture,
    ) -> windows_core::HRESULT,
    ResourceId: usize,
    Publisher: usize,
    PublisherId: usize,
    pub FullName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPackageManager,
    IPackageManager_Vtbl,
    0x9a7d4b65_5e8f_4fc7_a2e5_7f6925cb8b53
);
impl windows_core::RuntimeType for IPackageManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.Management.Deployment.IPackageManager",
    );
}
#[repr(C)]
pub struct IPackageManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    AddPackageAsync: usize,
    UpdatePackageAsync: usize,
    RemovePackageAsync: usize,
    StagePackageAsync: usize,
    RegisterPackageAsync: usize,
    FindPackages: usize,
    FindPackagesByUserSecurityId: usize,
    FindPackagesByNamePublisher: usize,
    FindPackagesByUserSecurityIdNamePublisher: usize,
    FindUsers: usize,
    SetPackageState: usize,
    FindPackageByPackageFullName: usize,
    CleanupPackageForUserAsync: usize,
    FindPackagesByPackageFamilyName: usize,
    pub FindPackagesByUserSecurityIdPackageFamilyName:
        unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPackageStatics,
    IPackageStatics_Vtbl,
    0x4e534bdf_2960_4878_97a4_9624deb72f2d
);
impl windows_core::RuntimeType for IPackageStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.IPackageStatics");
}
#[repr(C)]
pub struct IPackageStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Package(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Package,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Package {
    pub fn Id(&self) -> windows_core::Result<PackageId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    fn IPackageStatics<R, F: FnOnce(&IPackageStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Package, IPackageStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Package {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPackage>();
}
unsafe impl windows_core::Interface for Package {
    type Vtable = <IPackage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPackage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Package {
    const NAME: &'static str = "Windows.ApplicationModel.Package";
}
unsafe impl Send for Package {}
unsafe impl Sync for Package {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageId(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PackageId,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl PackageId {
    pub fn Version(&self) -> windows_core::Result<PackageVersion> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Version)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Architecture(&self) -> windows_core::Result<ProcessorArchitecture> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Architecture)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn FullName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FullName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for PackageId {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPackageId>();
}
unsafe impl windows_core::Interface for PackageId {
    type Vtable = <IPackageId as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPackageId as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageId {
    const NAME: &'static str = "Windows.ApplicationModel.PackageId";
}
unsafe impl Send for PackageId {}
unsafe impl Sync for PackageId {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PackageManager,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl PackageManager {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            PackageManager,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn FindPackagesByUserSecurityIdPackageFamilyName(
        &self,
        usersecurityid: &windows_core::HSTRING,
        packagefamilyname: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_collections::IIterable<Package>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindPackagesByUserSecurityIdPackageFamilyName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(usersecurityid),
                core::mem::transmute_copy(packagefamilyname),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PackageManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPackageManager>();
}
unsafe impl windows_core::Interface for PackageManager {
    type Vtable = <IPackageManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPackageManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageManager {
    const NAME: &'static str = "Windows.Management.Deployment.PackageManager";
}
unsafe impl Send for PackageManager {}
unsafe impl Sync for PackageManager {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PackageVersion {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl windows_core::imp::TypeKind for PackageVersion {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for PackageVersion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.ApplicationModel.PackageVersion;u2;u2;u2;u2)",
    );
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.PackageVersion");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProcessorArchitecture(pub i32);
impl ProcessorArchitecture {
    pub const X86: Self = Self(0);
    pub const Arm: Self = Self(5);
    pub const X64: Self = Self(9);
    pub const Neutral: Self = Self(11);
    pub const Arm64: Self = Self(12);
    pub const X86OnArm64: Self = Self(14);
    pub const Unknown: Self = Self(65535);
}
impl windows_core::imp::TypeKind for ProcessorArchitecture {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ProcessorArchitecture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.System.ProcessorArchitecture;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.System.ProcessorArchitecture");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedEventHandler<TSender, TResult>(
    windows_core::IUnknown,
    core::marker::PhantomData<TSender>,
    core::marker::PhantomData<TResult>,
)
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static;
unsafe impl<
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static,
> windows_core::Interface for TypedEventHandler<TSender, TResult>
{
    type Vtable = TypedEventHandler_Vtbl<TSender, TResult>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static>
    windows_core::RuntimeType for TypedEventHandler<TSender, TResult>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({9de1c534-6ae1-11e0-84e1-18a905bcc53f}")
        .push_slice(b";")
        .push_other(TSender::SIGNATURE)
        .push_slice(b";")
        .push_other(TResult::SIGNATURE)
        .push_slice(b")");
}
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static>
    TypedEventHandler<TSender, TResult>
{
    pub fn new<
        F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) -> windows_core::Result<()>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &TypedEventHandlerBox::<TSender, TResult, F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<TSender>,
        P1: windows_core::Param<TResult>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                sender.param().abi(),
                args.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct TypedEventHandler_Vtbl<TSender, TResult>
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: windows_core::imp::AbiType<TSender>,
        args: windows_core::imp::AbiType<TResult>,
    ) -> windows_core::HRESULT,
    TSender: core::marker::PhantomData<TSender>,
    TResult: core::marker::PhantomData<TResult>,
}
struct TypedEventHandlerBox<
    TSender,
    TResult,
    F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) -> windows_core::Result<()>
        + Send
        + 'static,
>(core::marker::PhantomData<(TSender, TResult, fn() -> F)>)
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static;
impl<
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static,
    F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) -> windows_core::Result<()>
        + Send
        + 'static,
> TypedEventHandlerBox<TSender, TResult, F>
{
    const VTABLE : TypedEventHandler_Vtbl < TSender , TResult , > = TypedEventHandler_Vtbl::< TSender , TResult , > { base__ : windows_core::IUnknown_Vtbl { QueryInterface : windows_core::imp::DelegateBox::< TypedEventHandler < TSender , TResult > , F >::QueryInterface , AddRef : windows_core::imp::DelegateBox::< TypedEventHandler < TSender , TResult > , F >::AddRef , Release : windows_core::imp::DelegateBox::< TypedEventHandler < TSender , TResult > , F >::Release , } , Invoke : Self::Invoke , TSender : core::marker::PhantomData::< TSender > , TResult : core::marker::PhantomData::< TResult > } ;
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: windows_core::imp::AbiType<TSender>,
        args: windows_core::imp::AbiType<TResult>,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<TypedEventHandler<TSender, TResult>, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&args),
            )
            .into()
        }
    }
}
