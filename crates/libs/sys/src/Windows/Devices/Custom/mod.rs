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
impl ::windows_sys::core::Interface for ICustomDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3710919967, data2: 50315, data3: 17341, data4: [188, 177, 222, 200, 143, 21, 20, 62] };
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
impl ::windows_sys::core::Interface for ICustomDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3357672210, data2: 61260, data3: 18097, data4: [165, 142, 238, 179, 8, 220, 137, 23] };
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
impl ::windows_sys::core::Interface for IIOControlCode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 244668903, data2: 24776, data3: 17269, data4: [167, 97, 127, 136, 8, 6, 108, 96] };
}
#[repr(C)]
pub struct IIOControlCodeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateIOControlCode: unsafe extern "system" fn(this: *mut *mut Self, devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIOControlCodeFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2238348528, data2: 19473, data3: 17582, data4: [175, 198, 184, 212, 162, 18, 120, 143] };
}
#[repr(C)]
pub struct IKnownDeviceTypesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Unknown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownDeviceTypesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3998513602, data2: 21576, data3: 17882, data4: [173, 27, 36, 148, 140, 35, 144, 148] };
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
