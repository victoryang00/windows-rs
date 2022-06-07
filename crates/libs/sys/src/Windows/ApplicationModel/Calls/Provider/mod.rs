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
impl ::windows_sys::core::Interface for IPhoneCallOrigin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 543241337, data2: 3833, data3: 17492, data4: [135, 28, 175, 182, 106, 20, 182, 165] };
}
#[repr(C)]
pub struct IPhoneCallOrigin2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneCallOrigin2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 80210304, data2: 39618, data3: 18280, data4: [181, 54, 182, 141, 164, 149, 125, 2] };
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
impl ::windows_sys::core::Interface for IPhoneCallOrigin3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1228083124, data2: 53671, data3: 17314, data4: [174, 238, 192, 123, 109, 186, 240, 104] };
}
#[repr(C)]
pub struct IPhoneCallOriginManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCurrentAppActiveCallOriginApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShowPhoneCallOriginSettingsUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetCallOrigin: unsafe extern "system" fn(this: *mut *mut Self, requestid: ::windows_sys::core::GUID, callorigin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneCallOriginManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3439090186, data2: 39671, data3: 24905, data4: [57, 208, 224, 118, 252, 206, 19, 149] };
}
#[repr(C)]
pub struct IPhoneCallOriginManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestSetAsActiveCallOriginAppAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetAsActiveCallOriginAppAsync: usize,
}
impl ::windows_sys::core::Interface for IPhoneCallOriginManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2348019263, data2: 16628, data3: 17280, data4: [140, 124, 174, 162, 201, 184, 221, 122] };
}
#[repr(C)]
pub struct IPhoneCallOriginManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneCallOriginManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 785815396, data2: 42723, data3: 20720, data4: [183, 106, 214, 124, 179, 155, 223, 222] };
}
pub type PhoneCallOrigin = *mut ::core::ffi::c_void;
