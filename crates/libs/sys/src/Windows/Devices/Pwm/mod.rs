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
#[repr(C)]
pub struct IPwmControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Pwm_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Pwm_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
}
#[repr(C)]
pub struct IPwmControllerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
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
