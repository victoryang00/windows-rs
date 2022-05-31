#[link(name = "windows")]
extern "system" {
    pub fn CreateToolhelp32Snapshot(dwflags: CREATE_TOOLHELP_SNAPSHOT_FLAGS, th32processid: u32) -> ::win32_foundation_sys::HANDLE;
    pub fn Heap32First(lphe: *mut HEAPENTRY32, th32processid: u32, th32heapid: usize) -> ::win32_foundation_sys::BOOL;
    pub fn Heap32ListFirst(hsnapshot: ::win32_foundation_sys::HANDLE, lphl: *mut HEAPLIST32) -> ::win32_foundation_sys::BOOL;
    pub fn Heap32ListNext(hsnapshot: ::win32_foundation_sys::HANDLE, lphl: *mut HEAPLIST32) -> ::win32_foundation_sys::BOOL;
    pub fn Heap32Next(lphe: *mut HEAPENTRY32) -> ::win32_foundation_sys::BOOL;
    pub fn Module32First(hsnapshot: ::win32_foundation_sys::HANDLE, lpme: *mut MODULEENTRY32) -> ::win32_foundation_sys::BOOL;
    pub fn Module32FirstW(hsnapshot: ::win32_foundation_sys::HANDLE, lpme: *mut MODULEENTRY32W) -> ::win32_foundation_sys::BOOL;
    pub fn Module32Next(hsnapshot: ::win32_foundation_sys::HANDLE, lpme: *mut MODULEENTRY32) -> ::win32_foundation_sys::BOOL;
    pub fn Module32NextW(hsnapshot: ::win32_foundation_sys::HANDLE, lpme: *mut MODULEENTRY32W) -> ::win32_foundation_sys::BOOL;
    pub fn Process32First(hsnapshot: ::win32_foundation_sys::HANDLE, lppe: *mut PROCESSENTRY32) -> ::win32_foundation_sys::BOOL;
    pub fn Process32FirstW(hsnapshot: ::win32_foundation_sys::HANDLE, lppe: *mut PROCESSENTRY32W) -> ::win32_foundation_sys::BOOL;
    pub fn Process32Next(hsnapshot: ::win32_foundation_sys::HANDLE, lppe: *mut PROCESSENTRY32) -> ::win32_foundation_sys::BOOL;
    pub fn Process32NextW(hsnapshot: ::win32_foundation_sys::HANDLE, lppe: *mut PROCESSENTRY32W) -> ::win32_foundation_sys::BOOL;
    pub fn Thread32First(hsnapshot: ::win32_foundation_sys::HANDLE, lpte: *mut THREADENTRY32) -> ::win32_foundation_sys::BOOL;
    pub fn Thread32Next(hsnapshot: ::win32_foundation_sys::HANDLE, lpte: *mut THREADENTRY32) -> ::win32_foundation_sys::BOOL;
    pub fn Toolhelp32ReadProcessMemory(th32processid: u32, lpbaseaddress: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, cbread: usize, lpnumberofbytesread: *mut usize) -> ::win32_foundation_sys::BOOL;
}
pub type CREATE_TOOLHELP_SNAPSHOT_FLAGS = u32;
pub const TH32CS_INHERIT: CREATE_TOOLHELP_SNAPSHOT_FLAGS = 2147483648u32;
pub const TH32CS_SNAPALL: CREATE_TOOLHELP_SNAPSHOT_FLAGS = 15u32;
pub const TH32CS_SNAPHEAPLIST: CREATE_TOOLHELP_SNAPSHOT_FLAGS = 1u32;
pub const TH32CS_SNAPMODULE: CREATE_TOOLHELP_SNAPSHOT_FLAGS = 8u32;
pub const TH32CS_SNAPMODULE32: CREATE_TOOLHELP_SNAPSHOT_FLAGS = 16u32;
pub const TH32CS_SNAPPROCESS: CREATE_TOOLHELP_SNAPSHOT_FLAGS = 2u32;
pub const TH32CS_SNAPTHREAD: CREATE_TOOLHELP_SNAPSHOT_FLAGS = 4u32;
#[repr(C)]
pub struct HEAPENTRY32 {
    pub dwSize: usize,
    pub hHandle: ::win32_foundation_sys::HANDLE,
    pub dwAddress: usize,
    pub dwBlockSize: usize,
    pub dwFlags: HEAPENTRY32_FLAGS,
    pub dwLockCount: u32,
    pub dwResvd: u32,
    pub th32ProcessID: u32,
    pub th32HeapID: usize,
}
impl ::core::marker::Copy for HEAPENTRY32 {}
impl ::core::clone::Clone for HEAPENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HEAPENTRY32_FLAGS = u32;
pub const LF32_FIXED: HEAPENTRY32_FLAGS = 1u32;
pub const LF32_FREE: HEAPENTRY32_FLAGS = 2u32;
pub const LF32_MOVEABLE: HEAPENTRY32_FLAGS = 4u32;
#[repr(C)]
pub struct HEAPLIST32 {
    pub dwSize: usize,
    pub th32ProcessID: u32,
    pub th32HeapID: usize,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for HEAPLIST32 {}
impl ::core::clone::Clone for HEAPLIST32 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HF32_DEFAULT: u32 = 1u32;
pub const HF32_SHARED: u32 = 2u32;
pub const MAX_MODULE_NAME32: u32 = 255u32;
#[repr(C)]
pub struct MODULEENTRY32 {
    pub dwSize: u32,
    pub th32ModuleID: u32,
    pub th32ProcessID: u32,
    pub GlblcntUsage: u32,
    pub ProccntUsage: u32,
    pub modBaseAddr: *mut u8,
    pub modBaseSize: u32,
    pub hModule: ::win32_foundation_sys::HINSTANCE,
    pub szModule: [::win32_foundation_sys::CHAR; 256],
    pub szExePath: [::win32_foundation_sys::CHAR; 260],
}
impl ::core::marker::Copy for MODULEENTRY32 {}
impl ::core::clone::Clone for MODULEENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MODULEENTRY32W {
    pub dwSize: u32,
    pub th32ModuleID: u32,
    pub th32ProcessID: u32,
    pub GlblcntUsage: u32,
    pub ProccntUsage: u32,
    pub modBaseAddr: *mut u8,
    pub modBaseSize: u32,
    pub hModule: ::win32_foundation_sys::HINSTANCE,
    pub szModule: [u16; 256],
    pub szExePath: [u16; 260],
}
impl ::core::marker::Copy for MODULEENTRY32W {}
impl ::core::clone::Clone for MODULEENTRY32W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESSENTRY32 {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ProcessID: u32,
    pub th32DefaultHeapID: usize,
    pub th32ModuleID: u32,
    pub cntThreads: u32,
    pub th32ParentProcessID: u32,
    pub pcPriClassBase: i32,
    pub dwFlags: u32,
    pub szExeFile: [::win32_foundation_sys::CHAR; 260],
}
impl ::core::marker::Copy for PROCESSENTRY32 {}
impl ::core::clone::Clone for PROCESSENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESSENTRY32W {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ProcessID: u32,
    pub th32DefaultHeapID: usize,
    pub th32ModuleID: u32,
    pub cntThreads: u32,
    pub th32ParentProcessID: u32,
    pub pcPriClassBase: i32,
    pub dwFlags: u32,
    pub szExeFile: [u16; 260],
}
impl ::core::marker::Copy for PROCESSENTRY32W {}
impl ::core::clone::Clone for PROCESSENTRY32W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct THREADENTRY32 {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ThreadID: u32,
    pub th32OwnerProcessID: u32,
    pub tpBasePri: i32,
    pub tpDeltaPri: i32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for THREADENTRY32 {}
impl ::core::clone::Clone for THREADENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
