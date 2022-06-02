#[cfg(feature = "Devices_I2c_Provider")]
pub mod Provider;
#[doc = "*Required features: `\"Devices_I2c\"`*"]
#[repr(transparent)]
pub struct I2cBusSpeed(pub i32);
impl I2cBusSpeed {
    pub const StandardMode: Self = Self(0i32);
    pub const FastMode: Self = Self(1i32);
}
impl ::core::marker::Copy for I2cBusSpeed {}
impl ::core::clone::Clone for I2cBusSpeed {
    fn clone(&self) -> Self {
        *self
    }
}
pub type I2cConnectionSettings = *mut ::core::ffi::c_void;
pub type I2cController = *mut ::core::ffi::c_void;
pub type I2cDevice = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_I2c\"`*"]
#[repr(transparent)]
pub struct I2cSharingMode(pub i32);
impl I2cSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for I2cSharingMode {}
impl ::core::clone::Clone for I2cSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_I2c\"`*"]
pub struct I2cTransferResult {
    pub Status: I2cTransferStatus,
    pub BytesTransferred: u32,
}
impl ::core::marker::Copy for I2cTransferResult {}
impl ::core::clone::Clone for I2cTransferResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_I2c\"`*"]
#[repr(transparent)]
pub struct I2cTransferStatus(pub i32);
impl I2cTransferStatus {
    pub const FullTransfer: Self = Self(0i32);
    pub const PartialTransfer: Self = Self(1i32);
    pub const SlaveAddressNotAcknowledged: Self = Self(2i32);
    pub const ClockStretchTimeout: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl ::core::marker::Copy for I2cTransferStatus {}
impl ::core::clone::Clone for I2cTransferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct II2cConnectionSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub SlaveAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSlaveAddress: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub BusSpeed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut I2cBusSpeed) -> ::windows_sys::core::HRESULT,
    pub SetBusSpeed: unsafe extern "system" fn(this: *mut *mut Self, value: I2cBusSpeed) -> ::windows_sys::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut I2cSharingMode) -> ::windows_sys::core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut *mut Self, value: I2cSharingMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct II2cConnectionSettingsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, slaveaddress: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct II2cController {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDevice: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct II2cControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_I2c_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_I2c_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[repr(C)]
pub struct II2cDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ConnectionSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *const u8) -> ::windows_sys::core::HRESULT,
    pub WritePartial: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *const u8, result__: *mut I2cTransferResult) -> ::windows_sys::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ReadPartial: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *mut u8, result__: *mut I2cTransferResult) -> ::windows_sys::core::HRESULT,
    pub WriteRead: unsafe extern "system" fn(this: *mut *mut Self, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows_sys::core::HRESULT,
    pub WriteReadPartial: unsafe extern "system" fn(this: *mut *mut Self, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8, result__: *mut I2cTransferResult) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct II2cDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, friendlyname: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
