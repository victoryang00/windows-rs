pub type CoreFrameworkInputView = *mut ::core::ffi::c_void;
pub type CoreFrameworkInputViewAnimationStartingEventArgs = *mut ::core::ffi::c_void;
pub type CoreFrameworkInputViewOcclusionsChangedEventArgs = *mut ::core::ffi::c_void;
pub type CoreInputView = *mut ::core::ffi::c_void;
pub type CoreInputViewAnimationStartingEventArgs = *mut ::core::ffi::c_void;
pub type CoreInputViewHidingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct CoreInputViewKind(pub i32);
impl CoreInputViewKind {
    pub const Default: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Handwriting: Self = Self(2i32);
    pub const Emoji: Self = Self(3i32);
    pub const Symbols: Self = Self(4i32);
    pub const Clipboard: Self = Self(5i32);
    pub const Dictation: Self = Self(6i32);
}
impl ::core::marker::Copy for CoreInputViewKind {}
impl ::core::clone::Clone for CoreInputViewKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreInputViewOcclusion = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct CoreInputViewOcclusionKind(pub i32);
impl CoreInputViewOcclusionKind {
    pub const Docked: Self = Self(0i32);
    pub const Floating: Self = Self(1i32);
    pub const Overlay: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreInputViewOcclusionKind {}
impl ::core::clone::Clone for CoreInputViewOcclusionKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreInputViewOcclusionsChangedEventArgs = *mut ::core::ffi::c_void;
pub type CoreInputViewShowingEventArgs = *mut ::core::ffi::c_void;
pub type CoreInputViewTransferringXYFocusEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_ViewManagement_Core\"`*"]
#[repr(transparent)]
pub struct CoreInputViewXYFocusTransferDirection(pub i32);
impl CoreInputViewXYFocusTransferDirection {
    pub const Up: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
    pub const Down: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreInputViewXYFocusTransferDirection {}
impl ::core::clone::Clone for CoreInputViewXYFocusTransferDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICoreFrameworkInputView {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PrimaryViewAnimationStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryViewAnimationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrimaryViewAnimationStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrimaryViewAnimationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub OcclusionsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OcclusionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOcclusionsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOcclusionsChanged: usize,
}
#[repr(C)]
pub struct ICoreFrameworkInputViewAnimationStartingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub FrameworkAnimationRecommended: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AnimationDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnimationDuration: usize,
}
#[repr(C)]
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreFrameworkInputViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForUIContext: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreInputView {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OcclusionsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OcclusionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOcclusionsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOcclusionsChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCoreInputViewOcclusions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCoreInputViewOcclusions: usize,
    pub TryShowPrimaryView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TryHidePrimaryView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreInputView2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub XYFocusTransferringFromPrimaryView: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    XYFocusTransferringFromPrimaryView: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveXYFocusTransferringFromPrimaryView: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveXYFocusTransferringFromPrimaryView: usize,
    #[cfg(feature = "Foundation")]
    pub XYFocusTransferredToPrimaryView: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    XYFocusTransferredToPrimaryView: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveXYFocusTransferredToPrimaryView: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveXYFocusTransferredToPrimaryView: usize,
    #[cfg(feature = "Foundation")]
    pub TryTransferXYFocusToPrimaryView: unsafe extern "system" fn(this: *mut *mut Self, origin: super::super::super::Foundation::Rect, direction: CoreInputViewXYFocusTransferDirection, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTransferXYFocusToPrimaryView: usize,
}
#[repr(C)]
pub struct ICoreInputView3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryShow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TryShowWithKind: unsafe extern "system" fn(this: *mut *mut Self, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TryHide: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreInputView4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PrimaryViewShowing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryViewShowing: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrimaryViewShowing: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrimaryViewShowing: usize,
    #[cfg(feature = "Foundation")]
    pub PrimaryViewHiding: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryViewHiding: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrimaryViewHiding: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrimaryViewHiding: usize,
}
#[repr(C)]
pub struct ICoreInputView5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsKindSupported: unsafe extern "system" fn(this: *mut *mut Self, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SupportedKindsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SupportedKindsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSupportedKindsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSupportedKindsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PrimaryViewAnimationStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryViewAnimationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrimaryViewAnimationStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrimaryViewAnimationStarting: usize,
}
#[repr(C)]
pub struct ICoreInputViewAnimationStartingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AnimationDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnimationDuration: usize,
}
#[repr(C)]
pub struct ICoreInputViewHidingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryCancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreInputViewOcclusion {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OccludingRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OccludingRect: usize,
    pub OcclusionKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreInputViewOcclusionKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreInputViewOcclusionsChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreInputViewShowingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryCancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreInputViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreInputViewStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForUIContext: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreInputViewTransferringXYFocusEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Origin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Origin: usize,
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreInputViewXYFocusTransferDirection) -> ::windows_sys::core::HRESULT,
    pub SetTransferHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TransferHandled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetKeepPrimaryViewVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub KeepPrimaryViewVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUISettingsController {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAdvancedEffectsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetAnimationsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoHideScrollBars: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetMessageDuration: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub SetTextScaleFactor: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUISettingsControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDefaultAsync: usize,
}
pub type UISettingsController = *mut ::core::ffi::c_void;
