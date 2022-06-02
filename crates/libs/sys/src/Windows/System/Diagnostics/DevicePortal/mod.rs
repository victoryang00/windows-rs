pub type DevicePortalConnection = *mut ::core::ffi::c_void;
pub type DevicePortalConnectionClosedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_Diagnostics_DevicePortal\"`*"]
#[repr(transparent)]
pub struct DevicePortalConnectionClosedReason(pub i32);
impl DevicePortalConnectionClosedReason {
    pub const Unknown: Self = Self(0i32);
    pub const ResourceLimitsExceeded: Self = Self(1i32);
    pub const ProtocolError: Self = Self(2i32);
    pub const NotAuthorized: Self = Self(3i32);
    pub const UserNotPresent: Self = Self(4i32);
    pub const ServiceTerminated: Self = Self(5i32);
}
impl ::core::marker::Copy for DevicePortalConnectionClosedReason {}
impl ::core::clone::Clone for DevicePortalConnectionClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DevicePortalConnectionRequestReceivedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDevicePortalConnection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RequestReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestReceived: usize,
}
#[repr(C)]
pub struct IDevicePortalConnectionClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DevicePortalConnectionClosedReason) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDevicePortalConnectionRequestReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Web_Http")]
    pub RequestMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    RequestMessage: usize,
    #[cfg(feature = "Web_Http")]
    pub ResponseMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    ResponseMessage: usize,
}
#[repr(C)]
pub struct IDevicePortalConnectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub GetForAppServiceConnection: unsafe extern "system" fn(this: *mut *mut Self, appserviceconnection: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    GetForAppServiceConnection: usize,
}
#[repr(C)]
pub struct IDevicePortalWebSocketConnection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest2: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest2: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest3: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: ::windows_sys::core::HSTRING, outboundbuffersizeinbytes: u32, maxmessagesize: u32, receivemode: super::super::super::Networking::Sockets::MessageWebSocketReceiveMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest3: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerStreamWebSocketForRequest: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerStreamWebSocketForRequest: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerStreamWebSocketForRequest2: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, protocol: ::windows_sys::core::HSTRING, outboundbuffersizeinbytes: u32, nodelay: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerStreamWebSocketForRequest2: usize,
}
#[repr(C)]
pub struct IDevicePortalWebSocketConnectionRequestReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsWebSocketUpgradeRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub WebSocketProtocolsRequested: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebSocketProtocolsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
