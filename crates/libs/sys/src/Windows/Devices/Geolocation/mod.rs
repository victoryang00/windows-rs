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
#[repr(C)]
pub struct IGeoboundingBox {
    pub base__: ::windows_sys::core::IInspectable,
    pub NorthwestCorner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BasicGeoposition) -> ::windows_sys::core::HRESULT,
    pub SoutheastCorner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BasicGeoposition) -> ::windows_sys::core::HRESULT,
    pub Center: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BasicGeoposition) -> ::windows_sys::core::HRESULT,
    pub MinAltitude: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MaxAltitude: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeoboundingBoxFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithAltitudeReference: unsafe extern "system" fn(this: *mut *mut Self, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(this: *mut *mut Self, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IGeocircle {
    pub base__: ::windows_sys::core::IInspectable,
    pub Center: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BasicGeoposition) -> ::windows_sys::core::HRESULT,
    pub Radius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeocircleFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, position: BasicGeoposition, radius: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithAltitudeReferenceSystem: unsafe extern "system" fn(this: *mut *mut Self, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithAltitudeReferenceSystemAndSpatialReferenceId: unsafe extern "system" fn(this: *mut *mut Self, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IGeocoordinateWithPoint {
    pub base__: ::windows_sys::core::IInspectable,
    pub Point: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeocoordinateWithPositionData {
    pub base__: ::windows_sys::core::IInspectable,
    pub PositionSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PositionSource) -> ::windows_sys::core::HRESULT,
    pub SatelliteData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeocoordinateWithPositionSourceTimestamp {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PositionSourceTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionSourceTimestamp: usize,
}
#[repr(C)]
pub struct IGeocoordinateWithRemoteSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRemoteSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IGeolocator2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowFallbackToConsentlessPositions: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IGeopath {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Positions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Positions: usize,
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
#[repr(C)]
pub struct IGeopoint {
    pub base__: ::windows_sys::core::IInspectable,
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BasicGeoposition) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeopointFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, position: BasicGeoposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithAltitudeReferenceSystem: unsafe extern "system" fn(this: *mut *mut Self, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithAltitudeReferenceSystemAndSpatialReferenceId: unsafe extern "system" fn(this: *mut *mut Self, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeoposition {
    pub base__: ::windows_sys::core::IInspectable,
    pub Coordinate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CivicAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeoposition2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub VenueData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeoshape {
    pub base__: ::windows_sys::core::IInspectable,
    pub GeoshapeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GeoshapeType) -> ::windows_sys::core::HRESULT,
    pub SpatialReferenceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AltitudeReferenceSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AltitudeReferenceSystem) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IGeovisitMonitorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetLastReportAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetLastReportAsync: usize,
}
#[repr(C)]
pub struct IGeovisitStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Visit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeovisitTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
#[repr(C)]
pub struct IPositionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStatusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PositionStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVenueData {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Level: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
