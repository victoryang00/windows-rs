pub type Battery = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBattery {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemainingChargePercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemainingDischargeTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingDischargeTime: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingChargePercentChanged: unsafe extern "system" fn(this: *mut *mut Self, changehandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingChargePercentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemainingChargePercentChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemainingChargePercentChanged: usize,
}
#[repr(C)]
pub struct IBatteryStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
