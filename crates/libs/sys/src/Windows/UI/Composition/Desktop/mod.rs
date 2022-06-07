pub type DesktopWindowTarget = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDesktopWindowTarget {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTopmost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDesktopWindowTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1663686346, data2: 13158, data3: 18702, data4: [157, 179, 37, 49, 41, 41, 172, 81] };
}
