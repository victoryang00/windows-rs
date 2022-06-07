#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const DISPID_EVENT_ON_CONTEXT_DATA: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const DISPID_EVENT_ON_SEND_ERROR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const DISPID_EVENT_ON_STATE_CHANGED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const DISPID_EVENT_ON_TERMINATION: u32 = 6u32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DRendezvousSessionEvents {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for DRendezvousSessionEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1067556088, data2: 25796, data3: 20307, data4: [174, 96, 99, 91, 56, 6, 236, 166] };
}
#[repr(C)]
pub struct IRendezvousApplication {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetRendezvousSession: unsafe extern "system" fn(this: *mut *mut Self, prendezvoussession: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRendezvousApplication {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1330448139, data2: 41589, data3: 18939, data4: [177, 13, 142, 194, 99, 135, 181, 13] };
}
#[repr(C)]
pub struct IRendezvousSession {
    pub base__: ::windows_sys::core::IUnknown,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoteUser: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoteUser: usize,
    pub Flags: unsafe extern "system" fn(this: *mut *mut Self, pflags: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SendContextData: unsafe extern "system" fn(this: *mut *mut Self, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendContextData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Terminate: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT, bstrappdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Terminate: usize,
}
impl ::windows_sys::core::Interface for IRendezvousSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2611261917, data2: 35596, data3: 18615, data4: [158, 124, 47, 37, 133, 124, 141, 245] };
}
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub type RENDEZVOUS_SESSION_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSF_NONE: RENDEZVOUS_SESSION_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSF_INVITER: RENDEZVOUS_SESSION_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSF_INVITEE: RENDEZVOUS_SESSION_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSF_ORIGINAL_INVITER: RENDEZVOUS_SESSION_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSF_REMOTE_LEGACYSESSION: RENDEZVOUS_SESSION_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSF_REMOTE_WIN7SESSION: RENDEZVOUS_SESSION_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub type RENDEZVOUS_SESSION_STATE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_UNKNOWN: RENDEZVOUS_SESSION_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_READY: RENDEZVOUS_SESSION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_INVITATION: RENDEZVOUS_SESSION_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_ACCEPTED: RENDEZVOUS_SESSION_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_CONNECTED: RENDEZVOUS_SESSION_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_CANCELLED: RENDEZVOUS_SESSION_STATE = 5i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_DECLINED: RENDEZVOUS_SESSION_STATE = 6i32;
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`*"]
pub const RSS_TERMINATED: RENDEZVOUS_SESSION_STATE = 7i32;
pub const RendezvousApplication: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 192807322, data2: 46558, data3: 18426, data4: [137, 102, 144, 130, 248, 47, 177, 146] };
