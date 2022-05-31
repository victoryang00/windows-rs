#[link(name = "windows")]
extern "system" {
    pub fn BindIoCompletionCallback(filehandle: ::win32_foundation_sys::HANDLE, function: LPOVERLAPPED_COMPLETION_ROUTINE, flags: u32) -> ::win32_foundation_sys::BOOL;
    pub fn CancelIo(hfile: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn CancelIoEx(hfile: ::win32_foundation_sys::HANDLE, lpoverlapped: *const OVERLAPPED) -> ::win32_foundation_sys::BOOL;
    pub fn CancelSynchronousIo(hthread: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn CreateIoCompletionPort(filehandle: ::win32_foundation_sys::HANDLE, existingcompletionport: ::win32_foundation_sys::HANDLE, completionkey: usize, numberofconcurrentthreads: u32) -> ::win32_foundation_sys::HANDLE;
    pub fn DeviceIoControl(hdevice: ::win32_foundation_sys::HANDLE, dwiocontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut OVERLAPPED) -> ::win32_foundation_sys::BOOL;
    pub fn GetOverlappedResult(hfile: ::win32_foundation_sys::HANDLE, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn GetOverlappedResultEx(hfile: ::win32_foundation_sys::HANDLE, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, dwmilliseconds: u32, balertable: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn GetQueuedCompletionStatus(completionport: ::win32_foundation_sys::HANDLE, lpnumberofbytestransferred: *mut u32, lpcompletionkey: *mut usize, lpoverlapped: *mut *mut OVERLAPPED, dwmilliseconds: u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetQueuedCompletionStatusEx(completionport: ::win32_foundation_sys::HANDLE, lpcompletionportentries: *mut OVERLAPPED_ENTRY, ulcount: u32, ulnumentriesremoved: *mut u32, dwmilliseconds: u32, falertable: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn PostQueuedCompletionStatus(completionport: ::win32_foundation_sys::HANDLE, dwnumberofbytestransferred: u32, dwcompletionkey: usize, lpoverlapped: *const OVERLAPPED) -> ::win32_foundation_sys::BOOL;
}
pub type LPOVERLAPPED_COMPLETION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(dwerrorcode: u32, dwnumberofbytestransfered: u32, lpoverlapped: *mut OVERLAPPED)>;
#[repr(C)]
pub struct OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Anonymous: OVERLAPPED_0,
    pub hEvent: ::win32_foundation_sys::HANDLE,
}
impl ::core::marker::Copy for OVERLAPPED {}
impl ::core::clone::Clone for OVERLAPPED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union OVERLAPPED_0 {
    pub Anonymous: OVERLAPPED_0_0,
    pub Pointer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for OVERLAPPED_0 {}
impl ::core::clone::Clone for OVERLAPPED_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OVERLAPPED_0_0 {
    pub Offset: u32,
    pub OffsetHigh: u32,
}
impl ::core::marker::Copy for OVERLAPPED_0_0 {}
impl ::core::clone::Clone for OVERLAPPED_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OVERLAPPED_ENTRY {
    pub lpCompletionKey: usize,
    pub lpOverlapped: *mut OVERLAPPED,
    pub Internal: usize,
    pub dwNumberOfBytesTransferred: u32,
}
impl ::core::marker::Copy for OVERLAPPED_ENTRY {}
impl ::core::clone::Clone for OVERLAPPED_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
