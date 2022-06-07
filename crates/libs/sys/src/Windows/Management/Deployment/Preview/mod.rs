#[repr(C)]
pub struct IClassicAppManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FindInstalledApp: unsafe extern "system" fn(this: *mut *mut Self, appuninstallkey: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IClassicAppManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3808089704, data2: 34860, data3: 20275, data4: [176, 53, 13, 247, 185, 13, 103, 230] };
}
#[repr(C)]
pub struct IInstalledClassicAppInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInstalledClassicAppInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 175979939, data2: 26064, data3: 16518, data4: [128, 214, 6, 16, 215, 96, 32, 125] };
}
pub type InstalledClassicAppInfo = *mut ::core::ffi::c_void;
