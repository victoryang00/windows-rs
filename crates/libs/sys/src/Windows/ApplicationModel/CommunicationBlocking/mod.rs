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
impl ::windows_sys::core::Interface for ICommunicationBlockingAccessManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 479631768, data2: 40234, data3: 23991, data4: [237, 213, 12, 228, 7, 252, 37, 149] };
}
#[repr(C)]
pub struct ICommunicationBlockingAppManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCurrentAppActiveBlockingApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShowCommunicationBlockingSettingsUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICommunicationBlockingAppManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2010863852, data2: 5286, data3: 19370, data4: [148, 42, 106, 103, 61, 153, 155, 242] };
}
#[repr(C)]
pub struct ICommunicationBlockingAppManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestSetAsActiveBlockingAppAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetAsActiveBlockingAppAsync: usize,
}
impl ::windows_sys::core::Interface for ICommunicationBlockingAppManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 346459869, data2: 60808, data3: 17786, data4: [163, 100, 163, 99, 77, 111, 22, 109] };
}
