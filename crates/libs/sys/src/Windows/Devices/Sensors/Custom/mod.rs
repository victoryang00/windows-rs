pub type CustomSensor = *mut ::core::ffi::c_void;
pub type CustomSensorReading = *mut ::core::ffi::c_void;
pub type CustomSensorReadingChangedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICustomSensor {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[repr(C)]
pub struct ICustomSensor2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICustomSensorReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct ICustomSensorReading2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
}
#[repr(C)]
pub struct ICustomSensorReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICustomSensorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, interfaceid: ::windows_sys::core::GUID, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, sensorid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
