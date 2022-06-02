#[repr(C)]
pub struct IDisplayDeviceInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub CreateSharedHandle: unsafe extern "system" fn(this: *mut *mut Self, pobject: *mut ::core::ffi::c_void, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::windows_sys::core::HSTRING, phandle: *mut super::super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    CreateSharedHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenSharedHandle: unsafe extern "system" fn(this: *mut *mut Self, nthandle: super::super::super::Foundation::HANDLE, riid: ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenSharedHandle: usize,
}
#[repr(C)]
pub struct IDisplayPathInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSourcePresentationHandle: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSourcePresentationHandle: usize,
    pub GetSourceId: unsafe extern "system" fn(this: *mut *mut Self, psourceid: *mut u32) -> ::windows_sys::core::HRESULT,
}
