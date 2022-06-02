#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type ATTENDEE_DISCONNECT_REASON = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_MIN: ATTENDEE_DISCONNECT_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_APP: ATTENDEE_DISCONNECT_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_ERR: ATTENDEE_DISCONNECT_REASON = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_CLI: ATTENDEE_DISCONNECT_REASON = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_MAX: ATTENDEE_DISCONNECT_REASON = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type CHANNEL_ACCESS_ENUM = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_ACCESS_ENUM_NONE: CHANNEL_ACCESS_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_ACCESS_ENUM_SENDRECEIVE: CHANNEL_ACCESS_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type CHANNEL_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_FLAGS_LEGACY: CHANNEL_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_FLAGS_UNCOMPRESSED: CHANNEL_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_FLAGS_DYNAMIC: CHANNEL_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type CHANNEL_PRIORITY = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_PRIORITY_LO: CHANNEL_PRIORITY = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_PRIORITY_MED: CHANNEL_PRIORITY = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_PRIORITY_HI: CHANNEL_PRIORITY = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type CTRL_LEVEL = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_MIN: CTRL_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_INVALID: CTRL_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_NONE: CTRL_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_VIEW: CTRL_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_INTERACTIVE: CTRL_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_REQCTRL_VIEW: CTRL_LEVEL = 4i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_REQCTRL_INTERACTIVE: CTRL_LEVEL = 5i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_MAX: CTRL_LEVEL = 5i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPAPI_EVENT_ON_BOUNDING_RECT_CHANGED: u32 = 340u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPFILTER_UPDATE: u32 = 322u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_CLOSE: u32 = 317u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_OPEN: u32 = 316u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_UPDATE: u32 = 318u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_CONNECTED: u32 = 301u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_DISCONNECTED: u32 = 302u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_UPDATE: u32 = 303u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_REQUEST: u32 = 309u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_RESPONSE: u32 = 338u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ERROR: u32 = 304u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_FOCUSRELEASED: u32 = 324u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_PAUSED: u32 = 310u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_RESUMED: u32 = 311u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_DESKTOP_SETTINGS_CHANGED: u32 = 325u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_RECT_CHANGED: u32 = 323u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_CLOSED: u32 = 634u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_DATARECEIVED: u32 = 633u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_SENDCOMPLETED: u32 = 632u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_AUTHENTICATED: u32 = 307u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTED: u32 = 305u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTFAILED: u32 = 308u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_DISCONNECTED: u32 = 306u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_DATARECEIVED: u32 = 314u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_JOIN: u32 = 312u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_LEAVE: u32 = 313u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_SENDCOMPLETED: u32 = 315u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_CLOSE: u32 = 320u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_OPEN: u32 = 319u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_UPDATE: u32 = 321u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_BUTTON_RECEIVED: u32 = 700u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_MOVE_RECEIVED: u32 = 701u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_WHEEL_RECEIVED: u32 = 702u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_ADD_TOUCH_INPUT: u32 = 125u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_BEGIN_TOUCH_FRAME: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_CLOSE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_CONNECTTOCLIENT: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_CONNECTUSINGTRANSPORTSTREAM: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_CREATE_INVITATION: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_END_TOUCH_FRAME: u32 = 126u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_GETFRAMEBUFFERBITS: u32 = 149u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_GETSHAREDRECT: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_OPEN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_PAUSE: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_REQUEST_COLOR_DEPTH_CHANGE: u32 = 115u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_REQUEST_CONTROL: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_RESUME: u32 = 113u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SENDCONTROLLEVELCHANGERESPONSE: u32 = 148u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_KEYBOARD_EVENT: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_BUTTON_EVENT: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_MOVE_EVENT: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_WHEEL_EVENT: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_SYNC_EVENT: u32 = 123u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SETSHAREDRECT: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SET_RENDERING_SURFACE: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SHOW_WINDOW: u32 = 114u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STARTREVCONNECTLISTENER: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMCLOSE: u32 = 426u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMOPEN: u32 = 425u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMREADDATA: u32 = 424u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMSENDDATA: u32 = 423u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAM_ALLOCBUFFER: u32 = 421u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAM_FREEBUFFER: u32 = 422u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_TERMINATE_CONNECTION: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIEWERCONNECT: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIEWERDISCONNECT: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_CREATE: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SEND_DATA: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SET_ACCESS: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPFILTERENABLED: u32 = 219u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPFILTER_ENABLED: u32 = 218u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPFLAGS: u32 = 223u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPLICATION: u32 = 211u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPLICATION_FILTER: u32 = 215u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPLICATION_LIST: u32 = 217u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPNAME: u32 = 214u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_ATTENDEELIMIT: u32 = 235u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_ATTENDEES: u32 = 203u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_ATTENDEE_FLAGS: u32 = 230u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CHANNELMANAGER: u32 = 206u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CODE: u32 = 241u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CONINFO: u32 = 231u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CONNECTION_STRING: u32 = 232u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_COUNT: u32 = 244u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CTRL_LEVEL: u32 = 242u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_DBG_CLX_CMDLINE: u32 = 222u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_DISCONNECTED_STRING: u32 = 237u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_DISPIDVALUE: u32 = 200u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER: u32 = 254u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_BPP: u32 = 253u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_HEIGHT: u32 = 251u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_WIDTH: u32 = 252u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_GROUP_NAME: u32 = 233u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_ID: u32 = 201u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_INVITATION: u32 = 205u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_INVITATIONITEM: u32 = 221u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_INVITATIONS: u32 = 204u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_LOCAL_IP: u32 = 227u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_LOCAL_PORT: u32 = 226u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_PASSWORD: u32 = 234u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_PEER_IP: u32 = 229u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_PEER_PORT: u32 = 228u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_PROTOCOL_TYPE: u32 = 225u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_REASON: u32 = 240u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_REMOTENAME: u32 = 243u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_REVOKED: u32 = 236u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_SESSION_COLORDEPTH: u32 = 239u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_SESSION_PROPERTIES: u32 = 202u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_SHARED: u32 = 220u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_CONTEXT: u32 = 560u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_FLAGS: u32 = 561u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADOFFSET: u32 = 559u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADSIZE: u32 = 558u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORAGE: u32 = 555u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORESIZE: u32 = 562u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_USESMARTSIZING: u32 = 238u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETFLAGS: u32 = 208u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETNAME: u32 = 207u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETPRIORITY: u32 = 209u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOWID: u32 = 210u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOWNAME: u32 = 213u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOWSHARED: u32 = 212u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOW_LIST: u32 = 216u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WNDFLAGS: u32 = 224u32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIApplication {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Windows: unsafe extern "system" fn(this: *mut *mut Self, pwindowlist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Windows: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pretval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Shared: unsafe extern "system" fn(this: *mut *mut Self, pretval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShared: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pretval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Flags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIApplicationFilter {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Applications: unsafe extern "system" fn(this: *mut *mut Self, papplications: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Applications: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Windows: unsafe extern "system" fn(this: *mut *mut Self, pwindows: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Windows: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, pretval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIApplicationList {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, item: i32, papplication: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIAttendee {
    pub base__: super::Com::IDispatch,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoteName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoteName: usize,
    pub ControlLevel: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut CTRL_LEVEL) -> ::windows_sys::core::HRESULT,
    pub SetControlLevel: unsafe extern "system" fn(this: *mut *mut Self, pnewval: CTRL_LEVEL) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Invitation: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invitation: usize,
    pub TerminateConnection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Flags: unsafe extern "system" fn(this: *mut *mut Self, plflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ConnectivityInfo: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIAttendeeDisconnectInfo {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Attendee: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Attendee: usize,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, preason: *mut ATTENDEE_DISCONNECT_REASON) -> ::windows_sys::core::HRESULT,
    pub Code: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIAttendeeManager {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, id: i32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[repr(C)]
pub struct IRDPSRAPIAudioStream {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pnperiodinhundrednsintervals: *mut i64) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(this: *mut *mut Self, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows_sys::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRDPSRAPIClipboardUseEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub OnPasteFromClipboard: unsafe extern "system" fn(this: *mut *mut Self, clipboardformat: u32, pattendee: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnPasteFromClipboard: usize,
}
#[repr(C)]
pub struct IRDPSRAPIDebug {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCLXCmdLine: unsafe extern "system" fn(this: *mut *mut Self, clxcmdline: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCLXCmdLine: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CLXCmdLine: unsafe extern "system" fn(this: *mut *mut Self, pclxcmdline: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CLXCmdLine: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIFrameBuffer {
    pub base__: super::Com::IDispatch,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, plwidth: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, plheight: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Bpp: unsafe extern "system" fn(this: *mut *mut Self, plbpp: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFrameBufferBits: unsafe extern "system" fn(this: *mut *mut Self, x: i32, y: i32, width: i32, heigth: i32, ppbits: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFrameBufferBits: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIInvitation {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectionString: unsafe extern "system" fn(this: *mut *mut Self, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectionString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GroupName: unsafe extern "system" fn(this: *mut *mut Self, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GroupName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Password: unsafe extern "system" fn(this: *mut *mut Self, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Password: usize,
    pub AttendeeLimit: unsafe extern "system" fn(this: *mut *mut Self, pretval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAttendeeLimit: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub Revoked: unsafe extern "system" fn(this: *mut *mut Self, pretval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetRevoked: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIInvitationManager {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppinvitation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pretval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateInvitation: unsafe extern "system" fn(this: *mut *mut Self, bstrauthstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attendeelimit: i32, ppinvitation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateInvitation: usize,
}
#[repr(C)]
pub struct IRDPSRAPIPerfCounterLogger {
    pub base__: ::windows_sys::core::IUnknown,
    pub LogValue: unsafe extern "system" fn(this: *mut *mut Self, lvalue: i64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRDPSRAPIPerfCounterLoggingManager {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateLogger: unsafe extern "system" fn(this: *mut *mut Self, bstrcountername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplogger: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateLogger: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPISessionProperties {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Property: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Property: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub put_Property: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    put_Property: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPISharingSession {
    pub base__: super::Com::IDispatch,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetColorDepth: unsafe extern "system" fn(this: *mut *mut Self, colordepth: i32) -> ::windows_sys::core::HRESULT,
    pub ColorDepth: unsafe extern "system" fn(this: *mut *mut Self, pcolordepth: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Attendees: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Attendees: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Invitations: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invitations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationFilter: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationFilter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub VirtualChannelManager: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    VirtualChannelManager: usize,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectToClient: unsafe extern "system" fn(this: *mut *mut Self, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectToClient: usize,
    pub SetDesktopSharedRect: unsafe extern "system" fn(this: *mut *mut Self, left: i32, top: i32, right: i32, bottom: i32) -> ::windows_sys::core::HRESULT,
    pub GetDesktopSharedRect: unsafe extern "system" fn(this: *mut *mut Self, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPISharingSession2 {
    pub base__: IRDPSRAPISharingSession,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectUsingTransportStream: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void, bstrgroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrauthenticatedattendeename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectUsingTransportStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub FrameBuffer: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FrameBuffer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SendControlLevelChangeResponse: unsafe extern "system" fn(this: *mut *mut Self, pattendee: *mut ::core::ffi::c_void, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SendControlLevelChangeResponse: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPITcpConnectionInfo {
    pub base__: super::Com::IDispatch,
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, plprotocol: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(this: *mut *mut Self, plport: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LocalIP: unsafe extern "system" fn(this: *mut *mut Self, pbsrlocalip: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LocalIP: usize,
    pub PeerPort: unsafe extern "system" fn(this: *mut *mut Self, plport: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PeerIP: unsafe extern "system" fn(this: *mut *mut Self, pbstrip: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PeerIP: usize,
}
#[repr(C)]
pub struct IRDPSRAPITransportStream {
    pub base__: ::windows_sys::core::IUnknown,
    pub AllocBuffer: unsafe extern "system" fn(this: *mut *mut Self, maxpayload: i32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut *mut Self, pbuffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WriteBuffer: unsafe extern "system" fn(this: *mut *mut Self, pbuffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReadBuffer: unsafe extern "system" fn(this: *mut *mut Self, pbuffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, pcallbacks: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRDPSRAPITransportStreamBuffer {
    pub base__: ::windows_sys::core::IUnknown,
    pub Storage: unsafe extern "system" fn(this: *mut *mut Self, ppbstorage: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub StorageSize: unsafe extern "system" fn(this: *mut *mut Self, plmaxstore: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PayloadSize: unsafe extern "system" fn(this: *mut *mut Self, plretval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPayloadSize: unsafe extern "system" fn(this: *mut *mut Self, lval: i32) -> ::windows_sys::core::HRESULT,
    pub PayloadOffset: unsafe extern "system" fn(this: *mut *mut Self, plretval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPayloadOffset: unsafe extern "system" fn(this: *mut *mut Self, lretval: i32) -> ::windows_sys::core::HRESULT,
    pub Flags: unsafe extern "system" fn(this: *mut *mut Self, plflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32) -> ::windows_sys::core::HRESULT,
    pub Context: unsafe extern "system" fn(this: *mut *mut Self, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContext: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRDPSRAPITransportStreamEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnWriteCompleted: unsafe extern "system" fn(this: *mut *mut Self, pbuffer: *mut ::core::ffi::c_void),
    pub OnReadCompleted: unsafe extern "system" fn(this: *mut *mut Self, pbuffer: *mut ::core::ffi::c_void),
    pub OnStreamClosed: unsafe extern "system" fn(this: *mut *mut Self, hrreason: ::windows_sys::core::HRESULT),
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIViewer {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Connect: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Attendees: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Attendees: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Invitations: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invitations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationFilter: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationFilter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub VirtualChannelManager: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    VirtualChannelManager: usize,
    pub SetSmartSizing: unsafe extern "system" fn(this: *mut *mut Self, vbsmartsizing: i16) -> ::windows_sys::core::HRESULT,
    pub SmartSizing: unsafe extern "system" fn(this: *mut *mut Self, pvbsmartsizing: *mut i16) -> ::windows_sys::core::HRESULT,
    pub RequestControl: unsafe extern "system" fn(this: *mut *mut Self, ctrllevel: CTRL_LEVEL) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisconnectedText: unsafe extern "system" fn(this: *mut *mut Self, bstrdisconnectedtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisconnectedText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisconnectedText: unsafe extern "system" fn(this: *mut *mut Self, pbstrdisconnectedtext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisconnectedText: usize,
    pub RequestColorDepthChange: unsafe extern "system" fn(this: *mut *mut Self, bpp: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartReverseConnectListener: unsafe extern "system" fn(this: *mut *mut Self, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrreverseconnectstring: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartReverseConnectListener: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIVirtualChannel {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub SendData: unsafe extern "system" fn(this: *mut *mut Self, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lattendeeid: i32, channelsendflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendData: usize,
    pub SetAccess: unsafe extern "system" fn(this: *mut *mut Self, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Flags: unsafe extern "system" fn(this: *mut *mut Self, plflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, ppriority: *mut CHANNEL_PRIORITY) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIVirtualChannelManager {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pchannel: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateVirtualChannel: unsafe extern "system" fn(this: *mut *mut Self, bstrchannelname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, priority: CHANNEL_PRIORITY, channelflags: u32, ppchannel: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateVirtualChannel: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIWindow {
    pub base__: super::Com::IDispatch,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pretval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Application: unsafe extern "system" fn(this: *mut *mut Self, papplication: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Application: usize,
    pub Shared: unsafe extern "system" fn(this: *mut *mut Self, pretval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShared: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pretval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Show: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Flags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIWindowList {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, item: i32, pwindow: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[repr(C)]
pub struct IRDPViewerInputSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub SendMouseButtonEvent: unsafe extern "system" fn(this: *mut *mut Self, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: i16, xpos: u32, ypos: u32) -> ::windows_sys::core::HRESULT,
    pub SendMouseMoveEvent: unsafe extern "system" fn(this: *mut *mut Self, xpos: u32, ypos: u32) -> ::windows_sys::core::HRESULT,
    pub SendMouseWheelEvent: unsafe extern "system" fn(this: *mut *mut Self, wheelrotation: u16) -> ::windows_sys::core::HRESULT,
    pub SendKeyboardEvent: unsafe extern "system" fn(this: *mut *mut Self, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: i16, vbrepeat: i16, vbextended: i16) -> ::windows_sys::core::HRESULT,
    pub SendSyncEvent: unsafe extern "system" fn(this: *mut *mut Self, syncflags: u32) -> ::windows_sys::core::HRESULT,
    pub BeginTouchFrame: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub AddTouchInput: unsafe extern "system" fn(this: *mut *mut Self, contactid: u32, event: u32, x: i32, y: i32) -> ::windows_sys::core::HRESULT,
    pub EndTouchFrame: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type RDPENCOMAPI_ATTENDEE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_FLAGS_LOCAL: RDPENCOMAPI_ATTENDEE_FLAGS = 1i32;
pub const RDPSRAPIApplication: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3239486596, data2: 19237, data3: 19359, data4: [138, 84, 185, 52, 176, 110, 87, 250] };
pub const RDPSRAPIApplicationFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3814379145, data2: 51176, data3: 17022, data4: [164, 249, 185, 218, 7, 40, 38, 189] };
pub const RDPSRAPIApplicationList: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2654062613, data2: 29747, data3: 18550, data4: [151, 251, 237, 89, 254, 43, 170, 34] };
pub const RDPSRAPIAttendee: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1962490805, data2: 30047, data3: 18574, data4: [138, 41, 35, 144, 16, 138, 239, 85] };
pub const RDPSRAPIAttendeeDisconnectInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3028120144, data2: 23515, data3: 16477, data4: [180, 135, 202, 173, 156, 86, 244, 248] };
pub const RDPSRAPIAttendeeManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3618716161, data2: 63444, data3: 17062, data4: [133, 149, 18, 252, 140, 36, 232, 81] };
pub const RDPSRAPIFrameBuffer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2767612876, data2: 21390, data3: 16641, data4: [149, 29, 48, 132, 122, 219, 81, 1] };
pub const RDPSRAPIInvitation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1226264006, data2: 1841, data3: 19294, data4: [142, 225, 131, 166, 61, 56, 104, 250] };
pub const RDPSRAPIInvitationManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1406781915, data2: 30123, data3: 17009, data4: [148, 138, 76, 78, 179, 106, 143, 43] };
pub const RDPSRAPISessionProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3715470591, data2: 59946, data3: 19462, data4: [143, 223, 19, 45, 228, 139, 101, 16] };
pub const RDPSRAPITcpConnectionInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3192511295, data2: 60342, data3: 17016, data4: [140, 224, 213, 69, 88, 51, 234, 238] };
pub const RDPSRAPIWindow: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 63915739, data2: 52805, data3: 19766, data4: [134, 237, 237, 40, 183, 67, 152, 191] };
pub const RDPSRAPIWindowList: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2619466424, data2: 24020, data3: 17100, data4: [129, 186, 28, 9, 152, 82, 230, 250] };
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type RDPSRAPI_APP_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const APP_FLAG_PRIVILEGED: RDPSRAPI_APP_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type RDPSRAPI_KBD_CODE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_CODE_SCANCODE: RDPSRAPI_KBD_CODE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_CODE_UNICODE: RDPSRAPI_KBD_CODE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type RDPSRAPI_KBD_SYNC_FLAG = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_SYNC_FLAG_SCROLL_LOCK: RDPSRAPI_KBD_SYNC_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_SYNC_FLAG_NUM_LOCK: RDPSRAPI_KBD_SYNC_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_SYNC_FLAG_CAPS_LOCK: RDPSRAPI_KBD_SYNC_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_SYNC_FLAG_KANA_LOCK: RDPSRAPI_KBD_SYNC_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type RDPSRAPI_MOUSE_BUTTON_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type RDPSRAPI_WND_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const WND_FLAG_PRIVILEGED: RDPSRAPI_WND_FLAGS = 1i32;
pub const RDPSession: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2608394470, data2: 15877, data3: 19035, data4: [178, 232, 231, 67, 168, 149, 107, 101] };
pub const RDPTransportStreamBuffer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2370444393, data2: 61823, data3: 17737, data4: [166, 153, 118, 28, 110, 107, 92, 10] };
pub const RDPTransportStreamEvents: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 837004064, data2: 21328, data3: 18495, data4: [157, 198, 103, 72, 102, 94, 253, 235] };
pub const RDPViewer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 851336914, data2: 23686, data3: 18447, data4: [169, 20, 15, 248, 136, 90, 27, 63] };
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _IRDPSessionEvents {
    pub base__: super::Com::IDispatch,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub type __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_MAX_CHANNEL_MESSAGE_SIZE: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = 1024i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_MAX_CHANNEL_NAME_LEN: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = 8i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_MAX_LEGACY_CHANNEL_MESSAGE_SIZE: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = 409600i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_ATTENDEE_ID_EVERYONE: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = -1i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_ATTENDEE_ID_HOST: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = 0i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_CONN_INTERVAL: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = 50i32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_ATTENDEE_ID_DEFAULT: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub struct __ReferenceRemainingTypes__ {
    pub __ctrlLevel__: CTRL_LEVEL,
    pub __attendeeDisconnectReason__: ATTENDEE_DISCONNECT_REASON,
    pub __channelPriority__: CHANNEL_PRIORITY,
    pub __channelFlags__: CHANNEL_FLAGS,
    pub __channelAccessEnum__: CHANNEL_ACCESS_ENUM,
    pub __rdpencomapiAttendeeFlags__: RDPENCOMAPI_ATTENDEE_FLAGS,
    pub __rdpsrapiWndFlags__: RDPSRAPI_WND_FLAGS,
    pub __rdpsrapiAppFlags__: RDPSRAPI_APP_FLAGS,
}
impl ::core::marker::Copy for __ReferenceRemainingTypes__ {}
impl ::core::clone::Clone for __ReferenceRemainingTypes__ {
    fn clone(&self) -> Self {
        *self
    }
}
