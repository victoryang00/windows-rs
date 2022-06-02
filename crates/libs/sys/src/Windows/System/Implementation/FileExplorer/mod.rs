#[repr(C)]
pub struct ISysStorageProviderEventReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Json: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISysStorageProviderEventReceivedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, json: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISysStorageProviderEventSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub EventReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EventReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEventReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEventReceived: usize,
}
#[repr(C)]
pub struct ISysStorageProviderHandlerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetHttpRequestProvider: unsafe extern "system" fn(this: *mut *mut Self, syncrootid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEventSource: unsafe extern "system" fn(this: *mut *mut Self, syncrootid: ::windows_sys::core::HSTRING, eventname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISysStorageProviderHttpRequestProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Http")))]
    SendRequestAsync: usize,
}
pub type SysStorageProviderEventReceivedEventArgs = *mut ::core::ffi::c_void;
