#[repr(C)]
pub struct IPwmControllerProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub PinCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ActualFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDesiredFrequency: unsafe extern "system" fn(this: *mut *mut Self, frequency: f64, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MaxFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MinFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub AcquirePin: unsafe extern "system" fn(this: *mut *mut Self, pin: i32) -> ::windows_sys::core::HRESULT,
    pub ReleasePin: unsafe extern "system" fn(this: *mut *mut Self, pin: i32) -> ::windows_sys::core::HRESULT,
    pub EnablePin: unsafe extern "system" fn(this: *mut *mut Self, pin: i32) -> ::windows_sys::core::HRESULT,
    pub DisablePin: unsafe extern "system" fn(this: *mut *mut Self, pin: i32) -> ::windows_sys::core::HRESULT,
    pub SetPulseParameters: unsafe extern "system" fn(this: *mut *mut Self, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPwmProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllers: usize,
}
