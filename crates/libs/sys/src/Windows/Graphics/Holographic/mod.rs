#[repr(C)]
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
pub struct HolographicAdapterId {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl ::core::marker::Copy for HolographicAdapterId {}
impl ::core::clone::Clone for HolographicAdapterId {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HolographicCamera = *mut ::core::ffi::c_void;
pub type HolographicCameraPose = *mut ::core::ffi::c_void;
pub type HolographicCameraRenderingParameters = *mut ::core::ffi::c_void;
pub type HolographicCameraViewportParameters = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicDepthReprojectionMethod(pub i32);
impl HolographicDepthReprojectionMethod {
    pub const DepthReprojection: Self = Self(0i32);
    pub const AutoPlanar: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicDepthReprojectionMethod {}
impl ::core::clone::Clone for HolographicDepthReprojectionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HolographicDisplay = *mut ::core::ffi::c_void;
pub type HolographicFrame = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
pub struct HolographicFrameId {
    pub Value: u64,
}
impl ::core::marker::Copy for HolographicFrameId {}
impl ::core::clone::Clone for HolographicFrameId {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HolographicFramePrediction = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicFramePresentResult(pub i32);
impl HolographicFramePresentResult {
    pub const Success: Self = Self(0i32);
    pub const DeviceRemoved: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicFramePresentResult {}
impl ::core::clone::Clone for HolographicFramePresentResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicFramePresentWaitBehavior(pub i32);
impl HolographicFramePresentWaitBehavior {
    pub const WaitForFrameToFinish: Self = Self(0i32);
    pub const DoNotWaitForFrameToFinish: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicFramePresentWaitBehavior {}
impl ::core::clone::Clone for HolographicFramePresentWaitBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HolographicFramePresentationMonitor = *mut ::core::ffi::c_void;
pub type HolographicFramePresentationReport = *mut ::core::ffi::c_void;
pub type HolographicFrameRenderingReport = *mut ::core::ffi::c_void;
pub type HolographicFrameScanoutMonitor = *mut ::core::ffi::c_void;
pub type HolographicFrameScanoutReport = *mut ::core::ffi::c_void;
pub type HolographicQuadLayer = *mut ::core::ffi::c_void;
pub type HolographicQuadLayerUpdateParameters = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicReprojectionMode(pub i32);
impl HolographicReprojectionMode {
    pub const PositionAndOrientation: Self = Self(0i32);
    pub const OrientationOnly: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
impl ::core::marker::Copy for HolographicReprojectionMode {}
impl ::core::clone::Clone for HolographicReprojectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HolographicSpace = *mut ::core::ffi::c_void;
pub type HolographicSpaceCameraAddedEventArgs = *mut ::core::ffi::c_void;
pub type HolographicSpaceCameraRemovedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicSpaceUserPresence(pub i32);
impl HolographicSpaceUserPresence {
    pub const Absent: Self = Self(0i32);
    pub const PresentPassive: Self = Self(1i32);
    pub const PresentActive: Self = Self(2i32);
}
impl ::core::marker::Copy for HolographicSpaceUserPresence {}
impl ::core::clone::Clone for HolographicSpaceUserPresence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Holographic\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct HolographicStereoTransform {
    pub Left: super::super::Foundation::Numerics::Matrix4x4,
    pub Right: super::super::Foundation::Numerics::Matrix4x4,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for HolographicStereoTransform {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for HolographicStereoTransform {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HolographicViewConfiguration = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Holographic\"`*"]
#[repr(transparent)]
pub struct HolographicViewConfigurationKind(pub i32);
impl HolographicViewConfigurationKind {
    pub const Display: Self = Self(0i32);
    pub const PhotoVideoCamera: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicViewConfigurationKind {}
impl ::core::clone::Clone for HolographicViewConfigurationKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IHolographicCamera {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RenderTargetSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenderTargetSize: usize,
    pub ViewportScaleFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetViewportScaleFactor: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub IsStereo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetNearPlaneDistance: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub SetFarPlaneDistance: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicCamera2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LeftViewportParameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RightViewportParameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Display: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicCamera3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPrimaryLayerEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPrimaryLayerEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MaxQuadLayerCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub QuadLayers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QuadLayers: usize,
}
#[repr(C)]
pub struct IHolographicCamera4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanOverrideViewport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicCamera5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsHardwareContentProtectionSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsHardwareContentProtectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHardwareContentProtectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicCamera6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViewConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicCameraPose {
    pub base__: ::windows_sys::core::IInspectable,
    pub HolographicCamera: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Viewport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Viewport: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetViewTransform: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetViewTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ProjectionTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HolographicStereoTransform) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ProjectionTransform: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetCullingFrustum: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetCullingFrustum: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetVisibleFrustum: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetVisibleFrustum: usize,
    pub NearPlaneDistance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub FarPlaneDistance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicCameraPose2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub OverrideViewTransform: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, coordinatesystemtoviewtransform: HolographicStereoTransform) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    OverrideViewTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub OverrideProjectionTransform: unsafe extern "system" fn(this: *mut *mut Self, projectiontransform: HolographicStereoTransform) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    OverrideProjectionTransform: usize,
    #[cfg(feature = "Foundation")]
    pub OverrideViewport: unsafe extern "system" fn(this: *mut *mut Self, leftviewport: super::super::Foundation::Rect, rightviewport: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OverrideViewport: usize,
}
#[repr(C)]
pub struct IHolographicCameraRenderingParameters {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetFocusPoint: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetFocusPoint: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetFocusPointWithNormal: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetFocusPointWithNormal: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetFocusPointWithNormalLinearVelocity: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3, linearvelocity: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetFocusPointWithNormalLinearVelocity: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11BackBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11BackBuffer: usize,
}
#[repr(C)]
pub struct IHolographicCameraRenderingParameters2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReprojectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HolographicReprojectionMode) -> ::windows_sys::core::HRESULT,
    pub SetReprojectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: HolographicReprojectionMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CommitDirect3D11DepthBuffer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CommitDirect3D11DepthBuffer: usize,
}
#[repr(C)]
pub struct IHolographicCameraRenderingParameters3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsContentProtectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsContentProtectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicCameraRenderingParameters4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DepthReprojectionMethod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HolographicDepthReprojectionMethod) -> ::windows_sys::core::HRESULT,
    pub SetDepthReprojectionMethod: unsafe extern "system" fn(this: *mut *mut Self, value: HolographicDepthReprojectionMethod) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicCameraViewportParameters {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub HiddenAreaMesh: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    HiddenAreaMesh: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub VisibleAreaMesh: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    VisibleAreaMesh: usize,
}
#[repr(C)]
pub struct IHolographicDisplay {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxViewportSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxViewportSize: usize,
    pub IsStereo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsOpaque: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AdapterId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HolographicAdapterId) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub SpatialLocator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    SpatialLocator: usize,
}
#[repr(C)]
pub struct IHolographicDisplay2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RefreshRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicDisplay3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetViewConfiguration: unsafe extern "system" fn(this: *mut *mut Self, kind: HolographicViewConfigurationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicDisplayStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AddedCameras: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddedCameras: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemovedCameras: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemovedCameras: usize,
    pub GetRenderingParameters: unsafe extern "system" fn(this: *mut *mut Self, camerapose: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub CurrentPrediction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UpdateCurrentPrediction: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PresentUsingCurrentPrediction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HolographicFramePresentResult) -> ::windows_sys::core::HRESULT,
    pub PresentUsingCurrentPredictionWithBehavior: unsafe extern "system" fn(this: *mut *mut Self, waitbehavior: HolographicFramePresentWaitBehavior, result__: *mut HolographicFramePresentResult) -> ::windows_sys::core::HRESULT,
    pub WaitForFrameToFinish: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicFrame2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetQuadLayerUpdateParameters: unsafe extern "system" fn(this: *mut *mut Self, layer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicFrame3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HolographicFrameId) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicFramePrediction {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CameraPoses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CameraPoses: usize,
    #[cfg(feature = "Perception")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception"))]
    Timestamp: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IHolographicFramePresentationMonitor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ReadReports: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ReadReports: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IHolographicFramePresentationReport {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CompositorGpuDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CompositorGpuDuration: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AppGpuDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AppGpuDuration: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AppGpuOverrun: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AppGpuOverrun: usize,
    #[cfg(feature = "deprecated")]
    pub MissedPresentationOpportunityCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MissedPresentationOpportunityCount: usize,
    #[cfg(feature = "deprecated")]
    pub PresentationCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PresentationCount: usize,
}
#[repr(C)]
pub struct IHolographicFrameRenderingReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub FrameId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HolographicFrameId) -> ::windows_sys::core::HRESULT,
    pub MissedLatchCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeFrameReadyTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeFrameReadyTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeActualGpuFinishTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeActualGpuFinishTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeTargetLatchTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeTargetLatchTime: usize,
}
#[repr(C)]
pub struct IHolographicFrameScanoutMonitor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
#[repr(C)]
pub struct IHolographicFrameScanoutReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub RenderingReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MissedScanoutCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeLatchTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeLatchTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeScanoutStartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeScanoutStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativePhotonTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativePhotonTime: usize,
}
#[repr(C)]
pub struct IHolographicQuadLayer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX")]
    pub PixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PixelFormat: usize,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
}
#[repr(C)]
pub struct IHolographicQuadLayerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, size: super::super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))]
    pub CreateWithPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, size: super::super::Foundation::Size, pixelformat: super::DirectX::DirectXPixelFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX")))]
    CreateWithPixelFormat: usize,
}
#[repr(C)]
pub struct IHolographicQuadLayerUpdateParameters {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub AcquireBufferToUpdateContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    AcquireBufferToUpdateContent: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateViewport: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateViewport: usize,
    pub UpdateContentProtectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub UpdateExtents: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UpdateExtents: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub UpdateLocationWithStationaryMode: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    UpdateLocationWithStationaryMode: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UpdateLocationWithDisplayRelativeMode: unsafe extern "system" fn(this: *mut *mut Self, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UpdateLocationWithDisplayRelativeMode: usize,
}
#[repr(C)]
pub struct IHolographicQuadLayerUpdateParameters2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanAcquireWithHardwareProtection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub AcquireBufferToUpdateContentWithHardwareProtection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    AcquireBufferToUpdateContentWithHardwareProtection: usize,
}
#[repr(C)]
pub struct IHolographicSpace {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrimaryAdapterId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HolographicAdapterId) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub SetDirect3D11Device: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    SetDirect3D11Device: usize,
    #[cfg(feature = "Foundation")]
    pub CameraAdded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraAdded: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraAdded: usize,
    #[cfg(feature = "Foundation")]
    pub CameraRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraRemoved: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraRemoved: usize,
    pub CreateNextFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicSpace2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UserPresence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HolographicSpaceUserPresence) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UserPresenceChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserPresenceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserPresenceChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserPresenceChanged: usize,
    pub WaitForNextFrameReady: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WaitForNextFrameReadyWithHeadStart: unsafe extern "system" fn(this: *mut *mut Self, requestedheadstartduration: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WaitForNextFrameReadyWithHeadStart: usize,
    #[cfg(feature = "deprecated")]
    pub CreateFramePresentationMonitor: unsafe extern "system" fn(this: *mut *mut Self, maxqueuedreports: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateFramePresentationMonitor: usize,
}
#[repr(C)]
pub struct IHolographicSpace3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFrameScanoutMonitor: unsafe extern "system" fn(this: *mut *mut Self, maxqueuedreports: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicSpaceCameraAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Camera: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IHolographicSpaceCameraRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Camera: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicSpaceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Core")]
    pub CreateForCoreWindow: unsafe extern "system" fn(this: *mut *mut Self, window: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    CreateForCoreWindow: usize,
}
#[repr(C)]
pub struct IHolographicSpaceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsAvailableChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsAvailableChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsAvailableChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsAvailableChanged: usize,
}
#[repr(C)]
pub struct IHolographicSpaceStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsConfigured: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicViewConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub NativeRenderTargetSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NativeRenderTargetSize: usize,
    #[cfg(feature = "Foundation")]
    pub RenderTargetSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenderTargetSize: usize,
    #[cfg(feature = "Foundation")]
    pub RequestRenderTargetSize: unsafe extern "system" fn(this: *mut *mut Self, size: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRenderTargetSize: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedPixelFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedPixelFormats: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub PixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PixelFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, value: super::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetPixelFormat: usize,
    pub IsStereo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RefreshRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HolographicViewConfigurationKind) -> ::windows_sys::core::HRESULT,
    pub Display: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicViewConfiguration2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDepthReprojectionMethods: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDepthReprojectionMethods: usize,
}
