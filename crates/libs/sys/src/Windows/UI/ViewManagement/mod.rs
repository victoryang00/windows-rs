#[cfg(feature = "UI_ViewManagement_Core")]
pub mod Core;
pub type AccessibilitySettings = *mut ::core::ffi::c_void;
pub type ActivationViewSwitcher = *mut ::core::ffi::c_void;
pub type ApplicationView = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct ApplicationViewBoundsMode(pub i32);
impl ApplicationViewBoundsMode {
    pub const UseVisible: Self = Self(0i32);
    pub const UseCoreWindow: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationViewBoundsMode {}
impl ::core::clone::Clone for ApplicationViewBoundsMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ApplicationViewConsolidatedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct ApplicationViewMode(pub i32);
impl ApplicationViewMode {
    pub const Default: Self = Self(0i32);
    pub const CompactOverlay: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationViewMode {}
impl ::core::clone::Clone for ApplicationViewMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct ApplicationViewOrientation(pub i32);
impl ApplicationViewOrientation {
    pub const Landscape: Self = Self(0i32);
    pub const Portrait: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationViewOrientation {}
impl ::core::clone::Clone for ApplicationViewOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ApplicationViewScaling = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_ViewManagement\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ApplicationViewState(pub i32);
#[cfg(feature = "deprecated")]
impl ApplicationViewState {
    pub const FullScreenLandscape: Self = Self(0i32);
    pub const Filled: Self = Self(1i32);
    pub const Snapped: Self = Self(2i32);
    pub const FullScreenPortrait: Self = Self(3i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for ApplicationViewState {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ApplicationViewState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct ApplicationViewSwitchingOptions(pub u32);
impl ApplicationViewSwitchingOptions {
    pub const Default: Self = Self(0u32);
    pub const SkipAnimation: Self = Self(1u32);
    pub const ConsolidateViews: Self = Self(2u32);
}
impl ::core::marker::Copy for ApplicationViewSwitchingOptions {}
impl ::core::clone::Clone for ApplicationViewSwitchingOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ApplicationViewTitleBar = *mut ::core::ffi::c_void;
pub type ApplicationViewTransferContext = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct ApplicationViewWindowingMode(pub i32);
impl ApplicationViewWindowingMode {
    pub const Auto: Self = Self(0i32);
    pub const PreferredLaunchViewSize: Self = Self(1i32);
    pub const FullScreen: Self = Self(2i32);
    pub const CompactOverlay: Self = Self(3i32);
    pub const Maximized: Self = Self(4i32);
}
impl ::core::marker::Copy for ApplicationViewWindowingMode {}
impl ::core::clone::Clone for ApplicationViewWindowingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct FullScreenSystemOverlayMode(pub i32);
impl FullScreenSystemOverlayMode {
    pub const Standard: Self = Self(0i32);
    pub const Minimal: Self = Self(1i32);
}
impl ::core::marker::Copy for FullScreenSystemOverlayMode {}
impl ::core::clone::Clone for FullScreenSystemOverlayMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct HandPreference(pub i32);
impl HandPreference {
    pub const LeftHanded: Self = Self(0i32);
    pub const RightHanded: Self = Self(1i32);
}
impl ::core::marker::Copy for HandPreference {}
impl ::core::clone::Clone for HandPreference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAccessibilitySettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub HighContrast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HighContrastScheme: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HighContrastChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HighContrastChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHighContrastChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHighContrastChanged: usize,
}
#[repr(C)]
pub struct IActivationViewSwitcher {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShowAsStandaloneAsync: unsafe extern "system" fn(this: *mut *mut Self, viewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsStandaloneAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAsStandaloneWithSizePreferenceAsync: unsafe extern "system" fn(this: *mut *mut Self, viewid: i32, sizepreference: ViewSizePreference, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsStandaloneWithSizePreferenceAsync: usize,
    pub IsViewPresentedOnActivationVirtualDesktop: unsafe extern "system" fn(this: *mut *mut Self, viewid: i32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationView {
    pub base__: ::windows_sys::core::IInspectable,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ApplicationViewOrientation) -> ::windows_sys::core::HRESULT,
    pub AdjacentToLeftDisplayEdge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AdjacentToRightDisplayEdge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsFullScreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsFullScreen: usize,
    pub IsOnLockScreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsScreenCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsScreenCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Consolidated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Consolidated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConsolidated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConsolidated: usize,
}
#[repr(C)]
pub struct IApplicationView2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub SuppressSystemOverlays: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SuppressSystemOverlays: usize,
    #[cfg(feature = "deprecated")]
    pub SetSuppressSystemOverlays: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSuppressSystemOverlays: usize,
    #[cfg(feature = "Foundation")]
    pub VisibleBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisibleBounds: usize,
    #[cfg(feature = "Foundation")]
    pub VisibleBoundsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisibleBoundsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVisibleBoundsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVisibleBoundsChanged: usize,
    pub SetDesiredBoundsMode: unsafe extern "system" fn(this: *mut *mut Self, boundsmode: ApplicationViewBoundsMode, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DesiredBoundsMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ApplicationViewBoundsMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationView3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TitleBar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FullScreenSystemOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FullScreenSystemOverlayMode) -> ::windows_sys::core::HRESULT,
    pub SetFullScreenSystemOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, value: FullScreenSystemOverlayMode) -> ::windows_sys::core::HRESULT,
    pub IsFullScreenMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TryEnterFullScreenMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ExitFullScreenMode: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ShowStandardSystemOverlays: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryResizeView: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Size, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryResizeView: usize,
    #[cfg(feature = "Foundation")]
    pub SetPreferredMinSize: unsafe extern "system" fn(this: *mut *mut Self, minsize: super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredMinSize: usize,
}
#[repr(C)]
pub struct IApplicationView4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViewMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ApplicationViewMode) -> ::windows_sys::core::HRESULT,
    pub IsViewModeSupported: unsafe extern "system" fn(this: *mut *mut Self, viewmode: ApplicationViewMode, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryEnterViewModeAsync: unsafe extern "system" fn(this: *mut *mut Self, viewmode: ApplicationViewMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryEnterViewModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryEnterViewModeWithPreferencesAsync: unsafe extern "system" fn(this: *mut *mut Self, viewmode: ApplicationViewMode, viewmodepreferences: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryEnterViewModeWithPreferencesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryConsolidateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryConsolidateAsync: usize,
}
#[repr(C)]
pub struct IApplicationView7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PersistedStateId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPersistedStateId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationView9 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_WindowManagement")]
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    WindowingEnvironment: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_WindowManagement"))]
    pub GetDisplayRegions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_WindowManagement")))]
    GetDisplayRegions: usize,
}
#[repr(C)]
pub struct IApplicationViewConsolidatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsUserInitiated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationViewConsolidatedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAppInitiated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IApplicationViewFullscreenStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub TryUnsnapToFullscreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryUnsnapToFullscreen: usize,
}
#[repr(C)]
pub struct IApplicationViewInteropStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Core")]
    pub GetApplicationViewIdForWindow: unsafe extern "system" fn(this: *mut *mut Self, window: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    GetApplicationViewIdForWindow: usize,
}
#[repr(C)]
pub struct IApplicationViewScaling {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IApplicationViewScalingStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisableLayoutScaling: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TrySetDisableLayoutScaling: unsafe extern "system" fn(this: *mut *mut Self, disablelayoutscaling: bool, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IApplicationViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ApplicationViewState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Value: usize,
    #[cfg(feature = "deprecated")]
    pub TryUnsnap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryUnsnap: usize,
}
#[repr(C)]
pub struct IApplicationViewStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TerminateAppOnFinalViewClose: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetTerminateAppOnFinalViewClose: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationViewStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PreferredLaunchWindowingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ApplicationViewWindowingMode) -> ::windows_sys::core::HRESULT,
    pub SetPreferredLaunchWindowingMode: unsafe extern "system" fn(this: *mut *mut Self, value: ApplicationViewWindowingMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PreferredLaunchViewSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreferredLaunchViewSize: usize,
    #[cfg(feature = "Foundation")]
    pub SetPreferredLaunchViewSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredLaunchViewSize: usize,
}
#[repr(C)]
pub struct IApplicationViewStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClearAllPersistedState: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ClearPersistedState: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationViewSwitcherStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisableShowingMainViewOnActivation: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryShowAsStandaloneAsync: unsafe extern "system" fn(this: *mut *mut Self, viewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowAsStandaloneAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryShowAsStandaloneWithSizePreferenceAsync: unsafe extern "system" fn(this: *mut *mut Self, viewid: i32, sizepreference: ViewSizePreference, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowAsStandaloneWithSizePreferenceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync: unsafe extern "system" fn(this: *mut *mut Self, viewid: i32, sizepreference: ViewSizePreference, anchorviewid: i32, anchorsizepreference: ViewSizePreference, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SwitchAsync: unsafe extern "system" fn(this: *mut *mut Self, viewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SwitchAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SwitchFromViewAsync: unsafe extern "system" fn(this: *mut *mut Self, toviewid: i32, fromviewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SwitchFromViewAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SwitchFromViewWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SwitchFromViewWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PrepareForCustomAnimatedSwitchAsync: unsafe extern "system" fn(this: *mut *mut Self, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrepareForCustomAnimatedSwitchAsync: usize,
}
#[repr(C)]
pub struct IApplicationViewSwitcherStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisableSystemViewActivationPolicy: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationViewSwitcherStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TryShowAsViewModeAsync: unsafe extern "system" fn(this: *mut *mut Self, viewid: i32, viewmode: ApplicationViewMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowAsViewModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryShowAsViewModeWithPreferencesAsync: unsafe extern "system" fn(this: *mut *mut Self, viewid: i32, viewmode: ApplicationViewMode, viewmodepreferences: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowAsViewModeWithPreferencesAsync: usize,
}
#[repr(C)]
pub struct IApplicationViewTitleBar {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonHoverForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonHoverForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonHoverForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonHoverForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonHoverBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonHoverBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonHoverBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonHoverBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonPressedForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonPressedForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonPressedForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonPressedForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonPressedBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonPressedBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonPressedBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonPressedBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetInactiveForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub InactiveForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetInactiveBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub InactiveBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonInactiveForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonInactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonInactiveForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonInactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonInactiveBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonInactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonInactiveBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonInactiveBackgroundColor: usize,
}
#[repr(C)]
pub struct IApplicationViewTransferContext {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViewId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetViewId: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationViewTransferContextStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataPackageFormatId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationViewWithContext {
    pub base__: ::windows_sys::core::IInspectable,
    pub UIContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInputPane {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Showing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Showing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShowing: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShowing: usize,
    #[cfg(feature = "Foundation")]
    pub Hiding: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Hiding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHiding: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHiding: usize,
    #[cfg(feature = "Foundation")]
    pub OccludedRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OccludedRect: usize,
}
#[repr(C)]
pub struct IInputPane2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryShow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TryHide: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInputPaneControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInputPaneStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInputPaneStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForUIContext: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInputPaneVisibilityEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OccludedRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OccludedRect: usize,
    pub SetEnsuredFocusedElementInView: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub EnsuredFocusedElementInView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProjectionManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartProjectingAsync: unsafe extern "system" fn(this: *mut *mut Self, projectionviewid: i32, anchorviewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartProjectingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SwapDisplaysForViewsAsync: unsafe extern "system" fn(this: *mut *mut Self, projectionviewid: i32, anchorviewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SwapDisplaysForViewsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopProjectingAsync: unsafe extern "system" fn(this: *mut *mut Self, projectionviewid: i32, anchorviewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopProjectingAsync: usize,
    pub ProjectionDisplayAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProjectionDisplayAvailableChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProjectionDisplayAvailableChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProjectionDisplayAvailableChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProjectionDisplayAvailableChanged: usize,
}
#[repr(C)]
pub struct IProjectionManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub StartProjectingWithDeviceInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, projectionviewid: i32, anchorviewid: i32, displaydeviceinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    StartProjectingWithDeviceInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStartProjectingAsync: unsafe extern "system" fn(this: *mut *mut Self, projectionviewid: i32, anchorviewid: i32, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStartProjectingAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub RequestStartProjectingWithPlacementAsync: unsafe extern "system" fn(this: *mut *mut Self, projectionviewid: i32, anchorviewid: i32, selection: super::super::Foundation::Rect, prefferedplacement: super::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    RequestStartProjectingWithPlacementAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStatusBar {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShowAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsync: usize,
    #[cfg(feature = "Foundation")]
    pub HideAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HideAsync: usize,
    pub BackgroundOpacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBackgroundColor: usize,
    pub ProgressIndicator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OccludedRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OccludedRect: usize,
    #[cfg(feature = "Foundation")]
    pub Showing: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Showing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShowing: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShowing: usize,
    #[cfg(feature = "Foundation")]
    pub Hiding: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Hiding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHiding: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHiding: usize,
}
#[repr(C)]
pub struct IStatusBarProgressIndicator {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShowAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsync: usize,
    #[cfg(feature = "Foundation")]
    pub HideAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HideAsync: usize,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProgressValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProgressValue: usize,
    #[cfg(feature = "Foundation")]
    pub SetProgressValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProgressValue: usize,
}
#[repr(C)]
pub struct IStatusBarStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUISettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub HandPreference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HandPreference) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CursorSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CursorSize: usize,
    #[cfg(feature = "Foundation")]
    pub ScrollBarSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScrollBarSize: usize,
    #[cfg(feature = "Foundation")]
    pub ScrollBarArrowSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScrollBarArrowSize: usize,
    #[cfg(feature = "Foundation")]
    pub ScrollBarThumbBoxSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScrollBarThumbBoxSize: usize,
    pub MessageDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AnimationsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CaretBrowsingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CaretBlinkRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CaretWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DoubleClickTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MouseHoverTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UIElementColor: unsafe extern "system" fn(this: *mut *mut Self, desiredelement: UIElementType, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUISettings2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextScaleFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TextScaleFactorChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextScaleFactorChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextScaleFactorChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextScaleFactorChanged: usize,
}
#[repr(C)]
pub struct IUISettings3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetColorValue: unsafe extern "system" fn(this: *mut *mut Self, desiredcolor: UIColorType, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ColorValuesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ColorValuesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveColorValuesChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveColorValuesChanged: usize,
}
#[repr(C)]
pub struct IUISettings4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AdvancedEffectsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AdvancedEffectsEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AdvancedEffectsEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdvancedEffectsEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdvancedEffectsEnabledChanged: usize,
}
#[repr(C)]
pub struct IUISettings5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AutoHideScrollBars: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AutoHideScrollBarsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoHideScrollBarsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAutoHideScrollBarsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAutoHideScrollBarsChanged: usize,
}
#[repr(C)]
pub struct IUISettings6 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AnimationsEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnimationsEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAnimationsEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAnimationsEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub MessageDurationChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageDurationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageDurationChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageDurationChanged: usize,
}
#[repr(C)]
pub struct IUISettingsAnimationsEnabledChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IUISettingsAutoHideScrollBarsChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IUISettingsMessageDurationChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IUIViewSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub UserInteractionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserInteractionMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIViewSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IViewModePreferences {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViewSizePreference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ViewSizePreference) -> ::windows_sys::core::HRESULT,
    pub SetViewSizePreference: unsafe extern "system" fn(this: *mut *mut Self, value: ViewSizePreference) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CustomSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CustomSize: usize,
    #[cfg(feature = "Foundation")]
    pub SetCustomSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCustomSize: usize,
}
#[repr(C)]
pub struct IViewModePreferencesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateDefault: unsafe extern "system" fn(this: *mut *mut Self, mode: ApplicationViewMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type InputPane = *mut ::core::ffi::c_void;
pub type InputPaneVisibilityEventArgs = *mut ::core::ffi::c_void;
pub type StatusBar = *mut ::core::ffi::c_void;
pub type StatusBarProgressIndicator = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct UIColorType(pub i32);
impl UIColorType {
    pub const Background: Self = Self(0i32);
    pub const Foreground: Self = Self(1i32);
    pub const AccentDark3: Self = Self(2i32);
    pub const AccentDark2: Self = Self(3i32);
    pub const AccentDark1: Self = Self(4i32);
    pub const Accent: Self = Self(5i32);
    pub const AccentLight1: Self = Self(6i32);
    pub const AccentLight2: Self = Self(7i32);
    pub const AccentLight3: Self = Self(8i32);
    pub const Complement: Self = Self(9i32);
}
impl ::core::marker::Copy for UIColorType {}
impl ::core::clone::Clone for UIColorType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct UIElementType(pub i32);
impl UIElementType {
    pub const ActiveCaption: Self = Self(0i32);
    pub const Background: Self = Self(1i32);
    pub const ButtonFace: Self = Self(2i32);
    pub const ButtonText: Self = Self(3i32);
    pub const CaptionText: Self = Self(4i32);
    pub const GrayText: Self = Self(5i32);
    pub const Highlight: Self = Self(6i32);
    pub const HighlightText: Self = Self(7i32);
    pub const Hotlight: Self = Self(8i32);
    pub const InactiveCaption: Self = Self(9i32);
    pub const InactiveCaptionText: Self = Self(10i32);
    pub const Window: Self = Self(11i32);
    pub const WindowText: Self = Self(12i32);
    pub const AccentColor: Self = Self(1000i32);
    pub const TextHigh: Self = Self(1001i32);
    pub const TextMedium: Self = Self(1002i32);
    pub const TextLow: Self = Self(1003i32);
    pub const TextContrastWithHigh: Self = Self(1004i32);
    pub const NonTextHigh: Self = Self(1005i32);
    pub const NonTextMediumHigh: Self = Self(1006i32);
    pub const NonTextMedium: Self = Self(1007i32);
    pub const NonTextMediumLow: Self = Self(1008i32);
    pub const NonTextLow: Self = Self(1009i32);
    pub const PageBackground: Self = Self(1010i32);
    pub const PopupBackground: Self = Self(1011i32);
    pub const OverlayOutsidePopup: Self = Self(1012i32);
}
impl ::core::marker::Copy for UIElementType {}
impl ::core::clone::Clone for UIElementType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UISettings = *mut ::core::ffi::c_void;
pub type UISettingsAnimationsEnabledChangedEventArgs = *mut ::core::ffi::c_void;
pub type UISettingsAutoHideScrollBarsChangedEventArgs = *mut ::core::ffi::c_void;
pub type UISettingsMessageDurationChangedEventArgs = *mut ::core::ffi::c_void;
pub type UIViewSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct UserInteractionMode(pub i32);
impl UserInteractionMode {
    pub const Mouse: Self = Self(0i32);
    pub const Touch: Self = Self(1i32);
}
impl ::core::marker::Copy for UserInteractionMode {}
impl ::core::clone::Clone for UserInteractionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ViewModePreferences = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct ViewSizePreference(pub i32);
impl ViewSizePreference {
    pub const Default: Self = Self(0i32);
    pub const UseLess: Self = Self(1i32);
    pub const UseHalf: Self = Self(2i32);
    pub const UseMore: Self = Self(3i32);
    pub const UseMinimum: Self = Self(4i32);
    pub const UseNone: Self = Self(5i32);
    pub const Custom: Self = Self(6i32);
}
impl ::core::marker::Copy for ViewSizePreference {}
impl ::core::clone::Clone for ViewSizePreference {
    fn clone(&self) -> Self {
        *self
    }
}
