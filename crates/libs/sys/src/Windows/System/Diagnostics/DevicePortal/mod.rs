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
impl ::windows_sys::core::Interface for IDevicePortalConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 256147281, data2: 4504, data3: 19873, data4: [141, 84, 189, 239, 57, 62, 9, 182] };
}
#[repr(C)]
pub struct IDevicePortalConnectionClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DevicePortalConnectionClosedReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDevicePortalConnectionClosedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4244049464, data2: 28722, data3: 17036, data4: [159, 80, 148, 92, 21, 169, 240, 203] };
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
impl ::windows_sys::core::Interface for IDevicePortalConnectionRequestReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1692065861, data2: 28634, data3: 17497, data4: [158, 189, 236, 206, 34, 227, 133, 89] };
}
#[repr(C)]
pub struct IDevicePortalConnectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub GetForAppServiceConnection: unsafe extern "system" fn(this: *mut *mut Self, appserviceconnection: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    GetForAppServiceConnection: usize,
}
impl ::windows_sys::core::Interface for IDevicePortalConnectionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1270755815, data2: 59833, data3: 17989, data4: [143, 237, 165, 62, 234, 14, 219, 214] };
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
impl ::windows_sys::core::Interface for IDevicePortalWebSocketConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1734703392, data2: 54874, data3: 17136, data4: [174, 244, 120, 120, 8, 9, 139, 123] };
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
impl ::windows_sys::core::Interface for IDevicePortalWebSocketConnectionRequestReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2046675642, data2: 5980, data3: 18233, data4: [159, 116, 221, 167, 151, 195, 91, 63] };
}
