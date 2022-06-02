#[cfg(feature = "UI_Composition_Core")]
pub mod Core;
#[cfg(feature = "UI_Composition_Desktop")]
pub mod Desktop;
#[cfg(feature = "UI_Composition_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "UI_Composition_Effects")]
pub mod Effects;
#[cfg(feature = "UI_Composition_Interactions")]
pub mod Interactions;
#[cfg(feature = "UI_Composition_Scenes")]
pub mod Scenes;
pub type AmbientLight = *mut ::core::ffi::c_void;
pub type AnimationController = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct AnimationControllerProgressBehavior(pub i32);
impl AnimationControllerProgressBehavior {
    pub const Default: Self = Self(0i32);
    pub const IncludesDelayTime: Self = Self(1i32);
}
impl ::core::marker::Copy for AnimationControllerProgressBehavior {}
impl ::core::clone::Clone for AnimationControllerProgressBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct AnimationDelayBehavior(pub i32);
impl AnimationDelayBehavior {
    pub const SetInitialValueAfterDelay: Self = Self(0i32);
    pub const SetInitialValueBeforeDelay: Self = Self(1i32);
}
impl ::core::marker::Copy for AnimationDelayBehavior {}
impl ::core::clone::Clone for AnimationDelayBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct AnimationDirection(pub i32);
impl AnimationDirection {
    pub const Normal: Self = Self(0i32);
    pub const Reverse: Self = Self(1i32);
    pub const Alternate: Self = Self(2i32);
    pub const AlternateReverse: Self = Self(3i32);
}
impl ::core::marker::Copy for AnimationDirection {}
impl ::core::clone::Clone for AnimationDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct AnimationIterationBehavior(pub i32);
impl AnimationIterationBehavior {
    pub const Count: Self = Self(0i32);
    pub const Forever: Self = Self(1i32);
}
impl ::core::marker::Copy for AnimationIterationBehavior {}
impl ::core::clone::Clone for AnimationIterationBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct AnimationPropertyAccessMode(pub i32);
impl AnimationPropertyAccessMode {
    pub const None: Self = Self(0i32);
    pub const ReadOnly: Self = Self(1i32);
    pub const WriteOnly: Self = Self(2i32);
    pub const ReadWrite: Self = Self(3i32);
}
impl ::core::marker::Copy for AnimationPropertyAccessMode {}
impl ::core::clone::Clone for AnimationPropertyAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AnimationPropertyInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct AnimationStopBehavior(pub i32);
impl AnimationStopBehavior {
    pub const LeaveCurrentValue: Self = Self(0i32);
    pub const SetToInitialValue: Self = Self(1i32);
    pub const SetToFinalValue: Self = Self(2i32);
}
impl ::core::marker::Copy for AnimationStopBehavior {}
impl ::core::clone::Clone for AnimationStopBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BackEasingFunction = *mut ::core::ffi::c_void;
pub type BooleanKeyFrameAnimation = *mut ::core::ffi::c_void;
pub type BounceEasingFunction = *mut ::core::ffi::c_void;
pub type BounceScalarNaturalMotionAnimation = *mut ::core::ffi::c_void;
pub type BounceVector2NaturalMotionAnimation = *mut ::core::ffi::c_void;
pub type BounceVector3NaturalMotionAnimation = *mut ::core::ffi::c_void;
pub type CircleEasingFunction = *mut ::core::ffi::c_void;
pub type ColorKeyFrameAnimation = *mut ::core::ffi::c_void;
pub type CompositionAnimation = *mut ::core::ffi::c_void;
pub type CompositionAnimationGroup = *mut ::core::ffi::c_void;
pub type CompositionBackdropBrush = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionBackfaceVisibility(pub i32);
impl CompositionBackfaceVisibility {
    pub const Inherit: Self = Self(0i32);
    pub const Visible: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
}
impl ::core::marker::Copy for CompositionBackfaceVisibility {}
impl ::core::clone::Clone for CompositionBackfaceVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositionBatchCompletedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionBatchTypes(pub u32);
impl CompositionBatchTypes {
    pub const None: Self = Self(0u32);
    pub const Animation: Self = Self(1u32);
    pub const Effect: Self = Self(2u32);
    pub const InfiniteAnimation: Self = Self(4u32);
    pub const AllAnimations: Self = Self(5u32);
}
impl ::core::marker::Copy for CompositionBatchTypes {}
impl ::core::clone::Clone for CompositionBatchTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionBitmapInterpolationMode(pub i32);
impl CompositionBitmapInterpolationMode {
    pub const NearestNeighbor: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const MagLinearMinLinearMipLinear: Self = Self(2i32);
    pub const MagLinearMinLinearMipNearest: Self = Self(3i32);
    pub const MagLinearMinNearestMipLinear: Self = Self(4i32);
    pub const MagLinearMinNearestMipNearest: Self = Self(5i32);
    pub const MagNearestMinLinearMipLinear: Self = Self(6i32);
    pub const MagNearestMinLinearMipNearest: Self = Self(7i32);
    pub const MagNearestMinNearestMipLinear: Self = Self(8i32);
    pub const MagNearestMinNearestMipNearest: Self = Self(9i32);
}
impl ::core::marker::Copy for CompositionBitmapInterpolationMode {}
impl ::core::clone::Clone for CompositionBitmapInterpolationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionBorderMode(pub i32);
impl CompositionBorderMode {
    pub const Inherit: Self = Self(0i32);
    pub const Soft: Self = Self(1i32);
    pub const Hard: Self = Self(2i32);
}
impl ::core::marker::Copy for CompositionBorderMode {}
impl ::core::clone::Clone for CompositionBorderMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositionBrush = *mut ::core::ffi::c_void;
pub type CompositionCapabilities = *mut ::core::ffi::c_void;
pub type CompositionClip = *mut ::core::ffi::c_void;
pub type CompositionColorBrush = *mut ::core::ffi::c_void;
pub type CompositionColorGradientStop = *mut ::core::ffi::c_void;
pub type CompositionColorGradientStopCollection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionColorSpace(pub i32);
impl CompositionColorSpace {
    pub const Auto: Self = Self(0i32);
    pub const Hsl: Self = Self(1i32);
    pub const Rgb: Self = Self(2i32);
    pub const HslLinear: Self = Self(3i32);
    pub const RgbLinear: Self = Self(4i32);
}
impl ::core::marker::Copy for CompositionColorSpace {}
impl ::core::clone::Clone for CompositionColorSpace {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositionCommitBatch = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionCompositeMode(pub i32);
impl CompositionCompositeMode {
    pub const Inherit: Self = Self(0i32);
    pub const SourceOver: Self = Self(1i32);
    pub const DestinationInvert: Self = Self(2i32);
    pub const MinBlend: Self = Self(3i32);
}
impl ::core::marker::Copy for CompositionCompositeMode {}
impl ::core::clone::Clone for CompositionCompositeMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositionContainerShape = *mut ::core::ffi::c_void;
pub type CompositionDrawingSurface = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionDropShadowSourcePolicy(pub i32);
impl CompositionDropShadowSourcePolicy {
    pub const Default: Self = Self(0i32);
    pub const InheritFromVisualContent: Self = Self(1i32);
}
impl ::core::marker::Copy for CompositionDropShadowSourcePolicy {}
impl ::core::clone::Clone for CompositionDropShadowSourcePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositionEasingFunction = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionEasingFunctionMode(pub i32);
impl CompositionEasingFunctionMode {
    pub const In: Self = Self(0i32);
    pub const Out: Self = Self(1i32);
    pub const InOut: Self = Self(2i32);
}
impl ::core::marker::Copy for CompositionEasingFunctionMode {}
impl ::core::clone::Clone for CompositionEasingFunctionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositionEffectBrush = *mut ::core::ffi::c_void;
pub type CompositionEffectFactory = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionEffectFactoryLoadStatus(pub i32);
impl CompositionEffectFactoryLoadStatus {
    pub const Success: Self = Self(0i32);
    pub const EffectTooComplex: Self = Self(1i32);
    pub const Pending: Self = Self(2i32);
    pub const Other: Self = Self(-1i32);
}
impl ::core::marker::Copy for CompositionEffectFactoryLoadStatus {}
impl ::core::clone::Clone for CompositionEffectFactoryLoadStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositionEffectSourceParameter = *mut ::core::ffi::c_void;
pub type CompositionEllipseGeometry = *mut ::core::ffi::c_void;
pub type CompositionGeometricClip = *mut ::core::ffi::c_void;
pub type CompositionGeometry = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionGetValueStatus(pub i32);
impl CompositionGetValueStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const TypeMismatch: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for CompositionGetValueStatus {}
impl ::core::clone::Clone for CompositionGetValueStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositionGradientBrush = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionGradientExtendMode(pub i32);
impl CompositionGradientExtendMode {
    pub const Clamp: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
    pub const Mirror: Self = Self(2i32);
}
impl ::core::marker::Copy for CompositionGradientExtendMode {}
impl ::core::clone::Clone for CompositionGradientExtendMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositionGraphicsDevice = *mut ::core::ffi::c_void;
pub type CompositionLight = *mut ::core::ffi::c_void;
pub type CompositionLineGeometry = *mut ::core::ffi::c_void;
pub type CompositionLinearGradientBrush = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionMappingMode(pub i32);
impl CompositionMappingMode {
    pub const Absolute: Self = Self(0i32);
    pub const Relative: Self = Self(1i32);
}
impl ::core::marker::Copy for CompositionMappingMode {}
impl ::core::clone::Clone for CompositionMappingMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositionMaskBrush = *mut ::core::ffi::c_void;
pub type CompositionMipmapSurface = *mut ::core::ffi::c_void;
pub type CompositionNineGridBrush = *mut ::core::ffi::c_void;
pub type CompositionObject = *mut ::core::ffi::c_void;
pub type CompositionPath = *mut ::core::ffi::c_void;
pub type CompositionPathGeometry = *mut ::core::ffi::c_void;
pub type CompositionProjectedShadow = *mut ::core::ffi::c_void;
pub type CompositionProjectedShadowCaster = *mut ::core::ffi::c_void;
pub type CompositionProjectedShadowCasterCollection = *mut ::core::ffi::c_void;
pub type CompositionProjectedShadowReceiver = *mut ::core::ffi::c_void;
pub type CompositionProjectedShadowReceiverUnorderedCollection = *mut ::core::ffi::c_void;
pub type CompositionPropertySet = *mut ::core::ffi::c_void;
pub type CompositionRadialGradientBrush = *mut ::core::ffi::c_void;
pub type CompositionRectangleGeometry = *mut ::core::ffi::c_void;
pub type CompositionRoundedRectangleGeometry = *mut ::core::ffi::c_void;
pub type CompositionScopedBatch = *mut ::core::ffi::c_void;
pub type CompositionShadow = *mut ::core::ffi::c_void;
pub type CompositionShape = *mut ::core::ffi::c_void;
pub type CompositionShapeCollection = *mut ::core::ffi::c_void;
pub type CompositionSpriteShape = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionStretch(pub i32);
impl CompositionStretch {
    pub const None: Self = Self(0i32);
    pub const Fill: Self = Self(1i32);
    pub const Uniform: Self = Self(2i32);
    pub const UniformToFill: Self = Self(3i32);
}
impl ::core::marker::Copy for CompositionStretch {}
impl ::core::clone::Clone for CompositionStretch {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionStrokeCap(pub i32);
impl CompositionStrokeCap {
    pub const Flat: Self = Self(0i32);
    pub const Square: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const Triangle: Self = Self(3i32);
}
impl ::core::marker::Copy for CompositionStrokeCap {}
impl ::core::clone::Clone for CompositionStrokeCap {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositionStrokeDashArray = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition\"`*"]
#[repr(transparent)]
pub struct CompositionStrokeLineJoin(pub i32);
impl CompositionStrokeLineJoin {
    pub const Miter: Self = Self(0i32);
    pub const Bevel: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const MiterOrBevel: Self = Self(3i32);
}
impl ::core::marker::Copy for CompositionStrokeLineJoin {}
impl ::core::clone::Clone for CompositionStrokeLineJoin {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositionSurfaceBrush = *mut ::core::ffi::c_void;
pub type CompositionTarget = *mut ::core::ffi::c_void;
pub type CompositionTransform = *mut ::core::ffi::c_void;
pub type CompositionViewBox = *mut ::core::ffi::c_void;
pub type CompositionVirtualDrawingSurface = *mut ::core::ffi::c_void;
pub type CompositionVisualSurface = *mut ::core::ffi::c_void;
pub type Compositor = *mut ::core::ffi::c_void;
pub type ContainerVisual = *mut ::core::ffi::c_void;
pub type CubicBezierEasingFunction = *mut ::core::ffi::c_void;
pub type DelegatedInkTrailVisual = *mut ::core::ffi::c_void;
pub type DistantLight = *mut ::core::ffi::c_void;
pub type DropShadow = *mut ::core::ffi::c_void;
pub type ElasticEasingFunction = *mut ::core::ffi::c_void;
pub type ExponentialEasingFunction = *mut ::core::ffi::c_void;
pub type ExpressionAnimation = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAmbientLight {
    pub base__: ::windows_sys::core::IInspectable,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAmbientLight2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Intensity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetIntensity: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAnimationController {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetProgress: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub ProgressBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AnimationControllerProgressBehavior) -> ::windows_sys::core::HRESULT,
    pub SetProgressBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: AnimationControllerProgressBehavior) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAnimationControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub MinPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAnimationObject {
    pub base__: ::windows_sys::core::IInspectable,
    pub PopulatePropertyInfo: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, propertyinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAnimationPropertyInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub AccessMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AnimationPropertyAccessMode) -> ::windows_sys::core::HRESULT,
    pub SetAccessMode: unsafe extern "system" fn(this: *mut *mut Self, value: AnimationPropertyAccessMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAnimationPropertyInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetResolvedCompositionObject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetResolvedCompositionObjectProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
    pub Amplitude: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBooleanKeyFrameAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub InsertKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBounceEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
    pub Bounces: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Bounciness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBounceScalarNaturalMotionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Acceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetAcceleration: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Restitution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRestitution: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBounceVector2NaturalMotionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Acceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetAcceleration: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Restitution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRestitution: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBounceVector3NaturalMotionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Acceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetAcceleration: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Restitution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRestitution: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICircleEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorKeyFrameAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub InterpolationColorSpace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionColorSpace) -> ::windows_sys::core::HRESULT,
    pub SetInterpolationColorSpace: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionColorSpace) -> ::windows_sys::core::HRESULT,
    pub InsertKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: super::Color) -> ::windows_sys::core::HRESULT,
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: super::Color, easingfunction: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClearAllParameters: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ClearParameter: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetColorParameter: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, value: super::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix3x2Parameter: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix3x2Parameter: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix4x4Parameter: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix4x4Parameter: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetQuaternionParameter: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, value: super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetQuaternionParameter: usize,
    pub SetReferenceParameter: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, compositionobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetScalarParameter: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetVector2Parameter: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetVector2Parameter: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetVector3Parameter: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetVector3Parameter: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetVector4Parameter: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, value: super::super::Foundation::Numerics::Vector4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetVector4Parameter: usize,
}
#[repr(C)]
pub struct ICompositionAnimation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetBooleanParameter: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, value: bool) -> ::windows_sys::core::HRESULT,
    pub Target: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTarget: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionAnimation3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub InitialValueExpressions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InitialValueExpressions: usize,
}
#[repr(C)]
pub struct ICompositionAnimation4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetExpressionReferenceParameter: unsafe extern "system" fn(this: *mut *mut Self, parametername: ::windows_sys::core::HSTRING, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionAnimationBase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionAnimationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionAnimationGroup {
    pub base__: ::windows_sys::core::IInspectable,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionBackdropBrush {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionBatchCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionBrush {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionBrushFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreEffectsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AreEffectsFast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[repr(C)]
pub struct ICompositionCapabilitiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionClip {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionClip2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub AnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AnchorPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetAnchorPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CenterPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CenterPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetCenterPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetCenterPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Offset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOffset: usize,
    pub RotationAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub RotationAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRotationAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Scale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetScale: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TransformMatrix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TransformMatrix: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformMatrix: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformMatrix: usize,
}
#[repr(C)]
pub struct ICompositionClipFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionColorBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionColorGradientStop {
    pub base__: ::windows_sys::core::IInspectable,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionColorGradientStopCollection {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionCommitBatch {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsEnded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
}
#[repr(C)]
pub struct ICompositionContainerShape {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Shapes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Shapes: usize,
}
#[repr(C)]
pub struct ICompositionDrawingSurface {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX")]
    pub AlphaMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    AlphaMode: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub PixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PixelFormat: usize,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
}
#[repr(C)]
pub struct ICompositionDrawingSurface2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics")]
    pub SizeInt32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::SizeInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SizeInt32: usize,
    #[cfg(feature = "Graphics")]
    pub Resize: unsafe extern "system" fn(this: *mut *mut Self, sizepixels: super::super::Graphics::SizeInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Resize: usize,
    #[cfg(feature = "Graphics")]
    pub Scroll: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::Graphics::PointInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Scroll: usize,
    #[cfg(feature = "Graphics")]
    pub ScrollRect: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::Graphics::PointInt32, scrollrect: super::super::Graphics::RectInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    ScrollRect: usize,
    #[cfg(feature = "Graphics")]
    pub ScrollWithClip: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::Graphics::PointInt32, cliprect: super::super::Graphics::RectInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    ScrollWithClip: usize,
    #[cfg(feature = "Graphics")]
    pub ScrollRectWithClip: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::Graphics::PointInt32, cliprect: super::super::Graphics::RectInt32, scrollrect: super::super::Graphics::RectInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    ScrollRectWithClip: usize,
}
#[repr(C)]
pub struct ICompositionDrawingSurfaceFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionEasingFunctionFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionEasingFunctionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCubicBezierEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, controlpoint1: super::super::Foundation::Numerics::Vector2, controlpoint2: super::super::Foundation::Numerics::Vector2, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCubicBezierEasingFunction: usize,
    pub CreateLinearEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateStepEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateStepEasingFunctionWithStepCount: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, stepcount: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBackEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, mode: CompositionEasingFunctionMode, amplitude: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBounceEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, mode: CompositionEasingFunctionMode, bounces: i32, bounciness: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateCircleEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, mode: CompositionEasingFunctionMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateElasticEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, mode: CompositionEasingFunctionMode, oscillations: i32, springiness: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateExponentialEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, mode: CompositionEasingFunctionMode, exponent: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePowerEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, mode: CompositionEasingFunctionMode, power: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSineEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut ::core::ffi::c_void, mode: CompositionEasingFunctionMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionEffectBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetSourceParameter: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSourceParameter: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub LoadStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEffectFactoryLoadStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionEffectSourceParameter {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionEffectSourceParameterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionEllipseGeometry {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Center: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Center: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetCenter: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetCenter: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Radius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Radius: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRadius: usize,
}
#[repr(C)]
pub struct ICompositionGeometricClip {
    pub base__: ::windows_sys::core::IInspectable,
    pub Geometry: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGeometry: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ViewBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetViewBox: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionGeometry {
    pub base__: ::windows_sys::core::IInspectable,
    pub TrimEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetTrimEnd: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub TrimOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetTrimOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub TrimStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetTrimStart: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionGeometryFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionGradientBrush {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub AnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AnchorPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetAnchorPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CenterPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CenterPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetCenterPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetCenterPoint: usize,
    pub ColorStops: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExtendMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionGradientExtendMode) -> ::windows_sys::core::HRESULT,
    pub SetExtendMode: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionGradientExtendMode) -> ::windows_sys::core::HRESULT,
    pub InterpolationSpace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionColorSpace) -> ::windows_sys::core::HRESULT,
    pub SetInterpolationSpace: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionColorSpace) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Offset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOffset: usize,
    pub RotationAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub RotationAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRotationAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Scale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetScale: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TransformMatrix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TransformMatrix: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformMatrix: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformMatrix: usize,
}
#[repr(C)]
pub struct ICompositionGradientBrush2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MappingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionMappingMode) -> ::windows_sys::core::HRESULT,
    pub SetMappingMode: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionMappingMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionGradientBrushFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionGraphicsDevice {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))]
    pub CreateDrawingSurface: unsafe extern "system" fn(this: *mut *mut Self, sizepixels: super::super::Foundation::Size, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX")))]
    CreateDrawingSurface: usize,
    #[cfg(feature = "Foundation")]
    pub RenderingDeviceReplaced: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenderingDeviceReplaced: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRenderingDeviceReplaced: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRenderingDeviceReplaced: usize,
}
#[repr(C)]
pub struct ICompositionGraphicsDevice2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateDrawingSurface2: unsafe extern "system" fn(this: *mut *mut Self, sizepixels: super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateDrawingSurface2: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateVirtualDrawingSurface: unsafe extern "system" fn(this: *mut *mut Self, sizepixels: super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateVirtualDrawingSurface: usize,
}
#[repr(C)]
pub struct ICompositionGraphicsDevice3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateMipmapSurface: unsafe extern "system" fn(this: *mut *mut Self, sizepixels: super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateMipmapSurface: usize,
    pub Trim: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionGraphicsDevice4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))]
    pub CaptureAsync: unsafe extern "system" fn(this: *mut *mut Self, capturevisual: *mut ::core::ffi::c_void, size: super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, sdrboost: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX")))]
    CaptureAsync: usize,
}
#[repr(C)]
pub struct ICompositionLight {
    pub base__: ::windows_sys::core::IInspectable,
    pub Targets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionLight2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExclusionsFromTargets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionLight3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionLightFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionLineGeometry {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Start: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetStart: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetStart: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub End: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    End: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetEnd: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetEnd: usize,
}
#[repr(C)]
pub struct ICompositionLinearGradientBrush {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub EndPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    EndPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetEndPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetEndPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub StartPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    StartPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetStartPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetStartPoint: usize,
}
#[repr(C)]
pub struct ICompositionMaskBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mask: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMask: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionMipmapSurface {
    pub base__: ::windows_sys::core::IInspectable,
    pub LevelCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub AlphaMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    AlphaMode: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub PixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PixelFormat: usize,
    #[cfg(feature = "Graphics")]
    pub SizeInt32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::SizeInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SizeInt32: usize,
    pub GetDrawingSurfaceForLevel: unsafe extern "system" fn(this: *mut *mut Self, level: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionNineGridBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub BottomInset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetBottomInset: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub BottomInsetScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetBottomInsetScale: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub IsCenterHollow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCenterHollow: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub LeftInset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetLeftInset: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub LeftInsetScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetLeftInsetScale: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub RightInset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRightInset: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub RightInsetScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRightInsetScale: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TopInset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetTopInset: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub TopInsetScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetTopInsetScale: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub SetInsets: unsafe extern "system" fn(this: *mut *mut Self, inset: f32) -> ::windows_sys::core::HRESULT,
    pub SetInsetsWithValues: unsafe extern "system" fn(this: *mut *mut Self, left: f32, top: f32, right: f32, bottom: f32) -> ::windows_sys::core::HRESULT,
    pub SetInsetScales: unsafe extern "system" fn(this: *mut *mut Self, scale: f32) -> ::windows_sys::core::HRESULT,
    pub SetInsetScalesWithValues: unsafe extern "system" fn(this: *mut *mut Self, left: f32, top: f32, right: f32, bottom: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionObject {
    pub base__: ::windows_sys::core::IInspectable,
    pub Compositor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub Dispatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Dispatcher: usize,
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartAnimation: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StopAnimation: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionObject2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Comment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetComment: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ImplicitAnimations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetImplicitAnimations: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartAnimationGroup: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StopAnimationGroup: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionObject3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
#[repr(C)]
pub struct ICompositionObject4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetAnimationController: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionObjectFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionObjectStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub StartAnimationWithIAnimationObject: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, propertyname: ::windows_sys::core::HSTRING, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartAnimationGroupWithIAnimationObject: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionPath {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionPathFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Create: usize,
}
#[repr(C)]
pub struct ICompositionPathGeometry {
    pub base__: ::windows_sys::core::IInspectable,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionProjectedShadow {
    pub base__: ::windows_sys::core::IInspectable,
    pub BlurRadiusMultiplier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetBlurRadiusMultiplier: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Casters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LightSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLightSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxBlurRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMaxBlurRadius: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub MinBlurRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMinBlurRadius: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Receivers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionProjectedShadowCaster {
    pub base__: ::windows_sys::core::IInspectable,
    pub Brush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CastingVisual: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCastingVisual: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionProjectedShadowCasterCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub InsertAbove: unsafe extern "system" fn(this: *mut *mut Self, newcaster: *mut ::core::ffi::c_void, reference: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAtBottom: unsafe extern "system" fn(this: *mut *mut Self, newcaster: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAtTop: unsafe extern "system" fn(this: *mut *mut Self, newcaster: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertBelow: unsafe extern "system" fn(this: *mut *mut Self, newcaster: *mut ::core::ffi::c_void, reference: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, caster: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionProjectedShadowCasterCollectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxRespectedCasters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionProjectedShadowReceiver {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReceivingVisual: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetReceivingVisual: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionProjectedShadowReceiverUnorderedCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionPropertySet {
    pub base__: ::windows_sys::core::IInspectable,
    pub InsertColor: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: super::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertMatrix3x2: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertMatrix3x2: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertMatrix4x4: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertMatrix4x4: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertQuaternion: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertQuaternion: usize,
    pub InsertScalar: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertVector2: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertVector2: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertVector3: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertVector3: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertVector4: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: super::super::Foundation::Numerics::Vector4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertVector4: usize,
    pub TryGetColor: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: *mut super::Color, result__: *mut CompositionGetValueStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetMatrix3x2: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: *mut super::super::Foundation::Numerics::Matrix3x2, result__: *mut CompositionGetValueStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetMatrix3x2: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetMatrix4x4: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: *mut super::super::Foundation::Numerics::Matrix4x4, result__: *mut CompositionGetValueStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetMatrix4x4: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetQuaternion: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: *mut super::super::Foundation::Numerics::Quaternion, result__: *mut CompositionGetValueStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetQuaternion: usize,
    pub TryGetScalar: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: *mut f32, result__: *mut CompositionGetValueStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetVector2: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: *mut super::super::Foundation::Numerics::Vector2, result__: *mut CompositionGetValueStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetVector2: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetVector3: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: *mut super::super::Foundation::Numerics::Vector3, result__: *mut CompositionGetValueStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetVector3: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetVector4: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: *mut super::super::Foundation::Numerics::Vector4, result__: *mut CompositionGetValueStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetVector4: usize,
}
#[repr(C)]
pub struct ICompositionPropertySet2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InsertBoolean: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: bool) -> ::windows_sys::core::HRESULT,
    pub TryGetBoolean: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: *mut bool, result__: *mut CompositionGetValueStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionRadialGradientBrush {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub EllipseCenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    EllipseCenter: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetEllipseCenter: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetEllipseCenter: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub EllipseRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    EllipseRadius: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetEllipseRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetEllipseRadius: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GradientOriginOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GradientOriginOffset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetGradientOriginOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetGradientOriginOffset: usize,
}
#[repr(C)]
pub struct ICompositionRectangleGeometry {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Offset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOffset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Size: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetSize: usize,
}
#[repr(C)]
pub struct ICompositionRoundedRectangleGeometry {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub CornerRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CornerRadius: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetCornerRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetCornerRadius: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Offset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOffset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Size: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetSize: usize,
}
#[repr(C)]
pub struct ICompositionScopedBatch {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsEnded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Suspend: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
}
#[repr(C)]
pub struct ICompositionShadow {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionShadowFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionShape {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub CenterPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CenterPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetCenterPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetCenterPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Offset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOffset: usize,
    pub RotationAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub RotationAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRotationAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Scale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetScale: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TransformMatrix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TransformMatrix: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformMatrix: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformMatrix: usize,
}
#[repr(C)]
pub struct ICompositionShapeFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionSpriteShape {
    pub base__: ::windows_sys::core::IInspectable,
    pub FillBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFillBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Geometry: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGeometry: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsStrokeNonScaling: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsStrokeNonScaling: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub StrokeBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetStrokeBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StrokeDashArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StrokeDashArray: usize,
    pub StrokeDashCap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionStrokeCap) -> ::windows_sys::core::HRESULT,
    pub SetStrokeDashCap: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionStrokeCap) -> ::windows_sys::core::HRESULT,
    pub StrokeDashOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetStrokeDashOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub StrokeEndCap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionStrokeCap) -> ::windows_sys::core::HRESULT,
    pub SetStrokeEndCap: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionStrokeCap) -> ::windows_sys::core::HRESULT,
    pub StrokeLineJoin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionStrokeLineJoin) -> ::windows_sys::core::HRESULT,
    pub SetStrokeLineJoin: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionStrokeLineJoin) -> ::windows_sys::core::HRESULT,
    pub StrokeMiterLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetStrokeMiterLimit: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub StrokeStartCap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionStrokeCap) -> ::windows_sys::core::HRESULT,
    pub SetStrokeStartCap: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionStrokeCap) -> ::windows_sys::core::HRESULT,
    pub StrokeThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetStrokeThickness: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionSupportsSystemBackdrop {
    pub base__: ::windows_sys::core::IInspectable,
    pub SystemBackdrop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSystemBackdrop: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionSurface {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionSurfaceBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub BitmapInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionBitmapInterpolationMode) -> ::windows_sys::core::HRESULT,
    pub SetBitmapInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionBitmapInterpolationMode) -> ::windows_sys::core::HRESULT,
    pub HorizontalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Stretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionStretch) -> ::windows_sys::core::HRESULT,
    pub SetStretch: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionStretch) -> ::windows_sys::core::HRESULT,
    pub Surface: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSurface: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetVerticalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionSurfaceBrush2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub AnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AnchorPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetAnchorPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CenterPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CenterPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetCenterPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetCenterPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Offset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOffset: usize,
    pub RotationAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub RotationAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRotationAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Scale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetScale: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TransformMatrix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TransformMatrix: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformMatrix: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformMatrix: usize,
}
#[repr(C)]
pub struct ICompositionSurfaceBrush3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SnapToPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSnapToPixels: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionSurfaceFacade {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetRealSurface: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionTarget {
    pub base__: ::windows_sys::core::IInspectable,
    pub Root: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRoot: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionTargetFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionTransform {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionTransformFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionViewBox {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Offset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOffset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Size: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetSize: usize,
    pub Stretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionStretch) -> ::windows_sys::core::HRESULT,
    pub SetStretch: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionStretch) -> ::windows_sys::core::HRESULT,
    pub VerticalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetVerticalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionVirtualDrawingSurface {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics")]
    pub Trim: unsafe extern "system" fn(this: *mut *mut Self, rects_array_size: u32, rects: *const super::super::Graphics::RectInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Trim: usize,
}
#[repr(C)]
pub struct ICompositionVirtualDrawingSurfaceFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionVisualSurface {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceVisual: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSourceVisual: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SourceOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SourceOffset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetSourceOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetSourceOffset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SourceSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SourceSize: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetSourceSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetSourceSize: usize,
}
#[repr(C)]
pub struct ICompositor {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateColorKeyFrameAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateColorBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateColorBrushWithColor: unsafe extern "system" fn(this: *mut *mut Self, color: super::Color, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateContainerVisual: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCubicBezierEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, controlpoint1: super::super::Foundation::Numerics::Vector2, controlpoint2: super::super::Foundation::Numerics::Vector2, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCubicBezierEasingFunction: usize,
    #[cfg(feature = "Graphics_Effects")]
    pub CreateEffectFactory: unsafe extern "system" fn(this: *mut *mut Self, graphicseffect: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))]
    CreateEffectFactory: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Effects"))]
    pub CreateEffectFactoryWithProperties: unsafe extern "system" fn(this: *mut *mut Self, graphicseffect: *mut ::core::ffi::c_void, animatableproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Effects")))]
    CreateEffectFactoryWithProperties: usize,
    pub CreateExpressionAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateExpressionAnimationWithExpression: unsafe extern "system" fn(this: *mut *mut Self, expression: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInsetClip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInsetClipWithInsets: unsafe extern "system" fn(this: *mut *mut Self, leftinset: f32, topinset: f32, rightinset: f32, bottominset: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLinearEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePropertySet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateQuaternionKeyFrameAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateScalarKeyFrameAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateScopedBatch: unsafe extern "system" fn(this: *mut *mut Self, batchtype: CompositionBatchTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSpriteVisual: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSurfaceBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSurfaceBrushWithSurface: unsafe extern "system" fn(this: *mut *mut Self, surface: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTargetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateVector2KeyFrameAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateVector3KeyFrameAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateVector4KeyFrameAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCommitBatch: unsafe extern "system" fn(this: *mut *mut Self, batchtype: CompositionBatchTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositor2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateAmbientLight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAnimationGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBackdropBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDistantLight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDropShadow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateImplicitAnimationCollection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLayerVisual: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateMaskBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateNineGridBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePointLight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSpotLight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateStepEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateStepEasingFunctionWithStepCount: unsafe extern "system" fn(this: *mut *mut Self, stepcount: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositor3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateHostBackdropBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositor4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateColorGradientStop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateColorGradientStopWithOffsetAndColor: unsafe extern "system" fn(this: *mut *mut Self, offset: f32, color: super::Color, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLinearGradientBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSpringScalarAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSpringVector2Animation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSpringVector3Animation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositor5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Comment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetComment: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GlobalPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetGlobalPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub CreateBounceScalarAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBounceVector2Animation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBounceVector3Animation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateContainerShape: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateEllipseGeometry: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLineGeometry: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePathGeometry: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePathGeometryWithPath: unsafe extern "system" fn(this: *mut *mut Self, path: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePathKeyFrameAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRectangleGeometry: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRoundedRectangleGeometry: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateShapeVisual: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSpriteShape: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSpriteShapeWithGeometry: unsafe extern "system" fn(this: *mut *mut Self, geometry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateViewBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestCommitAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCommitAsync: usize,
}
#[repr(C)]
pub struct ICompositor6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateGeometricClip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateGeometricClipWithGeometry: unsafe extern "system" fn(this: *mut *mut Self, geometry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRedirectVisual: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRedirectVisualWithSourceVisual: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBooleanKeyFrameAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositor7 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
    pub CreateAnimationPropertyInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRectangleClipWithSides: unsafe extern "system" fn(this: *mut *mut Self, left: f32, top: f32, right: f32, bottom: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateRectangleClipWithSidesAndRadius: unsafe extern "system" fn(this: *mut *mut Self, left: f32, top: f32, right: f32, bottom: f32, topleftradius: super::super::Foundation::Numerics::Vector2, toprightradius: super::super::Foundation::Numerics::Vector2, bottomrightradius: super::super::Foundation::Numerics::Vector2, bottomleftradius: super::super::Foundation::Numerics::Vector2, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateRectangleClipWithSidesAndRadius: usize,
}
#[repr(C)]
pub struct ICompositorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxGlobalPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub MinGlobalPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositorWithBlurredWallpaperBackdropBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryCreateBlurredWallpaperBackdropBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositorWithProjectedShadow {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateProjectedShadowCaster: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateProjectedShadow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateProjectedShadowReceiver: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositorWithRadialGradient {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateRadialGradientBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositorWithVisualSurface {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateVisualSurface: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContainerVisual {
    pub base__: ::windows_sys::core::IInspectable,
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContainerVisualFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICubicBezierEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub ControlPoint1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ControlPoint1: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ControlPoint2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ControlPoint2: usize,
}
#[repr(C)]
pub struct IDelegatedInkTrailVisual {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AddTrailPoints: unsafe extern "system" fn(this: *mut *mut Self, inkPoints_array_size: u32, inkpoints: *const InkTrailPoint, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddTrailPoints: usize,
    #[cfg(feature = "Foundation")]
    pub AddTrailPointsWithPrediction: unsafe extern "system" fn(this: *mut *mut Self, inkPoints_array_size: u32, inkpoints: *const InkTrailPoint, predictedInkPoints_array_size: u32, predictedinkpoints: *const InkTrailPoint, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddTrailPointsWithPrediction: usize,
    pub RemoveTrailPoints: unsafe extern "system" fn(this: *mut *mut Self, generationid: u32) -> ::windows_sys::core::HRESULT,
    pub StartNewTrail: unsafe extern "system" fn(this: *mut *mut Self, color: super::Color) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDelegatedInkTrailVisualStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateForSwapChain: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDistantLight {
    pub base__: ::windows_sys::core::IInspectable,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
    pub CoordinateSpace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCoordinateSpace: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Direction: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDirection: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDirection: usize,
}
#[repr(C)]
pub struct IDistantLight2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Intensity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetIntensity: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDropShadow {
    pub base__: ::windows_sys::core::IInspectable,
    pub BlurRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetBlurRadius: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
    pub Mask: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMask: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Offset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOffset: usize,
    pub Opacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDropShadow2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourcePolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionDropShadowSourcePolicy) -> ::windows_sys::core::HRESULT,
    pub SetSourcePolicy: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionDropShadowSourcePolicy) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IElasticEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
    pub Oscillations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Springiness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IExponentialEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
    pub Exponent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IExpressionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Expression: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetExpression: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImplicitAnimationCollection {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInsetClip {
    pub base__: ::windows_sys::core::IInspectable,
    pub BottomInset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetBottomInset: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub LeftInset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetLeftInset: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub RightInset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRightInset: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub TopInset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetTopInset: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyFrameAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DelayTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DelayTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDelayTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDelayTime: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub IterationBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AnimationIterationBehavior) -> ::windows_sys::core::HRESULT,
    pub SetIterationBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: AnimationIterationBehavior) -> ::windows_sys::core::HRESULT,
    pub IterationCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetIterationCount: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub KeyFrameCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub StopBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AnimationStopBehavior) -> ::windows_sys::core::HRESULT,
    pub SetStopBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: AnimationStopBehavior) -> ::windows_sys::core::HRESULT,
    pub InsertExpressionKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InsertExpressionKeyFrameWithEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: ::windows_sys::core::HSTRING, easingfunction: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyFrameAnimation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AnimationDirection) -> ::windows_sys::core::HRESULT,
    pub SetDirection: unsafe extern "system" fn(this: *mut *mut Self, value: AnimationDirection) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyFrameAnimation3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DelayBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AnimationDelayBehavior) -> ::windows_sys::core::HRESULT,
    pub SetDelayBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: AnimationDelayBehavior) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyFrameAnimationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ILayerVisual {
    pub base__: ::windows_sys::core::IInspectable,
    pub Effect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEffect: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILayerVisual2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Shadow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetShadow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILinearEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct INaturalMotionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub DelayBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AnimationDelayBehavior) -> ::windows_sys::core::HRESULT,
    pub SetDelayBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: AnimationDelayBehavior) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DelayTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DelayTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDelayTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDelayTime: usize,
    pub StopBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AnimationStopBehavior) -> ::windows_sys::core::HRESULT,
    pub SetStopBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: AnimationStopBehavior) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INaturalMotionAnimationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPathKeyFrameAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub InsertKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, path: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, path: *mut ::core::ffi::c_void, easingfunction: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointLight {
    pub base__: ::windows_sys::core::IInspectable,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
    pub ConstantAttenuation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetConstantAttenuation: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub CoordinateSpace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCoordinateSpace: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LinearAttenuation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetLinearAttenuation: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Offset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOffset: usize,
    pub QuadraticAttenuation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetQuadraticAttenuation: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointLight2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Intensity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetIntensity: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointLight3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MinAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMinAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub MaxAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMaxAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPowerEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
    pub Power: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IQuaternionKeyFrameAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertKeyFrame: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Quaternion, easingfunction: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertKeyFrameWithEasingFunction: usize,
}
#[repr(C)]
pub struct IRectangleClip {
    pub base__: ::windows_sys::core::IInspectable,
    pub Bottom: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetBottom: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub BottomLeftRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    BottomLeftRadius: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetBottomLeftRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetBottomLeftRadius: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub BottomRightRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    BottomRightRadius: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetBottomRightRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetBottomRightRadius: usize,
    pub Left: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetLeft: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Right: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRight: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Top: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetTop: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TopLeftRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TopLeftRadius: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTopLeftRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTopLeftRadius: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TopRightRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TopRightRadius: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTopRightRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTopRightRadius: usize,
}
#[repr(C)]
pub struct IRedirectVisual {
    pub base__: ::windows_sys::core::IInspectable,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRenderingDeviceReplacedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GraphicsDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScalarKeyFrameAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub InsertKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: f32) -> ::windows_sys::core::HRESULT,
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: f32, easingfunction: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScalarNaturalMotionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FinalValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinalValue: usize,
    #[cfg(feature = "Foundation")]
    pub SetFinalValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFinalValue: usize,
    #[cfg(feature = "Foundation")]
    pub InitialValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitialValue: usize,
    #[cfg(feature = "Foundation")]
    pub SetInitialValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInitialValue: usize,
    pub InitialVelocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInitialVelocity: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScalarNaturalMotionAnimationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IShapeVisual {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Shapes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Shapes: usize,
    pub ViewBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetViewBox: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISineEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpotLight {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConstantAttenuation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetConstantAttenuation: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub CoordinateSpace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCoordinateSpace: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Direction: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDirection: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDirection: usize,
    pub InnerConeAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInnerConeAngle: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub InnerConeAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInnerConeAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub InnerConeColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub SetInnerConeColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
    pub LinearAttenuation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetLinearAttenuation: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Offset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOffset: usize,
    pub OuterConeAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetOuterConeAngle: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub OuterConeAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetOuterConeAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub OuterConeColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub SetOuterConeColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
    pub QuadraticAttenuation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetQuadraticAttenuation: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpotLight2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InnerConeIntensity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInnerConeIntensity: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub OuterConeIntensity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetOuterConeIntensity: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpotLight3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MinAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMinAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub MaxAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMaxAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpringScalarNaturalMotionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub DampingRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDampingRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Period: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Period: usize,
    #[cfg(feature = "Foundation")]
    pub SetPeriod: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPeriod: usize,
}
#[repr(C)]
pub struct ISpringVector2NaturalMotionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub DampingRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDampingRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Period: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Period: usize,
    #[cfg(feature = "Foundation")]
    pub SetPeriod: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPeriod: usize,
}
#[repr(C)]
pub struct ISpringVector3NaturalMotionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub DampingRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDampingRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Period: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Period: usize,
    #[cfg(feature = "Foundation")]
    pub SetPeriod: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPeriod: usize,
}
#[repr(C)]
pub struct ISpriteVisual {
    pub base__: ::windows_sys::core::IInspectable,
    pub Brush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpriteVisual2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Shadow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetShadow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStepEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub FinalStep: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetFinalStep: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub InitialStep: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetInitialStep: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub IsFinalStepSingleFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFinalStepSingleFrame: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsInitialStepSingleFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsInitialStepSingleFrame: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub StepCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetStepCount: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVector2KeyFrameAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertKeyFrame: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector2, easingfunction: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertKeyFrameWithEasingFunction: usize,
}
#[repr(C)]
pub struct IVector2NaturalMotionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub FinalValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FinalValue: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetFinalValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetFinalValue: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub InitialValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InitialValue: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetInitialValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetInitialValue: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub InitialVelocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InitialVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetInitialVelocity: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetInitialVelocity: usize,
}
#[repr(C)]
pub struct IVector2NaturalMotionAnimationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IVector3KeyFrameAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertKeyFrame: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector3, easingfunction: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertKeyFrameWithEasingFunction: usize,
}
#[repr(C)]
pub struct IVector3NaturalMotionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub FinalValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FinalValue: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetFinalValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetFinalValue: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub InitialValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InitialValue: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetInitialValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetInitialValue: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub InitialVelocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InitialVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetInitialVelocity: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetInitialVelocity: usize,
}
#[repr(C)]
pub struct IVector3NaturalMotionAnimationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IVector4KeyFrameAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertKeyFrame: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector4, easingfunction: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    InsertKeyFrameWithEasingFunction: usize,
}
#[repr(C)]
pub struct IVisual {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub AnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AnchorPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetAnchorPoint: usize,
    pub BackfaceVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionBackfaceVisibility) -> ::windows_sys::core::HRESULT,
    pub SetBackfaceVisibility: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionBackfaceVisibility) -> ::windows_sys::core::HRESULT,
    pub BorderMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionBorderMode) -> ::windows_sys::core::HRESULT,
    pub SetBorderMode: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionBorderMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CenterPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CenterPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetCenterPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetCenterPoint: usize,
    pub Clip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetClip: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CompositeMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionCompositeMode) -> ::windows_sys::core::HRESULT,
    pub SetCompositeMode: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionCompositeMode) -> ::windows_sys::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Offset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOffset: usize,
    pub Opacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOrientation: usize,
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RotationAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub RotationAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRotationAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub RotationAxis: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RotationAxis: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRotationAxis: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRotationAxis: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Scale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetScale: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Size: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetSize: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TransformMatrix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TransformMatrix: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformMatrix: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformMatrix: usize,
}
#[repr(C)]
pub struct IVisual2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParentForTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetParentForTransform: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub RelativeOffsetAdjustment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RelativeOffsetAdjustment: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRelativeOffsetAdjustment: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRelativeOffsetAdjustment: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub RelativeSizeAdjustment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RelativeSizeAdjustment: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRelativeSizeAdjustment: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRelativeSizeAdjustment: usize,
}
#[repr(C)]
pub struct IVisual3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsHitTestVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHitTestVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisual4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPixelSnappingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPixelSnappingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisualCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub InsertAbove: unsafe extern "system" fn(this: *mut *mut Self, newchild: *mut ::core::ffi::c_void, sibling: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAtBottom: unsafe extern "system" fn(this: *mut *mut Self, newchild: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAtTop: unsafe extern "system" fn(this: *mut *mut Self, newchild: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertBelow: unsafe extern "system" fn(this: *mut *mut Self, newchild: *mut ::core::ffi::c_void, sibling: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, child: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisualElement {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IVisualElement2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetVisualInternal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisualFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IVisualUnorderedCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, newvisual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
pub type ImplicitAnimationCollection = *mut ::core::ffi::c_void;
pub type InitialValueExpressionCollection = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Composition\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct InkTrailPoint {
    pub Point: super::super::Foundation::Point,
    pub Radius: f32,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for InkTrailPoint {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for InkTrailPoint {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InsetClip = *mut ::core::ffi::c_void;
pub type KeyFrameAnimation = *mut ::core::ffi::c_void;
pub type LayerVisual = *mut ::core::ffi::c_void;
pub type LinearEasingFunction = *mut ::core::ffi::c_void;
pub type NaturalMotionAnimation = *mut ::core::ffi::c_void;
pub type PathKeyFrameAnimation = *mut ::core::ffi::c_void;
pub type PointLight = *mut ::core::ffi::c_void;
pub type PowerEasingFunction = *mut ::core::ffi::c_void;
pub type QuaternionKeyFrameAnimation = *mut ::core::ffi::c_void;
pub type RectangleClip = *mut ::core::ffi::c_void;
pub type RedirectVisual = *mut ::core::ffi::c_void;
pub type RenderingDeviceReplacedEventArgs = *mut ::core::ffi::c_void;
pub type ScalarKeyFrameAnimation = *mut ::core::ffi::c_void;
pub type ScalarNaturalMotionAnimation = *mut ::core::ffi::c_void;
pub type ShapeVisual = *mut ::core::ffi::c_void;
pub type SineEasingFunction = *mut ::core::ffi::c_void;
pub type SpotLight = *mut ::core::ffi::c_void;
pub type SpringScalarNaturalMotionAnimation = *mut ::core::ffi::c_void;
pub type SpringVector2NaturalMotionAnimation = *mut ::core::ffi::c_void;
pub type SpringVector3NaturalMotionAnimation = *mut ::core::ffi::c_void;
pub type SpriteVisual = *mut ::core::ffi::c_void;
pub type StepEasingFunction = *mut ::core::ffi::c_void;
pub type Vector2KeyFrameAnimation = *mut ::core::ffi::c_void;
pub type Vector2NaturalMotionAnimation = *mut ::core::ffi::c_void;
pub type Vector3KeyFrameAnimation = *mut ::core::ffi::c_void;
pub type Vector3NaturalMotionAnimation = *mut ::core::ffi::c_void;
pub type Vector4KeyFrameAnimation = *mut ::core::ffi::c_void;
pub type Visual = *mut ::core::ffi::c_void;
pub type VisualCollection = *mut ::core::ffi::c_void;
pub type VisualUnorderedCollection = *mut ::core::ffi::c_void;
