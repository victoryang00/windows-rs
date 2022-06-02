pub type ApplicationDataManager = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IApplicationDataManager {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IApplicationDataManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub CreateForPackageFamily: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateForPackageFamily: usize,
}
