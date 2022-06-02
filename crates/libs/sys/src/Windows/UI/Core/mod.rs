#[cfg(feature = "UI_Core_AnimationMetrics")]
pub mod AnimationMetrics;
#[cfg(feature = "UI_Core_Preview")]
pub mod Preview;
pub type AcceleratorKeyEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct AppViewBackButtonVisibility(pub i32);
impl AppViewBackButtonVisibility {
    pub const Visible: Self = Self(0i32);
    pub const Collapsed: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
impl ::core::marker::Copy for AppViewBackButtonVisibility {}
impl ::core::clone::Clone for AppViewBackButtonVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AutomationProviderRequestedEventArgs = *mut ::core::ffi::c_void;
pub type BackRequestedEventArgs = *mut ::core::ffi::c_void;
pub type CharacterReceivedEventArgs = *mut ::core::ffi::c_void;
pub type ClosestInteractiveBoundsRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreAcceleratorKeyEventType(pub i32);
impl CoreAcceleratorKeyEventType {
    pub const Character: Self = Self(2i32);
    pub const DeadCharacter: Self = Self(3i32);
    pub const KeyDown: Self = Self(0i32);
    pub const KeyUp: Self = Self(1i32);
    pub const SystemCharacter: Self = Self(6i32);
    pub const SystemDeadCharacter: Self = Self(7i32);
    pub const SystemKeyDown: Self = Self(4i32);
    pub const SystemKeyUp: Self = Self(5i32);
    pub const UnicodeCharacter: Self = Self(8i32);
}
impl ::core::marker::Copy for CoreAcceleratorKeyEventType {}
impl ::core::clone::Clone for CoreAcceleratorKeyEventType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreAcceleratorKeys = *mut ::core::ffi::c_void;
pub type CoreComponentInputSource = *mut ::core::ffi::c_void;
pub type CoreCursor = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreCursorType(pub i32);
impl CoreCursorType {
    pub const Arrow: Self = Self(0i32);
    pub const Cross: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
    pub const Hand: Self = Self(3i32);
    pub const Help: Self = Self(4i32);
    pub const IBeam: Self = Self(5i32);
    pub const SizeAll: Self = Self(6i32);
    pub const SizeNortheastSouthwest: Self = Self(7i32);
    pub const SizeNorthSouth: Self = Self(8i32);
    pub const SizeNorthwestSoutheast: Self = Self(9i32);
    pub const SizeWestEast: Self = Self(10i32);
    pub const UniversalNo: Self = Self(11i32);
    pub const UpArrow: Self = Self(12i32);
    pub const Wait: Self = Self(13i32);
    pub const Pin: Self = Self(14i32);
    pub const Person: Self = Self(15i32);
}
impl ::core::marker::Copy for CoreCursorType {}
impl ::core::clone::Clone for CoreCursorType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreDispatcher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreDispatcherPriority(pub i32);
impl CoreDispatcherPriority {
    pub const Idle: Self = Self(-2i32);
    pub const Low: Self = Self(-1i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreDispatcherPriority {}
impl ::core::clone::Clone for CoreDispatcherPriority {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreIndependentInputFilters(pub u32);
impl CoreIndependentInputFilters {
    pub const None: Self = Self(0u32);
    pub const MouseButton: Self = Self(1u32);
    pub const MouseWheel: Self = Self(2u32);
    pub const MouseHover: Self = Self(4u32);
    pub const PenWithBarrel: Self = Self(8u32);
    pub const PenInverted: Self = Self(16u32);
}
impl ::core::marker::Copy for CoreIndependentInputFilters {}
impl ::core::clone::Clone for CoreIndependentInputFilters {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreIndependentInputSource = *mut ::core::ffi::c_void;
pub type CoreIndependentInputSourceController = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreInputDeviceTypes(pub u32);
impl CoreInputDeviceTypes {
    pub const None: Self = Self(0u32);
    pub const Touch: Self = Self(1u32);
    pub const Pen: Self = Self(2u32);
    pub const Mouse: Self = Self(4u32);
}
impl ::core::marker::Copy for CoreInputDeviceTypes {}
impl ::core::clone::Clone for CoreInputDeviceTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Core\"`*"]
pub struct CorePhysicalKeyStatus {
    pub RepeatCount: u32,
    pub ScanCode: u32,
    pub IsExtendedKey: bool,
    pub IsMenuKeyDown: bool,
    pub WasKeyDown: bool,
    pub IsKeyReleased: bool,
}
impl ::core::marker::Copy for CorePhysicalKeyStatus {}
impl ::core::clone::Clone for CorePhysicalKeyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreProcessEventsOption(pub i32);
impl CoreProcessEventsOption {
    pub const ProcessOneAndAllPending: Self = Self(0i32);
    pub const ProcessOneIfPresent: Self = Self(1i32);
    pub const ProcessUntilQuit: Self = Self(2i32);
    pub const ProcessAllIfPresent: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreProcessEventsOption {}
impl ::core::clone::Clone for CoreProcessEventsOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Core\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct CoreProximityEvaluation {
    pub Score: i32,
    pub AdjustedPoint: super::super::Foundation::Point,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for CoreProximityEvaluation {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for CoreProximityEvaluation {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreProximityEvaluationScore(pub i32);
impl CoreProximityEvaluationScore {
    pub const Closest: Self = Self(0i32);
    pub const Farthest: Self = Self(2147483647i32);
}
impl ::core::marker::Copy for CoreProximityEvaluationScore {}
impl ::core::clone::Clone for CoreProximityEvaluationScore {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreVirtualKeyStates(pub u32);
impl CoreVirtualKeyStates {
    pub const None: Self = Self(0u32);
    pub const Down: Self = Self(1u32);
    pub const Locked: Self = Self(2u32);
}
impl ::core::marker::Copy for CoreVirtualKeyStates {}
impl ::core::clone::Clone for CoreVirtualKeyStates {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreWindow = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreWindowActivationMode(pub i32);
impl CoreWindowActivationMode {
    pub const None: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const ActivatedNotForeground: Self = Self(2i32);
    pub const ActivatedInForeground: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreWindowActivationMode {}
impl ::core::clone::Clone for CoreWindowActivationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreWindowActivationState(pub i32);
impl CoreWindowActivationState {
    pub const CodeActivated: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const PointerActivated: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWindowActivationState {}
impl ::core::clone::Clone for CoreWindowActivationState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreWindowDialog = *mut ::core::ffi::c_void;
pub type CoreWindowEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Core\"`*"]
#[repr(transparent)]
pub struct CoreWindowFlowDirection(pub i32);
impl CoreWindowFlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWindowFlowDirection {}
impl ::core::clone::Clone for CoreWindowFlowDirection {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreWindowFlyout = *mut ::core::ffi::c_void;
pub type CoreWindowPopupShowingEventArgs = *mut ::core::ffi::c_void;
pub type CoreWindowResizeManager = *mut ::core::ffi::c_void;
pub type DispatchedHandler = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAcceleratorKeyEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub EventType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreAcceleratorKeyEventType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub VirtualKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    VirtualKey: usize,
    pub KeyStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CorePhysicalKeyStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAcceleratorKeyEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationProviderRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub AutomationProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAutomationProvider: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICharacterReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub KeyStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CorePhysicalKeyStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IClosestInteractiveBoundsRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PointerPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerPosition: usize,
    #[cfg(feature = "Foundation")]
    pub SearchBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchBounds: usize,
    #[cfg(feature = "Foundation")]
    pub ClosestInteractiveBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClosestInteractiveBounds: usize,
    #[cfg(feature = "Foundation")]
    pub SetClosestInteractiveBounds: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetClosestInteractiveBounds: usize,
}
#[repr(C)]
pub struct ICoreAcceleratorKeys {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AcceleratorKeyActivated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcceleratorKeyActivated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAcceleratorKeyActivated: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAcceleratorKeyActivated: usize,
}
#[repr(C)]
pub struct ICoreClosestInteractiveBoundsRequested {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ClosestInteractiveBoundsRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClosestInteractiveBoundsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosestInteractiveBoundsRequested: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosestInteractiveBoundsRequested: usize,
}
#[repr(C)]
pub struct ICoreComponentFocusable {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LostFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLostFocus: usize,
}
#[repr(C)]
pub struct ICoreCursor {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreCursorType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreCursorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateCursor: unsafe extern "system" fn(this: *mut *mut Self, r#type: CoreCursorType, id: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreDispatcher {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasThreadAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ProcessEvents: unsafe extern "system" fn(this: *mut *mut Self, options: CoreProcessEventsOption) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RunAsync: unsafe extern "system" fn(this: *mut *mut Self, priority: CoreDispatcherPriority, agilecallback: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RunIdleAsync: unsafe extern "system" fn(this: *mut *mut Self, agilecallback: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunIdleAsync: usize,
}
#[repr(C)]
pub struct ICoreDispatcher2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TryRunAsync: unsafe extern "system" fn(this: *mut *mut Self, priority: CoreDispatcherPriority, agilecallback: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRunAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryRunIdleAsync: unsafe extern "system" fn(this: *mut *mut Self, agilecallback: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRunIdleAsync: usize,
}
#[repr(C)]
pub struct ICoreDispatcherWithTaskPriority {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentPriority: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreDispatcherPriority) -> ::windows_sys::core::HRESULT,
    pub SetCurrentPriority: unsafe extern "system" fn(this: *mut *mut Self, value: CoreDispatcherPriority) -> ::windows_sys::core::HRESULT,
    pub ShouldYield: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShouldYieldToPriority: unsafe extern "system" fn(this: *mut *mut Self, priority: CoreDispatcherPriority, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub StopProcessEvents: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreIndependentInputSourceController {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTransparentForUncontrolledInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTransparentForUncontrolledInput: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsPalmRejectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPalmRejectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetControlledInput: unsafe extern "system" fn(this: *mut *mut Self, inputtypes: CoreInputDeviceTypes) -> ::windows_sys::core::HRESULT,
    pub SetControlledInputWithFilters: unsafe extern "system" fn(this: *mut *mut Self, inputtypes: CoreInputDeviceTypes, required: CoreIndependentInputFilters, excluded: CoreIndependentInputFilters) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreIndependentInputSourceControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub CreateForVisual: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateForVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateForIVisualElement: unsafe extern "system" fn(this: *mut *mut Self, visualelement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateForIVisualElement: usize,
}
#[repr(C)]
pub struct ICoreInputSourceBase {
    pub base__: ::windows_sys::core::IInspectable,
    pub Dispatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsInputEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsInputEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InputEnabled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputEnabled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInputEnabled: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInputEnabled: usize,
}
#[repr(C)]
pub struct ICoreKeyboardInputSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetCurrentKeyState: unsafe extern "system" fn(this: *mut *mut Self, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetCurrentKeyState: usize,
    #[cfg(feature = "Foundation")]
    pub CharacterReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CharacterReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCharacterReceived: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCharacterReceived: usize,
    #[cfg(feature = "Foundation")]
    pub KeyDown: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeyDown: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyDown: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyDown: usize,
    #[cfg(feature = "Foundation")]
    pub KeyUp: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeyUp: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyUp: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyUp: usize,
}
#[repr(C)]
pub struct ICoreKeyboardInputSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentKeyEventDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICorePointerInputSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReleasePointerCapture: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetPointerCapture: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub HasCapture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PointerPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerPosition: usize,
    pub PointerCursor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPointerCursor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PointerCaptureLost: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerCaptureLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerCaptureLost: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerCaptureLost: usize,
    #[cfg(feature = "Foundation")]
    pub PointerEntered: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerEntered: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub PointerExited: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerExited: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub PointerMoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerMoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerMoved: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerMoved: usize,
    #[cfg(feature = "Foundation")]
    pub PointerPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerPressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub PointerReleased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerReleased: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub PointerWheelChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerWheelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerWheelChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerWheelChanged: usize,
}
#[repr(C)]
pub struct ICorePointerInputSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
#[repr(C)]
pub struct ICorePointerRedirector {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PointerRoutedAway: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerRoutedAway: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerRoutedAway: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerRoutedAway: usize,
    #[cfg(feature = "Foundation")]
    pub PointerRoutedTo: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerRoutedTo: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerRoutedTo: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerRoutedTo: usize,
    #[cfg(feature = "Foundation")]
    pub PointerRoutedReleased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerRoutedReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerRoutedReleased: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerRoutedReleased: usize,
}
#[repr(C)]
pub struct ICoreTouchHitTesting {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TouchHitTesting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TouchHitTesting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTouchHitTesting: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTouchHitTesting: usize,
}
#[repr(C)]
pub struct ICoreWindow {
    pub base__: ::windows_sys::core::IInspectable,
    pub AutomationHostProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomProperties: usize,
    pub Dispatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreWindowFlowDirection) -> ::windows_sys::core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(this: *mut *mut Self, value: CoreWindowFlowDirection) -> ::windows_sys::core::HRESULT,
    pub IsInputEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsInputEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PointerCursor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPointerCursor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PointerPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerPosition: usize,
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Activate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetAsyncKeyState: unsafe extern "system" fn(this: *mut *mut Self, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetAsyncKeyState: usize,
    #[cfg(feature = "System")]
    pub GetKeyState: unsafe extern "system" fn(this: *mut *mut Self, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetKeyState: usize,
    pub ReleasePointerCapture: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetPointerCapture: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Activated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    #[cfg(feature = "Foundation")]
    pub AutomationProviderRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutomationProviderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAutomationProviderRequested: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAutomationProviderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CharacterReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CharacterReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCharacterReceived: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCharacterReceived: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub InputEnabled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputEnabled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInputEnabled: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInputEnabled: usize,
    #[cfg(feature = "Foundation")]
    pub KeyDown: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeyDown: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyDown: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyDown: usize,
    #[cfg(feature = "Foundation")]
    pub KeyUp: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeyUp: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyUp: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyUp: usize,
    #[cfg(feature = "Foundation")]
    pub PointerCaptureLost: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerCaptureLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerCaptureLost: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerCaptureLost: usize,
    #[cfg(feature = "Foundation")]
    pub PointerEntered: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerEntered: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub PointerExited: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerExited: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub PointerMoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerMoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerMoved: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerMoved: usize,
    #[cfg(feature = "Foundation")]
    pub PointerPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerPressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub PointerReleased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerReleased: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub TouchHitTesting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TouchHitTesting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTouchHitTesting: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTouchHitTesting: usize,
    #[cfg(feature = "Foundation")]
    pub PointerWheelChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PointerWheelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerWheelChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerWheelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SizeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSizeChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub VisibilityChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisibilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVisibilityChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVisibilityChanged: usize,
}
#[repr(C)]
pub struct ICoreWindow2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetPointerPosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPointerPosition: usize,
}
#[repr(C)]
pub struct ICoreWindow3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ClosestInteractiveBoundsRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClosestInteractiveBoundsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosestInteractiveBoundsRequested: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosestInteractiveBoundsRequested: usize,
    pub GetCurrentKeyEventDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreWindow4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ResizeStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResizeStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResizeStarted: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResizeStarted: usize,
    #[cfg(feature = "Foundation")]
    pub ResizeCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResizeCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResizeCompleted: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResizeCompleted: usize,
}
#[repr(C)]
pub struct ICoreWindow5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
    pub ActivationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreWindowActivationMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreWindowDialog {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Showing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Showing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShowing: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShowing: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSize: usize,
    #[cfg(feature = "Foundation")]
    pub MinSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSize: usize,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsInteractionDelayed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetIsInteractionDelayed: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub Commands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups")))]
    Commands: usize,
    pub DefaultCommandIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDefaultCommandIndex: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CancelCommandIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCancelCommandIndex: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub BackButtonCommand: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    BackButtonCommand: usize,
    #[cfg(feature = "UI_Popups")]
    pub SetBackButtonCommand: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    SetBackButtonCommand: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowAsync: usize,
}
#[repr(C)]
pub struct ICoreWindowDialogFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithTitle: unsafe extern "system" fn(this: *mut *mut Self, title: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreWindowEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreWindowFlyout {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Showing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Showing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShowing: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShowing: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSize: usize,
    #[cfg(feature = "Foundation")]
    pub MinSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSize: usize,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsInteractionDelayed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetIsInteractionDelayed: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub Commands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups")))]
    Commands: usize,
    pub DefaultCommandIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDefaultCommandIndex: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub BackButtonCommand: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    BackButtonCommand: usize,
    #[cfg(feature = "UI_Popups")]
    pub SetBackButtonCommand: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    SetBackButtonCommand: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowAsync: usize,
}
#[repr(C)]
pub struct ICoreWindowFlyoutFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, position: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWithTitle: unsafe extern "system" fn(this: *mut *mut Self, position: super::super::Foundation::Point, title: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithTitle: usize,
}
#[repr(C)]
pub struct ICoreWindowPopupShowingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetDesiredSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredSize: usize,
}
#[repr(C)]
pub struct ICoreWindowResizeManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub NotifyLayoutCompleted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreWindowResizeManagerLayoutCapability {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetShouldWaitForLayoutCompletion: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ShouldWaitForLayoutCompletion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreWindowResizeManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreWindowStatic {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentThread: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreWindowWithContext {
    pub base__: ::windows_sys::core::IInspectable,
    pub UIContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIdleDispatchedHandlerArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDispatcherIdle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInitializeWithCoreWindow {
    pub base__: ::windows_sys::core::IInspectable,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, window: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInputEnabledEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InputEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub VirtualKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    VirtualKey: usize,
    pub KeyStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CorePhysicalKeyStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointerEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Input")]
    pub CurrentPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    CurrentPoint: usize,
    #[cfg(feature = "System")]
    pub KeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    KeyModifiers: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))]
    pub GetIntermediatePoints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Input")))]
    GetIntermediatePoints: usize,
}
#[repr(C)]
pub struct ISystemNavigationManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BackRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BackRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackRequested: usize,
}
#[repr(C)]
pub struct ISystemNavigationManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppViewBackButtonVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppViewBackButtonVisibility) -> ::windows_sys::core::HRESULT,
    pub SetAppViewBackButtonVisibility: unsafe extern "system" fn(this: *mut *mut Self, value: AppViewBackButtonVisibility) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemNavigationManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITouchHitTestingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ProximityEvaluation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreProximityEvaluation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProximityEvaluation: usize,
    #[cfg(feature = "Foundation")]
    pub SetProximityEvaluation: unsafe extern "system" fn(this: *mut *mut Self, value: CoreProximityEvaluation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProximityEvaluation: usize,
    #[cfg(feature = "Foundation")]
    pub Point: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Point: usize,
    #[cfg(feature = "Foundation")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BoundingBox: usize,
    #[cfg(feature = "Foundation")]
    pub EvaluateProximityToRect: unsafe extern "system" fn(this: *mut *mut Self, controlboundingbox: super::super::Foundation::Rect, result__: *mut CoreProximityEvaluation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EvaluateProximityToRect: usize,
    #[cfg(feature = "Foundation")]
    pub EvaluateProximityToPolygon: unsafe extern "system" fn(this: *mut *mut Self, controlVertices_array_size: u32, controlvertices: *const super::super::Foundation::Point, result__: *mut CoreProximityEvaluation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EvaluateProximityToPolygon: usize,
}
#[repr(C)]
pub struct IVisibilityChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindowActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub WindowActivationState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreWindowActivationState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindowSizeChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
}
pub type IdleDispatchedHandler = *mut ::core::ffi::c_void;
pub type IdleDispatchedHandlerArgs = *mut ::core::ffi::c_void;
pub type InputEnabledEventArgs = *mut ::core::ffi::c_void;
pub type KeyEventArgs = *mut ::core::ffi::c_void;
pub type PointerEventArgs = *mut ::core::ffi::c_void;
pub type SystemNavigationManager = *mut ::core::ffi::c_void;
pub type TouchHitTestingEventArgs = *mut ::core::ffi::c_void;
pub type VisibilityChangedEventArgs = *mut ::core::ffi::c_void;
pub type WindowActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WindowSizeChangedEventArgs = *mut ::core::ffi::c_void;
