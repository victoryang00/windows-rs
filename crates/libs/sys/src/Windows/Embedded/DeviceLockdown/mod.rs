pub type DeviceLockdownProfileInformation = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDeviceLockdownProfileInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceLockdownProfileInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2038489422, data2: 17841, data3: 19094, data4: [146, 252, 98, 117, 107, 115, 150, 120] };
}
#[repr(C)]
pub struct IDeviceLockdownProfileStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedLockdownProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedLockdownProfiles: usize,
    pub GetCurrentLockdownProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplyLockdownProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, profileid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplyLockdownProfileAsync: usize,
    pub GetLockdownProfileInformation: unsafe extern "system" fn(this: *mut *mut Self, profileid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceLockdownProfileStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1647274341, data2: 63912, data3: 16801, data4: [166, 145, 136, 205, 128, 199, 160, 105] };
}
