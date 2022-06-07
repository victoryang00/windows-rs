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
impl ::windows_sys::core::Interface for IAppBroadcastingMonitor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 16341608, data2: 35079, data3: 18592, data4: [184, 239, 36, 210, 8, 19, 117, 66] };
}
#[repr(C)]
pub struct IAppBroadcastingStatus {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanStartBroadcast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastingStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 304473311, data2: 929, data3: 17144, data4: [139, 128, 201, 34, 140, 217, 207, 46] };
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
impl ::windows_sys::core::Interface for IAppBroadcastingStatusDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 110996900, data2: 46451, data3: 20028, data4: [142, 25, 27, 175, 172, 208, 151, 19] };
}
#[repr(C)]
pub struct IAppBroadcastingUI {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShowBroadcastUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastingUI {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3849297807, data2: 61081, data3: 19914, data4: [163, 195, 112, 175, 61, 180, 79, 95] };
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
impl ::windows_sys::core::Interface for IAppBroadcastingUIStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1437116317, data2: 9163, data3: 17785, data4: [156, 52, 136, 111, 224, 44, 4, 90] };
}
