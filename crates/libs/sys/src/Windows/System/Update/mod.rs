#[repr(C)]
pub struct ISystemUpdateItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemUpdateItemState) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Revision: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DownloadProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub InstallProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemUpdateItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2006401259, data2: 22052, data3: 20894, data4: [168, 226, 9, 233, 23, 59, 63, 183] };
}
#[repr(C)]
pub struct ISystemUpdateLastErrorInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemUpdateManagerState) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub IsInteractive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemUpdateLastErrorInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2129168375, data2: 35396, data3: 23406, data4: [189, 7, 122, 236, 228, 17, 110, 169] };
}
#[repr(C)]
pub struct ISystemUpdateManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemUpdateManagerState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    pub DownloadProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub InstallProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UserActiveHoursStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserActiveHoursStart: usize,
    #[cfg(feature = "Foundation")]
    pub UserActiveHoursEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserActiveHoursEnd: usize,
    pub UserActiveHoursMax: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TrySetUserActiveHours: unsafe extern "system" fn(this: *mut *mut Self, start: super::super::Foundation::TimeSpan, end: super::super::Foundation::TimeSpan, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetUserActiveHours: usize,
    #[cfg(feature = "Foundation")]
    pub LastUpdateCheckTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdateCheckTime: usize,
    #[cfg(feature = "Foundation")]
    pub LastUpdateInstallTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdateInstallTime: usize,
    pub LastErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAutomaticRebootBlockIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAutomaticRebootBlockIds: usize,
    #[cfg(feature = "Foundation")]
    pub BlockAutomaticRebootAsync: unsafe extern "system" fn(this: *mut *mut Self, lockid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BlockAutomaticRebootAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnblockAutomaticRebootAsync: unsafe extern "system" fn(this: *mut *mut Self, lockid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnblockAutomaticRebootAsync: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUpdateItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUpdateItems: usize,
    pub AttentionRequiredReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemUpdateAttentionRequiredReason) -> ::windows_sys::core::HRESULT,
    pub SetFlightRing: unsafe extern "system" fn(this: *mut *mut Self, flightring: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetFlightRing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StartInstall: unsafe extern "system" fn(this: *mut *mut Self, action: SystemUpdateStartInstallAction) -> ::windows_sys::core::HRESULT,
    pub RebootToCompleteInstall: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StartCancelUpdates: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemUpdateManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3000237295, data2: 10609, data3: 20926, data4: [180, 26, 139, 215, 3, 187, 112, 26] };
}
#[doc = "*Required features: `\"System_Update\"`*"]
#[repr(transparent)]
pub struct SystemUpdateAttentionRequiredReason(pub i32);
impl SystemUpdateAttentionRequiredReason {
    pub const None: Self = Self(0i32);
    pub const NetworkRequired: Self = Self(1i32);
    pub const InsufficientDiskSpace: Self = Self(2i32);
    pub const InsufficientBattery: Self = Self(3i32);
    pub const UpdateBlocked: Self = Self(4i32);
}
impl ::core::marker::Copy for SystemUpdateAttentionRequiredReason {}
impl ::core::clone::Clone for SystemUpdateAttentionRequiredReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SystemUpdateItem = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_Update\"`*"]
#[repr(transparent)]
pub struct SystemUpdateItemState(pub i32);
impl SystemUpdateItemState {
    pub const NotStarted: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const Preparing: Self = Self(2i32);
    pub const Calculating: Self = Self(3i32);
    pub const Downloading: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const RebootRequired: Self = Self(7i32);
    pub const Error: Self = Self(8i32);
}
impl ::core::marker::Copy for SystemUpdateItemState {}
impl ::core::clone::Clone for SystemUpdateItemState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SystemUpdateLastErrorInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_Update\"`*"]
#[repr(transparent)]
pub struct SystemUpdateManagerState(pub i32);
impl SystemUpdateManagerState {
    pub const Idle: Self = Self(0i32);
    pub const Detecting: Self = Self(1i32);
    pub const ReadyToDownload: Self = Self(2i32);
    pub const Downloading: Self = Self(3i32);
    pub const ReadyToInstall: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const RebootRequired: Self = Self(6i32);
    pub const ReadyToFinalize: Self = Self(7i32);
    pub const Finalizing: Self = Self(8i32);
    pub const Completed: Self = Self(9i32);
    pub const AttentionRequired: Self = Self(10i32);
    pub const Error: Self = Self(11i32);
}
impl ::core::marker::Copy for SystemUpdateManagerState {}
impl ::core::clone::Clone for SystemUpdateManagerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System_Update\"`*"]
#[repr(transparent)]
pub struct SystemUpdateStartInstallAction(pub i32);
impl SystemUpdateStartInstallAction {
    pub const UpToReboot: Self = Self(0i32);
    pub const AllowReboot: Self = Self(1i32);
}
impl ::core::marker::Copy for SystemUpdateStartInstallAction {}
impl ::core::clone::Clone for SystemUpdateStartInstallAction {
    fn clone(&self) -> Self {
        *self
    }
}
