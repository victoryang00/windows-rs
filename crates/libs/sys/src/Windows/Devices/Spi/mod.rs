#[cfg(feature = "Devices_Spi_Provider")]
pub mod Provider;
#[repr(C)]
pub struct ISpiBusInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChipSelectLineCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinClockFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxClockFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDataBitLengths: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDataBitLengths: usize,
}
#[repr(C)]
pub struct ISpiConnectionSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChipSelectLine: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetChipSelectLine: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpiMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: SpiMode) -> ::windows_sys::core::HRESULT,
    pub DataBitLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDataBitLength: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub ClockFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetClockFrequency: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpiSharingMode) -> ::windows_sys::core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut *mut Self, value: SpiSharingMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpiConnectionSettingsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, chipselectline: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpiController {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDevice: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpiControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(all(feature = "Devices_Spi_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Spi_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
}
#[repr(C)]
pub struct ISpiDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ConnectionSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *const u8) -> ::windows_sys::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *mut u8) -> ::windows_sys::core::HRESULT,
    pub TransferSequential: unsafe extern "system" fn(this: *mut *mut Self, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows_sys::core::HRESULT,
    pub TransferFullDuplex: unsafe extern "system" fn(this: *mut *mut Self, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpiDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, friendlyname: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetBusInfo: unsafe extern "system" fn(this: *mut *mut Self, busid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, busid: ::windows_sys::core::HSTRING, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
pub type SpiBusInfo = *mut ::core::ffi::c_void;
pub type SpiConnectionSettings = *mut ::core::ffi::c_void;
pub type SpiController = *mut ::core::ffi::c_void;
pub type SpiDevice = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Spi\"`*"]
#[repr(transparent)]
pub struct SpiMode(pub i32);
impl SpiMode {
    pub const Mode0: Self = Self(0i32);
    pub const Mode1: Self = Self(1i32);
    pub const Mode2: Self = Self(2i32);
    pub const Mode3: Self = Self(3i32);
}
impl ::core::marker::Copy for SpiMode {}
impl ::core::clone::Clone for SpiMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Spi\"`*"]
#[repr(transparent)]
pub struct SpiSharingMode(pub i32);
impl SpiSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for SpiSharingMode {}
impl ::core::clone::Clone for SpiSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
