pub type AccessKeyDisplayDismissedEventArgs = *mut ::core::ffi::c_void;
pub type AccessKeyDisplayRequestedEventArgs = *mut ::core::ffi::c_void;
pub type AccessKeyInvokedEventArgs = *mut ::core::ffi::c_void;
pub type AccessKeyManager = *mut ::core::ffi::c_void;
pub type CanExecuteRequestedEventArgs = *mut ::core::ffi::c_void;
pub type CharacterReceivedRoutedEventArgs = *mut ::core::ffi::c_void;
pub type ContextRequestedEventArgs = *mut ::core::ffi::c_void;
pub type DoubleTappedEventHandler = *mut ::core::ffi::c_void;
pub type DoubleTappedRoutedEventArgs = *mut ::core::ffi::c_void;
pub type ExecuteRequestedEventArgs = *mut ::core::ffi::c_void;
pub type FindNextElementOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct FocusInputDeviceKind(pub i32);
impl FocusInputDeviceKind {
    pub const None: Self = Self(0i32);
    pub const Mouse: Self = Self(1i32);
    pub const Touch: Self = Self(2i32);
    pub const Pen: Self = Self(3i32);
    pub const Keyboard: Self = Self(4i32);
    pub const GameController: Self = Self(5i32);
}
impl ::core::marker::Copy for FocusInputDeviceKind {}
impl ::core::clone::Clone for FocusInputDeviceKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FocusManager = *mut ::core::ffi::c_void;
pub type FocusManagerGotFocusEventArgs = *mut ::core::ffi::c_void;
pub type FocusManagerLostFocusEventArgs = *mut ::core::ffi::c_void;
pub type FocusMovementResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct FocusNavigationDirection(pub i32);
impl FocusNavigationDirection {
    pub const Next: Self = Self(0i32);
    pub const Previous: Self = Self(1i32);
    pub const Up: Self = Self(2i32);
    pub const Down: Self = Self(3i32);
    pub const Left: Self = Self(4i32);
    pub const Right: Self = Self(5i32);
    pub const None: Self = Self(6i32);
}
impl ::core::marker::Copy for FocusNavigationDirection {}
impl ::core::clone::Clone for FocusNavigationDirection {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GettingFocusEventArgs = *mut ::core::ffi::c_void;
pub type HoldingEventHandler = *mut ::core::ffi::c_void;
pub type HoldingRoutedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAccessKeyDisplayDismissedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAccessKeyDisplayRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PressedKeys: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccessKeyInvokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccessKeyManager {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAccessKeyManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDisplayModeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsDisplayModeEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsDisplayModeEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsDisplayModeEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsDisplayModeEnabledChanged: usize,
    pub ExitDisplayMode: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccessKeyManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreKeyTipsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAreKeyTipsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICanExecuteRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parameter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanExecute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanExecute: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICharacterReceivedRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Character: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub KeyStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    KeyStatus: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommand {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CanExecuteChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanExecuteChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCanExecuteChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCanExecuteChanged: usize,
    pub CanExecute: unsafe extern "system" fn(this: *mut *mut Self, parameter: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Execute: unsafe extern "system" fn(this: *mut *mut Self, parameter: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContextRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryGetPosition: unsafe extern "system" fn(this: *mut *mut Self, relativeto: *mut ::core::ffi::c_void, point: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetPosition: usize,
}
#[repr(C)]
pub struct IDoubleTappedRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPosition: unsafe extern "system" fn(this: *mut *mut Self, relativeto: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPosition: usize,
}
#[repr(C)]
pub struct IExecuteRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parameter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFindNextElementOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub SearchRoot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSearchRoot: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExclusionRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExclusionRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetExclusionRect: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExclusionRect: usize,
    #[cfg(feature = "Foundation")]
    pub HintRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HintRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetHintRect: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetHintRect: usize,
    pub XYFocusNavigationStrategyOverride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut XYFocusNavigationStrategyOverride) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusNavigationStrategyOverride: unsafe extern "system" fn(this: *mut *mut Self, value: XYFocusNavigationStrategyOverride) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFocusManager {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFocusManagerGotFocusEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub NewFocusedElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFocusManagerLostFocusEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldFocusedElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFocusManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetFocusedElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFocusManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryMoveFocus: unsafe extern "system" fn(this: *mut *mut Self, focusnavigationdirection: FocusNavigationDirection, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFocusManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FindNextFocusableElement: unsafe extern "system" fn(this: *mut *mut Self, focusnavigationdirection: FocusNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FindNextFocusableElementWithHint: unsafe extern "system" fn(this: *mut *mut Self, focusnavigationdirection: FocusNavigationDirection, hintrect: super::super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindNextFocusableElementWithHint: usize,
}
#[repr(C)]
pub struct IFocusManagerStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryMoveFocusWithOptions: unsafe extern "system" fn(this: *mut *mut Self, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub FindNextElement: unsafe extern "system" fn(this: *mut *mut Self, focusnavigationdirection: FocusNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindFirstFocusableElement: unsafe extern "system" fn(this: *mut *mut Self, searchscope: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindLastFocusableElement: unsafe extern "system" fn(this: *mut *mut Self, searchscope: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindNextElementWithOptions: unsafe extern "system" fn(this: *mut *mut Self, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFocusManagerStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TryFocusAsync: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: super::FocusState, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryFocusAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveFocusAsync: unsafe extern "system" fn(this: *mut *mut Self, focusnavigationdirection: FocusNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveFocusAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveFocusWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveFocusWithOptionsAsync: usize,
}
#[repr(C)]
pub struct IFocusManagerStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LostFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub GettingFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GettingFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGettingFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGettingFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LosingFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LosingFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLosingFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLosingFocus: usize,
}
#[repr(C)]
pub struct IFocusManagerStatics7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetFocusedElement: unsafe extern "system" fn(this: *mut *mut Self, xamlroot: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFocusMovementResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Succeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGettingFocusEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldFocusedElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NewFocusedElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNewFocusedElement: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::FocusState) -> ::windows_sys::core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FocusNavigationDirection) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FocusInputDeviceKind) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGettingFocusEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryCancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TrySetNewFocusedElement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGettingFocusEventArgs3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHoldingRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "UI_Input")]
    pub HoldingState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Input::HoldingState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    HoldingState: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPosition: unsafe extern "system" fn(this: *mut *mut Self, relativeto: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPosition: usize,
}
#[repr(C)]
pub struct IInertiaExpansionBehavior {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesiredDeceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub DesiredExpansion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDesiredExpansion: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInertiaRotationBehavior {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesiredDeceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub DesiredRotation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDesiredRotation: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInertiaTranslationBehavior {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesiredDeceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub DesiredDisplacement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDesiredDisplacement: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInputScope {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Names: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Names: usize,
}
#[repr(C)]
pub struct IInputScopeName {
    pub base__: ::windows_sys::core::IInspectable,
    pub NameValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InputScopeNameValue) -> ::windows_sys::core::HRESULT,
    pub SetNameValue: unsafe extern "system" fn(this: *mut *mut Self, value: InputScopeNameValue) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInputScopeNameFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, namevalue: InputScopeNameValue, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub Key: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    Key: usize,
    #[cfg(feature = "UI_Core")]
    pub KeyStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    KeyStatus: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyRoutedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub OriginalKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    OriginalKey: usize,
}
#[repr(C)]
pub struct IKeyRoutedEventArgs3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyboardAccelerator {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub Key: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    Key: usize,
    #[cfg(feature = "System")]
    pub SetKey: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetKey: usize,
    #[cfg(feature = "System")]
    pub Modifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    Modifiers: usize,
    #[cfg(feature = "System")]
    pub SetModifiers: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetModifiers: usize,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ScopeOwner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetScopeOwner: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Invoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Invoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInvoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInvoked: usize,
}
#[repr(C)]
pub struct IKeyboardAcceleratorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyboardAcceleratorInvokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Element: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyboardAcceleratorInvokedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyboardAccelerator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyboardAcceleratorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ModifiersProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ScopeOwnerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILosingFocusEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldFocusedElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NewFocusedElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNewFocusedElement: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::FocusState) -> ::windows_sys::core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FocusNavigationDirection) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FocusInputDeviceKind) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILosingFocusEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryCancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TrySetNewFocusedElement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILosingFocusEventArgs3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IManipulationCompletedRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Container: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub IsInertial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Cumulative: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Input::ManipulationDelta) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Cumulative: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Velocities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Velocities: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
}
#[repr(C)]
pub struct IManipulationDeltaRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Container: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub IsInertial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Delta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Input::ManipulationDelta) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Delta: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Cumulative: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Input::ManipulationDelta) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Cumulative: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Velocities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Velocities: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IManipulationInertiaStartingRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Container: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExpansionBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetExpansionBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RotationBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRotationBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TranslationBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTranslationBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Delta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Input::ManipulationDelta) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Delta: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Cumulative: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Input::ManipulationDelta) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Cumulative: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Velocities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Velocities: usize,
}
#[repr(C)]
pub struct IManipulationPivot {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Center: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Center: usize,
    #[cfg(feature = "Foundation")]
    pub SetCenter: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCenter: usize,
    pub Radius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRadius: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IManipulationPivotFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceWithCenterAndRadius: unsafe extern "system" fn(this: *mut *mut Self, center: super::super::super::Foundation::Point, radius: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceWithCenterAndRadius: usize,
}
#[repr(C)]
pub struct IManipulationStartedRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Container: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Cumulative: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Input::ManipulationDelta) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Cumulative: usize,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IManipulationStartedRoutedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IManipulationStartingRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ManipulationModes) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: ManipulationModes) -> ::windows_sys::core::HRESULT,
    pub Container: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContainer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Pivot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPivot: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INoFocusCandidateFoundEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FocusNavigationDirection) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FocusInputDeviceKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointer {
    pub base__: ::windows_sys::core::IInspectable,
    pub PointerId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub IsInContact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsInRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointerRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Pointer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub KeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    KeyModifiers: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub GetCurrentPoint: unsafe extern "system" fn(this: *mut *mut Self, relativeto: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    GetCurrentPoint: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))]
    pub GetIntermediatePoints: unsafe extern "system" fn(this: *mut *mut Self, relativeto: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Input")))]
    GetIntermediatePoints: usize,
}
#[repr(C)]
pub struct IPointerRoutedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsGenerated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProcessKeyboardAcceleratorEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub Key: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    Key: usize,
    #[cfg(feature = "System")]
    pub Modifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    Modifiers: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRightTappedRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPosition: unsafe extern "system" fn(this: *mut *mut Self, relativeto: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPosition: usize,
}
#[repr(C)]
pub struct IStandardUICommand {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StandardUICommandKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStandardUICommand2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetKind: unsafe extern "system" fn(this: *mut *mut Self, value: StandardUICommandKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStandardUICommandFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstanceWithKind: unsafe extern "system" fn(this: *mut *mut Self, kind: StandardUICommandKind, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStandardUICommandStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub KindProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITappedRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPosition: unsafe extern "system" fn(this: *mut *mut Self, relativeto: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPosition: usize,
}
#[repr(C)]
pub struct IXamlUICommand {
    pub base__: ::windows_sys::core::IInspectable,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub IconSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    IconSource: usize,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub SetIconSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    SetIconSource: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub KeyboardAccelerators: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    KeyboardAccelerators: usize,
    pub AccessKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Command: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExecuteRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExecuteRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveExecuteRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveExecuteRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CanExecuteRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanExecuteRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCanExecuteRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCanExecuteRequested: usize,
    pub NotifyCanExecuteChanged: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlUICommandFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlUICommandStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub LabelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IconSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyboardAcceleratorsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DescriptionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CommandProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type InertiaExpansionBehavior = *mut ::core::ffi::c_void;
pub type InertiaRotationBehavior = *mut ::core::ffi::c_void;
pub type InertiaTranslationBehavior = *mut ::core::ffi::c_void;
pub type InputScope = *mut ::core::ffi::c_void;
pub type InputScopeName = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct InputScopeNameValue(pub i32);
impl InputScopeNameValue {
    pub const Default: Self = Self(0i32);
    pub const Url: Self = Self(1i32);
    pub const EmailSmtpAddress: Self = Self(5i32);
    pub const PersonalFullName: Self = Self(7i32);
    pub const CurrencyAmountAndSymbol: Self = Self(20i32);
    pub const CurrencyAmount: Self = Self(21i32);
    pub const DateMonthNumber: Self = Self(23i32);
    pub const DateDayNumber: Self = Self(24i32);
    pub const DateYear: Self = Self(25i32);
    pub const Digits: Self = Self(28i32);
    pub const Number: Self = Self(29i32);
    pub const Password: Self = Self(31i32);
    pub const TelephoneNumber: Self = Self(32i32);
    pub const TelephoneCountryCode: Self = Self(33i32);
    pub const TelephoneAreaCode: Self = Self(34i32);
    pub const TelephoneLocalNumber: Self = Self(35i32);
    pub const TimeHour: Self = Self(37i32);
    pub const TimeMinutesOrSeconds: Self = Self(38i32);
    pub const NumberFullWidth: Self = Self(39i32);
    pub const AlphanumericHalfWidth: Self = Self(40i32);
    pub const AlphanumericFullWidth: Self = Self(41i32);
    pub const Hiragana: Self = Self(44i32);
    pub const KatakanaHalfWidth: Self = Self(45i32);
    pub const KatakanaFullWidth: Self = Self(46i32);
    pub const Hanja: Self = Self(47i32);
    pub const HangulHalfWidth: Self = Self(48i32);
    pub const HangulFullWidth: Self = Self(49i32);
    pub const Search: Self = Self(50i32);
    pub const Formula: Self = Self(51i32);
    pub const SearchIncremental: Self = Self(52i32);
    pub const ChineseHalfWidth: Self = Self(53i32);
    pub const ChineseFullWidth: Self = Self(54i32);
    pub const NativeScript: Self = Self(55i32);
    pub const Text: Self = Self(57i32);
    pub const Chat: Self = Self(58i32);
    pub const NameOrPhoneNumber: Self = Self(59i32);
    pub const EmailNameOrAddress: Self = Self(60i32);
    pub const Private: Self = Self(61i32);
    pub const Maps: Self = Self(62i32);
    pub const NumericPassword: Self = Self(63i32);
    pub const NumericPin: Self = Self(64i32);
    pub const AlphanumericPin: Self = Self(65i32);
    pub const FormulaNumber: Self = Self(67i32);
    pub const ChatWithoutEmoji: Self = Self(68i32);
}
impl ::core::marker::Copy for InputScopeNameValue {}
impl ::core::clone::Clone for InputScopeNameValue {
    fn clone(&self) -> Self {
        *self
    }
}
pub type KeyEventHandler = *mut ::core::ffi::c_void;
pub type KeyRoutedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct KeyTipPlacementMode(pub i32);
impl KeyTipPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Bottom: Self = Self(1i32);
    pub const Top: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
    pub const Center: Self = Self(5i32);
    pub const Hidden: Self = Self(6i32);
}
impl ::core::marker::Copy for KeyTipPlacementMode {}
impl ::core::clone::Clone for KeyTipPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type KeyboardAccelerator = *mut ::core::ffi::c_void;
pub type KeyboardAcceleratorInvokedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct KeyboardAcceleratorPlacementMode(pub i32);
impl KeyboardAcceleratorPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
}
impl ::core::marker::Copy for KeyboardAcceleratorPlacementMode {}
impl ::core::clone::Clone for KeyboardAcceleratorPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct KeyboardNavigationMode(pub i32);
impl KeyboardNavigationMode {
    pub const Local: Self = Self(0i32);
    pub const Cycle: Self = Self(1i32);
    pub const Once: Self = Self(2i32);
}
impl ::core::marker::Copy for KeyboardNavigationMode {}
impl ::core::clone::Clone for KeyboardNavigationMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LosingFocusEventArgs = *mut ::core::ffi::c_void;
pub type ManipulationCompletedEventHandler = *mut ::core::ffi::c_void;
pub type ManipulationCompletedRoutedEventArgs = *mut ::core::ffi::c_void;
pub type ManipulationDeltaEventHandler = *mut ::core::ffi::c_void;
pub type ManipulationDeltaRoutedEventArgs = *mut ::core::ffi::c_void;
pub type ManipulationInertiaStartingEventHandler = *mut ::core::ffi::c_void;
pub type ManipulationInertiaStartingRoutedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationModes(pub u32);
impl ManipulationModes {
    pub const None: Self = Self(0u32);
    pub const TranslateX: Self = Self(1u32);
    pub const TranslateY: Self = Self(2u32);
    pub const TranslateRailsX: Self = Self(4u32);
    pub const TranslateRailsY: Self = Self(8u32);
    pub const Rotate: Self = Self(16u32);
    pub const Scale: Self = Self(32u32);
    pub const TranslateInertia: Self = Self(64u32);
    pub const RotateInertia: Self = Self(128u32);
    pub const ScaleInertia: Self = Self(256u32);
    pub const All: Self = Self(65535u32);
    pub const System: Self = Self(65536u32);
}
impl ::core::marker::Copy for ManipulationModes {}
impl ::core::clone::Clone for ManipulationModes {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ManipulationPivot = *mut ::core::ffi::c_void;
pub type ManipulationStartedEventHandler = *mut ::core::ffi::c_void;
pub type ManipulationStartedRoutedEventArgs = *mut ::core::ffi::c_void;
pub type ManipulationStartingEventHandler = *mut ::core::ffi::c_void;
pub type ManipulationStartingRoutedEventArgs = *mut ::core::ffi::c_void;
pub type NoFocusCandidateFoundEventArgs = *mut ::core::ffi::c_void;
pub type Pointer = *mut ::core::ffi::c_void;
pub type PointerEventHandler = *mut ::core::ffi::c_void;
pub type PointerRoutedEventArgs = *mut ::core::ffi::c_void;
pub type ProcessKeyboardAcceleratorEventArgs = *mut ::core::ffi::c_void;
pub type RightTappedEventHandler = *mut ::core::ffi::c_void;
pub type RightTappedRoutedEventArgs = *mut ::core::ffi::c_void;
pub type StandardUICommand = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct StandardUICommandKind(pub i32);
impl StandardUICommandKind {
    pub const None: Self = Self(0i32);
    pub const Cut: Self = Self(1i32);
    pub const Copy: Self = Self(2i32);
    pub const Paste: Self = Self(3i32);
    pub const SelectAll: Self = Self(4i32);
    pub const Delete: Self = Self(5i32);
    pub const Share: Self = Self(6i32);
    pub const Save: Self = Self(7i32);
    pub const Open: Self = Self(8i32);
    pub const Close: Self = Self(9i32);
    pub const Pause: Self = Self(10i32);
    pub const Play: Self = Self(11i32);
    pub const Stop: Self = Self(12i32);
    pub const Forward: Self = Self(13i32);
    pub const Backward: Self = Self(14i32);
    pub const Undo: Self = Self(15i32);
    pub const Redo: Self = Self(16i32);
}
impl ::core::marker::Copy for StandardUICommandKind {}
impl ::core::clone::Clone for StandardUICommandKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TappedEventHandler = *mut ::core::ffi::c_void;
pub type TappedRoutedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct XYFocusKeyboardNavigationMode(pub i32);
impl XYFocusKeyboardNavigationMode {
    pub const Auto: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
impl ::core::marker::Copy for XYFocusKeyboardNavigationMode {}
impl ::core::clone::Clone for XYFocusKeyboardNavigationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct XYFocusNavigationStrategy(pub i32);
impl XYFocusNavigationStrategy {
    pub const Auto: Self = Self(0i32);
    pub const Projection: Self = Self(1i32);
    pub const NavigationDirectionDistance: Self = Self(2i32);
    pub const RectilinearDistance: Self = Self(3i32);
}
impl ::core::marker::Copy for XYFocusNavigationStrategy {}
impl ::core::clone::Clone for XYFocusNavigationStrategy {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct XYFocusNavigationStrategyOverride(pub i32);
impl XYFocusNavigationStrategyOverride {
    pub const None: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const Projection: Self = Self(2i32);
    pub const NavigationDirectionDistance: Self = Self(3i32);
    pub const RectilinearDistance: Self = Self(4i32);
}
impl ::core::marker::Copy for XYFocusNavigationStrategyOverride {}
impl ::core::clone::Clone for XYFocusNavigationStrategyOverride {
    fn clone(&self) -> Self {
        *self
    }
}
pub type XamlUICommand = *mut ::core::ffi::c_void;
