#[repr(C)]
pub struct IPreviewBuildsManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub ArePreviewBuildsAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetArePreviewBuildsAllowed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub GetCurrentState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
}
impl ::windows_sys::core::Interface for IPreviewBuildsManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4194819425, data2: 32335, data3: 23031, data4: [124, 159, 222, 249, 5, 28, 95, 98] };
}
#[repr(C)]
pub struct IPreviewBuildsManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPreviewBuildsManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1044523143, data2: 45330, data3: 23152, data4: [125, 161, 151, 215, 141, 50, 170, 41] };
}
#[repr(C)]
pub struct IPreviewBuildsState {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
impl ::windows_sys::core::Interface for IPreviewBuildsState {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2733805630, data2: 45603, data3: 24419, data4: [117, 70, 62, 142, 172, 7, 10, 46] };
}
pub type PreviewBuildsManager = *mut ::core::ffi::c_void;
pub type PreviewBuildsState = *mut ::core::ffi::c_void;
