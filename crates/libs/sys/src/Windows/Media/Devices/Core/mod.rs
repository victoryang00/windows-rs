pub type CameraIntrinsics = *mut ::core::ffi::c_void;
pub type DepthCorrelatedCoordinateMapper = *mut ::core::ffi::c_void;
pub type FrameControlCapabilities = *mut ::core::ffi::c_void;
pub type FrameController = *mut ::core::ffi::c_void;
pub type FrameExposureCapabilities = *mut ::core::ffi::c_void;
pub type FrameExposureCompensationCapabilities = *mut ::core::ffi::c_void;
pub type FrameExposureCompensationControl = *mut ::core::ffi::c_void;
pub type FrameExposureControl = *mut ::core::ffi::c_void;
pub type FrameFlashCapabilities = *mut ::core::ffi::c_void;
pub type FrameFlashControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct FrameFlashMode(pub i32);
impl FrameFlashMode {
    pub const Disable: Self = Self(0i32);
    pub const Enable: Self = Self(1i32);
    pub const Global: Self = Self(2i32);
}
impl ::core::marker::Copy for FrameFlashMode {}
impl ::core::clone::Clone for FrameFlashMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FrameFocusCapabilities = *mut ::core::ffi::c_void;
pub type FrameFocusControl = *mut ::core::ffi::c_void;
pub type FrameIsoSpeedCapabilities = *mut ::core::ffi::c_void;
pub type FrameIsoSpeedControl = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICameraIntrinsics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub FocalLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FocalLength: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PrincipalPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PrincipalPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub RadialDistortion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RadialDistortion: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TangentialDistortion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TangentialDistortion: usize,
    pub ImageWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ImageHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub ProjectOntoFrame: unsafe extern "system" fn(this: *mut *mut Self, coordinate: super::super::super::Foundation::Numerics::Vector3, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ProjectOntoFrame: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UnprojectAtUnitDepth: unsafe extern "system" fn(this: *mut *mut Self, pixelcoordinate: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UnprojectAtUnitDepth: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ProjectManyOntoFrame: unsafe extern "system" fn(this: *mut *mut Self, coordinates_array_size: u32, coordinates: *const super::super::super::Foundation::Numerics::Vector3, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ProjectManyOntoFrame: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UnprojectPixelsAtUnitDepth: unsafe extern "system" fn(this: *mut *mut Self, pixelCoordinates_array_size: u32, pixelcoordinates: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UnprojectPixelsAtUnitDepth: usize,
}
impl ::windows_sys::core::Interface for ICameraIntrinsics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 178711858, data2: 25993, data3: 18906, data4: [175, 222, 89, 66, 112, 202, 10, 172] };
}
#[repr(C)]
pub struct ICameraIntrinsics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub UndistortedProjectionTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UndistortedProjectionTransform: usize,
    #[cfg(feature = "Foundation")]
    pub DistortPoint: unsafe extern "system" fn(this: *mut *mut Self, input: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DistortPoint: usize,
    #[cfg(feature = "Foundation")]
    pub DistortPoints: unsafe extern "system" fn(this: *mut *mut Self, inputs_array_size: u32, inputs: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DistortPoints: usize,
    #[cfg(feature = "Foundation")]
    pub UndistortPoint: unsafe extern "system" fn(this: *mut *mut Self, input: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UndistortPoint: usize,
    #[cfg(feature = "Foundation")]
    pub UndistortPoints: unsafe extern "system" fn(this: *mut *mut Self, inputs_array_size: u32, inputs: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UndistortPoints: usize,
}
impl ::windows_sys::core::Interface for ICameraIntrinsics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 215655495, data2: 1944, data3: 19277, data4: [131, 159, 197, 236, 65, 77, 178, 122] };
}
#[repr(C)]
pub struct ICameraIntrinsicsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, focallength: super::super::super::Foundation::Numerics::Vector2, principalpoint: super::super::super::Foundation::Numerics::Vector2, radialdistortion: super::super::super::Foundation::Numerics::Vector3, tangentialdistortion: super::super::super::Foundation::Numerics::Vector2, imagewidth: u32, imageheight: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for ICameraIntrinsicsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3235759238, data2: 8498, data3: 18996, data4: [166, 89, 155, 254, 42, 5, 87, 18] };
}
#[repr(C)]
pub struct IDepthCorrelatedCoordinateMapper {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub UnprojectPoint: unsafe extern "system" fn(this: *mut *mut Self, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    UnprojectPoint: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub UnprojectPoints: unsafe extern "system" fn(this: *mut *mut Self, sourcePoints_array_size: u32, sourcepoints: *const super::super::super::Foundation::Point, targetcoordinatesystem: *mut ::core::ffi::c_void, results_array_size: u32, results: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    UnprojectPoints: usize,
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))]
    pub MapPoint: unsafe extern "system" fn(this: *mut *mut Self, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: *mut ::core::ffi::c_void, targetcameraintrinsics: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Perception_Spatial")))]
    MapPoint: usize,
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))]
    pub MapPoints: unsafe extern "system" fn(this: *mut *mut Self, sourcePoints_array_size: u32, sourcepoints: *const super::super::super::Foundation::Point, targetcoordinatesystem: *mut ::core::ffi::c_void, targetcameraintrinsics: *mut ::core::ffi::c_void, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Perception_Spatial")))]
    MapPoints: usize,
}
impl ::windows_sys::core::Interface for IDepthCorrelatedCoordinateMapper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4183656955, data2: 35568, data3: 19632, data4: [146, 109, 105, 104, 102, 229, 4, 106] };
}
#[repr(C)]
pub struct IFrameControlCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Exposure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsoSpeed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Focus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PhotoConfirmationSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFrameControlCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2835328608, data2: 20126, data3: 17271, data4: [167, 137, 226, 76, 74, 231, 229, 68] };
}
#[repr(C)]
pub struct IFrameControlCapabilities2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Flash: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFrameControlCapabilities2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3466265700, data2: 18224, data3: 17423, data4: [189, 62, 239, 232, 168, 242, 48, 168] };
}
#[repr(C)]
pub struct IFrameController {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExposureControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExposureCompensationControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsoSpeedControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PhotoConfirmationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhotoConfirmationEnabled: usize,
    #[cfg(feature = "Foundation")]
    pub SetPhotoConfirmationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPhotoConfirmationEnabled: usize,
}
impl ::windows_sys::core::Interface for IFrameController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3244579289, data2: 47855, data3: 16466, data4: [145, 119, 72, 175, 242, 175, 117, 34] };
}
#[repr(C)]
pub struct IFrameController2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FlashControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFrameController2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 13876341, data2: 55420, data3: 18523, data4: [138, 9, 92, 53, 133, 104, 180, 39] };
}
#[repr(C)]
pub struct IFrameExposureCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Min: usize,
    #[cfg(feature = "Foundation")]
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Max: usize,
    #[cfg(feature = "Foundation")]
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Step: usize,
}
impl ::windows_sys::core::Interface for IFrameExposureCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3183385827, data2: 14725, data3: 20082, data4: [151, 194, 5, 144, 214, 19, 7, 161] };
}
#[repr(C)]
pub struct IFrameExposureCompensationCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFrameExposureCompensationCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3112740899, data2: 32869, data3: 16878, data4: [176, 79, 114, 34, 101, 149, 69, 0] };
}
#[repr(C)]
pub struct IFrameExposureCompensationControl {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
}
impl ::windows_sys::core::Interface for IFrameExposureCompensationControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3914897097, data2: 63481, data3: 18634, data4: [133, 145, 162, 101, 49, 203, 21, 120] };
}
#[repr(C)]
pub struct IFrameExposureControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Auto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
}
impl ::windows_sys::core::Interface for IFrameExposureControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2975881825, data2: 65455, data3: 18258, data4: [182, 33, 245, 182, 241, 23, 244, 50] };
}
#[repr(C)]
pub struct IFrameFlashCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RedEyeReductionSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFrameFlashCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3146989986, data2: 24254, data3: 20322, data4: [130, 35, 14, 43, 5, 191, 187, 208] };
}
#[repr(C)]
pub struct IFrameFlashControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FrameFlashMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: FrameFlashMode) -> ::windows_sys::core::HRESULT,
    pub Auto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RedEyeReduction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRedEyeReduction: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFrameFlashControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1976956615, data2: 48453, data3: 20395, data4: [147, 117, 69, 172, 4, 179, 50, 194] };
}
#[repr(C)]
pub struct IFrameFocusCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFrameFocusCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2066074968, data2: 448, data3: 16485, data4: [156, 64, 193, 167, 33, 66, 92, 26] };
}
#[repr(C)]
pub struct IFrameFocusControl {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
}
impl ::windows_sys::core::Interface for IFrameFocusControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657322448, data2: 55570, data3: 16916, data4: [166, 123, 227, 138, 141, 72, 216, 198] };
}
#[repr(C)]
pub struct IFrameIsoSpeedCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFrameIsoSpeedCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 381550433, data2: 28150, data3: 19145, data4: [185, 42, 159, 110, 205, 26, 210, 250] };
}
#[repr(C)]
pub struct IFrameIsoSpeedControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Auto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
}
impl ::windows_sys::core::Interface for IFrameIsoSpeedControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 436465645, data2: 30826, data3: 19573, data4: [165, 87, 122, 185, 168, 95, 88, 140] };
}
#[repr(C)]
pub struct IVariablePhotoSequenceController {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MaxPhotosPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub PhotosPerSecondLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPhotosPerSecondLimit: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(this: *mut *mut Self, captureproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub FrameCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFrameControllers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFrameControllers: usize,
}
impl ::windows_sys::core::Interface for IVariablePhotoSequenceController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2143287424, data2: 60812, data3: 17405, data4: [167, 195, 179, 88, 9, 228, 34, 154] };
}
pub type VariablePhotoSequenceController = *mut ::core::ffi::c_void;
