#[inline]
pub unsafe fn BindIoCompletionCallback<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(filehandle: Param0, function: LPOVERLAPPED_COMPLETION_ROUTINE, flags: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BindIoCompletionCallback(filehandle: ::win32_foundation::HANDLE, function: ::windows_core::RawPtr, flags: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(BindIoCompletionCallback(filehandle.into_param().abi(), ::core::mem::transmute(function), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CancelIo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hfile: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelIo(hfile: ::win32_foundation::HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CancelIo(hfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CancelIoEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hfile: Param0, lpoverlapped: *const OVERLAPPED) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelIoEx(hfile: ::win32_foundation::HANDLE, lpoverlapped: *const OVERLAPPED) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CancelIoEx(hfile.into_param().abi(), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CancelSynchronousIo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hthread: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelSynchronousIo(hthread: ::win32_foundation::HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CancelSynchronousIo(hthread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateIoCompletionPort<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(filehandle: Param0, existingcompletionport: Param1, completionkey: usize, numberofconcurrentthreads: u32) -> ::windows_core::Result<::win32_foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIoCompletionPort(filehandle: ::win32_foundation::HANDLE, existingcompletionport: ::win32_foundation::HANDLE, completionkey: usize, numberofconcurrentthreads: u32) -> ::win32_foundation::HANDLE;
        }
        let result__ = CreateIoCompletionPort(filehandle.into_param().abi(), existingcompletionport.into_param().abi(), ::core::mem::transmute(completionkey), ::core::mem::transmute(numberofconcurrentthreads));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DeviceIoControl<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hdevice: Param0, dwiocontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut OVERLAPPED) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeviceIoControl(hdevice: ::win32_foundation::HANDLE, dwiocontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut OVERLAPPED) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DeviceIoControl(hdevice.into_param().abi(), ::core::mem::transmute(dwiocontrolcode), ::core::mem::transmute(lpinbuffer), ::core::mem::transmute(ninbuffersize), ::core::mem::transmute(lpoutbuffer), ::core::mem::transmute(noutbuffersize), ::core::mem::transmute(lpbytesreturned), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetOverlappedResult<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hfile: Param0, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: Param3) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOverlappedResult(hfile: ::win32_foundation::HANDLE, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetOverlappedResult(hfile.into_param().abi(), ::core::mem::transmute(lpoverlapped), ::core::mem::transmute(lpnumberofbytestransferred), bwait.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetOverlappedResultEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hfile: Param0, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, dwmilliseconds: u32, balertable: Param4) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOverlappedResultEx(hfile: ::win32_foundation::HANDLE, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, dwmilliseconds: u32, balertable: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetOverlappedResultEx(hfile.into_param().abi(), ::core::mem::transmute(lpoverlapped), ::core::mem::transmute(lpnumberofbytestransferred), ::core::mem::transmute(dwmilliseconds), balertable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetQueuedCompletionStatus<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(completionport: Param0, lpnumberofbytestransferred: *mut u32, lpcompletionkey: *mut usize, lpoverlapped: *mut *mut OVERLAPPED, dwmilliseconds: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetQueuedCompletionStatus(completionport: ::win32_foundation::HANDLE, lpnumberofbytestransferred: *mut u32, lpcompletionkey: *mut usize, lpoverlapped: *mut *mut OVERLAPPED, dwmilliseconds: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetQueuedCompletionStatus(completionport.into_param().abi(), ::core::mem::transmute(lpnumberofbytestransferred), ::core::mem::transmute(lpcompletionkey), ::core::mem::transmute(lpoverlapped), ::core::mem::transmute(dwmilliseconds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetQueuedCompletionStatusEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(completionport: Param0, lpcompletionportentries: &mut [OVERLAPPED_ENTRY], ulnumentriesremoved: *mut u32, dwmilliseconds: u32, falertable: Param5) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetQueuedCompletionStatusEx(completionport: ::win32_foundation::HANDLE, lpcompletionportentries: *mut OVERLAPPED_ENTRY, ulcount: u32, ulnumentriesremoved: *mut u32, dwmilliseconds: u32, falertable: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetQueuedCompletionStatusEx(completionport.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpcompletionportentries)), lpcompletionportentries.len() as _, ::core::mem::transmute(ulnumentriesremoved), ::core::mem::transmute(dwmilliseconds), falertable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type LPOVERLAPPED_COMPLETION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(dwerrorcode: u32, dwnumberofbytestransfered: u32, lpoverlapped: *mut OVERLAPPED)>;
#[repr(C)]
pub struct OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Anonymous: OVERLAPPED_0,
    pub hEvent: ::win32_foundation::HANDLE,
}
impl ::core::marker::Copy for OVERLAPPED {}
impl ::core::clone::Clone for OVERLAPPED {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for OVERLAPPED {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OVERLAPPED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OVERLAPPED>()) == 0 }
    }
}
impl ::core::cmp::Eq for OVERLAPPED {}
impl ::core::default::Default for OVERLAPPED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows_core::Abi for OVERLAPPED_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OVERLAPPED_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OVERLAPPED_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for OVERLAPPED_0 {}
impl ::core::default::Default for OVERLAPPED_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for OVERLAPPED_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OVERLAPPED_0_0").field("Offset", &self.Offset).field("OffsetHigh", &self.OffsetHigh).finish()
    }
}
unsafe impl ::windows_core::Abi for OVERLAPPED_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OVERLAPPED_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OVERLAPPED_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for OVERLAPPED_0_0 {}
impl ::core::default::Default for OVERLAPPED_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for OVERLAPPED_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OVERLAPPED_ENTRY").field("lpCompletionKey", &self.lpCompletionKey).field("lpOverlapped", &self.lpOverlapped).field("Internal", &self.Internal).field("dwNumberOfBytesTransferred", &self.dwNumberOfBytesTransferred).finish()
    }
}
unsafe impl ::windows_core::Abi for OVERLAPPED_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OVERLAPPED_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OVERLAPPED_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for OVERLAPPED_ENTRY {}
impl ::core::default::Default for OVERLAPPED_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn PostQueuedCompletionStatus<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(completionport: Param0, dwnumberofbytestransferred: u32, dwcompletionkey: usize, lpoverlapped: *const OVERLAPPED) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PostQueuedCompletionStatus(completionport: ::win32_foundation::HANDLE, dwnumberofbytestransferred: u32, dwcompletionkey: usize, lpoverlapped: *const OVERLAPPED) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(PostQueuedCompletionStatus(completionport.into_param().abi(), ::core::mem::transmute(dwnumberofbytestransferred), ::core::mem::transmute(dwcompletionkey), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
