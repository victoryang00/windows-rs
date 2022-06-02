#[repr(C)]
pub struct ICoreUserActivityManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateUserActivitySessionInBackground: unsafe extern "system" fn(this: *mut *mut Self, activity: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteUserActivitySessionsInTimeRangeAsync: unsafe extern "system" fn(this: *mut *mut Self, channel: *mut ::core::ffi::c_void, starttime: super::super::super::Foundation::DateTime, endtime: super::super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteUserActivitySessionsInTimeRangeAsync: usize,
}
