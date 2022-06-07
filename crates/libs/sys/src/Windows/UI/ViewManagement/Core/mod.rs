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
impl ::windows_sys::core::Interface for ICoreFrameworkInputView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3615265966, data2: 18104, data3: 23882, data4: [148, 137, 141, 222, 195, 214, 57, 166] };
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
impl ::windows_sys::core::Interface for ICoreFrameworkInputViewAnimationStartingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3236728860, data2: 48036, data3: 20507, data4: [174, 139, 101, 201, 231, 86, 167, 25] };
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
impl ::windows_sys::core::Interface for ICoreFrameworkInputViewOcclusionsChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4084156745, data2: 51244, data3: 21457, data4: [167, 93, 43, 43, 175, 13, 155, 13] };
}
#[repr(C)]
pub struct ICoreFrameworkInputViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForUIContext: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreFrameworkInputViewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1860950454, data2: 60098, data3: 24459, data4: [151, 95, 119, 46, 227, 228, 46, 235] };
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
impl ::windows_sys::core::Interface for ICoreInputView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3346058618, data2: 28673, data3: 19506, data4: [191, 148, 37, 193, 245, 84, 203, 241] };
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
impl ::windows_sys::core::Interface for ICoreInputView2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 248981185, data2: 57498, data3: 19176, data4: [174, 223, 223, 164, 133, 125, 26, 1] };
}
#[repr(C)]
pub struct ICoreInputView3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryShow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TryShowWithKind: unsafe extern "system" fn(this: *mut *mut Self, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TryHide: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreInputView3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3163821651, data2: 15033, data3: 18505, data4: [143, 88, 70, 231, 240, 53, 60, 252] };
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
impl ::windows_sys::core::Interface for ICoreInputView4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2646998, data2: 55791, data3: 22507, data4: [140, 239, 119, 246, 206, 27, 126, 231] };
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
impl ::windows_sys::core::Interface for ICoreInputView5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 325261024, data2: 50901, data3: 23639, data4: [129, 30, 26, 216, 169, 155, 166, 171] };
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
impl ::windows_sys::core::Interface for ICoreInputViewAnimationStartingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2836679410, data2: 46428, data3: 24225, data4: [184, 171, 83, 64, 243, 233, 72, 151] };
}
#[repr(C)]
pub struct ICoreInputViewHidingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryCancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreInputViewHidingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3940173757, data2: 47813, data3: 21302, data4: [132, 141, 65, 8, 53, 132, 218, 173] };
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
impl ::windows_sys::core::Interface for ICoreInputViewOcclusion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3426143750, data2: 14437, data3: 16759, data4: [181, 245, 139, 101, 224, 185, 206, 132] };
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
impl ::windows_sys::core::Interface for ICoreInputViewOcclusionsChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3188729832, data2: 46062, data3: 19959, data4: [149, 84, 137, 205, 198, 96, 130, 194] };
}
#[repr(C)]
pub struct ICoreInputViewShowingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryCancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreInputViewShowingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3394381339, data2: 64414, data3: 23983, data4: [169, 140, 38, 43, 139, 118, 175, 80] };
}
#[repr(C)]
pub struct ICoreInputViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreInputViewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2107348941, data2: 60862, data3: 18895, data4: [165, 79, 51, 125, 224, 82, 144, 127] };
}
#[repr(C)]
pub struct ICoreInputViewStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForUIContext: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreInputViewStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2126252130, data2: 53321, data3: 20050, data4: [135, 176, 30, 144, 233, 140, 73, 237] };
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
impl ::windows_sys::core::Interface for ICoreInputViewTransferringXYFocusEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 81663647, data2: 47618, data3: 18512, data4: [139, 85, 216, 45, 3, 186, 109, 127] };
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
impl ::windows_sys::core::Interface for IUISettingsController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2024086212, data2: 5568, data3: 23067, data4: [167, 91, 172, 191, 156, 184, 187, 158] };
}
#[repr(C)]
pub struct IUISettingsControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDefaultAsync: usize,
}
impl ::windows_sys::core::Interface for IUISettingsControllerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3946604748, data2: 49696, data3: 22412, data4: [129, 25, 125, 179, 36, 237, 38, 166] };
}
pub type UISettingsController = *mut ::core::ffi::c_void;
