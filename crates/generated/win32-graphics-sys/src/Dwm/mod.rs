#[link(name = "windows")]
extern "system" {
    pub fn DwmAttachMilContent(hwnd: ::win32_foundation_sys::HWND) -> ::windows_core_sys::HRESULT;
    pub fn DwmDefWindowProc(hwnd: ::win32_foundation_sys::HWND, msg: u32, wparam: ::win32_foundation_sys::WPARAM, lparam: ::win32_foundation_sys::LPARAM, plresult: *mut ::win32_foundation_sys::LRESULT) -> ::win32_foundation_sys::BOOL;
    pub fn DwmDetachMilContent(hwnd: ::win32_foundation_sys::HWND) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-graphics-sys")]
    pub fn DwmEnableBlurBehindWindow(hwnd: ::win32_foundation_sys::HWND, pblurbehind: *const DWM_BLURBEHIND) -> ::windows_core_sys::HRESULT;
    pub fn DwmEnableComposition(ucompositionaction: u32) -> ::windows_core_sys::HRESULT;
    pub fn DwmEnableMMCSS(fenablemmcss: ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-ui-sys")]
    pub fn DwmExtendFrameIntoClientArea(hwnd: ::win32_foundation_sys::HWND, pmarinset: *const ::win32_ui_sys::Controls::MARGINS) -> ::windows_core_sys::HRESULT;
    pub fn DwmFlush() -> ::windows_core_sys::HRESULT;
    pub fn DwmGetColorizationColor(pcrcolorization: *mut u32, pfopaqueblend: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn DwmGetCompositionTimingInfo(hwnd: ::win32_foundation_sys::HWND, ptiminginfo: *mut DWM_TIMING_INFO) -> ::windows_core_sys::HRESULT;
    pub fn DwmGetGraphicsStreamClient(uindex: u32, pclientuuid: *mut ::windows_core_sys::GUID) -> ::windows_core_sys::HRESULT;
    pub fn DwmGetGraphicsStreamTransformHint(uindex: u32, ptransform: *mut MilMatrix3x2D) -> ::windows_core_sys::HRESULT;
    pub fn DwmGetTransportAttributes(pfisremoting: *mut ::win32_foundation_sys::BOOL, pfisconnected: *mut ::win32_foundation_sys::BOOL, pdwgeneration: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DwmGetUnmetTabRequirements(appwindow: ::win32_foundation_sys::HWND, value: *mut DWM_TAB_WINDOW_REQUIREMENTS) -> ::windows_core_sys::HRESULT;
    pub fn DwmGetWindowAttribute(hwnd: ::win32_foundation_sys::HWND, dwattribute: DWMWINDOWATTRIBUTE, pvattribute: *mut ::core::ffi::c_void, cbattribute: u32) -> ::windows_core_sys::HRESULT;
    pub fn DwmInvalidateIconicBitmaps(hwnd: ::win32_foundation_sys::HWND) -> ::windows_core_sys::HRESULT;
    pub fn DwmIsCompositionEnabled(pfenabled: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn DwmModifyPreviousDxFrameDuration(hwnd: ::win32_foundation_sys::HWND, crefreshes: i32, frelative: ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn DwmQueryThumbnailSourceSize(hthumbnail: isize, psize: *mut ::win32_foundation_sys::SIZE) -> ::windows_core_sys::HRESULT;
    pub fn DwmRegisterThumbnail(hwnddestination: ::win32_foundation_sys::HWND, hwndsource: ::win32_foundation_sys::HWND, phthumbnailid: *mut isize) -> ::windows_core_sys::HRESULT;
    pub fn DwmRenderGesture(gt: GESTURE_TYPE, ccontacts: u32, pdwpointerid: *const u32, ppoints: *const ::win32_foundation_sys::POINT) -> ::windows_core_sys::HRESULT;
    pub fn DwmSetDxFrameDuration(hwnd: ::win32_foundation_sys::HWND, crefreshes: i32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-graphics-sys")]
    pub fn DwmSetIconicLivePreviewBitmap(hwnd: ::win32_foundation_sys::HWND, hbmp: super::Gdi::HBITMAP, pptclient: *const ::win32_foundation_sys::POINT, dwsitflags: u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-graphics-sys")]
    pub fn DwmSetIconicThumbnail(hwnd: ::win32_foundation_sys::HWND, hbmp: super::Gdi::HBITMAP, dwsitflags: u32) -> ::windows_core_sys::HRESULT;
    pub fn DwmSetPresentParameters(hwnd: ::win32_foundation_sys::HWND, ppresentparams: *mut DWM_PRESENT_PARAMETERS) -> ::windows_core_sys::HRESULT;
    pub fn DwmSetWindowAttribute(hwnd: ::win32_foundation_sys::HWND, dwattribute: DWMWINDOWATTRIBUTE, pvattribute: *const ::core::ffi::c_void, cbattribute: u32) -> ::windows_core_sys::HRESULT;
    pub fn DwmShowContact(dwpointerid: u32, eshowcontact: DWM_SHOWCONTACT) -> ::windows_core_sys::HRESULT;
    pub fn DwmTetherContact(dwpointerid: u32, fenable: ::win32_foundation_sys::BOOL, pttether: ::win32_foundation_sys::POINT) -> ::windows_core_sys::HRESULT;
    pub fn DwmTransitionOwnedWindow(hwnd: ::win32_foundation_sys::HWND, target: DWMTRANSITION_OWNEDWINDOW_TARGET) -> ::windows_core_sys::HRESULT;
    pub fn DwmUnregisterThumbnail(hthumbnailid: isize) -> ::windows_core_sys::HRESULT;
    pub fn DwmUpdateThumbnailProperties(hthumbnailid: isize, ptnproperties: *const DWM_THUMBNAIL_PROPERTIES) -> ::windows_core_sys::HRESULT;
}
pub type DWMFLIP3DWINDOWPOLICY = i32;
pub const DWMFLIP3D_DEFAULT: DWMFLIP3DWINDOWPOLICY = 0i32;
pub const DWMFLIP3D_EXCLUDEBELOW: DWMFLIP3DWINDOWPOLICY = 1i32;
pub const DWMFLIP3D_EXCLUDEABOVE: DWMFLIP3DWINDOWPOLICY = 2i32;
pub const DWMFLIP3D_LAST: DWMFLIP3DWINDOWPOLICY = 3i32;
pub type DWMNCRENDERINGPOLICY = i32;
pub const DWMNCRP_USEWINDOWSTYLE: DWMNCRENDERINGPOLICY = 0i32;
pub const DWMNCRP_DISABLED: DWMNCRENDERINGPOLICY = 1i32;
pub const DWMNCRP_ENABLED: DWMNCRENDERINGPOLICY = 2i32;
pub const DWMNCRP_LAST: DWMNCRENDERINGPOLICY = 3i32;
pub type DWMTRANSITION_OWNEDWINDOW_TARGET = i32;
pub const DWMTRANSITION_OWNEDWINDOW_NULL: DWMTRANSITION_OWNEDWINDOW_TARGET = -1i32;
pub const DWMTRANSITION_OWNEDWINDOW_REPOSITION: DWMTRANSITION_OWNEDWINDOW_TARGET = 0i32;
pub const DWMWA_COLOR_DEFAULT: u32 = 4294967295u32;
pub const DWMWA_COLOR_NONE: u32 = 4294967294u32;
pub type DWMWINDOWATTRIBUTE = i32;
pub const DWMWA_NCRENDERING_ENABLED: DWMWINDOWATTRIBUTE = 1i32;
pub const DWMWA_NCRENDERING_POLICY: DWMWINDOWATTRIBUTE = 2i32;
pub const DWMWA_TRANSITIONS_FORCEDISABLED: DWMWINDOWATTRIBUTE = 3i32;
pub const DWMWA_ALLOW_NCPAINT: DWMWINDOWATTRIBUTE = 4i32;
pub const DWMWA_CAPTION_BUTTON_BOUNDS: DWMWINDOWATTRIBUTE = 5i32;
pub const DWMWA_NONCLIENT_RTL_LAYOUT: DWMWINDOWATTRIBUTE = 6i32;
pub const DWMWA_FORCE_ICONIC_REPRESENTATION: DWMWINDOWATTRIBUTE = 7i32;
pub const DWMWA_FLIP3D_POLICY: DWMWINDOWATTRIBUTE = 8i32;
pub const DWMWA_EXTENDED_FRAME_BOUNDS: DWMWINDOWATTRIBUTE = 9i32;
pub const DWMWA_HAS_ICONIC_BITMAP: DWMWINDOWATTRIBUTE = 10i32;
pub const DWMWA_DISALLOW_PEEK: DWMWINDOWATTRIBUTE = 11i32;
pub const DWMWA_EXCLUDED_FROM_PEEK: DWMWINDOWATTRIBUTE = 12i32;
pub const DWMWA_CLOAK: DWMWINDOWATTRIBUTE = 13i32;
pub const DWMWA_CLOAKED: DWMWINDOWATTRIBUTE = 14i32;
pub const DWMWA_FREEZE_REPRESENTATION: DWMWINDOWATTRIBUTE = 15i32;
pub const DWMWA_PASSIVE_UPDATE_MODE: DWMWINDOWATTRIBUTE = 16i32;
pub const DWMWA_USE_HOSTBACKDROPBRUSH: DWMWINDOWATTRIBUTE = 17i32;
pub const DWMWA_USE_IMMERSIVE_DARK_MODE: DWMWINDOWATTRIBUTE = 20i32;
pub const DWMWA_WINDOW_CORNER_PREFERENCE: DWMWINDOWATTRIBUTE = 33i32;
pub const DWMWA_BORDER_COLOR: DWMWINDOWATTRIBUTE = 34i32;
pub const DWMWA_CAPTION_COLOR: DWMWINDOWATTRIBUTE = 35i32;
pub const DWMWA_TEXT_COLOR: DWMWINDOWATTRIBUTE = 36i32;
pub const DWMWA_VISIBLE_FRAME_BORDER_THICKNESS: DWMWINDOWATTRIBUTE = 37i32;
pub const DWMWA_LAST: DWMWINDOWATTRIBUTE = 38i32;
pub const DWM_BB_BLURREGION: u32 = 2u32;
pub const DWM_BB_ENABLE: u32 = 1u32;
pub const DWM_BB_TRANSITIONONMAXIMIZED: u32 = 4u32;
#[repr(C, packed(1))]
#[cfg(feature = "win32-graphics-sys")]
pub struct DWM_BLURBEHIND {
    pub dwFlags: u32,
    pub fEnable: ::win32_foundation_sys::BOOL,
    pub hRgnBlur: super::Gdi::HRGN,
    pub fTransitionOnMaximized: ::win32_foundation_sys::BOOL,
}
#[cfg(feature = "win32-graphics-sys")]
impl ::core::marker::Copy for DWM_BLURBEHIND {}
#[cfg(feature = "win32-graphics-sys")]
impl ::core::clone::Clone for DWM_BLURBEHIND {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWM_CLOAKED_APP: u32 = 1u32;
pub const DWM_CLOAKED_INHERITED: u32 = 4u32;
pub const DWM_CLOAKED_SHELL: u32 = 2u32;
pub const DWM_EC_DISABLECOMPOSITION: u32 = 0u32;
pub const DWM_EC_ENABLECOMPOSITION: u32 = 1u32;
pub const DWM_FRAME_DURATION_DEFAULT: i32 = -1i32;
#[repr(C, packed(1))]
pub struct DWM_PRESENT_PARAMETERS {
    pub cbSize: u32,
    pub fQueue: ::win32_foundation_sys::BOOL,
    pub cRefreshStart: u64,
    pub cBuffer: u32,
    pub fUseSourceRate: ::win32_foundation_sys::BOOL,
    pub rateSource: UNSIGNED_RATIO,
    pub cRefreshesPerFrame: u32,
    pub eSampling: DWM_SOURCE_FRAME_SAMPLING,
}
impl ::core::marker::Copy for DWM_PRESENT_PARAMETERS {}
impl ::core::clone::Clone for DWM_PRESENT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DWM_SHOWCONTACT = u32;
pub const DWMSC_DOWN: DWM_SHOWCONTACT = 1u32;
pub const DWMSC_UP: DWM_SHOWCONTACT = 2u32;
pub const DWMSC_DRAG: DWM_SHOWCONTACT = 4u32;
pub const DWMSC_HOLD: DWM_SHOWCONTACT = 8u32;
pub const DWMSC_PENBARREL: DWM_SHOWCONTACT = 16u32;
pub const DWMSC_NONE: DWM_SHOWCONTACT = 0u32;
pub const DWMSC_ALL: DWM_SHOWCONTACT = 4294967295u32;
pub const DWM_SIT_DISPLAYFRAME: u32 = 1u32;
pub type DWM_SOURCE_FRAME_SAMPLING = i32;
pub const DWM_SOURCE_FRAME_SAMPLING_POINT: DWM_SOURCE_FRAME_SAMPLING = 0i32;
pub const DWM_SOURCE_FRAME_SAMPLING_COVERAGE: DWM_SOURCE_FRAME_SAMPLING = 1i32;
pub const DWM_SOURCE_FRAME_SAMPLING_LAST: DWM_SOURCE_FRAME_SAMPLING = 2i32;
pub type DWM_TAB_WINDOW_REQUIREMENTS = u32;
pub const DWMTWR_NONE: DWM_TAB_WINDOW_REQUIREMENTS = 0u32;
pub const DWMTWR_IMPLEMENTED_BY_SYSTEM: DWM_TAB_WINDOW_REQUIREMENTS = 1u32;
pub const DWMTWR_WINDOW_RELATIONSHIP: DWM_TAB_WINDOW_REQUIREMENTS = 2u32;
pub const DWMTWR_WINDOW_STYLES: DWM_TAB_WINDOW_REQUIREMENTS = 4u32;
pub const DWMTWR_WINDOW_REGION: DWM_TAB_WINDOW_REQUIREMENTS = 8u32;
pub const DWMTWR_WINDOW_DWM_ATTRIBUTES: DWM_TAB_WINDOW_REQUIREMENTS = 16u32;
pub const DWMTWR_WINDOW_MARGINS: DWM_TAB_WINDOW_REQUIREMENTS = 32u32;
pub const DWMTWR_TABBING_ENABLED: DWM_TAB_WINDOW_REQUIREMENTS = 64u32;
pub const DWMTWR_USER_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = 128u32;
pub const DWMTWR_GROUP_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = 256u32;
pub const DWMTWR_APP_COMPAT: DWM_TAB_WINDOW_REQUIREMENTS = 512u32;
#[repr(C, packed(1))]
pub struct DWM_THUMBNAIL_PROPERTIES {
    pub dwFlags: u32,
    pub rcDestination: ::win32_foundation_sys::RECT,
    pub rcSource: ::win32_foundation_sys::RECT,
    pub opacity: u8,
    pub fVisible: ::win32_foundation_sys::BOOL,
    pub fSourceClientAreaOnly: ::win32_foundation_sys::BOOL,
}
impl ::core::marker::Copy for DWM_THUMBNAIL_PROPERTIES {}
impl ::core::clone::Clone for DWM_THUMBNAIL_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct DWM_TIMING_INFO {
    pub cbSize: u32,
    pub rateRefresh: UNSIGNED_RATIO,
    pub qpcRefreshPeriod: u64,
    pub rateCompose: UNSIGNED_RATIO,
    pub qpcVBlank: u64,
    pub cRefresh: u64,
    pub cDXRefresh: u32,
    pub qpcCompose: u64,
    pub cFrame: u64,
    pub cDXPresent: u32,
    pub cRefreshFrame: u64,
    pub cFrameSubmitted: u64,
    pub cDXPresentSubmitted: u32,
    pub cFrameConfirmed: u64,
    pub cDXPresentConfirmed: u32,
    pub cRefreshConfirmed: u64,
    pub cDXRefreshConfirmed: u32,
    pub cFramesLate: u64,
    pub cFramesOutstanding: u32,
    pub cFrameDisplayed: u64,
    pub qpcFrameDisplayed: u64,
    pub cRefreshFrameDisplayed: u64,
    pub cFrameComplete: u64,
    pub qpcFrameComplete: u64,
    pub cFramePending: u64,
    pub qpcFramePending: u64,
    pub cFramesDisplayed: u64,
    pub cFramesComplete: u64,
    pub cFramesPending: u64,
    pub cFramesAvailable: u64,
    pub cFramesDropped: u64,
    pub cFramesMissed: u64,
    pub cRefreshNextDisplayed: u64,
    pub cRefreshNextPresented: u64,
    pub cRefreshesDisplayed: u64,
    pub cRefreshesPresented: u64,
    pub cRefreshStarted: u64,
    pub cPixelsReceived: u64,
    pub cPixelsDrawn: u64,
    pub cBuffersEmpty: u64,
}
impl ::core::marker::Copy for DWM_TIMING_INFO {}
impl ::core::clone::Clone for DWM_TIMING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWM_TNP_OPACITY: u32 = 4u32;
pub const DWM_TNP_RECTDESTINATION: u32 = 1u32;
pub const DWM_TNP_RECTSOURCE: u32 = 2u32;
pub const DWM_TNP_SOURCECLIENTAREAONLY: u32 = 16u32;
pub const DWM_TNP_VISIBLE: u32 = 8u32;
pub type DWM_WINDOW_CORNER_PREFERENCE = i32;
pub const DWMWCP_DEFAULT: DWM_WINDOW_CORNER_PREFERENCE = 0i32;
pub const DWMWCP_DONOTROUND: DWM_WINDOW_CORNER_PREFERENCE = 1i32;
pub const DWMWCP_ROUND: DWM_WINDOW_CORNER_PREFERENCE = 2i32;
pub const DWMWCP_ROUNDSMALL: DWM_WINDOW_CORNER_PREFERENCE = 3i32;
pub type GESTURE_TYPE = i32;
pub const GT_PEN_TAP: GESTURE_TYPE = 0i32;
pub const GT_PEN_DOUBLETAP: GESTURE_TYPE = 1i32;
pub const GT_PEN_RIGHTTAP: GESTURE_TYPE = 2i32;
pub const GT_PEN_PRESSANDHOLD: GESTURE_TYPE = 3i32;
pub const GT_PEN_PRESSANDHOLDABORT: GESTURE_TYPE = 4i32;
pub const GT_TOUCH_TAP: GESTURE_TYPE = 5i32;
pub const GT_TOUCH_DOUBLETAP: GESTURE_TYPE = 6i32;
pub const GT_TOUCH_RIGHTTAP: GESTURE_TYPE = 7i32;
pub const GT_TOUCH_PRESSANDHOLD: GESTURE_TYPE = 8i32;
pub const GT_TOUCH_PRESSANDHOLDABORT: GESTURE_TYPE = 9i32;
pub const GT_TOUCH_PRESSANDTAP: GESTURE_TYPE = 10i32;
#[repr(C, packed(1))]
pub struct MilMatrix3x2D {
    pub S_11: f64,
    pub S_12: f64,
    pub S_21: f64,
    pub S_22: f64,
    pub DX: f64,
    pub DY: f64,
}
impl ::core::marker::Copy for MilMatrix3x2D {}
impl ::core::clone::Clone for MilMatrix3x2D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct UNSIGNED_RATIO {
    pub uiNumerator: u32,
    pub uiDenominator: u32,
}
impl ::core::marker::Copy for UNSIGNED_RATIO {}
impl ::core::clone::Clone for UNSIGNED_RATIO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const c_DwmMaxAdapters: u32 = 16u32;
pub const c_DwmMaxMonitors: u32 = 16u32;
pub const c_DwmMaxQueuedBuffers: u32 = 8u32;
