#[repr(C)]
pub struct IInkWorkspaceHostedAppManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetThumbnailAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetThumbnailAsync: usize,
}
#[repr(C)]
pub struct IInkWorkspaceHostedAppManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type InkWorkspaceHostedAppManager = *mut ::core::ffi::c_void;
