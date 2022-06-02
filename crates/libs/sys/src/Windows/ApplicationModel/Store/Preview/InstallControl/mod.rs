pub type AppInstallItem = *mut ::core::ffi::c_void;
pub type AppInstallManager = *mut ::core::ffi::c_void;
pub type AppInstallManagerItemEventArgs = *mut ::core::ffi::c_void;
pub type AppInstallOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallState(pub i32);
impl AppInstallState {
    pub const Pending: Self = Self(0i32);
    pub const Starting: Self = Self(1i32);
    pub const AcquiringLicense: Self = Self(2i32);
    pub const Downloading: Self = Self(3i32);
    pub const RestoringData: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const Paused: Self = Self(8i32);
    pub const Error: Self = Self(9i32);
    pub const PausedLowBattery: Self = Self(10i32);
    pub const PausedWiFiRecommended: Self = Self(11i32);
    pub const PausedWiFiRequired: Self = Self(12i32);
    pub const ReadyToDownload: Self = Self(13i32);
}
impl ::core::marker::Copy for AppInstallState {}
impl ::core::clone::Clone for AppInstallState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppInstallStatus = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallType(pub i32);
impl AppInstallType {
    pub const Install: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Repair: Self = Self(2i32);
}
impl ::core::marker::Copy for AppInstallType {}
impl ::core::clone::Clone for AppInstallType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallationToastNotificationMode(pub i32);
impl AppInstallationToastNotificationMode {
    pub const Default: Self = Self(0i32);
    pub const Toast: Self = Self(1i32);
    pub const ToastWithoutPopup: Self = Self(2i32);
    pub const NoToast: Self = Self(3i32);
}
impl ::core::marker::Copy for AppInstallationToastNotificationMode {}
impl ::core::clone::Clone for AppInstallationToastNotificationMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppUpdateOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AutoUpdateSetting(pub i32);
impl AutoUpdateSetting {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const EnabledByPolicy: Self = Self(3i32);
}
impl ::core::marker::Copy for AutoUpdateSetting {}
impl ::core::clone::Clone for AutoUpdateSetting {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GetEntitlementResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct GetEntitlementStatus(pub i32);
impl GetEntitlementStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoStoreAccount: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
}
impl ::core::marker::Copy for GetEntitlementStatus {}
impl ::core::clone::Clone for GetEntitlementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAppInstallItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InstallType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppInstallType) -> ::windows_sys::core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetCurrentStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Restart: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
#[repr(C)]
pub struct IAppInstallItem2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CancelWithTelemetry: unsafe extern "system" fn(this: *mut *mut Self, correlationvector: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PauseWithTelemetry: unsafe extern "system" fn(this: *mut *mut Self, correlationvector: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RestartWithTelemetry: unsafe extern "system" fn(this: *mut *mut Self, correlationvector: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstallItem3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    pub ItemOperationsMightAffectOtherItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstallItem4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LaunchAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetLaunchAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstallItem5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PinToStartAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPinToStartAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppInstallationToastNotificationMode) -> ::windows_sys::core::HRESULT,
    pub SetCompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut *mut Self, value: AppInstallationToastNotificationMode) -> ::windows_sys::core::HRESULT,
    pub InstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppInstallationToastNotificationMode) -> ::windows_sys::core::HRESULT,
    pub SetInstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut *mut Self, value: AppInstallationToastNotificationMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstallManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AppInstallItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppInstallItems: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Restart: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ItemStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemStatusChanged: usize,
    pub AutoUpdateSetting: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutoUpdateSetting) -> ::windows_sys::core::HRESULT,
    pub SetAutoUpdateSetting: unsafe extern "system" fn(this: *mut *mut Self, value: AutoUpdateSetting) -> ::windows_sys::core::HRESULT,
    pub AcquisitionIdentity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAcquisitionIdentity: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetIsApplicableAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, skuid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsApplicableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartAppInstallAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, skuid: ::windows_sys::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAppInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAppByPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAppByPackageFamilyNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SearchForUpdatesAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, skuid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchForUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub IsStoreBlockedByPolicyAsync: unsafe extern "system" fn(this: *mut *mut Self, storeclientname: ::windows_sys::core::HSTRING, storeclientpublisher: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsStoreBlockedByPolicyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsAppAllowedToInstallAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsAppAllowedToInstallAsync: usize,
}
#[repr(C)]
pub struct IAppInstallManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartAppInstallWithTelemetryAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, skuid: ::windows_sys::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, catalogid: ::windows_sys::core::HSTRING, bundleid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAppInstallWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAppByPackageFamilyNameWithTelemetryAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAppByPackageFamilyNameWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SearchForUpdatesWithTelemetryAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, skuid: ::windows_sys::core::HSTRING, catalogid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchForUpdatesWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesWithTelemetryAsync: unsafe extern "system" fn(this: *mut *mut Self, correlationvector: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsAppAllowedToInstallWithTelemetryAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, skuid: ::windows_sys::core::HSTRING, catalogid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsAppAllowedToInstallWithTelemetryAsync: usize,
    pub CancelWithTelemetry: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PauseWithTelemetry: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RestartWithTelemetry: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstallManager3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub StartProductInstallAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, catalogid: ::windows_sys::core::HSTRING, flightid: ::windows_sys::core::HSTRING, clientid: ::windows_sys::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::windows_sys::core::HSTRING, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment")))]
    StartProductInstallAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System"))]
    pub StartProductInstallForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, productid: ::windows_sys::core::HSTRING, catalogid: ::windows_sys::core::HSTRING, flightid: ::windows_sys::core::HSTRING, clientid: ::windows_sys::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::windows_sys::core::HSTRING, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System")))]
    StartProductInstallForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub UpdateAppByPackageFamilyNameForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, packagefamilyname: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    UpdateAppByPackageFamilyNameForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub SearchForUpdatesForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, productid: ::windows_sys::core::HSTRING, skuid: ::windows_sys::core::HSTRING, catalogid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    SearchForUpdatesForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub SearchForAllUpdatesForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, correlationvector: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    SearchForAllUpdatesForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetIsAppAllowedToInstallForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, productid: ::windows_sys::core::HSTRING, skuid: ::windows_sys::core::HSTRING, catalogid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetIsAppAllowedToInstallForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetIsApplicableForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, productid: ::windows_sys::core::HSTRING, skuid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetIsApplicableForUserAsync: usize,
    pub MoveToFrontOfDownloadQueue: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstallManager4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetFreeUserEntitlementAsync: unsafe extern "system" fn(this: *mut *mut Self, storeid: ::windows_sys::core::HSTRING, campaignid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFreeUserEntitlementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetFreeUserEntitlementForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, storeid: ::windows_sys::core::HSTRING, campaignid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetFreeUserEntitlementForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFreeDeviceEntitlementAsync: unsafe extern "system" fn(this: *mut *mut Self, storeid: ::windows_sys::core::HSTRING, campaignid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFreeDeviceEntitlementAsync: usize,
}
#[repr(C)]
pub struct IAppInstallManager5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AppInstallItemsWithGroupSupport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppInstallItemsWithGroupSupport: usize,
}
#[repr(C)]
pub struct IAppInstallManager6 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesWithUpdateOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, correlationvector: ::windows_sys::core::HSTRING, clientid: ::windows_sys::core::HSTRING, updateoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesWithUpdateOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub SearchForAllUpdatesWithUpdateOptionsForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, correlationvector: ::windows_sys::core::HSTRING, clientid: ::windows_sys::core::HSTRING, updateoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    SearchForAllUpdatesWithUpdateOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SearchForUpdatesWithUpdateOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, skuid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, clientid: ::windows_sys::core::HSTRING, updateoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchForUpdatesWithUpdateOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub SearchForUpdatesWithUpdateOptionsForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, productid: ::windows_sys::core::HSTRING, skuid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, clientid: ::windows_sys::core::HSTRING, updateoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    SearchForUpdatesWithUpdateOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartProductInstallWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, flightid: ::windows_sys::core::HSTRING, clientid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, installoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartProductInstallWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub StartProductInstallWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, productid: ::windows_sys::core::HSTRING, flightid: ::windows_sys::core::HSTRING, clientid: ::windows_sys::core::HSTRING, correlationvector: ::windows_sys::core::HSTRING, installoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    StartProductInstallWithOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsPackageIdentityAllowedToInstallAsync: unsafe extern "system" fn(this: *mut *mut Self, correlationvector: ::windows_sys::core::HSTRING, packageidentityname: ::windows_sys::core::HSTRING, publishercertificatename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsPackageIdentityAllowedToInstallAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetIsPackageIdentityAllowedToInstallForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, correlationvector: ::windows_sys::core::HSTRING, packageidentityname: ::windows_sys::core::HSTRING, publishercertificatename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetIsPackageIdentityAllowedToInstallForUserAsync: usize,
}
#[repr(C)]
pub struct IAppInstallManager7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanInstallForAllUsers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstallManagerItemEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstallOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub CatalogId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCatalogId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ForceUseOfNonRemovableStorage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceUseOfNonRemovableStorage: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Repair: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRepair: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Management_Deployment")]
    pub TargetVolume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    TargetVolume: usize,
    #[cfg(feature = "Management_Deployment")]
    pub SetTargetVolume: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    SetTargetVolume: usize,
    pub LaunchAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetLaunchAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstallOptions2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PinToStartAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPinToStartAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppInstallationToastNotificationMode) -> ::windows_sys::core::HRESULT,
    pub SetCompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut *mut Self, value: AppInstallationToastNotificationMode) -> ::windows_sys::core::HRESULT,
    pub InstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppInstallationToastNotificationMode) -> ::windows_sys::core::HRESULT,
    pub SetInstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut *mut Self, value: AppInstallationToastNotificationMode) -> ::windows_sys::core::HRESULT,
    pub InstallForAllUsers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetInstallForAllUsers: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub StageButDoNotInstall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStageButDoNotInstall: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CampaignId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCampaignId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ExtendedCampaignId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetExtendedCampaignId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstallStatus {
    pub base__: ::windows_sys::core::IInspectable,
    pub InstallState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppInstallState) -> ::windows_sys::core::HRESULT,
    pub DownloadSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub BytesDownloaded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstallStatus2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub ReadyForLaunch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstallStatus3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsStaged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppUpdateOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub CatalogId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCatalogId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppUpdateOptions2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGetEntitlementResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GetEntitlementStatus) -> ::windows_sys::core::HRESULT,
}
