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
impl ::windows_sys::core::Interface for IBattery {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3163115462, data2: 114, data3: 18376, data4: [139, 93, 97, 74, 170, 122, 67, 126] };
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
impl ::windows_sys::core::Interface for IBatteryReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3380972602, data2: 19987, data3: 16906, data4: [168, 208, 36, 241, 143, 57, 84, 1] };
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
impl ::windows_sys::core::Interface for IBatteryStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2043507382, data2: 40542, data3: 17490, data4: [190, 166, 223, 205, 84, 30, 89, 127] };
}
