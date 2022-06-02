pub type AddDeleteThemeTransition = *mut ::core::ffi::c_void;
pub type BackEase = *mut ::core::ffi::c_void;
pub type BasicConnectedAnimationConfiguration = *mut ::core::ffi::c_void;
pub type BeginStoryboard = *mut ::core::ffi::c_void;
pub type BounceEase = *mut ::core::ffi::c_void;
pub type CircleEase = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media_Animation\"`*"]
#[repr(transparent)]
pub struct ClockState(pub i32);
impl ClockState {
    pub const Active: Self = Self(0i32);
    pub const Filling: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
}
impl ::core::marker::Copy for ClockState {}
impl ::core::clone::Clone for ClockState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ColorAnimation = *mut ::core::ffi::c_void;
pub type ColorAnimationUsingKeyFrames = *mut ::core::ffi::c_void;
pub type ColorKeyFrame = *mut ::core::ffi::c_void;
pub type ColorKeyFrameCollection = *mut ::core::ffi::c_void;
pub type CommonNavigationTransitionInfo = *mut ::core::ffi::c_void;
pub type ConnectedAnimation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media_Animation\"`*"]
#[repr(transparent)]
pub struct ConnectedAnimationComponent(pub i32);
impl ConnectedAnimationComponent {
    pub const OffsetX: Self = Self(0i32);
    pub const OffsetY: Self = Self(1i32);
    pub const CrossFade: Self = Self(2i32);
    pub const Scale: Self = Self(3i32);
}
impl ::core::marker::Copy for ConnectedAnimationComponent {}
impl ::core::clone::Clone for ConnectedAnimationComponent {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ConnectedAnimationConfiguration = *mut ::core::ffi::c_void;
pub type ConnectedAnimationService = *mut ::core::ffi::c_void;
pub type ContentThemeTransition = *mut ::core::ffi::c_void;
pub type ContinuumNavigationTransitionInfo = *mut ::core::ffi::c_void;
pub type CubicEase = *mut ::core::ffi::c_void;
pub type DirectConnectedAnimationConfiguration = *mut ::core::ffi::c_void;
pub type DiscreteColorKeyFrame = *mut ::core::ffi::c_void;
pub type DiscreteDoubleKeyFrame = *mut ::core::ffi::c_void;
pub type DiscreteObjectKeyFrame = *mut ::core::ffi::c_void;
pub type DiscretePointKeyFrame = *mut ::core::ffi::c_void;
pub type DoubleAnimation = *mut ::core::ffi::c_void;
pub type DoubleAnimationUsingKeyFrames = *mut ::core::ffi::c_void;
pub type DoubleKeyFrame = *mut ::core::ffi::c_void;
pub type DoubleKeyFrameCollection = *mut ::core::ffi::c_void;
pub type DragItemThemeAnimation = *mut ::core::ffi::c_void;
pub type DragOverThemeAnimation = *mut ::core::ffi::c_void;
pub type DrillInNavigationTransitionInfo = *mut ::core::ffi::c_void;
pub type DrillInThemeAnimation = *mut ::core::ffi::c_void;
pub type DrillOutThemeAnimation = *mut ::core::ffi::c_void;
pub type DropTargetItemThemeAnimation = *mut ::core::ffi::c_void;
pub type EasingColorKeyFrame = *mut ::core::ffi::c_void;
pub type EasingDoubleKeyFrame = *mut ::core::ffi::c_void;
pub type EasingFunctionBase = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media_Animation\"`*"]
#[repr(transparent)]
pub struct EasingMode(pub i32);
impl EasingMode {
    pub const EaseOut: Self = Self(0i32);
    pub const EaseIn: Self = Self(1i32);
    pub const EaseInOut: Self = Self(2i32);
}
impl ::core::marker::Copy for EasingMode {}
impl ::core::clone::Clone for EasingMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EasingPointKeyFrame = *mut ::core::ffi::c_void;
pub type EdgeUIThemeTransition = *mut ::core::ffi::c_void;
pub type ElasticEase = *mut ::core::ffi::c_void;
pub type EntranceNavigationTransitionInfo = *mut ::core::ffi::c_void;
pub type EntranceThemeTransition = *mut ::core::ffi::c_void;
pub type ExponentialEase = *mut ::core::ffi::c_void;
pub type FadeInThemeAnimation = *mut ::core::ffi::c_void;
pub type FadeOutThemeAnimation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media_Animation\"`*"]
#[repr(transparent)]
pub struct FillBehavior(pub i32);
impl FillBehavior {
    pub const HoldEnd: Self = Self(0i32);
    pub const Stop: Self = Self(1i32);
}
impl ::core::marker::Copy for FillBehavior {}
impl ::core::clone::Clone for FillBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GravityConnectedAnimationConfiguration = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAddDeleteThemeTransition {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IBackEase {
    pub base__: ::windows_sys::core::IInspectable,
    pub Amplitude: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetAmplitude: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackEaseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AmplitudeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBasicConnectedAnimationConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IBasicConnectedAnimationConfigurationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBeginStoryboard {
    pub base__: ::windows_sys::core::IInspectable,
    pub Storyboard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetStoryboard: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBeginStoryboardStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub StoryboardProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBounceEase {
    pub base__: ::windows_sys::core::IInspectable,
    pub Bounces: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBounces: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Bounciness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetBounciness: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBounceEaseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub BouncesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BouncinessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICircleEase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IColorAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub From: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    From: usize,
    #[cfg(feature = "Foundation")]
    pub SetFrom: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFrom: usize,
    #[cfg(feature = "Foundation")]
    pub To: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    To: usize,
    #[cfg(feature = "Foundation")]
    pub SetTo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTo: usize,
    #[cfg(feature = "Foundation")]
    pub By: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    By: usize,
    #[cfg(feature = "Foundation")]
    pub SetBy: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBy: usize,
    pub EasingFunction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ToProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ByProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EasingFunctionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorAnimationUsingKeyFrames {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub KeyFrames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    KeyFrames: usize,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorAnimationUsingKeyFramesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub KeyTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut KeyTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeyTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetKeyTime: unsafe extern "system" fn(this: *mut *mut Self, value: KeyTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetKeyTime: usize,
}
#[repr(C)]
pub struct IColorKeyFrameFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorKeyFrameStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyTimeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommonNavigationTransitionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsStaggeringEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsStaggeringEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommonNavigationTransitionInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsStaggeringEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsStaggerElementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsStaggerElement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsStaggerElement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConnectedAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    pub TryStart: unsafe extern "system" fn(this: *mut *mut Self, destination: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConnectedAnimation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsScaleAnimationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsScaleAnimationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TryStartWithCoordinatedElements: unsafe extern "system" fn(this: *mut *mut Self, destination: *mut ::core::ffi::c_void, coordinatedelements: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryStartWithCoordinatedElements: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetAnimationComponent: unsafe extern "system" fn(this: *mut *mut Self, component: ConnectedAnimationComponent, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetAnimationComponent: usize,
}
#[repr(C)]
pub struct IConnectedAnimation3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetConfiguration: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConnectedAnimationConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IConnectedAnimationConfigurationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IConnectedAnimationService {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DefaultDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDefaultDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDefaultDuration: usize,
    #[cfg(feature = "UI_Composition")]
    pub DefaultEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    DefaultEasingFunction: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetDefaultEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetDefaultEasingFunction: usize,
    pub PrepareToAnimate: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAnimation: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConnectedAnimationServiceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentThemeTransition {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentThemeTransitionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContinuumNavigationTransitionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExitElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetExitElement: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContinuumNavigationTransitionInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExitElementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsEntranceElementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsEntranceElement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEntranceElement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsExitElementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsExitElement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsExitElement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub ExitElementContainerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub GetExitElementContainer: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    GetExitElementContainer: usize,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub SetExitElementContainer: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    SetExitElementContainer: usize,
}
#[repr(C)]
pub struct ICubicEase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDirectConnectedAnimationConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDirectConnectedAnimationConfigurationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDiscreteColorKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDiscreteDoubleKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDiscreteObjectKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDiscretePointKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDoubleAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub From: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    From: usize,
    #[cfg(feature = "Foundation")]
    pub SetFrom: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFrom: usize,
    #[cfg(feature = "Foundation")]
    pub To: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    To: usize,
    #[cfg(feature = "Foundation")]
    pub SetTo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTo: usize,
    #[cfg(feature = "Foundation")]
    pub By: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    By: usize,
    #[cfg(feature = "Foundation")]
    pub SetBy: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBy: usize,
    pub EasingFunction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDoubleAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ToProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ByProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EasingFunctionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDoubleAnimationUsingKeyFrames {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub KeyFrames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    KeyFrames: usize,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDoubleAnimationUsingKeyFramesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDoubleKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub KeyTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut KeyTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeyTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetKeyTime: unsafe extern "system" fn(this: *mut *mut Self, value: KeyTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetKeyTime: usize,
}
#[repr(C)]
pub struct IDoubleKeyFrameFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDoubleKeyFrameStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyTimeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragItemThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragItemThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragOverThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ToOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetToOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    Direction: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetDirection: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetDirection: usize,
}
#[repr(C)]
pub struct IDragOverThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ToOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DirectionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDrillInNavigationTransitionInfo {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDrillInThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub EntranceTargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetEntranceTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EntranceTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEntranceTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExitTargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetExitTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ExitTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetExitTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDrillInThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EntranceTargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EntranceTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExitTargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExitTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDrillOutThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub EntranceTargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetEntranceTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EntranceTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEntranceTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExitTargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetExitTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ExitTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetExitTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDrillOutThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EntranceTargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EntranceTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExitTargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExitTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDropTargetItemThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDropTargetItemThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEasingColorKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub EasingFunction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEasingColorKeyFrameStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EasingFunctionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEasingDoubleKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub EasingFunction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEasingDoubleKeyFrameStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EasingFunctionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEasingFunctionBase {
    pub base__: ::windows_sys::core::IInspectable,
    pub EasingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EasingMode) -> ::windows_sys::core::HRESULT,
    pub SetEasingMode: unsafe extern "system" fn(this: *mut *mut Self, value: EasingMode) -> ::windows_sys::core::HRESULT,
    pub Ease: unsafe extern "system" fn(this: *mut *mut Self, normalizedtime: f64, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEasingFunctionBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IEasingFunctionBaseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EasingModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEasingPointKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub EasingFunction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEasingPointKeyFrameStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EasingFunctionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEdgeUIThemeTransition {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub Edge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    Edge: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetEdge: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetEdge: usize,
}
#[repr(C)]
pub struct IEdgeUIThemeTransitionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EdgeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IElasticEase {
    pub base__: ::windows_sys::core::IInspectable,
    pub Oscillations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetOscillations: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Springiness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetSpringiness: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IElasticEaseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OscillationsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SpringinessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEntranceNavigationTransitionInfo {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IEntranceNavigationTransitionInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTargetElementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsTargetElement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTargetElement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEntranceThemeTransition {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFromHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub FromVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFromVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub IsStaggeringEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsStaggeringEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEntranceThemeTransitionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromVerticalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsStaggeringEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IExponentialEase {
    pub base__: ::windows_sys::core::IInspectable,
    pub Exponent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetExponent: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IExponentialEaseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExponentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFadeInThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFadeInThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFadeOutThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFadeOutThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGravityConnectedAnimationConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGravityConnectedAnimationConfiguration2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsShadowEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsShadowEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGravityConnectedAnimationConfigurationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeySpline {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ControlPoint1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ControlPoint1: usize,
    #[cfg(feature = "Foundation")]
    pub SetControlPoint1: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetControlPoint1: usize,
    #[cfg(feature = "Foundation")]
    pub ControlPoint2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ControlPoint2: usize,
    #[cfg(feature = "Foundation")]
    pub SetControlPoint2: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetControlPoint2: usize,
}
#[repr(C)]
pub struct IKeyTimeHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IKeyTimeHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromTimeSpan: unsafe extern "system" fn(this: *mut *mut Self, timespan: super::super::super::super::Foundation::TimeSpan, result__: *mut KeyTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromTimeSpan: usize,
}
#[repr(C)]
pub struct ILinearColorKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ILinearDoubleKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ILinearPointKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct INavigationThemeTransition {
    pub base__: ::windows_sys::core::IInspectable,
    pub DefaultNavigationTransitionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDefaultNavigationTransitionInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationThemeTransitionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DefaultNavigationTransitionInfoProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationTransitionInfo {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct INavigationTransitionInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationTransitionInfoOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetNavigationStateCore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNavigationStateCore: unsafe extern "system" fn(this: *mut *mut Self, navigationstate: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IObjectAnimationUsingKeyFrames {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub KeyFrames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    KeyFrames: usize,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IObjectAnimationUsingKeyFramesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IObjectKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub KeyTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut KeyTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeyTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetKeyTime: unsafe extern "system" fn(this: *mut *mut Self, value: KeyTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetKeyTime: usize,
}
#[repr(C)]
pub struct IObjectKeyFrameFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IObjectKeyFrameStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyTimeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaneThemeTransition {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub Edge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    Edge: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetEdge: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetEdge: usize,
}
#[repr(C)]
pub struct IPaneThemeTransitionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EdgeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub From: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    From: usize,
    #[cfg(feature = "Foundation")]
    pub SetFrom: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFrom: usize,
    #[cfg(feature = "Foundation")]
    pub To: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    To: usize,
    #[cfg(feature = "Foundation")]
    pub SetTo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTo: usize,
    #[cfg(feature = "Foundation")]
    pub By: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    By: usize,
    #[cfg(feature = "Foundation")]
    pub SetBy: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBy: usize,
    pub EasingFunction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ToProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ByProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EasingFunctionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointAnimationUsingKeyFrames {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub KeyFrames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    KeyFrames: usize,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointAnimationUsingKeyFramesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
    #[cfg(feature = "Foundation")]
    pub KeyTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut KeyTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeyTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetKeyTime: unsafe extern "system" fn(this: *mut *mut Self, value: KeyTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetKeyTime: usize,
}
#[repr(C)]
pub struct IPointKeyFrameFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointKeyFrameStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyTimeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointerDownThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointerDownThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointerUpThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointerUpThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPopInThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FromHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFromHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub FromVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFromVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPopInThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromVerticalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPopOutThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPopOutThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPopupThemeTransition {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFromHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub FromVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFromVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPopupThemeTransitionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromVerticalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPowerEase {
    pub base__: ::windows_sys::core::IInspectable,
    pub Power: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetPower: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPowerEaseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PowerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IQuadraticEase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IQuarticEase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IQuinticEase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IReorderThemeTransition {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRepeatBehaviorHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRepeatBehaviorHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Forever: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RepeatBehavior) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Forever: usize,
    #[cfg(feature = "Foundation")]
    pub FromCount: unsafe extern "system" fn(this: *mut *mut Self, count: f64, result__: *mut RepeatBehavior) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromCount: usize,
    #[cfg(feature = "Foundation")]
    pub FromDuration: unsafe extern "system" fn(this: *mut *mut Self, duration: super::super::super::super::Foundation::TimeSpan, result__: *mut RepeatBehavior) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromDuration: usize,
    #[cfg(feature = "Foundation")]
    pub GetHasCount: unsafe extern "system" fn(this: *mut *mut Self, target: RepeatBehavior, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetHasCount: usize,
    #[cfg(feature = "Foundation")]
    pub GetHasDuration: unsafe extern "system" fn(this: *mut *mut Self, target: RepeatBehavior, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetHasDuration: usize,
    #[cfg(feature = "Foundation")]
    pub Equals: unsafe extern "system" fn(this: *mut *mut Self, target: RepeatBehavior, value: RepeatBehavior, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Equals: usize,
}
#[repr(C)]
pub struct IRepositionThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FromHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFromHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub FromVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFromVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRepositionThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromVerticalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRepositionThemeTransition {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRepositionThemeTransition2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsStaggeringEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsStaggeringEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRepositionThemeTransitionStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsStaggeringEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISineEase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISlideNavigationTransitionInfo {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISlideNavigationTransitionInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Effect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SlideNavigationTransitionEffect) -> ::windows_sys::core::HRESULT,
    pub SetEffect: unsafe extern "system" fn(this: *mut *mut Self, value: SlideNavigationTransitionEffect) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISlideNavigationTransitionInfoStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EffectProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplineColorKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeySpline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetKeySpline: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplineColorKeyFrameStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeySplineProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplineDoubleKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeySpline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetKeySpline: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplineDoubleKeyFrameStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeySplineProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplinePointKeyFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeySpline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetKeySpline: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplinePointKeyFrameStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeySplineProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplitCloseThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenedTargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetOpenedTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OpenedTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOpenedTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClosedTargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetClosedTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ClosedTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetClosedTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContentTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContentTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenedLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOpenedLength: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ClosedLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetClosedLength: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub OffsetFromCenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOffsetFromCenter: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub ContentTranslationDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    ContentTranslationDirection: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetContentTranslationDirection: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetContentTranslationDirection: usize,
    pub ContentTranslationOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetContentTranslationOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplitCloseThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenedTargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenedTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClosedTargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClosedTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenedLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClosedLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OffsetFromCenterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTranslationDirectionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTranslationOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplitOpenThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenedTargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetOpenedTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OpenedTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOpenedTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClosedTargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetClosedTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ClosedTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetClosedTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContentTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContentTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenedLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOpenedLength: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ClosedLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetClosedLength: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub OffsetFromCenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOffsetFromCenter: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub ContentTranslationDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    ContentTranslationDirection: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetContentTranslationDirection: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetContentTranslationDirection: usize,
    pub ContentTranslationOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetContentTranslationOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISplitOpenThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenedTargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenedTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClosedTargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClosedTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenedLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClosedLengthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OffsetFromCenterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTranslationDirectionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentTranslationOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStoryboard {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    #[cfg(feature = "Foundation")]
    pub Seek: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Seek: usize,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Begin: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetCurrentState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ClockState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetCurrentTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentTime: usize,
    #[cfg(feature = "Foundation")]
    pub SeekAlignedToLastTick: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SeekAlignedToLastTick: usize,
    pub SkipToFill: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStoryboardStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetPropertyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, path: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTargetName: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, name: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTarget: unsafe extern "system" fn(this: *mut *mut Self, timeline: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISuppressNavigationTransitionInfo {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISwipeBackThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FromHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFromHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub FromVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFromVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwipeBackThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromVerticalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwipeHintThemeAnimation {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ToHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetToHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ToVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetToVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwipeHintThemeAnimationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ToHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ToVerticalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimeline {
    pub base__: ::windows_sys::core::IInspectable,
    pub AutoReverse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoReverse: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BeginTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeginTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetBeginTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBeginTime: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Duration) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Duration) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub SpeedRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetSpeedRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub FillBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FillBehavior) -> ::windows_sys::core::HRESULT,
    pub SetFillBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: FillBehavior) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RepeatBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RepeatBehavior) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RepeatBehavior: usize,
    #[cfg(feature = "Foundation")]
    pub SetRepeatBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: RepeatBehavior) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRepeatBehavior: usize,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
}
#[repr(C)]
pub struct ITimelineFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimelineStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowDependentAnimations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowDependentAnimations: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AutoReverseProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BeginTimeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DurationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SpeedRatioProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FillBehaviorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RepeatBehaviorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITransition {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITransitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
pub type KeySpline = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Media_Animation\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct KeyTime {
    pub TimeSpan: super::super::super::super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for KeyTime {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for KeyTime {
    fn clone(&self) -> Self {
        *self
    }
}
pub type KeyTimeHelper = *mut ::core::ffi::c_void;
pub type LinearColorKeyFrame = *mut ::core::ffi::c_void;
pub type LinearDoubleKeyFrame = *mut ::core::ffi::c_void;
pub type LinearPointKeyFrame = *mut ::core::ffi::c_void;
pub type NavigationThemeTransition = *mut ::core::ffi::c_void;
pub type NavigationTransitionInfo = *mut ::core::ffi::c_void;
pub type ObjectAnimationUsingKeyFrames = *mut ::core::ffi::c_void;
pub type ObjectKeyFrame = *mut ::core::ffi::c_void;
pub type ObjectKeyFrameCollection = *mut ::core::ffi::c_void;
pub type PaneThemeTransition = *mut ::core::ffi::c_void;
pub type PointAnimation = *mut ::core::ffi::c_void;
pub type PointAnimationUsingKeyFrames = *mut ::core::ffi::c_void;
pub type PointKeyFrame = *mut ::core::ffi::c_void;
pub type PointKeyFrameCollection = *mut ::core::ffi::c_void;
pub type PointerDownThemeAnimation = *mut ::core::ffi::c_void;
pub type PointerUpThemeAnimation = *mut ::core::ffi::c_void;
pub type PopInThemeAnimation = *mut ::core::ffi::c_void;
pub type PopOutThemeAnimation = *mut ::core::ffi::c_void;
pub type PopupThemeTransition = *mut ::core::ffi::c_void;
pub type PowerEase = *mut ::core::ffi::c_void;
pub type QuadraticEase = *mut ::core::ffi::c_void;
pub type QuarticEase = *mut ::core::ffi::c_void;
pub type QuinticEase = *mut ::core::ffi::c_void;
pub type ReorderThemeTransition = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Media_Animation\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct RepeatBehavior {
    pub Count: f64,
    pub Duration: super::super::super::super::Foundation::TimeSpan,
    pub Type: RepeatBehaviorType,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for RepeatBehavior {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for RepeatBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RepeatBehaviorHelper = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media_Animation\"`*"]
#[repr(transparent)]
pub struct RepeatBehaviorType(pub i32);
impl RepeatBehaviorType {
    pub const Count: Self = Self(0i32);
    pub const Duration: Self = Self(1i32);
    pub const Forever: Self = Self(2i32);
}
impl ::core::marker::Copy for RepeatBehaviorType {}
impl ::core::clone::Clone for RepeatBehaviorType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RepositionThemeAnimation = *mut ::core::ffi::c_void;
pub type RepositionThemeTransition = *mut ::core::ffi::c_void;
pub type SineEase = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media_Animation\"`*"]
#[repr(transparent)]
pub struct SlideNavigationTransitionEffect(pub i32);
impl SlideNavigationTransitionEffect {
    pub const FromBottom: Self = Self(0i32);
    pub const FromLeft: Self = Self(1i32);
    pub const FromRight: Self = Self(2i32);
}
impl ::core::marker::Copy for SlideNavigationTransitionEffect {}
impl ::core::clone::Clone for SlideNavigationTransitionEffect {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SlideNavigationTransitionInfo = *mut ::core::ffi::c_void;
pub type SplineColorKeyFrame = *mut ::core::ffi::c_void;
pub type SplineDoubleKeyFrame = *mut ::core::ffi::c_void;
pub type SplinePointKeyFrame = *mut ::core::ffi::c_void;
pub type SplitCloseThemeAnimation = *mut ::core::ffi::c_void;
pub type SplitOpenThemeAnimation = *mut ::core::ffi::c_void;
pub type Storyboard = *mut ::core::ffi::c_void;
pub type SuppressNavigationTransitionInfo = *mut ::core::ffi::c_void;
pub type SwipeBackThemeAnimation = *mut ::core::ffi::c_void;
pub type SwipeHintThemeAnimation = *mut ::core::ffi::c_void;
pub type Timeline = *mut ::core::ffi::c_void;
pub type TimelineCollection = *mut ::core::ffi::c_void;
pub type Transition = *mut ::core::ffi::c_void;
pub type TransitionCollection = *mut ::core::ffi::c_void;
