#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseGestureInfoHandle(hgestureinfo: HGESTUREINFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseTouchInputHandle(htouchinput: HTOUCHINPUT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGestureConfig(hwnd: super::super::super::Foundation::HWND, dwreserved: u32, dwflags: u32, pcids: *const u32, pgestureconfig: *mut GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGestureExtraArgs(hgestureinfo: HGESTUREINFO, cbextraargs: u32, pextraargs: *mut u8) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGestureInfo(hgestureinfo: HGESTUREINFO, pgestureinfo: *mut GESTUREINFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTouchInputInfo(htouchinput: HTOUCHINPUT, cinputs: u32, pinputs: *mut TOUCHINPUT, cbsize: i32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsTouchWindow(hwnd: super::super::super::Foundation::HWND, pulflags: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterTouchWindow(hwnd: super::super::super::Foundation::HWND, ulflags: REGISTER_TOUCH_WINDOW_FLAGS) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetGestureConfig(hwnd: super::super::super::Foundation::HWND, dwreserved: u32, cids: u32, pgestureconfig: *const GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterTouchWindow(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::BOOL;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub struct GESTURECONFIG {
    pub dwID: GESTURECONFIG_ID,
    pub dwWant: u32,
    pub dwBlock: u32,
}
impl ::core::marker::Copy for GESTURECONFIG {}
impl ::core::clone::Clone for GESTURECONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub type GESTURECONFIG_ID = u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_BEGIN: GESTURECONFIG_ID = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_END: GESTURECONFIG_ID = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_ZOOM: GESTURECONFIG_ID = 3u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_PAN: GESTURECONFIG_ID = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_ROTATE: GESTURECONFIG_ID = 5u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_TWOFINGERTAP: GESTURECONFIG_ID = 6u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_PRESSANDTAP: GESTURECONFIG_ID = 7u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_ROLLOVER: GESTURECONFIG_ID = 7u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GESTUREINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwID: u32,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptsLocation: super::super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
    pub dwSequenceID: u32,
    pub ullArguments: u64,
    pub cbExtraArgs: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GESTUREINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GESTUREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GESTURENOTIFYSTRUCT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptsLocation: super::super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GESTURENOTIFYSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GESTURENOTIFYSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HGESTUREINFO = isize;
