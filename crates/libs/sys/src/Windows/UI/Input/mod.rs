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
impl ::windows_sys::core::Interface for IAttachableInputObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2608998196, data2: 41921, data3: 21546, data4: [178, 244, 14, 50, 183, 115, 251, 7] };
}
#[repr(C)]
pub struct IAttachableInputObjectFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IAttachableInputObjectFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2764393550, data2: 17084, data3: 22778, data4: [166, 64, 234, 21, 22, 244, 192, 107] };
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
impl ::windows_sys::core::Interface for ICrossSlidingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3912714040, data2: 28552, data3: 16857, data4: [135, 32, 120, 224, 142, 57, 131, 73] };
}
#[repr(C)]
pub struct ICrossSlidingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICrossSlidingEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4009459016, data2: 49264, data3: 23027, data4: [141, 171, 188, 175, 98, 29, 134, 135] };
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
impl ::windows_sys::core::Interface for IDraggingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 479220612, data2: 2108, data3: 19411, data4: [181, 89, 23, 156, 221, 235, 51, 236] };
}
#[repr(C)]
pub struct IDraggingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDraggingEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1911544825, data2: 14378, data3: 21962, data4: [180, 185, 0, 129, 35, 193, 191, 26] };
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
impl ::windows_sys::core::Interface for IEdgeGesture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1477268114, data2: 10929, data3: 18858, data4: [167, 240, 51, 189, 63, 141, 249, 241] };
}
#[repr(C)]
pub struct IEdgeGestureEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EdgeGestureKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEdgeGestureEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1157253668, data2: 11529, data3: 17121, data4: [139, 94, 54, 130, 8, 121, 106, 76] };
}
#[repr(C)]
pub struct IEdgeGestureStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEdgeGestureStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3161097497, data2: 6382, data3: 16451, data4: [152, 57, 79, 197, 132, 214, 10, 20] };
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
impl ::windows_sys::core::Interface for IGestureRecognizer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3027908543, data2: 15723, data3: 20360, data4: [131, 232, 109, 203, 64, 18, 255, 176] };
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
impl ::windows_sys::core::Interface for IGestureRecognizer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3594914175, data2: 28407, data3: 22342, data4: [139, 168, 143, 242, 32, 110, 111, 59] };
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
impl ::windows_sys::core::Interface for IHoldingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 737629637, data2: 59289, data3: 16820, data4: [187, 64, 36, 47, 64, 149, 155, 113] };
}
#[repr(C)]
pub struct IHoldingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHoldingEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 337488362, data2: 19577, data3: 22132, data4: [175, 234, 73, 63, 222, 185, 31, 25] };
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
impl ::windows_sys::core::Interface for IInputActivationListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1567444690, data2: 10439, data3: 23267, data4: [170, 116, 201, 24, 169, 242, 67, 202] };
}
#[repr(C)]
pub struct IInputActivationListenerActivationChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InputActivationState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInputActivationListenerActivationChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1989784677, data2: 7631, data3: 22417, data4: [180, 185, 108, 175, 190, 237, 32, 86] };
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
impl ::windows_sys::core::Interface for IKeyboardDeliveryInterceptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3032150120, data2: 36681, data3: 17516, data4: [141, 181, 140, 15, 254, 133, 204, 158] };
}
#[repr(C)]
pub struct IKeyboardDeliveryInterceptorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKeyboardDeliveryInterceptorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4193663906, data2: 52922, data3: 18261, data4: [138, 126, 20, 192, 255, 236, 210, 57] };
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
impl ::windows_sys::core::Interface for IManipulationCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3008016939, data2: 53659, data3: 18175, data4: [159, 56, 222, 199, 117, 75, 185, 231] };
}
#[repr(C)]
pub struct IManipulationCompletedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IManipulationCompletedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4039171303, data2: 12457, data3: 23446, data4: [136, 111, 101, 96, 168, 94, 71, 87] };
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
impl ::windows_sys::core::Interface for IManipulationInertiaStartingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3711412376, data2: 9919, data3: 18042, data4: [156, 229, 204, 243, 251, 17, 55, 30] };
}
#[repr(C)]
pub struct IManipulationInertiaStartingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IManipulationInertiaStartingEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3260287416, data2: 63994, data3: 23109, data4: [189, 151, 220, 187, 178, 32, 24, 96] };
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
impl ::windows_sys::core::Interface for IManipulationStartedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3723265854, data2: 53198, data3: 18738, data4: [140, 29, 60, 61, 1, 26, 52, 192] };
}
#[repr(C)]
pub struct IManipulationStartedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IManipulationStartedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 765713230, data2: 58755, data3: 20565, data4: [175, 170, 22, 253, 152, 101, 49, 166] };
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
impl ::windows_sys::core::Interface for IManipulationUpdatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409267941, data2: 43960, data3: 20383, data4: [179, 206, 129, 129, 170, 97, 173, 130] };
}
#[repr(C)]
pub struct IManipulationUpdatedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IManipulationUpdatedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4091525482, data2: 13062, data3: 22787, data4: [161, 197, 255, 151, 87, 168, 104, 158] };
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
impl ::windows_sys::core::Interface for IMouseWheelParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3939551812, data2: 40429, data3: 16439, data4: [129, 73, 94, 76, 194, 86, 68, 104] };
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
impl ::windows_sys::core::Interface for IPointerPoint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3918868861, data2: 29334, data3: 17113, data4: [130, 51, 197, 190, 115, 183, 74, 74] };
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
impl ::windows_sys::core::Interface for IPointerPointProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3348990539, data2: 49507, data3: 20199, data4: [128, 63, 103, 206, 121, 249, 151, 45] };
}
#[repr(C)]
pub struct IPointerPointProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ZDistance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ZDistance: usize,
}
impl ::windows_sys::core::Interface for IPointerPointProperties2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 583222074, data2: 51259, data3: 16832, data4: [162, 150, 94, 35, 45, 100, 214, 175] };
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
impl ::windows_sys::core::Interface for IPointerPointStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2768659341, data2: 10778, data3: 16702, data4: [188, 117, 159, 56, 56, 28, 192, 105] };
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
impl ::windows_sys::core::Interface for IPointerPointTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1298129231, data2: 47228, data3: 16424, data4: [188, 156, 89, 233, 148, 127, 176, 86] };
}
#[repr(C)]
pub struct IPointerVisualizationSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsContactFeedbackEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsContactFeedbackEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsBarrelButtonFeedbackEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsBarrelButtonFeedbackEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPointerVisualizationSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1293837409, data2: 34039, data3: 18845, data4: [189, 145, 42, 54, 226, 183, 170, 162] };
}
#[repr(C)]
pub struct IPointerVisualizationSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPointerVisualizationSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1753681627, data2: 5723, data3: 16916, data4: [180, 243, 88, 78, 202, 140, 138, 105] };
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
impl ::windows_sys::core::Interface for IRadialController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 810930632, data2: 57169, data3: 17364, data4: [178, 59, 14, 16, 55, 70, 122, 9] };
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
impl ::windows_sys::core::Interface for IRadialController2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144319, data2: 19694, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
}
#[repr(C)]
pub struct IRadialControllerButtonClickedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRadialControllerButtonClickedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 543859768, data2: 58961, data3: 4581, data4: [191, 98, 44, 39, 215, 64, 78, 133] };
}
#[repr(C)]
pub struct IRadialControllerButtonClickedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
impl ::windows_sys::core::Interface for IRadialControllerButtonClickedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144307, data2: 15598, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
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
impl ::windows_sys::core::Interface for IRadialControllerButtonHoldingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144302, data2: 15598, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
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
impl ::windows_sys::core::Interface for IRadialControllerButtonPressedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144301, data2: 19694, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
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
impl ::windows_sys::core::Interface for IRadialControllerButtonReleasedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144303, data2: 15598, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
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
impl ::windows_sys::core::Interface for IRadialControllerConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2797051595, data2: 27218, data3: 17456, data4: [145, 12, 86, 55, 10, 157, 107, 66] };
}
#[repr(C)]
pub struct IRadialControllerConfiguration2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetActiveControllerWhenMenuIsSuppressed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ActiveControllerWhenMenuIsSuppressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIsMenuSuppressed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsMenuSuppressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRadialControllerConfiguration2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144311, data2: 15598, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
}
#[repr(C)]
pub struct IRadialControllerConfigurationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRadialControllerConfigurationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2042015973, data2: 1690, data3: 17542, data4: [169, 157, 141, 183, 114, 185, 100, 47] };
}
#[repr(C)]
pub struct IRadialControllerConfigurationStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAppController: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIsAppControllerEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAppControllerEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRadialControllerConfigurationStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1407224599, data2: 57861, data3: 18643, data4: [156, 175, 128, 255, 71, 196, 215, 199] };
}
#[repr(C)]
pub struct IRadialControllerControlAcquiredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRadialControllerControlAcquiredEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 543859769, data2: 58961, data3: 4581, data4: [191, 98, 44, 39, 215, 64, 78, 133] };
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
impl ::windows_sys::core::Interface for IRadialControllerControlAcquiredEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144308, data2: 15598, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
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
impl ::windows_sys::core::Interface for IRadialControllerMenu {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2231808861, data2: 63040, data3: 17426, data4: [171, 160, 186, 208, 119, 229, 234, 138] };
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
impl ::windows_sys::core::Interface for IRadialControllerMenuItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3356477837, data2: 44299, data3: 19612, data4: [143, 47, 19, 106, 35, 115, 166, 186] };
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
impl ::windows_sys::core::Interface for IRadialControllerMenuItemStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 614336647, data2: 55362, data3: 17700, data4: [157, 248, 224, 214, 71, 237, 200, 135] };
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
impl ::windows_sys::core::Interface for IRadialControllerMenuItemStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 213610686, data2: 32318, data3: 18621, data4: [190, 4, 44, 127, 202, 169, 193, 255] };
}
#[repr(C)]
pub struct IRadialControllerRotationChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RotationDeltaInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRadialControllerRotationChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 543859765, data2: 58961, data3: 4581, data4: [191, 98, 44, 39, 215, 64, 78, 133] };
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
impl ::windows_sys::core::Interface for IRadialControllerRotationChangedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144300, data2: 19694, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
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
impl ::windows_sys::core::Interface for IRadialControllerScreenContact {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 543859764, data2: 58961, data3: 4581, data4: [191, 98, 44, 39, 215, 64, 78, 133] };
}
#[repr(C)]
pub struct IRadialControllerScreenContactContinuedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRadialControllerScreenContactContinuedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 543859767, data2: 58961, data3: 4581, data4: [191, 98, 44, 39, 215, 64, 78, 133] };
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
impl ::windows_sys::core::Interface for IRadialControllerScreenContactContinuedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144305, data2: 15598, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
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
impl ::windows_sys::core::Interface for IRadialControllerScreenContactEndedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144306, data2: 15598, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
}
#[repr(C)]
pub struct IRadialControllerScreenContactStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRadialControllerScreenContactStartedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 543859766, data2: 58961, data3: 4581, data4: [191, 98, 44, 39, 215, 64, 78, 133] };
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
impl ::windows_sys::core::Interface for IRadialControllerScreenContactStartedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144304, data2: 15598, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
}
#[repr(C)]
pub struct IRadialControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CreateForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRadialControllerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4208906423, data2: 47180, data3: 18580, data4: [135, 170, 143, 37, 170, 95, 40, 139] };
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
impl ::windows_sys::core::Interface for IRightTappedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1287602365, data2: 44922, data3: 18998, data4: [148, 118, 177, 220, 225, 65, 112, 154] };
}
#[repr(C)]
pub struct IRightTappedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRightTappedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1640478651, data2: 40791, data3: 22615, data4: [163, 60, 197, 140, 61, 250, 149, 158] };
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
impl ::windows_sys::core::Interface for ISystemButtonEventController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1505268649, data2: 29628, data3: 21173, data4: [186, 65, 130, 81, 27, 44, 180, 108] };
}
#[repr(C)]
pub struct ISystemButtonEventControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub CreateForDispatcherQueue: unsafe extern "system" fn(this: *mut *mut Self, queue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForDispatcherQueue: usize,
}
impl ::windows_sys::core::Interface for ISystemButtonEventControllerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1664069755, data2: 8381, data3: 24085, data4: [175, 74, 0, 219, 242, 6, 79, 250] };
}
#[repr(C)]
pub struct ISystemFunctionButtonEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemFunctionButtonEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1211337071, data2: 32977, data3: 24022, data4: [146, 167, 98, 165, 8, 255, 239, 90] };
}
#[repr(C)]
pub struct ISystemFunctionLockChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub IsLocked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemFunctionLockChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3439592968, data2: 64761, data3: 22620, data4: [190, 171, 241, 210, 234, 243, 100, 171] };
}
#[repr(C)]
pub struct ISystemFunctionLockIndicatorChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub IsIndicatorOn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemFunctionLockIndicatorChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2987571534, data2: 31343, data3: 22702, data4: [179, 4, 186, 230, 29, 3, 113, 185] };
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
impl ::windows_sys::core::Interface for ITappedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3483444964, data2: 9530, data3: 19516, data4: [149, 59, 57, 92, 55, 174, 211, 9] };
}
#[repr(C)]
pub struct ITappedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITappedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 692291826, data2: 6014, data3: 20949, data4: [190, 86, 238, 8, 102, 250, 150, 140] };
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
