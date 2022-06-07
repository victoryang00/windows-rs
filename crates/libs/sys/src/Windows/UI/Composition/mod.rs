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
impl ::windows_sys::core::Interface for IAmbientLight {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2759930017, data2: 47044, data3: 18167, data4: [185, 191, 218, 244, 58, 68, 230, 238] };
}
#[repr(C)]
pub struct IAmbientLight2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Intensity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetIntensity: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAmbientLight2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 996452031, data2: 24471, data3: 19604, data4: [134, 229, 4, 45, 211, 134, 178, 125] };
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
impl ::windows_sys::core::Interface for IAnimationController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3375689682, data2: 1826, data3: 20319, data4: [164, 226, 149, 16, 243, 212, 59, 247] };
}
#[repr(C)]
pub struct IAnimationControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub MinPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAnimationControllerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3876676831, data2: 25883, data3: 18432, data4: [185, 229, 106, 59, 207, 237, 51, 101] };
}
#[repr(C)]
pub struct IAnimationObject {
    pub base__: ::windows_sys::core::IInspectable,
    pub PopulatePropertyInfo: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, propertyinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAnimationObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3876855306, data2: 1208, data3: 20421, data4: [164, 220, 25, 83, 146, 229, 120, 7] };
}
#[repr(C)]
pub struct IAnimationPropertyInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub AccessMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AnimationPropertyAccessMode) -> ::windows_sys::core::HRESULT,
    pub SetAccessMode: unsafe extern "system" fn(this: *mut *mut Self, value: AnimationPropertyAccessMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAnimationPropertyInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4101074693, data2: 60791, data3: 20028, data4: [179, 40, 92, 57, 133, 179, 115, 143] };
}
#[repr(C)]
pub struct IAnimationPropertyInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetResolvedCompositionObject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetResolvedCompositionObjectProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAnimationPropertyInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1494687924, data2: 29810, data3: 21016, data4: [139, 57, 223, 254, 97, 90, 230, 218] };
}
#[repr(C)]
pub struct IBackEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
    pub Amplitude: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackEasingFunction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3092647332, data2: 24124, data3: 21597, data4: [178, 99, 121, 135, 162, 189, 39, 203] };
}
#[repr(C)]
pub struct IBooleanKeyFrameAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub InsertKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBooleanKeyFrameAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2514631176, data2: 53748, data3: 18802, data4: [151, 112, 62, 254, 104, 216, 46, 20] };
}
#[repr(C)]
pub struct IBounceEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
    pub Bounces: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Bounciness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBounceEasingFunction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3892163659, data2: 43733, data3: 20852, data4: [148, 33, 238, 248, 183, 90, 106, 67] };
}
#[repr(C)]
pub struct IBounceScalarNaturalMotionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Acceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetAcceleration: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Restitution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRestitution: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBounceScalarNaturalMotionAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3131248076, data2: 42547, data3: 17944, data4: [155, 6, 127, 124, 114, 200, 124, 255] };
}
#[repr(C)]
pub struct IBounceVector2NaturalMotionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Acceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetAcceleration: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Restitution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRestitution: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBounceVector2NaturalMotionAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3660857750, data2: 8532, data3: 19260, data4: [136, 170, 71, 54, 18, 4, 236, 205] };
}
#[repr(C)]
pub struct IBounceVector3NaturalMotionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Acceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetAcceleration: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Restitution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRestitution: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBounceVector3NaturalMotionAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1205517361, data2: 4307, data3: 17688, data4: [134, 241, 9, 202, 247, 66, 209, 19] };
}
#[repr(C)]
pub struct ICircleEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICircleEasingFunction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 503783978, data2: 28546, data3: 23080, data4: [135, 72, 46, 146, 252, 70, 238, 43] };
}
#[repr(C)]
pub struct IColorKeyFrameAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub InterpolationColorSpace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionColorSpace) -> ::windows_sys::core::HRESULT,
    pub SetInterpolationColorSpace: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionColorSpace) -> ::windows_sys::core::HRESULT,
    pub InsertKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: super::Color) -> ::windows_sys::core::HRESULT,
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: super::Color, easingfunction: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IColorKeyFrameAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2477635049, data2: 36357, data3: 17811, data4: [132, 163, 220, 161, 82, 120, 30, 86] };
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
impl ::windows_sys::core::Interface for ICompositionAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1179405356, data2: 7338, data3: 16481, data4: [155, 64, 225, 63, 222, 21, 3, 202] };
}
#[repr(C)]
pub struct ICompositionAnimation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetBooleanParameter: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, value: bool) -> ::windows_sys::core::HRESULT,
    pub Target: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTarget: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionAnimation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 916152382, data2: 43023, data3: 18760, data4: [147, 227, 237, 35, 251, 56, 198, 203] };
}
#[repr(C)]
pub struct ICompositionAnimation3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub InitialValueExpressions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InitialValueExpressions: usize,
}
impl ::windows_sys::core::Interface for ICompositionAnimation3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3575513869, data2: 32164, data3: 19415, data4: [188, 45, 244, 81, 117, 41, 244, 58] };
}
#[repr(C)]
pub struct ICompositionAnimation4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetExpressionReferenceParameter: unsafe extern "system" fn(this: *mut *mut Self, parametername: ::windows_sys::core::HSTRING, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionAnimation4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1996568510, data2: 30396, data3: 20003, data4: [191, 237, 254, 156, 194, 15, 110, 201] };
}
#[repr(C)]
pub struct ICompositionAnimationBase {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionAnimationBase {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 472656281, data2: 59416, data3: 18643, data4: [166, 221, 215, 140, 130, 248, 172, 233] };
}
#[repr(C)]
pub struct ICompositionAnimationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionAnimationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 284607739, data2: 28241, data3: 19493, data4: [187, 211, 88, 106, 155, 236, 62, 244] };
}
#[repr(C)]
pub struct ICompositionAnimationGroup {
    pub base__: ::windows_sys::core::IInspectable,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionAnimationGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1585236236, data2: 52500, data3: 19975, data4: [138, 85, 199, 37, 39, 170, 189, 172] };
}
#[repr(C)]
pub struct ICompositionBackdropBrush {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionBackdropBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3316428376, data2: 14488, data3: 18846, data4: [141, 127, 34, 78, 145, 40, 106, 93] };
}
#[repr(C)]
pub struct ICompositionBatchCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionBatchCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 218159824, data2: 37988, data3: 17674, data4: [165, 98, 46, 38, 152, 176, 168, 18] };
}
#[repr(C)]
pub struct ICompositionBrush {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2869786120, data2: 12480, data3: 16617, data4: [181, 104, 182, 10, 107, 209, 251, 70] };
}
#[repr(C)]
pub struct ICompositionBrushFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionBrushFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3662936908, data2: 18000, data3: 18372, data4: [173, 118, 118, 83, 121, 96, 126, 214] };
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
impl ::windows_sys::core::Interface for ICompositionCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2186491198, data2: 46359, data3: 18620, data4: [177, 232, 75, 53, 97, 162, 225, 129] };
}
#[repr(C)]
pub struct ICompositionCapabilitiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionCapabilitiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4156008558, data2: 25622, data3: 18917, data4: [141, 223, 175, 233, 73, 226, 5, 98] };
}
#[repr(C)]
pub struct ICompositionClip {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionClip {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 483207762, data2: 53191, data3: 19150, data4: [153, 131, 20, 107, 184, 235, 106, 60] };
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
impl ::windows_sys::core::Interface for ICompositionClip2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1486086249, data2: 13590, data3: 16609, data4: [137, 224, 91, 169, 36, 146, 114, 53] };
}
#[repr(C)]
pub struct ICompositionClipFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionClipFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3108523183, data2: 8391, data3: 19181, data4: [172, 74, 156, 120, 186, 19, 2, 207] };
}
#[repr(C)]
pub struct ICompositionColorBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionColorBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 723930206, data2: 48949, data3: 18481, data4: [134, 66, 207, 112, 194, 15, 255, 47] };
}
#[repr(C)]
pub struct ICompositionColorGradientStop {
    pub base__: ::windows_sys::core::IInspectable,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionColorGradientStop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1862322834, data2: 51201, data3: 20033, data4: [154, 143, 165, 62, 32, 245, 119, 120] };
}
#[repr(C)]
pub struct ICompositionColorGradientStopCollection {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionColorGradientStopCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2669486316, data2: 31492, data3: 19229, data4: [144, 188, 159, 163, 44, 12, 253, 38] };
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
impl ::windows_sys::core::Interface for ICompositionCommitBatch {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 218159824, data2: 51719, data3: 17408, data4: [140, 142, 203, 93, 176, 133, 89, 204] };
}
#[repr(C)]
pub struct ICompositionContainerShape {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Shapes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Shapes: usize,
}
impl ::windows_sys::core::Interface for ICompositionContainerShape {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1331594651, data2: 11867, data3: 17576, data4: [152, 44, 170, 15, 105, 193, 96, 89] };
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
impl ::windows_sys::core::Interface for ICompositionDrawingSurface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2707866368, data2: 64208, data3: 19729, data4: [158, 103, 228, 51, 22, 47, 244, 158] };
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
impl ::windows_sys::core::Interface for ICompositionDrawingSurface2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4207995019, data2: 58196, data3: 17640, data4: [142, 61, 196, 136, 13, 90, 33, 63] };
}
#[repr(C)]
pub struct ICompositionDrawingSurfaceFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionDrawingSurfaceFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2492968970, data2: 12589, data3: 18105, data4: [157, 179, 65, 47, 215, 148, 100, 200] };
}
#[repr(C)]
pub struct ICompositionEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionEasingFunction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1363534678, data2: 49017, data3: 20136, data4: [140, 194, 107, 91, 71, 46, 108, 154] };
}
#[repr(C)]
pub struct ICompositionEasingFunctionFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionEasingFunctionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1619265396, data2: 15776, data3: 18761, data4: [130, 0, 114, 6, 192, 1, 144, 160] };
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
impl ::windows_sys::core::Interface for ICompositionEasingFunctionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 396846774, data2: 10550, data3: 21482, data4: [181, 175, 198, 66, 244, 166, 16, 131] };
}
#[repr(C)]
pub struct ICompositionEffectBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetSourceParameter: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSourceParameter: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionEffectBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3212802398, data2: 33740, data3: 17599, data4: [164, 71, 62, 60, 7, 23, 137, 236] };
}
#[repr(C)]
pub struct ICompositionEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub LoadStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEffectFactoryLoadStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionEffectFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3193316527, data2: 47742, data3: 17680, data4: [152, 80, 65, 192, 180, 255, 116, 223] };
}
#[repr(C)]
pub struct ICompositionEffectSourceParameter {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionEffectSourceParameter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2240459066, data2: 12946, data3: 20046, data4: [179, 187, 43, 108, 101, 68, 166, 238] };
}
#[repr(C)]
pub struct ICompositionEffectSourceParameterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionEffectSourceParameterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3017405046, data2: 43939, data3: 18212, data4: [172, 243, 208, 57, 116, 100, 219, 28] };
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
impl ::windows_sys::core::Interface for ICompositionEllipseGeometry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1208088708, data2: 63149, data3: 19347, data4: [175, 169, 137, 123, 100, 229, 123, 31] };
}
#[repr(C)]
pub struct ICompositionGeometricClip {
    pub base__: ::windows_sys::core::IInspectable,
    pub Geometry: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGeometry: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ViewBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetViewBox: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionGeometricClip {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3359683969, data2: 33225, data3: 17476, data4: [162, 193, 204, 174, 206, 58, 80, 229] };
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
impl ::windows_sys::core::Interface for ICompositionGeometry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3917816188, data2: 27159, data3: 16903, data4: [171, 216, 95, 211, 221, 97, 42, 157] };
}
#[repr(C)]
pub struct ICompositionGeometryFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionGeometryFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3221143521, data2: 35877, data3: 18443, data4: [159, 86, 254, 214, 178, 136, 5, 93] };
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
impl ::windows_sys::core::Interface for ICompositionGradientBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 496437728, data2: 65478, data3: 19470, data4: [169, 171, 52, 20, 77, 76, 144, 152] };
}
#[repr(C)]
pub struct ICompositionGradientBrush2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MappingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionMappingMode) -> ::windows_sys::core::HRESULT,
    pub SetMappingMode: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionMappingMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionGradientBrush2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2308822433, data2: 46279, data3: 19251, data4: [161, 182, 38, 74, 221, 194, 109, 16] };
}
#[repr(C)]
pub struct ICompositionGradientBrushFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionGradientBrushFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1456956887, data2: 61833, data3: 18633, data4: [156, 141, 148, 218, 241, 190, 192, 16] };
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
impl ::windows_sys::core::Interface for ICompositionGraphicsDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4213360353, data2: 32930, data3: 18023, data4: [153, 54, 219, 234, 246, 238, 254, 149] };
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
impl ::windows_sys::core::Interface for ICompositionGraphicsDevice2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 263765494, data2: 49392, data3: 19404, data4: [159, 184, 8, 73, 130, 73, 13, 125] };
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
impl ::windows_sys::core::Interface for ICompositionGraphicsDevice3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 938898708, data2: 54255, data3: 18897, data4: [182, 157, 13, 142, 171, 235, 54, 38] };
}
#[repr(C)]
pub struct ICompositionGraphicsDevice4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))]
    pub CaptureAsync: unsafe extern "system" fn(this: *mut *mut Self, capturevisual: *mut ::core::ffi::c_void, size: super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, sdrboost: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX")))]
    CaptureAsync: usize,
}
impl ::windows_sys::core::Interface for ICompositionGraphicsDevice4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1517535225, data2: 43391, data3: 19701, data4: [186, 70, 152, 239, 53, 142, 113, 177] };
}
#[repr(C)]
pub struct ICompositionLight {
    pub base__: ::windows_sys::core::IInspectable,
    pub Targets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionLight {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1101453250, data2: 11869, data3: 19393, data4: [176, 158, 143, 10, 3, 227, 216, 211] };
}
#[repr(C)]
pub struct ICompositionLight2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExclusionsFromTargets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionLight2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2814171762, data2: 62301, data3: 16989, data4: [155, 152, 35, 244, 32, 95, 102, 105] };
}
#[repr(C)]
pub struct ICompositionLight3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionLight3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1259012324, data2: 57095, data3: 18777, data4: [183, 164, 79, 126, 66, 51, 248, 56] };
}
#[repr(C)]
pub struct ICompositionLightFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionLightFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 110949126, data2: 55868, data3: 19268, data4: [131, 138, 94, 3, 213, 26, 206, 85] };
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
impl ::windows_sys::core::Interface for ICompositionLineGeometry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3715503524, data2: 3226, data3: 19303, data4: [141, 206, 68, 10, 91, 249, 205, 236] };
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
impl ::windows_sys::core::Interface for ICompositionLinearGradientBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2554053913, data2: 43483, data3: 16700, data4: [162, 216, 42, 144, 86, 252, 82, 94] };
}
#[repr(C)]
pub struct ICompositionMaskBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mask: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMask: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionMaskBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1378676894, data2: 48747, data3: 20289, data4: [190, 73, 249, 34, 109, 71, 27, 74] };
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
impl ::windows_sys::core::Interface for ICompositionMipmapSurface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1214474076, data2: 53066, data3: 19228, data4: [158, 206, 197, 236, 12, 43, 47, 230] };
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
impl ::windows_sys::core::Interface for ICompositionNineGridBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4065416420, data2: 48268, data3: 19431, data4: [184, 15, 134, 133, 184, 60, 1, 134] };
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
impl ::windows_sys::core::Interface for ICompositionObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3165957445, data2: 30217, data3: 17744, data4: [147, 79, 22, 0, 42, 104, 253, 237] };
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
impl ::windows_sys::core::Interface for ICompositionObject2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4018622113, data2: 23807, data3: 19304, data4: [158, 48, 161, 81, 157, 8, 186, 3] };
}
#[repr(C)]
pub struct ICompositionObject3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
impl ::windows_sys::core::Interface for ICompositionObject3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1271036197, data2: 56013, data3: 19698, data4: [152, 177, 152, 107, 118, 231, 235, 230] };
}
#[repr(C)]
pub struct ICompositionObject4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetAnimationController: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionObject4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 196311116, data2: 13419, data3: 19068, data4: [150, 107, 115, 16, 150, 101, 83, 213] };
}
#[repr(C)]
pub struct ICompositionObjectFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionObjectFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1361075294, data2: 21898, data3: 20266, data4: [141, 57, 55, 191, 225, 226, 13, 221] };
}
#[repr(C)]
pub struct ICompositionObjectStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub StartAnimationWithIAnimationObject: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, propertyname: ::windows_sys::core::HSTRING, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartAnimationGroupWithIAnimationObject: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionObjectStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3253536047, data2: 7074, data3: 17594, data4: [169, 4, 106, 136, 42, 10, 90, 219] };
}
#[repr(C)]
pub struct ICompositionPath {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionPath {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1725570399, data2: 11792, data3: 20258, data4: [138, 6, 10, 129, 81, 145, 158, 96] };
}
#[repr(C)]
pub struct ICompositionPathFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for ICompositionPathFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2619247722, data2: 3891, data3: 18257, data4: [148, 55, 235, 63, 185, 211, 171, 7] };
}
#[repr(C)]
pub struct ICompositionPathGeometry {
    pub base__: ::windows_sys::core::IInspectable,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionPathGeometry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 191512958, data2: 11383, data3: 19491, data4: [175, 94, 99, 4, 193, 71, 187, 97] };
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
impl ::windows_sys::core::Interface for ICompositionProjectedShadow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 677088882, data2: 17192, data3: 21055, data4: [188, 242, 85, 87, 197, 44, 59, 37] };
}
#[repr(C)]
pub struct ICompositionProjectedShadowCaster {
    pub base__: ::windows_sys::core::IInspectable,
    pub Brush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CastingVisual: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCastingVisual: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionProjectedShadowCaster {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2983711782, data2: 7734, data3: 23138, data4: [190, 86, 161, 97, 18, 253, 209, 72] };
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
impl ::windows_sys::core::Interface for ICompositionProjectedShadowCasterCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3528612876, data2: 57471, data3: 22691, data4: [172, 145, 55, 247, 62, 233, 23, 64] };
}
#[repr(C)]
pub struct ICompositionProjectedShadowCasterCollectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxRespectedCasters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionProjectedShadowCasterCollectionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1459335478, data2: 59727, data3: 21145, data4: [171, 91, 110, 21, 227, 139, 216, 153] };
}
#[repr(C)]
pub struct ICompositionProjectedShadowReceiver {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReceivingVisual: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetReceivingVisual: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionProjectedShadowReceiver {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 326604890, data2: 27209, data3: 21354, data4: [155, 228, 169, 106, 142, 82, 152, 169] };
}
#[repr(C)]
pub struct ICompositionProjectedShadowReceiverUnorderedCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionProjectedShadowReceiverUnorderedCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 45343671, data2: 10194, data3: 22943, data4: [172, 75, 171, 120, 124, 221, 230, 253] };
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
impl ::windows_sys::core::Interface for ICompositionPropertySet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3386298882, data2: 24423, data3: 17491, data4: [145, 23, 158, 173, 212, 48, 211, 194] };
}
#[repr(C)]
pub struct ICompositionPropertySet2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InsertBoolean: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: bool) -> ::windows_sys::core::HRESULT,
    pub TryGetBoolean: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, value: *mut bool, result__: *mut CompositionGetValueStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionPropertySet2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3732960030, data2: 41489, data3: 17493, data4: [136, 128, 125, 15, 63, 106, 68, 253] };
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
impl ::windows_sys::core::Interface for ICompositionRadialGradientBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1027297477, data2: 58362, data3: 19682, data4: [185, 252, 62, 225, 37, 97, 120, 143] };
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
impl ::windows_sys::core::Interface for ICompositionRectangleGeometry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 215290920, data2: 21334, data3: 16966, data4: [174, 207, 122, 11, 118, 151, 84, 0] };
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
impl ::windows_sys::core::Interface for ICompositionRoundedRectangleGeometry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2272315426, data2: 7504, data3: 19339, data4: [176, 19, 124, 154, 14, 70, 147, 95] };
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
impl ::windows_sys::core::Interface for ICompositionScopedBatch {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 218159824, data2: 64263, data3: 18173, data4: [140, 114, 98, 128, 209, 163, 209, 221] };
}
#[repr(C)]
pub struct ICompositionShadow {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionShadow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 849236706, data2: 17205, data3: 18892, data4: [177, 74, 55, 120, 45, 16, 240, 196] };
}
#[repr(C)]
pub struct ICompositionShadowFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionShadowFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 572475695, data2: 56506, data3: 19345, data4: [153, 158, 29, 194, 23, 160, 21, 48] };
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
impl ::windows_sys::core::Interface for ICompositionShape {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3028083447, data2: 39560, data3: 17092, data4: [158, 135, 46, 80, 12, 168, 104, 140] };
}
#[repr(C)]
pub struct ICompositionShapeFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionShapeFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 503068368, data2: 45146, data3: 17647, data4: [130, 176, 18, 17, 139, 205, 76, 208] };
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
impl ::windows_sys::core::Interface for ICompositionSpriteShape {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1075536315, data2: 7, data3: 17251, data4: [177, 243, 107, 204, 0, 63, 184, 62] };
}
#[repr(C)]
pub struct ICompositionSupportsSystemBackdrop {
    pub base__: ::windows_sys::core::IInspectable,
    pub SystemBackdrop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSystemBackdrop: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionSupportsSystemBackdrop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 964538340, data2: 46786, data3: 23481, data4: [149, 29, 245, 112, 125, 232, 183, 188] };
}
#[repr(C)]
pub struct ICompositionSurface {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionSurface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 354898957, data2: 17095, data3: 18342, data4: [164, 8, 102, 143, 121, 169, 13, 251] };
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
impl ::windows_sys::core::Interface for ICompositionSurfaceBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2902551929, data2: 7756, data3: 19469, data4: [156, 41, 131, 51, 140, 135, 193, 98] };
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
impl ::windows_sys::core::Interface for ICompositionSurfaceBrush2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3530650837, data2: 25845, data3: 18066, data4: [157, 199, 113, 182, 29, 126, 88, 128] };
}
#[repr(C)]
pub struct ICompositionSurfaceBrush3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SnapToPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSnapToPixels: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionSurfaceBrush3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1426829961, data2: 8160, data3: 17125, data4: [129, 149, 30, 239, 168, 127, 240, 142] };
}
#[repr(C)]
pub struct ICompositionSurfaceFacade {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetRealSurface: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionSurfaceFacade {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3759547080, data2: 9010, data3: 21959, data4: [136, 104, 167, 49, 44, 92, 34, 157] };
}
#[repr(C)]
pub struct ICompositionTarget {
    pub base__: ::windows_sys::core::IInspectable,
    pub Root: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRoot: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2713626810, data2: 55078, data3: 18019, data4: [129, 41, 107, 94, 121, 39, 255, 166] };
}
#[repr(C)]
pub struct ICompositionTargetFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionTargetFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2479725867, data2: 34070, data3: 19220, data4: [168, 206, 244, 158, 33, 25, 236, 66] };
}
#[repr(C)]
pub struct ICompositionTransform {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2094351657, data2: 64493, data3: 16658, data4: [171, 197, 24, 89, 6, 221, 146, 124] };
}
#[repr(C)]
pub struct ICompositionTransformFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionTransformFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2863581734, data2: 49481, data3: 20858, data4: [143, 114, 107, 255, 122, 101, 206, 8] };
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
impl ::windows_sys::core::Interface for ICompositionViewBox {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3024142087, data2: 1679, data3: 17719, data4: [132, 198, 78, 203, 224, 25, 225, 244] };
}
#[repr(C)]
pub struct ICompositionVirtualDrawingSurface {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics")]
    pub Trim: unsafe extern "system" fn(this: *mut *mut Self, rects_array_size: u32, rects: *const super::super::Graphics::RectInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Trim: usize,
}
impl ::windows_sys::core::Interface for ICompositionVirtualDrawingSurface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2848163035, data2: 34624, data3: 20372, data4: [139, 157, 182, 133, 33, 231, 134, 61] };
}
#[repr(C)]
pub struct ICompositionVirtualDrawingSurfaceFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionVirtualDrawingSurfaceFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1734742124, data2: 54635, data3: 19017, data4: [177, 223, 80, 118, 160, 98, 7, 104] };
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
impl ::windows_sys::core::Interface for ICompositionVisualSurface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2988759043, data2: 20334, data3: 19007, data4: [140, 174, 61, 193, 205, 167, 79, 198] };
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
impl ::windows_sys::core::Interface for ICompositor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3020147280, data2: 32652, data3: 20099, data4: [152, 95, 204, 69, 6, 0, 54, 216] };
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
impl ::windows_sys::core::Interface for ICompositor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1934655964, data2: 24100, data3: 17882, data4: [163, 143, 227, 44, 195, 73, 169, 160] };
}
#[repr(C)]
pub struct ICompositor3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateHostBackdropBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositor3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3386740464, data2: 28337, data3: 20028, data4: [166, 88, 103, 93, 156, 100, 212, 171] };
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
impl ::windows_sys::core::Interface for ICompositor4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2923947914, data2: 30992, data3: 17445, data4: [164, 130, 160, 91, 117, 138, 220, 233] };
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
impl ::windows_sys::core::Interface for ICompositor5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1223307693, data2: 32717, data3: 16502, data4: [167, 156, 144, 204, 75, 133, 44, 155] };
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
impl ::windows_sys::core::Interface for ICompositor6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2050536125, data2: 52936, data3: 20203, data4: [131, 15, 216, 208, 122, 237, 235, 195] };
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
impl ::windows_sys::core::Interface for ICompositor7 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3544727469, data2: 39442, data3: 21434, data4: [191, 200, 136, 183, 255, 121, 119, 198] };
}
#[repr(C)]
pub struct ICompositorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxGlobalPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub MinGlobalPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 135117118, data2: 4638, data3: 19863, data4: [139, 116, 29, 252, 249, 25, 135, 234] };
}
#[repr(C)]
pub struct ICompositorWithBlurredWallpaperBackdropBrush {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryCreateBlurredWallpaperBackdropBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositorWithBlurredWallpaperBackdropBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 227520912, data2: 61730, data3: 23437, data4: [159, 221, 84, 59, 13, 142, 183, 243] };
}
#[repr(C)]
pub struct ICompositorWithProjectedShadow {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateProjectedShadowCaster: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateProjectedShadow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateProjectedShadowReceiver: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositorWithProjectedShadow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2732995342, data2: 35424, data3: 23096, data4: [187, 133, 180, 78, 169, 1, 103, 124] };
}
#[repr(C)]
pub struct ICompositorWithRadialGradient {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateRadialGradientBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositorWithRadialGradient {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2562310567, data2: 36465, data3: 19283, data4: [180, 168, 105, 186, 93, 25, 220, 91] };
}
#[repr(C)]
pub struct ICompositorWithVisualSurface {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateVisualSurface: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositorWithVisualSurface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3483461003, data2: 291, data3: 17745, data4: [136, 145, 137, 189, 204, 64, 50, 43] };
}
#[repr(C)]
pub struct IContainerVisual {
    pub base__: ::windows_sys::core::IInspectable,
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContainerVisual {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 49724532, data2: 60704, data3: 18291, data4: [175, 230, 212, 155, 74, 147, 219, 50] };
}
#[repr(C)]
pub struct IContainerVisualFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IContainerVisualFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 56862299, data2: 51162, data3: 19866, data4: [149, 244, 105, 181, 200, 223, 103, 11] };
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
impl ::windows_sys::core::Interface for ICubicBezierEasingFunction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 842335846, data2: 49640, data3: 17657, data4: [150, 184, 201, 138, 207, 10, 230, 152] };
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
impl ::windows_sys::core::Interface for IDelegatedInkTrailVisual {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2238603441, data2: 57771, data3: 23331, data4: [142, 61, 213, 19, 242, 33, 201, 152] };
}
#[repr(C)]
pub struct IDelegatedInkTrailVisualStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateForSwapChain: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDelegatedInkTrailVisualStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 229600213, data2: 17094, data3: 21852, data4: [146, 103, 224, 172, 102, 58, 248, 54] };
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
impl ::windows_sys::core::Interface for IDistantLight {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 831322876, data2: 23779, data3: 19285, data4: [171, 93, 7, 160, 3, 83, 172, 153] };
}
#[repr(C)]
pub struct IDistantLight2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Intensity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetIntensity: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDistantLight2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3687688732, data2: 10571, data3: 18647, data4: [182, 14, 118, 223, 100, 170, 57, 43] };
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
impl ::windows_sys::core::Interface for IDropShadow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3415702535, data2: 41300, data3: 18513, data4: [133, 231, 168, 146, 76, 132, 250, 216] };
}
#[repr(C)]
pub struct IDropShadow2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourcePolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionDropShadowSourcePolicy) -> ::windows_sys::core::HRESULT,
    pub SetSourcePolicy: unsafe extern "system" fn(this: *mut *mut Self, value: CompositionDropShadowSourcePolicy) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDropShadow2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1816271036, data2: 5561, data3: 19501, data4: [141, 74, 7, 103, 223, 17, 151, 122] };
}
#[repr(C)]
pub struct IElasticEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
    pub Oscillations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Springiness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IElasticEasingFunction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1725850245, data2: 1358, data3: 21908, data4: [132, 117, 194, 44, 181, 31, 27, 213] };
}
#[repr(C)]
pub struct IExponentialEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
    pub Exponent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IExponentialEasingFunction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1870469713, data2: 39122, data3: 22072, data4: [163, 74, 0, 72, 101, 84, 199, 80] };
}
#[repr(C)]
pub struct IExpressionAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Expression: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetExpression: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IExpressionAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1791775793, data2: 32061, data3: 19443, data4: [171, 182, 244, 75, 220, 72, 136, 193] };
}
#[repr(C)]
pub struct IImplicitAnimationCollection {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IImplicitAnimationCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 93889535, data2: 2706, data3: 19613, data4: [164, 39, 178, 85, 25, 37, 13, 191] };
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
impl ::windows_sys::core::Interface for IInsetClip {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 510912071, data2: 33991, data3: 18298, data4: [180, 116, 88, 128, 224, 68, 46, 21] };
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
impl ::windows_sys::core::Interface for IKeyFrameAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 309231394, data2: 15081, data3: 17728, data4: [154, 138, 222, 174, 138, 74, 74, 132] };
}
#[repr(C)]
pub struct IKeyFrameAnimation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AnimationDirection) -> ::windows_sys::core::HRESULT,
    pub SetDirection: unsafe extern "system" fn(this: *mut *mut Self, value: AnimationDirection) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKeyFrameAnimation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4105472187, data2: 10560, data3: 20160, data4: [164, 26, 235, 109, 128, 26, 47, 24] };
}
#[repr(C)]
pub struct IKeyFrameAnimation3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DelayBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AnimationDelayBehavior) -> ::windows_sys::core::HRESULT,
    pub SetDelayBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: AnimationDelayBehavior) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKeyFrameAnimation3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2220617908, data2: 55518, data3: 17967, data4: [135, 83, 200, 13, 67, 198, 255, 90] };
}
#[repr(C)]
pub struct IKeyFrameAnimationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IKeyFrameAnimationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3204973560, data2: 28970, data3: 20417, data4: [140, 135, 151, 8, 89, 237, 141, 46] };
}
#[repr(C)]
pub struct ILayerVisual {
    pub base__: ::windows_sys::core::IInspectable,
    pub Effect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEffect: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILayerVisual {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2944678277, data2: 1092, data3: 18567, data4: [142, 131, 180, 11, 37, 63, 130, 44] };
}
#[repr(C)]
pub struct ILayerVisual2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Shadow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetShadow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILayerVisual2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2566500075, data2: 28451, data3: 18929, data4: [144, 177, 31, 89, 161, 79, 188, 227] };
}
#[repr(C)]
pub struct ILinearEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ILinearEasingFunction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2483066714, data2: 51110, data3: 18099, data4: [172, 247, 26, 38, 138, 10, 17, 125] };
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
impl ::windows_sys::core::Interface for INaturalMotionAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1133371693, data2: 30363, data3: 18465, data4: [169, 73, 40, 74, 101, 71, 232, 115] };
}
#[repr(C)]
pub struct INaturalMotionAnimationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for INaturalMotionAnimationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4114270982, data2: 53098, data3: 17287, data4: [163, 254, 82, 33, 243, 231, 224, 224] };
}
#[repr(C)]
pub struct IPathKeyFrameAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub InsertKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, path: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, path: *mut ::core::ffi::c_void, easingfunction: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPathKeyFrameAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2634881225, data2: 5494, data3: 19263, data4: [190, 96, 29, 80, 49, 245, 231, 27] };
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
impl ::windows_sys::core::Interface for IPointLight {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2978301363, data2: 3162, data3: 19120, data4: [190, 220, 79, 53, 70, 148, 130, 114] };
}
#[repr(C)]
pub struct IPointLight2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Intensity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetIntensity: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPointLight2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4025061164, data2: 1656, data3: 20329, data4: [177, 100, 168, 16, 217, 149, 188, 183] };
}
#[repr(C)]
pub struct IPointLight3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MinAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMinAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub MaxAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMaxAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPointLight3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1275757415, data2: 54505, data3: 18058, data4: [135, 174, 123, 164, 58, 178, 148, 133] };
}
#[repr(C)]
pub struct IPowerEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
    pub Power: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPowerEasingFunction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3288290262, data2: 5003, data3: 22549, data4: [137, 26, 183, 246, 21, 204, 197, 99] };
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
impl ::windows_sys::core::Interface for IQuaternionKeyFrameAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1078876213, data2: 60662, data3: 16960, data4: [133, 32, 103, 18, 121, 207, 54, 188] };
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
impl ::windows_sys::core::Interface for IRectangleClip {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3018282142, data2: 180, data3: 23379, data4: [139, 232, 53, 63, 108, 67, 49, 1] };
}
#[repr(C)]
pub struct IRedirectVisual {
    pub base__: ::windows_sys::core::IInspectable,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRedirectVisual {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2361844544, data2: 35701, data3: 21538, data4: [176, 111, 9, 255, 233, 248, 97, 126] };
}
#[repr(C)]
pub struct IRenderingDeviceReplacedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GraphicsDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRenderingDeviceReplacedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 976333949, data2: 10431, data3: 20090, data4: [133, 36, 113, 103, 157, 72, 15, 56] };
}
#[repr(C)]
pub struct IScalarKeyFrameAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub InsertKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: f32) -> ::windows_sys::core::HRESULT,
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, normalizedprogresskey: f32, value: f32, easingfunction: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IScalarKeyFrameAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2921893801, data2: 9516, data3: 19349, data4: [167, 37, 191, 133, 227, 128, 0, 161] };
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
impl ::windows_sys::core::Interface for IScalarNaturalMotionAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2494121345, data2: 49042, data3: 18779, data4: [181, 189, 210, 198, 89, 67, 7, 55] };
}
#[repr(C)]
pub struct IScalarNaturalMotionAnimationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IScalarNaturalMotionAnimationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2203755772, data2: 26396, data3: 16861, data4: [175, 72, 174, 141, 239, 139, 21, 41] };
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
impl ::windows_sys::core::Interface for IShapeVisual {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4072477635, data2: 47742, data3: 19215, data4: [145, 38, 255, 183, 83, 107, 129, 118] };
}
#[repr(C)]
pub struct ISineEasingFunction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CompositionEasingFunctionMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISineEasingFunction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4055177407, data2: 38243, data3: 21620, data4: [189, 19, 68, 178, 223, 75, 29, 88] };
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
impl ::windows_sys::core::Interface for ISpotLight {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1520427635, data2: 17569, data3: 20373, data4: [164, 34, 143, 165, 17, 107, 219, 68] };
}
#[repr(C)]
pub struct ISpotLight2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InnerConeIntensity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInnerConeIntensity: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub OuterConeIntensity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetOuterConeIntensity: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpotLight2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1693344094, data2: 1670, data3: 19946, data4: [169, 232, 188, 58, 140, 112, 20, 89] };
}
#[repr(C)]
pub struct ISpotLight3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MinAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMinAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub MaxAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMaxAttenuationCutoff: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpotLight3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3838852842, data2: 4895, data3: 18446, data4: [133, 158, 184, 39, 5, 183, 67, 96] };
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
impl ::windows_sys::core::Interface for ISpringScalarNaturalMotionAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 91400543, data2: 14329, data3: 20414, data4: [184, 123, 92, 208, 58, 137, 80, 28] };
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
impl ::windows_sys::core::Interface for ISpringVector2NaturalMotionAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 603231413, data2: 61043, data3: 20239, data4: [164, 35, 64, 43, 148, 109, 244, 179] };
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
impl ::windows_sys::core::Interface for ISpringVector3NaturalMotionAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1820805599, data2: 54651, data3: 18324, data4: [142, 45, 206, 203, 17, 225, 148, 229] };
}
#[repr(C)]
pub struct ISpriteVisual {
    pub base__: ::windows_sys::core::IInspectable,
    pub Brush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpriteVisual {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 148919681, data2: 6865, data3: 20375, data4: [151, 87, 64, 45, 118, 228, 35, 59] };
}
#[repr(C)]
pub struct ISpriteVisual2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Shadow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetShadow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpriteVisual2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1485608548, data2: 39290, data3: 18512, data4: [145, 254, 83, 203, 88, 248, 28, 233] };
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
impl ::windows_sys::core::Interface for IStepEasingFunction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3502942027, data2: 22028, data3: 18955, data4: [165, 246, 32, 108, 168, 195, 236, 214] };
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
impl ::windows_sys::core::Interface for IVector2KeyFrameAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3745596693, data2: 20009, data3: 20241, data4: [181, 94, 191, 42, 110, 179, 98, 148] };
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
impl ::windows_sys::core::Interface for IVector2NaturalMotionAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 255724413, data2: 58642, data3: 18333, data4: [160, 12, 119, 201, 58, 48, 163, 149] };
}
#[repr(C)]
pub struct IVector2NaturalMotionAnimationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IVector2NaturalMotionAnimationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2356477793, data2: 1889, data3: 18594, data4: [189, 219, 106, 252, 197, 43, 137, 216] };
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
impl ::windows_sys::core::Interface for IVector3KeyFrameAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3355680170, data2: 41601, data3: 17346, data4: [167, 61, 182, 142, 60, 83, 60, 64] };
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
impl ::windows_sys::core::Interface for IVector3NaturalMotionAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2618754092, data2: 58058, data3: 17837, data4: [150, 158, 78, 120, 183, 185, 173, 65] };
}
#[repr(C)]
pub struct IVector3NaturalMotionAnimationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IVector3NaturalMotionAnimationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 564665647, data2: 2176, data3: 17787, data4: [172, 135, 182, 9, 1, 140, 135, 109] };
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
impl ::windows_sys::core::Interface for IVector4KeyFrameAnimation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 609719387, data2: 44509, data3: 17285, data4: [150, 6, 182, 163, 213, 228, 225, 185] };
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
impl ::windows_sys::core::Interface for IVisual {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 293478445, data2: 43097, data3: 19593, data4: [135, 59, 194, 170, 86, 103, 136, 227] };
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
impl ::windows_sys::core::Interface for IVisual2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 810726929, data2: 22211, data3: 19518, data4: [139, 243, 246, 225, 173, 71, 63, 6] };
}
#[repr(C)]
pub struct IVisual3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsHitTestVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHitTestVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVisual3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 817780749, data2: 62646, data3: 19127, data4: [128, 221, 55, 56, 203, 172, 159, 44] };
}
#[repr(C)]
pub struct IVisual4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPixelSnappingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPixelSnappingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVisual4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2490810129, data2: 57931, data3: 23545, data4: [158, 190, 98, 116, 16, 155, 39, 17] };
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
impl ::windows_sys::core::Interface for IVisualCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2339656965, data2: 64830, data3: 19096, data4: [132, 168, 233, 73, 70, 140, 107, 203] };
}
#[repr(C)]
pub struct IVisualElement {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IVisualElement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 31868434, data2: 7554, data3: 17140, data4: [142, 63, 167, 34, 222, 211, 63, 199] };
}
#[repr(C)]
pub struct IVisualElement2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetVisualInternal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVisualElement2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2570774688, data2: 24663, data3: 24128, data4: [145, 140, 224, 110, 11, 126, 124, 100] };
}
#[repr(C)]
pub struct IVisualFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IVisualFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2903505214, data2: 46338, data3: 20149, data4: [135, 180, 154, 56, 167, 29, 1, 55] };
}
#[repr(C)]
pub struct IVisualUnorderedCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, newvisual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVisualUnorderedCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 865053296, data2: 21704, data3: 16551, data4: [128, 41, 201, 206, 235, 10, 162, 80] };
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
