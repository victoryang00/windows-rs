#[repr(C)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromWin32Handle: unsafe extern "system" fn(this: *mut *mut Self, win32handle: u64, enableaboutdata: u8, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1267692805, data2: 45625, data3: 20091, data4: [136, 175, 246, 104, 37, 117, 216, 97] };
}
#[repr(C)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop {
    pub base__: ::windows_sys::core::IInspectable,
    pub Win32Handle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWindowsDevicesAllJoynBusAttachmentInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4253664859, data2: 46350, data3: 18969, data4: [157, 12, 180, 43, 120, 50, 129, 205] };
}
#[repr(C)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromWin32Handle: unsafe extern "system" fn(this: *mut *mut Self, win32handle: u64, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1635050758, data2: 35733, data3: 20022, data4: [149, 192, 184, 143, 237, 52, 147, 140] };
}
#[repr(C)]
pub struct IWindowsDevicesAllJoynBusObjectInterop {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddPropertyGetHandler: unsafe extern "system" fn(this: *mut *mut Self, context: *const ::core::ffi::c_void, interfacename: ::windows_sys::core::HSTRING, callback: isize) -> ::windows_sys::core::HRESULT,
    pub AddPropertySetHandler: unsafe extern "system" fn(this: *mut *mut Self, context: *const ::core::ffi::c_void, interfacename: ::windows_sys::core::HSTRING, callback: isize) -> ::windows_sys::core::HRESULT,
    pub Win32Handle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWindowsDevicesAllJoynBusObjectInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3616187349, data2: 20564, data3: 17039, data4: [153, 242, 236, 58, 93, 227, 195, 188] };
}
