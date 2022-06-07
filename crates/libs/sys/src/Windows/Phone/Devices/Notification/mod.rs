#[repr(C)]
pub struct IVibrationDevice {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Vibrate: unsafe extern "system" fn(this: *mut *mut Self, duration: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Vibrate: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVibrationDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 457860501, data2: 53197, data3: 19976, data4: [146, 251, 193, 144, 109, 4, 73, 140] };
}
#[repr(C)]
pub struct IVibrationDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVibrationDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 858772209, data2: 7273, data3: 19601, data4: [148, 158, 75, 182, 122, 133, 189, 199] };
}
pub type VibrationDevice = *mut ::core::ffi::c_void;
