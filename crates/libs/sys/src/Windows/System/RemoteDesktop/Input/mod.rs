#[repr(C)]
pub struct IRemoteTextConnection {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RegisterThread: unsafe extern "system" fn(this: *mut *mut Self, threadid: u32) -> ::windows_sys::core::HRESULT,
    pub UnregisterThread: unsafe extern "system" fn(this: *mut *mut Self, threadid: u32) -> ::windows_sys::core::HRESULT,
    pub ReportDataReceived: unsafe extern "system" fn(this: *mut *mut Self, pduData_array_size: u32, pdudata: *const u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteTextConnectionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, connectionid: ::windows_sys::core::GUID, pduforwarder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type RemoteTextConnection = *mut ::core::ffi::c_void;
pub type RemoteTextConnectionDataHandler = *mut ::core::ffi::c_void;
