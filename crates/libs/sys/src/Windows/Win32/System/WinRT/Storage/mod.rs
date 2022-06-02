#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub type HANDLE_ACCESS_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HAO_NONE: HANDLE_ACCESS_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HAO_READ_ATTRIBUTES: HANDLE_ACCESS_OPTIONS = 128u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HAO_READ: HANDLE_ACCESS_OPTIONS = 1179785u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HAO_WRITE: HANDLE_ACCESS_OPTIONS = 1179926u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HAO_DELETE: HANDLE_ACCESS_OPTIONS = 65536u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub type HANDLE_CREATION_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HCO_CREATE_NEW: HANDLE_CREATION_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HCO_CREATE_ALWAYS: HANDLE_CREATION_OPTIONS = 2i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HCO_OPEN_EXISTING: HANDLE_CREATION_OPTIONS = 3i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HCO_OPEN_ALWAYS: HANDLE_CREATION_OPTIONS = 4i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HCO_TRUNCATE_EXISTING: HANDLE_CREATION_OPTIONS = 5i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub type HANDLE_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_NONE: HANDLE_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_OPEN_REQUIRING_OPLOCK: HANDLE_OPTIONS = 262144u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_DELETE_ON_CLOSE: HANDLE_OPTIONS = 67108864u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_SEQUENTIAL_SCAN: HANDLE_OPTIONS = 134217728u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_RANDOM_ACCESS: HANDLE_OPTIONS = 268435456u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_NO_BUFFERING: HANDLE_OPTIONS = 536870912u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_OVERLAPPED: HANDLE_OPTIONS = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_WRITE_THROUGH: HANDLE_OPTIONS = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub type HANDLE_SHARING_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HSO_SHARE_NONE: HANDLE_SHARING_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HSO_SHARE_READ: HANDLE_SHARING_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HSO_SHARE_WRITE: HANDLE_SHARING_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HSO_SHARE_DELETE: HANDLE_SHARING_OPTIONS = 4u32;
#[repr(C)]
pub struct IOplockBreakingHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub OplockBreaking: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRandomAccessStreamFileAccessMode {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetMode: unsafe extern "system" fn(this: *mut *mut Self, fileaccessmode: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageFolderHandleAccess {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: *mut ::core::ffi::c_void, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
}
#[repr(C)]
pub struct IStorageItemHandleAccess {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: *mut ::core::ffi::c_void, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
}
#[repr(C)]
pub struct IUnbufferedFileHandleOplockCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnBrokenCallback: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUnbufferedFileHandleProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub OpenUnbufferedFileHandle: unsafe extern "system" fn(this: *mut *mut Self, oplockbreakcallback: *mut ::core::ffi::c_void, filehandle: *mut usize) -> ::windows_sys::core::HRESULT,
    pub CloseUnbufferedFileHandle: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
