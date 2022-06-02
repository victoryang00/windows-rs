pub const CLSID_AutoScrollBehavior: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 638741073, data2: 15472, data3: 19610, data4: [174, 194, 148, 136, 73, 238, 176, 147] };
pub const CLSID_DeferContactService: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3619060980, data2: 33979, data3: 17230, data4: [134, 174, 101, 146, 187, 201, 171, 217] };
pub const CLSID_DragDropConfigurationBehavior: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 162536254, data2: 47724, data3: 17741, data4: [130, 232, 149, 227, 82, 50, 159, 35] };
pub const CLSID_HorizontalIndicatorContent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3889270005, data2: 16071, data3: 17621, data4: [167, 107, 55, 112, 243, 207, 144, 61] };
pub const CLSID_VerticalIndicatorContent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2701877015, data2: 45024, data3: 19106, data4: [145, 233, 62, 112, 1, 210, 230, 180] };
pub const CLSID_VirtualViewportContent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 839295386, data2: 34544, data3: 19636, data4: [167, 243, 22, 227, 183, 226, 216, 82] };
pub const DCompManipulationCompositor: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2044634663, data2: 41098, data3: 17324, data4: [142, 245, 105, 0, 185, 41, 145, 38] };
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_STOP: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_FORWARD: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_REVERSE: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_CONFIGURATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_NONE: DIRECTMANIPULATION_CONFIGURATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_INTERACTION: DIRECTMANIPULATION_CONFIGURATION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_X: DIRECTMANIPULATION_CONFIGURATION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_Y: DIRECTMANIPULATION_CONFIGURATION = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING: DIRECTMANIPULATION_CONFIGURATION = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_INERTIA: DIRECTMANIPULATION_CONFIGURATION = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING_INERTIA: DIRECTMANIPULATION_CONFIGURATION = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_X: DIRECTMANIPULATION_CONFIGURATION = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_Y: DIRECTMANIPULATION_CONFIGURATION = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_VERTICAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HORIZONTAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_ONLY: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HOLD_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_DRAG_DROP_STATUS = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_READY: DIRECTMANIPULATION_DRAG_DROP_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_PRESELECT: DIRECTMANIPULATION_DRAG_DROP_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_SELECTING: DIRECTMANIPULATION_DRAG_DROP_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_DRAGGING: DIRECTMANIPULATION_DRAG_DROP_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CANCELLED: DIRECTMANIPULATION_DRAG_DROP_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_COMMITTED: DIRECTMANIPULATION_DRAG_DROP_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_GESTURE_CONFIGURATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_NONE: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_DEFAULT: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_VERTICAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_HORIZONTAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_HITTEST_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_ASYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_AUTO_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_HORIZONTALALIGNMENT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_NONE: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_LEFT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_CENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_RIGHT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_INPUT_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INPUT_MODE_AUTOMATIC: DIRECTMANIPULATION_INPUT_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INPUT_MODE_MANUAL: DIRECTMANIPULATION_INPUT_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_INTERACTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_BEGIN: DIRECTMANIPULATION_INTERACTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_MANIPULATION: DIRECTMANIPULATION_INTERACTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_TAP: DIRECTMANIPULATION_INTERACTION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_HOLD: DIRECTMANIPULATION_INTERACTION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_CROSS_SLIDE: DIRECTMANIPULATION_INTERACTION_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_INTERACTION_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_END: DIRECTMANIPULATION_INTERACTION_TYPE = 100i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_KEYBOARDFOCUS: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_MOTION_TYPES = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_NONE: DIRECTMANIPULATION_MOTION_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_TRANSLATEX: DIRECTMANIPULATION_MOTION_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_TRANSLATEY: DIRECTMANIPULATION_MOTION_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_ZOOM: DIRECTMANIPULATION_MOTION_TYPES = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_CENTERX: DIRECTMANIPULATION_MOTION_TYPES = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_CENTERY: DIRECTMANIPULATION_MOTION_TYPES = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_ALL: DIRECTMANIPULATION_MOTION_TYPES = 55i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOUSEFOCUS: u32 = 4294967293u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_SNAPPOINT_COORDINATE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_BOUNDARY: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_ORIGIN: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_MIRRORED: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_SNAPPOINT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY: DIRECTMANIPULATION_SNAPPOINT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL: DIRECTMANIPULATION_SNAPPOINT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_STATUS = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_BUILDING: DIRECTMANIPULATION_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_ENABLED: DIRECTMANIPULATION_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DISABLED: DIRECTMANIPULATION_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_RUNNING: DIRECTMANIPULATION_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INERTIA: DIRECTMANIPULATION_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_READY: DIRECTMANIPULATION_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SUSPENDED: DIRECTMANIPULATION_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_VERTICALALIGNMENT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_NONE: DIRECTMANIPULATION_VERTICALALIGNMENT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_TOP: DIRECTMANIPULATION_VERTICALALIGNMENT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_CENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_BOTTOM: DIRECTMANIPULATION_VERTICALALIGNMENT = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_VIEWPORT_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DEFAULT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_AUTODISABLE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_MANUALUPDATE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_INPUT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_EXPLICITHITTEST: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DISABLEPIXELSNAPPING: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 16i32;
pub const DirectManipulationManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1424101814, data2: 13904, data3: 20341, data4: [131, 52, 250, 53, 149, 152, 225, 197] };
pub const DirectManipulationPrimaryContent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3399493217, data2: 54686, data3: 16839, data4: [131, 147, 59, 163, 186, 203, 107, 87] };
pub const DirectManipulationSharedManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2574856838, data2: 30668, data3: 19287, data4: [150, 219, 59, 53, 79, 111, 159, 181] };
pub const DirectManipulationUpdateManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2680274901, data2: 6197, data3: 17434, data4: [179, 177, 182, 204, 116, 183, 39, 208] };
pub const DirectManipulationViewport: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 887230902, data2: 13904, data3: 20341, data4: [131, 52, 250, 53, 149, 152, 225, 197] };
#[repr(C)]
pub struct IDirectManipulationAutoScrollBehavior {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetConfiguration: unsafe extern "system" fn(this: *mut *mut Self, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationCompositor {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddContent: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveContent: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetUpdateManager: unsafe extern "system" fn(this: *mut *mut Self, updatemanager: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationCompositor2 {
    pub base__: IDirectManipulationCompositor,
    pub AddContentWithCrossProcessChaining: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationContent {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetContentRect: unsafe extern "system" fn(this: *mut *mut Self, contentsize: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetContentRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContentRect: unsafe extern "system" fn(this: *mut *mut Self, contentsize: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContentRect: usize,
    pub GetViewport: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_sys::core::HRESULT,
    pub GetOutputTransform: unsafe extern "system" fn(this: *mut *mut Self, matrix: *mut f32, pointcount: u32) -> ::windows_sys::core::HRESULT,
    pub GetContentTransform: unsafe extern "system" fn(this: *mut *mut Self, matrix: *mut f32, pointcount: u32) -> ::windows_sys::core::HRESULT,
    pub SyncContentTransform: unsafe extern "system" fn(this: *mut *mut Self, matrix: *const f32, pointcount: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationDeferContactService {
    pub base__: ::windows_sys::core::IUnknown,
    pub DeferContact: unsafe extern "system" fn(this: *mut *mut Self, pointerid: u32, timeout: u32) -> ::windows_sys::core::HRESULT,
    pub CancelContact: unsafe extern "system" fn(this: *mut *mut Self, pointerid: u32) -> ::windows_sys::core::HRESULT,
    pub CancelDeferral: unsafe extern "system" fn(this: *mut *mut Self, pointerid: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationDragDropBehavior {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetConfiguration: unsafe extern "system" fn(this: *mut *mut Self, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationDragDropEventHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnDragDropStatusChange: unsafe extern "system" fn(this: *mut *mut Self, viewport: *mut ::core::ffi::c_void, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationFrameInfoProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetNextFrameInfo: unsafe extern "system" fn(this: *mut *mut Self, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationInteractionEventHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnInteraction: unsafe extern "system" fn(this: *mut *mut Self, viewport: *mut ::core::ffi::c_void, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationManager {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Activate: unsafe extern "system" fn(this: *mut *mut Self, window: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Activate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Deactivate: unsafe extern "system" fn(this: *mut *mut Self, window: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Deactivate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterHitTestTarget: unsafe extern "system" fn(this: *mut *mut Self, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterHitTestTarget: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub ProcessInput: unsafe extern "system" fn(this: *mut *mut Self, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    ProcessInput: usize,
    pub GetUpdateManager: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateViewport: unsafe extern "system" fn(this: *mut *mut Self, frameinfo: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateViewport: usize,
    pub CreateContent: unsafe extern "system" fn(this: *mut *mut Self, frameinfo: *mut ::core::ffi::c_void, clsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationManager2 {
    pub base__: IDirectManipulationManager,
    pub CreateBehavior: unsafe extern "system" fn(this: *mut *mut Self, clsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationManager3 {
    pub base__: IDirectManipulationManager2,
    pub GetService: unsafe extern "system" fn(this: *mut *mut Self, clsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationPrimaryContent {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetSnapInterval: unsafe extern "system" fn(this: *mut *mut Self, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows_sys::core::HRESULT,
    pub SetSnapPoints: unsafe extern "system" fn(this: *mut *mut Self, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows_sys::core::HRESULT,
    pub SetSnapType: unsafe extern "system" fn(this: *mut *mut Self, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows_sys::core::HRESULT,
    pub SetSnapCoordinate: unsafe extern "system" fn(this: *mut *mut Self, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows_sys::core::HRESULT,
    pub SetZoomBoundaries: unsafe extern "system" fn(this: *mut *mut Self, zoomminimum: f32, zoommaximum: f32) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalAlignment: unsafe extern "system" fn(this: *mut *mut Self, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows_sys::core::HRESULT,
    pub SetVerticalAlignment: unsafe extern "system" fn(this: *mut *mut Self, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows_sys::core::HRESULT,
    pub GetInertiaEndTransform: unsafe extern "system" fn(this: *mut *mut Self, matrix: *mut f32, pointcount: u32) -> ::windows_sys::core::HRESULT,
    pub GetCenterPoint: unsafe extern "system" fn(this: *mut *mut Self, centerx: *mut f32, centery: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationUpdateHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationUpdateManager {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterWaitHandleCallback: unsafe extern "system" fn(this: *mut *mut Self, handle: super::super::Foundation::HANDLE, eventhandler: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterWaitHandleCallback: usize,
    pub UnregisterWaitHandleCallback: unsafe extern "system" fn(this: *mut *mut Self, cookie: u32) -> ::windows_sys::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, frameinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationViewport {
    pub base__: ::windows_sys::core::IUnknown,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetContact: unsafe extern "system" fn(this: *mut *mut Self, pointerid: u32) -> ::windows_sys::core::HRESULT,
    pub ReleaseContact: unsafe extern "system" fn(this: *mut *mut Self, pointerid: u32) -> ::windows_sys::core::HRESULT,
    pub ReleaseAllContacts: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows_sys::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetViewportRect: unsafe extern "system" fn(this: *mut *mut Self, viewport: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetViewportRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetViewportRect: unsafe extern "system" fn(this: *mut *mut Self, viewport: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetViewportRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ZoomToRect: unsafe extern "system" fn(this: *mut *mut Self, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ZoomToRect: usize,
    pub SetViewportTransform: unsafe extern "system" fn(this: *mut *mut Self, matrix: *const f32, pointcount: u32) -> ::windows_sys::core::HRESULT,
    pub SyncDisplayTransform: unsafe extern "system" fn(this: *mut *mut Self, matrix: *const f32, pointcount: u32) -> ::windows_sys::core::HRESULT,
    pub GetPrimaryContent: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddContent: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveContent: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetViewportOptions: unsafe extern "system" fn(this: *mut *mut Self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows_sys::core::HRESULT,
    pub AddConfiguration: unsafe extern "system" fn(this: *mut *mut Self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows_sys::core::HRESULT,
    pub RemoveConfiguration: unsafe extern "system" fn(this: *mut *mut Self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows_sys::core::HRESULT,
    pub ActivateConfiguration: unsafe extern "system" fn(this: *mut *mut Self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows_sys::core::HRESULT,
    pub SetManualGesture: unsafe extern "system" fn(this: *mut *mut Self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows_sys::core::HRESULT,
    pub SetChaining: unsafe extern "system" fn(this: *mut *mut Self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddEventHandler: unsafe extern "system" fn(this: *mut *mut Self, window: super::super::Foundation::HWND, eventhandler: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddEventHandler: usize,
    pub RemoveEventHandler: unsafe extern "system" fn(this: *mut *mut Self, cookie: u32) -> ::windows_sys::core::HRESULT,
    pub SetInputMode: unsafe extern "system" fn(this: *mut *mut Self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows_sys::core::HRESULT,
    pub SetUpdateMode: unsafe extern "system" fn(this: *mut *mut Self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Abandon: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationViewport2 {
    pub base__: IDirectManipulationViewport,
    pub AddBehavior: unsafe extern "system" fn(this: *mut *mut Self, behavior: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows_sys::core::HRESULT,
    pub RemoveBehavior: unsafe extern "system" fn(this: *mut *mut Self, cookie: u32) -> ::windows_sys::core::HRESULT,
    pub RemoveAllBehaviors: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirectManipulationViewportEventHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnViewportStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, viewport: *mut ::core::ffi::c_void, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows_sys::core::HRESULT,
    pub OnViewportUpdated: unsafe extern "system" fn(this: *mut *mut Self, viewport: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnContentUpdated: unsafe extern "system" fn(this: *mut *mut Self, viewport: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
