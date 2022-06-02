#[cfg(feature = "UI_Input_Core")]
pub mod Core;
#[cfg(feature = "UI_Input_Inking")]
pub mod Inking;
#[cfg(feature = "UI_Input_Preview")]
pub mod Preview;
#[cfg(feature = "UI_Input_Spatial")]
pub mod Spatial;
pub type AttachableInputObject = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Input\"`*"]
pub struct CrossSlideThresholds {
    pub SelectionStart: f32,
    pub SpeedBumpStart: f32,
    pub SpeedBumpEnd: f32,
    pub RearrangeStart: f32,
}
impl ::core::marker::Copy for CrossSlideThresholds {}
impl ::core::clone::Clone for CrossSlideThresholds {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CrossSlidingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct CrossSlidingState(pub i32);
impl CrossSlidingState {
    pub const Started: Self = Self(0i32);
    pub const Dragging: Self = Self(1i32);
    pub const Selecting: Self = Self(2i32);
    pub const SelectSpeedBumping: Self = Self(3i32);
    pub const SpeedBumping: Self = Self(4i32);
    pub const Rearranging: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl ::core::marker::Copy for CrossSlidingState {}
impl ::core::clone::Clone for CrossSlidingState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DraggingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct DraggingState(pub i32);
impl DraggingState {
    pub const Started: Self = Self(0i32);
    pub const Continuing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for DraggingState {}
impl ::core::clone::Clone for DraggingState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EdgeGesture = *mut ::core::ffi::c_void;
pub type EdgeGestureEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct EdgeGestureKind(pub i32);
impl EdgeGestureKind {
    pub const Touch: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
}
impl ::core::marker::Copy for EdgeGestureKind {}
impl ::core::clone::Clone for EdgeGestureKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct GazeInputAccessStatus(pub i32);
impl GazeInputAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for GazeInputAccessStatus {}
impl ::core::clone::Clone for GazeInputAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GestureRecognizer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct GestureSettings(pub u32);
impl GestureSettings {
    pub const None: Self = Self(0u32);
    pub const Tap: Self = Self(1u32);
    pub const DoubleTap: Self = Self(2u32);
    pub const Hold: Self = Self(4u32);
    pub const HoldWithMouse: Self = Self(8u32);
    pub const RightTap: Self = Self(16u32);
    pub const Drag: Self = Self(32u32);
    pub const ManipulationTranslateX: Self = Self(64u32);
    pub const ManipulationTranslateY: Self = Self(128u32);
    pub const ManipulationTranslateRailsX: Self = Self(256u32);
    pub const ManipulationTranslateRailsY: Self = Self(512u32);
    pub const ManipulationRotate: Self = Self(1024u32);
    pub const ManipulationScale: Self = Self(2048u32);
    pub const ManipulationTranslateInertia: Self = Self(4096u32);
    pub const ManipulationRotateInertia: Self = Self(8192u32);
    pub const ManipulationScaleInertia: Self = Self(16384u32);
    pub const CrossSlide: Self = Self(32768u32);
    pub const ManipulationMultipleFingerPanning: Self = Self(65536u32);
}
impl ::core::marker::Copy for GestureSettings {}
impl ::core::clone::Clone for GestureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HoldingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct HoldingState(pub i32);
impl HoldingState {
    pub const Started: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for HoldingState {}
impl ::core::clone::Clone for HoldingState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAttachableInputObject {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAttachableInputObjectFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICrossSlidingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub CrossSlidingState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CrossSlidingState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICrossSlidingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDraggingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub DraggingState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DraggingState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDraggingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEdgeGesture {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Starting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Starting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStarting: usize,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Canceled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Canceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCanceled: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCanceled: usize,
}
#[repr(C)]
pub struct IEdgeGestureEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EdgeGestureKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEdgeGestureStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGestureRecognizer {
    pub base__: ::windows_sys::core::IInspectable,
    pub GestureSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GestureSettings) -> ::windows_sys::core::HRESULT,
    pub SetGestureSettings: unsafe extern "system" fn(this: *mut *mut Self, value: GestureSettings) -> ::windows_sys::core::HRESULT,
    pub IsInertial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShowGestureFeedback: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowGestureFeedback: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PivotCenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PivotCenter: usize,
    #[cfg(feature = "Foundation")]
    pub SetPivotCenter: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPivotCenter: usize,
    pub PivotRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPivotRadius: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub InertiaTranslationDeceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInertiaTranslationDeceleration: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub InertiaRotationDeceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInertiaRotationDeceleration: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub InertiaExpansionDeceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInertiaExpansionDeceleration: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub InertiaTranslationDisplacement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInertiaTranslationDisplacement: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub InertiaRotationAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInertiaRotationAngle: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub InertiaExpansion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInertiaExpansion: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub ManipulationExact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetManipulationExact: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CrossSlideThresholds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CrossSlideThresholds) -> ::windows_sys::core::HRESULT,
    pub SetCrossSlideThresholds: unsafe extern "system" fn(this: *mut *mut Self, value: CrossSlideThresholds) -> ::windows_sys::core::HRESULT,
    pub CrossSlideHorizontally: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCrossSlideHorizontally: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CrossSlideExact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCrossSlideExact: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AutoProcessInertia: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoProcessInertia: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MouseWheelParameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanBeDoubleTap: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ProcessDownEvent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProcessMoveEvents: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProcessMoveEvents: usize,
    pub ProcessUpEvent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProcessMouseWheelEvent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, isshiftkeydown: bool, iscontrolkeydown: bool) -> ::windows_sys::core::HRESULT,
    pub ProcessInertia: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CompleteGesture: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Tapped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Tapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTapped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RightTapped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RightTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRightTapped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRightTapped: usize,
    #[cfg(feature = "Foundation")]
    pub Holding: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Holding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHolding: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHolding: usize,
    #[cfg(feature = "Foundation")]
    pub Dragging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Dragging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDragging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDragging: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationInertiaStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationInertiaStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationInertiaStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationInertiaStarting: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub CrossSliding: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CrossSliding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCrossSliding: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCrossSliding: usize,
}
#[repr(C)]
pub struct IGestureRecognizer2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TapMinContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTapMinContactCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub TapMaxContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTapMaxContactCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub HoldMinContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetHoldMinContactCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub HoldMaxContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetHoldMaxContactCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub HoldRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetHoldRadius: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HoldStartDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldStartDelay: usize,
    #[cfg(feature = "Foundation")]
    pub SetHoldStartDelay: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetHoldStartDelay: usize,
    pub TranslationMinContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTranslationMinContactCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub TranslationMaxContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTranslationMaxContactCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHoldingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub HoldingState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HoldingState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHoldingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInputActivationListener {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InputActivationState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InputActivationChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputActivationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInputActivationChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInputActivationChanged: usize,
}
#[repr(C)]
pub struct IInputActivationListenerActivationChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InputActivationState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyboardDeliveryInterceptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsInterceptionEnabledWhenInForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsInterceptionEnabledWhenInForeground: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub KeyDown: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    KeyDown: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyDown: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyDown: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub KeyUp: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    KeyUp: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyUp: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyUp: usize,
}
#[repr(C)]
pub struct IKeyboardDeliveryInterceptorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IManipulationCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Cumulative: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ManipulationDelta) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Cumulative: usize,
    #[cfg(feature = "Foundation")]
    pub Velocities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ManipulationVelocities) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Velocities: usize,
}
#[repr(C)]
pub struct IManipulationCompletedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IManipulationInertiaStartingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Delta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ManipulationDelta) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Delta: usize,
    #[cfg(feature = "Foundation")]
    pub Cumulative: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ManipulationDelta) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Cumulative: usize,
    #[cfg(feature = "Foundation")]
    pub Velocities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ManipulationVelocities) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Velocities: usize,
}
#[repr(C)]
pub struct IManipulationInertiaStartingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IManipulationStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Cumulative: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ManipulationDelta) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Cumulative: usize,
}
#[repr(C)]
pub struct IManipulationStartedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IManipulationUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Delta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ManipulationDelta) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Delta: usize,
    #[cfg(feature = "Foundation")]
    pub Cumulative: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ManipulationDelta) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Cumulative: usize,
    #[cfg(feature = "Foundation")]
    pub Velocities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ManipulationVelocities) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Velocities: usize,
}
#[repr(C)]
pub struct IManipulationUpdatedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMouseWheelParameters {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CharTranslation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CharTranslation: usize,
    #[cfg(feature = "Foundation")]
    pub SetCharTranslation: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCharTranslation: usize,
    pub DeltaScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDeltaScale: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub DeltaRotationAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDeltaRotationAngle: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PageTranslation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PageTranslation: usize,
    #[cfg(feature = "Foundation")]
    pub SetPageTranslation: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPageTranslation: usize,
}
#[repr(C)]
pub struct IPointerPoint {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDevice: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub RawPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RawPosition: usize,
    pub PointerId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub FrameId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub IsInContact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointerPointProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Pressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub IsInverted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsEraser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub XTilt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub YTilt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Twist: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContactRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContactRect: usize,
    #[cfg(feature = "Foundation")]
    pub ContactRectRaw: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContactRectRaw: usize,
    pub TouchConfidence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsLeftButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsRightButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsMiddleButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MouseWheelDelta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsHorizontalMouseWheel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPrimary: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsInRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsBarrelButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsXButton1Pressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsXButton2Pressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PointerUpdateKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PointerUpdateKind) -> ::windows_sys::core::HRESULT,
    pub HasUsage: unsafe extern "system" fn(this: *mut *mut Self, usagepage: u32, usageid: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetUsageValue: unsafe extern "system" fn(this: *mut *mut Self, usagepage: u32, usageid: u32, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointerPointProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ZDistance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ZDistance: usize,
}
#[repr(C)]
pub struct IPointerPointStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentPoint: unsafe extern "system" fn(this: *mut *mut Self, pointerid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntermediatePoints: unsafe extern "system" fn(this: *mut *mut Self, pointerid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntermediatePoints: usize,
    pub GetCurrentPointTransformed: unsafe extern "system" fn(this: *mut *mut Self, pointerid: u32, transform: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntermediatePointsTransformed: unsafe extern "system" fn(this: *mut *mut Self, pointerid: u32, transform: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntermediatePointsTransformed: usize,
}
#[repr(C)]
pub struct IPointerPointTransform {
    pub base__: ::windows_sys::core::IInspectable,
    pub Inverse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryTransform: unsafe extern "system" fn(this: *mut *mut Self, inpoint: super::super::Foundation::Point, outpoint: *mut super::super::Foundation::Point, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTransform: usize,
    #[cfg(feature = "Foundation")]
    pub TransformBounds: unsafe extern "system" fn(this: *mut *mut Self, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransformBounds: usize,
}
#[repr(C)]
pub struct IPointerVisualizationSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsContactFeedbackEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsContactFeedbackEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsBarrelButtonFeedbackEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsBarrelButtonFeedbackEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointerVisualizationSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadialController {
    pub base__: ::windows_sys::core::IInspectable,
    pub Menu: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RotationResolutionInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRotationResolutionInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub UseAutomaticHapticFeedback: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetUseAutomaticHapticFeedback: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ScreenContactStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenContactStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScreenContactStarted: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScreenContactStarted: usize,
    #[cfg(feature = "Foundation")]
    pub ScreenContactEnded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenContactEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScreenContactEnded: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScreenContactEnded: usize,
    #[cfg(feature = "Foundation")]
    pub ScreenContactContinued: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenContactContinued: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScreenContactContinued: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScreenContactContinued: usize,
    #[cfg(feature = "Foundation")]
    pub ControlLost: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ControlLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveControlLost: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveControlLost: usize,
    #[cfg(feature = "Foundation")]
    pub RotationChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RotationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRotationChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRotationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonClicked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveButtonClicked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub ControlAcquired: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ControlAcquired: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveControlAcquired: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveControlAcquired: usize,
}
#[repr(C)]
pub struct IRadialController2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonHolding: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonHolding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveButtonHolding: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveButtonHolding: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonReleased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveButtonReleased: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveButtonReleased: usize,
}
#[repr(C)]
pub struct IRadialControllerButtonClickedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadialControllerButtonClickedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[repr(C)]
pub struct IRadialControllerButtonHoldingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[repr(C)]
pub struct IRadialControllerButtonPressedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[repr(C)]
pub struct IRadialControllerButtonReleasedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[repr(C)]
pub struct IRadialControllerConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SetDefaultMenuItems: unsafe extern "system" fn(this: *mut *mut Self, buttons: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetDefaultMenuItems: usize,
    pub ResetToDefaultMenuItems: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub TrySelectDefaultMenuItem: unsafe extern "system" fn(this: *mut *mut Self, r#type: RadialControllerSystemMenuItemKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadialControllerConfiguration2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetActiveControllerWhenMenuIsSuppressed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ActiveControllerWhenMenuIsSuppressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIsMenuSuppressed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsMenuSuppressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadialControllerConfigurationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadialControllerConfigurationStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAppController: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIsAppControllerEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAppControllerEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadialControllerControlAcquiredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadialControllerControlAcquiredEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[repr(C)]
pub struct IRadialControllerMenu {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub GetSelectedMenuItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectMenuItem: unsafe extern "system" fn(this: *mut *mut Self, menuitem: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TrySelectPreviouslySelectedMenuItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadialControllerMenuItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Invoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Invoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInvoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInvoked: usize,
}
#[repr(C)]
pub struct IRadialControllerMenuItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromIcon: unsafe extern "system" fn(this: *mut *mut Self, displaytext: ::windows_sys::core::HSTRING, icon: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromIcon: usize,
    pub CreateFromKnownIcon: unsafe extern "system" fn(this: *mut *mut Self, displaytext: ::windows_sys::core::HSTRING, value: RadialControllerMenuKnownIcon, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadialControllerMenuItemStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromFontGlyph: unsafe extern "system" fn(this: *mut *mut Self, displaytext: ::windows_sys::core::HSTRING, glyph: ::windows_sys::core::HSTRING, fontfamily: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateFromFontGlyphWithUri: unsafe extern "system" fn(this: *mut *mut Self, displaytext: ::windows_sys::core::HSTRING, glyph: ::windows_sys::core::HSTRING, fontfamily: ::windows_sys::core::HSTRING, fonturi: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromFontGlyphWithUri: usize,
}
#[repr(C)]
pub struct IRadialControllerRotationChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RotationDeltaInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadialControllerRotationChangedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[repr(C)]
pub struct IRadialControllerScreenContact {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[repr(C)]
pub struct IRadialControllerScreenContactContinuedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadialControllerScreenContactContinuedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[repr(C)]
pub struct IRadialControllerScreenContactEndedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[repr(C)]
pub struct IRadialControllerScreenContactStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadialControllerScreenContactStartedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[repr(C)]
pub struct IRadialControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CreateForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRightTappedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[repr(C)]
pub struct IRightTappedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemButtonEventController {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SystemFunctionButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemFunctionButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemFunctionButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemFunctionButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub SystemFunctionButtonReleased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemFunctionButtonReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemFunctionButtonReleased: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemFunctionButtonReleased: usize,
    #[cfg(feature = "Foundation")]
    pub SystemFunctionLockChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemFunctionLockChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemFunctionLockChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemFunctionLockChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SystemFunctionLockIndicatorChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemFunctionLockIndicatorChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemFunctionLockIndicatorChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemFunctionLockIndicatorChanged: usize,
}
#[repr(C)]
pub struct ISystemButtonEventControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub CreateForDispatcherQueue: unsafe extern "system" fn(this: *mut *mut Self, queue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForDispatcherQueue: usize,
}
#[repr(C)]
pub struct ISystemFunctionButtonEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemFunctionLockChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub IsLocked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemFunctionLockIndicatorChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub IsIndicatorOn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITappedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub TapCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITappedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
pub type InputActivationListener = *mut ::core::ffi::c_void;
pub type InputActivationListenerActivationChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct InputActivationState(pub i32);
impl InputActivationState {
    pub const None: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const ActivatedNotForeground: Self = Self(2i32);
    pub const ActivatedInForeground: Self = Self(3i32);
}
impl ::core::marker::Copy for InputActivationState {}
impl ::core::clone::Clone for InputActivationState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type KeyboardDeliveryInterceptor = *mut ::core::ffi::c_void;
pub type ManipulationCompletedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct ManipulationDelta {
    pub Translation: super::super::Foundation::Point,
    pub Scale: f32,
    pub Rotation: f32,
    pub Expansion: f32,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for ManipulationDelta {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for ManipulationDelta {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ManipulationInertiaStartingEventArgs = *mut ::core::ffi::c_void;
pub type ManipulationStartedEventArgs = *mut ::core::ffi::c_void;
pub type ManipulationUpdatedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct ManipulationVelocities {
    pub Linear: super::super::Foundation::Point,
    pub Angular: f32,
    pub Expansion: f32,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for ManipulationVelocities {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for ManipulationVelocities {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MouseWheelParameters = *mut ::core::ffi::c_void;
pub type PointerPoint = *mut ::core::ffi::c_void;
pub type PointerPointProperties = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct PointerUpdateKind(pub i32);
impl PointerUpdateKind {
    pub const Other: Self = Self(0i32);
    pub const LeftButtonPressed: Self = Self(1i32);
    pub const LeftButtonReleased: Self = Self(2i32);
    pub const RightButtonPressed: Self = Self(3i32);
    pub const RightButtonReleased: Self = Self(4i32);
    pub const MiddleButtonPressed: Self = Self(5i32);
    pub const MiddleButtonReleased: Self = Self(6i32);
    pub const XButton1Pressed: Self = Self(7i32);
    pub const XButton1Released: Self = Self(8i32);
    pub const XButton2Pressed: Self = Self(9i32);
    pub const XButton2Released: Self = Self(10i32);
}
impl ::core::marker::Copy for PointerUpdateKind {}
impl ::core::clone::Clone for PointerUpdateKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PointerVisualizationSettings = *mut ::core::ffi::c_void;
pub type RadialController = *mut ::core::ffi::c_void;
pub type RadialControllerButtonClickedEventArgs = *mut ::core::ffi::c_void;
pub type RadialControllerButtonHoldingEventArgs = *mut ::core::ffi::c_void;
pub type RadialControllerButtonPressedEventArgs = *mut ::core::ffi::c_void;
pub type RadialControllerButtonReleasedEventArgs = *mut ::core::ffi::c_void;
pub type RadialControllerConfiguration = *mut ::core::ffi::c_void;
pub type RadialControllerControlAcquiredEventArgs = *mut ::core::ffi::c_void;
pub type RadialControllerMenu = *mut ::core::ffi::c_void;
pub type RadialControllerMenuItem = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerMenuKnownIcon(pub i32);
impl RadialControllerMenuKnownIcon {
    pub const Scroll: Self = Self(0i32);
    pub const Zoom: Self = Self(1i32);
    pub const UndoRedo: Self = Self(2i32);
    pub const Volume: Self = Self(3i32);
    pub const NextPreviousTrack: Self = Self(4i32);
    pub const Ruler: Self = Self(5i32);
    pub const InkColor: Self = Self(6i32);
    pub const InkThickness: Self = Self(7i32);
    pub const PenType: Self = Self(8i32);
}
impl ::core::marker::Copy for RadialControllerMenuKnownIcon {}
impl ::core::clone::Clone for RadialControllerMenuKnownIcon {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RadialControllerRotationChangedEventArgs = *mut ::core::ffi::c_void;
pub type RadialControllerScreenContact = *mut ::core::ffi::c_void;
pub type RadialControllerScreenContactContinuedEventArgs = *mut ::core::ffi::c_void;
pub type RadialControllerScreenContactEndedEventArgs = *mut ::core::ffi::c_void;
pub type RadialControllerScreenContactStartedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerSystemMenuItemKind(pub i32);
impl RadialControllerSystemMenuItemKind {
    pub const Scroll: Self = Self(0i32);
    pub const Zoom: Self = Self(1i32);
    pub const UndoRedo: Self = Self(2i32);
    pub const Volume: Self = Self(3i32);
    pub const NextPreviousTrack: Self = Self(4i32);
}
impl ::core::marker::Copy for RadialControllerSystemMenuItemKind {}
impl ::core::clone::Clone for RadialControllerSystemMenuItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RightTappedEventArgs = *mut ::core::ffi::c_void;
pub type SystemButtonEventController = *mut ::core::ffi::c_void;
pub type SystemFunctionButtonEventArgs = *mut ::core::ffi::c_void;
pub type SystemFunctionLockChangedEventArgs = *mut ::core::ffi::c_void;
pub type SystemFunctionLockIndicatorChangedEventArgs = *mut ::core::ffi::c_void;
pub type TappedEventArgs = *mut ::core::ffi::c_void;
