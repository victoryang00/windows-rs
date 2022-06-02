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
#[repr(C)]
pub struct ICameraIntrinsicsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, focallength: super::super::super::Foundation::Numerics::Vector2, principalpoint: super::super::super::Foundation::Numerics::Vector2, radialdistortion: super::super::super::Foundation::Numerics::Vector3, tangentialdistortion: super::super::super::Foundation::Numerics::Vector2, imagewidth: u32, imageheight: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Create: usize,
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
#[repr(C)]
pub struct IFrameControlCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Exposure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsoSpeed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Focus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PhotoConfirmationSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameControlCapabilities2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Flash: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IFrameController2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FlashControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IFrameExposureCompensationCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IFrameFlashCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RedEyeReductionSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IFrameFocusCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IFrameIsoSpeedCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
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
pub type VariablePhotoSequenceController = *mut ::core::ffi::c_void;
