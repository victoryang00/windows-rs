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
impl ::windows_sys::core::Interface for IProviderSpiConnectionSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4127409488, data2: 42306, data3: 20160, data4: [150, 1, 164, 221, 104, 248, 105, 123] };
}
#[repr(C)]
pub struct IProviderSpiConnectionSettingsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, chipselectline: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProviderSpiConnectionSettingsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1715825498, data2: 3193, data3: 17379, data4: [159, 60, 229, 151, 128, 172, 24, 250] };
}
#[repr(C)]
pub struct ISpiControllerProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceProvider: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpiControllerProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3244844292, data2: 718, data3: 16934, data4: [163, 133, 79, 17, 251, 4, 180, 27] };
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
impl ::windows_sys::core::Interface for ISpiDeviceProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 219952195, data2: 12363, data3: 16476, data4: [180, 247, 245, 171, 16, 116, 70, 30] };
}
#[repr(C)]
pub struct ISpiProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllersAsync: usize,
}
impl ::windows_sys::core::Interface for ISpiProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2528403938, data2: 30676, data3: 18638, data4: [170, 160, 117, 113, 90, 131, 98, 207] };
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
