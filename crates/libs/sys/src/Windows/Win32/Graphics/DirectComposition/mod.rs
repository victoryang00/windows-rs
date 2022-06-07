#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionAttachMouseDragToHwnd(visual: *mut *mut IDCompositionVisual, hwnd: super::super::Foundation::HWND, enable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionAttachMouseWheelToHwnd(visual: *mut *mut IDCompositionVisual, hwnd: super::super::Foundation::HWND, enable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionBoostCompositorClock(enable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn DCompositionCreateDevice(dxgidevice: *mut *mut super::Dxgi::IDXGIDevice, iid: *const ::windows_sys::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub fn DCompositionCreateDevice2(renderingdevice: *mut *mut ::windows_sys::core::IUnknown, iid: *const ::windows_sys::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub fn DCompositionCreateDevice3(renderingdevice: *mut *mut ::windows_sys::core::IUnknown, iid: *const ::windows_sys::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DCompositionCreateSurfaceHandle(desiredaccess: u32, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, surfacehandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub fn DCompositionGetFrameId(frameidtype: COMPOSITION_FRAME_ID_TYPE, frameid: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionGetStatistics(frameid: u64, framestats: *mut COMPOSITION_FRAME_STATS, targetidcount: u32, targetids: *mut COMPOSITION_TARGET_ID, actualtargetidcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionGetTargetStatistics(frameid: u64, targetid: *const COMPOSITION_TARGET_ID, targetstats: *mut COMPOSITION_TARGET_STATS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionWaitForCompositorClock(count: u32, handles: *const super::super::Foundation::HANDLE, timeoutinms: u32) -> u32;
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITIONOBJECT_READ: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITIONOBJECT_WRITE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub type COMPOSITION_FRAME_ID_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITION_FRAME_ID_CREATED: COMPOSITION_FRAME_ID_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITION_FRAME_ID_CONFIRMED: COMPOSITION_FRAME_ID_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITION_FRAME_ID_COMPLETED: COMPOSITION_FRAME_ID_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub struct COMPOSITION_FRAME_STATS {
    pub startTime: u64,
    pub targetTime: u64,
    pub framePeriod: u64,
}
impl ::core::marker::Copy for COMPOSITION_FRAME_STATS {}
impl ::core::clone::Clone for COMPOSITION_FRAME_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub struct COMPOSITION_STATS {
    pub presentCount: u32,
    pub refreshCount: u32,
    pub virtualRefreshCount: u32,
    pub time: u64,
}
impl ::core::marker::Copy for COMPOSITION_STATS {}
impl ::core::clone::Clone for COMPOSITION_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITION_STATS_MAX_TARGETS: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMPOSITION_TARGET_ID {
    pub displayAdapterLuid: super::super::Foundation::LUID,
    pub renderAdapterLuid: super::super::Foundation::LUID,
    pub vidPnSourceId: u32,
    pub vidPnTargetId: u32,
    pub uniqueId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMPOSITION_TARGET_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMPOSITION_TARGET_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub struct COMPOSITION_TARGET_STATS {
    pub outstandingPresents: u32,
    pub presentTime: u64,
    pub vblankDuration: u64,
    pub presentedStats: COMPOSITION_STATS,
    pub completedStats: COMPOSITION_STATS,
}
impl ::core::marker::Copy for COMPOSITION_TARGET_STATS {}
impl ::core::clone::Clone for COMPOSITION_TARGET_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub type DCOMPOSITION_BACKFACE_VISIBILITY = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BACKFACE_VISIBILITY_VISIBLE: DCOMPOSITION_BACKFACE_VISIBILITY = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BACKFACE_VISIBILITY_HIDDEN: DCOMPOSITION_BACKFACE_VISIBILITY = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BACKFACE_VISIBILITY_INHERIT: DCOMPOSITION_BACKFACE_VISIBILITY = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub type DCOMPOSITION_BITMAP_INTERPOLATION_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_LINEAR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_INHERIT: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub type DCOMPOSITION_BORDER_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BORDER_MODE_SOFT: DCOMPOSITION_BORDER_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BORDER_MODE_HARD: DCOMPOSITION_BORDER_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BORDER_MODE_INHERIT: DCOMPOSITION_BORDER_MODE = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub type DCOMPOSITION_COMPOSITE_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_COMPOSITE_MODE_SOURCE_OVER: DCOMPOSITION_COMPOSITE_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_COMPOSITE_MODE_DESTINATION_INVERT: DCOMPOSITION_COMPOSITE_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_COMPOSITE_MODE_MIN_BLEND: DCOMPOSITION_COMPOSITE_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_COMPOSITE_MODE_INHERIT: DCOMPOSITION_COMPOSITE_MODE = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub type DCOMPOSITION_DEPTH_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_DEPTH_MODE_TREE: DCOMPOSITION_DEPTH_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_DEPTH_MODE_SPATIAL: DCOMPOSITION_DEPTH_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_DEPTH_MODE_SORTED: DCOMPOSITION_DEPTH_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_DEPTH_MODE_INHERIT: DCOMPOSITION_DEPTH_MODE = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DCOMPOSITION_FRAME_STATISTICS {
    pub lastFrameTime: i64,
    pub currentCompositionRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub currentTime: i64,
    pub timeFrequency: i64,
    pub nextEstimatedFrameTime: i64,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DCOMPOSITION_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub type DCOMPOSITION_OPACITY_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_OPACITY_MODE_LAYER: DCOMPOSITION_OPACITY_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_OPACITY_MODE_MULTIPLY: DCOMPOSITION_OPACITY_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_OPACITY_MODE_INHERIT: DCOMPOSITION_OPACITY_MODE = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub struct DCompositionInkTrailPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
impl ::core::marker::Copy for DCompositionInkTrailPoint {}
impl ::core::clone::Clone for DCompositionInkTrailPoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IDCompositionAffineTransform2DEffect {
    pub base__: IDCompositionFilterEffect,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetInterpolationMode: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBorderMode: unsafe extern "system" fn(this: *mut *mut Self, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBorderMode: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformMatrix: unsafe extern "system" fn(this: *mut *mut Self, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformMatrix: usize,
    pub SetTransformMatrixElement: unsafe extern "system" fn(this: *mut *mut Self, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTransformMatrixElement2: unsafe extern "system" fn(this: *mut *mut Self, row: i32, column: i32, value: f32) -> ::windows_sys::core::HRESULT,
    pub SetSharpness: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSharpness2: unsafe extern "system" fn(this: *mut *mut Self, sharpness: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionAffineTransform2DEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 192199144, data2: 52694, data3: 18735, data4: [187, 188, 94, 211, 33, 87, 2, 109] };
}
#[repr(C)]
pub struct IDCompositionAnimation {
    pub base__: ::windows_sys::core::IUnknown,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetAbsoluteBeginTime: unsafe extern "system" fn(this: *mut *mut Self, begintime: i64) -> ::windows_sys::core::HRESULT,
    pub AddCubic: unsafe extern "system" fn(this: *mut *mut Self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows_sys::core::HRESULT,
    pub AddSinusoidal: unsafe extern "system" fn(this: *mut *mut Self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows_sys::core::HRESULT,
    pub AddRepeat: unsafe extern "system" fn(this: *mut *mut Self, beginoffset: f64, durationtorepeat: f64) -> ::windows_sys::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut *mut Self, endoffset: f64, endvalue: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3422392793, data2: 20914, data3: 17892, data4: [179, 222, 209, 156, 207, 184, 99, 197] };
}
#[repr(C)]
pub struct IDCompositionArithmeticCompositeEffect {
    pub base__: IDCompositionFilterEffect,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetCoefficients: unsafe extern "system" fn(this: *mut *mut Self, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetCoefficients: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut *mut Self, clampoutput: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
    pub SetCoefficient1: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCoefficient12: unsafe extern "system" fn(this: *mut *mut Self, coeffcient1: f32) -> ::windows_sys::core::HRESULT,
    pub SetCoefficient2: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCoefficient22: unsafe extern "system" fn(this: *mut *mut Self, coefficient2: f32) -> ::windows_sys::core::HRESULT,
    pub SetCoefficient3: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCoefficient32: unsafe extern "system" fn(this: *mut *mut Self, coefficient3: f32) -> ::windows_sys::core::HRESULT,
    pub SetCoefficient4: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCoefficient42: unsafe extern "system" fn(this: *mut *mut Self, coefficient4: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionArithmeticCompositeEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 996663208, data2: 58333, data3: 20065, data4: [182, 64, 70, 194, 243, 215, 57, 220] };
}
#[repr(C)]
pub struct IDCompositionBlendEffect {
    pub base__: IDCompositionFilterEffect,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
impl ::windows_sys::core::Interface for IDCompositionBlendEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 871160842, data2: 22410, data3: 18961, data4: [156, 20, 12, 185, 5, 23, 249, 197] };
}
#[repr(C)]
pub struct IDCompositionBrightnessEffect {
    pub base__: IDCompositionFilterEffect,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetWhitePoint: unsafe extern "system" fn(this: *mut *mut Self, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetWhitePoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBlackPoint: unsafe extern "system" fn(this: *mut *mut Self, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBlackPoint: usize,
    pub SetWhitePointX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetWhitePointX2: unsafe extern "system" fn(this: *mut *mut Self, whitepointx: f32) -> ::windows_sys::core::HRESULT,
    pub SetWhitePointY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetWhitePointY2: unsafe extern "system" fn(this: *mut *mut Self, whitepointy: f32) -> ::windows_sys::core::HRESULT,
    pub SetBlackPointX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBlackPointX2: unsafe extern "system" fn(this: *mut *mut Self, blackpointx: f32) -> ::windows_sys::core::HRESULT,
    pub SetBlackPointY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBlackPointY2: unsafe extern "system" fn(this: *mut *mut Self, blackpointy: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionBrightnessEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1613187438, data2: 52026, data3: 18859, data4: [147, 79, 215, 152, 218, 79, 125, 166] };
}
#[repr(C)]
pub struct IDCompositionClip {
    pub base__: ::windows_sys::core::IUnknown,
}
impl ::windows_sys::core::Interface for IDCompositionClip {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1689007875, data2: 40255, data3: 17900, data4: [161, 9, 124, 172, 14, 122, 19, 167] };
}
#[repr(C)]
pub struct IDCompositionColorMatrixEffect {
    pub base__: IDCompositionFilterEffect,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut *mut Self, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut *mut Self, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut *mut Self, row: i32, column: i32, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetAlphaMode: unsafe extern "system" fn(this: *mut *mut Self, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetAlphaMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut *mut Self, clamp: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
}
impl ::windows_sys::core::Interface for IDCompositionColorMatrixEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3239512610, data2: 15586, data3: 18790, data4: [144, 212, 85, 64, 139, 252, 132, 196] };
}
#[repr(C)]
pub struct IDCompositionCompositeEffect {
    pub base__: IDCompositionFilterEffect,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
impl ::windows_sys::core::Interface for IDCompositionCompositeEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1466308288, data2: 41521, data3: 18765, data4: [163, 141, 0, 253, 94, 196, 219, 70] };
}
#[repr(C)]
pub struct IDCompositionDelegatedInkTrail {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddTrailPoints: unsafe extern "system" fn(this: *mut *mut Self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AddTrailPointsWithPrediction: unsafe extern "system" fn(this: *mut *mut Self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub RemoveTrailPoints: unsafe extern "system" fn(this: *mut *mut Self, generationid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub StartNewTrail: unsafe extern "system" fn(this: *mut *mut Self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    StartNewTrail: usize,
}
impl ::windows_sys::core::Interface for IDCompositionDelegatedInkTrail {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3259272859, data2: 21629, data3: 16471, data4: [140, 245, 129, 68, 237, 225, 194, 218] };
}
#[repr(C)]
pub struct IDCompositionDesktopDevice {
    pub base__: IDCompositionDevice2,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTargetForHwnd: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTargetForHwnd: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(this: *mut *mut Self, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHwnd: usize,
}
impl ::windows_sys::core::Interface for IDCompositionDesktopDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1598436350, data2: 7688, data3: 19640, data4: [140, 117, 206, 36, 51, 63, 86, 2] };
}
#[repr(C)]
pub struct IDCompositionDevice {
    pub base__: ::windows_sys::core::IUnknown,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WaitForCommitCompletion: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut *mut Self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFrameStatistics: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTargetForHwnd: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTargetForHwnd: usize,
    pub CreateVisual: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut *mut Self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut *mut Self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(this: *mut *mut Self, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHwnd: usize,
    pub CreateTranslateTransform: unsafe extern "system" fn(this: *mut *mut Self, translatetransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateScaleTransform: unsafe extern "system" fn(this: *mut *mut Self, scaletransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRotateTransform: unsafe extern "system" fn(this: *mut *mut Self, rotatetransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSkewTransform: unsafe extern "system" fn(this: *mut *mut Self, skewtransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut *mut Self, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTransformGroup: unsafe extern "system" fn(this: *mut *mut Self, transforms: *const *mut ::core::ffi::c_void, elements: u32, transformgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTranslateTransform3D: unsafe extern "system" fn(this: *mut *mut Self, translatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateScaleTransform3D: unsafe extern "system" fn(this: *mut *mut Self, scaletransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRotateTransform3D: unsafe extern "system" fn(this: *mut *mut Self, rotatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateMatrixTransform3D: unsafe extern "system" fn(this: *mut *mut Self, matrixtransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTransform3DGroup: unsafe extern "system" fn(this: *mut *mut Self, transforms3d: *const *mut ::core::ffi::c_void, elements: u32, transform3dgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateEffectGroup: unsafe extern "system" fn(this: *mut *mut Self, effectgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(this: *mut *mut Self, clip: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAnimation: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckDeviceState: unsafe extern "system" fn(this: *mut *mut Self, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckDeviceState: usize,
}
impl ::windows_sys::core::Interface for IDCompositionDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3279857978, data2: 59306, data3: 17677, data4: [177, 111, 151, 70, 203, 4, 7, 243] };
}
#[repr(C)]
pub struct IDCompositionDevice2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WaitForCommitCompletion: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut *mut Self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFrameStatistics: usize,
    pub CreateVisual: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSurfaceFactory: unsafe extern "system" fn(this: *mut *mut Self, renderingdevice: *mut ::core::ffi::c_void, surfacefactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut *mut Self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut *mut Self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
    pub CreateTranslateTransform: unsafe extern "system" fn(this: *mut *mut Self, translatetransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateScaleTransform: unsafe extern "system" fn(this: *mut *mut Self, scaletransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRotateTransform: unsafe extern "system" fn(this: *mut *mut Self, rotatetransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSkewTransform: unsafe extern "system" fn(this: *mut *mut Self, skewtransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut *mut Self, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTransformGroup: unsafe extern "system" fn(this: *mut *mut Self, transforms: *const *mut ::core::ffi::c_void, elements: u32, transformgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTranslateTransform3D: unsafe extern "system" fn(this: *mut *mut Self, translatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateScaleTransform3D: unsafe extern "system" fn(this: *mut *mut Self, scaletransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRotateTransform3D: unsafe extern "system" fn(this: *mut *mut Self, rotatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateMatrixTransform3D: unsafe extern "system" fn(this: *mut *mut Self, matrixtransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTransform3DGroup: unsafe extern "system" fn(this: *mut *mut Self, transforms3d: *const *mut ::core::ffi::c_void, elements: u32, transform3dgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateEffectGroup: unsafe extern "system" fn(this: *mut *mut Self, effectgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(this: *mut *mut Self, clip: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAnimation: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionDevice2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1979074189, data2: 7054, data3: 17532, data4: [155, 198, 117, 254, 168, 11, 91, 37] };
}
#[repr(C)]
pub struct IDCompositionDevice3 {
    pub base__: IDCompositionDevice2,
    pub CreateGaussianBlurEffect: unsafe extern "system" fn(this: *mut *mut Self, gaussianblureffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBrightnessEffect: unsafe extern "system" fn(this: *mut *mut Self, brightnesseffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateColorMatrixEffect: unsafe extern "system" fn(this: *mut *mut Self, colormatrixeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateShadowEffect: unsafe extern "system" fn(this: *mut *mut Self, shadoweffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateHueRotationEffect: unsafe extern "system" fn(this: *mut *mut Self, huerotationeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSaturationEffect: unsafe extern "system" fn(this: *mut *mut Self, saturationeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTurbulenceEffect: unsafe extern "system" fn(this: *mut *mut Self, turbulenceeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLinearTransferEffect: unsafe extern "system" fn(this: *mut *mut Self, lineartransfereffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTableTransferEffect: unsafe extern "system" fn(this: *mut *mut Self, tabletransfereffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateCompositeEffect: unsafe extern "system" fn(this: *mut *mut Self, compositeeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBlendEffect: unsafe extern "system" fn(this: *mut *mut Self, blendeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateArithmeticCompositeEffect: unsafe extern "system" fn(this: *mut *mut Self, arithmeticcompositeeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAffineTransform2DEffect: unsafe extern "system" fn(this: *mut *mut Self, affinetransform2deffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionDevice3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 159894278, data2: 63766, data3: 18623, data4: [141, 53, 206, 118, 65, 120, 27, 217] };
}
#[repr(C)]
pub struct IDCompositionDeviceDebug {
    pub base__: ::windows_sys::core::IUnknown,
    pub EnableDebugCounters: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DisableDebugCounters: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionDeviceDebug {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2711864906, data2: 8783, data3: 19073, data4: [151, 115, 79, 3, 168, 157, 60, 108] };
}
#[repr(C)]
pub struct IDCompositionEffect {
    pub base__: ::windows_sys::core::IUnknown,
}
impl ::windows_sys::core::Interface for IDCompositionEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3967922319, data2: 49099, data3: 20109, data4: [177, 147, 169, 21, 88, 121, 153, 232] };
}
#[repr(C)]
pub struct IDCompositionEffectGroup {
    pub base__: IDCompositionEffect,
    pub SetOpacity: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(this: *mut *mut Self, opacity: f32) -> ::windows_sys::core::HRESULT,
    pub SetTransform3D: unsafe extern "system" fn(this: *mut *mut Self, transform3d: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionEffectGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2811402868, data2: 59058, data3: 19414, data4: [139, 149, 64, 64, 17, 156, 163, 77] };
}
#[repr(C)]
pub struct IDCompositionFilterEffect {
    pub base__: IDCompositionEffect,
    pub SetInput: unsafe extern "system" fn(this: *mut *mut Self, index: u32, input: *mut ::core::ffi::c_void, flags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionFilterEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 818160085, data2: 36018, data3: 20127, data4: [177, 51, 55, 190, 39, 13, 74, 194] };
}
#[repr(C)]
pub struct IDCompositionGaussianBlurEffect {
    pub base__: IDCompositionFilterEffect,
    pub SetStandardDeviation: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(this: *mut *mut Self, amount: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBorderMode: unsafe extern "system" fn(this: *mut *mut Self, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBorderMode: usize,
}
impl ::windows_sys::core::Interface for IDCompositionGaussianBlurEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1171574967, data2: 7124, data3: 17742, data4: [136, 148, 43, 250, 104, 68, 48, 51] };
}
#[repr(C)]
pub struct IDCompositionHueRotationEffect {
    pub base__: IDCompositionFilterEffect,
    pub SetAngle: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut *mut Self, amountdegrees: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionHueRotationEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1840904480, data2: 1904, data3: 18305, data4: [176, 198, 56, 25, 18, 249, 209, 103] };
}
#[repr(C)]
pub struct IDCompositionInkTrailDevice {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateDelegatedInkTrail: unsafe extern "system" fn(this: *mut *mut Self, inktrail: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDelegatedInkTrailForSwapChain: unsafe extern "system" fn(this: *mut *mut Self, swapchain: *mut ::core::ffi::c_void, inktrail: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionInkTrailDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3742137580, data2: 52715, data3: 19786, data4: [185, 28, 114, 27, 242, 47, 78, 108] };
}
#[repr(C)]
pub struct IDCompositionLinearTransferEffect {
    pub base__: IDCompositionFilterEffect,
    pub SetRedYIntercept: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRedYIntercept2: unsafe extern "system" fn(this: *mut *mut Self, redyintercept: f32) -> ::windows_sys::core::HRESULT,
    pub SetRedSlope: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRedSlope2: unsafe extern "system" fn(this: *mut *mut Self, redslope: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRedDisable: unsafe extern "system" fn(this: *mut *mut Self, reddisable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRedDisable: usize,
    pub SetGreenYIntercept: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGreenYIntercept2: unsafe extern "system" fn(this: *mut *mut Self, greenyintercept: f32) -> ::windows_sys::core::HRESULT,
    pub SetGreenSlope: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGreenSlope2: unsafe extern "system" fn(this: *mut *mut Self, greenslope: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGreenDisable: unsafe extern "system" fn(this: *mut *mut Self, greendisable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGreenDisable: usize,
    pub SetBlueYIntercept: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBlueYIntercept2: unsafe extern "system" fn(this: *mut *mut Self, blueyintercept: f32) -> ::windows_sys::core::HRESULT,
    pub SetBlueSlope: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBlueSlope2: unsafe extern "system" fn(this: *mut *mut Self, blueslope: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBlueDisable: unsafe extern "system" fn(this: *mut *mut Self, bluedisable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBlueDisable: usize,
    pub SetAlphaYIntercept: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAlphaYIntercept2: unsafe extern "system" fn(this: *mut *mut Self, alphayintercept: f32) -> ::windows_sys::core::HRESULT,
    pub SetAlphaSlope: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAlphaSlope2: unsafe extern "system" fn(this: *mut *mut Self, alphaslope: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAlphaDisable: unsafe extern "system" fn(this: *mut *mut Self, alphadisable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAlphaDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut *mut Self, clampoutput: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
}
impl ::windows_sys::core::Interface for IDCompositionLinearTransferEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1124462171, data2: 50336, data3: 19592, data4: [147, 133, 103, 18, 78, 1, 118, 131] };
}
#[repr(C)]
pub struct IDCompositionMatrixTransform {
    pub base__: IDCompositionTransform,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut *mut Self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut *mut Self, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut *mut Self, row: i32, column: i32, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionMatrixTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 382598919, data2: 50435, data3: 16796, data4: [131, 242, 9, 101, 199, 175, 31, 166] };
}
#[repr(C)]
pub struct IDCompositionMatrixTransform3D {
    pub base__: IDCompositionTransform3D,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut *mut Self, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut *mut Self, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut *mut Self, row: i32, column: i32, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionMatrixTransform3D {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1261659120, data2: 25659, data3: 16823, data4: [182, 224, 204, 242, 45, 52, 70, 124] };
}
#[repr(C)]
pub struct IDCompositionRectangleClip {
    pub base__: IDCompositionClip,
    pub SetLeft: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLeft2: unsafe extern "system" fn(this: *mut *mut Self, left: f32) -> ::windows_sys::core::HRESULT,
    pub SetTop: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTop2: unsafe extern "system" fn(this: *mut *mut Self, top: f32) -> ::windows_sys::core::HRESULT,
    pub SetRight: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRight2: unsafe extern "system" fn(this: *mut *mut Self, right: f32) -> ::windows_sys::core::HRESULT,
    pub SetBottom: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBottom2: unsafe extern "system" fn(this: *mut *mut Self, bottom: f32) -> ::windows_sys::core::HRESULT,
    pub SetTopLeftRadiusX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTopLeftRadiusX2: unsafe extern "system" fn(this: *mut *mut Self, radius: f32) -> ::windows_sys::core::HRESULT,
    pub SetTopLeftRadiusY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTopLeftRadiusY2: unsafe extern "system" fn(this: *mut *mut Self, radius: f32) -> ::windows_sys::core::HRESULT,
    pub SetTopRightRadiusX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTopRightRadiusX2: unsafe extern "system" fn(this: *mut *mut Self, radius: f32) -> ::windows_sys::core::HRESULT,
    pub SetTopRightRadiusY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTopRightRadiusY2: unsafe extern "system" fn(this: *mut *mut Self, radius: f32) -> ::windows_sys::core::HRESULT,
    pub SetBottomLeftRadiusX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBottomLeftRadiusX2: unsafe extern "system" fn(this: *mut *mut Self, radius: f32) -> ::windows_sys::core::HRESULT,
    pub SetBottomLeftRadiusY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBottomLeftRadiusY2: unsafe extern "system" fn(this: *mut *mut Self, radius: f32) -> ::windows_sys::core::HRESULT,
    pub SetBottomRightRadiusX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBottomRightRadiusX2: unsafe extern "system" fn(this: *mut *mut Self, radius: f32) -> ::windows_sys::core::HRESULT,
    pub SetBottomRightRadiusY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBottomRightRadiusY2: unsafe extern "system" fn(this: *mut *mut Self, radius: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionRectangleClip {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2554506621, data2: 55759, data3: 18696, data4: [174, 215, 72, 181, 29, 165, 231, 194] };
}
#[repr(C)]
pub struct IDCompositionRotateTransform {
    pub base__: IDCompositionTransform,
    pub SetAngle: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut *mut Self, angle: f32) -> ::windows_sys::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut *mut Self, centerx: f32) -> ::windows_sys::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut *mut Self, centery: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionRotateTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1679743036, data2: 44694, data3: 18117, data4: [144, 220, 50, 119, 76, 197, 198, 213] };
}
#[repr(C)]
pub struct IDCompositionRotateTransform3D {
    pub base__: IDCompositionTransform3D,
    pub SetAngle: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut *mut Self, angle: f32) -> ::windows_sys::core::HRESULT,
    pub SetAxisX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAxisX2: unsafe extern "system" fn(this: *mut *mut Self, axisx: f32) -> ::windows_sys::core::HRESULT,
    pub SetAxisY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAxisY2: unsafe extern "system" fn(this: *mut *mut Self, axisy: f32) -> ::windows_sys::core::HRESULT,
    pub SetAxisZ: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAxisZ2: unsafe extern "system" fn(this: *mut *mut Self, axisz: f32) -> ::windows_sys::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut *mut Self, centerx: f32) -> ::windows_sys::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut *mut Self, centery: f32) -> ::windows_sys::core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCenterZ2: unsafe extern "system" fn(this: *mut *mut Self, centerz: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionRotateTransform3D {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3639980607, data2: 54313, data3: 19089, data4: [181, 90, 210, 244, 95, 215, 91, 24] };
}
#[repr(C)]
pub struct IDCompositionSaturationEffect {
    pub base__: IDCompositionFilterEffect,
    pub SetSaturation: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSaturation2: unsafe extern "system" fn(this: *mut *mut Self, ratio: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionSaturationEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2693655514, data2: 12888, data3: 20388, data4: [159, 22, 145, 116, 211, 254, 147, 177] };
}
#[repr(C)]
pub struct IDCompositionScaleTransform {
    pub base__: IDCompositionTransform,
    pub SetScaleX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetScaleX2: unsafe extern "system" fn(this: *mut *mut Self, scalex: f32) -> ::windows_sys::core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetScaleY2: unsafe extern "system" fn(this: *mut *mut Self, scaley: f32) -> ::windows_sys::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut *mut Self, centerx: f32) -> ::windows_sys::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut *mut Self, centery: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionScaleTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1912465684, data2: 16623, data3: 17903, data4: [189, 81, 104, 176, 55, 195, 57, 249] };
}
#[repr(C)]
pub struct IDCompositionScaleTransform3D {
    pub base__: IDCompositionTransform3D,
    pub SetScaleX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetScaleX2: unsafe extern "system" fn(this: *mut *mut Self, scalex: f32) -> ::windows_sys::core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetScaleY2: unsafe extern "system" fn(this: *mut *mut Self, scaley: f32) -> ::windows_sys::core::HRESULT,
    pub SetScaleZ: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetScaleZ2: unsafe extern "system" fn(this: *mut *mut Self, scalez: f32) -> ::windows_sys::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut *mut Self, centerx: f32) -> ::windows_sys::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut *mut Self, centery: f32) -> ::windows_sys::core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCenterZ2: unsafe extern "system" fn(this: *mut *mut Self, centerz: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionScaleTransform3D {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 715038381, data2: 13899, data3: 19221, data4: [167, 196, 161, 153, 127, 120, 179, 137] };
}
#[repr(C)]
pub struct IDCompositionShadowEffect {
    pub base__: IDCompositionFilterEffect,
    pub SetStandardDeviation: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(this: *mut *mut Self, amount: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetColor: usize,
    pub SetRed: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRed2: unsafe extern "system" fn(this: *mut *mut Self, amount: f32) -> ::windows_sys::core::HRESULT,
    pub SetGreen: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGreen2: unsafe extern "system" fn(this: *mut *mut Self, amount: f32) -> ::windows_sys::core::HRESULT,
    pub SetBlue: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBlue2: unsafe extern "system" fn(this: *mut *mut Self, amount: f32) -> ::windows_sys::core::HRESULT,
    pub SetAlpha: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAlpha2: unsafe extern "system" fn(this: *mut *mut Self, amount: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionShadowEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1255246528, data2: 53202, data3: 19503, data4: [187, 98, 150, 229, 79, 219, 104, 121] };
}
#[repr(C)]
pub struct IDCompositionSkewTransform {
    pub base__: IDCompositionTransform,
    pub SetAngleX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAngleX2: unsafe extern "system" fn(this: *mut *mut Self, anglex: f32) -> ::windows_sys::core::HRESULT,
    pub SetAngleY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAngleY2: unsafe extern "system" fn(this: *mut *mut Self, angley: f32) -> ::windows_sys::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut *mut Self, centerx: f32) -> ::windows_sys::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut *mut Self, centery: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionSkewTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3850020661, data2: 56539, data3: 19570, data4: [156, 97, 5, 145, 245, 136, 137, 238] };
}
#[repr(C)]
pub struct IDCompositionSurface {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginDraw: unsafe extern "system" fn(this: *mut *mut Self, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows_sys::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginDraw: usize,
    pub EndDraw: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Scroll: unsafe extern "system" fn(this: *mut *mut Self, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Scroll: usize,
}
impl ::windows_sys::core::Interface for IDCompositionSurface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3146402131, data2: 11417, data3: 20314, data4: [150, 245, 72, 25, 2, 127, 163, 172] };
}
#[repr(C)]
pub struct IDCompositionSurfaceFactory {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut *mut Self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut *mut Self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
}
impl ::windows_sys::core::Interface for IDCompositionSurfaceFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3811884050, data2: 14647, data3: 19970, data4: [133, 235, 252, 244, 235, 48, 210, 200] };
}
#[repr(C)]
pub struct IDCompositionTableTransferEffect {
    pub base__: IDCompositionFilterEffect,
    pub SetRedTable: unsafe extern "system" fn(this: *mut *mut Self, tablevalues: *const f32, count: u32) -> ::windows_sys::core::HRESULT,
    pub SetGreenTable: unsafe extern "system" fn(this: *mut *mut Self, tablevalues: *const f32, count: u32) -> ::windows_sys::core::HRESULT,
    pub SetBlueTable: unsafe extern "system" fn(this: *mut *mut Self, tablevalues: *const f32, count: u32) -> ::windows_sys::core::HRESULT,
    pub SetAlphaTable: unsafe extern "system" fn(this: *mut *mut Self, tablevalues: *const f32, count: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRedDisable: unsafe extern "system" fn(this: *mut *mut Self, reddisable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRedDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGreenDisable: unsafe extern "system" fn(this: *mut *mut Self, greendisable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGreenDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBlueDisable: unsafe extern "system" fn(this: *mut *mut Self, bluedisable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBlueDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAlphaDisable: unsafe extern "system" fn(this: *mut *mut Self, alphadisable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAlphaDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut *mut Self, clampoutput: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
    pub SetRedTableValue: unsafe extern "system" fn(this: *mut *mut Self, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRedTableValue2: unsafe extern "system" fn(this: *mut *mut Self, index: u32, value: f32) -> ::windows_sys::core::HRESULT,
    pub SetGreenTableValue: unsafe extern "system" fn(this: *mut *mut Self, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGreenTableValue2: unsafe extern "system" fn(this: *mut *mut Self, index: u32, value: f32) -> ::windows_sys::core::HRESULT,
    pub SetBlueTableValue: unsafe extern "system" fn(this: *mut *mut Self, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBlueTableValue2: unsafe extern "system" fn(this: *mut *mut Self, index: u32, value: f32) -> ::windows_sys::core::HRESULT,
    pub SetAlphaTableValue: unsafe extern "system" fn(this: *mut *mut Self, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAlphaTableValue2: unsafe extern "system" fn(this: *mut *mut Self, index: u32, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionTableTransferEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2608759522, data2: 27077, data3: 20148, data4: [165, 245, 167, 3, 63, 81, 50, 205] };
}
#[repr(C)]
pub struct IDCompositionTarget {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetRoot: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3939356748, data2: 4478, data3: 19991, data4: [136, 244, 209, 177, 43, 14, 61, 137] };
}
#[repr(C)]
pub struct IDCompositionTransform {
    pub base__: IDCompositionTransform3D,
}
impl ::windows_sys::core::Interface for IDCompositionTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4250270375, data2: 14304, data3: 19488, data4: [149, 210, 155, 228, 91, 195, 63, 85] };
}
#[repr(C)]
pub struct IDCompositionTransform3D {
    pub base__: IDCompositionEffect,
}
impl ::windows_sys::core::Interface for IDCompositionTransform3D {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1897420578, data2: 9323, data3: 16882, data4: [170, 209, 4, 67, 247, 244, 191, 194] };
}
#[repr(C)]
pub struct IDCompositionTranslateTransform {
    pub base__: IDCompositionTransform,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut *mut Self, offsetx: f32) -> ::windows_sys::core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut *mut Self, offsety: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionTranslateTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 108597538, data2: 50928, data3: 16765, data4: [131, 35, 38, 158, 152, 127, 89, 84] };
}
#[repr(C)]
pub struct IDCompositionTranslateTransform3D {
    pub base__: IDCompositionTransform3D,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut *mut Self, offsetx: f32) -> ::windows_sys::core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut *mut Self, offsety: f32) -> ::windows_sys::core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(this: *mut *mut Self, offsetz: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionTranslateTransform3D {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2439212363, data2: 39841, data3: 17714, data4: [170, 247, 227, 52, 73, 148, 215, 136] };
}
#[repr(C)]
pub struct IDCompositionTurbulenceEffect {
    pub base__: IDCompositionFilterEffect,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetOffset: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBaseFrequency: unsafe extern "system" fn(this: *mut *mut Self, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBaseFrequency: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetSize: usize,
    pub SetNumOctaves: unsafe extern "system" fn(this: *mut *mut Self, numoctaves: u32) -> ::windows_sys::core::HRESULT,
    pub SetSeed: unsafe extern "system" fn(this: *mut *mut Self, seed: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetNoise: unsafe extern "system" fn(this: *mut *mut Self, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetNoise: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStitchable: unsafe extern "system" fn(this: *mut *mut Self, stitchable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStitchable: usize,
}
impl ::windows_sys::core::Interface for IDCompositionTurbulenceEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2795854810, data2: 49308, data3: 18931, data4: [145, 147, 164, 25, 34, 200, 151, 21] };
}
#[repr(C)]
pub struct IDCompositionVirtualSurface {
    pub base__: IDCompositionSurface,
    pub Resize: unsafe extern "system" fn(this: *mut *mut Self, width: u32, height: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Trim: unsafe extern "system" fn(this: *mut *mut Self, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Trim: usize,
}
impl ::windows_sys::core::Interface for IDCompositionVirtualSurface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2923895889, data2: 24403, data3: 18980, data4: [141, 62, 208, 195, 156, 48, 179, 240] };
}
#[repr(C)]
pub struct IDCompositionVisual {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut *mut Self, offsetx: f32) -> ::windows_sys::core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut *mut Self, offsety: f32) -> ::windows_sys::core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform2: unsafe extern "system" fn(this: *mut *mut Self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform2: usize,
    pub SetTransformParent: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEffect: unsafe extern "system" fn(this: *mut *mut Self, effect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBitmapInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows_sys::core::HRESULT,
    pub SetBorderMode: unsafe extern "system" fn(this: *mut *mut Self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows_sys::core::HRESULT,
    pub SetClip: unsafe extern "system" fn(this: *mut *mut Self, clip: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetClip2: unsafe extern "system" fn(this: *mut *mut Self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetClip2: usize,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddVisual: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void, insertabove: super::super::Foundation::BOOL, referencevisual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddVisual: usize,
    pub RemoveVisual: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAllVisuals: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetCompositeMode: unsafe extern "system" fn(this: *mut *mut Self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionVisual {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1301480861, data2: 2427, data3: 18001, data4: [154, 96, 240, 242, 81, 22, 226, 243] };
}
#[repr(C)]
pub struct IDCompositionVisual2 {
    pub base__: IDCompositionVisual,
    pub SetOpacityMode: unsafe extern "system" fn(this: *mut *mut Self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows_sys::core::HRESULT,
    pub SetBackFaceVisibility: unsafe extern "system" fn(this: *mut *mut Self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionVisual2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3906868793, data2: 17201, data3: 19238, data4: [188, 95, 106, 50, 29, 52, 122, 133] };
}
#[repr(C)]
pub struct IDCompositionVisual3 {
    pub base__: IDCompositionVisualDebug,
    pub SetDepthMode: unsafe extern "system" fn(this: *mut *mut Self, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows_sys::core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(this: *mut *mut Self, offsetz: f32) -> ::windows_sys::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(this: *mut *mut Self, opacity: f32) -> ::windows_sys::core::HRESULT,
    pub SetTransform3: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetTransform4: unsafe extern "system" fn(this: *mut *mut Self, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetTransform4: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVisible: unsafe extern "system" fn(this: *mut *mut Self, visible: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVisible: usize,
}
impl ::windows_sys::core::Interface for IDCompositionVisual3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 662041698, data2: 46785, data3: 16405, data4: [176, 190, 179, 231, 214, 164, 151, 109] };
}
#[repr(C)]
pub struct IDCompositionVisualDebug {
    pub base__: IDCompositionVisual2,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub EnableHeatMap: unsafe extern "system" fn(this: *mut *mut Self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    EnableHeatMap: usize,
    pub DisableHeatMap: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnableRedrawRegions: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DisableRedrawRegions: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDCompositionVisualDebug {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4275222536, data2: 24244, data3: 17312, data4: [174, 163, 53, 246, 82, 128, 249, 27] };
}
