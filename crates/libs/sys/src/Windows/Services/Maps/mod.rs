#[cfg(feature = "Services_Maps_Guidance")]
pub mod Guidance;
#[cfg(feature = "Services_Maps_LocalSearch")]
pub mod LocalSearch;
#[cfg(feature = "Services_Maps_OfflineMaps")]
pub mod OfflineMaps;
pub type EnhancedWaypoint = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IEnhancedWaypoint {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Point: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Point: usize,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WaypointKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnhancedWaypoint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3978726516, data2: 22803, data3: 4582, data4: [139, 119, 134, 243, 12, 168, 147, 211] };
}
#[repr(C)]
pub struct IEnhancedWaypointFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, point: *mut ::core::ffi::c_void, kind: WaypointKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IEnhancedWaypointFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2944828535, data2: 41642, data3: 18141, data4: [182, 69, 35, 179, 27, 138, 166, 199] };
}
#[repr(C)]
pub struct IManeuverWarning {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ManeuverWarningKind) -> ::windows_sys::core::HRESULT,
    pub Severity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ManeuverWarningSeverity) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IManeuverWarning {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3248713098, data2: 9776, data3: 17272, data4: [158, 74, 110, 68, 37, 61, 206, 186] };
}
#[repr(C)]
pub struct IMapAddress {
    pub base__: ::windows_sys::core::IInspectable,
    pub BuildingName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BuildingFloor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BuildingRoom: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BuildingWing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StreetNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Street: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Neighborhood: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub District: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Town: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Region: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RegionCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Country: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CountryCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PostCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Continent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapAddress {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3483871603, data2: 41908, data3: 17556, data4: [179, 255, 203, 169, 77, 182, 150, 153] };
}
#[repr(C)]
pub struct IMapAddress2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FormattedAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapAddress2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1976397297, data2: 58797, data3: 17833, data4: [191, 64, 108, 242, 86, 193, 221, 19] };
}
#[repr(C)]
pub struct IMapLocation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Point: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Point: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapLocation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1007107927, data2: 3492, data3: 17128, data4: [158, 226, 169, 111, 207, 35, 113, 220] };
}
#[repr(C)]
pub struct IMapLocationFinderResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Locations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Locations: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapLocationFinderStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapLocationFinderResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1139929465, data2: 59596, data3: 17910, data4: [190, 210, 84, 204, 191, 150, 93, 154] };
}
#[repr(C)]
pub struct IMapLocationFinderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocationsAtAsync: unsafe extern "system" fn(this: *mut *mut Self, querypoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocationsAtAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocationsAsync: unsafe extern "system" fn(this: *mut *mut Self, searchtext: ::windows_sys::core::HSTRING, referencepoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocationsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocationsWithMaxCountAsync: unsafe extern "system" fn(this: *mut *mut Self, searchtext: ::windows_sys::core::HSTRING, referencepoint: *mut ::core::ffi::c_void, maxcount: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocationsWithMaxCountAsync: usize,
}
impl ::windows_sys::core::Interface for IMapLocationFinderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 831183709, data2: 7261, data3: 20277, data4: [162, 223, 170, 202, 148, 149, 149, 23] };
}
#[repr(C)]
pub struct IMapLocationFinderStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocationsAtWithAccuracyAsync: unsafe extern "system" fn(this: *mut *mut Self, querypoint: *mut ::core::ffi::c_void, accuracy: MapLocationDesiredAccuracy, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocationsAtWithAccuracyAsync: usize,
}
impl ::windows_sys::core::Interface for IMapLocationFinderStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2509933462, data2: 25733, data3: 19965, data4: [133, 26, 51, 172, 49, 126, 58, 246] };
}
#[repr(C)]
pub struct IMapManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowDownloadedMapsUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ShowMapsUpdateUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 937682197, data2: 33460, data3: 19796, data4: [143, 217, 175, 38, 36, 179, 1, 28] };
}
#[repr(C)]
pub struct IMapRoute {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    BoundingBox: usize,
    pub LengthInMeters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EstimatedDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EstimatedDuration: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Legs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Legs: usize,
    pub IsTrafficBased: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapRoute {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4211586866, data2: 22605, data3: 17795, data4: [156, 96, 100, 31, 234, 39, 67, 73] };
}
#[repr(C)]
pub struct IMapRoute2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViolatedRestrictions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapRouteRestrictions) -> ::windows_sys::core::HRESULT,
    pub HasBlockedRoads: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapRoute2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3519403020, data2: 8723, data3: 19120, data4: [162, 96, 70, 179, 129, 105, 190, 172] };
}
#[repr(C)]
pub struct IMapRoute3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DurationWithoutTraffic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DurationWithoutTraffic: usize,
    pub TrafficCongestion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TrafficCongestion) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapRoute3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2240618158, data2: 62125, data3: 17055, data4: [187, 55, 205, 33, 9, 79, 252, 146] };
}
#[repr(C)]
pub struct IMapRoute4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsScenic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapRoute4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 913083557, data2: 12371, data3: 20385, data4: [128, 255, 212, 117, 243, 237, 30, 110] };
}
#[repr(C)]
pub struct IMapRouteDrivingOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxAlternateRouteCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxAlternateRouteCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InitialHeading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitialHeading: usize,
    #[cfg(feature = "Foundation")]
    pub SetInitialHeading: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInitialHeading: usize,
    pub RouteOptimization: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapRouteOptimization) -> ::windows_sys::core::HRESULT,
    pub SetRouteOptimization: unsafe extern "system" fn(this: *mut *mut Self, value: MapRouteOptimization) -> ::windows_sys::core::HRESULT,
    pub RouteRestrictions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapRouteRestrictions) -> ::windows_sys::core::HRESULT,
    pub SetRouteRestrictions: unsafe extern "system" fn(this: *mut *mut Self, value: MapRouteRestrictions) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapRouteDrivingOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1746220621, data2: 50908, data3: 18071, data4: [164, 82, 177, 143, 143, 11, 103, 161] };
}
#[repr(C)]
pub struct IMapRouteDrivingOptions2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DepartureTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DepartureTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDepartureTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDepartureTime: usize,
}
impl ::windows_sys::core::Interface for IMapRouteDrivingOptions2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 903644784, data2: 49816, data3: 18640, data4: [181, 173, 130, 84, 96, 100, 86, 3] };
}
#[repr(C)]
pub struct IMapRouteFinderResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Route: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapRouteFinderStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapRouteFinderResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2825429786, data2: 37922, data3: 18092, data4: [140, 161, 177, 97, 77, 75, 251, 226] };
}
#[repr(C)]
pub struct IMapRouteFinderResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AlternateRoutes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AlternateRoutes: usize,
}
impl ::windows_sys::core::Interface for IMapRouteFinderResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 544250989, data2: 55564, data3: 18120, data4: [145, 198, 125, 75, 228, 239, 178, 21] };
}
#[repr(C)]
pub struct IMapRouteFinderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteAsync: unsafe extern "system" fn(this: *mut *mut Self, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteWithOptimizationAsync: unsafe extern "system" fn(this: *mut *mut Self, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteWithOptimizationAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteWithOptimizationAndRestrictionsAsync: unsafe extern "system" fn(this: *mut *mut Self, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteWithOptimizationAndRestrictionsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync: unsafe extern "system" fn(this: *mut *mut Self, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetDrivingRouteFromWaypointsAsync: unsafe extern "system" fn(this: *mut *mut Self, waypoints: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetDrivingRouteFromWaypointsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetDrivingRouteFromWaypointsAndOptimizationAsync: unsafe extern "system" fn(this: *mut *mut Self, waypoints: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetDrivingRouteFromWaypointsAndOptimizationAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync: unsafe extern "system" fn(this: *mut *mut Self, waypoints: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync: unsafe extern "system" fn(this: *mut *mut Self, waypoints: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetWalkingRouteAsync: unsafe extern "system" fn(this: *mut *mut Self, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetWalkingRouteAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetWalkingRouteFromWaypointsAsync: unsafe extern "system" fn(this: *mut *mut Self, waypoints: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetWalkingRouteFromWaypointsAsync: usize,
}
impl ::windows_sys::core::Interface for IMapRouteFinderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3097871631, data2: 7268, data3: 19514, data4: [129, 235, 31, 124, 21, 42, 251, 187] };
}
#[repr(C)]
pub struct IMapRouteFinderStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteWithOptionsAsync: usize,
}
impl ::windows_sys::core::Interface for IMapRouteFinderStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2949393523, data2: 30560, data3: 18863, data4: [179, 189, 186, 241, 53, 183, 3, 225] };
}
#[repr(C)]
pub struct IMapRouteFinderStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDrivingRouteFromEnhancedWaypointsAsync: unsafe extern "system" fn(this: *mut *mut Self, waypoints: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDrivingRouteFromEnhancedWaypointsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, waypoints: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync: usize,
}
impl ::windows_sys::core::Interface for IMapRouteFinderStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4127818036, data2: 22803, data3: 4582, data4: [139, 119, 134, 243, 12, 168, 147, 211] };
}
#[repr(C)]
pub struct IMapRouteLeg {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    BoundingBox: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    pub LengthInMeters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EstimatedDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EstimatedDuration: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Maneuvers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Maneuvers: usize,
}
impl ::windows_sys::core::Interface for IMapRouteLeg {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2532881142, data2: 23482, data3: 19735, data4: [157, 182, 26, 38, 63, 236, 116, 113] };
}
#[repr(C)]
pub struct IMapRouteLeg2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DurationWithoutTraffic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DurationWithoutTraffic: usize,
    pub TrafficCongestion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TrafficCongestion) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapRouteLeg2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 48367149, data2: 51654, data3: 17848, data4: [142, 84, 26, 16, 181, 122, 23, 232] };
}
#[repr(C)]
pub struct IMapRouteManeuver {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub StartingPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    StartingPoint: usize,
    pub LengthInMeters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub InstructionText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapRouteManeuverKind) -> ::windows_sys::core::HRESULT,
    pub ExitNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ManeuverNotices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapManeuverNotices) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapRouteManeuver {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3982235632, data2: 42667, data3: 19813, data4: [160, 134, 250, 138, 126, 52, 13, 242] };
}
#[repr(C)]
pub struct IMapRouteManeuver2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub StartHeading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub EndHeading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub StreetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapRouteManeuver2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1568394652, data2: 31899, data3: 16863, data4: [131, 139, 234, 226, 30, 75, 5, 169] };
}
#[repr(C)]
pub struct IMapRouteManeuver3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Warnings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Warnings: usize,
}
impl ::windows_sys::core::Interface for IMapRouteManeuver3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2795583711, data2: 1155, data3: 16742, data4: [133, 190, 185, 147, 54, 193, 24, 117] };
}
#[repr(C)]
pub struct IMapServiceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetServiceToken: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ServiceToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapServiceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 21278085, data2: 49228, data3: 19677, data4: [135, 26, 160, 114, 109, 9, 124, 212] };
}
#[repr(C)]
pub struct IMapServiceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub WorldViewRegionCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapServiceStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4162404077, data2: 40069, data3: 16553, data4: [136, 150, 15, 195, 253, 43, 124, 42] };
}
#[repr(C)]
pub struct IMapServiceStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataAttributions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapServiceStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 168939040, data2: 25511, data3: 18516, data4: [179, 85, 214, 220, 218, 34, 61, 27] };
}
#[repr(C)]
pub struct IMapServiceStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDataUsagePreference: unsafe extern "system" fn(this: *mut *mut Self, value: MapServiceDataUsagePreference) -> ::windows_sys::core::HRESULT,
    pub DataUsagePreference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapServiceDataUsagePreference) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapServiceStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 143272034, data2: 27324, data3: 16910, data4: [148, 95, 76, 253, 137, 198, 115, 86] };
}
#[repr(C)]
pub struct IPlaceInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Show: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowWithPreferredPlacement: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowWithPreferredPlacement: usize,
    pub Identifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Geoshape: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Geoshape: usize,
}
impl ::windows_sys::core::Interface for IPlaceInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2584219830, data2: 12744, data3: 20330, data4: [159, 24, 149, 11, 76, 56, 149, 26] };
}
#[repr(C)]
pub struct IPlaceInfoCreateOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlaceInfoCreateOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3442721061, data2: 26609, data3: 19379, data4: [153, 7, 236, 206, 147, 155, 3, 153] };
}
#[repr(C)]
pub struct IPlaceInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, referencepoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Create: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateWithGeopointAndOptions: unsafe extern "system" fn(this: *mut *mut Self, referencepoint: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateWithGeopointAndOptions: usize,
    pub CreateFromIdentifier: unsafe extern "system" fn(this: *mut *mut Self, identifier: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromIdentifierWithOptions: unsafe extern "system" fn(this: *mut *mut Self, identifier: ::windows_sys::core::HSTRING, defaultpoint: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromIdentifierWithOptions: usize,
    pub CreateFromMapLocation: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsShowSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlaceInfoStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2193227633, data2: 27856, data3: 18596, data4: [175, 217, 94, 216, 32, 151, 147, 107] };
}
#[repr(C)]
pub struct IPlaceInfoStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromAddress: unsafe extern "system" fn(this: *mut *mut Self, displayaddress: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromAddressWithName: unsafe extern "system" fn(this: *mut *mut Self, displayaddress: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlaceInfoStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1930363465, data2: 16455, data3: 17571, data4: [143, 129, 37, 80, 165, 33, 99, 112] };
}
pub type ManeuverWarning = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct ManeuverWarningKind(pub i32);
impl ManeuverWarningKind {
    pub const None: Self = Self(0i32);
    pub const Accident: Self = Self(1i32);
    pub const AdministrativeDivisionChange: Self = Self(2i32);
    pub const Alert: Self = Self(3i32);
    pub const BlockedRoad: Self = Self(4i32);
    pub const CheckTimetable: Self = Self(5i32);
    pub const Congestion: Self = Self(6i32);
    pub const Construction: Self = Self(7i32);
    pub const CountryChange: Self = Self(8i32);
    pub const DisabledVehicle: Self = Self(9i32);
    pub const GateAccess: Self = Self(10i32);
    pub const GetOffTransit: Self = Self(11i32);
    pub const GetOnTransit: Self = Self(12i32);
    pub const IllegalUTurn: Self = Self(13i32);
    pub const MassTransit: Self = Self(14i32);
    pub const Miscellaneous: Self = Self(15i32);
    pub const NoIncident: Self = Self(16i32);
    pub const Other: Self = Self(17i32);
    pub const OtherNews: Self = Self(18i32);
    pub const OtherTrafficIncidents: Self = Self(19i32);
    pub const PlannedEvent: Self = Self(20i32);
    pub const PrivateRoad: Self = Self(21i32);
    pub const RestrictedTurn: Self = Self(22i32);
    pub const RoadClosures: Self = Self(23i32);
    pub const RoadHazard: Self = Self(24i32);
    pub const ScheduledConstruction: Self = Self(25i32);
    pub const SeasonalClosures: Self = Self(26i32);
    pub const Tollbooth: Self = Self(27i32);
    pub const TollRoad: Self = Self(28i32);
    pub const TollZoneEnter: Self = Self(29i32);
    pub const TollZoneExit: Self = Self(30i32);
    pub const TrafficFlow: Self = Self(31i32);
    pub const TransitLineChange: Self = Self(32i32);
    pub const UnpavedRoad: Self = Self(33i32);
    pub const UnscheduledConstruction: Self = Self(34i32);
    pub const Weather: Self = Self(35i32);
}
impl ::core::marker::Copy for ManeuverWarningKind {}
impl ::core::clone::Clone for ManeuverWarningKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct ManeuverWarningSeverity(pub i32);
impl ManeuverWarningSeverity {
    pub const None: Self = Self(0i32);
    pub const LowImpact: Self = Self(1i32);
    pub const Minor: Self = Self(2i32);
    pub const Moderate: Self = Self(3i32);
    pub const Serious: Self = Self(4i32);
}
impl ::core::marker::Copy for ManeuverWarningSeverity {}
impl ::core::clone::Clone for ManeuverWarningSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapAddress = *mut ::core::ffi::c_void;
pub type MapLocation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapLocationDesiredAccuracy(pub i32);
impl MapLocationDesiredAccuracy {
    pub const High: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
}
impl ::core::marker::Copy for MapLocationDesiredAccuracy {}
impl ::core::clone::Clone for MapLocationDesiredAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapLocationFinderResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapLocationFinderStatus(pub i32);
impl MapLocationFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const BadLocation: Self = Self(3i32);
    pub const IndexFailure: Self = Self(4i32);
    pub const NetworkFailure: Self = Self(5i32);
    pub const NotSupported: Self = Self(6i32);
}
impl ::core::marker::Copy for MapLocationFinderStatus {}
impl ::core::clone::Clone for MapLocationFinderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapManeuverNotices(pub u32);
impl MapManeuverNotices {
    pub const None: Self = Self(0u32);
    pub const Toll: Self = Self(1u32);
    pub const Unpaved: Self = Self(2u32);
}
impl ::core::marker::Copy for MapManeuverNotices {}
impl ::core::clone::Clone for MapManeuverNotices {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapRoute = *mut ::core::ffi::c_void;
pub type MapRouteDrivingOptions = *mut ::core::ffi::c_void;
pub type MapRouteFinderResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapRouteFinderStatus(pub i32);
impl MapRouteFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const NoRouteFound: Self = Self(3i32);
    pub const NoRouteFoundWithGivenOptions: Self = Self(4i32);
    pub const StartPointNotFound: Self = Self(5i32);
    pub const EndPointNotFound: Self = Self(6i32);
    pub const NoPedestrianRouteFound: Self = Self(7i32);
    pub const NetworkFailure: Self = Self(8i32);
    pub const NotSupported: Self = Self(9i32);
}
impl ::core::marker::Copy for MapRouteFinderStatus {}
impl ::core::clone::Clone for MapRouteFinderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapRouteLeg = *mut ::core::ffi::c_void;
pub type MapRouteManeuver = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapRouteManeuverKind(pub i32);
impl MapRouteManeuverKind {
    pub const None: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const Stopover: Self = Self(2i32);
    pub const StopoverResume: Self = Self(3i32);
    pub const End: Self = Self(4i32);
    pub const GoStraight: Self = Self(5i32);
    pub const UTurnLeft: Self = Self(6i32);
    pub const UTurnRight: Self = Self(7i32);
    pub const TurnKeepLeft: Self = Self(8i32);
    pub const TurnKeepRight: Self = Self(9i32);
    pub const TurnLightLeft: Self = Self(10i32);
    pub const TurnLightRight: Self = Self(11i32);
    pub const TurnLeft: Self = Self(12i32);
    pub const TurnRight: Self = Self(13i32);
    pub const TurnHardLeft: Self = Self(14i32);
    pub const TurnHardRight: Self = Self(15i32);
    pub const FreewayEnterLeft: Self = Self(16i32);
    pub const FreewayEnterRight: Self = Self(17i32);
    pub const FreewayLeaveLeft: Self = Self(18i32);
    pub const FreewayLeaveRight: Self = Self(19i32);
    pub const FreewayContinueLeft: Self = Self(20i32);
    pub const FreewayContinueRight: Self = Self(21i32);
    pub const TrafficCircleLeft: Self = Self(22i32);
    pub const TrafficCircleRight: Self = Self(23i32);
    pub const TakeFerry: Self = Self(24i32);
}
impl ::core::marker::Copy for MapRouteManeuverKind {}
impl ::core::clone::Clone for MapRouteManeuverKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapRouteOptimization(pub i32);
impl MapRouteOptimization {
    pub const Time: Self = Self(0i32);
    pub const Distance: Self = Self(1i32);
    pub const TimeWithTraffic: Self = Self(2i32);
    pub const Scenic: Self = Self(3i32);
}
impl ::core::marker::Copy for MapRouteOptimization {}
impl ::core::clone::Clone for MapRouteOptimization {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapRouteRestrictions(pub u32);
impl MapRouteRestrictions {
    pub const None: Self = Self(0u32);
    pub const Highways: Self = Self(1u32);
    pub const TollRoads: Self = Self(2u32);
    pub const Ferries: Self = Self(4u32);
    pub const Tunnels: Self = Self(8u32);
    pub const DirtRoads: Self = Self(16u32);
    pub const Motorail: Self = Self(32u32);
}
impl ::core::marker::Copy for MapRouteRestrictions {}
impl ::core::clone::Clone for MapRouteRestrictions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapServiceDataUsagePreference(pub i32);
impl MapServiceDataUsagePreference {
    pub const Default: Self = Self(0i32);
    pub const OfflineMapDataOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for MapServiceDataUsagePreference {}
impl ::core::clone::Clone for MapServiceDataUsagePreference {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PlaceInfo = *mut ::core::ffi::c_void;
pub type PlaceInfoCreateOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct TrafficCongestion(pub i32);
impl TrafficCongestion {
    pub const Unknown: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Mild: Self = Self(2i32);
    pub const Medium: Self = Self(3i32);
    pub const Heavy: Self = Self(4i32);
}
impl ::core::marker::Copy for TrafficCongestion {}
impl ::core::clone::Clone for TrafficCongestion {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct WaypointKind(pub i32);
impl WaypointKind {
    pub const Stop: Self = Self(0i32);
    pub const Via: Self = Self(1i32);
}
impl ::core::marker::Copy for WaypointKind {}
impl ::core::clone::Clone for WaypointKind {
    fn clone(&self) -> Self {
        *self
    }
}
