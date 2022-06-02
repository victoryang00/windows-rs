pub type Battery = *mut ::core::ffi::c_void;
pub type BatteryReport = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBattery {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReportUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReportUpdated: usize,
}
#[repr(C)]
pub struct IBatteryReport {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ChargeRateInMilliwatts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChargeRateInMilliwatts: usize,
    #[cfg(feature = "Foundation")]
    pub DesignCapacityInMilliwattHours: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesignCapacityInMilliwattHours: usize,
    #[cfg(feature = "Foundation")]
    pub FullChargeCapacityInMilliwattHours: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FullChargeCapacityInMilliwattHours: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingCapacityInMilliwattHours: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingCapacityInMilliwattHours: usize,
    #[cfg(feature = "System_Power")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::Power::BatteryStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System_Power"))]
    Status: usize,
}
#[repr(C)]
pub struct IBatteryStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AggregateBattery: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
