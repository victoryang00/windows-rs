#[cfg(feature = "Devices_Adc_Provider")]
pub mod Provider;
pub type AdcChannel = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Adc\"`*"]
#[repr(transparent)]
pub struct AdcChannelMode(pub i32);
impl AdcChannelMode {
    pub const SingleEnded: Self = Self(0i32);
    pub const Differential: Self = Self(1i32);
}
impl ::core::marker::Copy for AdcChannelMode {}
impl ::core::clone::Clone for AdcChannelMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AdcController = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAdcChannel {
    pub base__: ::windows_sys::core::IInspectable,
    pub Controller: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReadValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ReadRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdcChannel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 67892244, data2: 9608, data3: 19030, data4: [171, 239, 115, 162, 96, 172, 198, 10] };
}
#[repr(C)]
pub struct IAdcController {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChannelCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ResolutionInBits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ChannelMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdcChannelMode) -> ::windows_sys::core::HRESULT,
    pub SetChannelMode: unsafe extern "system" fn(this: *mut *mut Self, value: AdcChannelMode) -> ::windows_sys::core::HRESULT,
    pub IsChannelModeSupported: unsafe extern "system" fn(this: *mut *mut Self, channelmode: AdcChannelMode, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub OpenChannel: unsafe extern "system" fn(this: *mut *mut Self, channelnumber: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdcController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 712434864, data2: 43158, data3: 16921, data4: [134, 182, 234, 140, 220, 233, 143, 86] };
}
#[repr(C)]
pub struct IAdcControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Adc_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
}
impl ::windows_sys::core::Interface for IAdcControllerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3437858316, data2: 504, data3: 18577, data4: [188, 59, 190, 83, 239, 39, 156, 164] };
}
#[repr(C)]
pub struct IAdcControllerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
impl ::windows_sys::core::Interface for IAdcControllerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2730048285, data2: 38779, data3: 20314, data4: [165, 254, 166, 171, 175, 254, 100, 132] };
}
