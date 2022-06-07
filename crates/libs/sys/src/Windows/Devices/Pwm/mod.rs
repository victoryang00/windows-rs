#[cfg(feature = "Devices_Pwm_Provider")]
pub mod Provider;
#[repr(C)]
pub struct IPwmController {
    pub base__: ::windows_sys::core::IInspectable,
    pub PinCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ActualFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDesiredFrequency: unsafe extern "system" fn(this: *mut *mut Self, desiredfrequency: f64, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MinFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MaxFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OpenPin: unsafe extern "system" fn(this: *mut *mut Self, pinnumber: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPwmController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3294583941, data2: 53992, data3: 17103, data4: [155, 214, 207, 94, 208, 41, 230, 167] };
}
#[repr(C)]
pub struct IPwmControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Pwm_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Pwm_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
}
impl ::windows_sys::core::Interface for IPwmControllerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1113832865, data2: 35142, data3: 17412, data4: [189, 72, 129, 221, 18, 74, 244, 217] };
}
#[repr(C)]
pub struct IPwmControllerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
impl ::windows_sys::core::Interface for IPwmControllerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1157389087, data2: 61721, data3: 19421, data4: [151, 173, 247, 110, 249, 134, 115, 109] };
}
#[repr(C)]
pub struct IPwmControllerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, friendlyname: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
impl ::windows_sys::core::Interface for IPwmControllerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2992117873, data2: 553, data3: 17220, data4: [174, 63, 155, 124, 208, 230, 107, 148] };
}
#[repr(C)]
pub struct IPwmPin {
    pub base__: ::windows_sys::core::IInspectable,
    pub Controller: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetActiveDutyCyclePercentage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetActiveDutyCyclePercentage: unsafe extern "system" fn(this: *mut *mut Self, dutycyclepercentage: f64) -> ::windows_sys::core::HRESULT,
    pub Polarity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PwmPulsePolarity) -> ::windows_sys::core::HRESULT,
    pub SetPolarity: unsafe extern "system" fn(this: *mut *mut Self, value: PwmPulsePolarity) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub IsStarted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPwmPin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 580333000, data2: 50895, data3: 18465, data4: [183, 249, 198, 69, 79, 182, 175, 121] };
}
pub type PwmController = *mut ::core::ffi::c_void;
pub type PwmPin = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Pwm\"`*"]
#[repr(transparent)]
pub struct PwmPulsePolarity(pub i32);
impl PwmPulsePolarity {
    pub const ActiveHigh: Self = Self(0i32);
    pub const ActiveLow: Self = Self(1i32);
}
impl ::core::marker::Copy for PwmPulsePolarity {}
impl ::core::clone::Clone for PwmPulsePolarity {
    fn clone(&self) -> Self {
        *self
    }
}
