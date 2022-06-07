#[repr(C)]
pub struct IInkWorkspaceHostedAppManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetThumbnailAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetThumbnailAsync: usize,
}
impl ::windows_sys::core::Interface for IInkWorkspaceHostedAppManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4262099344, data2: 24153, data3: 19383, data4: [138, 99, 125, 33, 140, 217, 99, 0] };
}
#[repr(C)]
pub struct IInkWorkspaceHostedAppManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkWorkspaceHostedAppManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3422391493, data2: 41314, data3: 19396, data4: [132, 238, 232, 113, 109, 82, 51, 197] };
}
pub type InkWorkspaceHostedAppManager = *mut ::core::ffi::c_void;
