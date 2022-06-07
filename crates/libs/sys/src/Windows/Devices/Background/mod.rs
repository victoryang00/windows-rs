pub type DeviceServicingDetails = *mut ::core::ffi::c_void;
pub type DeviceUseDetails = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDeviceServicingDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpectedDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpectedDuration: usize,
}
impl ::windows_sys::core::Interface for IDeviceServicingDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1252781609, data2: 9028, data3: 19140, data4: [133, 39, 74, 142, 246, 144, 86, 69] };
}
#[repr(C)]
pub struct IDeviceUseDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceUseDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2102808897, data2: 21886, data3: 16724, data4: [185, 148, 228, 247, 161, 31, 179, 35] };
}
