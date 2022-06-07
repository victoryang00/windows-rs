#[repr(C)]
pub struct ISysStorageProviderEventReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Json: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISysStorageProviderEventReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3778204089, data2: 31645, data3: 22560, data4: [151, 40, 66, 98, 181, 40, 145, 66] };
}
#[repr(C)]
pub struct ISysStorageProviderEventReceivedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, json: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISysStorageProviderEventReceivedEventArgsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3726276622, data2: 59765, data3: 24424, data4: [188, 198, 251, 70, 40, 28, 106, 97] };
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
impl ::windows_sys::core::Interface for ISysStorageProviderEventSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 523682934, data2: 38214, data3: 21354, data4: [131, 129, 47, 154, 44, 8, 206, 221] };
}
#[repr(C)]
pub struct ISysStorageProviderHandlerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetHttpRequestProvider: unsafe extern "system" fn(this: *mut *mut Self, syncrootid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEventSource: unsafe extern "system" fn(this: *mut *mut Self, syncrootid: ::windows_sys::core::HSTRING, eventname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISysStorageProviderHandlerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4000941105, data2: 33299, data3: 24201, data4: [166, 35, 20, 216, 199, 43, 138, 97] };
}
#[repr(C)]
pub struct ISysStorageProviderHttpRequestProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Http")))]
    SendRequestAsync: usize,
}
impl ::windows_sys::core::Interface for ISysStorageProviderHttpRequestProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3413110710, data2: 59242, data3: 23589, data4: [163, 62, 62, 120, 166, 224, 224, 206] };
}
pub type SysStorageProviderEventReceivedEventArgs = *mut ::core::ffi::c_void;
