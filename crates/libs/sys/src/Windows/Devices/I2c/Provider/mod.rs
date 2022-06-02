#[repr(C)]
pub struct II2cControllerProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceProvider: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct II2cDeviceProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *const u8) -> ::windows_sys::core::HRESULT,
    pub WritePartial: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *const u8, result__: *mut ProviderI2cTransferResult) -> ::windows_sys::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ReadPartial: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> ::windows_sys::core::HRESULT,
    pub WriteRead: unsafe extern "system" fn(this: *mut *mut Self, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows_sys::core::HRESULT,
    pub WriteReadPartial: unsafe extern "system" fn(this: *mut *mut Self, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct II2cProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllersAsync: usize,
}
#[repr(C)]
pub struct IProviderI2cConnectionSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub SlaveAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSlaveAddress: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub BusSpeed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProviderI2cBusSpeed) -> ::windows_sys::core::HRESULT,
    pub SetBusSpeed: unsafe extern "system" fn(this: *mut *mut Self, value: ProviderI2cBusSpeed) -> ::windows_sys::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProviderI2cSharingMode) -> ::windows_sys::core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut *mut Self, value: ProviderI2cSharingMode) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
#[repr(transparent)]
pub struct ProviderI2cBusSpeed(pub i32);
impl ProviderI2cBusSpeed {
    pub const StandardMode: Self = Self(0i32);
    pub const FastMode: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderI2cBusSpeed {}
impl ::core::clone::Clone for ProviderI2cBusSpeed {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ProviderI2cConnectionSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
#[repr(transparent)]
pub struct ProviderI2cSharingMode(pub i32);
impl ProviderI2cSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderI2cSharingMode {}
impl ::core::clone::Clone for ProviderI2cSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
pub struct ProviderI2cTransferResult {
    pub Status: ProviderI2cTransferStatus,
    pub BytesTransferred: u32,
}
impl ::core::marker::Copy for ProviderI2cTransferResult {}
impl ::core::clone::Clone for ProviderI2cTransferResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
#[repr(transparent)]
pub struct ProviderI2cTransferStatus(pub i32);
impl ProviderI2cTransferStatus {
    pub const FullTransfer: Self = Self(0i32);
    pub const PartialTransfer: Self = Self(1i32);
    pub const SlaveAddressNotAcknowledged: Self = Self(2i32);
}
impl ::core::marker::Copy for ProviderI2cTransferStatus {}
impl ::core::clone::Clone for ProviderI2cTransferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
