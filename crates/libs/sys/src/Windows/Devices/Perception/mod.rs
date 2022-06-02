#[cfg(feature = "Devices_Perception_Provider")]
pub mod Provider;
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IKnownCameraIntrinsicsPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub FocalLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FocalLength: usize,
    #[cfg(feature = "deprecated")]
    pub PrincipalPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PrincipalPoint: usize,
    #[cfg(feature = "deprecated")]
    pub RadialDistortion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RadialDistortion: usize,
    #[cfg(feature = "deprecated")]
    pub TangentialDistortion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TangentialDistortion: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IKnownPerceptionColorFrameSourcePropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Exposure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Exposure: usize,
    #[cfg(feature = "deprecated")]
    pub AutoExposureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AutoExposureEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExposureCompensation: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IKnownPerceptionDepthFrameSourcePropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub MinDepth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MinDepth: usize,
    #[cfg(feature = "deprecated")]
    pub MaxDepth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MaxDepth: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub PhysicalDeviceIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PhysicalDeviceIds: usize,
    #[cfg(feature = "deprecated")]
    pub FrameKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameKind: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceModelVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceModelVersion: usize,
    #[cfg(feature = "deprecated")]
    pub EnclosureLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EnclosureLocation: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IKnownPerceptionInfraredFrameSourcePropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Exposure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Exposure: usize,
    #[cfg(feature = "deprecated")]
    pub AutoExposureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AutoExposureEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExposureCompensation: usize,
    #[cfg(feature = "deprecated")]
    pub ActiveIlluminationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ActiveIlluminationEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub AmbientSubtractionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AmbientSubtractionEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub StructureLightPatternEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StructureLightPatternEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub InterleavedIlluminationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InterleavedIlluminationEnabled: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IKnownPerceptionVideoFrameSourcePropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub VideoProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    VideoProfile: usize,
    #[cfg(feature = "deprecated")]
    pub SupportedVideoProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportedVideoProfiles: usize,
    #[cfg(feature = "deprecated")]
    pub AvailableVideoProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AvailableVideoProfiles: usize,
    #[cfg(feature = "deprecated")]
    pub IsMirrored: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsMirrored: usize,
    #[cfg(feature = "deprecated")]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CameraIntrinsics: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IKnownPerceptionVideoProfilePropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BitmapPixelFormat: usize,
    #[cfg(feature = "deprecated")]
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BitmapAlphaMode: usize,
    #[cfg(feature = "deprecated")]
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Width: usize,
    #[cfg(feature = "deprecated")]
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Height: usize,
    #[cfg(feature = "deprecated")]
    pub FrameDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameDuration: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionColorFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Media", feature = "deprecated"))]
    pub VideoFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media", feature = "deprecated")))]
    VideoFrame: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionColorFrameArrivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RelativeTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RelativeTime: usize,
    #[cfg(feature = "deprecated")]
    pub TryOpenFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryOpenFrame: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionColorFrameReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FrameArrived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FrameArrived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveFrameArrived: usize,
    #[cfg(feature = "deprecated")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Source: usize,
    #[cfg(feature = "deprecated")]
    pub IsPaused: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsPaused: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsPaused: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsPaused: usize,
    #[cfg(feature = "deprecated")]
    pub TryReadLatestFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryReadLatestFrame: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionColorFrameSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AvailableChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AvailableChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAvailableChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAvailableChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ActiveChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ActiveChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveActiveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveActiveChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PropertiesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PropertiesChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePropertiesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePropertiesChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub VideoProfileChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    VideoProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveVideoProfileChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveVideoProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CameraIntrinsicsChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCameraIntrinsicsChanged: usize,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub Available: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Available: usize,
    #[cfg(feature = "deprecated")]
    pub Active: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Active: usize,
    #[cfg(feature = "deprecated")]
    pub IsControlled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsControlled: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Properties: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SupportedVideoProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SupportedVideoProfiles: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AvailableVideoProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AvailableVideoProfiles: usize,
    #[cfg(feature = "deprecated")]
    pub VideoProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    VideoProfile: usize,
    #[cfg(all(feature = "Media_Devices_Core", feature = "deprecated"))]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices_Core", feature = "deprecated")))]
    CameraIntrinsics: usize,
    #[cfg(feature = "deprecated")]
    pub AcquireControlSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AcquireControlSession: usize,
    #[cfg(feature = "deprecated")]
    pub CanControlIndependentlyFrom: unsafe extern "system" fn(this: *mut *mut Self, targetid: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanControlIndependentlyFrom: usize,
    #[cfg(feature = "deprecated")]
    pub IsCorrelatedWith: unsafe extern "system" fn(this: *mut *mut Self, targetid: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsCorrelatedWith: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub TryGetTransformTo: unsafe extern "system" fn(this: *mut *mut Self, targetid: ::windows_sys::core::HSTRING, result: *mut super::super::Foundation::Numerics::Matrix4x4, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    TryGetTransformTo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TryGetDepthCorrelatedCameraIntrinsicsAsync: unsafe extern "system" fn(this: *mut *mut Self, correlateddepthframesource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TryGetDepthCorrelatedCameraIntrinsicsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TryGetDepthCorrelatedCoordinateMapperAsync: unsafe extern "system" fn(this: *mut *mut Self, targetsourceid: ::windows_sys::core::HSTRING, correlateddepthframesource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TryGetDepthCorrelatedCoordinateMapperAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TrySetVideoProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, controlsession: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TrySetVideoProfileAsync: usize,
    #[cfg(feature = "deprecated")]
    pub OpenReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OpenReader: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionColorFrameSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionColorFrameSourceAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub FrameSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameSource: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionColorFrameSourceRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub FrameSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameSource: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionColorFrameSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CreateWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateWatcher: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestAccessAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionColorFrameSourceWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceAdded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Stopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    EnumerationCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveEnumerationCompleted: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "deprecated"))]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Enumeration::DeviceWatcherStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "deprecated")))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Start: usize,
    #[cfg(feature = "deprecated")]
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Stop: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionControlSession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ControlLost: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ControlLost: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveControlLost: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveControlLost: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TrySetPropertyAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TrySetPropertyAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionDepthCorrelatedCameraIntrinsics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub UnprojectPixelAtCorrelatedDepth: unsafe extern "system" fn(this: *mut *mut Self, pixelcoordinate: super::super::Foundation::Point, depthframe: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    UnprojectPixelAtCorrelatedDepth: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub UnprojectPixelsAtCorrelatedDepth: unsafe extern "system" fn(this: *mut *mut Self, sourceCoordinates_array_size: u32, sourcecoordinates: *const super::super::Foundation::Point, depthframe: *mut ::core::ffi::c_void, results_array_size: u32, results: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    UnprojectPixelsAtCorrelatedDepth: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub UnprojectRegionPixelsAtCorrelatedDepthAsync: unsafe extern "system" fn(this: *mut *mut Self, region: super::super::Foundation::Rect, depthframe: *mut ::core::ffi::c_void, results_array_size: u32, results: *mut super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    UnprojectRegionPixelsAtCorrelatedDepthAsync: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub UnprojectAllPixelsAtCorrelatedDepthAsync: unsafe extern "system" fn(this: *mut *mut Self, depthframe: *mut ::core::ffi::c_void, results_array_size: u32, results: *mut super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    UnprojectAllPixelsAtCorrelatedDepthAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionDepthCorrelatedCoordinateMapper {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MapPixelToTarget: unsafe extern "system" fn(this: *mut *mut Self, sourcepixelcoordinate: super::super::Foundation::Point, depthframe: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MapPixelToTarget: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MapPixelsToTarget: unsafe extern "system" fn(this: *mut *mut Self, sourceCoordinates_array_size: u32, sourcecoordinates: *const super::super::Foundation::Point, depthframe: *mut ::core::ffi::c_void, results_array_size: u32, results: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MapPixelsToTarget: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MapRegionOfPixelsToTargetAsync: unsafe extern "system" fn(this: *mut *mut Self, region: super::super::Foundation::Rect, depthframe: *mut ::core::ffi::c_void, targetCoordinates_array_size: u32, targetcoordinates: *mut super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MapRegionOfPixelsToTargetAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MapAllPixelsToTargetAsync: unsafe extern "system" fn(this: *mut *mut Self, depthframe: *mut ::core::ffi::c_void, targetCoordinates_array_size: u32, targetcoordinates: *mut super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MapAllPixelsToTargetAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionDepthFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Media", feature = "deprecated"))]
    pub VideoFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media", feature = "deprecated")))]
    VideoFrame: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionDepthFrameArrivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RelativeTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RelativeTime: usize,
    #[cfg(feature = "deprecated")]
    pub TryOpenFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryOpenFrame: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionDepthFrameReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FrameArrived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FrameArrived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveFrameArrived: usize,
    #[cfg(feature = "deprecated")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Source: usize,
    #[cfg(feature = "deprecated")]
    pub IsPaused: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsPaused: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsPaused: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsPaused: usize,
    #[cfg(feature = "deprecated")]
    pub TryReadLatestFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryReadLatestFrame: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionDepthFrameSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AvailableChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AvailableChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAvailableChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAvailableChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ActiveChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ActiveChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveActiveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveActiveChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PropertiesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PropertiesChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePropertiesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePropertiesChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub VideoProfileChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    VideoProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveVideoProfileChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveVideoProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CameraIntrinsicsChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCameraIntrinsicsChanged: usize,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub Available: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Available: usize,
    #[cfg(feature = "deprecated")]
    pub Active: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Active: usize,
    #[cfg(feature = "deprecated")]
    pub IsControlled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsControlled: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Properties: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SupportedVideoProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SupportedVideoProfiles: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AvailableVideoProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AvailableVideoProfiles: usize,
    #[cfg(feature = "deprecated")]
    pub VideoProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    VideoProfile: usize,
    #[cfg(all(feature = "Media_Devices_Core", feature = "deprecated"))]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices_Core", feature = "deprecated")))]
    CameraIntrinsics: usize,
    #[cfg(feature = "deprecated")]
    pub AcquireControlSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AcquireControlSession: usize,
    #[cfg(feature = "deprecated")]
    pub CanControlIndependentlyFrom: unsafe extern "system" fn(this: *mut *mut Self, targetid: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanControlIndependentlyFrom: usize,
    #[cfg(feature = "deprecated")]
    pub IsCorrelatedWith: unsafe extern "system" fn(this: *mut *mut Self, targetid: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsCorrelatedWith: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub TryGetTransformTo: unsafe extern "system" fn(this: *mut *mut Self, targetid: ::windows_sys::core::HSTRING, result: *mut super::super::Foundation::Numerics::Matrix4x4, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    TryGetTransformTo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TryGetDepthCorrelatedCameraIntrinsicsAsync: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TryGetDepthCorrelatedCameraIntrinsicsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TryGetDepthCorrelatedCoordinateMapperAsync: unsafe extern "system" fn(this: *mut *mut Self, targetid: ::windows_sys::core::HSTRING, depthframesourcetomapwith: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TryGetDepthCorrelatedCoordinateMapperAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TrySetVideoProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, controlsession: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TrySetVideoProfileAsync: usize,
    #[cfg(feature = "deprecated")]
    pub OpenReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OpenReader: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionDepthFrameSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionDepthFrameSourceAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub FrameSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameSource: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionDepthFrameSourceRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub FrameSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameSource: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionDepthFrameSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CreateWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateWatcher: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestAccessAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionDepthFrameSourceWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceAdded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Stopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    EnumerationCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveEnumerationCompleted: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "deprecated"))]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Enumeration::DeviceWatcherStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "deprecated")))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Start: usize,
    #[cfg(feature = "deprecated")]
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Stop: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionFrameSourcePropertiesChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub CollectionChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Collections::CollectionChange) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    CollectionChange: usize,
    #[cfg(feature = "deprecated")]
    pub Key: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Key: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionFrameSourcePropertyChangeResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PerceptionFrameSourcePropertyChangeStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub NewValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NewValue: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionInfraredFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Media", feature = "deprecated"))]
    pub VideoFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media", feature = "deprecated")))]
    VideoFrame: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionInfraredFrameArrivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RelativeTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RelativeTime: usize,
    #[cfg(feature = "deprecated")]
    pub TryOpenFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryOpenFrame: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionInfraredFrameReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FrameArrived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FrameArrived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveFrameArrived: usize,
    #[cfg(feature = "deprecated")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Source: usize,
    #[cfg(feature = "deprecated")]
    pub IsPaused: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsPaused: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsPaused: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsPaused: usize,
    #[cfg(feature = "deprecated")]
    pub TryReadLatestFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryReadLatestFrame: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionInfraredFrameSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AvailableChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AvailableChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAvailableChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAvailableChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ActiveChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ActiveChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveActiveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveActiveChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PropertiesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PropertiesChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePropertiesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePropertiesChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub VideoProfileChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    VideoProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveVideoProfileChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveVideoProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CameraIntrinsicsChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCameraIntrinsicsChanged: usize,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub Available: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Available: usize,
    #[cfg(feature = "deprecated")]
    pub Active: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Active: usize,
    #[cfg(feature = "deprecated")]
    pub IsControlled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsControlled: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Properties: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SupportedVideoProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SupportedVideoProfiles: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AvailableVideoProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AvailableVideoProfiles: usize,
    #[cfg(feature = "deprecated")]
    pub VideoProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    VideoProfile: usize,
    #[cfg(all(feature = "Media_Devices_Core", feature = "deprecated"))]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices_Core", feature = "deprecated")))]
    CameraIntrinsics: usize,
    #[cfg(feature = "deprecated")]
    pub AcquireControlSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AcquireControlSession: usize,
    #[cfg(feature = "deprecated")]
    pub CanControlIndependentlyFrom: unsafe extern "system" fn(this: *mut *mut Self, targetid: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanControlIndependentlyFrom: usize,
    #[cfg(feature = "deprecated")]
    pub IsCorrelatedWith: unsafe extern "system" fn(this: *mut *mut Self, targetid: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsCorrelatedWith: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub TryGetTransformTo: unsafe extern "system" fn(this: *mut *mut Self, targetid: ::windows_sys::core::HSTRING, result: *mut super::super::Foundation::Numerics::Matrix4x4, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    TryGetTransformTo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TryGetDepthCorrelatedCameraIntrinsicsAsync: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TryGetDepthCorrelatedCameraIntrinsicsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TryGetDepthCorrelatedCoordinateMapperAsync: unsafe extern "system" fn(this: *mut *mut Self, targetid: ::windows_sys::core::HSTRING, depthframesourcetomapwith: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TryGetDepthCorrelatedCoordinateMapperAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TrySetVideoProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, controlsession: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TrySetVideoProfileAsync: usize,
    #[cfg(feature = "deprecated")]
    pub OpenReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OpenReader: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionInfraredFrameSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionInfraredFrameSourceAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub FrameSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameSource: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionInfraredFrameSourceRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub FrameSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameSource: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionInfraredFrameSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CreateWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateWatcher: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestAccessAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionInfraredFrameSourceWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceAdded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Stopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    EnumerationCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveEnumerationCompleted: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "deprecated"))]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Enumeration::DeviceWatcherStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "deprecated")))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Start: usize,
    #[cfg(feature = "deprecated")]
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Stop: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionVideoProfile {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))]
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Imaging", feature = "deprecated")))]
    BitmapPixelFormat: usize,
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))]
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Imaging", feature = "deprecated")))]
    BitmapAlphaMode: usize,
    #[cfg(feature = "deprecated")]
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Width: usize,
    #[cfg(feature = "deprecated")]
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Height: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FrameDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FrameDuration: usize,
    #[cfg(feature = "deprecated")]
    pub IsEqual: unsafe extern "system" fn(this: *mut *mut Self, other: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsEqual: usize,
}
pub type PerceptionColorFrame = *mut ::core::ffi::c_void;
pub type PerceptionColorFrameArrivedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionColorFrameReader = *mut ::core::ffi::c_void;
pub type PerceptionColorFrameSource = *mut ::core::ffi::c_void;
pub type PerceptionColorFrameSourceAddedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionColorFrameSourceRemovedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionColorFrameSourceWatcher = *mut ::core::ffi::c_void;
pub type PerceptionControlSession = *mut ::core::ffi::c_void;
pub type PerceptionDepthCorrelatedCameraIntrinsics = *mut ::core::ffi::c_void;
pub type PerceptionDepthCorrelatedCoordinateMapper = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrame = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrameArrivedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrameReader = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrameSource = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrameSourceAddedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrameSourceRemovedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionDepthFrameSourceWatcher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionFrameSourceAccessStatus(pub i32);
#[cfg(feature = "deprecated")]
impl PerceptionFrameSourceAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for PerceptionFrameSourceAccessStatus {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionFrameSourceAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PerceptionFrameSourcePropertiesChangedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionFrameSourcePropertyChangeResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionFrameSourcePropertyChangeStatus(pub i32);
#[cfg(feature = "deprecated")]
impl PerceptionFrameSourcePropertyChangeStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Accepted: Self = Self(1i32);
    pub const LostControl: Self = Self(2i32);
    pub const PropertyNotSupported: Self = Self(3i32);
    pub const PropertyReadOnly: Self = Self(4i32);
    pub const ValueOutOfRange: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for PerceptionFrameSourcePropertyChangeStatus {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionFrameSourcePropertyChangeStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PerceptionInfraredFrame = *mut ::core::ffi::c_void;
pub type PerceptionInfraredFrameArrivedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionInfraredFrameReader = *mut ::core::ffi::c_void;
pub type PerceptionInfraredFrameSource = *mut ::core::ffi::c_void;
pub type PerceptionInfraredFrameSourceAddedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionInfraredFrameSourceRemovedEventArgs = *mut ::core::ffi::c_void;
pub type PerceptionInfraredFrameSourceWatcher = *mut ::core::ffi::c_void;
pub type PerceptionVideoProfile = *mut ::core::ffi::c_void;
