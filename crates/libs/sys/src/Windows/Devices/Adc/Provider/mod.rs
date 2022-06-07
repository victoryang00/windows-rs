#[repr(C)]
pub struct IAdcControllerProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChannelCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ResolutionInBits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ChannelMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProviderAdcChannelMode) -> ::windows_sys::core::HRESULT,
    pub SetChannelMode: unsafe extern "system" fn(this: *mut *mut Self, value: ProviderAdcChannelMode) -> ::windows_sys::core::HRESULT,
    pub IsChannelModeSupported: unsafe extern "system" fn(this: *mut *mut Self, channelmode: ProviderAdcChannelMode, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AcquireChannel: unsafe extern "system" fn(this: *mut *mut Self, channel: i32) -> ::windows_sys::core::HRESULT,
    pub ReleaseChannel: unsafe extern "system" fn(this: *mut *mut Self, channel: i32) -> ::windows_sys::core::HRESULT,
    pub ReadValue: unsafe extern "system" fn(this: *mut *mut Self, channelnumber: i32, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdcControllerProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3193198632, data2: 33133, data3: 19941, data4: [160, 72, 171, 160, 105, 88, 170, 168] };
}
#[repr(C)]
pub struct IAdcProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllers: usize,
}
impl ::windows_sys::core::Interface for IAdcProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 680867432, data2: 37721, data3: 19543, data4: [188, 136, 226, 117, 232, 22, 56, 201] };
}
#[doc = "*Required features: `\"Devices_Adc_Provider\"`*"]
#[repr(transparent)]
pub struct ProviderAdcChannelMode(pub i32);
impl ProviderAdcChannelMode {
    pub const SingleEnded: Self = Self(0i32);
    pub const Differential: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderAdcChannelMode {}
impl ::core::clone::Clone for ProviderAdcChannelMode {
    fn clone(&self) -> Self {
        *self
    }
}
