#[cfg(feature = "UI_Xaml_Media_Animation")]
pub mod Animation;
#[cfg(feature = "UI_Xaml_Media_Imaging")]
pub mod Imaging;
#[cfg(feature = "UI_Xaml_Media_Media3D")]
pub mod Media3D;
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct AcrylicBackgroundSource(pub i32);
impl AcrylicBackgroundSource {
    pub const HostBackdrop: Self = Self(0i32);
    pub const Backdrop: Self = Self(1i32);
}
impl ::core::marker::Copy for AcrylicBackgroundSource {}
impl ::core::clone::Clone for AcrylicBackgroundSource {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AcrylicBrush = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct AlignmentX(pub i32);
impl AlignmentX {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for AlignmentX {}
impl ::core::clone::Clone for AlignmentX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct AlignmentY(pub i32);
impl AlignmentY {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
impl ::core::marker::Copy for AlignmentY {}
impl ::core::clone::Clone for AlignmentY {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ArcSegment = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct AudioCategory(pub i32);
impl AudioCategory {
    pub const Other: Self = Self(0i32);
    pub const ForegroundOnlyMedia: Self = Self(1i32);
    pub const BackgroundCapableMedia: Self = Self(2i32);
    pub const Communications: Self = Self(3i32);
    pub const Alerts: Self = Self(4i32);
    pub const SoundEffects: Self = Self(5i32);
    pub const GameEffects: Self = Self(6i32);
    pub const GameMedia: Self = Self(7i32);
    pub const GameChat: Self = Self(8i32);
    pub const Speech: Self = Self(9i32);
    pub const Movie: Self = Self(10i32);
    pub const Media: Self = Self(11i32);
}
impl ::core::marker::Copy for AudioCategory {}
impl ::core::clone::Clone for AudioCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct AudioDeviceType(pub i32);
impl AudioDeviceType {
    pub const Console: Self = Self(0i32);
    pub const Multimedia: Self = Self(1i32);
    pub const Communications: Self = Self(2i32);
}
impl ::core::marker::Copy for AudioDeviceType {}
impl ::core::clone::Clone for AudioDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BezierSegment = *mut ::core::ffi::c_void;
pub type BitmapCache = *mut ::core::ffi::c_void;
pub type Brush = *mut ::core::ffi::c_void;
pub type BrushCollection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct BrushMappingMode(pub i32);
impl BrushMappingMode {
    pub const Absolute: Self = Self(0i32);
    pub const RelativeToBoundingBox: Self = Self(1i32);
}
impl ::core::marker::Copy for BrushMappingMode {}
impl ::core::clone::Clone for BrushMappingMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CacheMode = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct ColorInterpolationMode(pub i32);
impl ColorInterpolationMode {
    pub const ScRgbLinearInterpolation: Self = Self(0i32);
    pub const SRgbLinearInterpolation: Self = Self(1i32);
}
impl ::core::marker::Copy for ColorInterpolationMode {}
impl ::core::clone::Clone for ColorInterpolationMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositeTransform = *mut ::core::ffi::c_void;
pub type CompositionTarget = *mut ::core::ffi::c_void;
pub type DoubleCollection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct ElementCompositeMode(pub i32);
impl ElementCompositeMode {
    pub const Inherit: Self = Self(0i32);
    pub const SourceOver: Self = Self(1i32);
    pub const MinBlend: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementCompositeMode {}
impl ::core::clone::Clone for ElementCompositeMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EllipseGeometry = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct FastPlayFallbackBehaviour(pub i32);
impl FastPlayFallbackBehaviour {
    pub const Skip: Self = Self(0i32);
    pub const Hide: Self = Self(1i32);
    pub const Disable: Self = Self(2i32);
}
impl ::core::marker::Copy for FastPlayFallbackBehaviour {}
impl ::core::clone::Clone for FastPlayFallbackBehaviour {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct FillRule(pub i32);
impl FillRule {
    pub const EvenOdd: Self = Self(0i32);
    pub const Nonzero: Self = Self(1i32);
}
impl ::core::marker::Copy for FillRule {}
impl ::core::clone::Clone for FillRule {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FontFamily = *mut ::core::ffi::c_void;
pub type GeneralTransform = *mut ::core::ffi::c_void;
pub type Geometry = *mut ::core::ffi::c_void;
pub type GeometryCollection = *mut ::core::ffi::c_void;
pub type GeometryGroup = *mut ::core::ffi::c_void;
pub type GradientBrush = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct GradientSpreadMethod(pub i32);
impl GradientSpreadMethod {
    pub const Pad: Self = Self(0i32);
    pub const Reflect: Self = Self(1i32);
    pub const Repeat: Self = Self(2i32);
}
impl ::core::marker::Copy for GradientSpreadMethod {}
impl ::core::clone::Clone for GradientSpreadMethod {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GradientStop = *mut ::core::ffi::c_void;
pub type GradientStopCollection = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAcrylicBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AcrylicBackgroundSource) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundSource: unsafe extern "system" fn(this: *mut *mut Self, value: AcrylicBackgroundSource) -> ::windows_sys::core::HRESULT,
    pub TintColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetTintColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Color) -> ::windows_sys::core::HRESULT,
    pub TintOpacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetTintOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TintTransitionDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TintTransitionDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetTintTransitionDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTintTransitionDuration: usize,
    pub AlwaysUseFallback: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlwaysUseFallback: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAcrylicBrush2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TintLuminosityOpacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TintLuminosityOpacity: usize,
    #[cfg(feature = "Foundation")]
    pub SetTintLuminosityOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTintLuminosityOpacity: usize,
}
#[repr(C)]
pub struct IAcrylicBrushFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAcrylicBrushStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TintColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TintOpacityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TintTransitionDurationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AlwaysUseFallbackProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAcrylicBrushStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TintLuminosityOpacityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IArcSegment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Point: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Point: usize,
    #[cfg(feature = "Foundation")]
    pub SetPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPoint: usize,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    #[cfg(feature = "Foundation")]
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSize: usize,
    pub RotationAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub IsLargeArc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsLargeArc: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SweepDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SweepDirection) -> ::windows_sys::core::HRESULT,
    pub SetSweepDirection: unsafe extern "system" fn(this: *mut *mut Self, value: SweepDirection) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IArcSegmentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PointProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RotationAngleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsLargeArcProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SweepDirectionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBezierSegment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Point1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Point1: usize,
    #[cfg(feature = "Foundation")]
    pub SetPoint1: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPoint1: usize,
    #[cfg(feature = "Foundation")]
    pub Point2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Point2: usize,
    #[cfg(feature = "Foundation")]
    pub SetPoint2: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPoint2: usize,
    #[cfg(feature = "Foundation")]
    pub Point3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Point3: usize,
    #[cfg(feature = "Foundation")]
    pub SetPoint3: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPoint3: usize,
}
#[repr(C)]
pub struct IBezierSegmentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Point1Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Point2Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Point3Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapCache {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub Opacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Transform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RelativeTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRelativeTransform: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBrushFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBrushOverrides2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub PopulatePropertyInfoOverride: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, animationpropertyinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    PopulatePropertyInfoOverride: usize,
}
#[repr(C)]
pub struct IBrushStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpacityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TransformProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RelativeTransformProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICacheMode {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICacheModeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositeTransform {
    pub base__: ::windows_sys::core::IInspectable,
    pub CenterX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub CenterY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ScaleX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetScaleX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ScaleY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub SkewX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetSkewX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub SkewY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetSkewY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub TranslateX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetTranslateX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub TranslateY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetTranslateY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositeTransformStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CenterXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ScaleXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ScaleYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SkewXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SkewYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RotationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TranslateXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TranslateYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionTarget {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionTargetStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Rendering: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Rendering: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRendering: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRendering: usize,
    #[cfg(feature = "Foundation")]
    pub SurfaceContentsLost: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SurfaceContentsLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSurfaceContentsLost: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSurfaceContentsLost: usize,
}
#[repr(C)]
pub struct ICompositionTargetStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Rendered: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Rendered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRendered: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRendered: usize,
}
#[repr(C)]
pub struct IEllipseGeometry {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Center: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Center: usize,
    #[cfg(feature = "Foundation")]
    pub SetCenter: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCenter: usize,
    pub RadiusX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRadiusX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RadiusY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRadiusY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEllipseGeometryStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CenterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RadiusXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RadiusYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFontFamily {
    pub base__: ::windows_sys::core::IInspectable,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFontFamilyFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithName: unsafe extern "system" fn(this: *mut *mut Self, familyname: ::windows_sys::core::HSTRING, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFontFamilyStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub XamlAutoFontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeneralTransform {
    pub base__: ::windows_sys::core::IInspectable,
    pub Inverse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TransformPoint: unsafe extern "system" fn(this: *mut *mut Self, point: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransformPoint: usize,
    #[cfg(feature = "Foundation")]
    pub TryTransform: unsafe extern "system" fn(this: *mut *mut Self, inpoint: super::super::super::Foundation::Point, outpoint: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTransform: usize,
    #[cfg(feature = "Foundation")]
    pub TransformBounds: unsafe extern "system" fn(this: *mut *mut Self, rect: super::super::super::Foundation::Rect, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransformBounds: usize,
}
#[repr(C)]
pub struct IGeneralTransformFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeneralTransformOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub InverseCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryTransformCore: unsafe extern "system" fn(this: *mut *mut Self, inpoint: super::super::super::Foundation::Point, outpoint: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTransformCore: usize,
    #[cfg(feature = "Foundation")]
    pub TransformBoundsCore: unsafe extern "system" fn(this: *mut *mut Self, rect: super::super::super::Foundation::Rect, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransformBoundsCore: usize,
}
#[repr(C)]
pub struct IGeometry {
    pub base__: ::windows_sys::core::IInspectable,
    pub Transform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
}
#[repr(C)]
pub struct IGeometryFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGeometryGroup {
    pub base__: ::windows_sys::core::IInspectable,
    pub FillRule: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FillRule) -> ::windows_sys::core::HRESULT,
    pub SetFillRule: unsafe extern "system" fn(this: *mut *mut Self, value: FillRule) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetChildren: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetChildren: usize,
}
#[repr(C)]
pub struct IGeometryGroupStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FillRuleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ChildrenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeometryStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Empty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StandardFlatteningTolerance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub TransformProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGradientBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub SpreadMethod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GradientSpreadMethod) -> ::windows_sys::core::HRESULT,
    pub SetSpreadMethod: unsafe extern "system" fn(this: *mut *mut Self, value: GradientSpreadMethod) -> ::windows_sys::core::HRESULT,
    pub MappingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BrushMappingMode) -> ::windows_sys::core::HRESULT,
    pub SetMappingMode: unsafe extern "system" fn(this: *mut *mut Self, value: BrushMappingMode) -> ::windows_sys::core::HRESULT,
    pub ColorInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ColorInterpolationMode) -> ::windows_sys::core::HRESULT,
    pub SetColorInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, value: ColorInterpolationMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GradientStops: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GradientStops: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetGradientStops: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetGradientStops: usize,
}
#[repr(C)]
pub struct IGradientBrushFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGradientBrushStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SpreadMethodProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MappingModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ColorInterpolationModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GradientStopsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGradientStop {
    pub base__: ::windows_sys::core::IInspectable,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Color) -> ::windows_sys::core::HRESULT,
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGradientStopStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImageBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub ImageSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetImageSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ImageFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveImageFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveImageFailed: usize,
    #[cfg(feature = "Foundation")]
    pub ImageOpened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveImageOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveImageOpened: usize,
}
#[repr(C)]
pub struct IImageBrushStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ImageSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImageSource {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IImageSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ILineGeometry {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartPoint: usize,
    #[cfg(feature = "Foundation")]
    pub EndPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndPoint: usize,
}
#[repr(C)]
pub struct ILineGeometryStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub StartPointProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EndPointProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILineSegment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Point: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Point: usize,
    #[cfg(feature = "Foundation")]
    pub SetPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPoint: usize,
}
#[repr(C)]
pub struct ILineSegmentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PointProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILinearGradientBrush {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartPoint: usize,
    #[cfg(feature = "Foundation")]
    pub EndPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndPoint: usize,
}
#[repr(C)]
pub struct ILinearGradientBrushFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstanceWithGradientStopCollectionAndAngle: unsafe extern "system" fn(this: *mut *mut Self, gradientstopcollection: *mut ::core::ffi::c_void, angle: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstanceWithGradientStopCollectionAndAngle: usize,
}
#[repr(C)]
pub struct ILinearGradientBrushStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub StartPointProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EndPointProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILoadedImageSourceLoadCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LoadedImageSourceLoadStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILoadedImageSurface {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DecodedPhysicalSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DecodedPhysicalSize: usize,
    #[cfg(feature = "Foundation")]
    pub DecodedSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DecodedSize: usize,
    #[cfg(feature = "Foundation")]
    pub NaturalSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NaturalSize: usize,
    #[cfg(feature = "Foundation")]
    pub LoadCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLoadCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLoadCompleted: usize,
}
#[repr(C)]
pub struct ILoadedImageSurfaceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartLoadFromUriWithSize: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, desiredmaxsize: super::super::super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartLoadFromUriWithSize: usize,
    #[cfg(feature = "Foundation")]
    pub StartLoadFromUri: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartLoadFromUri: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub StartLoadFromStreamWithSize: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, desiredmaxsize: super::super::super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    StartLoadFromStreamWithSize: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StartLoadFromStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StartLoadFromStream: usize,
}
#[repr(C)]
pub struct IMatrix3DProjection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub ProjectionMatrix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Media3D::Matrix3D) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Media3D"))]
    ProjectionMatrix: usize,
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub SetProjectionMatrix: unsafe extern "system" fn(this: *mut *mut Self, value: Media3D::Matrix3D) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Media3D"))]
    SetProjectionMatrix: usize,
}
#[repr(C)]
pub struct IMatrix3DProjectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProjectionMatrixProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMatrixHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMatrixHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Identity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Matrix) -> ::windows_sys::core::HRESULT,
    pub FromElements: unsafe extern "system" fn(this: *mut *mut Self, m11: f64, m12: f64, m21: f64, m22: f64, offsetx: f64, offsety: f64, result__: *mut Matrix) -> ::windows_sys::core::HRESULT,
    pub GetIsIdentity: unsafe extern "system" fn(this: *mut *mut Self, target: Matrix, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Transform: unsafe extern "system" fn(this: *mut *mut Self, target: Matrix, point: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Transform: usize,
}
#[repr(C)]
pub struct IMatrixTransform {
    pub base__: ::windows_sys::core::IInspectable,
    pub Matrix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Matrix) -> ::windows_sys::core::HRESULT,
    pub SetMatrix: unsafe extern "system" fn(this: *mut *mut Self, value: Matrix) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMatrixTransformStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MatrixProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaTransportControlsThumbnailRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnailImage: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnailImage: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IPartialMediaFailureDetectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Playback")]
    pub StreamKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Media::Playback::FailedMediaStreamKind) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    StreamKind: usize,
}
#[repr(C)]
pub struct IPartialMediaFailureDetectedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPathFigure {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Segments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Segments: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetSegments: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetSegments: usize,
    #[cfg(feature = "Foundation")]
    pub StartPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartPoint: usize,
    pub IsClosed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsClosed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsFilled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFilled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPathFigureStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SegmentsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartPointProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsClosedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsFilledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPathGeometry {
    pub base__: ::windows_sys::core::IInspectable,
    pub FillRule: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FillRule) -> ::windows_sys::core::HRESULT,
    pub SetFillRule: unsafe extern "system" fn(this: *mut *mut Self, value: FillRule) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Figures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Figures: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetFigures: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetFigures: usize,
}
#[repr(C)]
pub struct IPathGeometryStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FillRuleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FiguresProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPathSegment {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPathSegmentFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPlaneProjection {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalOffsetX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLocalOffsetX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub LocalOffsetY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLocalOffsetY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub LocalOffsetZ: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLocalOffsetZ: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RotationX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRotationX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RotationY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRotationY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RotationZ: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRotationZ: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub CenterOfRotationX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterOfRotationX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub CenterOfRotationY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterOfRotationY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub CenterOfRotationZ: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterOfRotationZ: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub GlobalOffsetX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetGlobalOffsetX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub GlobalOffsetY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetGlobalOffsetY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub GlobalOffsetZ: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetGlobalOffsetZ: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub ProjectionMatrix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Media3D::Matrix3D) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Media3D"))]
    ProjectionMatrix: usize,
}
#[repr(C)]
pub struct IPlaneProjectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalOffsetXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LocalOffsetYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LocalOffsetZProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RotationXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RotationYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RotationZProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CenterOfRotationXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CenterOfRotationYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CenterOfRotationZProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GlobalOffsetXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GlobalOffsetYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GlobalOffsetZProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProjectionMatrixProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPolyBezierSegment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Points: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Points: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPoints: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPoints: usize,
}
#[repr(C)]
pub struct IPolyBezierSegmentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PointsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPolyLineSegment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Points: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Points: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPoints: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPoints: usize,
}
#[repr(C)]
pub struct IPolyLineSegmentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PointsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPolyQuadraticBezierSegment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Points: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Points: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPoints: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPoints: usize,
}
#[repr(C)]
pub struct IPolyQuadraticBezierSegmentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PointsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProjection {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IProjectionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IQuadraticBezierSegment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Point1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Point1: usize,
    #[cfg(feature = "Foundation")]
    pub SetPoint1: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPoint1: usize,
    #[cfg(feature = "Foundation")]
    pub Point2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Point2: usize,
    #[cfg(feature = "Foundation")]
    pub SetPoint2: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPoint2: usize,
}
#[repr(C)]
pub struct IQuadraticBezierSegmentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Point1Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Point2Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRateChangedRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRectangleGeometry {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Rect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Rect: usize,
    #[cfg(feature = "Foundation")]
    pub SetRect: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRect: usize,
}
#[repr(C)]
pub struct IRectangleGeometryStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RectProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRenderedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FrameDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameDuration: usize,
}
#[repr(C)]
pub struct IRenderingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RenderingTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenderingTime: usize,
}
#[repr(C)]
pub struct IRevealBackgroundBrush {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRevealBackgroundBrushFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRevealBorderBrush {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRevealBorderBrushFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRevealBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Color) -> ::windows_sys::core::HRESULT,
    pub TargetTheme: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::ApplicationTheme) -> ::windows_sys::core::HRESULT,
    pub SetTargetTheme: unsafe extern "system" fn(this: *mut *mut Self, value: super::ApplicationTheme) -> ::windows_sys::core::HRESULT,
    pub AlwaysUseFallback: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlwaysUseFallback: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRevealBrushFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRevealBrushStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TargetThemeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AlwaysUseFallbackProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: RevealBrushState) -> ::windows_sys::core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut RevealBrushState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRotateTransform {
    pub base__: ::windows_sys::core::IInspectable,
    pub CenterX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub CenterY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Angle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetAngle: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRotateTransformStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CenterXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AngleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScaleTransform {
    pub base__: ::windows_sys::core::IInspectable,
    pub CenterX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub CenterY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ScaleX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetScaleX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ScaleY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScaleTransformStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CenterXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ScaleXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ScaleYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IShadow {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IShadowFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISkewTransform {
    pub base__: ::windows_sys::core::IInspectable,
    pub CenterX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub CenterY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub AngleX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetAngleX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub AngleY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetAngleY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISkewTransformStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CenterXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AngleXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AngleYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISolidColorBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Color) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISolidColorBrushFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithColor: unsafe extern "system" fn(this: *mut *mut Self, color: super::super::Color, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISolidColorBrushStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IThemeShadow {
    pub base__: ::windows_sys::core::IInspectable,
    pub Receivers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IThemeShadowFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITileBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlignmentX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AlignmentX) -> ::windows_sys::core::HRESULT,
    pub SetAlignmentX: unsafe extern "system" fn(this: *mut *mut Self, value: AlignmentX) -> ::windows_sys::core::HRESULT,
    pub AlignmentY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AlignmentY) -> ::windows_sys::core::HRESULT,
    pub SetAlignmentY: unsafe extern "system" fn(this: *mut *mut Self, value: AlignmentY) -> ::windows_sys::core::HRESULT,
    pub Stretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Stretch) -> ::windows_sys::core::HRESULT,
    pub SetStretch: unsafe extern "system" fn(this: *mut *mut Self, value: Stretch) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITileBrushFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITileBrushStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlignmentXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AlignmentYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimelineMarker {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Time: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Time: usize,
    #[cfg(feature = "Foundation")]
    pub SetTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTime: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimelineMarkerRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Marker: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMarker: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimelineMarkerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TimeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITransform {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITransformFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITransformGroup {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetChildren: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetChildren: usize,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Matrix) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITransformGroupStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChildrenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITranslateTransform {
    pub base__: ::windows_sys::core::IInspectable,
    pub X: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Y: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITranslateTransformStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub XProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub YProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisualTreeHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IVisualTreeHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindElementsInHostCoordinatesPoint: unsafe extern "system" fn(this: *mut *mut Self, intersectingpoint: super::super::super::Foundation::Point, subtree: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindElementsInHostCoordinatesPoint: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindElementsInHostCoordinatesRect: unsafe extern "system" fn(this: *mut *mut Self, intersectingrect: super::super::super::Foundation::Rect, subtree: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindElementsInHostCoordinatesRect: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllElementsInHostCoordinatesPoint: unsafe extern "system" fn(this: *mut *mut Self, intersectingpoint: super::super::super::Foundation::Point, subtree: *mut ::core::ffi::c_void, includeallelements: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllElementsInHostCoordinatesPoint: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllElementsInHostCoordinatesRect: unsafe extern "system" fn(this: *mut *mut Self, intersectingrect: super::super::super::Foundation::Rect, subtree: *mut ::core::ffi::c_void, includeallelements: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllElementsInHostCoordinatesRect: usize,
    pub GetChild: unsafe extern "system" fn(this: *mut *mut Self, reference: *mut ::core::ffi::c_void, childindex: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetChildrenCount: unsafe extern "system" fn(this: *mut *mut Self, reference: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut *mut Self, reference: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisconnectChildrenRecursive: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisualTreeHelperStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives"))]
    pub GetOpenPopups: unsafe extern "system" fn(this: *mut *mut Self, window: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives")))]
    GetOpenPopups: usize,
}
#[repr(C)]
pub struct IVisualTreeHelperStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives"))]
    pub GetOpenPopupsForXamlRoot: unsafe extern "system" fn(this: *mut *mut Self, xamlroot: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls_Primitives")))]
    GetOpenPopupsForXamlRoot: usize,
}
#[repr(C)]
pub struct IXamlCompositionBrushBase {
    pub base__: ::windows_sys::core::IInspectable,
    pub FallbackColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetFallbackColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Color) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlCompositionBrushBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlCompositionBrushBaseOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnConnected: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnDisconnected: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlCompositionBrushBaseProtected {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub CompositionBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CompositionBrush: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetCompositionBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetCompositionBrush: usize,
}
#[repr(C)]
pub struct IXamlCompositionBrushBaseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FallbackColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlLight {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IXamlLightFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlLightOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OnConnected: unsafe extern "system" fn(this: *mut *mut Self, newelement: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnDisconnected: unsafe extern "system" fn(this: *mut *mut Self, oldelement: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlLightProtected {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub CompositionLight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CompositionLight: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetCompositionLight: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetCompositionLight: usize,
}
#[repr(C)]
pub struct IXamlLightStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddTargetElement: unsafe extern "system" fn(this: *mut *mut Self, lightid: ::windows_sys::core::HSTRING, element: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveTargetElement: unsafe extern "system" fn(this: *mut *mut Self, lightid: ::windows_sys::core::HSTRING, element: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddTargetBrush: unsafe extern "system" fn(this: *mut *mut Self, lightid: ::windows_sys::core::HSTRING, brush: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveTargetBrush: unsafe extern "system" fn(this: *mut *mut Self, lightid: ::windows_sys::core::HSTRING, brush: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type ImageBrush = *mut ::core::ffi::c_void;
pub type ImageSource = *mut ::core::ffi::c_void;
pub type LineGeometry = *mut ::core::ffi::c_void;
pub type LineSegment = *mut ::core::ffi::c_void;
pub type LinearGradientBrush = *mut ::core::ffi::c_void;
pub type LoadedImageSourceLoadCompletedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct LoadedImageSourceLoadStatus(pub i32);
impl LoadedImageSourceLoadStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const InvalidFormat: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for LoadedImageSourceLoadStatus {}
impl ::core::clone::Clone for LoadedImageSourceLoadStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LoadedImageSurface = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
pub struct Matrix {
    pub M11: f64,
    pub M12: f64,
    pub M21: f64,
    pub M22: f64,
    pub OffsetX: f64,
    pub OffsetY: f64,
}
impl ::core::marker::Copy for Matrix {}
impl ::core::clone::Clone for Matrix {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Matrix3DProjection = *mut ::core::ffi::c_void;
pub type MatrixHelper = *mut ::core::ffi::c_void;
pub type MatrixTransform = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct MediaCanPlayResponse(pub i32);
impl MediaCanPlayResponse {
    pub const NotSupported: Self = Self(0i32);
    pub const Maybe: Self = Self(1i32);
    pub const Probably: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaCanPlayResponse {}
impl ::core::clone::Clone for MediaCanPlayResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct MediaElementState(pub i32);
impl MediaElementState {
    pub const Closed: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
    pub const Stopped: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaElementState {}
impl ::core::clone::Clone for MediaElementState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaTransportControlsThumbnailRequestedEventArgs = *mut ::core::ffi::c_void;
pub type PartialMediaFailureDetectedEventArgs = *mut ::core::ffi::c_void;
pub type PathFigure = *mut ::core::ffi::c_void;
pub type PathFigureCollection = *mut ::core::ffi::c_void;
pub type PathGeometry = *mut ::core::ffi::c_void;
pub type PathSegment = *mut ::core::ffi::c_void;
pub type PathSegmentCollection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct PenLineCap(pub i32);
impl PenLineCap {
    pub const Flat: Self = Self(0i32);
    pub const Square: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const Triangle: Self = Self(3i32);
}
impl ::core::marker::Copy for PenLineCap {}
impl ::core::clone::Clone for PenLineCap {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct PenLineJoin(pub i32);
impl PenLineJoin {
    pub const Miter: Self = Self(0i32);
    pub const Bevel: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
}
impl ::core::marker::Copy for PenLineJoin {}
impl ::core::clone::Clone for PenLineJoin {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PlaneProjection = *mut ::core::ffi::c_void;
pub type PointCollection = *mut ::core::ffi::c_void;
pub type PolyBezierSegment = *mut ::core::ffi::c_void;
pub type PolyLineSegment = *mut ::core::ffi::c_void;
pub type PolyQuadraticBezierSegment = *mut ::core::ffi::c_void;
pub type Projection = *mut ::core::ffi::c_void;
pub type QuadraticBezierSegment = *mut ::core::ffi::c_void;
pub type RateChangedRoutedEventArgs = *mut ::core::ffi::c_void;
pub type RateChangedRoutedEventHandler = *mut ::core::ffi::c_void;
pub type RectangleGeometry = *mut ::core::ffi::c_void;
pub type RenderedEventArgs = *mut ::core::ffi::c_void;
pub type RenderingEventArgs = *mut ::core::ffi::c_void;
pub type RevealBackgroundBrush = *mut ::core::ffi::c_void;
pub type RevealBorderBrush = *mut ::core::ffi::c_void;
pub type RevealBrush = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct RevealBrushState(pub i32);
impl RevealBrushState {
    pub const Normal: Self = Self(0i32);
    pub const PointerOver: Self = Self(1i32);
    pub const Pressed: Self = Self(2i32);
}
impl ::core::marker::Copy for RevealBrushState {}
impl ::core::clone::Clone for RevealBrushState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RotateTransform = *mut ::core::ffi::c_void;
pub type ScaleTransform = *mut ::core::ffi::c_void;
pub type Shadow = *mut ::core::ffi::c_void;
pub type SkewTransform = *mut ::core::ffi::c_void;
pub type SolidColorBrush = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct Stereo3DVideoPackingMode(pub i32);
impl Stereo3DVideoPackingMode {
    pub const None: Self = Self(0i32);
    pub const SideBySide: Self = Self(1i32);
    pub const TopBottom: Self = Self(2i32);
}
impl ::core::marker::Copy for Stereo3DVideoPackingMode {}
impl ::core::clone::Clone for Stereo3DVideoPackingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct Stereo3DVideoRenderMode(pub i32);
impl Stereo3DVideoRenderMode {
    pub const Mono: Self = Self(0i32);
    pub const Stereo: Self = Self(1i32);
}
impl ::core::marker::Copy for Stereo3DVideoRenderMode {}
impl ::core::clone::Clone for Stereo3DVideoRenderMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct Stretch(pub i32);
impl Stretch {
    pub const None: Self = Self(0i32);
    pub const Fill: Self = Self(1i32);
    pub const Uniform: Self = Self(2i32);
    pub const UniformToFill: Self = Self(3i32);
}
impl ::core::marker::Copy for Stretch {}
impl ::core::clone::Clone for Stretch {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct StyleSimulations(pub i32);
impl StyleSimulations {
    pub const None: Self = Self(0i32);
    pub const BoldSimulation: Self = Self(1i32);
    pub const ItalicSimulation: Self = Self(2i32);
    pub const BoldItalicSimulation: Self = Self(3i32);
}
impl ::core::marker::Copy for StyleSimulations {}
impl ::core::clone::Clone for StyleSimulations {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
#[repr(transparent)]
pub struct SweepDirection(pub i32);
impl SweepDirection {
    pub const Counterclockwise: Self = Self(0i32);
    pub const Clockwise: Self = Self(1i32);
}
impl ::core::marker::Copy for SweepDirection {}
impl ::core::clone::Clone for SweepDirection {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ThemeShadow = *mut ::core::ffi::c_void;
pub type TileBrush = *mut ::core::ffi::c_void;
pub type TimelineMarker = *mut ::core::ffi::c_void;
pub type TimelineMarkerCollection = *mut ::core::ffi::c_void;
pub type TimelineMarkerRoutedEventArgs = *mut ::core::ffi::c_void;
pub type TimelineMarkerRoutedEventHandler = *mut ::core::ffi::c_void;
pub type Transform = *mut ::core::ffi::c_void;
pub type TransformCollection = *mut ::core::ffi::c_void;
pub type TransformGroup = *mut ::core::ffi::c_void;
pub type TranslateTransform = *mut ::core::ffi::c_void;
pub type VisualTreeHelper = *mut ::core::ffi::c_void;
pub type XamlCompositionBrushBase = *mut ::core::ffi::c_void;
pub type XamlLight = *mut ::core::ffi::c_void;
