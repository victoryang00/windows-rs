#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type AddServiceFlag = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asfAllowPendingRegistration: AddServiceFlag = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asfAllowOnlineRegistration: AddServiceFlag = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asfRegisterServiceWithAU: AddServiceFlag = 4i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type AutoDownloadMode = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const adLetWindowsUpdateDecide: AutoDownloadMode = 0i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const adNeverAutoDownload: AutoDownloadMode = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const adAlwaysAutoDownload: AutoDownloadMode = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type AutoSelectionMode = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asLetWindowsUpdateDecide: AutoSelectionMode = 0i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asAutoSelectIfDownloaded: AutoSelectionMode = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asNeverAutoSelect: AutoSelectionMode = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asAlwaysAutoSelect: AutoSelectionMode = 3i32;
pub const AutomaticUpdates: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3219230364, data2: 28039, data3: 17488, data4: [179, 124, 224, 47, 11, 55, 56, 3] };
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type AutomaticUpdatesNotificationLevel = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const aunlNotConfigured: AutomaticUpdatesNotificationLevel = 0i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const aunlDisabled: AutomaticUpdatesNotificationLevel = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const aunlNotifyBeforeDownload: AutomaticUpdatesNotificationLevel = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const aunlNotifyBeforeInstallation: AutomaticUpdatesNotificationLevel = 3i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const aunlScheduledInstallation: AutomaticUpdatesNotificationLevel = 4i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type AutomaticUpdatesPermissionType = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auptSetNotificationLevel: AutomaticUpdatesPermissionType = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auptDisableAutomaticUpdates: AutomaticUpdatesPermissionType = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auptSetIncludeRecommendedUpdates: AutomaticUpdatesPermissionType = 3i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auptSetFeaturedUpdatesEnabled: AutomaticUpdatesPermissionType = 4i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auptSetNonAdministratorsElevated: AutomaticUpdatesPermissionType = 5i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type AutomaticUpdatesScheduledInstallationDay = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEveryDay: AutomaticUpdatesScheduledInstallationDay = 0i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEverySunday: AutomaticUpdatesScheduledInstallationDay = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEveryMonday: AutomaticUpdatesScheduledInstallationDay = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEveryTuesday: AutomaticUpdatesScheduledInstallationDay = 3i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEveryWednesday: AutomaticUpdatesScheduledInstallationDay = 4i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEveryThursday: AutomaticUpdatesScheduledInstallationDay = 5i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEveryFriday: AutomaticUpdatesScheduledInstallationDay = 6i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEverySaturday: AutomaticUpdatesScheduledInstallationDay = 7i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type AutomaticUpdatesUserType = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auutCurrentUser: AutomaticUpdatesUserType = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auutLocalAdministrator: AutomaticUpdatesUserType = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type DeploymentAction = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const daNone: DeploymentAction = 0i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const daInstallation: DeploymentAction = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const daUninstallation: DeploymentAction = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const daDetection: DeploymentAction = 3i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const daOptionalInstallation: DeploymentAction = 4i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type DownloadPhase = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dphInitializing: DownloadPhase = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dphDownloading: DownloadPhase = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dphVerifying: DownloadPhase = 3i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type DownloadPriority = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dpLow: DownloadPriority = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dpNormal: DownloadPriority = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dpHigh: DownloadPriority = 3i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dpExtraHigh: DownloadPriority = 4i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAutomaticUpdates {
    pub base__: super::Com::IDispatch,
    pub DetectNow: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ShowSettingsDialog: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Settings: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Settings: usize,
    pub ServiceEnabled: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub EnableService: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAutomaticUpdates {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1731470783, data2: 49282, data3: 19580, data4: [189, 253, 86, 148, 100, 184, 224, 206] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAutomaticUpdates2 {
    pub base__: IAutomaticUpdates,
    #[cfg(feature = "Win32_System_Com")]
    pub Results: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Results: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAutomaticUpdates2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1244617777, data2: 53209, data3: 16654, data4: [183, 251, 41, 166, 83, 151, 58, 15] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAutomaticUpdatesResults {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastSearchSuccessDate: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastSearchSuccessDate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastInstallationSuccessDate: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastInstallationSuccessDate: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAutomaticUpdatesResults {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3886339636, data2: 31042, data3: 19929, data4: [161, 17, 130, 34, 139, 163, 57, 1] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAutomaticUpdatesSettings {
    pub base__: super::Com::IDispatch,
    pub NotificationLevel: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut AutomaticUpdatesNotificationLevel) -> ::windows_sys::core::HRESULT,
    pub SetNotificationLevel: unsafe extern "system" fn(this: *mut *mut Self, value: AutomaticUpdatesNotificationLevel) -> ::windows_sys::core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Required: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub ScheduledInstallationDay: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut AutomaticUpdatesScheduledInstallationDay) -> ::windows_sys::core::HRESULT,
    pub SetScheduledInstallationDay: unsafe extern "system" fn(this: *mut *mut Self, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows_sys::core::HRESULT,
    pub ScheduledInstallationTime: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetScheduledInstallationTime: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAutomaticUpdatesSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 786730786, data2: 44860, data3: 16479, data4: [137, 112, 247, 27, 225, 46, 233, 162] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAutomaticUpdatesSettings2 {
    pub base__: IAutomaticUpdatesSettings,
    pub IncludeRecommendedUpdates: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIncludeRecommendedUpdates: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub CheckPermission: unsafe extern "system" fn(this: *mut *mut Self, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType, userhaspermission: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAutomaticUpdatesSettings2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1790710634, data2: 50122, data3: 17284, data4: [129, 113, 203, 43, 30, 89, 184, 220] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAutomaticUpdatesSettings3 {
    pub base__: IAutomaticUpdatesSettings2,
    pub NonAdministratorsElevated: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetNonAdministratorsElevated: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub FeaturedUpdatesEnabled: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetFeaturedUpdatesEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAutomaticUpdatesSettings3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3045586371, data2: 62846, data3: 18527, data4: [187, 245, 13, 24, 28, 92, 208, 220] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICategory {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CategoryID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CategoryID: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Children: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Image: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Image: usize,
    pub Order: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Type: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICategory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2178793912, data2: 40245, data3: 18342, data4: [180, 113, 91, 128, 245, 25, 34, 59] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICategoryCollection {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICategoryCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 978763704, data2: 22380, data3: 17399, data4: [147, 53, 254, 72, 56, 253, 126, 55] };
}
#[repr(C)]
pub struct IDownloadCompletedCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(this: *mut *mut Self, downloadjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
impl ::windows_sys::core::Interface for IDownloadCompletedCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1998932070, data2: 40795, data3: 19598, data4: [185, 226, 199, 122, 133, 48, 214, 75] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDownloadCompletedCallbackArgs {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDownloadCompletedCallbackArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4199963427, data2: 18828, data3: 18336, data4: [151, 157, 231, 213, 177, 129, 51, 96] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDownloadJob {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AsyncState: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AsyncState: usize,
    pub IsCompleted: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    pub CleanUp: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProgress: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProgress: usize,
    pub RequestAbort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDownloadJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3312770693, data2: 29528, data3: 17398, data4: [170, 232, 134, 151, 230, 45, 139, 167] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDownloadProgress {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentUpdateBytesDownloaded: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentUpdateBytesDownloaded: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentUpdateBytesToDownload: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentUpdateBytesToDownload: usize,
    pub CurrentUpdateIndex: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TotalBytesDownloaded: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TotalBytesDownloaded: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TotalBytesToDownload: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TotalBytesToDownload: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUpdateResult: unsafe extern "system" fn(this: *mut *mut Self, updateindex: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUpdateResult: usize,
    pub CurrentUpdateDownloadPhase: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut DownloadPhase) -> ::windows_sys::core::HRESULT,
    pub CurrentUpdatePercentComplete: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDownloadProgress {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3541719980, data2: 63257, data3: 16760, data4: [157, 187, 94, 44, 180, 127, 209, 138] };
}
#[repr(C)]
pub struct IDownloadProgressChangedCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(this: *mut *mut Self, downloadjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
impl ::windows_sys::core::Interface for IDownloadProgressChangedCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2352946397, data2: 24947, data3: 17809, data4: [174, 189, 165, 106, 83, 202, 119, 193] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDownloadProgressChangedCallbackArgs {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Progress: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDownloadProgressChangedCallbackArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 844100294, data2: 18817, data3: 19204, data4: [148, 18, 87, 72, 23, 69, 171, 36] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDownloadResult {
    pub base__: super::Com::IDispatch,
    pub HResult: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut OperationResultCode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUpdateResult: unsafe extern "system" fn(this: *mut *mut Self, updateindex: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUpdateResult: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDownloadResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3668245968, data2: 18215, data3: 19902, data4: [161, 231, 116, 93, 202, 49, 113, 68] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IImageInformation {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub AltText: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AltText: usize,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Source: usize,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IImageInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2089842788, data2: 13420, data3: 19179, data4: [143, 63, 87, 218, 40, 159, 150, 159] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationAgent {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RecordInstallationResult: unsafe extern "system" fn(this: *mut *mut Self, installationresultcookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hresult: i32, extendedreportingdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RecordInstallationResult: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInstallationAgent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2455551000, data2: 41706, data3: 17992, data4: [191, 28, 236, 139, 173, 207, 226, 10] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationBehavior {
    pub base__: super::Com::IDispatch,
    pub CanRequestUserInput: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Impact: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut InstallationImpact) -> ::windows_sys::core::HRESULT,
    pub RebootBehavior: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut InstallationRebootBehavior) -> ::windows_sys::core::HRESULT,
    pub RequiresNetworkConnectivity: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInstallationBehavior {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3651507001, data2: 57925, data3: 19901, data4: [150, 134, 77, 87, 99, 227, 150, 36] };
}
#[repr(C)]
pub struct IInstallationCompletedCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(this: *mut *mut Self, installationjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
impl ::windows_sys::core::Interface for IInstallationCompletedCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1173681907, data2: 54786, data3: 20376, data4: [154, 138, 62, 250, 21, 42, 210, 211] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationCompletedCallbackArgs {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInstallationCompletedCallbackArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 621682950, data2: 36603, data3: 18181, data4: [150, 83, 239, 19, 197, 129, 182, 161] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationJob {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AsyncState: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AsyncState: usize,
    pub IsCompleted: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    pub CleanUp: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProgress: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProgress: usize,
    pub RequestAbort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInstallationJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1545641739, data2: 47829, data3: 17194, data4: [149, 86, 70, 153, 190, 210, 99, 138] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationProgress {
    pub base__: super::Com::IDispatch,
    pub CurrentUpdateIndex: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CurrentUpdatePercentComplete: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUpdateResult: unsafe extern "system" fn(this: *mut *mut Self, updateindex: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUpdateResult: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInstallationProgress {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 878477892, data2: 17315, data3: 20018, data4: [163, 104, 101, 240, 115, 183, 111, 54] };
}
#[repr(C)]
pub struct IInstallationProgressChangedCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(this: *mut *mut Self, installationjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
impl ::windows_sys::core::Interface for IInstallationProgressChangedCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3759407829, data2: 63706, data3: 17338, data4: [160, 18, 56, 137, 75, 208, 72, 241] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationProgressChangedCallbackArgs {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Progress: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInstallationProgressChangedCallbackArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3841019422, data2: 26781, data3: 16920, data4: [160, 185, 188, 24, 156, 72, 74, 1] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationResult {
    pub base__: super::Com::IDispatch,
    pub HResult: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RebootRequired: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut OperationResultCode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUpdateResult: unsafe extern "system" fn(this: *mut *mut Self, updateindex: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUpdateResult: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInstallationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2755417814, data2: 29777, data3: 18644, data4: [175, 150, 182, 205, 45, 13, 155, 122] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInvalidProductLicenseException {
    pub base__: IUpdateException,
    #[cfg(feature = "Win32_Foundation")]
    pub Product: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Product: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInvalidProductLicenseException {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2742878453, data2: 31664, data3: 18771, data4: [180, 20, 249, 233, 131, 38, 242, 232] };
}
#[repr(C)]
pub struct ISearchCompletedCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(this: *mut *mut Self, searchjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
impl ::windows_sys::core::Interface for ISearchCompletedCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2293162072, data2: 54448, data3: 18213, data4: [162, 241, 129, 74, 103, 174, 150, 76] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISearchCompletedCallbackArgs {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ISearchCompletedCallbackArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2801837620, data2: 10320, data3: 19527, data4: [147, 138, 158, 75, 110, 90, 249, 166] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISearchJob {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AsyncState: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AsyncState: usize,
    pub IsCompleted: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub CleanUp: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RequestAbort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ISearchJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1936124438, data2: 31258, data3: 20130, data4: [176, 66, 151, 61, 62, 156, 217, 155] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISearchResult {
    pub base__: super::Com::IDispatch,
    pub ResultCode: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut OperationResultCode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RootCategories: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootCategories: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Warnings: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Warnings: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ISearchResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3557621602, data2: 57484, data3: 17560, data4: [148, 26, 1, 226, 95, 15, 211, 60] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IStringCollection {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Item: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Copy: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Insert: unsafe extern "system" fn(this: *mut *mut Self, index: i32, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Insert: usize,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IStringCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4026074498, data2: 11740, data3: 18447, data4: [160, 109, 96, 243, 251, 195, 98, 195] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISystemInformation {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub OemHardwareSupportLink: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OemHardwareSupportLink: usize,
    pub RebootRequired: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ISystemInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2917694455, data2: 31574, data3: 17013, data4: [143, 171, 185, 176, 229, 145, 132, 75] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdate {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Title: usize,
    pub AutoSelectOnWebSites: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub BundledUpdates: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BundledUpdates: usize,
    pub CanRequireSource: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Categories: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Categories: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Deadline: usize,
    pub DeltaCompressedContentAvailable: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub DeltaCompressedContentPreferred: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    pub EulaAccepted: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EulaText: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EulaText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HandlerID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HandlerID: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Identity: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Identity: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Image: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Image: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstallationBehavior: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstallationBehavior: usize,
    pub IsBeta: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsDownloaded: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIsHidden: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub IsInstalled: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsMandatory: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsUninstallable: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Languages: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Languages: usize,
    pub LastDeploymentChangeTime: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MaxDownloadSize: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MaxDownloadSize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MinDownloadSize: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MinDownloadSize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MoreInfoUrls: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MoreInfoUrls: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MsrcSeverity: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MsrcSeverity: usize,
    pub RecommendedCpuSpeed: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RecommendedHardDiskSpace: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RecommendedMemory: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseNotes: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseNotes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SecurityBulletinIDs: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SecurityBulletinIDs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupersededUpdateIDs: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupersededUpdateIDs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportUrl: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportUrl: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut UpdateType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UninstallationNotes: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UninstallationNotes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UninstallationBehavior: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UninstallationBehavior: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UninstallationSteps: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UninstallationSteps: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub KBArticleIDs: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    KBArticleIDs: usize,
    pub AcceptEula: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DeploymentAction: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut DeploymentAction) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyFromCache: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, toextractcabfiles: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyFromCache: usize,
    pub DownloadPriority: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut DownloadPriority) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DownloadContents: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DownloadContents: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1787998330, data2: 55329, data3: 18050, data4: [180, 35, 92, 128, 80, 34, 204, 77] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdate2 {
    pub base__: IUpdate,
    pub RebootRequired: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsPresent: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CveIDs: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CveIDs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyToCache: unsafe extern "system" fn(this: *mut *mut Self, pfiles: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyToCache: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdate2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 340781488, data2: 53821, data3: 19083, data4: [134, 52, 251, 68, 87, 83, 59, 122] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdate3 {
    pub base__: IUpdate2,
    pub BrowseOnly: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdate3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 288283243, data2: 38323, data3: 18287, data4: [157, 144, 174, 232, 44, 107, 129, 129] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdate4 {
    pub base__: IUpdate3,
    pub PerUser: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdate4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 669600525, data2: 20793, data3: 18850, data4: [154, 97, 147, 82, 45, 197, 70, 82] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdate5 {
    pub base__: IUpdate4,
    pub AutoSelection: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut AutoSelectionMode) -> ::windows_sys::core::HRESULT,
    pub AutoDownload: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut AutoDownloadMode) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdate5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3250778650, data2: 54004, data3: 18690, data4: [181, 198, 138, 8, 28, 25, 168, 144] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateCollection {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub put_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    put_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Copy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Insert: unsafe extern "system" fn(this: *mut *mut Self, index: i32, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Insert: usize,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 133645196, data2: 30473, data3: 19621, data4: [181, 24, 145, 39, 146, 136, 19, 78] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateDownloadContent {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub DownloadUrl: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DownloadUrl: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateDownloadContent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1419954989, data2: 39436, data3: 18614, data4: [138, 80, 154, 187, 105, 238, 45, 2] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateDownloadContent2 {
    pub base__: IUpdateDownloadContent,
    pub IsDeltaCompressedContent: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateDownloadContent2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3380269339, data2: 62039, data3: 16907, data4: [157, 159, 55, 127, 115, 63, 111, 104] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateDownloadContentCollection {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateDownloadContentCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3159692232, data2: 46008, data3: 19447, data4: [164, 212, 54, 28, 13, 140, 136, 186] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateDownloadResult {
    pub base__: super::Com::IDispatch,
    pub HResult: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut OperationResultCode) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateDownloadResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3214520182, data2: 46453, data3: 17069, data4: [138, 164, 51, 203, 181, 71, 122, 241] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateDownloader {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientApplicationID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientApplicationID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientApplicationID: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientApplicationID: usize,
    pub IsForced: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIsForced: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut DownloadPriority) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, value: DownloadPriority) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetUpdates: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetUpdates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BeginDownload: unsafe extern "system" fn(this: *mut *mut Self, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BeginDownload: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Download: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Download: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EndDownload: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EndDownload: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateDownloader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1760675577, data2: 32460, data3: 18022, data4: [164, 100, 36, 127, 225, 36, 150, 195] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateException {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Message: usize,
    pub HResult: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Context: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut UpdateExceptionContext) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateException {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2742476126, data2: 2516, data3: 17023, data4: [175, 124, 254, 213, 182, 225, 193, 214] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateExceptionCollection {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateExceptionCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1345726115, data2: 36372, data3: 18217, data4: [147, 85, 15, 230, 100, 189, 35, 33] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateHistoryEntry {
    pub base__: super::Com::IDispatch,
    pub Operation: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut UpdateOperation) -> ::windows_sys::core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut OperationResultCode) -> ::windows_sys::core::HRESULT,
    pub HResult: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateIdentity: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Title: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    pub UnmappedResultCode: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientApplicationID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientApplicationID: usize,
    pub ServerSelection: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut ServerSelection) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceID: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UninstallationSteps: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UninstallationSteps: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UninstallationNotes: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UninstallationNotes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportUrl: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportUrl: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateHistoryEntry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3193349700, data2: 44814, data3: 19982, data4: [163, 17, 193, 216, 230, 149, 203, 255] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateHistoryEntry2 {
    pub base__: IUpdateHistoryEntry,
    #[cfg(feature = "Win32_System_Com")]
    pub Categories: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Categories: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateHistoryEntry2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3267344256, data2: 17721, data3: 16690, data4: [171, 140, 10, 135, 114, 1, 58, 182] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateHistoryEntryCollection {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateHistoryEntryCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2817543996, data2: 41616, data3: 17243, data4: [170, 223, 161, 22, 195, 53, 122, 92] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateIdentity {
    pub base__: super::Com::IDispatch,
    pub RevisionNumber: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateID: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateIdentity {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1177122851, data2: 39232, data3: 19465, data4: [174, 217, 205, 62, 166, 208, 89, 104] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateInstallationResult {
    pub base__: super::Com::IDispatch,
    pub HResult: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RebootRequired: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut OperationResultCode) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateInstallationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3644911864, data2: 15547, data3: 20432, data4: [153, 63, 71, 30, 127, 35, 40, 173] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateInstaller {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientApplicationID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientApplicationID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientApplicationID: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientApplicationID: usize,
    pub IsForced: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIsForced: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ParentHwnd: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ParentHwnd: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetParentHwnd: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetParentHwnd: usize,
    pub SetParentWindow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetUpdates: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetUpdates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BeginInstall: unsafe extern "system" fn(this: *mut *mut Self, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BeginInstall: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BeginUninstall: unsafe extern "system" fn(this: *mut *mut Self, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BeginUninstall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EndInstall: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EndInstall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EndUninstall: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EndUninstall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Install: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Install: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RunWizard: unsafe extern "system" fn(this: *mut *mut Self, dialogtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RunWizard: usize,
    pub IsBusy: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Uninstall: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Uninstall: usize,
    pub AllowSourcePrompts: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowSourcePrompts: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub RebootRequiredBeforeInstallation: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateInstaller {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2073205864, data2: 52444, data3: 16934, data4: [150, 177, 135, 36, 96, 11, 84, 194] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateInstaller2 {
    pub base__: IUpdateInstaller,
    pub ForceQuiet: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetForceQuiet: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateInstaller2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 876795134, data2: 8781, data3: 19694, data4: [152, 207, 48, 224, 196, 210, 41, 230] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateInstaller3 {
    pub base__: IUpdateInstaller2,
    pub AttemptCloseAppsIfNecessary: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAttemptCloseAppsIfNecessary: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateInstaller3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 382802997, data2: 2458, data3: 18640, data4: [131, 56, 95, 174, 100, 4, 127, 142] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateInstaller4 {
    pub base__: IUpdateInstaller3,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateInstaller4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4018276586, data2: 8964, data3: 18733, data4: [145, 9, 35, 129, 59, 9, 88, 225] };
}
#[repr(C)]
pub struct IUpdateLockdown {
    pub base__: ::windows_sys::core::IUnknown,
    pub LockDown: unsafe extern "system" fn(this: *mut *mut Self, flags: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUpdateLockdown {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2843132557, data2: 30113, data3: 17066, data4: [148, 174, 138, 248, 184, 114, 8, 154] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateSearcher {
    pub base__: super::Com::IDispatch,
    pub CanAutomaticallyUpgradeService: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetCanAutomaticallyUpgradeService: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientApplicationID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientApplicationID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientApplicationID: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientApplicationID: usize,
    pub IncludePotentiallySupersededUpdates: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIncludePotentiallySupersededUpdates: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub ServerSelection: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut ServerSelection) -> ::windows_sys::core::HRESULT,
    pub SetServerSelection: unsafe extern "system" fn(this: *mut *mut Self, value: ServerSelection) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BeginSearch: unsafe extern "system" fn(this: *mut *mut Self, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BeginSearch: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EndSearch: unsafe extern "system" fn(this: *mut *mut Self, searchjob: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EndSearch: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EscapeString: unsafe extern "system" fn(this: *mut *mut Self, unescaped: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EscapeString: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryHistory: unsafe extern "system" fn(this: *mut *mut Self, startindex: i32, count: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryHistory: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Search: unsafe extern "system" fn(this: *mut *mut Self, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Search: usize,
    pub Online: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetOnline: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub GetTotalHistoryCount: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceID: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceID: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateSearcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2403707889, data2: 63918, data3: 19349, data4: [169, 51, 240, 246, 110, 80, 86, 234] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateSearcher2 {
    pub base__: IUpdateSearcher,
    pub IgnoreDownloadPriority: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIgnoreDownloadPriority: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateSearcher2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1287506733, data2: 5513, data3: 19435, data4: [189, 28, 62, 88, 47, 240, 173, 208] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateSearcher3 {
    pub base__: IUpdateSearcher2,
    pub SearchScope: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut SearchScope) -> ::windows_sys::core::HRESULT,
    pub SetSearchScope: unsafe extern "system" fn(this: *mut *mut Self, value: SearchScope) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateSearcher3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 80120157, data2: 60146, data3: 16436, data4: [151, 243, 49, 29, 233, 190, 65, 58] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateService {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ContentValidationCert: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ContentValidationCert: usize,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut f64) -> ::windows_sys::core::HRESULT,
    pub IsManaged: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsRegisteredWithAU: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IssueDate: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OffersWindowsUpdates: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RedirectUrls: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RedirectUrls: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceID: usize,
    pub IsScanPackageService: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub CanRegisterWithAU: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceUrl: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceUrl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetupPrefix: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetupPrefix: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1991487870, data2: 44758, data3: 19877, data4: [133, 240, 131, 88, 127, 129, 171, 227] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateService2 {
    pub base__: IUpdateService,
    pub IsDefaultAUService: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateService2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 353940576, data2: 25880, data3: 16754, data4: [148, 15, 199, 88, 131, 178, 76, 235] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateServiceCollection {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateServiceCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2600686506, data2: 3666, data3: 17663, data4: [184, 176, 31, 127, 160, 67, 127, 136] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateServiceManager {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Services: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Services: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddService: unsafe extern "system" fn(this: *mut *mut Self, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorizationcabpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddService: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterServiceWithAU: unsafe extern "system" fn(this: *mut *mut Self, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterServiceWithAU: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveService: unsafe extern "system" fn(this: *mut *mut Self, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveService: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterServiceWithAU: unsafe extern "system" fn(this: *mut *mut Self, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterServiceWithAU: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddScanPackageService: unsafe extern "system" fn(this: *mut *mut Self, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scanfilelocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddScanPackageService: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetOption: unsafe extern "system" fn(this: *mut *mut Self, optionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetOption: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateServiceManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 595951164, data2: 698, data3: 17571, data4: [148, 35, 177, 201, 0, 128, 95, 55] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateServiceManager2 {
    pub base__: IUpdateServiceManager,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientApplicationID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientApplicationID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientApplicationID: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientApplicationID: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryServiceRegistration: unsafe extern "system" fn(this: *mut *mut Self, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryServiceRegistration: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddService2: unsafe extern "system" fn(this: *mut *mut Self, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, authorizationcabpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddService2: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateServiceManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 196629277, data2: 32397, data3: 16975, data4: [152, 108, 160, 184, 246, 10, 62, 123] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateServiceRegistration {
    pub base__: super::Com::IDispatch,
    pub RegistrationState: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut UpdateServiceRegistrationState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceID: usize,
    pub IsPendingRegistrationWithAU: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Service: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Service: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateServiceRegistration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3722453632, data2: 4787, data3: 19979, data4: [147, 123, 103, 71, 246, 172, 178, 134] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateSession {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientApplicationID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientApplicationID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientApplicationID: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientApplicationID: usize,
    pub ReadOnly: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WebProxy: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WebProxy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWebProxy: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWebProxy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateUpdateSearcher: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateUpdateSearcher: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateUpdateDownloader: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateUpdateDownloader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateUpdateInstaller: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateUpdateInstaller: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2171099300, data2: 9741, data3: 16992, data4: [147, 58, 37, 133, 241, 171, 199, 107] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateSession2 {
    pub base__: IUpdateSession,
    pub UserLocale: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetUserLocale: unsafe extern "system" fn(this: *mut *mut Self, lcid: u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateSession2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2445998000, data2: 60195, data3: 18925, data4: [153, 55, 197, 45, 129, 127, 70, 247] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateSession3 {
    pub base__: IUpdateSession2,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateUpdateServiceManager: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateUpdateServiceManager: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryHistory: unsafe extern "system" fn(this: *mut *mut Self, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, startindex: i32, count: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryHistory: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUpdateSession3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2442067230, data2: 46552, data3: 19600, data4: [133, 64, 174, 185, 189, 197, 111, 157] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWebProxy {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Address: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAddress: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BypassList: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BypassList: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBypassList: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBypassList: usize,
    pub BypassProxyOnLocal: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetBypassProxyOnLocal: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UserName: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPassword: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPassword: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PromptForCredentials: unsafe extern "system" fn(this: *mut *mut Self, parentwindow: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PromptForCredentials: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PromptForCredentialsFromHwnd: unsafe extern "system" fn(this: *mut *mut Self, parentwindow: super::super::Foundation::HWND, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PromptForCredentialsFromHwnd: usize,
    pub AutoDetect: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAutoDetect: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWebProxy {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 390889982, data2: 44749, data3: 19886, data4: [184, 160, 44, 99, 24, 221, 134, 168] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdate {
    pub base__: IUpdate,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverClass: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverClass: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverHardwareID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverHardwareID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverManufacturer: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverManufacturer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverModel: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverModel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverProvider: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverProvider: usize,
    pub DriverVerDate: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut f64) -> ::windows_sys::core::HRESULT,
    pub DeviceProblemNumber: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DeviceStatus: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWindowsDriverUpdate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3011759386, data2: 23785, data3: 17668, data4: [159, 99, 118, 75, 18, 54, 241, 145] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdate2 {
    pub base__: IWindowsDriverUpdate,
    pub RebootRequired: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsPresent: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CveIDs: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CveIDs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyToCache: unsafe extern "system" fn(this: *mut *mut Self, pfiles: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyToCache: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWindowsDriverUpdate2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1633436265, data2: 31304, data3: 17341, data4: [150, 183, 191, 108, 162, 125, 108, 62] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdate3 {
    pub base__: IWindowsDriverUpdate2,
    pub BrowseOnly: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWindowsDriverUpdate3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1240192258, data2: 19094, data3: 16829, data4: [158, 62, 76, 80, 87, 244, 37, 12] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdate4 {
    pub base__: IWindowsDriverUpdate3,
    #[cfg(feature = "Win32_System_Com")]
    pub WindowsDriverUpdateEntries: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WindowsDriverUpdateEntries: usize,
    pub PerUser: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWindowsDriverUpdate4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 5007915, data2: 3097, data3: 19561, data4: [159, 92, 162, 105, 178, 86, 13, 185] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdate5 {
    pub base__: IWindowsDriverUpdate4,
    pub AutoSelection: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut AutoSelectionMode) -> ::windows_sys::core::HRESULT,
    pub AutoDownload: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut AutoDownloadMode) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWindowsDriverUpdate5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1892637826, data2: 34370, data3: 17083, data4: [157, 188, 12, 253, 38, 60, 108, 79] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdateEntry {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverClass: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverClass: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverHardwareID: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverHardwareID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverManufacturer: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverManufacturer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverModel: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverModel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverProvider: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverProvider: usize,
    pub DriverVerDate: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut f64) -> ::windows_sys::core::HRESULT,
    pub DeviceProblemNumber: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DeviceStatus: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWindowsDriverUpdateEntry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3985374784, data2: 42507, data3: 17130, data4: [150, 82, 129, 125, 252, 250, 35, 236] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdateEntryCollection {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWindowsDriverUpdateEntryCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 223483648, data2: 41842, data3: 19439, data4: [130, 139, 61, 0, 193, 10, 222, 189] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsUpdateAgentInfo {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetInfo: unsafe extern "system" fn(this: *mut *mut Self, varinfoidentifier: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetInfo: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWindowsUpdateAgentInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2238791585, data2: 30614, data3: 20386, data4: [190, 59, 226, 214, 18, 77, 211, 115] };
}
pub const InstallationAgent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 830378748, data2: 5753, data3: 18173, data4: [160, 181, 240, 137, 20, 221, 134, 35] };
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type InstallationImpact = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const iiNormal: InstallationImpact = 0i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const iiMinor: InstallationImpact = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const iiRequiresExclusiveHandling: InstallationImpact = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type InstallationRebootBehavior = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const irbNeverReboots: InstallationRebootBehavior = 0i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const irbAlwaysRequiresReboot: InstallationRebootBehavior = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const irbCanRequestReboot: InstallationRebootBehavior = 2i32;
pub const LIBID_WUApiLib: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3046558879, data2: 22245, data3: 16798, data4: [166, 34, 224, 27, 180, 87, 67, 30] };
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type OperationResultCode = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const orcNotStarted: OperationResultCode = 0i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const orcInProgress: OperationResultCode = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const orcSucceeded: OperationResultCode = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const orcSucceededWithErrors: OperationResultCode = 3i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const orcFailed: OperationResultCode = 4i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const orcAborted: OperationResultCode = 5i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type SearchScope = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const searchScopeDefault: SearchScope = 0i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const searchScopeMachineOnly: SearchScope = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const searchScopeCurrentUserOnly: SearchScope = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const searchScopeMachineAndCurrentUser: SearchScope = 3i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const searchScopeMachineAndAllUsers: SearchScope = 4i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const searchScopeAllUsers: SearchScope = 5i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type ServerSelection = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ssDefault: ServerSelection = 0i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ssManagedServer: ServerSelection = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ssWindowsUpdate: ServerSelection = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ssOthers: ServerSelection = 3i32;
pub const StringCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1925807476, data2: 31803, data3: 16558, data4: [183, 125, 171, 219, 34, 235, 166, 251] };
pub const SystemInformation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3223034784, data2: 48807, data3: 16826, data4: [182, 4, 208, 163, 111, 70, 145, 51] };
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const UPDATE_LOCKDOWN_WEBSITE_ACCESS: u32 = 1u32;
pub const UpdateCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 325293155, data2: 219, data3: 17990, data4: [128, 61, 82, 128, 38, 20, 13, 136] };
pub const UpdateDownloader: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1538221386, data2: 23047, data3: 16996, data4: [162, 85, 159, 245, 76, 113, 81, 231] };
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type UpdateExceptionContext = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uecGeneral: UpdateExceptionContext = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uecWindowsDriver: UpdateExceptionContext = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uecWindowsInstaller: UpdateExceptionContext = 3i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uecSearchIncomplete: UpdateExceptionContext = 4i32;
pub const UpdateInstaller: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3537960575, data2: 53822, data3: 18657, data4: [147, 192, 111, 168, 204, 52, 100, 116] };
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type UpdateLockdownOption = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uloForWebsiteAccess: UpdateLockdownOption = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type UpdateOperation = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uoInstallation: UpdateOperation = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uoUninstallation: UpdateOperation = 2i32;
pub const UpdateSearcher: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3063539176, data2: 26623, data3: 16759, data4: [136, 176, 54, 132, 163, 56, 139, 251] };
pub const UpdateServiceManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4174533593, data2: 35236, data3: 19882, data4: [135, 182, 17, 104, 54, 159, 11, 33] };
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type UpdateServiceOption = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const usoNonVolatileService: UpdateServiceOption = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type UpdateServiceRegistrationState = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const usrsNotRegistered: UpdateServiceRegistrationState = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const usrsRegistrationPending: UpdateServiceRegistrationState = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const usrsRegistered: UpdateServiceRegistrationState = 3i32;
pub const UpdateSession: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1286880639, data2: 32494, data3: 18694, data4: [134, 152, 96, 218, 28, 56, 242, 254] };
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub type UpdateType = i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const utSoftware: UpdateType = 1i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const utDriver: UpdateType = 2i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_ALL_UPDATES_FAILED: ::windows_sys::core::HRESULT = -2145124318i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AUCLIENT_UNEXPECTED: ::windows_sys::core::HRESULT = -2145107969i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_CALL_CANCELLED: ::windows_sys::core::HRESULT = -2145124267i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_DETECT_SVCID_MISMATCH: ::windows_sys::core::HRESULT = -2145083386i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_LEGACYCLIENTDISABLED: ::windows_sys::core::HRESULT = -2145083389i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_NONLEGACYSERVER: ::windows_sys::core::HRESULT = -2145083390i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_NOSERVICE: ::windows_sys::core::HRESULT = -2145083392i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_NO_REGISTERED_SERVICE: ::windows_sys::core::HRESULT = -2145083387i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_OOBE_IN_PROGRESS: ::windows_sys::core::HRESULT = -2145083384i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_PAUSED: ::windows_sys::core::HRESULT = -2145083388i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_UNEXPECTED: ::windows_sys::core::HRESULT = -2145079297i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_BAD_FILE_URL: ::windows_sys::core::HRESULT = -2145124282i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_BAD_XML_HARDWARECAPABILITY: ::windows_sys::core::HRESULT = -2145079038i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_BIN_SOURCE_ABSENT: ::windows_sys::core::HRESULT = -2145124308i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CALLBACK_COOKIE_NOT_FOUND: ::windows_sys::core::HRESULT = -2145062907i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CALL_CANCELLED: ::windows_sys::core::HRESULT = -2145124341i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CALL_CANCELLED_BY_HIDE: ::windows_sys::core::HRESULT = -2145124262i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CALL_CANCELLED_BY_INTERACTIVE_SEARCH: ::windows_sys::core::HRESULT = -2145124253i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CALL_CANCELLED_BY_INVALID: ::windows_sys::core::HRESULT = -2145124261i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CALL_CANCELLED_BY_POLICY: ::windows_sys::core::HRESULT = -2145124305i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_COULDNOTCANCEL: ::windows_sys::core::HRESULT = -2145124342i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CYCLE_DETECTED: ::windows_sys::core::HRESULT = -2145124337i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_BG_ERROR_TOKEN_REQUIRED: ::windows_sys::core::HRESULT = -2145099761i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_BITSTRANSFERERROR: ::windows_sys::core::HRESULT = -2145099767i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_CONTENTCHANGED: ::windows_sys::core::HRESULT = -2145099765i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOSVC_REQUIRED: ::windows_sys::core::HRESULT = -2145099746i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOWNLOADFILEMISSING: ::windows_sys::core::HRESULT = -2145099758i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOWNLOADFILEPATHUNKNOWN: ::windows_sys::core::HRESULT = -2145099759i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOWNLOADLIMITEDBYUPDATESIZE: ::windows_sys::core::HRESULT = -2145099764i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOWNLOADLOCATIONCHANGED: ::windows_sys::core::HRESULT = -2145099766i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOWNLOADSANDBOXNOTFOUND: ::windows_sys::core::HRESULT = -2145099760i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOWNLOAD_VOLUME_CONFLICT: ::windows_sys::core::HRESULT = -2145099749i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_FAILTOCONNECTTOBITS: ::windows_sys::core::HRESULT = -2145099768i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_FALLINGBACKTOBITS: ::windows_sys::core::HRESULT = -2145099750i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_HARDRESERVEID_CONFLICT: ::windows_sys::core::HRESULT = -2145099747i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_INCORRECTFILEHASH: ::windows_sys::core::HRESULT = -2145099774i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_NEEDDOWNLOADREQUEST: ::windows_sys::core::HRESULT = -2145099772i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_NONETWORK: ::windows_sys::core::HRESULT = -2145099771i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_NOTDOWNLOADED: ::windows_sys::core::HRESULT = -2145099769i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_READRANGEFAILED: ::windows_sys::core::HRESULT = -2145099756i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_SANDBOX_HASH_MISMATCH: ::windows_sys::core::HRESULT = -2145099748i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNAUTHORIZED: ::windows_sys::core::HRESULT = -2145099762i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNAUTHORIZED_DOMAIN_USER: ::windows_sys::core::HRESULT = -2145099752i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNAUTHORIZED_LOCAL_USER: ::windows_sys::core::HRESULT = -2145099753i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNAUTHORIZED_MSA_USER: ::windows_sys::core::HRESULT = -2145099751i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNAUTHORIZED_NO_USER: ::windows_sys::core::HRESULT = -2145099754i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNEXPECTED: ::windows_sys::core::HRESULT = -2145095681i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNKNOWNALGORITHM: ::windows_sys::core::HRESULT = -2145099773i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UPDATEREMOVED: ::windows_sys::core::HRESULT = -2145099757i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_URLNOTAVAILABLE: ::windows_sys::core::HRESULT = -2145099775i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_WRONGBITSVERSION: ::windows_sys::core::HRESULT = -2145099770i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DOWNLOAD_FAILED: ::windows_sys::core::HRESULT = -2145124300i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_DEVICE_PROBLEM: ::windows_sys::core::HRESULT = -2145075192i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_MISSING_ATTRIBUTE: ::windows_sys::core::HRESULT = -2145075195i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_NOPROP_OR_LEGACY: ::windows_sys::core::HRESULT = -2145075198i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_NO_METADATA: ::windows_sys::core::HRESULT = -2145075196i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_NO_PRINTER_CONTENT: ::windows_sys::core::HRESULT = -2145075193i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_PRUNED: ::windows_sys::core::HRESULT = -2145075199i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_REG_MISMATCH: ::windows_sys::core::HRESULT = -2145075197i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_SYNC_FAILED: ::windows_sys::core::HRESULT = -2145075194i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_UNEXPECTED: ::windows_sys::core::HRESULT = -2145071105i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_BADVERSION: ::windows_sys::core::HRESULT = -2145091578i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_CANNOTREGISTER: ::windows_sys::core::HRESULT = -2145091568i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_CANTDELETE: ::windows_sys::core::HRESULT = -2145091573i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_DATANOTAVAILABLE: ::windows_sys::core::HRESULT = -2145091554i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_DATANOTLOADED: ::windows_sys::core::HRESULT = -2145091553i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_DECLINENOTALLOWED: ::windows_sys::core::HRESULT = -2145091562i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_DUPLICATEUPDATEID: ::windows_sys::core::HRESULT = -2145091565i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_IMPERSONATED: ::windows_sys::core::HRESULT = -2145091555i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_INUSE: ::windows_sys::core::HRESULT = -2145091583i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_INVALID: ::windows_sys::core::HRESULT = -2145091582i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_INVALIDOPERATION: ::windows_sys::core::HRESULT = -2145091558i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_INVALIDTABLENAME: ::windows_sys::core::HRESULT = -2145091579i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_LOCKTIMEOUTEXPIRED: ::windows_sys::core::HRESULT = -2145091572i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_MISSINGDATA: ::windows_sys::core::HRESULT = -2145091576i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_MISSINGREF: ::windows_sys::core::HRESULT = -2145091575i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NEEDWINDOWSSERVICE: ::windows_sys::core::HRESULT = -2145091559i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NOCATEGORIES: ::windows_sys::core::HRESULT = -2145091571i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA: ::windows_sys::core::HRESULT = -2145091577i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_CCR: ::windows_sys::core::HRESULT = -2145091546i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_COOKIE: ::windows_sys::core::HRESULT = -2145091548i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_DOWNLOADJOB: ::windows_sys::core::HRESULT = -2145091544i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_EULA: ::windows_sys::core::HRESULT = -2145091550i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_FILE: ::windows_sys::core::HRESULT = -2145091545i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_NOSUCHREVISION: ::windows_sys::core::HRESULT = -2145091552i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_NOSUCHUPDATE: ::windows_sys::core::HRESULT = -2145091551i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_SERVICE: ::windows_sys::core::HRESULT = -2145091549i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_TIMER: ::windows_sys::core::HRESULT = -2145091547i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_TMI: ::windows_sys::core::HRESULT = -2145091543i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_RESETREQUIRED: ::windows_sys::core::HRESULT = -2145091556i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_ROWEXISTS: ::windows_sys::core::HRESULT = -2145091570i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_SCHEMAMISMATCH: ::windows_sys::core::HRESULT = -2145091557i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_SERVICEEXPIRED: ::windows_sys::core::HRESULT = -2145091563i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_SESSIONLOCKMISMATCH: ::windows_sys::core::HRESULT = -2145091560i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_SHUTDOWN: ::windows_sys::core::HRESULT = -2145091584i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_STOREFILELOCKED: ::windows_sys::core::HRESULT = -2145091569i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_TABLEINCORRECT: ::windows_sys::core::HRESULT = -2145091580i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_TABLEMISSING: ::windows_sys::core::HRESULT = -2145091581i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_TABLESESSIONMISMATCH: ::windows_sys::core::HRESULT = -2145091561i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_UNABLETOSTART: ::windows_sys::core::HRESULT = -2145091567i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_UNEXPECTED: ::windows_sys::core::HRESULT = -2145087489i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_UNKNOWNHANDLER: ::windows_sys::core::HRESULT = -2145091574i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_UNKNOWNSERVICE: ::windows_sys::core::HRESULT = -2145091564i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DUPLICATE_ITEM: ::windows_sys::core::HRESULT = -2145124333i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_CLUSTER_ERROR: ::windows_sys::core::HRESULT = -2145067001i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_INVALID_ATTRIBUTEDATA: ::windows_sys::core::HRESULT = -2145067002i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_INVALID_EXPRESSION: ::windows_sys::core::HRESULT = -2145067006i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_INVALID_VERSION: ::windows_sys::core::HRESULT = -2145067004i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_MISSING_METADATA: ::windows_sys::core::HRESULT = -2145067005i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2145067003i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_UNEXPECTED: ::windows_sys::core::HRESULT = -2145062913i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_UNKNOWN_EXPRESSION: ::windows_sys::core::HRESULT = -2145067007i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EULAS_DECLINED: ::windows_sys::core::HRESULT = -2145124317i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EULA_UNAVAILABLE: ::windows_sys::core::HRESULT = -2145124301i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EXCLUSIVE_INSTALL_CONFLICT: ::windows_sys::core::HRESULT = -2145124327i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EXTENDEDERROR_FAILED: ::windows_sys::core::HRESULT = -2145124257i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EXTENDEDERROR_NOTSET: ::windows_sys::core::HRESULT = -2145124258i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_FILETRUST_DUALSIGNATURE_ECC: ::windows_sys::core::HRESULT = -2145078526i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_FILETRUST_DUALSIGNATURE_RSA: ::windows_sys::core::HRESULT = -2145078527i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_FILETRUST_SHA2SIGNATURE_MISSING: ::windows_sys::core::HRESULT = -2145124255i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_IDLESHUTDOWN_OPCOUNT_DISCOVERY: ::windows_sys::core::HRESULT = -2145124273i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_IDLESHUTDOWN_OPCOUNT_DOWNLOAD: ::windows_sys::core::HRESULT = -2145124271i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_IDLESHUTDOWN_OPCOUNT_INSTALL: ::windows_sys::core::HRESULT = -2145124270i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_IDLESHUTDOWN_OPCOUNT_OTHER: ::windows_sys::core::HRESULT = -2145124269i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_IDLESHUTDOWN_OPCOUNT_SEARCH: ::windows_sys::core::HRESULT = -2145124272i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_IDLESHUTDOWN_OPCOUNT_SERVICEREGISTRATION: ::windows_sys::core::HRESULT = -2145124256i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INFRASTRUCTUREFILE_INVALID_FORMAT: ::windows_sys::core::HRESULT = -2145124275i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INFRASTRUCTUREFILE_REQUIRES_SSL: ::windows_sys::core::HRESULT = -2145124274i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALLATION_RESULTS_INVALID_DATA: ::windows_sys::core::HRESULT = -2145112062i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALLATION_RESULTS_NOT_FOUND: ::windows_sys::core::HRESULT = -2145112061i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALLATION_RESULTS_UNKNOWN_VERSION: ::windows_sys::core::HRESULT = -2145112063i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALL_JOB_NOT_SUSPENDED: ::windows_sys::core::HRESULT = -2145124251i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALL_JOB_RESUME_NOT_ALLOWED: ::windows_sys::core::HRESULT = -2145124252i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALL_NOT_ALLOWED: ::windows_sys::core::HRESULT = -2145124330i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALL_USERCONTEXT_ACCESSDENIED: ::windows_sys::core::HRESULT = -2145124250i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INTERACTIVE_CALL_CANCELLED: ::windows_sys::core::HRESULT = -2145124268i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALIDINDEX: ::windows_sys::core::HRESULT = -2145124345i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_CRITERIA: ::windows_sys::core::HRESULT = -2145124302i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_EVENT: ::windows_sys::core::HRESULT = -2145062909i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_EVENT_PAYLOAD: ::windows_sys::core::HRESULT = -2145095677i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_EVENT_PAYLOADSIZE: ::windows_sys::core::HRESULT = -2145095676i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_FILE: ::windows_sys::core::HRESULT = -2145124303i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_INSTALL_REQUESTED: ::windows_sys::core::HRESULT = -2145124332i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_NOTIFICATION_INFO: ::windows_sys::core::HRESULT = -2145124280i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_OPERATION: ::windows_sys::core::HRESULT = -2145124298i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_PRODUCT_LICENSE: ::windows_sys::core::HRESULT = -2145124311i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_PROXY_SERVER: ::windows_sys::core::HRESULT = -2145124304i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_RELATIONSHIP: ::windows_sys::core::HRESULT = -2145124335i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_SERIALIZATION_VERSION: ::windows_sys::core::HRESULT = -2145124264i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_UPDATE: ::windows_sys::core::HRESULT = -2145124323i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_UPDATE_TYPE: ::windows_sys::core::HRESULT = -2145124314i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_VOLUMEID: ::windows_sys::core::HRESULT = -2145124260i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVENTORY_GET_INVENTORY_TYPE_FAILED: ::windows_sys::core::HRESULT = -2145087486i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVENTORY_PARSEFAILED: ::windows_sys::core::HRESULT = -2145087487i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVENTORY_RESULT_UPLOAD_FAILED: ::windows_sys::core::HRESULT = -2145087485i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVENTORY_UNEXPECTED: ::windows_sys::core::HRESULT = -2145087484i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVENTORY_WMI_ERROR: ::windows_sys::core::HRESULT = -2145087483i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_ITEMNOTFOUND: ::windows_sys::core::HRESULT = -2145124344i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_LEGACYSERVER: ::windows_sys::core::HRESULT = -2145124309i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_LOW_BATTERY: ::windows_sys::core::HRESULT = -2145124276i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MAX_CAPACITY_REACHED: ::windows_sys::core::HRESULT = -2145124350i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATATRUST_CERTIFICATECHAIN_VERIFICATION: ::windows_sys::core::HRESULT = -2145095344i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATATRUST_UNTRUSTED_CERTIFICATECHAIN: ::windows_sys::core::HRESULT = -2145095343i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_BAD_FRAGMENTSIGNING_CONFIG: ::windows_sys::core::HRESULT = -2145095417i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_BAD_SIGNATURE: ::windows_sys::core::HRESULT = -2145095360i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_CERT_MISSING: ::windows_sys::core::HRESULT = -2145095296i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_CERT_UNTRUSTED: ::windows_sys::core::HRESULT = -2145095293i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_CONFIG_INVALID_BINARY_ENCODING: ::windows_sys::core::HRESULT = -2145095423i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_FAILURE_PROCESSING_FRAGMENTSIGNING_CONFIG: ::windows_sys::core::HRESULT = -2145095416i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_FETCH_CONFIG: ::windows_sys::core::HRESULT = -2145095422i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_INTCERT_BAD_TRANSPORT_ENCODING: ::windows_sys::core::HRESULT = -2145095294i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_INVALID_PARAMETER: ::windows_sys::core::HRESULT = -2145095420i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_LEAFCERT_BAD_TRANSPORT_ENCODING: ::windows_sys::core::HRESULT = -2145095295i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_NOOP: ::windows_sys::core::HRESULT = -2145095424i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_NO_VERIFICATION_DATA: ::windows_sys::core::HRESULT = -2145095418i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_SIGNATURE_VERIFY_FAILED: ::windows_sys::core::HRESULT = -2145095358i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_ALL_BAD: ::windows_sys::core::HRESULT = -2145095321i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_CACHELOOKUP: ::windows_sys::core::HRESULT = -2145095319i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_CERTCHAIN: ::windows_sys::core::HRESULT = -2145095323i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_MISSING: ::windows_sys::core::HRESULT = -2145095328i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_NODATA: ::windows_sys::core::HRESULT = -2145095320i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_REFRESHONLINE: ::windows_sys::core::HRESULT = -2145095322i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_SIGNATURE: ::windows_sys::core::HRESULT = -2145095324i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_UNEXPECTED: ::windows_sys::core::HRESULT = -2145095297i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_UNTRUSTED: ::windows_sys::core::HRESULT = -2145095326i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_VALIDITYWINDOW_UNEXPECTED: ::windows_sys::core::HRESULT = -2145095298i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_VALIDITY_WINDOW: ::windows_sys::core::HRESULT = -2145095325i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_VERIFICATION_FAILED: ::windows_sys::core::HRESULT = -2145095327i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_UNEXPECTED: ::windows_sys::core::HRESULT = -2145095419i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_UNSUPPORTED_HASH_ALG: ::windows_sys::core::HRESULT = -2145095359i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_BASE64CERDATA_MISSING: ::windows_sys::core::HRESULT = -2145095384i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_FRAGMENTSIGNING_MISSING: ::windows_sys::core::HRESULT = -2145095391i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_INTERMEDIATECERT_MISSING: ::windows_sys::core::HRESULT = -2145095386i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_LEAFCERT_ID_MISSING: ::windows_sys::core::HRESULT = -2145095385i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_LEAFCERT_MISSING: ::windows_sys::core::HRESULT = -2145095387i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_MISSING: ::windows_sys::core::HRESULT = -2145095392i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_MODE_INVALID: ::windows_sys::core::HRESULT = -2145095389i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_MODE_MISSING: ::windows_sys::core::HRESULT = -2145095390i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_VALIDITY_INVALID: ::windows_sys::core::HRESULT = -2145095388i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MISSING_HANDLER: ::windows_sys::core::HRESULT = -2145124310i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MSI_NOT_CONFIGURED: ::windows_sys::core::HRESULT = -2145120254i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MSI_NOT_PRESENT: ::windows_sys::core::HRESULT = -2145120251i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MSI_WRONG_APP_CONTEXT: ::windows_sys::core::HRESULT = -2145120252i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MSI_WRONG_VERSION: ::windows_sys::core::HRESULT = -2145120255i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MSP_DISABLED: ::windows_sys::core::HRESULT = -2145120253i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MSP_UNEXPECTED: ::windows_sys::core::HRESULT = -2145116161i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NETWORK_COST_EXCEEDS_POLICY: ::windows_sys::core::HRESULT = -2145124263i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NON_UI_MODE: ::windows_sys::core::HRESULT = -2145107971i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NOOP: ::windows_sys::core::HRESULT = -2145124340i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NOT_APPLICABLE: ::windows_sys::core::HRESULT = -2145124329i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2145124348i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2145124297i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_CONNECTION: ::windows_sys::core::HRESULT = -2145124321i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_INTERACTIVE_USER: ::windows_sys::core::HRESULT = -2145124320i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_SERVER_CORE_SUPPORT: ::windows_sys::core::HRESULT = -2145124288i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_SERVICE: ::windows_sys::core::HRESULT = -2145124351i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_SUCH_HANDLER_PLUGIN: ::windows_sys::core::HRESULT = -2145124265i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_UI_SUPPORT: ::windows_sys::core::HRESULT = -2145124285i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_UPDATE: ::windows_sys::core::HRESULT = -2145124316i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_USERTOKEN: ::windows_sys::core::HRESULT = -2145124328i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_OL_INVALID_SCANFILE: ::windows_sys::core::HRESULT = -2145095679i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_OL_NEWCLIENT_REQUIRED: ::windows_sys::core::HRESULT = -2145095678i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_OL_UNEXPECTED: ::windows_sys::core::HRESULT = -2145091585i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_OPERATIONINPROGRESS: ::windows_sys::core::HRESULT = -2145124343i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_ORPHANED_DOWNLOAD_JOB: ::windows_sys::core::HRESULT = -2145124277i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_OUTOFRANGE: ::windows_sys::core::HRESULT = -2145124279i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PER_MACHINE_UPDATE_ACCESS_DENIED: ::windows_sys::core::HRESULT = -2145124284i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_POLICY_NOT_SET: ::windows_sys::core::HRESULT = -2145124326i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ADDRESS_IN_USE: ::windows_sys::core::HRESULT = -2145123256i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ADDRESS_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -2145123255i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_CATALOG_SYNC_REQUIRED: ::windows_sys::core::HRESULT = -2145123274i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_CONFIG_PROP_MISSING: ::windows_sys::core::HRESULT = -2145107926i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_DOUBLE_INITIALIZATION: ::windows_sys::core::HRESULT = -2145107950i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_FAILURE_TO_DECOMPRESS_CAB_FILE: ::windows_sys::core::HRESULT = -2145107916i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_FAILURE_TO_EXTRACT_DIGEST: ::windows_sys::core::HRESULT = -2145107917i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_FILE_LOCATION_ERROR: ::windows_sys::core::HRESULT = -2145107915i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_INIT_FAILED: ::windows_sys::core::HRESULT = -2145107920i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_INVALID_FILE_FORMAT: ::windows_sys::core::HRESULT = -2145107919i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_INVALID_METADATA: ::windows_sys::core::HRESULT = -2145107918i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_SUCCEEDED_WITH_ERRORS: ::windows_sys::core::HRESULT = -2145107921i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ENDPOINTURL_NOTAVAIL: ::windows_sys::core::HRESULT = -2145123265i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ENDPOINT_DISCONNECTED: ::windows_sys::core::HRESULT = -2145123264i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ENDPOINT_REFRESH_REQUIRED: ::windows_sys::core::HRESULT = -2145123266i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ENDPOINT_UNREACHABLE: ::windows_sys::core::HRESULT = -2145123272i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_EXCEEDED_MAX_SERVER_TRIPS: ::windows_sys::core::HRESULT = -2145107952i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_FILE_LOCATIONS_CHANGED: ::windows_sys::core::HRESULT = -2145107931i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_BAD_GATEWAY: ::windows_sys::core::HRESULT = -2145107935i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_BAD_METHOD: ::windows_sys::core::HRESULT = -2145107942i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_BAD_REQUEST: ::windows_sys::core::HRESULT = -2145107946i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_CONFLICT: ::windows_sys::core::HRESULT = -2145107939i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_DENIED: ::windows_sys::core::HRESULT = -2145107945i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_FORBIDDEN: ::windows_sys::core::HRESULT = -2145107944i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_GATEWAY_TIMEOUT: ::windows_sys::core::HRESULT = -2145107933i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_GONE: ::windows_sys::core::HRESULT = -2145107938i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_NOT_FOUND: ::windows_sys::core::HRESULT = -2145107943i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_NOT_MAPPED: ::windows_sys::core::HRESULT = -2145107925i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2145107936i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_PROXY_AUTH_REQ: ::windows_sys::core::HRESULT = -2145107941i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_REQUEST_TIMEOUT: ::windows_sys::core::HRESULT = -2145107940i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_SERVER_ERROR: ::windows_sys::core::HRESULT = -2145107937i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_SERVICE_UNAVAIL: ::windows_sys::core::HRESULT = -2145107934i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_VERSION_NOT_SUP: ::windows_sys::core::HRESULT = -2145107932i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_INVALID_COMPUTER_NAME: ::windows_sys::core::HRESULT = -2145107949i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_INVALID_CONFIG_PROP: ::windows_sys::core::HRESULT = -2145107927i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_INVALID_FORMAT: ::windows_sys::core::HRESULT = -2145123271i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_INVALID_OPERATION: ::windows_sys::core::HRESULT = -2145123263i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_INVALID_URL: ::windows_sys::core::HRESULT = -2145123270i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_LOAD_SHEDDING: ::windows_sys::core::HRESULT = -2145107923i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_NO_AUTH_COOKIES_CREATED: ::windows_sys::core::HRESULT = -2145107928i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_NO_AUTH_PLUGINS_REQUESTED: ::windows_sys::core::HRESULT = -2145107929i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_NO_MANAGED_RECOVER: ::windows_sys::core::HRESULT = -2145103826i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_NO_TRANSLATION_AVAILABLE: ::windows_sys::core::HRESULT = -2145123257i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_NUMERIC_OVERFLOW: ::windows_sys::core::HRESULT = -2145123261i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_NWS_NOT_LOADED: ::windows_sys::core::HRESULT = -2145123269i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_OBJECT_FAULTED: ::windows_sys::core::HRESULT = -2145123262i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_OPERATION_ABANDONED: ::windows_sys::core::HRESULT = -2145123259i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_OPERATION_ABORTED: ::windows_sys::core::HRESULT = -2145123260i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_OTHER: ::windows_sys::core::HRESULT = -2145123254i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_PROXY_AUTH_SCHEME_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2145123268i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_QUOTA_EXCEEDED: ::windows_sys::core::HRESULT = -2145123258i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_REFRESH_CACHE_REQUIRED: ::windows_sys::core::HRESULT = -2145107947i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_REGISTRATION_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2145107930i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SAME_REDIR_ID: ::windows_sys::core::HRESULT = -2145103827i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SECURITY_SYSTEM_FAILURE: ::windows_sys::core::HRESULT = -2145123253i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SECURITY_VERIFICATION_FAILURE: ::windows_sys::core::HRESULT = -2145123273i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_BASE: ::windows_sys::core::HRESULT = -2145107968i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_CONNECT: ::windows_sys::core::HRESULT = -2145107964i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_GENERATE: ::windows_sys::core::HRESULT = -2145107965i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_INITIALIZE: ::windows_sys::core::HRESULT = -2145107967i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_OUTOFMEMORY: ::windows_sys::core::HRESULT = -2145107966i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_PARSE: ::windows_sys::core::HRESULT = -2145107958i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_PARSEFAULT: ::windows_sys::core::HRESULT = -2145107960i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_READ: ::windows_sys::core::HRESULT = -2145107959i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_SEND: ::windows_sys::core::HRESULT = -2145107963i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_SERVER: ::windows_sys::core::HRESULT = -2145107962i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_SOAPFAULT: ::windows_sys::core::HRESULT = -2145107961i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAP_CLIENT: ::windows_sys::core::HRESULT = -2145107955i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAP_MUST_UNDERSTAND: ::windows_sys::core::HRESULT = -2145107956i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAP_SERVER: ::windows_sys::core::HRESULT = -2145107954i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAP_VERSION: ::windows_sys::core::HRESULT = -2145107957i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SUS_SERVER_NOT_SET: ::windows_sys::core::HRESULT = -2145107951i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_UNEXPECTED: ::windows_sys::core::HRESULT = -2145103873i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_WINHTTP_NAME_NOT_RESOLVED: ::windows_sys::core::HRESULT = -2145107924i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_WMI_ERROR: ::windows_sys::core::HRESULT = -2145107953i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_RANGEOVERLAP: ::windows_sys::core::HRESULT = -2145124347i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REBOOT_IN_PROGRESS: ::windows_sys::core::HRESULT = -2145083385i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_ATTRPROVIDER_EXCEEDED_MAX_NAMEVALUE: ::windows_sys::core::HRESULT = -2145103864i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_ATTRPROVIDER_INVALID_NAME: ::windows_sys::core::HRESULT = -2145103863i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_ATTRPROVIDER_INVALID_VALUE: ::windows_sys::core::HRESULT = -2145103862i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_CONNECT_POLICY: ::windows_sys::core::HRESULT = -2145103860i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_ID_SMALLER: ::windows_sys::core::HRESULT = -2145103869i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_INVALID_RESPONSE: ::windows_sys::core::HRESULT = -2145103866i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_LOAD_XML: ::windows_sys::core::HRESULT = -2145103871i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_ONLINE_DISALLOWED: ::windows_sys::core::HRESULT = -2145103859i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_SLS_GENERIC_ERROR: ::windows_sys::core::HRESULT = -2145103861i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_S_FALSE: ::windows_sys::core::HRESULT = -2145103870i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_UNEXPECTED: ::windows_sys::core::HRESULT = -2145103617i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_UNKNOWN_SERVICE: ::windows_sys::core::HRESULT = -2145103868i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_UNSUPPORTED_CONTENTTYPE: ::windows_sys::core::HRESULT = -2145103867i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REG_VALUE_INVALID: ::windows_sys::core::HRESULT = -2145124334i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REPORTER_EVENTCACHECORRUPT: ::windows_sys::core::HRESULT = -2145062911i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REPORTER_EVENTNAMESPACEPARSEFAILED: ::windows_sys::core::HRESULT = -2145062910i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REPORTER_UNEXPECTED: ::windows_sys::core::HRESULT = -2145058817i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REVERT_NOT_ALLOWED: ::windows_sys::core::HRESULT = -2145124281i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SELFUPDATE_IN_PROGRESS: ::windows_sys::core::HRESULT = -2145124325i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SELFUPDATE_REQUIRED: ::windows_sys::core::HRESULT = -2145071087i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SELFUPDATE_REQUIRED_ADMIN: ::windows_sys::core::HRESULT = -2145071086i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SELFUPDATE_SKIP_ON_FAILURE: ::windows_sys::core::HRESULT = -2145071096i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SERVER_BUSY: ::windows_sys::core::HRESULT = -2145062908i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SERVICEPROP_NOTAVAIL: ::windows_sys::core::HRESULT = -2145123267i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SERVICE_NOT_REGISTERED: ::windows_sys::core::HRESULT = -2145095675i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SERVICE_STOP: ::windows_sys::core::HRESULT = -2145124322i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_ALREADYRUNNING: ::windows_sys::core::HRESULT = -2145071091i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = -2145071101i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_BLOCKED_CONFIGURATION: ::windows_sys::core::HRESULT = -2145071093i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_DEFERRABLE_REBOOT_PENDING: ::windows_sys::core::HRESULT = -2145071084i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_FAIL: ::windows_sys::core::HRESULT = -2145071082i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_HANDLER_EXEC_FAILURE: ::windows_sys::core::HRESULT = -2145071089i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_INVALID_IDENTDATA: ::windows_sys::core::HRESULT = -2145071102i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_INVALID_INFDATA: ::windows_sys::core::HRESULT = -2145071103i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_INVALID_REGISTRY_DATA: ::windows_sys::core::HRESULT = -2145071088i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_IN_PROGRESS: ::windows_sys::core::HRESULT = -2145124278i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_NON_DEFERRABLE_REBOOT_PENDING: ::windows_sys::core::HRESULT = -2145071083i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2145071100i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_REBOOTREQUIRED: ::windows_sys::core::HRESULT = -2145071090i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_REBOOT_TO_FIX: ::windows_sys::core::HRESULT = -2145071092i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_REGISTRATION_FAILED: ::windows_sys::core::HRESULT = -2145071097i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_SKIP_UPDATE: ::windows_sys::core::HRESULT = -2145071095i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_SOURCE_VERSION_MISMATCH: ::windows_sys::core::HRESULT = -2145071099i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_TARGET_VERSION_GREATER: ::windows_sys::core::HRESULT = -2145071098i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_UNEXPECTED: ::windows_sys::core::HRESULT = -2145067009i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_UNSUPPORTED_CONFIGURATION: ::windows_sys::core::HRESULT = -2145071094i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_WRONG_SERVER_VERSION: ::windows_sys::core::HRESULT = -2145071085i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_ACTION_NOT_FOUND: ::windows_sys::core::HRESULT = -2145103611i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_ANOTHER_INSTANCE_RUNNING: ::windows_sys::core::HRESULT = -2145103597i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_BLOCKED_FOR_PLATFORM: ::windows_sys::core::HRESULT = -2145103598i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_DNSRESILIENCY_OFF: ::windows_sys::core::HRESULT = -2145103596i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_ENGINE_EXCEPTION: ::windows_sys::core::HRESULT = -2145103599i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_INVALIDHASH: ::windows_sys::core::HRESULT = -2145103609i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_NONSTDEXCEPTION: ::windows_sys::core::HRESULT = -2145103600i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_NO_ENGINE: ::windows_sys::core::HRESULT = -2145103608i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_PARSE: ::windows_sys::core::HRESULT = -2145103605i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_POLICY: ::windows_sys::core::HRESULT = -2145103602i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_POST_REBOOT_INSTALL_FAILED: ::windows_sys::core::HRESULT = -2145103607i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_POST_REBOOT_NO_CACHED_SLS_RESPONSE: ::windows_sys::core::HRESULT = -2145103606i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_PPL: ::windows_sys::core::HRESULT = -2145103603i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_SECURITY: ::windows_sys::core::HRESULT = -2145103604i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_SLS_PARSE: ::windows_sys::core::HRESULT = -2145103610i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_STDEXCEPTION: ::windows_sys::core::HRESULT = -2145103601i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_UNEXPECTED: ::windows_sys::core::HRESULT = -2145103361i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_VERIFY_DOWNLOAD_ENGINE: ::windows_sys::core::HRESULT = -2145103615i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_VERIFY_DOWNLOAD_PAYLOAD: ::windows_sys::core::HRESULT = -2145103614i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_VERIFY_STAGE_ENGINE: ::windows_sys::core::HRESULT = -2145103613i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_VERIFY_STAGE_PAYLOAD: ::windows_sys::core::HRESULT = -2145103612i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SKIPPED_UPDATE_INSTALLATION: ::windows_sys::core::HRESULT = -2145079035i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SLS_INVALID_REVISION: ::windows_sys::core::HRESULT = -2145078783i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SOURCE_ABSENT: ::windows_sys::core::HRESULT = -2145124307i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SYSPREP_IN_PROGRESS: ::windows_sys::core::HRESULT = -2145124287i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SYSTEM_UNSUPPORTED: ::windows_sys::core::HRESULT = -2145124266i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TIME_OUT: ::windows_sys::core::HRESULT = -2145124319i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TOOMANYRANGES: ::windows_sys::core::HRESULT = -2145124346i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TOO_DEEP_RELATION: ::windows_sys::core::HRESULT = -2145124336i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TOO_MANY_RESYNC: ::windows_sys::core::HRESULT = -2145124295i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TRAYICON_FAILURE: ::windows_sys::core::HRESULT = -2145112060i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TRUST_PROVIDER_UNKNOWN: ::windows_sys::core::HRESULT = -2145078524i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TRUST_SUBJECT_NOT_TRUSTED: ::windows_sys::core::HRESULT = -2145078525i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_APPX_DEFAULT_PACKAGE_VOLUME_UNAVAILABLE: ::windows_sys::core::HRESULT = -2145116127i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_APPX_INSTALLED_PACKAGE_VOLUME_UNAVAILABLE: ::windows_sys::core::HRESULT = -2145116126i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_APPX_INVALID_PACKAGE_VOLUME: ::windows_sys::core::HRESULT = -2145116128i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_APPX_NOT_PRESENT: ::windows_sys::core::HRESULT = -2145116130i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_APPX_PACKAGE_FAMILY_NOT_FOUND: ::windows_sys::core::HRESULT = -2145116125i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_APPX_SYSTEM_VOLUME_NOT_FOUND: ::windows_sys::core::HRESULT = -2145116124i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_BADCBSPACKAGEID: ::windows_sys::core::HRESULT = -2145116141i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_BADHANDLERXML: ::windows_sys::core::HRESULT = -2145116151i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_CALLED_BACK_FAILURE: ::windows_sys::core::HRESULT = -2145116136i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_CANREQUIREINPUT: ::windows_sys::core::HRESULT = -2145116150i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_CUSTOMINSTALLER_INVALID_SIGNATURE: ::windows_sys::core::HRESULT = -2145116135i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_DECRYPTFAILURE: ::windows_sys::core::HRESULT = -2145116132i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_DOESNOTSUPPORTACTION: ::windows_sys::core::HRESULT = -2145116156i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_FALLBACKERROR: ::windows_sys::core::HRESULT = -2145116144i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_FALLBACKTOSELFCONTAINED: ::windows_sys::core::HRESULT = -2145116148i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_HANDLER_DISABLEDUNTILREBOOT: ::windows_sys::core::HRESULT = -2145116131i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_INCONSISTENT_FILE_NAMES: ::windows_sys::core::HRESULT = -2145116145i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_INSTALLERFAILURE: ::windows_sys::core::HRESULT = -2145116149i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_INSTALLERHUNG: ::windows_sys::core::HRESULT = -2145116153i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_INVALIDMETADATA: ::windows_sys::core::HRESULT = -2145116154i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_INVALID_TARGETSESSION: ::windows_sys::core::HRESULT = -2145116133i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_LOCALONLY: ::windows_sys::core::HRESULT = -2145116159i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_NEEDANOTHERDOWNLOAD: ::windows_sys::core::HRESULT = -2145116147i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_NEW_SERVICING_STACK_REQUIRED: ::windows_sys::core::HRESULT = -2145116137i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_NOTIFYFAILURE: ::windows_sys::core::HRESULT = -2145116146i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_NOTREADYTOCOMMIT: ::windows_sys::core::HRESULT = -2145116129i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_OPERATIONCANCELLED: ::windows_sys::core::HRESULT = -2145116152i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_POSTREBOOTRESULTUNKNOWN: ::windows_sys::core::HRESULT = -2145116139i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_POSTREBOOTSTILLPENDING: ::windows_sys::core::HRESULT = -2145116140i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_POSTREBOOTUNEXPECTEDSTATE: ::windows_sys::core::HRESULT = -2145116138i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_REMOTEALREADYACTIVE: ::windows_sys::core::HRESULT = -2145116157i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_REMOTEUNAVAILABLE: ::windows_sys::core::HRESULT = -2145116160i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_TOOMANYDOWNLOADREQUESTS: ::windows_sys::core::HRESULT = -2145116143i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_UNEXPECTED: ::windows_sys::core::HRESULT = -2145112065i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_UNEXPECTEDCBSRESPONSE: ::windows_sys::core::HRESULT = -2145116142i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_UNKNOWNHANDLER: ::windows_sys::core::HRESULT = -2145116158i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_UNSUPPORTED_INSTALLCONTEXT: ::windows_sys::core::HRESULT = -2145116134i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_WRONGHANDLER: ::windows_sys::core::HRESULT = -2145116155i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNEXPECTED: ::windows_sys::core::HRESULT = -2145120257i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNINSTALL_NOT_ALLOWED: ::windows_sys::core::HRESULT = -2145124312i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNKNOWN_HARDWARECAPABILITY: ::windows_sys::core::HRESULT = -2145079039i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNKNOWN_ID: ::windows_sys::core::HRESULT = -2145124349i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNKNOWN_SERVICE: ::windows_sys::core::HRESULT = -2145124286i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNRECOGNIZED_VOLUMEID: ::windows_sys::core::HRESULT = -2145124259i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNSUPPORTED_SEARCHSCOPE: ::windows_sys::core::HRESULT = -2145124283i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UPDATE_MERGE_NOT_ALLOWED: ::windows_sys::core::HRESULT = -2145079036i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UPDATE_NOT_APPROVED: ::windows_sys::core::HRESULT = -2145124254i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UPDATE_NOT_PROCESSED: ::windows_sys::core::HRESULT = -2145124299i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_URL_TOO_LONG: ::windows_sys::core::HRESULT = -2145124313i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_USER_ACCESS_DISABLED: ::windows_sys::core::HRESULT = -2145124315i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WINHTTP_INVALID_FILE: ::windows_sys::core::HRESULT = -2145124296i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WMI_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2145079037i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WUCLTUI_UNSUPPORTED_VERSION: ::windows_sys::core::HRESULT = -2145107970i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WUTASK_CANCELINSTALL_DISALLOWED: ::windows_sys::core::HRESULT = -2145079291i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WUTASK_INPROGRESS: ::windows_sys::core::HRESULT = -2145079295i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WUTASK_NOT_STARTED: ::windows_sys::core::HRESULT = -2145079293i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WUTASK_RETRY: ::windows_sys::core::HRESULT = -2145079292i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WUTASK_STATUS_DISABLED: ::windows_sys::core::HRESULT = -2145079294i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WU_DISABLED: ::windows_sys::core::HRESULT = -2145124306i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_XML_INVALID: ::windows_sys::core::HRESULT = -2145124338i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_XML_MISSINGDATA: ::windows_sys::core::HRESULT = -2145124339i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_ALREADY_DOWNLOADED: ::windows_sys::core::HRESULT = 2359304i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_ALREADY_INSTALLED: ::windows_sys::core::HRESULT = 2359302i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_ALREADY_REVERTED: ::windows_sys::core::HRESULT = 2359306i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_ALREADY_UNINSTALLED: ::windows_sys::core::HRESULT = 2359303i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_DM_ALREADYDOWNLOADING: ::windows_sys::core::HRESULT = 2383873i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_MARKED_FOR_DISCONNECT: ::windows_sys::core::HRESULT = 2359300i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_METADATA_IGNORED_SIGNATURE_VERIFICATION: ::windows_sys::core::HRESULT = 2388226i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_METADATA_SKIPPED_BY_ENFORCEMENTMODE: ::windows_sys::core::HRESULT = 2388225i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_REBOOT_REQUIRED: ::windows_sys::core::HRESULT = 2359301i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_SEARCH_CRITERIA_NOT_SUPPORTED: ::windows_sys::core::HRESULT = 2359312i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_SEARCH_LOAD_SHEDDING: ::windows_sys::core::HRESULT = 2392065i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_SELFUPDATE: ::windows_sys::core::HRESULT = 2359298i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_SERVICE_STOP: ::windows_sys::core::HRESULT = 2359297i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_SIH_NOOP: ::windows_sys::core::HRESULT = 2379777i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_SOME_UPDATES_SKIPPED_ON_BATTERY: ::windows_sys::core::HRESULT = 2359305i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_UH_DOWNLOAD_SIZE_CALCULATED: ::windows_sys::core::HRESULT = 2367510i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_UH_INSTALLSTILLPENDING: ::windows_sys::core::HRESULT = 2367509i32;
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_UPDATE_ERROR: ::windows_sys::core::HRESULT = 2359299i32;
pub const WebProxy: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1694827471, data2: 37128, data3: 19932, data4: [162, 206, 108, 35, 65, 225, 197, 130] };
pub const WindowsUpdateAgentInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3270020143, data2: 28507, data3: 19114, data4: [137, 75, 85, 200, 71, 173, 58, 45] };
