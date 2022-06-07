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
impl ::windows_sys::core::Interface for ICustomSensor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2704734637, data2: 16436, data3: 19277, data4: [153, 221, 83, 26, 172, 100, 156, 9] };
}
#[repr(C)]
pub struct ICustomSensor2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICustomSensor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 551235857, data2: 60504, data3: 19871, data4: [191, 189, 231, 120, 37, 8, 133, 16] };
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
impl ::windows_sys::core::Interface for ICustomSensorReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1677741901, data2: 17514, data3: 17254, data4: [168, 122, 95, 150, 50, 104, 236, 83] };
}
#[repr(C)]
pub struct ICustomSensorReading2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
}
impl ::windows_sys::core::Interface for ICustomSensorReading2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 574396650, data2: 49011, data3: 18834, data4: [154, 72, 211, 200, 151, 89, 76, 203] };
}
#[repr(C)]
pub struct ICustomSensorReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICustomSensorReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1797267491, data2: 53245, data3: 19649, data4: [143, 240, 226, 24, 35, 215, 111, 204] };
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
impl ::windows_sys::core::Interface for ICustomSensorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2569032399, data2: 62498, data3: 19581, data4: [131, 107, 231, 220, 116, 167, 18, 75] };
}
