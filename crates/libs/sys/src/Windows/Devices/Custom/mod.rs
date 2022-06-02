pub type CustomDevice = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
pub struct DeviceAccessMode(pub i32);
impl DeviceAccessMode {
    pub const Read: Self = Self(0i32);
    pub const Write: Self = Self(1i32);
    pub const ReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for DeviceAccessMode {}
impl ::core::clone::Clone for DeviceAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
pub struct DeviceSharingMode(pub i32);
impl DeviceSharingMode {
    pub const Shared: Self = Self(0i32);
    pub const Exclusive: Self = Self(1i32);
}
impl ::core::marker::Copy for DeviceSharingMode {}
impl ::core::clone::Clone for DeviceSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICustomDevice {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendIOControlAsync: unsafe extern "system" fn(this: *mut *mut Self, iocontrolcode: *mut ::core::ffi::c_void, inputbuffer: *mut ::core::ffi::c_void, outputbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendIOControlAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TrySendIOControlAsync: unsafe extern "system" fn(this: *mut *mut Self, iocontrolcode: *mut ::core::ffi::c_void, inputbuffer: *mut ::core::ffi::c_void, outputbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TrySendIOControlAsync: usize,
}
#[repr(C)]
pub struct ICustomDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, classguid: ::windows_sys::core::GUID, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[repr(C)]
pub struct IIOControlCode {
    pub base__: ::windows_sys::core::IInspectable,
    pub AccessMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IOControlAccessMode) -> ::windows_sys::core::HRESULT,
    pub BufferingMethod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IOControlBufferingMethod) -> ::windows_sys::core::HRESULT,
    pub Function: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub DeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub ControlCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIOControlCodeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateIOControlCode: unsafe extern "system" fn(this: *mut *mut Self, devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKnownDeviceTypesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Unknown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
pub struct IOControlAccessMode(pub i32);
impl IOControlAccessMode {
    pub const Any: Self = Self(0i32);
    pub const Read: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
    pub const ReadWrite: Self = Self(3i32);
}
impl ::core::marker::Copy for IOControlAccessMode {}
impl ::core::clone::Clone for IOControlAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Custom\"`*"]
#[repr(transparent)]
pub struct IOControlBufferingMethod(pub i32);
impl IOControlBufferingMethod {
    pub const Buffered: Self = Self(0i32);
    pub const DirectInput: Self = Self(1i32);
    pub const DirectOutput: Self = Self(2i32);
    pub const Neither: Self = Self(3i32);
}
impl ::core::marker::Copy for IOControlBufferingMethod {}
impl ::core::clone::Clone for IOControlBufferingMethod {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IOControlCode = *mut ::core::ffi::c_void;
