#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
pub type CreateProcessMethod = i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
pub const CpCreateProcess: CreateProcessMethod = 0i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
pub const CpCreateProcessAsUser: CreateProcessMethod = 1i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
pub const CpAicLaunchAdminProcess: CreateProcessMethod = 2i32;
#[repr(C)]
pub struct IDDEInitializer {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_UI_Shell")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, fileextensionorprotocol: ::windows_sys::core::PCWSTR, method: CreateProcessMethod, currentdirectory: ::windows_sys::core::PCWSTR, exectarget: *mut ::core::ffi::c_void, site: *mut ::core::ffi::c_void, application: ::windows_sys::core::PCWSTR, targetfile: ::windows_sys::core::PCWSTR, arguments: ::windows_sys::core::PCWSTR, verb: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell"))]
    Initialize: usize,
}
impl ::windows_sys::core::Interface for IDDEInitializer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 819761951, data2: 13308, data3: 20477, data4: [161, 104, 148, 34, 88, 207, 60, 164] };
}
