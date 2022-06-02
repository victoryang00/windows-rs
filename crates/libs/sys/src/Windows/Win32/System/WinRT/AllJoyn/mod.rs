#[repr(C)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromWin32Handle: unsafe extern "system" fn(this: *mut *mut Self, win32handle: u64, enableaboutdata: u8, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop {
    pub base__: ::windows_sys::core::IInspectable,
    pub Win32Handle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromWin32Handle: unsafe extern "system" fn(this: *mut *mut Self, win32handle: u64, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindowsDevicesAllJoynBusObjectInterop {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddPropertyGetHandler: unsafe extern "system" fn(this: *mut *mut Self, context: *const ::core::ffi::c_void, interfacename: ::windows_sys::core::HSTRING, callback: isize) -> ::windows_sys::core::HRESULT,
    pub AddPropertySetHandler: unsafe extern "system" fn(this: *mut *mut Self, context: *const ::core::ffi::c_void, interfacename: ::windows_sys::core::HSTRING, callback: isize) -> ::windows_sys::core::HRESULT,
    pub Win32Handle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut u64) -> ::windows_sys::core::HRESULT,
}
