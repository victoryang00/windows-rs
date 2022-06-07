#[repr(C)]
pub struct IWebApplicationActivation {
    pub base__: ::windows_sys::core::IUnknown,
    pub CancelPendingActivation: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebApplicationActivation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3168587998, data2: 13070, data3: 18459, data4: [184, 67, 72, 152, 166, 168, 235, 172] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWebApplicationAuthoringMode {
    pub base__: super::super::super::Com::IServiceProvider,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthoringClientBinary: unsafe extern "system" fn(this: *mut *mut Self, designmodedllpath: *mut super::super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthoringClientBinary: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWebApplicationAuthoringMode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1913318035, data2: 6500, data3: 19888, data4: [176, 5, 41, 235, 158, 43, 24, 169] };
}
#[repr(C)]
pub struct IWebApplicationHost {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub HWND: unsafe extern "system" fn(this: *mut *mut Self, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HWND: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub Document: unsafe extern "system" fn(this: *mut *mut Self, htmldocument: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    Document: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut *mut Self, interfaceid: *const ::windows_sys::core::GUID, callback: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut *mut Self, cookie: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebApplicationHost {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3469464259, data2: 41893, data3: 18249, data4: [150, 129, 32, 233, 22, 28, 103, 148] };
}
#[repr(C)]
pub struct IWebApplicationNavigationEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub BeforeNavigate: unsafe extern "system" fn(this: *mut *mut Self, htmlwindow: *mut ::core::ffi::c_void, url: ::windows_sys::core::PCWSTR, navigationflags: u32, targetframename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    BeforeNavigate: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub NavigateComplete: unsafe extern "system" fn(this: *mut *mut Self, htmlwindow: *mut ::core::ffi::c_void, url: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    NavigateComplete: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub NavigateError: unsafe extern "system" fn(this: *mut *mut Self, htmlwindow: *mut ::core::ffi::c_void, url: ::windows_sys::core::PCWSTR, targetframename: ::windows_sys::core::PCWSTR, statuscode: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    NavigateError: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub DocumentComplete: unsafe extern "system" fn(this: *mut *mut Self, htmlwindow: *mut ::core::ffi::c_void, url: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    DocumentComplete: usize,
    pub DownloadBegin: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DownloadComplete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebApplicationNavigationEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3257275858, data2: 54040, data3: 19874, data4: [132, 34, 31, 202, 247, 123, 16, 228] };
}
#[repr(C)]
pub struct IWebApplicationScriptEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub BeforeScriptExecute: unsafe extern "system" fn(this: *mut *mut Self, htmlwindow: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    BeforeScriptExecute: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub ScriptError: unsafe extern "system" fn(this: *mut *mut Self, htmlwindow: *mut ::core::ffi::c_void, scripterror: *mut ::core::ffi::c_void, url: ::windows_sys::core::PCWSTR, errorhandled: super::super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    ScriptError: usize,
}
impl ::windows_sys::core::Interface for IWebApplicationScriptEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2084530584, data2: 5479, data3: 19386, data4: [181, 43, 72, 211, 33, 65, 214, 19] };
}
#[repr(C)]
pub struct IWebApplicationUIEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub SecurityProblem: unsafe extern "system" fn(this: *mut *mut Self, securityproblem: u32, result: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebApplicationUIEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1529560985, data2: 12940, data3: 16853, data4: [166, 247, 116, 131, 237, 142, 113, 221] };
}
#[repr(C)]
pub struct IWebApplicationUpdateEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnPaint: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnCssChanged: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebApplicationUpdateEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1046079159, data2: 50770, data3: 19887, data4: [173, 94, 22, 254, 179, 80, 205, 227] };
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type RegisterAuthoringClientFunctionType = ::core::option::Option<unsafe extern "system" fn(authoringmodeobject: *mut *mut IWebApplicationAuthoringMode, host: *mut *mut IWebApplicationHost) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
pub type UnregisterAuthoringClientFunctionType = ::core::option::Option<unsafe extern "system" fn(host: *mut *mut IWebApplicationHost) -> ::windows_sys::core::HRESULT>;
