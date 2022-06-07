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
impl ::windows_sys::core::Interface for IPwmControllerProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 318789947, data2: 58083, data3: 16548, data4: [183, 217, 72, 223, 240, 55, 122, 82] };
}
#[repr(C)]
pub struct IPwmProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllers: usize,
}
impl ::windows_sys::core::Interface for IPwmProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2737836584, data2: 21233, data3: 18352, data4: [147, 73, 102, 186, 67, 210, 89, 2] };
}
