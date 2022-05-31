#[link(name = "windows")]
extern "system" {
    pub fn CallNamedPipeA(lpnamedpipename: ::windows_core_sys::PCSTR, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> ::win32_foundation_sys::BOOL;
    pub fn CallNamedPipeW(lpnamedpipename: ::windows_core_sys::PCWSTR, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_System_IO")]
    pub fn ConnectNamedPipe(hnamedpipe: ::win32_foundation_sys::HANDLE, lpoverlapped: *mut super::IO::OVERLAPPED) -> ::win32_foundation_sys::BOOL;
    #[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
    pub fn CreateNamedPipeA(lpname: ::windows_core_sys::PCSTR, dwopenmode: ::win32_storage_sys::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES) -> ::win32_foundation_sys::HANDLE;
    #[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
    pub fn CreateNamedPipeW(lpname: ::windows_core_sys::PCWSTR, dwopenmode: ::win32_storage_sys::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "Win32_Security")]
    pub fn CreatePipe(hreadpipe: *mut ::win32_foundation_sys::HANDLE, hwritepipe: *mut ::win32_foundation_sys::HANDLE, lppipeattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, nsize: u32) -> ::win32_foundation_sys::BOOL;
    pub fn DisconnectNamedPipe(hnamedpipe: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn GetNamedPipeClientComputerNameA(pipe: ::win32_foundation_sys::HANDLE, clientcomputername: ::windows_core_sys::PSTR, clientcomputernamelength: u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetNamedPipeClientComputerNameW(pipe: ::win32_foundation_sys::HANDLE, clientcomputername: ::windows_core_sys::PWSTR, clientcomputernamelength: u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetNamedPipeClientProcessId(pipe: ::win32_foundation_sys::HANDLE, clientprocessid: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetNamedPipeClientSessionId(pipe: ::win32_foundation_sys::HANDLE, clientsessionid: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetNamedPipeHandleStateA(hnamedpipe: ::win32_foundation_sys::HANDLE, lpstate: *mut NAMED_PIPE_MODE, lpcurinstances: *mut u32, lpmaxcollectioncount: *mut u32, lpcollectdatatimeout: *mut u32, lpusername: ::windows_core_sys::PSTR, nmaxusernamesize: u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetNamedPipeHandleStateW(hnamedpipe: ::win32_foundation_sys::HANDLE, lpstate: *mut NAMED_PIPE_MODE, lpcurinstances: *mut u32, lpmaxcollectioncount: *mut u32, lpcollectdatatimeout: *mut u32, lpusername: ::windows_core_sys::PWSTR, nmaxusernamesize: u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetNamedPipeInfo(hnamedpipe: ::win32_foundation_sys::HANDLE, lpflags: *mut NAMED_PIPE_MODE, lpoutbuffersize: *mut u32, lpinbuffersize: *mut u32, lpmaxinstances: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetNamedPipeServerProcessId(pipe: ::win32_foundation_sys::HANDLE, serverprocessid: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetNamedPipeServerSessionId(pipe: ::win32_foundation_sys::HANDLE, serversessionid: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn ImpersonateNamedPipeClient(hnamedpipe: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn PeekNamedPipe(hnamedpipe: ::win32_foundation_sys::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nbuffersize: u32, lpbytesread: *mut u32, lptotalbytesavail: *mut u32, lpbytesleftthismessage: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetNamedPipeHandleState(hnamedpipe: ::win32_foundation_sys::HANDLE, lpmode: *const NAMED_PIPE_MODE, lpmaxcollectioncount: *const u32, lpcollectdatatimeout: *const u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_System_IO")]
    pub fn TransactNamedPipe(hnamedpipe: ::win32_foundation_sys::HANDLE, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, lpoverlapped: *mut super::IO::OVERLAPPED) -> ::win32_foundation_sys::BOOL;
    pub fn WaitNamedPipeA(lpnamedpipename: ::windows_core_sys::PCSTR, ntimeout: u32) -> ::win32_foundation_sys::BOOL;
    pub fn WaitNamedPipeW(lpnamedpipename: ::windows_core_sys::PCWSTR, ntimeout: u32) -> ::win32_foundation_sys::BOOL;
}
pub type NAMED_PIPE_MODE = u32;
pub const PIPE_WAIT: NAMED_PIPE_MODE = 0u32;
pub const PIPE_NOWAIT: NAMED_PIPE_MODE = 1u32;
pub const PIPE_READMODE_BYTE: NAMED_PIPE_MODE = 0u32;
pub const PIPE_READMODE_MESSAGE: NAMED_PIPE_MODE = 2u32;
pub const PIPE_CLIENT_END: NAMED_PIPE_MODE = 0u32;
pub const PIPE_SERVER_END: NAMED_PIPE_MODE = 1u32;
pub const PIPE_TYPE_BYTE: NAMED_PIPE_MODE = 0u32;
pub const PIPE_TYPE_MESSAGE: NAMED_PIPE_MODE = 4u32;
pub const PIPE_ACCEPT_REMOTE_CLIENTS: NAMED_PIPE_MODE = 0u32;
pub const PIPE_REJECT_REMOTE_CLIENTS: NAMED_PIPE_MODE = 8u32;
pub const NMPWAIT_NOWAIT: u32 = 1u32;
pub const NMPWAIT_USE_DEFAULT_WAIT: u32 = 0u32;
pub const NMPWAIT_WAIT_FOREVER: u32 = 4294967295u32;
pub const PIPE_UNLIMITED_INSTANCES: u32 = 255u32;
