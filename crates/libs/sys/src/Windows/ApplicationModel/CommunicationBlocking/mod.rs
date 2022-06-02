#[repr(C)]
pub struct ICommunicationBlockingAccessManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBlockingActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsBlockedNumberAsync: unsafe extern "system" fn(this: *mut *mut Self, number: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsBlockedNumberAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShowBlockNumbersUI: unsafe extern "system" fn(this: *mut *mut Self, phonenumbers: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShowBlockNumbersUI: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShowUnblockNumbersUI: unsafe extern "system" fn(this: *mut *mut Self, phonenumbers: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShowUnblockNumbersUI: usize,
    pub ShowBlockedCallsUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ShowBlockedMessagesUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommunicationBlockingAppManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCurrentAppActiveBlockingApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShowCommunicationBlockingSettingsUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommunicationBlockingAppManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestSetAsActiveBlockingAppAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetAsActiveBlockingAppAsync: usize,
}
