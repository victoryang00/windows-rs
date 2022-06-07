#[repr(C)]
pub struct IWindowManagementPreview {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IWindowManagementPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1324702477, data2: 22045, data3: 20796, data4: [166, 124, 44, 2, 182, 156, 239, 65] };
}
#[repr(C)]
pub struct IWindowManagementPreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetPreferredMinSize: unsafe extern "system" fn(this: *mut *mut Self, window: *mut ::core::ffi::c_void, preferredframeminsize: super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredMinSize: usize,
}
impl ::windows_sys::core::Interface for IWindowManagementPreviewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 261563846, data2: 49156, data3: 23075, data4: [143, 210, 141, 9, 44, 226, 112, 74] };
}
pub type WindowManagementPreview = *mut ::core::ffi::c_void;
