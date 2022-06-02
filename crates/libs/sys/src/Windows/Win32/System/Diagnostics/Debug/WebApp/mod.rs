#[repr(C)]
pub struct IWebApplicationActivation {
    pub base__: ::windows_sys::core::IUnknown,
    pub CancelPendingActivation: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IWebApplicationUIEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub SecurityProblem: unsafe extern "system" fn(this: *mut *mut Self, securityproblem: u32, result: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebApplicationUpdateEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnPaint: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnCssChanged: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type RegisterAuthoringClientFunctionType = ::core::option::Option<unsafe extern "system" fn(authoringmodeobject: *mut *mut IWebApplicationAuthoringMode, host: *mut *mut IWebApplicationHost) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
pub type UnregisterAuthoringClientFunctionType = ::core::option::Option<unsafe extern "system" fn(host: *mut *mut IWebApplicationHost) -> ::windows_sys::core::HRESULT>;
