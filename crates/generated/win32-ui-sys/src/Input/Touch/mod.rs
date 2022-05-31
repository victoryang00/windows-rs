#[link(name = "windows")]
extern "system" {
    pub fn CloseGestureInfoHandle(hgestureinfo: HGESTUREINFO) -> ::win32_foundation_sys::BOOL;
    pub fn CloseTouchInputHandle(htouchinput: HTOUCHINPUT) -> ::win32_foundation_sys::BOOL;
    pub fn GetGestureConfig(hwnd: ::win32_foundation_sys::HWND, dwreserved: u32, dwflags: u32, pcids: *const u32, pgestureconfig: *mut GESTURECONFIG, cbsize: u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetGestureExtraArgs(hgestureinfo: HGESTUREINFO, cbextraargs: u32, pextraargs: *mut u8) -> ::win32_foundation_sys::BOOL;
    pub fn GetGestureInfo(hgestureinfo: HGESTUREINFO, pgestureinfo: *mut GESTUREINFO) -> ::win32_foundation_sys::BOOL;
    pub fn GetTouchInputInfo(htouchinput: HTOUCHINPUT, cinputs: u32, pinputs: *mut TOUCHINPUT, cbsize: i32) -> ::win32_foundation_sys::BOOL;
    pub fn IsTouchWindow(hwnd: ::win32_foundation_sys::HWND, pulflags: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn RegisterTouchWindow(hwnd: ::win32_foundation_sys::HWND, ulflags: REGISTER_TOUCH_WINDOW_FLAGS) -> ::win32_foundation_sys::BOOL;
    pub fn SetGestureConfig(hwnd: ::win32_foundation_sys::HWND, dwreserved: u32, cids: u32, pgestureconfig: *const GESTURECONFIG, cbsize: u32) -> ::win32_foundation_sys::BOOL;
    pub fn UnregisterTouchWindow(hwnd: ::win32_foundation_sys::HWND) -> ::win32_foundation_sys::BOOL;
}
#[repr(C)]
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
pub type GESTURECONFIG_ID = u32;
pub const GID_BEGIN: GESTURECONFIG_ID = 1u32;
pub const GID_END: GESTURECONFIG_ID = 2u32;
pub const GID_ZOOM: GESTURECONFIG_ID = 3u32;
pub const GID_PAN: GESTURECONFIG_ID = 4u32;
pub const GID_ROTATE: GESTURECONFIG_ID = 5u32;
pub const GID_TWOFINGERTAP: GESTURECONFIG_ID = 6u32;
pub const GID_PRESSANDTAP: GESTURECONFIG_ID = 7u32;
pub const GID_ROLLOVER: GESTURECONFIG_ID = 7u32;
#[repr(C)]
pub struct GESTUREINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwID: u32,
    pub hwndTarget: ::win32_foundation_sys::HWND,
    pub ptsLocation: ::win32_foundation_sys::POINTS,
    pub dwInstanceID: u32,
    pub dwSequenceID: u32,
    pub ullArguments: u64,
    pub cbExtraArgs: u32,
}
impl ::core::marker::Copy for GESTUREINFO {}
impl ::core::clone::Clone for GESTUREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GESTURENOTIFYSTRUCT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTarget: ::win32_foundation_sys::HWND,
    pub ptsLocation: ::win32_foundation_sys::POINTS,
    pub dwInstanceID: u32,
}
impl ::core::marker::Copy for GESTURENOTIFYSTRUCT {}
impl ::core::clone::Clone for GESTURENOTIFYSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HGESTUREINFO = isize;
pub type HTOUCHINPUT = isize;
pub type IInertiaProcessor = *mut ::core::ffi::c_void;
pub type IManipulationProcessor = *mut ::core::ffi::c_void;
pub const InertiaProcessor: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2880598151, data2: 19680, data3: 20056, data4: [160, 203, 226, 77, 249, 104, 20, 190] };
pub type MANIPULATION_PROCESSOR_MANIPULATIONS = i32;
pub const MANIPULATION_NONE: MANIPULATION_PROCESSOR_MANIPULATIONS = 0i32;
pub const MANIPULATION_TRANSLATE_X: MANIPULATION_PROCESSOR_MANIPULATIONS = 1i32;
pub const MANIPULATION_TRANSLATE_Y: MANIPULATION_PROCESSOR_MANIPULATIONS = 2i32;
pub const MANIPULATION_SCALE: MANIPULATION_PROCESSOR_MANIPULATIONS = 4i32;
pub const MANIPULATION_ROTATE: MANIPULATION_PROCESSOR_MANIPULATIONS = 8i32;
pub const MANIPULATION_ALL: MANIPULATION_PROCESSOR_MANIPULATIONS = 15i32;
pub const ManipulationProcessor: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1501384624, data2: 18429, data3: 19199, data4: [137, 185, 198, 207, 174, 140, 240, 142] };
pub type REGISTER_TOUCH_WINDOW_FLAGS = u32;
pub const TWF_FINETOUCH: REGISTER_TOUCH_WINDOW_FLAGS = 1u32;
pub const TWF_WANTPALM: REGISTER_TOUCH_WINDOW_FLAGS = 2u32;
pub type TOUCHEVENTF_FLAGS = u32;
pub const TOUCHEVENTF_MOVE: TOUCHEVENTF_FLAGS = 1u32;
pub const TOUCHEVENTF_DOWN: TOUCHEVENTF_FLAGS = 2u32;
pub const TOUCHEVENTF_UP: TOUCHEVENTF_FLAGS = 4u32;
pub const TOUCHEVENTF_INRANGE: TOUCHEVENTF_FLAGS = 8u32;
pub const TOUCHEVENTF_PRIMARY: TOUCHEVENTF_FLAGS = 16u32;
pub const TOUCHEVENTF_NOCOALESCE: TOUCHEVENTF_FLAGS = 32u32;
pub const TOUCHEVENTF_PEN: TOUCHEVENTF_FLAGS = 64u32;
pub const TOUCHEVENTF_PALM: TOUCHEVENTF_FLAGS = 128u32;
#[repr(C)]
pub struct TOUCHINPUT {
    pub x: i32,
    pub y: i32,
    pub hSource: ::win32_foundation_sys::HANDLE,
    pub dwID: u32,
    pub dwFlags: TOUCHEVENTF_FLAGS,
    pub dwMask: TOUCHINPUTMASKF_MASK,
    pub dwTime: u32,
    pub dwExtraInfo: usize,
    pub cxContact: u32,
    pub cyContact: u32,
}
impl ::core::marker::Copy for TOUCHINPUT {}
impl ::core::clone::Clone for TOUCHINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TOUCHINPUTMASKF_MASK = u32;
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: TOUCHINPUTMASKF_MASK = 1u32;
pub const TOUCHINPUTMASKF_EXTRAINFO: TOUCHINPUTMASKF_MASK = 2u32;
pub const TOUCHINPUTMASKF_CONTACTAREA: TOUCHINPUTMASKF_MASK = 4u32;
pub type _IManipulationEvents = *mut ::core::ffi::c_void;
