#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceAudioMeasurementSystem(pub i32);
impl GuidanceAudioMeasurementSystem {
    pub const Meters: Self = Self(0i32);
    pub const MilesAndYards: Self = Self(1i32);
    pub const MilesAndFeet: Self = Self(2i32);
}
impl ::core::marker::Copy for GuidanceAudioMeasurementSystem {}
impl ::core::clone::Clone for GuidanceAudioMeasurementSystem {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceAudioNotificationKind(pub i32);
impl GuidanceAudioNotificationKind {
    pub const Maneuver: Self = Self(0i32);
    pub const Route: Self = Self(1i32);
    pub const Gps: Self = Self(2i32);
    pub const SpeedLimit: Self = Self(3i32);
    pub const Traffic: Self = Self(4i32);
    pub const TrafficCamera: Self = Self(5i32);
}
impl ::core::marker::Copy for GuidanceAudioNotificationKind {}
impl ::core::clone::Clone for GuidanceAudioNotificationKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GuidanceAudioNotificationRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceAudioNotifications(pub u32);
impl GuidanceAudioNotifications {
    pub const None: Self = Self(0u32);
    pub const Maneuver: Self = Self(1u32);
    pub const Route: Self = Self(2u32);
    pub const Gps: Self = Self(4u32);
    pub const SpeedLimit: Self = Self(8u32);
    pub const Traffic: Self = Self(16u32);
    pub const TrafficCamera: Self = Self(32u32);
}
impl ::core::marker::Copy for GuidanceAudioNotifications {}
impl ::core::clone::Clone for GuidanceAudioNotifications {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GuidanceLaneInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceLaneMarkers(pub u32);
impl GuidanceLaneMarkers {
    pub const None: Self = Self(0u32);
    pub const LightRight: Self = Self(1u32);
    pub const Right: Self = Self(2u32);
    pub const HardRight: Self = Self(4u32);
    pub const Straight: Self = Self(8u32);
    pub const UTurnLeft: Self = Self(16u32);
    pub const HardLeft: Self = Self(32u32);
    pub const Left: Self = Self(64u32);
    pub const LightLeft: Self = Self(128u32);
    pub const UTurnRight: Self = Self(256u32);
    pub const Unknown: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for GuidanceLaneMarkers {}
impl ::core::clone::Clone for GuidanceLaneMarkers {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GuidanceManeuver = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceManeuverKind(pub i32);
impl GuidanceManeuverKind {
    pub const None: Self = Self(0i32);
    pub const GoStraight: Self = Self(1i32);
    pub const UTurnRight: Self = Self(2i32);
    pub const UTurnLeft: Self = Self(3i32);
    pub const TurnKeepRight: Self = Self(4i32);
    pub const TurnLightRight: Self = Self(5i32);
    pub const TurnRight: Self = Self(6i32);
    pub const TurnHardRight: Self = Self(7i32);
    pub const KeepMiddle: Self = Self(8i32);
    pub const TurnKeepLeft: Self = Self(9i32);
    pub const TurnLightLeft: Self = Self(10i32);
    pub const TurnLeft: Self = Self(11i32);
    pub const TurnHardLeft: Self = Self(12i32);
    pub const FreewayEnterRight: Self = Self(13i32);
    pub const FreewayEnterLeft: Self = Self(14i32);
    pub const FreewayLeaveRight: Self = Self(15i32);
    pub const FreewayLeaveLeft: Self = Self(16i32);
    pub const FreewayKeepRight: Self = Self(17i32);
    pub const FreewayKeepLeft: Self = Self(18i32);
    pub const TrafficCircleRight1: Self = Self(19i32);
    pub const TrafficCircleRight2: Self = Self(20i32);
    pub const TrafficCircleRight3: Self = Self(21i32);
    pub const TrafficCircleRight4: Self = Self(22i32);
    pub const TrafficCircleRight5: Self = Self(23i32);
    pub const TrafficCircleRight6: Self = Self(24i32);
    pub const TrafficCircleRight7: Self = Self(25i32);
    pub const TrafficCircleRight8: Self = Self(26i32);
    pub const TrafficCircleRight9: Self = Self(27i32);
    pub const TrafficCircleRight10: Self = Self(28i32);
    pub const TrafficCircleRight11: Self = Self(29i32);
    pub const TrafficCircleRight12: Self = Self(30i32);
    pub const TrafficCircleLeft1: Self = Self(31i32);
    pub const TrafficCircleLeft2: Self = Self(32i32);
    pub const TrafficCircleLeft3: Self = Self(33i32);
    pub const TrafficCircleLeft4: Self = Self(34i32);
    pub const TrafficCircleLeft5: Self = Self(35i32);
    pub const TrafficCircleLeft6: Self = Self(36i32);
    pub const TrafficCircleLeft7: Self = Self(37i32);
    pub const TrafficCircleLeft8: Self = Self(38i32);
    pub const TrafficCircleLeft9: Self = Self(39i32);
    pub const TrafficCircleLeft10: Self = Self(40i32);
    pub const TrafficCircleLeft11: Self = Self(41i32);
    pub const TrafficCircleLeft12: Self = Self(42i32);
    pub const Start: Self = Self(43i32);
    pub const End: Self = Self(44i32);
    pub const TakeFerry: Self = Self(45i32);
    pub const PassTransitStation: Self = Self(46i32);
    pub const LeaveTransitStation: Self = Self(47i32);
}
impl ::core::marker::Copy for GuidanceManeuverKind {}
impl ::core::clone::Clone for GuidanceManeuverKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GuidanceMapMatchedCoordinate = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceMode(pub i32);
impl GuidanceMode {
    pub const None: Self = Self(0i32);
    pub const Simulation: Self = Self(1i32);
    pub const Navigation: Self = Self(2i32);
    pub const Tracking: Self = Self(3i32);
}
impl ::core::marker::Copy for GuidanceMode {}
impl ::core::clone::Clone for GuidanceMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GuidanceNavigator = *mut ::core::ffi::c_void;
pub type GuidanceReroutedEventArgs = *mut ::core::ffi::c_void;
pub type GuidanceRoadSegment = *mut ::core::ffi::c_void;
pub type GuidanceRoadSignpost = *mut ::core::ffi::c_void;
pub type GuidanceRoute = *mut ::core::ffi::c_void;
pub type GuidanceTelemetryCollector = *mut ::core::ffi::c_void;
pub type GuidanceUpdatedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IGuidanceAudioNotificationRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub AudioNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GuidanceAudioNotificationKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AudioFilePaths: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AudioFilePaths: usize,
    pub AudioText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceLaneInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub LaneMarkers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GuidanceLaneMarkers) -> ::windows_sys::core::HRESULT,
    pub IsOnRoute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceManeuver {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub StartLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    StartLocation: usize,
    pub DistanceFromRouteStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DistanceFromPreviousManeuver: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DepartureRoadName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NextRoadName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DepartureShortRoadName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NextShortRoadName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GuidanceManeuverKind) -> ::windows_sys::core::HRESULT,
    pub StartAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub EndAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RoadSignpost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InstructionText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceMapMatchedCoordinate {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    pub CurrentHeading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub CurrentSpeed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub IsOnStreet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Road: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceNavigator {
    pub base__: ::windows_sys::core::IInspectable,
    pub StartNavigating: unsafe extern "system" fn(this: *mut *mut Self, route: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartSimulating: unsafe extern "system" fn(this: *mut *mut Self, route: *mut ::core::ffi::c_void, speedinmeterspersecond: i32) -> ::windows_sys::core::HRESULT,
    pub StartTracking: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RepeatLastAudioNotification: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub AudioMeasurementSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GuidanceAudioMeasurementSystem) -> ::windows_sys::core::HRESULT,
    pub SetAudioMeasurementSystem: unsafe extern "system" fn(this: *mut *mut Self, value: GuidanceAudioMeasurementSystem) -> ::windows_sys::core::HRESULT,
    pub AudioNotifications: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GuidanceAudioNotifications) -> ::windows_sys::core::HRESULT,
    pub SetAudioNotifications: unsafe extern "system" fn(this: *mut *mut Self, value: GuidanceAudioNotifications) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GuidanceUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GuidanceUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGuidanceUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGuidanceUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub DestinationReached: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DestinationReached: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDestinationReached: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDestinationReached: usize,
    #[cfg(feature = "Foundation")]
    pub Rerouting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Rerouting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRerouting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRerouting: usize,
    #[cfg(feature = "Foundation")]
    pub Rerouted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Rerouted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRerouted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRerouted: usize,
    #[cfg(feature = "Foundation")]
    pub RerouteFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RerouteFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRerouteFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRerouteFailed: usize,
    #[cfg(feature = "Foundation")]
    pub UserLocationLost: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserLocationLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserLocationLost: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserLocationLost: usize,
    #[cfg(feature = "Foundation")]
    pub UserLocationRestored: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserLocationRestored: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserLocationRestored: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserLocationRestored: usize,
    pub SetGuidanceVoice: unsafe extern "system" fn(this: *mut *mut Self, voiceid: i32, voicefolder: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub UpdateUserLocation: unsafe extern "system" fn(this: *mut *mut Self, userlocation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    UpdateUserLocation: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub UpdateUserLocationWithPositionOverride: unsafe extern "system" fn(this: *mut *mut Self, userlocation: *mut ::core::ffi::c_void, positionoverride: super::super::super::Devices::Geolocation::BasicGeoposition) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    UpdateUserLocationWithPositionOverride: usize,
}
#[repr(C)]
pub struct IGuidanceNavigator2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AudioNotificationRequested: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioNotificationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioNotificationRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioNotificationRequested: usize,
    pub IsGuidanceAudioMuted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsGuidanceAudioMuted: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceNavigatorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceNavigatorStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UseAppProvidedVoice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceReroutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Route: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceRoadSegment {
    pub base__: ::windows_sys::core::IInspectable,
    pub RoadName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ShortRoadName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SpeedLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TravelTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TravelTime: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsHighway: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsTunnel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsTollRoad: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceRoadSegment2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsScenic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceRoadSignpost {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExitNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Exit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub ForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ForegroundColor: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ExitDirections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExitDirections: usize,
}
#[repr(C)]
pub struct IGuidanceRoute {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub Distance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Maneuvers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Maneuvers: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    BoundingBox: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RoadSegments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RoadSegments: usize,
    pub ConvertToMapRoute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceRouteStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanCreateFromMapRoute: unsafe extern "system" fn(this: *mut *mut Self, maproute: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TryCreateFromMapRoute: unsafe extern "system" fn(this: *mut *mut Self, maproute: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceTelemetryCollector {
    pub base__: ::windows_sys::core::IInspectable,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ClearLocalData: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SpeedTrigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetSpeedTrigger: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub UploadFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetUploadFrequency: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceTelemetryCollectorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGuidanceUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GuidanceMode) -> ::windows_sys::core::HRESULT,
    pub NextManeuver: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NextManeuverDistance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AfterNextManeuver: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AfterNextManeuverDistance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DistanceToDestination: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ElapsedDistance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ElapsedTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ElapsedTime: usize,
    #[cfg(feature = "Foundation")]
    pub TimeToDestination: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToDestination: usize,
    pub RoadName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Route: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CurrentLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsNewManeuver: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub LaneInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaneInfo: usize,
}
