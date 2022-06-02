pub type AppBroadcastingMonitor = *mut ::core::ffi::c_void;
pub type AppBroadcastingStatus = *mut ::core::ffi::c_void;
pub type AppBroadcastingStatusDetails = *mut ::core::ffi::c_void;
pub type AppBroadcastingUI = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAppBroadcastingMonitor {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCurrentAppBroadcasting: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsCurrentAppBroadcastingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsCurrentAppBroadcastingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsCurrentAppBroadcastingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsCurrentAppBroadcastingChanged: usize,
}
#[repr(C)]
pub struct IAppBroadcastingStatus {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanStartBroadcast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBroadcastingStatusDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAnyAppBroadcasting: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCaptureResourceUnavailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsGameStreamInProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsAppInactive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsBlockedForApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDisabledByUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDisabledBySystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBroadcastingUI {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShowBroadcastUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBroadcastingUIStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
