#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Security_Authorization_UI")]
    pub fn DSCreateISecurityInfoObject(pwszobjectpath: ::windows_core_sys::PCWSTR, pwszobjectclass: ::windows_core_sys::PCWSTR, dwflags: u32, ppsi: *mut super::Authorization::UI::ISecurityInformation, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: ::win32_foundation_sys::LPARAM) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Security_Authorization_UI")]
    pub fn DSCreateISecurityInfoObjectEx(pwszobjectpath: ::windows_core_sys::PCWSTR, pwszobjectclass: ::windows_core_sys::PCWSTR, pwszserver: ::windows_core_sys::PCWSTR, pwszusername: ::windows_core_sys::PCWSTR, pwszpassword: ::windows_core_sys::PCWSTR, dwflags: u32, ppsi: *mut super::Authorization::UI::ISecurityInformation, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: ::win32_foundation_sys::LPARAM) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_UI_Controls")]
    pub fn DSCreateSecurityPage(pwszobjectpath: ::windows_core_sys::PCWSTR, pwszobjectclass: ::windows_core_sys::PCWSTR, dwflags: u32, phpage: *mut ::win32_ui_sys::Controls::HPROPSHEETPAGE, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: ::win32_foundation_sys::LPARAM) -> ::windows_core_sys::HRESULT;
    pub fn DSEditSecurity(hwndowner: ::win32_foundation_sys::HWND, pwszobjectpath: ::windows_core_sys::PCWSTR, pwszobjectclass: ::windows_core_sys::PCWSTR, dwflags: u32, pwszcaption: ::windows_core_sys::PCWSTR, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: ::win32_foundation_sys::LPARAM) -> ::windows_core_sys::HRESULT;
}
pub const DSSI_IS_ROOT: u32 = 16u32;
pub const DSSI_NO_ACCESS_CHECK: u32 = 2u32;
pub const DSSI_NO_EDIT_OWNER: u32 = 8u32;
pub const DSSI_NO_EDIT_SACL: u32 = 4u32;
pub const DSSI_NO_FILTER: u32 = 32u32;
pub const DSSI_NO_READONLY_MESSAGE: u32 = 64u32;
pub const DSSI_READ_ONLY: u32 = 1u32;
#[cfg(feature = "Win32_Security_Authorization_UI")]
pub type PFNDSCREATEISECINFO = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core_sys::PCWSTR, param1: ::windows_core_sys::PCWSTR, param2: u32, param3: *mut super::Authorization::UI::ISecurityInformation, param4: PFNREADOBJECTSECURITY, param5: PFNWRITEOBJECTSECURITY, param6: ::win32_foundation_sys::LPARAM) -> ::windows_core_sys::HRESULT>;
#[cfg(feature = "Win32_Security_Authorization_UI")]
pub type PFNDSCREATEISECINFOEX = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core_sys::PCWSTR, param1: ::windows_core_sys::PCWSTR, param2: ::windows_core_sys::PCWSTR, param3: ::windows_core_sys::PCWSTR, param4: ::windows_core_sys::PCWSTR, param5: u32, param6: *mut super::Authorization::UI::ISecurityInformation, param7: PFNREADOBJECTSECURITY, param8: PFNWRITEOBJECTSECURITY, param9: ::win32_foundation_sys::LPARAM) -> ::windows_core_sys::HRESULT>;
#[cfg(feature = "Win32_UI_Controls")]
pub type PFNDSCREATESECPAGE = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core_sys::PCWSTR, param1: ::windows_core_sys::PCWSTR, param2: u32, param3: *mut ::win32_ui_sys::Controls::HPROPSHEETPAGE, param4: PFNREADOBJECTSECURITY, param5: PFNWRITEOBJECTSECURITY, param6: ::win32_foundation_sys::LPARAM) -> ::windows_core_sys::HRESULT>;
pub type PFNDSEDITSECURITY = ::core::option::Option<unsafe extern "system" fn(param0: ::win32_foundation_sys::HWND, param1: ::windows_core_sys::PCWSTR, param2: ::windows_core_sys::PCWSTR, param3: u32, param4: ::windows_core_sys::PCWSTR, param5: PFNREADOBJECTSECURITY, param6: PFNWRITEOBJECTSECURITY, param7: ::win32_foundation_sys::LPARAM) -> ::windows_core_sys::HRESULT>;
pub type PFNREADOBJECTSECURITY = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core_sys::PCWSTR, param1: u32, param2: *mut super::PSECURITY_DESCRIPTOR, param3: ::win32_foundation_sys::LPARAM) -> ::windows_core_sys::HRESULT>;
pub type PFNWRITEOBJECTSECURITY = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core_sys::PCWSTR, param1: u32, param2: super::PSECURITY_DESCRIPTOR, param3: ::win32_foundation_sys::LPARAM) -> ::windows_core_sys::HRESULT>;
