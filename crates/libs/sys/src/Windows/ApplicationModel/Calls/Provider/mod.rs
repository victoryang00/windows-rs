#[repr(C)]
pub struct IPhoneCallOrigin {
    pub base__: ::windows_sys::core::IInspectable,
    pub Category: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CategoryDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCategoryDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPhoneCallOrigin2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPhoneCallOrigin3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub DisplayPicture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    DisplayPicture: usize,
    #[cfg(feature = "Storage")]
    pub SetDisplayPicture: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetDisplayPicture: usize,
}
#[repr(C)]
pub struct IPhoneCallOriginManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCurrentAppActiveCallOriginApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShowPhoneCallOriginSettingsUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetCallOrigin: unsafe extern "system" fn(this: *mut *mut Self, requestid: ::windows_sys::core::GUID, callorigin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPhoneCallOriginManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestSetAsActiveCallOriginAppAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetAsActiveCallOriginAppAsync: usize,
}
#[repr(C)]
pub struct IPhoneCallOriginManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
pub type PhoneCallOrigin = *mut ::core::ffi::c_void;
