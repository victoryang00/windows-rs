#[repr(C)]
pub struct IClassicAppManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FindInstalledApp: unsafe extern "system" fn(this: *mut *mut Self, appuninstallkey: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInstalledClassicAppInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
pub type InstalledClassicAppInfo = *mut ::core::ffi::c_void;
