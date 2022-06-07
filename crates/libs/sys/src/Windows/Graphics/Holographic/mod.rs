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
impl ::windows_sys::core::Interface for IHolographicCamera {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3840508997, data2: 39917, data3: 18816, data4: [155, 160, 232, 118, 128, 209, 203, 116] };
}
#[repr(C)]
pub struct IHolographicCamera2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LeftViewportParameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RightViewportParameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Display: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicCamera2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3042680602, data2: 47756, data3: 20356, data4: [173, 121, 46, 126, 30, 36, 80, 243] };
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
impl ::windows_sys::core::Interface for IHolographicCamera3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1168789427, data2: 31577, data3: 21070, data4: [74, 63, 74, 106, 214, 101, 4, 119] };
}
#[repr(C)]
pub struct IHolographicCamera4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanOverrideViewport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicCamera4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2586128854, data2: 18211, data3: 20281, data4: [169, 165, 157, 5, 24, 29, 155, 68] };
}
#[repr(C)]
pub struct IHolographicCamera5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsHardwareContentProtectionSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsHardwareContentProtectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHardwareContentProtectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicCamera5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 580323058, data2: 25229, data3: 20213, data4: [156, 8, 166, 63, 221, 119, 135, 198] };
}
#[repr(C)]
pub struct IHolographicCamera6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViewConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicCamera6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 34150735, data2: 25389, data3: 20820, data4: [171, 82, 11, 93, 21, 177, 37, 5] };
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
impl ::windows_sys::core::Interface for IHolographicCameraPose {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 226328112, data2: 4830, data3: 17853, data4: [145, 43, 199, 246, 86, 21, 153, 209] };
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
impl ::windows_sys::core::Interface for IHolographicCameraPose2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 590078067, data2: 23853, data3: 17760, data4: [129, 78, 38, 151, 196, 252, 225, 107] };
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
impl ::windows_sys::core::Interface for IHolographicCameraRenderingParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2393648849, data2: 23540, data3: 19990, data4: [130, 54, 174, 8, 0, 193, 29, 13] };
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
impl ::windows_sys::core::Interface for IHolographicCameraRenderingParameters2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 638742755, data2: 46742, data3: 17972, data4: [148, 214, 190, 6, 129, 100, 53, 153] };
}
#[repr(C)]
pub struct IHolographicCameraRenderingParameters3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsContentProtectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsContentProtectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicCameraRenderingParameters3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2980729151, data2: 4973, data3: 19206, data4: [185, 212, 228, 185, 20, 205, 6, 131] };
}
#[repr(C)]
pub struct IHolographicCameraRenderingParameters4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DepthReprojectionMethod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HolographicDepthReprojectionMethod) -> ::windows_sys::core::HRESULT,
    pub SetDepthReprojectionMethod: unsafe extern "system" fn(this: *mut *mut Self, value: HolographicDepthReprojectionMethod) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicCameraRenderingParameters4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 142146124, data2: 57699, data3: 22492, data4: [130, 183, 196, 6, 171, 62, 5, 55] };
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
impl ::windows_sys::core::Interface for IHolographicCameraViewportParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2160980983, data2: 33834, data3: 16865, data4: [147, 237, 86, 146, 171, 31, 187, 16] };
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
impl ::windows_sys::core::Interface for IHolographicDisplay {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2597233684, data2: 7583, data3: 16528, data4: [163, 136, 144, 192, 111, 110, 174, 156] };
}
#[repr(C)]
pub struct IHolographicDisplay2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RefreshRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicDisplay2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1974222722, data2: 59221, data3: 17260, data4: [141, 150, 77, 50, 209, 49, 71, 62] };
}
#[repr(C)]
pub struct IHolographicDisplay3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetViewConfiguration: unsafe extern "system" fn(this: *mut *mut Self, kind: HolographicViewConfigurationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicDisplay3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4232866502, data2: 25728, data3: 20488, data4: [178, 158, 21, 125, 119, 200, 67, 247] };
}
#[repr(C)]
pub struct IHolographicDisplayStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicDisplayStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409398147, data2: 59312, data3: 18497, data4: [131, 85, 58, 229, 181, 54, 233, 164] };
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
impl ::windows_sys::core::Interface for IHolographicFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3331886774, data2: 43193, data3: 12372, data4: [166, 235, 214, 36, 182, 83, 99, 117] };
}
#[repr(C)]
pub struct IHolographicFrame2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetQuadLayerUpdateParameters: unsafe extern "system" fn(this: *mut *mut Self, layer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicFrame2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 675231679, data2: 15346, data3: 24209, data4: [102, 51, 135, 5, 116, 230, 242, 23] };
}
#[repr(C)]
pub struct IHolographicFrame3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HolographicFrameId) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicFrame3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3857278153, data2: 35367, data3: 21971, data4: [159, 152, 148, 83, 13, 54, 144, 82] };
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
impl ::windows_sys::core::Interface for IHolographicFramePrediction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1376734689, data2: 23562, data3: 20089, data4: [168, 30, 106, 190, 2, 187, 39, 57] };
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
impl ::windows_sys::core::Interface for IHolographicFramePresentationMonitor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3397854572, data2: 28590, data3: 17038, data4: [187, 131, 37, 223, 238, 81, 19, 107] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IHolographicFramePresentationReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2159736340, data2: 62196, data3: 19594, data4: [141, 227, 6, 92, 120, 246, 213, 222] };
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
impl ::windows_sys::core::Interface for IHolographicFrameRenderingReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 99823076, data2: 58244, data3: 20915, data4: [185, 52, 240, 211, 160, 247, 134, 6] };
}
#[repr(C)]
pub struct IHolographicFrameScanoutMonitor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
impl ::windows_sys::core::Interface for IHolographicFrameScanoutMonitor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2122575785, data2: 33852, data3: 21505, data4: [128, 149, 155, 193, 184, 176, 134, 56] };
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
impl ::windows_sys::core::Interface for IHolographicFrameScanoutReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 247195142, data2: 928, data3: 23712, data4: [180, 110, 187, 160, 104, 215, 35, 63] };
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
impl ::windows_sys::core::Interface for IHolographicQuadLayer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2419351753, data2: 51673, data3: 23900, data4: [65, 172, 162, 213, 171, 15, 211, 49] };
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
impl ::windows_sys::core::Interface for IHolographicQuadLayerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2792700147, data2: 23060, data3: 23056, data4: [72, 154, 69, 80, 101, 179, 123, 118] };
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
impl ::windows_sys::core::Interface for IHolographicQuadLayerUpdateParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 722379696, data2: 31117, data3: 23498, data4: [85, 194, 44, 12, 118, 46, 187, 8] };
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
impl ::windows_sys::core::Interface for IHolographicQuadLayerUpdateParameters2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1328796461, data2: 33473, data3: 18113, data4: [137, 128, 60, 183, 13, 152, 24, 43] };
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
impl ::windows_sys::core::Interface for IHolographicSpace {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1132518310, data2: 24184, data3: 17231, data4: [128, 124, 52, 51, 209, 239, 232, 183] };
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
impl ::windows_sys::core::Interface for IHolographicSpace2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1333897640, data2: 47103, data3: 18563, data4: [152, 39, 125, 103, 114, 135, 234, 112] };
}
#[repr(C)]
pub struct IHolographicSpace3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFrameScanoutMonitor: unsafe extern "system" fn(this: *mut *mut Self, maxqueuedreports: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicSpace3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3742839761, data2: 61988, data3: 22654, data4: [141, 113, 30, 143, 200, 240, 123, 31] };
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
impl ::windows_sys::core::Interface for IHolographicSpaceCameraAddedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1492245045, data2: 48051, data3: 15503, data4: [153, 61, 108, 128, 231, 254, 185, 159] };
}
#[repr(C)]
pub struct IHolographicSpaceCameraRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Camera: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicSpaceCameraRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2153006248, data2: 62126, data3: 12846, data4: [141, 169, 131, 106, 10, 149, 164, 193] };
}
#[repr(C)]
pub struct IHolographicSpaceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Core")]
    pub CreateForCoreWindow: unsafe extern "system" fn(this: *mut *mut Self, window: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    CreateForCoreWindow: usize,
}
impl ::windows_sys::core::Interface for IHolographicSpaceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911106148, data2: 51442, data3: 15265, data4: [131, 145, 102, 184, 72, 158, 103, 253] };
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
impl ::windows_sys::core::Interface for IHolographicSpaceStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 242708616, data2: 30204, data3: 18607, data4: [135, 88, 6, 82, 246, 240, 124, 89] };
}
#[repr(C)]
pub struct IHolographicSpaceStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsConfigured: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHolographicSpaceStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 989912637, data2: 45475, data3: 19966, data4: [142, 121, 254, 197, 144, 158, 109, 248] };
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
impl ::windows_sys::core::Interface for IHolographicViewConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1545463526, data2: 26601, data3: 20484, data4: [176, 44, 103, 163, 161, 34, 181, 118] };
}
#[repr(C)]
pub struct IHolographicViewConfiguration2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDepthReprojectionMethods: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDepthReprojectionMethods: usize,
}
impl ::windows_sys::core::Interface for IHolographicViewConfiguration2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3795940718, data2: 57552, data3: 20505, data4: [154, 245, 27, 22, 91, 194, 245, 78] };
}
