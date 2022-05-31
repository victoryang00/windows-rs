#[link(name = "windows")]
extern "system" {
    pub fn BroadcastSystemMessageA(flags: u32, lpinfo: *mut u32, msg: u32, wparam: ::win32_foundation_sys::WPARAM, lparam: ::win32_foundation_sys::LPARAM) -> i32;
    pub fn BroadcastSystemMessageExA(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: ::win32_foundation_sys::WPARAM, lparam: ::win32_foundation_sys::LPARAM, pbsminfo: *mut BSMINFO) -> i32;
    pub fn BroadcastSystemMessageExW(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: ::win32_foundation_sys::WPARAM, lparam: ::win32_foundation_sys::LPARAM, pbsminfo: *mut BSMINFO) -> i32;
    pub fn BroadcastSystemMessageW(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: ::win32_foundation_sys::WPARAM, lparam: ::win32_foundation_sys::LPARAM) -> i32;
    pub fn CloseDesktop(hdesktop: HDESK) -> ::win32_foundation_sys::BOOL;
    pub fn CloseWindowStation(hwinsta: HWINSTA) -> ::win32_foundation_sys::BOOL;
    #[cfg(all(feature = "win32-graphics-sys", feature = "win32-security-sys"))]
    pub fn CreateDesktopA(lpszdesktop: ::windows_core_sys::PCSTR, lpszdevice: ::windows_core_sys::PCSTR, pdevmode: *mut ::win32_graphics_sys::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const ::win32_security_sys::SECURITY_ATTRIBUTES) -> HDESK;
    #[cfg(all(feature = "win32-graphics-sys", feature = "win32-security-sys"))]
    pub fn CreateDesktopExA(lpszdesktop: ::windows_core_sys::PCSTR, lpszdevice: ::windows_core_sys::PCSTR, pdevmode: *mut ::win32_graphics_sys::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const ::win32_security_sys::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> HDESK;
    #[cfg(all(feature = "win32-graphics-sys", feature = "win32-security-sys"))]
    pub fn CreateDesktopExW(lpszdesktop: ::windows_core_sys::PCWSTR, lpszdevice: ::windows_core_sys::PCWSTR, pdevmode: *mut ::win32_graphics_sys::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const ::win32_security_sys::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> HDESK;
    #[cfg(all(feature = "win32-graphics-sys", feature = "win32-security-sys"))]
    pub fn CreateDesktopW(lpszdesktop: ::windows_core_sys::PCWSTR, lpszdevice: ::windows_core_sys::PCWSTR, pdevmode: *mut ::win32_graphics_sys::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const ::win32_security_sys::SECURITY_ATTRIBUTES) -> HDESK;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateWindowStationA(lpwinsta: ::windows_core_sys::PCSTR, dwflags: u32, dwdesiredaccess: u32, lpsa: *const ::win32_security_sys::SECURITY_ATTRIBUTES) -> HWINSTA;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateWindowStationW(lpwinsta: ::windows_core_sys::PCWSTR, dwflags: u32, dwdesiredaccess: u32, lpsa: *const ::win32_security_sys::SECURITY_ATTRIBUTES) -> HWINSTA;
    #[cfg(feature = "win32-ui-sys")]
    pub fn EnumDesktopWindows(hdesktop: HDESK, lpfn: ::win32_ui_sys::WindowsAndMessaging::WNDENUMPROC, lparam: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL;
    pub fn EnumDesktopsA(hwinsta: HWINSTA, lpenumfunc: DESKTOPENUMPROCA, lparam: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL;
    pub fn EnumDesktopsW(hwinsta: HWINSTA, lpenumfunc: DESKTOPENUMPROCW, lparam: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL;
    pub fn EnumWindowStationsA(lpenumfunc: WINSTAENUMPROCA, lparam: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL;
    pub fn EnumWindowStationsW(lpenumfunc: WINSTAENUMPROCW, lparam: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL;
    pub fn GetProcessWindowStation() -> HWINSTA;
    pub fn GetThreadDesktop(dwthreadid: u32) -> HDESK;
    pub fn GetUserObjectInformationA(hobj: ::win32_foundation_sys::HANDLE, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetUserObjectInformationW(hobj: ::win32_foundation_sys::HANDLE, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn OpenDesktopA(lpszdesktop: ::windows_core_sys::PCSTR, dwflags: u32, finherit: ::win32_foundation_sys::BOOL, dwdesiredaccess: u32) -> HDESK;
    pub fn OpenDesktopW(lpszdesktop: ::windows_core_sys::PCWSTR, dwflags: u32, finherit: ::win32_foundation_sys::BOOL, dwdesiredaccess: u32) -> HDESK;
    pub fn OpenInputDesktop(dwflags: u32, finherit: ::win32_foundation_sys::BOOL, dwdesiredaccess: u32) -> HDESK;
    pub fn OpenWindowStationA(lpszwinsta: ::windows_core_sys::PCSTR, finherit: ::win32_foundation_sys::BOOL, dwdesiredaccess: u32) -> HWINSTA;
    pub fn OpenWindowStationW(lpszwinsta: ::windows_core_sys::PCWSTR, finherit: ::win32_foundation_sys::BOOL, dwdesiredaccess: u32) -> HWINSTA;
    pub fn SetProcessWindowStation(hwinsta: HWINSTA) -> ::win32_foundation_sys::BOOL;
    pub fn SetThreadDesktop(hdesktop: HDESK) -> ::win32_foundation_sys::BOOL;
    pub fn SetUserObjectInformationA(hobj: ::win32_foundation_sys::HANDLE, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetUserObjectInformationW(hobj: ::win32_foundation_sys::HANDLE, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SwitchDesktop(hdesktop: HDESK) -> ::win32_foundation_sys::BOOL;
}
pub type BROADCAST_SYSTEM_MESSAGE_FLAGS = u32;
pub const BSF_ALLOWSFW: BROADCAST_SYSTEM_MESSAGE_FLAGS = 128u32;
pub const BSF_FLUSHDISK: BROADCAST_SYSTEM_MESSAGE_FLAGS = 4u32;
pub const BSF_FORCEIFHUNG: BROADCAST_SYSTEM_MESSAGE_FLAGS = 32u32;
pub const BSF_IGNORECURRENTTASK: BROADCAST_SYSTEM_MESSAGE_FLAGS = 2u32;
pub const BSF_NOHANG: BROADCAST_SYSTEM_MESSAGE_FLAGS = 8u32;
pub const BSF_NOTIMEOUTIFNOTHUNG: BROADCAST_SYSTEM_MESSAGE_FLAGS = 64u32;
pub const BSF_POSTMESSAGE: BROADCAST_SYSTEM_MESSAGE_FLAGS = 16u32;
pub const BSF_QUERY: BROADCAST_SYSTEM_MESSAGE_FLAGS = 1u32;
pub const BSF_SENDNOTIFYMESSAGE: BROADCAST_SYSTEM_MESSAGE_FLAGS = 256u32;
pub const BSF_LUID: BROADCAST_SYSTEM_MESSAGE_FLAGS = 1024u32;
pub const BSF_RETURNHDESK: BROADCAST_SYSTEM_MESSAGE_FLAGS = 512u32;
pub type BROADCAST_SYSTEM_MESSAGE_INFO = u32;
pub const BSM_ALLCOMPONENTS: BROADCAST_SYSTEM_MESSAGE_INFO = 0u32;
pub const BSM_ALLDESKTOPS: BROADCAST_SYSTEM_MESSAGE_INFO = 16u32;
pub const BSM_APPLICATIONS: BROADCAST_SYSTEM_MESSAGE_INFO = 8u32;
#[repr(C)]
pub struct BSMINFO {
    pub cbSize: u32,
    pub hdesk: HDESK,
    pub hwnd: ::win32_foundation_sys::HWND,
    pub luid: ::win32_foundation_sys::LUID,
}
impl ::core::marker::Copy for BSMINFO {}
impl ::core::clone::Clone for BSMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DESKTOPENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core_sys::PCSTR, param1: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL>;
pub type DESKTOPENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core_sys::PCWSTR, param1: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL>;
pub type HDESK = isize;
pub type HWINSTA = isize;
#[repr(C)]
pub struct USEROBJECTFLAGS {
    pub fInherit: ::win32_foundation_sys::BOOL,
    pub fReserved: ::win32_foundation_sys::BOOL,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for USEROBJECTFLAGS {}
impl ::core::clone::Clone for USEROBJECTFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type USER_OBJECT_INFORMATION_INDEX = u32;
pub const UOI_FLAGS: USER_OBJECT_INFORMATION_INDEX = 1u32;
pub const UOI_HEAPSIZE: USER_OBJECT_INFORMATION_INDEX = 5u32;
pub const UOI_IO: USER_OBJECT_INFORMATION_INDEX = 6u32;
pub const UOI_NAME: USER_OBJECT_INFORMATION_INDEX = 2u32;
pub const UOI_TYPE: USER_OBJECT_INFORMATION_INDEX = 3u32;
pub const UOI_USER_SID: USER_OBJECT_INFORMATION_INDEX = 4u32;
pub type WINSTAENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core_sys::PCSTR, param1: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL>;
pub type WINSTAENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core_sys::PCWSTR, param1: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL>;
