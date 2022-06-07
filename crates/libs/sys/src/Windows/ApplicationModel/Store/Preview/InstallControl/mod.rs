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
impl ::windows_sys::core::Interface for IAppInstallItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1238622123, data2: 5770, data3: 19647, data4: [169, 58, 158, 68, 140, 130, 115, 125] };
}
#[repr(C)]
pub struct IAppInstallItem2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CancelWithTelemetry: unsafe extern "system" fn(this: *mut *mut Self, correlationvector: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PauseWithTelemetry: unsafe extern "system" fn(this: *mut *mut Self, correlationvector: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RestartWithTelemetry: unsafe extern "system" fn(this: *mut *mut Self, correlationvector: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppInstallItem2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3549899512, data2: 16576, data3: 20439, data4: [170, 108, 10, 161, 60, 166, 24, 140] };
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
impl ::windows_sys::core::Interface for IAppInstallItem3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1866320280, data2: 56647, data3: 17212, data4: [146, 52, 86, 1, 114, 214, 122, 69] };
}
#[repr(C)]
pub struct IAppInstallItem4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LaunchAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetLaunchAfterInstall: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppInstallItem4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3268529682, data2: 29183, data3: 20424, data4: [181, 64, 69, 61, 75, 55, 225, 209] };
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
impl ::windows_sys::core::Interface for IAppInstallItem5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1427171276, data2: 16502, data3: 18955, data4: [148, 114, 194, 29, 157, 56, 14, 85] };
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
impl ::windows_sys::core::Interface for IAppInstallManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2471747952, data2: 33857, data3: 19269, data4: [189, 114, 124, 47, 169, 37, 190, 238] };
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
impl ::windows_sys::core::Interface for IAppInstallManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 378763345, data2: 60727, data3: 18445, data4: [131, 20, 82, 226, 124, 3, 240, 74] };
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
impl ::windows_sys::core::Interface for IAppInstallManager3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2511489815, data2: 59754, data3: 19726, data4: [132, 225, 200, 203, 65, 122, 1, 120] };
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
impl ::windows_sys::core::Interface for IAppInstallManager4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 638200342, data2: 23198, data3: 20157, data4: [185, 68, 242, 186, 117, 195, 17, 89] };
}
#[repr(C)]
pub struct IAppInstallManager5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AppInstallItemsWithGroupSupport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppInstallItemsWithGroupSupport: usize,
}
impl ::windows_sys::core::Interface for IAppInstallManager5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1020771916, data2: 7145, data3: 20351, data4: [182, 117, 170, 29, 100, 165, 41, 178] };
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
impl ::windows_sys::core::Interface for IAppInstallManager6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3387413512, data2: 62074, data3: 17521, data4: [178, 244, 231, 110, 252, 190, 188, 202] };
}
#[repr(C)]
pub struct IAppInstallManager7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanInstallForAllUsers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppInstallManager7 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2783869744, data2: 54756, data3: 18851, data4: [152, 83, 61, 176, 50, 3, 50, 29] };
}
#[repr(C)]
pub struct IAppInstallManagerItemEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppInstallManagerItemEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3159381827, data2: 18036, data3: 19921, data4: [149, 126, 194, 86, 130, 8, 106, 20] };
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
impl ::windows_sys::core::Interface for IAppInstallOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3380642560, data2: 7352, data3: 20150, data4: [140, 159, 106, 48, 198, 74, 91, 81] };
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
impl ::windows_sys::core::Interface for IAppInstallOptions2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2315567319, data2: 51531, data3: 16990, data4: [149, 180, 191, 39, 250, 234, 238, 137] };
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
impl ::windows_sys::core::Interface for IAppInstallStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2473446650, data2: 9296, data3: 16678, data4: [136, 177, 97, 39, 166, 68, 221, 92] };
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
impl ::windows_sys::core::Interface for IAppInstallStatus2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2531754378, data2: 24210, data3: 19113, data4: [142, 220, 88, 254, 212, 184, 126, 0] };
}
#[repr(C)]
pub struct IAppInstallStatus3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsStaged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppInstallStatus3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3414690902, data2: 33659, data3: 19276, data4: [158, 187, 109, 68, 160, 169, 99, 7] };
}
#[repr(C)]
pub struct IAppUpdateOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub CatalogId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCatalogId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppUpdateOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 653307951, data2: 49907, data3: 19178, data4: [175, 140, 99, 8, 221, 157, 184, 95] };
}
#[repr(C)]
pub struct IAppUpdateOptions2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppUpdateOptions2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4100222472, data2: 60710, data3: 19449, data4: [150, 121, 72, 246, 40, 229, 61, 248] };
}
#[repr(C)]
pub struct IGetEntitlementResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GetEntitlementStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGetEntitlementResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1962705983, data2: 6814, data3: 17929, data4: [142, 77, 129, 144, 134, 208, 138, 61] };
}
