pub type CustomMapTileDataSource = *mut ::core::ffi::c_void;
pub type HttpMapTileDataSource = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICustomMapTileDataSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BitmapRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BitmapRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBitmapRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBitmapRequested: usize,
}
#[repr(C)]
pub struct ICustomMapTileDataSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpMapTileDataSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub UriFormatString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetUriFormatString: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AdditionalRequestHeaders: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AdditionalRequestHeaders: usize,
    pub AllowCaching: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowCaching: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UriRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UriRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUriRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUriRequested: usize,
}
#[repr(C)]
pub struct IHttpMapTileDataSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstanceWithUriFormatString: unsafe extern "system" fn(this: *mut *mut Self, uriformatstring: ::windows_sys::core::HSTRING, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILocalMapTileDataSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub UriFormatString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetUriFormatString: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UriRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UriRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUriRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUriRequested: usize,
}
#[repr(C)]
pub struct ILocalMapTileDataSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstanceWithUriFormatString: unsafe extern "system" fn(this: *mut *mut Self, uriformatstring: ::windows_sys::core::HSTRING, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapActualCameraChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Camera: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapActualCameraChangedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapCameraChangeReason) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapActualCameraChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Camera: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapActualCameraChangingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapCameraChangeReason) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapBillboard {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetLocation: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetLocation: usize,
    #[cfg(feature = "Foundation")]
    pub NormalizedAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NormalizedAnchorPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetNormalizedAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNormalizedAnchorPoint: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Image: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Image: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetImage: usize,
    pub CollisionBehaviorDesired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapElementCollisionBehavior) -> ::windows_sys::core::HRESULT,
    pub SetCollisionBehaviorDesired: unsafe extern "system" fn(this: *mut *mut Self, value: MapElementCollisionBehavior) -> ::windows_sys::core::HRESULT,
    pub ReferenceCamera: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapBillboardFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceFromCamera: unsafe extern "system" fn(this: *mut *mut Self, camera: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapBillboardStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NormalizedAnchorPointProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CollisionBehaviorDesiredProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapCamera {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetLocation: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetLocation: usize,
    pub Heading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHeading: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Pitch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetPitch: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Roll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRoll: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub FieldOfView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFieldOfView: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapCameraFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateInstanceWithLocation: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateInstanceWithLocation: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateInstanceWithLocationAndHeading: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, headingindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateInstanceWithLocationAndHeading: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateInstanceWithLocationHeadingAndPitch: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, headingindegrees: f64, pitchindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateInstanceWithLocationHeadingAndPitch: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateInstanceWithLocationHeadingPitchRollAndFieldOfView: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, headingindegrees: f64, pitchindegrees: f64, rollindegrees: f64, fieldofviewindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateInstanceWithLocationHeadingPitchRollAndFieldOfView: usize,
}
#[repr(C)]
pub struct IMapContextRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MapElements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MapElements: usize,
}
#[repr(C)]
pub struct IMapControl {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Center: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Center: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetCenter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetCenter: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    pub ColorScheme: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapColorScheme) -> ::windows_sys::core::HRESULT,
    pub SetColorScheme: unsafe extern "system" fn(this: *mut *mut Self, value: MapColorScheme) -> ::windows_sys::core::HRESULT,
    pub DesiredPitch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDesiredPitch: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Heading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHeading: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub LandmarksVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetLandmarksVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub LoadingStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapLoadingStatus) -> ::windows_sys::core::HRESULT,
    pub MapServiceToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMapServiceToken: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MaxZoomLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MinZoomLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub PedestrianFeaturesVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPedestrianFeaturesVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Pitch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Style: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapStyle) -> ::windows_sys::core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(this: *mut *mut Self, value: MapStyle) -> ::windows_sys::core::HRESULT,
    pub TrafficFlowVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetTrafficFlowVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TransformOrigin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransformOrigin: usize,
    #[cfg(feature = "Foundation")]
    pub SetTransformOrigin: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTransformOrigin: usize,
    pub WatermarkMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapWatermarkMode) -> ::windows_sys::core::HRESULT,
    pub SetWatermarkMode: unsafe extern "system" fn(this: *mut *mut Self, value: MapWatermarkMode) -> ::windows_sys::core::HRESULT,
    pub ZoomLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetZoomLevel: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MapElements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MapElements: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Routes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Routes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TileSources: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TileSources: usize,
    #[cfg(feature = "Foundation")]
    pub CenterChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CenterChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCenterChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCenterChanged: usize,
    #[cfg(feature = "Foundation")]
    pub HeadingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHeadingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHeadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub LoadingStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadingStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLoadingStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLoadingStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub MapDoubleTapped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapDoubleTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapDoubleTapped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapDoubleTapped: usize,
    #[cfg(feature = "Foundation")]
    pub MapHolding: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapHolding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapHolding: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapHolding: usize,
    #[cfg(feature = "Foundation")]
    pub MapTapped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapTapped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapTapped: usize,
    #[cfg(feature = "Foundation")]
    pub PitchChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PitchChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePitchChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePitchChanged: usize,
    #[cfg(feature = "Foundation")]
    pub TransformOriginChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransformOriginChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransformOriginChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransformOriginChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ZoomLevelChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ZoomLevelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveZoomLevelChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveZoomLevelChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindMapElementsAtOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindMapElementsAtOffset: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetLocationFromOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::super::super::Foundation::Point, location: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetLocationFromOffset: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetOffsetFromLocation: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, offset: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetOffsetFromLocation: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub IsLocationInView: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, isinview: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    IsLocationInView: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TrySetViewBoundsAsync: unsafe extern "system" fn(this: *mut *mut Self, bounds: *mut ::core::ffi::c_void, margin: *mut ::core::ffi::c_void, animation: MapAnimationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TrySetViewBoundsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TrySetViewWithCenterAsync: unsafe extern "system" fn(this: *mut *mut Self, center: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TrySetViewWithCenterAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TrySetViewWithCenterAndZoomAsync: unsafe extern "system" fn(this: *mut *mut Self, center: *mut ::core::ffi::c_void, zoomlevel: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TrySetViewWithCenterAndZoomAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TrySetViewWithCenterZoomHeadingAndPitchAsync: unsafe extern "system" fn(this: *mut *mut Self, center: *mut ::core::ffi::c_void, zoomlevel: *mut ::core::ffi::c_void, heading: *mut ::core::ffi::c_void, desiredpitch: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TrySetViewWithCenterZoomHeadingAndPitchAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync: unsafe extern "system" fn(this: *mut *mut Self, center: *mut ::core::ffi::c_void, zoomlevel: *mut ::core::ffi::c_void, heading: *mut ::core::ffi::c_void, desiredpitch: *mut ::core::ffi::c_void, animation: MapAnimationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync: usize,
}
#[repr(C)]
pub struct IMapControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BusinessLandmarksVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetBusinessLandmarksVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TransitFeaturesVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetTransitFeaturesVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PanInteractionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapPanInteractionMode) -> ::windows_sys::core::HRESULT,
    pub SetPanInteractionMode: unsafe extern "system" fn(this: *mut *mut Self, value: MapPanInteractionMode) -> ::windows_sys::core::HRESULT,
    pub RotateInteractionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapInteractionMode) -> ::windows_sys::core::HRESULT,
    pub SetRotateInteractionMode: unsafe extern "system" fn(this: *mut *mut Self, value: MapInteractionMode) -> ::windows_sys::core::HRESULT,
    pub TiltInteractionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapInteractionMode) -> ::windows_sys::core::HRESULT,
    pub SetTiltInteractionMode: unsafe extern "system" fn(this: *mut *mut Self, value: MapInteractionMode) -> ::windows_sys::core::HRESULT,
    pub ZoomInteractionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapInteractionMode) -> ::windows_sys::core::HRESULT,
    pub SetZoomInteractionMode: unsafe extern "system" fn(this: *mut *mut Self, value: MapInteractionMode) -> ::windows_sys::core::HRESULT,
    pub Is3DSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStreetsideSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Scene: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetScene: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ActualCamera: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TargetCamera: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CustomExperience: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCustomExperience: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MapElementClick: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapElementClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapElementClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapElementClick: usize,
    #[cfg(feature = "Foundation")]
    pub MapElementPointerEntered: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapElementPointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapElementPointerEntered: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapElementPointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub MapElementPointerExited: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapElementPointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapElementPointerExited: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapElementPointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub ActualCameraChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActualCameraChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActualCameraChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActualCameraChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ActualCameraChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActualCameraChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActualCameraChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActualCameraChanging: usize,
    #[cfg(feature = "Foundation")]
    pub TargetCameraChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetCameraChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTargetCameraChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTargetCameraChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CustomExperienceChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CustomExperienceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCustomExperienceChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCustomExperienceChanged: usize,
    pub StartContinuousRotate: unsafe extern "system" fn(this: *mut *mut Self, rateindegreespersecond: f64) -> ::windows_sys::core::HRESULT,
    pub StopContinuousRotate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StartContinuousTilt: unsafe extern "system" fn(this: *mut *mut Self, rateindegreespersecond: f64) -> ::windows_sys::core::HRESULT,
    pub StopContinuousTilt: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StartContinuousZoom: unsafe extern "system" fn(this: *mut *mut Self, rateofchangepersecond: f64) -> ::windows_sys::core::HRESULT,
    pub StopContinuousZoom: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryRotateAsync: unsafe extern "system" fn(this: *mut *mut Self, degrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRotateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryRotateToAsync: unsafe extern "system" fn(this: *mut *mut Self, angleindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRotateToAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryTiltAsync: unsafe extern "system" fn(this: *mut *mut Self, degrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTiltAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryTiltToAsync: unsafe extern "system" fn(this: *mut *mut Self, angleindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTiltToAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryZoomInAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryZoomInAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryZoomOutAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryZoomOutAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryZoomToAsync: unsafe extern "system" fn(this: *mut *mut Self, zoomlevel: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryZoomToAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetSceneAsync: unsafe extern "system" fn(this: *mut *mut Self, scene: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetSceneAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetSceneWithAnimationAsync: unsafe extern "system" fn(this: *mut *mut Self, scene: *mut ::core::ffi::c_void, animationkind: MapAnimationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetSceneWithAnimationAsync: usize,
}
#[repr(C)]
pub struct IMapControl3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MapRightTapped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapRightTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapRightTapped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapRightTapped: usize,
}
#[repr(C)]
pub struct IMapControl4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BusinessLandmarksEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetBusinessLandmarksEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TransitFeaturesEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetTransitFeaturesEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub GetVisibleRegion: unsafe extern "system" fn(this: *mut *mut Self, region: MapVisibleRegionKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    GetVisibleRegion: usize,
}
#[repr(C)]
pub struct IMapControl5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MapProjection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapProjection) -> ::windows_sys::core::HRESULT,
    pub SetMapProjection: unsafe extern "system" fn(this: *mut *mut Self, value: MapProjection) -> ::windows_sys::core::HRESULT,
    pub StyleSheet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetStyleSheet: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ViewPadding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetViewPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Thickness) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MapContextRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapContextRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapContextRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapContextRequested: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindMapElementsAtOffsetWithRadius: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::super::super::Foundation::Point, radius: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindMapElementsAtOffsetWithRadius: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetLocationFromOffsetWithReferenceSystem: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetLocationFromOffsetWithReferenceSystem: usize,
    pub StartContinuousPan: unsafe extern "system" fn(this: *mut *mut Self, horizontalpixelspersecond: f64, verticalpixelspersecond: f64) -> ::windows_sys::core::HRESULT,
    pub StopContinuousPan: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryPanAsync: unsafe extern "system" fn(this: *mut *mut Self, horizontalpixels: f64, verticalpixels: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPanAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TryPanToAsync: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TryPanToAsync: usize,
}
#[repr(C)]
pub struct IMapControl6 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Layers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Layers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetLayers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetLayers: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TryGetLocationFromOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::super::super::Foundation::Point, location: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TryGetLocationFromOffset: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TryGetLocationFromOffsetWithReferenceSystem: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TryGetLocationFromOffsetWithReferenceSystem: usize,
}
#[repr(C)]
pub struct IMapControl7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Region: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRegion: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapControl8 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanTiltDown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanTiltUp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanZoomIn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanZoomOut: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapControlBusinessLandmarkClickEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch"))]
    pub LocalLocations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch")))]
    LocalLocations: usize,
}
#[repr(C)]
pub struct IMapControlBusinessLandmarkPointerEnteredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch"))]
    pub LocalLocations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch")))]
    LocalLocations: usize,
}
#[repr(C)]
pub struct IMapControlBusinessLandmarkPointerExitedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch"))]
    pub LocalLocations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch")))]
    LocalLocations: usize,
}
#[repr(C)]
pub struct IMapControlBusinessLandmarkRightTappedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch"))]
    pub LocalLocations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch")))]
    LocalLocations: usize,
}
#[repr(C)]
pub struct IMapControlDataHelper {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BusinessLandmarkClick: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BusinessLandmarkClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBusinessLandmarkClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBusinessLandmarkClick: usize,
    #[cfg(feature = "Foundation")]
    pub TransitFeatureClick: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransitFeatureClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransitFeatureClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransitFeatureClick: usize,
    #[cfg(feature = "Foundation")]
    pub BusinessLandmarkRightTapped: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BusinessLandmarkRightTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBusinessLandmarkRightTapped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBusinessLandmarkRightTapped: usize,
    #[cfg(feature = "Foundation")]
    pub TransitFeatureRightTapped: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransitFeatureRightTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransitFeatureRightTapped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransitFeatureRightTapped: usize,
}
#[repr(C)]
pub struct IMapControlDataHelper2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BusinessLandmarkPointerEntered: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BusinessLandmarkPointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBusinessLandmarkPointerEntered: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBusinessLandmarkPointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub TransitFeaturePointerEntered: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransitFeaturePointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransitFeaturePointerEntered: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransitFeaturePointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub BusinessLandmarkPointerExited: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BusinessLandmarkPointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBusinessLandmarkPointerExited: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBusinessLandmarkPointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub TransitFeaturePointerExited: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransitFeaturePointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransitFeaturePointerExited: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransitFeaturePointerExited: usize,
}
#[repr(C)]
pub struct IMapControlDataHelperFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, map: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapControlDataHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMapControl: unsafe extern "system" fn(this: *mut *mut Self, rasterrendermode: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapControlStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CenterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ChildrenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ColorSchemeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DesiredPitchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeadingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LandmarksVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LoadingStatusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MapServiceTokenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PedestrianFeaturesVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PitchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TrafficFlowVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TransformOriginProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WatermarkModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ZoomLevelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MapElementsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RoutesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TileSourcesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LocationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub GetLocation: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    GetLocation: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetLocation: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetLocation: usize,
    pub NormalizedAnchorPointProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetNormalizedAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetNormalizedAnchorPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetNormalizedAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNormalizedAnchorPoint: usize,
}
#[repr(C)]
pub struct IMapControlStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BusinessLandmarksVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TransitFeaturesVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PanInteractionModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RotateInteractionModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TiltInteractionModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ZoomInteractionModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Is3DSupportedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsStreetsideSupportedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SceneProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapControlStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BusinessLandmarksEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TransitFeaturesEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapControlStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MapProjectionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StyleSheetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ViewPaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapControlStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LayersProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapControlStatics7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RegionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapControlStatics8 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanTiltDownProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanTiltUpProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanZoomInProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanZoomOutProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapControlTransitFeatureClickEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TransitProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransitProperties: usize,
}
#[repr(C)]
pub struct IMapControlTransitFeaturePointerEnteredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TransitProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransitProperties: usize,
}
#[repr(C)]
pub struct IMapControlTransitFeaturePointerExitedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TransitProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransitProperties: usize,
}
#[repr(C)]
pub struct IMapControlTransitFeatureRightTappedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TransitProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransitProperties: usize,
}
#[repr(C)]
pub struct IMapCustomExperience {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMapCustomExperienceChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMapCustomExperienceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElement {
    pub base__: ::windows_sys::core::IInspectable,
    pub ZIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElement2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MapTabIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMapTabIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElement3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MapStyleSheetEntry: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMapStyleSheetEntry: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MapStyleSheetEntryState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMapStyleSheetEntryState: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElement3D {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetLocation: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetLocation: usize,
    pub Model: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetModel: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Heading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHeading: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Pitch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetPitch: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Roll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRoll: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Scale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetScale: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetScale: usize,
}
#[repr(C)]
pub struct IMapElement3DStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeadingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PitchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RollProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ScaleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElement4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElementClickEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MapElements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MapElements: usize,
}
#[repr(C)]
pub struct IMapElementFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElementPointerEnteredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    pub MapElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElementPointerExitedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    pub MapElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElementStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ZIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElementStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MapTabIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElementStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MapStyleSheetEntryProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MapStyleSheetEntryStateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TagProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElementStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElementsLayer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub MapElements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MapElements: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetMapElements: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetMapElements: usize,
    #[cfg(feature = "Foundation")]
    pub MapElementClick: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapElementClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapElementClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapElementClick: usize,
    #[cfg(feature = "Foundation")]
    pub MapElementPointerEntered: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapElementPointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapElementPointerEntered: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapElementPointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub MapElementPointerExited: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapElementPointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapElementPointerExited: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapElementPointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub MapContextRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapContextRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapContextRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapContextRequested: usize,
}
#[repr(C)]
pub struct IMapElementsLayerClickEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MapElements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MapElements: usize,
}
#[repr(C)]
pub struct IMapElementsLayerContextRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MapElements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MapElements: usize,
}
#[repr(C)]
pub struct IMapElementsLayerPointerEnteredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    pub MapElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElementsLayerPointerExitedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    pub MapElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapElementsLayerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MapElementsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapIcon {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetLocation: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetLocation: usize,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NormalizedAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NormalizedAnchorPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetNormalizedAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNormalizedAnchorPoint: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Image: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Image: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetImage: usize,
}
#[repr(C)]
pub struct IMapIcon2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CollisionBehaviorDesired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapElementCollisionBehavior) -> ::windows_sys::core::HRESULT,
    pub SetCollisionBehaviorDesired: unsafe extern "system" fn(this: *mut *mut Self, value: MapElementCollisionBehavior) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapIconStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TitleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NormalizedAnchorPointProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapIconStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CollisionBehaviorDesiredProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapInputEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
}
#[repr(C)]
pub struct IMapItemsControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemsSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemsSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub ItemTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapItemsControlStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemsSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapLayer {
    pub base__: ::windows_sys::core::IInspectable,
    pub MapTabIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMapTabIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ZIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapLayerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapLayerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MapTabIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ZIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapModel3D {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMapModel3DFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapModel3DStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateFrom3MFAsync: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateFrom3MFAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateFrom3MFWithShadingOptionAsync: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, shadingoption: MapModel3DShadingOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateFrom3MFWithShadingOptionAsync: usize,
}
#[repr(C)]
pub struct IMapPolygon {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetPath: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetPath: usize,
    pub StrokeColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetStrokeColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Color) -> ::windows_sys::core::HRESULT,
    pub StrokeThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetStrokeThickness: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub StrokeDashed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStrokeDashed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub FillColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetFillColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Color) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapPolygon2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub Paths: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    Paths: usize,
}
#[repr(C)]
pub struct IMapPolygonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PathProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeDashedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapPolyline {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetPath: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetPath: usize,
    pub StrokeColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetStrokeColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Color) -> ::windows_sys::core::HRESULT,
    pub StrokeThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetStrokeThickness: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub StrokeDashed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStrokeDashed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapPolylineStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PathProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeDashedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapRightTappedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
}
#[repr(C)]
pub struct IMapRouteView {
    pub base__: ::windows_sys::core::IInspectable,
    pub RouteColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetRouteColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Color) -> ::windows_sys::core::HRESULT,
    pub OutlineColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetOutlineColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Services_Maps")]
    pub Route: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Services_Maps"))]
    Route: usize,
}
#[repr(C)]
pub struct IMapRouteViewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Services_Maps")]
    pub CreateInstanceWithMapRoute: unsafe extern "system" fn(this: *mut *mut Self, route: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Services_Maps"))]
    CreateInstanceWithMapRoute: usize,
}
#[repr(C)]
pub struct IMapScene {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetCamera: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TargetCameraChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetCameraChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTargetCameraChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTargetCameraChanged: usize,
}
#[repr(C)]
pub struct IMapSceneStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromBoundingBox: unsafe extern "system" fn(this: *mut *mut Self, bounds: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromBoundingBox: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromBoundingBoxWithHeadingAndPitch: unsafe extern "system" fn(this: *mut *mut Self, bounds: *mut ::core::ffi::c_void, headingindegrees: f64, pitchindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromBoundingBoxWithHeadingAndPitch: usize,
    pub CreateFromCamera: unsafe extern "system" fn(this: *mut *mut Self, camera: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromLocation: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromLocation: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromLocationWithHeadingAndPitch: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, headingindegrees: f64, pitchindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromLocationWithHeadingAndPitch: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromLocationAndRadius: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, radiusinmeters: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromLocationAndRadius: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromLocationAndRadiusWithHeadingAndPitch: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, radiusinmeters: f64, headingindegrees: f64, pitchindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromLocationAndRadiusWithHeadingAndPitch: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub CreateFromLocations: unsafe extern "system" fn(this: *mut *mut Self, locations: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    CreateFromLocations: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub CreateFromLocationsWithHeadingAndPitch: unsafe extern "system" fn(this: *mut *mut Self, locations: *mut ::core::ffi::c_void, headingindegrees: f64, pitchindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    CreateFromLocationsWithHeadingAndPitch: usize,
}
#[repr(C)]
pub struct IMapStyleSheet {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMapStyleSheetEntriesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Area: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Airport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Cemetery: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Continent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Education: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IndigenousPeoplesReserve: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Island: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Medical: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Military: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Nautical: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Neighborhood: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Runway: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sand: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ShoppingCenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Stadium: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Vegetation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Forest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GolfCourse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Park: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PlayingField: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Reserve: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Point: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NaturalPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Peak: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VolcanicPeak: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WaterPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PointOfInterest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Business: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FoodPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PopulatedPlace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Capital: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AdminDistrictCapital: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CountryRegionCapital: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RoadShield: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RoadExit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Transit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Political: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CountryRegion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AdminDistrict: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub District: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Structure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Building: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EducationBuilding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MedicalBuilding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TransitBuilding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Transportation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Road: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ControlledAccessHighway: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HighSpeedRamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Highway: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MajorRoad: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ArterialRoad: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Street: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Ramp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UnpavedStreet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TollRoad: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Railway: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Trail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WaterRoute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Water: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub River: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RouteLine: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WalkingRoute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DrivingRoute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapStyleSheetEntryStatesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Disabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Hover: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Selected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapStyleSheetStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Aerial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AerialWithOverlay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RoadLight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RoadDark: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RoadHighContrastLight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RoadHighContrastDark: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Combine: unsafe extern "system" fn(this: *mut *mut Self, stylesheets: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Combine: usize,
    pub ParseFromJson: unsafe extern "system" fn(this: *mut *mut Self, styleasjson: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParseFromJson: unsafe extern "system" fn(this: *mut *mut Self, styleasjson: ::windows_sys::core::HSTRING, stylesheet: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTargetCameraChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Camera: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTargetCameraChangedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapCameraChangeReason) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTileBitmapRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub PixelData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PixelData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPixelData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPixelData: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTileBitmapRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTileBitmapRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub X: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Y: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ZoomLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTileBitmapRequestedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FrameIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTileDataSource {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMapTileDataSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTileSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDataSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Layer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapTileLayer) -> ::windows_sys::core::HRESULT,
    pub SetLayer: unsafe extern "system" fn(this: *mut *mut Self, value: MapTileLayer) -> ::windows_sys::core::HRESULT,
    pub ZoomLevelRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapZoomLevelRange) -> ::windows_sys::core::HRESULT,
    pub SetZoomLevelRange: unsafe extern "system" fn(this: *mut *mut Self, value: MapZoomLevelRange) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Bounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Bounds: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetBounds: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetBounds: usize,
    pub AllowOverstretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowOverstretch: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsFadingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFadingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsTransparencyEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTransparencyEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsRetryEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRetryEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ZIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub TilePixelSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTilePixelSize: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTileSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AnimationState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MapTileAnimationState) -> ::windows_sys::core::HRESULT,
    pub AutoPlay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoPlay: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub FrameCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetFrameCount: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FrameDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetFrameDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFrameDuration: usize,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTileSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstanceWithDataSource: unsafe extern "system" fn(this: *mut *mut Self, datasource: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstanceWithDataSourceAndZoomRange: unsafe extern "system" fn(this: *mut *mut Self, datasource: *mut ::core::ffi::c_void, zoomlevelrange: MapZoomLevelRange, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateInstanceWithDataSourceZoomRangeAndBounds: unsafe extern "system" fn(this: *mut *mut Self, datasource: *mut ::core::ffi::c_void, zoomlevelrange: MapZoomLevelRange, bounds: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateInstanceWithDataSourceZoomRangeAndBounds: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize: unsafe extern "system" fn(this: *mut *mut Self, datasource: *mut ::core::ffi::c_void, zoomlevelrange: MapZoomLevelRange, bounds: *mut ::core::ffi::c_void, tilesizeinpixels: i32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize: usize,
}
#[repr(C)]
pub struct IMapTileSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LayerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ZoomLevelRangeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BoundsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AllowOverstretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsFadingEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTransparencyEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsRetryEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ZIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TilePixelSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTileSourceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AnimationStateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AutoPlayProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FrameCountProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FrameDurationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTileUriRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTileUriRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTileUriRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub X: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Y: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ZoomLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMapTileUriRequestedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FrameIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStreetsideExperience {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddressTextVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAddressTextVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CursorVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCursorVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub OverviewMapVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetOverviewMapVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub StreetLabelsVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStreetLabelsVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ExitButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetExitButtonVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ZoomButtonsVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetZoomButtonsVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStreetsideExperienceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithPanorama: unsafe extern "system" fn(this: *mut *mut Self, panorama: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstanceWithPanoramaHeadingPitchAndFieldOfView: unsafe extern "system" fn(this: *mut *mut Self, panorama: *mut ::core::ffi::c_void, headingindegrees: f64, pitchindegrees: f64, fieldofviewindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStreetsidePanorama {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
}
#[repr(C)]
pub struct IStreetsidePanoramaStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindNearbyWithLocationAsync: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindNearbyWithLocationAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindNearbyWithLocationAndRadiusAsync: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, radiusinmeters: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindNearbyWithLocationAndRadiusAsync: usize,
}
pub type LocalMapTileDataSource = *mut ::core::ffi::c_void;
pub type MapActualCameraChangedEventArgs = *mut ::core::ffi::c_void;
pub type MapActualCameraChangingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapAnimationKind(pub i32);
impl MapAnimationKind {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Linear: Self = Self(2i32);
    pub const Bow: Self = Self(3i32);
}
impl ::core::marker::Copy for MapAnimationKind {}
impl ::core::clone::Clone for MapAnimationKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapBillboard = *mut ::core::ffi::c_void;
pub type MapCamera = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapCameraChangeReason(pub i32);
impl MapCameraChangeReason {
    pub const System: Self = Self(0i32);
    pub const UserInteraction: Self = Self(1i32);
    pub const Programmatic: Self = Self(2i32);
}
impl ::core::marker::Copy for MapCameraChangeReason {}
impl ::core::clone::Clone for MapCameraChangeReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapColorScheme(pub i32);
impl MapColorScheme {
    pub const Light: Self = Self(0i32);
    pub const Dark: Self = Self(1i32);
}
impl ::core::marker::Copy for MapColorScheme {}
impl ::core::clone::Clone for MapColorScheme {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapContextRequestedEventArgs = *mut ::core::ffi::c_void;
pub type MapControl = *mut ::core::ffi::c_void;
pub type MapControlBusinessLandmarkClickEventArgs = *mut ::core::ffi::c_void;
pub type MapControlBusinessLandmarkPointerEnteredEventArgs = *mut ::core::ffi::c_void;
pub type MapControlBusinessLandmarkPointerExitedEventArgs = *mut ::core::ffi::c_void;
pub type MapControlBusinessLandmarkRightTappedEventArgs = *mut ::core::ffi::c_void;
pub type MapControlDataHelper = *mut ::core::ffi::c_void;
pub type MapControlTransitFeatureClickEventArgs = *mut ::core::ffi::c_void;
pub type MapControlTransitFeaturePointerEnteredEventArgs = *mut ::core::ffi::c_void;
pub type MapControlTransitFeaturePointerExitedEventArgs = *mut ::core::ffi::c_void;
pub type MapControlTransitFeatureRightTappedEventArgs = *mut ::core::ffi::c_void;
pub type MapCustomExperience = *mut ::core::ffi::c_void;
pub type MapCustomExperienceChangedEventArgs = *mut ::core::ffi::c_void;
pub type MapElement = *mut ::core::ffi::c_void;
pub type MapElement3D = *mut ::core::ffi::c_void;
pub type MapElementClickEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapElementCollisionBehavior(pub i32);
impl MapElementCollisionBehavior {
    pub const Hide: Self = Self(0i32);
    pub const RemainVisible: Self = Self(1i32);
}
impl ::core::marker::Copy for MapElementCollisionBehavior {}
impl ::core::clone::Clone for MapElementCollisionBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapElementPointerEnteredEventArgs = *mut ::core::ffi::c_void;
pub type MapElementPointerExitedEventArgs = *mut ::core::ffi::c_void;
pub type MapElementsLayer = *mut ::core::ffi::c_void;
pub type MapElementsLayerClickEventArgs = *mut ::core::ffi::c_void;
pub type MapElementsLayerContextRequestedEventArgs = *mut ::core::ffi::c_void;
pub type MapElementsLayerPointerEnteredEventArgs = *mut ::core::ffi::c_void;
pub type MapElementsLayerPointerExitedEventArgs = *mut ::core::ffi::c_void;
pub type MapIcon = *mut ::core::ffi::c_void;
pub type MapInputEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapInteractionMode(pub i32);
impl MapInteractionMode {
    pub const Auto: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const GestureOnly: Self = Self(2i32);
    pub const PointerAndKeyboard: Self = Self(2i32);
    pub const ControlOnly: Self = Self(3i32);
    pub const GestureAndControl: Self = Self(4i32);
    pub const PointerKeyboardAndControl: Self = Self(4i32);
    pub const PointerOnly: Self = Self(5i32);
}
impl ::core::marker::Copy for MapInteractionMode {}
impl ::core::clone::Clone for MapInteractionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapItemsControl = *mut ::core::ffi::c_void;
pub type MapLayer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapLoadingStatus(pub i32);
impl MapLoadingStatus {
    pub const Loading: Self = Self(0i32);
    pub const Loaded: Self = Self(1i32);
    pub const DataUnavailable: Self = Self(2i32);
    pub const DownloadedMapsManagerUnavailable: Self = Self(3i32);
}
impl ::core::marker::Copy for MapLoadingStatus {}
impl ::core::clone::Clone for MapLoadingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapModel3D = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapModel3DShadingOption(pub i32);
impl MapModel3DShadingOption {
    pub const Default: Self = Self(0i32);
    pub const Flat: Self = Self(1i32);
    pub const Smooth: Self = Self(2i32);
}
impl ::core::marker::Copy for MapModel3DShadingOption {}
impl ::core::clone::Clone for MapModel3DShadingOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapPanInteractionMode(pub i32);
impl MapPanInteractionMode {
    pub const Auto: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
impl ::core::marker::Copy for MapPanInteractionMode {}
impl ::core::clone::Clone for MapPanInteractionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapPolygon = *mut ::core::ffi::c_void;
pub type MapPolyline = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapProjection(pub i32);
impl MapProjection {
    pub const WebMercator: Self = Self(0i32);
    pub const Globe: Self = Self(1i32);
}
impl ::core::marker::Copy for MapProjection {}
impl ::core::clone::Clone for MapProjection {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapRightTappedEventArgs = *mut ::core::ffi::c_void;
pub type MapRouteView = *mut ::core::ffi::c_void;
pub type MapScene = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapStyle(pub i32);
impl MapStyle {
    pub const None: Self = Self(0i32);
    pub const Road: Self = Self(1i32);
    pub const Aerial: Self = Self(2i32);
    pub const AerialWithRoads: Self = Self(3i32);
    pub const Terrain: Self = Self(4i32);
    pub const Aerial3D: Self = Self(5i32);
    pub const Aerial3DWithRoads: Self = Self(6i32);
    pub const Custom: Self = Self(7i32);
}
impl ::core::marker::Copy for MapStyle {}
impl ::core::clone::Clone for MapStyle {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapStyleSheet = *mut ::core::ffi::c_void;
pub type MapTargetCameraChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapTileAnimationState(pub i32);
impl MapTileAnimationState {
    pub const Stopped: Self = Self(0i32);
    pub const Paused: Self = Self(1i32);
    pub const Playing: Self = Self(2i32);
}
impl ::core::marker::Copy for MapTileAnimationState {}
impl ::core::clone::Clone for MapTileAnimationState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapTileBitmapRequest = *mut ::core::ffi::c_void;
pub type MapTileBitmapRequestDeferral = *mut ::core::ffi::c_void;
pub type MapTileBitmapRequestedEventArgs = *mut ::core::ffi::c_void;
pub type MapTileDataSource = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapTileLayer(pub i32);
impl MapTileLayer {
    pub const LabelOverlay: Self = Self(0i32);
    pub const RoadOverlay: Self = Self(1i32);
    pub const AreaOverlay: Self = Self(2i32);
    pub const BackgroundOverlay: Self = Self(3i32);
    pub const BackgroundReplacement: Self = Self(4i32);
}
impl ::core::marker::Copy for MapTileLayer {}
impl ::core::clone::Clone for MapTileLayer {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MapTileSource = *mut ::core::ffi::c_void;
pub type MapTileUriRequest = *mut ::core::ffi::c_void;
pub type MapTileUriRequestDeferral = *mut ::core::ffi::c_void;
pub type MapTileUriRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapVisibleRegionKind(pub i32);
impl MapVisibleRegionKind {
    pub const Near: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
}
impl ::core::marker::Copy for MapVisibleRegionKind {}
impl ::core::clone::Clone for MapVisibleRegionKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapWatermarkMode(pub i32);
impl MapWatermarkMode {
    pub const Automatic: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
impl ::core::marker::Copy for MapWatermarkMode {}
impl ::core::clone::Clone for MapWatermarkMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
pub struct MapZoomLevelRange {
    pub Min: f64,
    pub Max: f64,
}
impl ::core::marker::Copy for MapZoomLevelRange {}
impl ::core::clone::Clone for MapZoomLevelRange {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StreetsideExperience = *mut ::core::ffi::c_void;
pub type StreetsidePanorama = *mut ::core::ffi::c_void;
