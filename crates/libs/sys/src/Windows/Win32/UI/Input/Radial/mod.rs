#[repr(C)]
pub struct IRadialControllerConfigurationInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
impl ::windows_sys::core::Interface for IRadialControllerConfigurationInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2021448364, data2: 12678, data3: 18285, data4: [135, 228, 185, 55, 74, 123, 153, 112] };
}
#[repr(C)]
pub struct IRadialControllerIndependentInputSourceInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
}
impl ::windows_sys::core::Interface for IRadialControllerIndependentInputSourceInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144319, data2: 19694, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
}
#[repr(C)]
pub struct IRadialControllerInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
}
impl ::windows_sys::core::Interface for IRadialControllerInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 453326281, data2: 22445, data3: 17857, data4: [157, 121, 173, 92, 52, 54, 5, 19] };
}
