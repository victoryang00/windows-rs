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
impl ::windows_sys::core::Interface for IKnownCameraIntrinsicsPropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 146815352, data2: 17274, data3: 19863, data4: [166, 99, 253, 49, 149, 96, 2, 73] };
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
impl ::windows_sys::core::Interface for IKnownPerceptionColorFrameSourcePropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1576127650, data2: 504, data3: 19079, data4: [184, 89, 213, 229, 183, 225, 222, 75] };
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
impl ::windows_sys::core::Interface for IKnownPerceptionDepthFrameSourcePropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1576127650, data2: 504, data3: 19079, data4: [184, 89, 213, 229, 183, 225, 222, 74] };
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
impl ::windows_sys::core::Interface for IKnownPerceptionFrameSourcePropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1576127650, data2: 504, data3: 19079, data4: [184, 89, 213, 229, 183, 225, 222, 71] };
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
impl ::windows_sys::core::Interface for IKnownPerceptionFrameSourcePropertiesStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2848483441, data2: 1500, data3: 19021, data4: [138, 92, 164, 236, 242, 107, 188, 70] };
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
impl ::windows_sys::core::Interface for IKnownPerceptionInfraredFrameSourcePropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1576127650, data2: 504, data3: 19079, data4: [184, 89, 213, 229, 183, 225, 222, 73] };
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
impl ::windows_sys::core::Interface for IKnownPerceptionVideoFrameSourcePropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1576127650, data2: 504, data3: 19079, data4: [184, 89, 213, 229, 183, 225, 222, 72] };
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
impl ::windows_sys::core::Interface for IKnownPerceptionVideoProfilePropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2399724263, data2: 23158, data3: 17379, data4: [161, 58, 218, 61, 145, 169, 239, 152] };
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
impl ::windows_sys::core::Interface for IPerceptionColorFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4267840841, data2: 11455, data3: 20372, data4: [152, 97, 248, 23, 234, 49, 119, 71] };
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
impl ::windows_sys::core::Interface for IPerceptionColorFrameArrivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2410480341, data2: 34551, data3: 19853, data4: [185, 102, 90, 55, 97, 186, 159, 89] };
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
impl ::windows_sys::core::Interface for IPerceptionColorFrameReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1985017198, data2: 47605, data3: 17947, data4: [131, 173, 242, 34, 175, 42, 170, 220] };
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
impl ::windows_sys::core::Interface for IPerceptionColorFrameSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3698178684, data2: 2904, data3: 18061, data4: [156, 161, 109, 176, 76, 192, 71, 124] };
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
impl ::windows_sys::core::Interface for IPerceptionColorFrameSource2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4169140453, data2: 22065, data3: 17901, data4: [173, 152, 140, 106, 160, 76, 251, 145] };
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
impl ::windows_sys::core::Interface for IPerceptionColorFrameSourceAddedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3513513190, data2: 55844, data3: 17452, data4: [187, 213, 85, 84, 155, 91, 148, 243] };
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
impl ::windows_sys::core::Interface for IPerceptionColorFrameSourceRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3531078249, data2: 60236, data3: 17135, data4: [186, 79, 40, 143, 97, 92, 147, 193] };
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
impl ::windows_sys::core::Interface for IPerceptionColorFrameSourceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1576258722, data2: 504, data3: 19079, data4: [184, 89, 213, 229, 183, 225, 222, 73] };
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
impl ::windows_sys::core::Interface for IPerceptionColorFrameSourceWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2528973714, data2: 58983, data3: 16580, data4: [137, 249, 20, 98, 222, 166, 169, 204] };
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
impl ::windows_sys::core::Interface for IPerceptionControlSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2576975443, data2: 23101, data3: 16767, data4: [146, 57, 241, 136, 158, 84, 139, 72] };
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
impl ::windows_sys::core::Interface for IPerceptionDepthCorrelatedCameraIntrinsics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1699269121, data2: 34526, data3: 23521, data4: [101, 130, 128, 127, 207, 76, 149, 207] };
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
impl ::windows_sys::core::Interface for IPerceptionDepthCorrelatedCoordinateMapper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1531813149, data2: 46582, data3: 18076, data4: [184, 194, 185, 122, 69, 230, 134, 59] };
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
impl ::windows_sys::core::Interface for IPerceptionDepthFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2742780412, data2: 39174, data3: 20477, data4: [145, 97, 0, 36, 179, 96, 182, 87] };
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
impl ::windows_sys::core::Interface for IPerceptionDepthFrameArrivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1144858034, data2: 45698, data3: 17975, data4: [145, 115, 172, 151, 132, 53, 201, 133] };
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
impl ::windows_sys::core::Interface for IPerceptionDepthFrameReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2980298911, data2: 10651, data3: 17938, data4: [164, 247, 39, 15, 37, 160, 150, 236] };
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
impl ::windows_sys::core::Interface for IPerceptionDepthFrameSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2043950038, data2: 18427, data3: 19953, data4: [191, 201, 240, 29, 64, 189, 153, 66] };
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
impl ::windows_sys::core::Interface for IPerceptionDepthFrameSource2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3822206254, data2: 28204, data3: 20077, data4: [145, 217, 112, 76, 216, 223, 247, 157] };
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
impl ::windows_sys::core::Interface for IPerceptionDepthFrameSourceAddedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2477031784, data2: 35832, data3: 17874, data4: [162, 248, 74, 192, 147, 28, 199, 166] };
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
impl ::windows_sys::core::Interface for IPerceptionDepthFrameSourceRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2696989773, data2: 59756, data3: 19841, data4: [134, 221, 56, 185, 94, 73, 198, 223] };
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
impl ::windows_sys::core::Interface for IPerceptionDepthFrameSourceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1576258722, data2: 504, data3: 19079, data4: [184, 89, 213, 229, 183, 225, 222, 72] };
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
impl ::windows_sys::core::Interface for IPerceptionDepthFrameSourceWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2014222033, data2: 36098, data3: 19755, data4: [173, 164, 91, 166, 36, 160, 235, 16] };
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
impl ::windows_sys::core::Interface for IPerceptionFrameSourcePropertiesChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1818812520, data2: 48369, data3: 20172, data4: [184, 145, 118, 37, 209, 36, 75, 107] };
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
impl ::windows_sys::core::Interface for IPerceptionFrameSourcePropertyChangeResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 506673418, data2: 15504, data3: 19746, data4: [184, 152, 244, 43, 186, 100, 24, 255] };
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
impl ::windows_sys::core::Interface for IPerceptionInfraredFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2961728118, data2: 33950, data3: 19578, data4: [138, 230, 181, 96, 100, 83, 33, 83] };
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
impl ::windows_sys::core::Interface for IPerceptionInfraredFrameArrivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2675440327, data2: 46269, data3: 18519, data4: [157, 80, 190, 142, 240, 117, 218, 239] };
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
impl ::windows_sys::core::Interface for IPerceptionInfraredFrameReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2036387352, data2: 54171, data3: 20424, data4: [160, 74, 146, 151, 52, 198, 117, 108] };
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
impl ::windows_sys::core::Interface for IPerceptionInfraredFrameSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1437632322, data2: 6152, data3: 18766, data4: [158, 48, 157, 42, 123, 232, 247, 0] };
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
impl ::windows_sys::core::Interface for IPerceptionInfraredFrameSource2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3704936344, data2: 19211, data3: 17152, data4: [141, 133, 65, 8, 23, 250, 160, 50] };
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
impl ::windows_sys::core::Interface for IPerceptionInfraredFrameSourceAddedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1832075552, data2: 38350, data3: 18016, data4: [144, 122, 217, 128, 53, 170, 43, 124] };
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
impl ::windows_sys::core::Interface for IPerceptionInfraredFrameSourceRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3927605361, data2: 31344, data3: 19041, data4: [175, 148, 7, 48, 56, 83, 246, 149] };
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
impl ::windows_sys::core::Interface for IPerceptionInfraredFrameSourceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1576258722, data2: 504, data3: 19079, data4: [184, 89, 213, 229, 183, 225, 222, 71] };
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
impl ::windows_sys::core::Interface for IPerceptionInfraredFrameSourceWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 943521689, data2: 55052, data3: 17485, data4: [168, 176, 114, 12, 46, 102, 254, 59] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPerceptionVideoProfile {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1970683555, data2: 282, data3: 18190, data4: [130, 37, 111, 5, 173, 226, 86, 72] };
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