pub type HTOUCHINPUT = isize;
#[repr(C)]
pub struct IInertiaProcessor {
    pub base__: ::windows_sys::core::IUnknown,
    pub InitialOriginX: unsafe extern "system" fn(this: *mut *mut Self, x: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInitialOriginX: unsafe extern "system" fn(this: *mut *mut Self, x: f32) -> ::windows_sys::core::HRESULT,
    pub InitialOriginY: unsafe extern "system" fn(this: *mut *mut Self, y: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInitialOriginY: unsafe extern "system" fn(this: *mut *mut Self, y: f32) -> ::windows_sys::core::HRESULT,
    pub InitialVelocityX: unsafe extern "system" fn(this: *mut *mut Self, x: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInitialVelocityX: unsafe extern "system" fn(this: *mut *mut Self, x: f32) -> ::windows_sys::core::HRESULT,
    pub InitialVelocityY: unsafe extern "system" fn(this: *mut *mut Self, y: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInitialVelocityY: unsafe extern "system" fn(this: *mut *mut Self, y: f32) -> ::windows_sys::core::HRESULT,
    pub InitialAngularVelocity: unsafe extern "system" fn(this: *mut *mut Self, velocity: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInitialAngularVelocity: unsafe extern "system" fn(this: *mut *mut Self, velocity: f32) -> ::windows_sys::core::HRESULT,
    pub InitialExpansionVelocity: unsafe extern "system" fn(this: *mut *mut Self, velocity: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInitialExpansionVelocity: unsafe extern "system" fn(this: *mut *mut Self, velocity: f32) -> ::windows_sys::core::HRESULT,
    pub InitialRadius: unsafe extern "system" fn(this: *mut *mut Self, radius: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetInitialRadius: unsafe extern "system" fn(this: *mut *mut Self, radius: f32) -> ::windows_sys::core::HRESULT,
    pub BoundaryLeft: unsafe extern "system" fn(this: *mut *mut Self, left: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetBoundaryLeft: unsafe extern "system" fn(this: *mut *mut Self, left: f32) -> ::windows_sys::core::HRESULT,
    pub BoundaryTop: unsafe extern "system" fn(this: *mut *mut Self, top: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetBoundaryTop: unsafe extern "system" fn(this: *mut *mut Self, top: f32) -> ::windows_sys::core::HRESULT,
    pub BoundaryRight: unsafe extern "system" fn(this: *mut *mut Self, right: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetBoundaryRight: unsafe extern "system" fn(this: *mut *mut Self, right: f32) -> ::windows_sys::core::HRESULT,
    pub BoundaryBottom: unsafe extern "system" fn(this: *mut *mut Self, bottom: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetBoundaryBottom: unsafe extern "system" fn(this: *mut *mut Self, bottom: f32) -> ::windows_sys::core::HRESULT,
    pub ElasticMarginLeft: unsafe extern "system" fn(this: *mut *mut Self, left: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetElasticMarginLeft: unsafe extern "system" fn(this: *mut *mut Self, left: f32) -> ::windows_sys::core::HRESULT,
    pub ElasticMarginTop: unsafe extern "system" fn(this: *mut *mut Self, top: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetElasticMarginTop: unsafe extern "system" fn(this: *mut *mut Self, top: f32) -> ::windows_sys::core::HRESULT,
    pub ElasticMarginRight: unsafe extern "system" fn(this: *mut *mut Self, right: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetElasticMarginRight: unsafe extern "system" fn(this: *mut *mut Self, right: f32) -> ::windows_sys::core::HRESULT,
    pub ElasticMarginBottom: unsafe extern "system" fn(this: *mut *mut Self, bottom: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetElasticMarginBottom: unsafe extern "system" fn(this: *mut *mut Self, bottom: f32) -> ::windows_sys::core::HRESULT,
    pub DesiredDisplacement: unsafe extern "system" fn(this: *mut *mut Self, displacement: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDesiredDisplacement: unsafe extern "system" fn(this: *mut *mut Self, displacement: f32) -> ::windows_sys::core::HRESULT,
    pub DesiredRotation: unsafe extern "system" fn(this: *mut *mut Self, rotation: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDesiredRotation: unsafe extern "system" fn(this: *mut *mut Self, rotation: f32) -> ::windows_sys::core::HRESULT,
    pub DesiredExpansion: unsafe extern "system" fn(this: *mut *mut Self, expansion: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDesiredExpansion: unsafe extern "system" fn(this: *mut *mut Self, expansion: f32) -> ::windows_sys::core::HRESULT,
    pub DesiredDeceleration: unsafe extern "system" fn(this: *mut *mut Self, deceleration: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(this: *mut *mut Self, deceleration: f32) -> ::windows_sys::core::HRESULT,
    pub DesiredAngularDeceleration: unsafe extern "system" fn(this: *mut *mut Self, deceleration: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDesiredAngularDeceleration: unsafe extern "system" fn(this: *mut *mut Self, deceleration: f32) -> ::windows_sys::core::HRESULT,
    pub DesiredExpansionDeceleration: unsafe extern "system" fn(this: *mut *mut Self, deceleration: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDesiredExpansionDeceleration: unsafe extern "system" fn(this: *mut *mut Self, deceleration: f32) -> ::windows_sys::core::HRESULT,
    pub InitialTimestamp: unsafe extern "system" fn(this: *mut *mut Self, timestamp: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetInitialTimestamp: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Process: unsafe extern "system" fn(this: *mut *mut Self, completed: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Process: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProcessTime: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u32, completed: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProcessTime: usize,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CompleteTime: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IManipulationProcessor {
    pub base__: ::windows_sys::core::IUnknown,
    pub SupportedManipulations: unsafe extern "system" fn(this: *mut *mut Self, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows_sys::core::HRESULT,
    pub SetSupportedManipulations: unsafe extern "system" fn(this: *mut *mut Self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows_sys::core::HRESULT,
    pub PivotPointX: unsafe extern "system" fn(this: *mut *mut Self, pivotpointx: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPivotPointX: unsafe extern "system" fn(this: *mut *mut Self, pivotpointx: f32) -> ::windows_sys::core::HRESULT,
    pub PivotPointY: unsafe extern "system" fn(this: *mut *mut Self, pivotpointy: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPivotPointY: unsafe extern "system" fn(this: *mut *mut Self, pivotpointy: f32) -> ::windows_sys::core::HRESULT,
    pub PivotRadius: unsafe extern "system" fn(this: *mut *mut Self, pivotradius: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPivotRadius: unsafe extern "system" fn(this: *mut *mut Self, pivotradius: f32) -> ::windows_sys::core::HRESULT,
    pub CompleteManipulation: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ProcessDown: unsafe extern "system" fn(this: *mut *mut Self, manipulatorid: u32, x: f32, y: f32) -> ::windows_sys::core::HRESULT,
    pub ProcessMove: unsafe extern "system" fn(this: *mut *mut Self, manipulatorid: u32, x: f32, y: f32) -> ::windows_sys::core::HRESULT,
    pub ProcessUp: unsafe extern "system" fn(this: *mut *mut Self, manipulatorid: u32, x: f32, y: f32) -> ::windows_sys::core::HRESULT,
    pub ProcessDownWithTime: unsafe extern "system" fn(this: *mut *mut Self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_sys::core::HRESULT,
    pub ProcessMoveWithTime: unsafe extern "system" fn(this: *mut *mut Self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_sys::core::HRESULT,
    pub ProcessUpWithTime: unsafe extern "system" fn(this: *mut *mut Self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_sys::core::HRESULT,
    pub GetVelocityX: unsafe extern "system" fn(this: *mut *mut Self, velocityx: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetVelocityY: unsafe extern "system" fn(this: *mut *mut Self, velocityy: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetExpansionVelocity: unsafe extern "system" fn(this: *mut *mut Self, expansionvelocity: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetAngularVelocity: unsafe extern "system" fn(this: *mut *mut Self, angularvelocity: *mut f32) -> ::windows_sys::core::HRESULT,
    pub MinimumScaleRotateRadius: unsafe extern "system" fn(this: *mut *mut Self, minradius: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMinimumScaleRotateRadius: unsafe extern "system" fn(this: *mut *mut Self, minradius: f32) -> ::windows_sys::core::HRESULT,
}
pub const InertiaProcessor: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2880598151, data2: 19680, data3: 20056, data4: [160, 203, 226, 77, 249, 104, 20, 190] };
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub type MANIPULATION_PROCESSOR_MANIPULATIONS = i32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_NONE: MANIPULATION_PROCESSOR_MANIPULATIONS = 0i32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_TRANSLATE_X: MANIPULATION_PROCESSOR_MANIPULATIONS = 1i32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_TRANSLATE_Y: MANIPULATION_PROCESSOR_MANIPULATIONS = 2i32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_SCALE: MANIPULATION_PROCESSOR_MANIPULATIONS = 4i32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_ROTATE: MANIPULATION_PROCESSOR_MANIPULATIONS = 8i32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_ALL: MANIPULATION_PROCESSOR_MANIPULATIONS = 15i32;
pub const ManipulationProcessor: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1501384624, data2: 18429, data3: 19199, data4: [137, 185, 198, 207, 174, 140, 240, 142] };
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub type REGISTER_TOUCH_WINDOW_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TWF_FINETOUCH: REGISTER_TOUCH_WINDOW_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TWF_WANTPALM: REGISTER_TOUCH_WINDOW_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub type TOUCHEVENTF_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_MOVE: TOUCHEVENTF_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_DOWN: TOUCHEVENTF_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_UP: TOUCHEVENTF_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_INRANGE: TOUCHEVENTF_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_PRIMARY: TOUCHEVENTF_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_NOCOALESCE: TOUCHEVENTF_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_PEN: TOUCHEVENTF_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_PALM: TOUCHEVENTF_FLAGS = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOUCHINPUT {
    pub x: i32,
    pub y: i32,
    pub hSource: super::super::super::Foundation::HANDLE,
    pub dwID: u32,
    pub dwFlags: TOUCHEVENTF_FLAGS,
    pub dwMask: TOUCHINPUTMASKF_MASK,
    pub dwTime: u32,
    pub dwExtraInfo: usize,
    pub cxContact: u32,
    pub cyContact: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOUCHINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOUCHINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub type TOUCHINPUTMASKF_MASK = u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: TOUCHINPUTMASKF_MASK = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHINPUTMASKF_EXTRAINFO: TOUCHINPUTMASKF_MASK = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHINPUTMASKF_CONTACTAREA: TOUCHINPUTMASKF_MASK = 4u32;
#[repr(C)]
pub struct _IManipulationEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub ManipulationStarted: unsafe extern "system" fn(this: *mut *mut Self, x: f32, y: f32) -> ::windows_sys::core::HRESULT,
    pub ManipulationDelta: unsafe extern "system" fn(this: *mut *mut Self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows_sys::core::HRESULT,
    pub ManipulationCompleted: unsafe extern "system" fn(this: *mut *mut Self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows_sys::core::HRESULT,
}
