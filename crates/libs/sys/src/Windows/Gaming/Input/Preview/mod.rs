#[repr(C)]
pub struct IGameControllerProviderInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Gaming_Input_Custom")]
    pub GetParentProviderId: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))]
    GetParentProviderId: usize,
    #[cfg(feature = "Gaming_Input_Custom")]
    pub GetProviderId: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))]
    GetProviderId: usize,
}
