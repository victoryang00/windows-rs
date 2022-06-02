#[cfg(feature = "Devices_Sensors_Custom")]
pub mod Custom;
pub type Accelerometer = *mut ::core::ffi::c_void;
pub type AccelerometerDataThreshold = *mut ::core::ffi::c_void;
pub type AccelerometerReading = *mut ::core::ffi::c_void;
pub type AccelerometerReadingChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct AccelerometerReadingType(pub i32);
impl AccelerometerReadingType {
    pub const Standard: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const Gravity: Self = Self(2i32);
}
impl ::core::marker::Copy for AccelerometerReadingType {}
impl ::core::clone::Clone for AccelerometerReadingType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AccelerometerShakenEventArgs = *mut ::core::ffi::c_void;
pub type ActivitySensor = *mut ::core::ffi::c_void;
pub type ActivitySensorReading = *mut ::core::ffi::c_void;
pub type ActivitySensorReadingChangeReport = *mut ::core::ffi::c_void;
pub type ActivitySensorReadingChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ActivitySensorReadingConfidence(pub i32);
impl ActivitySensorReadingConfidence {
    pub const High: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
}
impl ::core::marker::Copy for ActivitySensorReadingConfidence {}
impl ::core::clone::Clone for ActivitySensorReadingConfidence {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ActivitySensorTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct ActivityType(pub i32);
impl ActivityType {
    pub const Unknown: Self = Self(0i32);
    pub const Idle: Self = Self(1i32);
    pub const Stationary: Self = Self(2i32);
    pub const Fidgeting: Self = Self(3i32);
    pub const Walking: Self = Self(4i32);
    pub const Running: Self = Self(5i32);
    pub const InVehicle: Self = Self(6i32);
    pub const Biking: Self = Self(7i32);
}
impl ::core::marker::Copy for ActivityType {}
impl ::core::clone::Clone for ActivityType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Altimeter = *mut ::core::ffi::c_void;
pub type AltimeterReading = *mut ::core::ffi::c_void;
pub type AltimeterReadingChangedEventArgs = *mut ::core::ffi::c_void;
pub type Barometer = *mut ::core::ffi::c_void;
pub type BarometerDataThreshold = *mut ::core::ffi::c_void;
pub type BarometerReading = *mut ::core::ffi::c_void;
pub type BarometerReadingChangedEventArgs = *mut ::core::ffi::c_void;
pub type Compass = *mut ::core::ffi::c_void;
pub type CompassDataThreshold = *mut ::core::ffi::c_void;
pub type CompassReading = *mut ::core::ffi::c_void;
pub type CompassReadingChangedEventArgs = *mut ::core::ffi::c_void;
pub type Gyrometer = *mut ::core::ffi::c_void;
pub type GyrometerDataThreshold = *mut ::core::ffi::c_void;
pub type GyrometerReading = *mut ::core::ffi::c_void;
pub type GyrometerReadingChangedEventArgs = *mut ::core::ffi::c_void;
pub type HingeAngleReading = *mut ::core::ffi::c_void;
pub type HingeAngleSensor = *mut ::core::ffi::c_void;
pub type HingeAngleSensorReadingChangedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAccelerometer {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub Shaken: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Shaken: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShaken: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShaken: usize,
}
#[repr(C)]
pub struct IAccelerometer2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[repr(C)]
pub struct IAccelerometer3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccelerometer4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReadingType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AccelerometerReadingType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccelerometer5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccelerometerDataThreshold {
    pub base__: ::windows_sys::core::IInspectable,
    pub XAxisInGForce: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetXAxisInGForce: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub YAxisInGForce: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetYAxisInGForce: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ZAxisInGForce: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetZAxisInGForce: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccelerometerDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccelerometerReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub AccelerationX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub AccelerationY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub AccelerationZ: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccelerometerReading2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IAccelerometerReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccelerometerShakenEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[repr(C)]
pub struct IAccelerometerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccelerometerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultWithAccelerometerReadingType: unsafe extern "system" fn(this: *mut *mut Self, readingtype: AccelerometerReadingType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccelerometerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, readingtype: AccelerometerReadingType, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IActivitySensor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetCurrentReadingAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentReadingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SubscribedActivities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SubscribedActivities: usize,
    pub PowerInMilliwatts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedActivities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedActivities: usize,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[repr(C)]
pub struct IActivitySensorReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Activity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ActivityType) -> ::windows_sys::core::HRESULT,
    pub Confidence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ActivitySensorReadingConfidence) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IActivitySensorReadingChangeReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IActivitySensorReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IActivitySensorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryAsync: unsafe extern "system" fn(this: *mut *mut Self, fromtime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryWithDurationAsync: unsafe extern "system" fn(this: *mut *mut Self, fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryWithDurationAsync: usize,
}
#[repr(C)]
pub struct IActivitySensorTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
#[repr(C)]
pub struct IAltimeter {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[repr(C)]
pub struct IAltimeter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAltimeterReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub AltitudeChangeInMeters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAltimeterReading2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IAltimeterReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAltimeterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBarometer {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[repr(C)]
pub struct IBarometer2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBarometer3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBarometerDataThreshold {
    pub base__: ::windows_sys::core::IInspectable,
    pub Hectopascals: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHectopascals: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBarometerReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub StationPressureInHectopascals: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBarometerReading2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IBarometerReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBarometerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBarometerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompass {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[repr(C)]
pub struct ICompass2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[repr(C)]
pub struct ICompass3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompass4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompassDataThreshold {
    pub base__: ::windows_sys::core::IInspectable,
    pub Degrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompassDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompassReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub HeadingMagneticNorth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HeadingTrueNorth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeadingTrueNorth: usize,
}
#[repr(C)]
pub struct ICompassReading2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct ICompassReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompassReadingHeadingAccuracy {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeadingAccuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagnetometerAccuracy) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompassStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompassStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[repr(C)]
pub struct IGyrometer {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[repr(C)]
pub struct IGyrometer2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[repr(C)]
pub struct IGyrometer3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGyrometer4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGyrometerDataThreshold {
    pub base__: ::windows_sys::core::IInspectable,
    pub XAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetXAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub YAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetYAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ZAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetZAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGyrometerDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGyrometerReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub AngularVelocityX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub AngularVelocityY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub AngularVelocityZ: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGyrometerReading2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IGyrometerReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGyrometerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGyrometerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[repr(C)]
pub struct IHingeAngleReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub AngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IHingeAngleSensor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetCurrentReadingAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentReadingAsync: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MinReportThresholdInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ReportThresholdInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetReportThresholdInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[repr(C)]
pub struct IHingeAngleSensorReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHingeAngleSensorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetRelatedToAdjacentPanelsAsync: unsafe extern "system" fn(this: *mut *mut Self, firstpanelid: ::windows_sys::core::HSTRING, secondpanelid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRelatedToAdjacentPanelsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[repr(C)]
pub struct IInclinometer {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[repr(C)]
pub struct IInclinometer2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
    pub ReadingType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SensorReadingType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInclinometer3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInclinometer4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInclinometerDataThreshold {
    pub base__: ::windows_sys::core::IInspectable,
    pub PitchInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPitchInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub RollInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRollInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub YawInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetYawInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInclinometerDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInclinometerReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub PitchDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub RollDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub YawDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInclinometerReading2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IInclinometerReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInclinometerReadingYawAccuracy {
    pub base__: ::windows_sys::core::IInspectable,
    pub YawAccuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagnetometerAccuracy) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInclinometerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInclinometerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultForRelativeReadings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInclinometerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultWithSensorReadingType: unsafe extern "system" fn(this: *mut *mut Self, sensorreadingtype: SensorReadingType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInclinometerStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, readingtype: SensorReadingType, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[repr(C)]
pub struct ILightSensor {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[repr(C)]
pub struct ILightSensor2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILightSensor3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILightSensorDataThreshold {
    pub base__: ::windows_sys::core::IInspectable,
    pub LuxPercentage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetLuxPercentage: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub AbsoluteLux: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetAbsoluteLux: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILightSensorDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILightSensorReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub IlluminanceInLux: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILightSensorReading2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct ILightSensorReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILightSensorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILightSensorStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[repr(C)]
pub struct IMagnetometer {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[repr(C)]
pub struct IMagnetometer2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[repr(C)]
pub struct IMagnetometer3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMagnetometer4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMagnetometerDataThreshold {
    pub base__: ::windows_sys::core::IInspectable,
    pub XAxisMicroteslas: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetXAxisMicroteslas: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub YAxisMicroteslas: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetYAxisMicroteslas: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub ZAxisMicroteslas: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetZAxisMicroteslas: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMagnetometerDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMagnetometerReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub MagneticFieldX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub MagneticFieldY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub MagneticFieldZ: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub DirectionalAccuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagnetometerAccuracy) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMagnetometerReading2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IMagnetometerReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMagnetometerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMagnetometerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[repr(C)]
pub struct IOrientationSensor {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[repr(C)]
pub struct IOrientationSensor2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
    pub ReadingType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SensorReadingType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOrientationSensor3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOrientationSensorDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOrientationSensorReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub RotationMatrix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Quaternion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOrientationSensorReading2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IOrientationSensorReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOrientationSensorReadingYawAccuracy {
    pub base__: ::windows_sys::core::IInspectable,
    pub YawAccuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagnetometerAccuracy) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOrientationSensorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOrientationSensorStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultForRelativeReadings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOrientationSensorStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultWithSensorReadingType: unsafe extern "system" fn(this: *mut *mut Self, sensorreadingtype: SensorReadingType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal: unsafe extern "system" fn(this: *mut *mut Self, sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOrientationSensorStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, readingtype: SensorReadingType, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal: unsafe extern "system" fn(this: *mut *mut Self, readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[repr(C)]
pub struct IPedometer {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PowerInMilliwatts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[repr(C)]
pub struct IPedometer2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentReadings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentReadings: usize,
}
#[repr(C)]
pub struct IPedometerDataThresholdFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, sensor: *mut ::core::ffi::c_void, stepgoal: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPedometerReading {
    pub base__: ::windows_sys::core::IInspectable,
    pub StepKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PedometerStepKind) -> ::windows_sys::core::HRESULT,
    pub CumulativeSteps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Foundation")]
    pub CumulativeStepsDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CumulativeStepsDuration: usize,
}
#[repr(C)]
pub struct IPedometerReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPedometerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryAsync: unsafe extern "system" fn(this: *mut *mut Self, fromtime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryWithDurationAsync: unsafe extern "system" fn(this: *mut *mut Self, fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryWithDurationAsync: usize,
}
#[repr(C)]
pub struct IPedometerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetReadingsFromTriggerDetails: unsafe extern "system" fn(this: *mut *mut Self, triggerdetails: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetReadingsFromTriggerDetails: usize,
}
#[repr(C)]
pub struct IProximitySensor {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxDistanceInMillimeters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxDistanceInMillimeters: usize,
    #[cfg(feature = "Foundation")]
    pub MinDistanceInMillimeters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinDistanceInMillimeters: usize,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CreateDisplayOnOffController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDisplayOnOffController: usize,
}
#[repr(C)]
pub struct IProximitySensorDataThresholdFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, sensor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProximitySensorReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub IsDetected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DistanceInMillimeters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DistanceInMillimeters: usize,
}
#[repr(C)]
pub struct IProximitySensorReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProximitySensorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut *mut Self, sensorid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProximitySensorStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetReadingsFromTriggerDetails: unsafe extern "system" fn(this: *mut *mut Self, triggerdetails: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetReadingsFromTriggerDetails: usize,
}
#[repr(C)]
pub struct ISensorDataThreshold {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISensorDataThresholdTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SensorType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SensorType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISensorQuaternion {
    pub base__: ::windows_sys::core::IInspectable,
    pub W: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub X: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Y: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Z: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISensorRotationMatrix {
    pub base__: ::windows_sys::core::IInspectable,
    pub M11: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub M12: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub M13: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub M21: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub M22: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub M23: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub M31: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub M32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub M33: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISimpleOrientationSensor {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentOrientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SimpleOrientation) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OrientationChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OrientationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOrientationChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOrientationChanged: usize,
}
#[repr(C)]
pub struct ISimpleOrientationSensor2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[repr(C)]
pub struct ISimpleOrientationSensorDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISimpleOrientationSensorOrientationChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SimpleOrientation) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISimpleOrientationSensorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISimpleOrientationSensorStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
pub type Inclinometer = *mut ::core::ffi::c_void;
pub type InclinometerDataThreshold = *mut ::core::ffi::c_void;
pub type InclinometerReading = *mut ::core::ffi::c_void;
pub type InclinometerReadingChangedEventArgs = *mut ::core::ffi::c_void;
pub type LightSensor = *mut ::core::ffi::c_void;
pub type LightSensorDataThreshold = *mut ::core::ffi::c_void;
pub type LightSensorReading = *mut ::core::ffi::c_void;
pub type LightSensorReadingChangedEventArgs = *mut ::core::ffi::c_void;
pub type Magnetometer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct MagnetometerAccuracy(pub i32);
impl MagnetometerAccuracy {
    pub const Unknown: Self = Self(0i32);
    pub const Unreliable: Self = Self(1i32);
    pub const Approximate: Self = Self(2i32);
    pub const High: Self = Self(3i32);
}
impl ::core::marker::Copy for MagnetometerAccuracy {}
impl ::core::clone::Clone for MagnetometerAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MagnetometerDataThreshold = *mut ::core::ffi::c_void;
pub type MagnetometerReading = *mut ::core::ffi::c_void;
pub type MagnetometerReadingChangedEventArgs = *mut ::core::ffi::c_void;
pub type OrientationSensor = *mut ::core::ffi::c_void;
pub type OrientationSensorReading = *mut ::core::ffi::c_void;
pub type OrientationSensorReadingChangedEventArgs = *mut ::core::ffi::c_void;
pub type Pedometer = *mut ::core::ffi::c_void;
pub type PedometerDataThreshold = *mut ::core::ffi::c_void;
pub type PedometerReading = *mut ::core::ffi::c_void;
pub type PedometerReadingChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct PedometerStepKind(pub i32);
impl PedometerStepKind {
    pub const Unknown: Self = Self(0i32);
    pub const Walking: Self = Self(1i32);
    pub const Running: Self = Self(2i32);
}
impl ::core::marker::Copy for PedometerStepKind {}
impl ::core::clone::Clone for PedometerStepKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ProximitySensor = *mut ::core::ffi::c_void;
pub type ProximitySensorDataThreshold = *mut ::core::ffi::c_void;
pub type ProximitySensorDisplayOnOffController = *mut ::core::ffi::c_void;
pub type ProximitySensorReading = *mut ::core::ffi::c_void;
pub type ProximitySensorReadingChangedEventArgs = *mut ::core::ffi::c_void;
pub type SensorDataThresholdTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct SensorOptimizationGoal(pub i32);
impl SensorOptimizationGoal {
    pub const Precision: Self = Self(0i32);
    pub const PowerEfficiency: Self = Self(1i32);
}
impl ::core::marker::Copy for SensorOptimizationGoal {}
impl ::core::clone::Clone for SensorOptimizationGoal {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SensorQuaternion = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct SensorReadingType(pub i32);
impl SensorReadingType {
    pub const Absolute: Self = Self(0i32);
    pub const Relative: Self = Self(1i32);
}
impl ::core::marker::Copy for SensorReadingType {}
impl ::core::clone::Clone for SensorReadingType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SensorRotationMatrix = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct SensorType(pub i32);
impl SensorType {
    pub const Accelerometer: Self = Self(0i32);
    pub const ActivitySensor: Self = Self(1i32);
    pub const Barometer: Self = Self(2i32);
    pub const Compass: Self = Self(3i32);
    pub const CustomSensor: Self = Self(4i32);
    pub const Gyroscope: Self = Self(5i32);
    pub const ProximitySensor: Self = Self(6i32);
    pub const Inclinometer: Self = Self(7i32);
    pub const LightSensor: Self = Self(8i32);
    pub const OrientationSensor: Self = Self(9i32);
    pub const Pedometer: Self = Self(10i32);
    pub const RelativeInclinometer: Self = Self(11i32);
    pub const RelativeOrientationSensor: Self = Self(12i32);
    pub const SimpleOrientationSensor: Self = Self(13i32);
}
impl ::core::marker::Copy for SensorType {}
impl ::core::clone::Clone for SensorType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Sensors\"`*"]
#[repr(transparent)]
pub struct SimpleOrientation(pub i32);
impl SimpleOrientation {
    pub const NotRotated: Self = Self(0i32);
    pub const Rotated90DegreesCounterclockwise: Self = Self(1i32);
    pub const Rotated180DegreesCounterclockwise: Self = Self(2i32);
    pub const Rotated270DegreesCounterclockwise: Self = Self(3i32);
    pub const Faceup: Self = Self(4i32);
    pub const Facedown: Self = Self(5i32);
}
impl ::core::marker::Copy for SimpleOrientation {}
impl ::core::clone::Clone for SimpleOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SimpleOrientationSensor = *mut ::core::ffi::c_void;
pub type SimpleOrientationSensorOrientationChangedEventArgs = *mut ::core::ffi::c_void;
