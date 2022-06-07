#[cfg(feature = "Devices_Geolocation_Geofencing")]
pub mod Geofencing;
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct AltitudeReferenceSystem(pub i32);
impl AltitudeReferenceSystem {
    pub const Unspecified: Self = Self(0i32);
    pub const Terrain: Self = Self(1i32);
    pub const Ellipsoid: Self = Self(2i32);
    pub const Geoid: Self = Self(3i32);
    pub const Surface: Self = Self(4i32);
}
impl ::core::marker::Copy for AltitudeReferenceSystem {}
impl ::core::clone::Clone for AltitudeReferenceSystem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
pub struct BasicGeoposition {
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
}
impl ::core::marker::Copy for BasicGeoposition {}
impl ::core::clone::Clone for BasicGeoposition {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CivicAddress = *mut ::core::ffi::c_void;
pub type GeoboundingBox = *mut ::core::ffi::c_void;
pub type Geocircle = *mut ::core::ffi::c_void;
pub type Geocoordinate = *mut ::core::ffi::c_void;
pub type GeocoordinateSatelliteData = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct GeolocationAccessStatus(pub i32);
impl GeolocationAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl ::core::marker::Copy for GeolocationAccessStatus {}
impl ::core::clone::Clone for GeolocationAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Geolocator = *mut ::core::ffi::c_void;
pub type Geopath = *mut ::core::ffi::c_void;
pub type Geopoint = *mut ::core::ffi::c_void;
pub type Geoposition = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct GeoshapeType(pub i32);
impl GeoshapeType {
    pub const Geopoint: Self = Self(0i32);
    pub const Geocircle: Self = Self(1i32);
    pub const Geopath: Self = Self(2i32);
    pub const GeoboundingBox: Self = Self(3i32);
}
impl ::core::marker::Copy for GeoshapeType {}
impl ::core::clone::Clone for GeoshapeType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Geovisit = *mut ::core::ffi::c_void;
pub type GeovisitMonitor = *mut ::core::ffi::c_void;
pub type GeovisitStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type GeovisitTriggerDetails = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICivicAddress {
    pub base__: ::windows_sys::core::IInspectable,
    pub Country: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
impl ::windows_sys::core::Interface for ICivicAddress {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2824239642, data2: 25844, data3: 19784, data4: [188, 234, 246, 176, 8, 236, 163, 76] };
}
#[repr(C)]
pub struct IGeoboundingBox {
    pub base__: ::windows_sys::core::IInspectable,
    pub NorthwestCorner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BasicGeoposition) -> ::windows_sys::core::HRESULT,
    pub SoutheastCorner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BasicGeoposition) -> ::windows_sys::core::HRESULT,
    pub Center: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BasicGeoposition) -> ::windows_sys::core::HRESULT,
    pub MinAltitude: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MaxAltitude: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeoboundingBox {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 144099339, data2: 10063, data3: 17370, data4: [154, 6, 203, 252, 218, 235, 78, 194] };
}
#[repr(C)]
pub struct IGeoboundingBoxFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithAltitudeReference: unsafe extern "system" fn(this: *mut *mut Self, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(this: *mut *mut Self, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeoboundingBoxFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1308337545, data2: 1041, data3: 19132, data4: [179, 181, 91, 188, 203, 87, 217, 140] };
}
#[repr(C)]
pub struct IGeoboundingBoxStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub TryCompute: unsafe extern "system" fn(this: *mut *mut Self, positions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryCompute: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryComputeWithAltitudeReference: unsafe extern "system" fn(this: *mut *mut Self, positions: *mut ::core::ffi::c_void, altituderefsystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryComputeWithAltitudeReference: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryComputeWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(this: *mut *mut Self, positions: *mut ::core::ffi::c_void, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryComputeWithAltitudeReferenceAndSpatialReference: usize,
}
impl ::windows_sys::core::Interface for IGeoboundingBoxStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1740113672, data2: 58906, data3: 19664, data4: [132, 27, 147, 35, 55, 146, 181, 202] };
}
#[repr(C)]
pub struct IGeocircle {
    pub base__: ::windows_sys::core::IInspectable,
    pub Center: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BasicGeoposition) -> ::windows_sys::core::HRESULT,
    pub Radius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeocircle {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 971266115, data2: 43001, data3: 20067, data4: [146, 167, 186, 12, 40, 209, 36, 177] };
}
#[repr(C)]
pub struct IGeocircleFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, position: BasicGeoposition, radius: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithAltitudeReferenceSystem: unsafe extern "system" fn(this: *mut *mut Self, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithAltitudeReferenceSystemAndSpatialReferenceId: unsafe extern "system" fn(this: *mut *mut Self, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeocircleFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2950058783, data2: 29361, data3: 20349, data4: [135, 204, 78, 212, 201, 132, 156, 5] };
}
#[repr(C)]
pub struct IGeocoordinate {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Latitude: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Latitude: usize,
    #[cfg(feature = "deprecated")]
    pub Longitude: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Longitude: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Altitude: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Altitude: usize,
    pub Accuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AltitudeAccuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AltitudeAccuracy: usize,
    #[cfg(feature = "Foundation")]
    pub Heading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Heading: usize,
    #[cfg(feature = "Foundation")]
    pub Speed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Speed: usize,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
impl ::windows_sys::core::Interface for IGeocoordinate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3995181994, data2: 38762, data3: 19568, data4: [128, 61, 8, 62, 165, 91, 203, 196] };
}
#[repr(C)]
pub struct IGeocoordinateSatelliteData {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PositionDilutionOfPrecision: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionDilutionOfPrecision: usize,
    #[cfg(feature = "Foundation")]
    pub HorizontalDilutionOfPrecision: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HorizontalDilutionOfPrecision: usize,
    #[cfg(feature = "Foundation")]
    pub VerticalDilutionOfPrecision: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VerticalDilutionOfPrecision: usize,
}
impl ::windows_sys::core::Interface for IGeocoordinateSatelliteData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3274339545, data2: 9736, data3: 18252, data4: [145, 44, 6, 221, 73, 15, 74, 247] };
}
#[repr(C)]
pub struct IGeocoordinateSatelliteData2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GeometricDilutionOfPrecision: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GeometricDilutionOfPrecision: usize,
    #[cfg(feature = "Foundation")]
    pub TimeDilutionOfPrecision: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeDilutionOfPrecision: usize,
}
impl ::windows_sys::core::Interface for IGeocoordinateSatelliteData2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1981582589, data2: 41373, data3: 23121, data4: [128, 245, 113, 103, 97, 21, 72, 62] };
}
#[repr(C)]
pub struct IGeocoordinateWithPoint {
    pub base__: ::windows_sys::core::IInspectable,
    pub Point: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeocoordinateWithPoint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4276749605, data2: 53804, data3: 19782, data4: [181, 39, 11, 150, 6, 111, 199, 219] };
}
#[repr(C)]
pub struct IGeocoordinateWithPositionData {
    pub base__: ::windows_sys::core::IInspectable,
    pub PositionSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PositionSource) -> ::windows_sys::core::HRESULT,
    pub SatelliteData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeocoordinateWithPositionData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2514891966, data2: 56278, data3: 16556, data4: [184, 242, 166, 92, 3, 64, 217, 166] };
}
#[repr(C)]
pub struct IGeocoordinateWithPositionSourceTimestamp {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PositionSourceTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionSourceTimestamp: usize,
}
impl ::windows_sys::core::Interface for IGeocoordinateWithPositionSourceTimestamp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2235825154, data2: 51697, data3: 17936, data4: [175, 224, 139, 195, 166, 168, 112, 54] };
}
#[repr(C)]
pub struct IGeocoordinateWithRemoteSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRemoteSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeocoordinateWithRemoteSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 964488151, data2: 60984, data3: 24379, data4: [137, 0, 196, 167, 188, 156, 249, 83] };
}
#[repr(C)]
pub struct IGeolocator {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesiredAccuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PositionAccuracy) -> ::windows_sys::core::HRESULT,
    pub SetDesiredAccuracy: unsafe extern "system" fn(this: *mut *mut Self, value: PositionAccuracy) -> ::windows_sys::core::HRESULT,
    pub MovementThreshold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMovementThreshold: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub LocationStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PositionStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetGeopositionAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetGeopositionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetGeopositionAsyncWithAgeAndTimeout: unsafe extern "system" fn(this: *mut *mut Self, maximumage: super::super::Foundation::TimeSpan, timeout: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetGeopositionAsyncWithAgeAndTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub PositionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
impl ::windows_sys::core::Interface for IGeolocator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2848178018, data2: 17700, data3: 18825, data4: [138, 169, 222, 1, 157, 46, 85, 31] };
}
#[repr(C)]
pub struct IGeolocator2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowFallbackToConsentlessPositions: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeolocator2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3518246509, data2: 34961, data3: 17332, data4: [173, 54, 39, 198, 254, 154, 151, 177] };
}
#[repr(C)]
pub struct IGeolocatorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetGeopositionHistoryAsync: unsafe extern "system" fn(this: *mut *mut Self, starttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetGeopositionHistoryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetGeopositionHistoryWithDurationAsync: unsafe extern "system" fn(this: *mut *mut Self, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetGeopositionHistoryWithDurationAsync: usize,
}
impl ::windows_sys::core::Interface for IGeolocatorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2593027441, data2: 11765, data3: 17809, data4: [159, 135, 235, 95, 216, 148, 233, 183] };
}
#[repr(C)]
pub struct IGeolocatorStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDefaultGeopositionRecommended: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDefaultGeoposition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDefaultGeoposition: usize,
    #[cfg(feature = "Foundation")]
    pub DefaultGeoposition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultGeoposition: usize,
}
impl ::windows_sys::core::Interface for IGeolocatorStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2570064290, data2: 64028, data3: 17969, data4: [167, 29, 13, 190, 177, 37, 13, 156] };
}
#[repr(C)]
pub struct IGeolocatorWithScalarAccuracy {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DesiredAccuracyInMeters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredAccuracyInMeters: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredAccuracyInMeters: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredAccuracyInMeters: usize,
}
impl ::windows_sys::core::Interface for IGeolocatorWithScalarAccuracy {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2532692929, data2: 47119, data3: 17930, data4: [153, 77, 169, 108, 71, 165, 26, 164] };
}
#[repr(C)]
pub struct IGeopath {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Positions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Positions: usize,
}
impl ::windows_sys::core::Interface for IGeopath {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3846166457, data2: 11684, data3: 18196, data4: [166, 82, 222, 133, 147, 40, 152, 152] };
}
#[repr(C)]
pub struct IGeopathFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, positions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAltitudeReference: unsafe extern "system" fn(this: *mut *mut Self, positions: *mut ::core::ffi::c_void, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAltitudeReference: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(this: *mut *mut Self, positions: *mut ::core::ffi::c_void, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAltitudeReferenceAndSpatialReference: usize,
}
impl ::windows_sys::core::Interface for IGeopathFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 666806728, data2: 51175, data3: 17241, data4: [155, 155, 252, 163, 224, 94, 245, 147] };
}
#[repr(C)]
pub struct IGeopoint {
    pub base__: ::windows_sys::core::IInspectable,
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BasicGeoposition) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeopoint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1811546347, data2: 58734, data3: 18875, data4: [156, 175, 203, 170, 120, 168, 188, 239] };
}
#[repr(C)]
pub struct IGeopointFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, position: BasicGeoposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithAltitudeReferenceSystem: unsafe extern "system" fn(this: *mut *mut Self, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithAltitudeReferenceSystemAndSpatialReferenceId: unsafe extern "system" fn(this: *mut *mut Self, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeopointFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3681258803, data2: 30397, data3: 20016, data4: [138, 247, 168, 68, 220, 55, 183, 160] };
}
#[repr(C)]
pub struct IGeoposition {
    pub base__: ::windows_sys::core::IInspectable,
    pub Coordinate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CivicAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeoposition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3247244372, data2: 32065, data3: 20471, data4: [169, 87, 157, 255, 180, 239, 127, 91] };
}
#[repr(C)]
pub struct IGeoposition2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub VenueData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeoposition2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2137192087, data2: 34417, data3: 19213, data4: [134, 248, 71, 74, 132, 150, 24, 124] };
}
#[repr(C)]
pub struct IGeoshape {
    pub base__: ::windows_sys::core::IInspectable,
    pub GeoshapeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GeoshapeType) -> ::windows_sys::core::HRESULT,
    pub SpatialReferenceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AltitudeReferenceSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AltitudeReferenceSystem) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeoshape {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3382485679, data2: 50985, data3: 17345, data4: [143, 171, 214, 222, 201, 20, 223, 126] };
}
#[repr(C)]
pub struct IGeovisit {
    pub base__: ::windows_sys::core::IInspectable,
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StateChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VisitStateChange) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
impl ::windows_sys::core::Interface for IGeovisit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2978445942, data2: 40694, data3: 16811, data4: [160, 221, 121, 62, 206, 118, 226, 222] };
}
#[repr(C)]
pub struct IGeovisitMonitor {
    pub base__: ::windows_sys::core::IInspectable,
    pub MonitoringScope: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VisitMonitoringScope) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, value: VisitMonitoringScope) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub VisitStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisitStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVisitStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVisitStateChanged: usize,
}
impl ::windows_sys::core::Interface for IGeovisitMonitor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2148633263, data2: 22852, data3: 17809, data4: [131, 193, 57, 102, 71, 245, 79, 44] };
}
#[repr(C)]
pub struct IGeovisitMonitorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetLastReportAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetLastReportAsync: usize,
}
impl ::windows_sys::core::Interface for IGeovisitMonitorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3170465447, data2: 48114, data3: 19677, data4: [149, 207, 85, 76, 130, 237, 251, 135] };
}
#[repr(C)]
pub struct IGeovisitStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Visit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGeovisitStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3467956735, data2: 35667, data3: 18792, data4: [190, 237, 76, 236, 208, 41, 206, 21] };
}
#[repr(C)]
pub struct IGeovisitTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
impl ::windows_sys::core::Interface for IGeovisitTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3933670814, data2: 53705, data3: 17739, data4: [153, 183, 178, 248, 205, 210, 72, 47] };
}
#[repr(C)]
pub struct IPositionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPositionChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 931503333, data2: 40222, data3: 18117, data4: [191, 59, 106, 216, 202, 193, 160, 147] };
}
#[repr(C)]
pub struct IStatusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PositionStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStatusChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 877908698, data2: 35987, data3: 16657, data4: [162, 5, 154, 236, 252, 155, 229, 192] };
}
#[repr(C)]
pub struct IVenueData {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Level: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVenueData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1727238535, data2: 24803, data3: 19247, data4: [181, 39, 79, 83, 241, 195, 198, 119] };
}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct PositionAccuracy(pub i32);
impl PositionAccuracy {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for PositionAccuracy {}
impl ::core::clone::Clone for PositionAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PositionChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct PositionSource(pub i32);
impl PositionSource {
    pub const Cellular: Self = Self(0i32);
    pub const Satellite: Self = Self(1i32);
    pub const WiFi: Self = Self(2i32);
    pub const IPAddress: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
    pub const Default: Self = Self(5i32);
    pub const Obfuscated: Self = Self(6i32);
}
impl ::core::marker::Copy for PositionSource {}
impl ::core::clone::Clone for PositionSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct PositionStatus(pub i32);
impl PositionStatus {
    pub const Ready: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const NoData: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
    pub const NotInitialized: Self = Self(4i32);
    pub const NotAvailable: Self = Self(5i32);
}
impl ::core::marker::Copy for PositionStatus {}
impl ::core::clone::Clone for PositionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StatusChangedEventArgs = *mut ::core::ffi::c_void;
pub type VenueData = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct VisitMonitoringScope(pub i32);
impl VisitMonitoringScope {
    pub const Venue: Self = Self(0i32);
    pub const City: Self = Self(1i32);
}
impl ::core::marker::Copy for VisitMonitoringScope {}
impl ::core::clone::Clone for VisitMonitoringScope {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct VisitStateChange(pub i32);
impl VisitStateChange {
    pub const TrackingLost: Self = Self(0i32);
    pub const Arrived: Self = Self(1i32);
    pub const Departed: Self = Self(2i32);
    pub const OtherMovement: Self = Self(3i32);
}
impl ::core::marker::Copy for VisitStateChange {}
impl ::core::clone::Clone for VisitStateChange {
    fn clone(&self) -> Self {
        *self
    }
}
