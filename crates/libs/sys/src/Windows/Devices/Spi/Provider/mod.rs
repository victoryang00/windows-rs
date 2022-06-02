#[repr(C)]
pub struct IProviderSpiConnectionSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChipSelectLine: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetChipSelectLine: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProviderSpiMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: ProviderSpiMode) -> ::windows_sys::core::HRESULT,
    pub DataBitLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDataBitLength: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub ClockFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetClockFrequency: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProviderSpiSharingMode) -> ::windows_sys::core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut *mut Self, value: ProviderSpiSharingMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProviderSpiConnectionSettingsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, chipselectline: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpiControllerProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceProvider: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpiDeviceProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ConnectionSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *const u8) -> ::windows_sys::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *mut u8) -> ::windows_sys::core::HRESULT,
    pub TransferSequential: unsafe extern "system" fn(this: *mut *mut Self, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows_sys::core::HRESULT,
    pub TransferFullDuplex: unsafe extern "system" fn(this: *mut *mut Self, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpiProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllersAsync: usize,
}
pub type ProviderSpiConnectionSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Spi_Provider\"`*"]
#[repr(transparent)]
pub struct ProviderSpiMode(pub i32);
impl ProviderSpiMode {
    pub const Mode0: Self = Self(0i32);
    pub const Mode1: Self = Self(1i32);
    pub const Mode2: Self = Self(2i32);
    pub const Mode3: Self = Self(3i32);
}
impl ::core::marker::Copy for ProviderSpiMode {}
impl ::core::clone::Clone for ProviderSpiMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Spi_Provider\"`*"]
#[repr(transparent)]
pub struct ProviderSpiSharingMode(pub i32);
impl ProviderSpiSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderSpiSharingMode {}
impl ::core::clone::Clone for ProviderSpiSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
