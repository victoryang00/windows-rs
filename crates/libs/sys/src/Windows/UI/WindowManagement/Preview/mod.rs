#[repr(C)]
pub struct IWindowManagementPreview {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IWindowManagementPreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetPreferredMinSize: unsafe extern "system" fn(this: *mut *mut Self, window: *mut ::core::ffi::c_void, preferredframeminsize: super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredMinSize: usize,
}
pub type WindowManagementPreview = *mut ::core::ffi::c_void;
