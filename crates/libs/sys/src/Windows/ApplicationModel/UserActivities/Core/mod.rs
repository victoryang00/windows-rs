#[repr(C)]
pub struct ICoreUserActivityManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateUserActivitySessionInBackground: unsafe extern "system" fn(this: *mut *mut Self, activity: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteUserActivitySessionsInTimeRangeAsync: unsafe extern "system" fn(this: *mut *mut Self, channel: *mut ::core::ffi::c_void, starttime: super::super::super::Foundation::DateTime, endtime: super::super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteUserActivitySessionsInTimeRangeAsync: usize,
}
impl ::windows_sys::core::Interface for ICoreUserActivityManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3392854786, data2: 42174, data3: 19789, data4: [191, 168, 103, 149, 244, 38, 78, 251] };
}
