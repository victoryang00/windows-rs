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
impl ::windows_sys::core::Interface for IAccelerometer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3742909768, data2: 10001, data3: 19879, data4: [128, 152, 75, 130, 32, 93, 60, 125] };
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
impl ::windows_sys::core::Interface for IAccelerometer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3908080366, data2: 18788, data3: 16410, data4: [182, 2, 34, 13, 113, 83, 198, 10] };
}
#[repr(C)]
pub struct IAccelerometer3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccelerometer3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2279604778, data2: 60800, data3: 18923, data4: [191, 138, 164, 234, 49, 229, 205, 132] };
}
#[repr(C)]
pub struct IAccelerometer4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReadingType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AccelerometerReadingType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccelerometer4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 490159183, data2: 17107, data3: 17842, data4: [129, 68, 171, 127, 182, 101, 235, 89] };
}
#[repr(C)]
pub struct IAccelerometer5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccelerometer5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2122215457, data2: 57076, data3: 21414, data4: [175, 67, 128, 111, 213, 56, 237, 246] };
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
impl ::windows_sys::core::Interface for IAccelerometerDataThreshold {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4180417384, data2: 25376, data3: 21879, data4: [135, 158, 153, 66, 98, 28, 61, 217] };
}
#[repr(C)]
pub struct IAccelerometerDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccelerometerDeviceId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2125227177, data2: 38869, data3: 17517, data4: [171, 90, 145, 125, 249, 185, 106, 44] };
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
impl ::windows_sys::core::Interface for IAccelerometerReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3120462539, data2: 54097, data3: 16559, data4: [139, 182, 122, 169, 174, 100, 31, 183] };
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
impl ::windows_sys::core::Interface for IAccelerometerReading2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 176573090, data2: 5550, data3: 19008, data4: [190, 85, 219, 88, 215, 222, 115, 137] };
}
#[repr(C)]
pub struct IAccelerometerReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccelerometerReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 9815643, data2: 46764, data3: 18266, data4: [159, 68, 139, 50, 211, 90, 63, 37] };
}
#[repr(C)]
pub struct IAccelerometerShakenEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
impl ::windows_sys::core::Interface for IAccelerometerShakenEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2516517329, data2: 18984, data3: 20277, data4: [152, 232, 129, 120, 170, 228, 8, 74] };
}
#[repr(C)]
pub struct IAccelerometerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccelerometerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2783087476, data2: 23175, data3: 18989, data4: [190, 204, 15, 144, 110, 160, 97, 221] };
}
#[repr(C)]
pub struct IAccelerometerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultWithAccelerometerReadingType: unsafe extern "system" fn(this: *mut *mut Self, readingtype: AccelerometerReadingType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccelerometerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3301213231, data2: 55403, data3: 18053, data4: [178, 215, 51, 150, 247, 152, 213, 123] };
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
impl ::windows_sys::core::Interface for IAccelerometerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2648840399, data2: 17757, data3: 19699, data4: [130, 0, 112, 225, 65, 3, 64, 248] };
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
impl ::windows_sys::core::Interface for IActivitySensor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3447350028, data2: 64351, data3: 18667, data4: [176, 155, 162, 112, 141, 28, 97, 239] };
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
impl ::windows_sys::core::Interface for IActivitySensorReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2232572566, data2: 5234, data3: 16546, data4: [178, 174, 225, 239, 41, 34, 108, 120] };
}
#[repr(C)]
pub struct IActivitySensorReadingChangeReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IActivitySensorReadingChangeReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1329342741, data2: 55611, data3: 18365, data4: [150, 10, 242, 15, 178, 243, 34, 185] };
}
#[repr(C)]
pub struct IActivitySensorReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IActivitySensorReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3728238359, data2: 44726, data3: 20167, data4: [148, 106, 217, 204, 25, 185, 81, 236] };
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
impl ::windows_sys::core::Interface for IActivitySensorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2803764893, data2: 61067, data3: 17873, data4: [178, 91, 8, 204, 13, 249, 42, 182] };
}
#[repr(C)]
pub struct IActivitySensorTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
impl ::windows_sys::core::Interface for IActivitySensorTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 748578322, data2: 47562, data3: 18039, data4: [178, 99, 36, 50, 151, 247, 157, 58] };
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
impl ::windows_sys::core::Interface for IAltimeter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1928353789, data2: 36612, data3: 18929, data4: [180, 167, 244, 227, 99, 183, 1, 162] };
}
#[repr(C)]
pub struct IAltimeter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAltimeter2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3376880633, data2: 10973, data3: 18677, data4: [159, 8, 61, 12, 118, 96, 217, 56] };
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
impl ::windows_sys::core::Interface for IAltimeterReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4226346867, data2: 32606, data3: 18632, data4: [170, 26, 241, 243, 190, 252, 17, 68] };
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
impl ::windows_sys::core::Interface for IAltimeterReading2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1413094361, data2: 27915, data3: 17074, data4: [189, 105, 188, 143, 174, 15, 120, 44] };
}
#[repr(C)]
pub struct IAltimeterReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAltimeterReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1885982839, data2: 17517, data3: 18423, data4: [153, 140, 235, 194, 59, 69, 228, 162] };
}
#[repr(C)]
pub struct IAltimeterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAltimeterStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2662651843, data2: 58796, data3: 18382, data4: [142, 239, 211, 113, 129, 104, 192, 31] };
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
impl ::windows_sys::core::Interface for IBarometer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2470737320, data2: 30911, data3: 17711, data4: [176, 23, 240, 32, 156, 230, 218, 180] };
}
#[repr(C)]
pub struct IBarometer2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarometer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 851231768, data2: 16107, data3: 19716, data4: [149, 116, 118, 51, 168, 120, 31, 159] };
}
#[repr(C)]
pub struct IBarometer3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarometer3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 238416106, data2: 693, data3: 23044, data4: [176, 61, 130, 32, 132, 134, 58, 84] };
}
#[repr(C)]
pub struct IBarometerDataThreshold {
    pub base__: ::windows_sys::core::IInspectable,
    pub Hectopascals: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHectopascals: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarometerDataThreshold {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 124491052, data2: 52066, data3: 23184, data4: [160, 209, 248, 94, 74, 147, 99, 148] };
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
impl ::windows_sys::core::Interface for IBarometerReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4122596070, data2: 7670, data3: 18970, data4: [167, 173, 50, 29, 79, 93, 178, 71] };
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
impl ::windows_sys::core::Interface for IBarometerReading2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2242004203, data2: 37061, data3: 18549, data4: [137, 28, 56, 101, 180, 195, 87, 231] };
}
#[repr(C)]
pub struct IBarometerReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarometerReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1032098911, data2: 891, data3: 16463, data4: [155, 187, 98, 50, 214, 149, 67, 195] };
}
#[repr(C)]
pub struct IBarometerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarometerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 678110986, data2: 739, data3: 20358, data4: [132, 252, 253, 216, 146, 181, 148, 15] };
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
impl ::windows_sys::core::Interface for IBarometerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2412163559, data2: 38399, data3: 17580, data4: [135, 142, 214, 92, 131, 8, 195, 76] };
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
impl ::windows_sys::core::Interface for ICompass {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 691010196, data2: 6981, data3: 16444, data4: [186, 6, 177, 6, 219, 166, 154, 100] };
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
impl ::windows_sys::core::Interface for ICompass2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 921857289, data2: 51159, data3: 17231, data4: [180, 97, 151, 157, 223, 194, 50, 47] };
}
#[repr(C)]
pub struct ICompass3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompass3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2753855515, data2: 50666, data3: 19781, data4: [160, 236, 75, 121, 31, 4, 26, 137] };
}
#[repr(C)]
pub struct ICompass4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompass4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 689864465, data2: 60466, data3: 24012, data4: [191, 203, 11, 179, 158, 186, 87, 116] };
}
#[repr(C)]
pub struct ICompassDataThreshold {
    pub base__: ::windows_sys::core::IInspectable,
    pub Degrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompassDataThreshold {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3512423091, data2: 54173, data3: 24264, data4: [178, 228, 241, 147, 230, 171, 52, 237] };
}
#[repr(C)]
pub struct ICompassDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompassDeviceId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3514944041, data2: 45189, data3: 19229, data4: [135, 10, 79, 245, 123, 167, 79, 212] };
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
impl ::windows_sys::core::Interface for ICompassReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2190545192, data2: 20797, data3: 19913, data4: [183, 129, 94, 237, 251, 240, 45, 12] };
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
impl ::windows_sys::core::Interface for ICompassReading2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2973394462, data2: 20923, data3: 18962, data4: [190, 221, 173, 71, 255, 135, 210, 232] };
}
#[repr(C)]
pub struct ICompassReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompassReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2400537008, data2: 59580, data3: 19582, data4: [176, 9, 78, 65, 223, 19, 112, 114] };
}
#[repr(C)]
pub struct ICompassReadingHeadingAccuracy {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeadingAccuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagnetometerAccuracy) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompassReadingHeadingAccuracy {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3881907534, data2: 35089, data3: 16631, data4: [158, 22, 110, 204, 125, 174, 197, 222] };
}
#[repr(C)]
pub struct ICompassStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompassStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2596050911, data2: 22252, data3: 19493, data4: [181, 77, 64, 166, 139, 181, 178, 105] };
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
impl ::windows_sys::core::Interface for ICompassStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 181276333, data2: 15274, data3: 18832, data4: [156, 228, 190, 9, 19, 117, 78, 210] };
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
impl ::windows_sys::core::Interface for IGyrometer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4256803268, data2: 33969, data3: 19618, data4: [151, 99, 155, 88, 149, 6, 199, 12] };
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
impl ::windows_sys::core::Interface for IGyrometer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1675568195, data2: 36072, data3: 16835, data4: [172, 68, 134, 152, 129, 11, 85, 127] };
}
#[repr(C)]
pub struct IGyrometer3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGyrometer3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1567590613, data2: 36796, data3: 17540, data4: [145, 75, 82, 138, 223, 217, 71, 177] };
}
#[repr(C)]
pub struct IGyrometer4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGyrometer4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 103327244, data2: 19531, data3: 20630, data4: [148, 230, 195, 86, 223, 104, 190, 247] };
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
impl ::windows_sys::core::Interface for IGyrometerDataThreshold {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2252911390, data2: 28242, data3: 21081, data4: [187, 173, 36, 42, 105, 220, 56, 200] };
}
#[repr(C)]
pub struct IGyrometerDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGyrometerDeviceId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 518383992, data2: 35234, data3: 17013, data4: [158, 149, 113, 38, 244, 112, 135, 96] };
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
impl ::windows_sys::core::Interface for IGyrometerReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3017203292, data2: 7908, data3: 17775, data4: [157, 231, 226, 73, 59, 92, 142, 3] };
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
impl ::windows_sys::core::Interface for IGyrometerReading2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 380625212, data2: 11145, data3: 17595, data4: [130, 43, 209, 225, 85, 111, 240, 155] };
}
#[repr(C)]
pub struct IGyrometerReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGyrometerReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 266279061, data2: 28574, data3: 17102, data4: [141, 88, 56, 140, 10, 184, 53, 109] };
}
#[repr(C)]
pub struct IGyrometerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGyrometerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2209802185, data2: 58525, data3: 19257, data4: [134, 230, 205, 85, 75, 228, 197, 193] };
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
impl ::windows_sys::core::Interface for IGyrometerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4018403233, data2: 55040, data3: 16900, data4: [150, 19, 121, 198, 177, 97, 223, 78] };
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
impl ::windows_sys::core::Interface for IHingeAngleReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2748138937, data2: 7153, data3: 20325, data4: [167, 4, 226, 218, 4, 241, 130, 192] };
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
impl ::windows_sys::core::Interface for IHingeAngleSensor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3922968066, data2: 49119, data3: 17279, data4: [140, 41, 136, 199, 115, 147, 211, 9] };
}
#[repr(C)]
pub struct IHingeAngleSensorReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHingeAngleSensorReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 618222987, data2: 64208, data3: 17080, data4: [168, 84, 120, 146, 48, 73, 161, 186] };
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
impl ::windows_sys::core::Interface for IHingeAngleSensorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3082172688, data2: 64433, data3: 16675, data4: [137, 206, 78, 163, 78, 176, 223, 202] };
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
impl ::windows_sys::core::Interface for IInclinometer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 642304623, data2: 8838, data3: 16495, data4: [145, 97, 240, 196, 189, 128, 110, 191] };
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
impl ::windows_sys::core::Interface for IInclinometer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 43987859, data2: 10418, data3: 17912, data4: [187, 22, 97, 232, 106, 127, 174, 110] };
}
#[repr(C)]
pub struct IInclinometer3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInclinometer3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 973688836, data2: 55141, data3: 17284, data4: [163, 215, 2, 131, 243, 171, 230, 174] };
}
#[repr(C)]
pub struct IInclinometer4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInclinometer4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1132799512, data2: 36810, data3: 21646, data4: [187, 245, 92, 80, 65, 43, 106, 164] };
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
impl ::windows_sys::core::Interface for IInclinometerDataThreshold {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4161423235, data2: 31742, data3: 21598, data4: [187, 96, 160, 235, 196, 123, 210, 251] };
}
#[repr(C)]
pub struct IInclinometerDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInclinometerDeviceId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 32053634, data2: 16895, data3: 17414, data4: [174, 131, 98, 33, 15, 241, 111, 227] };
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
impl ::windows_sys::core::Interface for IInclinometerReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2672095317, data2: 46838, data3: 18815, data4: [177, 39, 26, 119, 94, 80, 20, 88] };
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
impl ::windows_sys::core::Interface for IInclinometerReading2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1326860161, data2: 59659, data3: 18008, data4: [137, 21, 1, 3, 224, 138, 128, 90] };
}
#[repr(C)]
pub struct IInclinometerReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInclinometerReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1256791489, data2: 59371, data3: 18744, data4: [133, 17, 174, 13, 107, 68, 4, 56] };
}
#[repr(C)]
pub struct IInclinometerReadingYawAccuracy {
    pub base__: ::windows_sys::core::IInspectable,
    pub YawAccuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagnetometerAccuracy) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInclinometerReadingYawAccuracy {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3025397888, data2: 8163, data3: 18822, data4: [162, 87, 230, 236, 226, 114, 57, 73] };
}
#[repr(C)]
pub struct IInclinometerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInclinometerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4063151441, data2: 39984, data3: 17722, data4: [139, 73, 60, 62, 235, 51, 203, 97] };
}
#[repr(C)]
pub struct IInclinometerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultForRelativeReadings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInclinometerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 71276405, data2: 27166, data3: 18844, data4: [134, 224, 99, 140, 26, 134, 75, 0] };
}
#[repr(C)]
pub struct IInclinometerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultWithSensorReadingType: unsafe extern "system" fn(this: *mut *mut Self, sensorreadingtype: SensorReadingType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInclinometerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3181003392, data2: 47386, data3: 18473, data4: [147, 146, 171, 192, 182, 189, 242, 180] };
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
impl ::windows_sys::core::Interface for IInclinometerStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3904542457, data2: 28293, data3: 19075, data4: [174, 208, 215, 205, 204, 152, 86, 200] };
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
impl ::windows_sys::core::Interface for ILightSensor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4165732120, data2: 3156, data3: 18350, data4: [146, 46, 120, 159, 87, 251, 3, 160] };
}
#[repr(C)]
pub struct ILightSensor2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILightSensor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1214981352, data2: 43340, data3: 16528, data4: [143, 72, 9, 247, 130, 169, 247, 213] };
}
#[repr(C)]
pub struct ILightSensor3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILightSensor3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1215746303, data2: 40780, data3: 24434, data4: [173, 189, 163, 71, 27, 6, 60, 0] };
}
#[repr(C)]
pub struct ILightSensorDataThreshold {
    pub base__: ::windows_sys::core::IInspectable,
    pub LuxPercentage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetLuxPercentage: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub AbsoluteLux: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetAbsoluteLux: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILightSensorDataThreshold {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2975903697, data2: 34703, data3: 21650, data4: [159, 44, 51, 220, 58, 229, 132, 163] };
}
#[repr(C)]
pub struct ILightSensorDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILightSensorDeviceId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2146322936, data2: 2811, data3: 20305, data4: [135, 240, 108, 38, 55, 92, 233, 79] };
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
impl ::windows_sys::core::Interface for ILightSensorReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4292829952, data2: 8828, data3: 19755, data4: [179, 2, 252, 1, 66, 72, 92, 104] };
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
impl ::windows_sys::core::Interface for ILightSensorReading2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3075547525, data2: 17571, data3: 17609, data4: [129, 144, 158, 246, 222, 10, 138, 116] };
}
#[repr(C)]
pub struct ILightSensorReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILightSensorReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2745365711, data2: 9611, data3: 16908, data4: [184, 171, 142, 221, 96, 30, 207, 80] };
}
#[repr(C)]
pub struct ILightSensorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILightSensorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1172016260, data2: 50088, data3: 18206, data4: [154, 83, 100, 87, 250, 216, 124, 14] };
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
impl ::windows_sys::core::Interface for ILightSensorStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 247506512, data2: 56774, data3: 16555, data4: [172, 227, 236, 51, 89, 212, 44, 81] };
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
impl ::windows_sys::core::Interface for IMagnetometer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1213162094, data2: 54217, data3: 16657, data4: [179, 246, 44, 241, 250, 164, 24, 213] };
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
impl ::windows_sys::core::Interface for IMagnetometer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3026545797, data2: 9974, data3: 17483, data4: [169, 226, 162, 63, 150, 108, 211, 104] };
}
#[repr(C)]
pub struct IMagnetometer3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagnetometer3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3197361020, data2: 42533, data3: 18671, data4: [172, 247, 250, 193, 4, 131, 38, 113] };
}
#[repr(C)]
pub struct IMagnetometer4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagnetometer4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3752950017, data2: 15887, data3: 20623, data4: [178, 75, 242, 187, 117, 1, 95, 64] };
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
impl ::windows_sys::core::Interface for IMagnetometerDataThreshold {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3514288897, data2: 36963, data3: 24485, data4: [181, 150, 180, 69, 233, 220, 52, 1] };
}
#[repr(C)]
pub struct IMagnetometerDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagnetometerDeviceId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1488230594, data2: 32331, data3: 16460, data4: [159, 197, 93, 232, 180, 14, 186, 227] };
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
impl ::windows_sys::core::Interface for IMagnetometerReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 204260365, data2: 60413, data3: 20060, data4: [187, 17, 175, 194, 155, 60, 174, 97] };
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
impl ::windows_sys::core::Interface for IMagnetometerReading2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3569966177, data2: 25049, data3: 16459, data4: [163, 40, 6, 111, 23, 122, 20, 9] };
}
#[repr(C)]
pub struct IMagnetometerReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagnetometerReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 401270898, data2: 11961, data3: 20199, data4: [138, 208, 49, 39, 83, 125, 148, 155] };
}
#[repr(C)]
pub struct IMagnetometerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagnetometerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2235327692, data2: 1688, data3: 19930, data4: [166, 223, 156, 185, 204, 74, 180, 10] };
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
impl ::windows_sys::core::Interface for IMagnetometerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 738728432, data2: 65478, data3: 20361, data4: [160, 111, 24, 250, 16, 121, 41, 51] };
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
impl ::windows_sys::core::Interface for IOrientationSensor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1580549685, data2: 53099, data3: 19555, data4: [171, 216, 16, 37, 43, 11, 246, 236] };
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
impl ::windows_sys::core::Interface for IOrientationSensor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 227691769, data2: 12063, data3: 18889, data4: [128, 66, 74, 24, 19, 214, 119, 96] };
}
#[repr(C)]
pub struct IOrientationSensor3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOrientationSensor3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 751720333, data2: 25707, data3: 18629, data4: [183, 238, 68, 253, 196, 198, 170, 253] };
}
#[repr(C)]
pub struct IOrientationSensorDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOrientationSensorDeviceId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1516877384, data2: 19497, data3: 18924, data4: [178, 143, 234, 29, 17, 123, 102, 240] };
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
impl ::windows_sys::core::Interface for IOrientationSensorReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1196870035, data2: 26005, data3: 18583, data4: [188, 198, 213, 55, 238, 117, 117, 100] };
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
impl ::windows_sys::core::Interface for IOrientationSensorReading2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 5729887, data2: 18936, data3: 19461, data4: [158, 7, 36, 250, 199, 148, 8, 195] };
}
#[repr(C)]
pub struct IOrientationSensorReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOrientationSensorReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 19665286, data2: 50106, data3: 18108, data4: [174, 101, 122, 152, 153, 108, 191, 184] };
}
#[repr(C)]
pub struct IOrientationSensorReadingYawAccuracy {
    pub base__: ::windows_sys::core::IInspectable,
    pub YawAccuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagnetometerAccuracy) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOrientationSensorReadingYawAccuracy {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3517749284, data2: 16218, data3: 18850, data4: [188, 123, 17, 128, 188, 56, 205, 43] };
}
#[repr(C)]
pub struct IOrientationSensorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOrientationSensorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 284133138, data2: 64332, data3: 17034, data4: [137, 139, 39, 101, 228, 9, 230, 105] };
}
#[repr(C)]
pub struct IOrientationSensorStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultForRelativeReadings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOrientationSensorStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1507462411, data2: 54282, data3: 19569, data4: [146, 118, 138, 39, 42, 10, 102, 25] };
}
#[repr(C)]
pub struct IOrientationSensorStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultWithSensorReadingType: unsafe extern "system" fn(this: *mut *mut Self, sensorreadingtype: SensorReadingType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal: unsafe extern "system" fn(this: *mut *mut Self, sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOrientationSensorStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3626821920, data2: 10103, data3: 16639, data4: [159, 89, 214, 84, 176, 133, 241, 47] };
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
impl ::windows_sys::core::Interface for IOrientationSensorStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2793401173, data2: 11397, data3: 19240, data4: [160, 254, 88, 196, 178, 4, 149, 245] };
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
impl ::windows_sys::core::Interface for IPedometer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2585657661, data2: 15768, data3: 17912, data4: [137, 32, 142, 78, 202, 202, 95, 151] };
}
#[repr(C)]
pub struct IPedometer2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentReadings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentReadings: usize,
}
impl ::windows_sys::core::Interface for IPedometer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3852732127, data2: 11137, data3: 19165, data4: [178, 255, 119, 171, 108, 152, 186, 25] };
}
#[repr(C)]
pub struct IPedometerDataThresholdFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, sensor: *mut ::core::ffi::c_void, stepgoal: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPedometerDataThresholdFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3417149264, data2: 31316, data3: 18027, data4: [144, 16, 119, 161, 98, 252, 165, 215] };
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
impl ::windows_sys::core::Interface for IPedometerReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 575003892, data2: 43233, data3: 17199, data4: [137, 106, 190, 13, 217, 176, 45, 36] };
}
#[repr(C)]
pub struct IPedometerReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPedometerReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4166378622, data2: 43964, data3: 17494, data4: [134, 168, 37, 207, 43, 51, 55, 66] };
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
impl ::windows_sys::core::Interface for IPedometerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2191002159, data2: 16515, data3: 19963, data4: [180, 17, 147, 142, 160, 244, 185, 70] };
}
#[repr(C)]
pub struct IPedometerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetReadingsFromTriggerDetails: unsafe extern "system" fn(this: *mut *mut Self, triggerdetails: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetReadingsFromTriggerDetails: usize,
}
impl ::windows_sys::core::Interface for IPedometerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2046150331, data2: 52750, data3: 16691, data4: [180, 126, 134, 39, 234, 114, 246, 119] };
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
impl ::windows_sys::core::Interface for IProximitySensor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1421899448, data2: 60667, data3: 18756, data4: [185, 40, 116, 252, 80, 77, 71, 238] };
}
#[repr(C)]
pub struct IProximitySensorDataThresholdFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, sensor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProximitySensorDataThresholdFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2421866785, data2: 27943, data3: 19155, data4: [157, 181, 100, 103, 242, 165, 173, 157] };
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
impl ::windows_sys::core::Interface for IProximitySensorReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1898089817, data2: 4909, data3: 19807, data4: [143, 249, 47, 13, 184, 117, 28, 237] };
}
#[repr(C)]
pub struct IProximitySensorReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProximitySensorReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3485660006, data2: 50152, data3: 16637, data4: [140, 195, 103, 226, 137, 0, 73, 56] };
}
#[repr(C)]
pub struct IProximitySensorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut *mut Self, sensorid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProximitySensorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 689464905, data2: 25193, data3: 20055, data4: [165, 173, 130, 190, 128, 129, 51, 146] };
}
#[repr(C)]
pub struct IProximitySensorStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetReadingsFromTriggerDetails: unsafe extern "system" fn(this: *mut *mut Self, triggerdetails: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetReadingsFromTriggerDetails: usize,
}
impl ::windows_sys::core::Interface for IProximitySensorStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3421795246, data2: 59850, data3: 16943, data4: [173, 103, 76, 61, 37, 223, 53, 12] };
}
#[repr(C)]
pub struct ISensorDataThreshold {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISensorDataThreshold {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1423633505, data2: 65099, data3: 19975, data4: [178, 96, 58, 76, 223, 190, 57, 110] };
}
#[repr(C)]
pub struct ISensorDataThresholdTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SensorType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SensorType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISensorDataThresholdTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2433151415, data2: 59533, data3: 18609, data4: [188, 144, 97, 156, 123, 52, 147, 145] };
}
#[repr(C)]
pub struct ISensorQuaternion {
    pub base__: ::windows_sys::core::IInspectable,
    pub W: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub X: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Y: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Z: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISensorQuaternion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3385182247, data2: 50972, data3: 18151, data4: [157, 163, 54, 161, 147, 178, 50, 188] };
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
impl ::windows_sys::core::Interface for ISensorRotationMatrix {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 171792999, data2: 8948, data3: 17298, data4: [149, 56, 101, 208, 189, 6, 74, 166] };
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
impl ::windows_sys::core::Interface for ISimpleOrientationSensor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1609906262, data2: 8522, data3: 19950, data4: [163, 249, 97, 111, 26, 176, 111, 253] };
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
impl ::windows_sys::core::Interface for ISimpleOrientationSensor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2725750680, data2: 34928, data3: 17726, data4: [139, 214, 184, 245, 216, 215, 148, 27] };
}
#[repr(C)]
pub struct ISimpleOrientationSensorDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISimpleOrientationSensorDeviceId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4223666891, data2: 15222, data3: 16886, data4: [128, 145, 48, 239, 230, 70, 211, 207] };
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
impl ::windows_sys::core::Interface for ISimpleOrientationSensorOrientationChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3168126560, data2: 9172, data3: 19276, data4: [162, 46, 186, 129, 173, 224, 198, 1] };
}
#[repr(C)]
pub struct ISimpleOrientationSensorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISimpleOrientationSensorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1928136303, data2: 28842, data3: 16582, data4: [155, 27, 52, 51, 247, 69, 155, 78] };
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
impl ::windows_sys::core::Interface for ISimpleOrientationSensorStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2224004223, data2: 45368, data3: 19985, data4: [137, 16, 162, 162, 163, 181, 109, 131] };
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
