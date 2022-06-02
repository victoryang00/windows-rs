#[repr(C)]
pub struct IWiFiDirectService {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub RemoteServiceInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RemoteServiceInfo: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedConfigurationMethods: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedConfigurationMethods: usize,
    pub PreferGroupOwnerMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPreferGroupOwnerMode: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SessionInfo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSessionInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSessionInfo: usize,
    pub ServiceError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectServiceError) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SessionDeferred: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionDeferred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionDeferred: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionDeferred: usize,
    #[cfg(feature = "Foundation")]
    pub GetProvisioningInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, selectedconfigurationmethod: WiFiDirectServiceConfigurationMethod, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetProvisioningInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectAsyncWithPin: unsafe extern "system" fn(this: *mut *mut Self, pin: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsyncWithPin: usize,
}
#[repr(C)]
pub struct IWiFiDirectServiceAdvertiser {
    pub base__: ::windows_sys::core::IInspectable,
    pub ServiceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ServiceNamePrefixes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServiceNamePrefixes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ServiceInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ServiceInfo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetServiceInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetServiceInfo: usize,
    pub AutoAcceptSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoAcceptSession: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PreferGroupOwnerMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPreferGroupOwnerMode: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PreferredConfigurationMethods: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PreferredConfigurationMethods: usize,
    pub ServiceStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectServiceStatus) -> ::windows_sys::core::HRESULT,
    pub SetServiceStatus: unsafe extern "system" fn(this: *mut *mut Self, value: WiFiDirectServiceStatus) -> ::windows_sys::core::HRESULT,
    pub CustomServiceStatusCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCustomServiceStatusCode: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub DeferredSessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DeferredSessionInfo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetDeferredSessionInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetDeferredSessionInfo: usize,
    pub AdvertisementStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectServiceAdvertisementStatus) -> ::windows_sys::core::HRESULT,
    pub ServiceError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectServiceError) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SessionRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub AutoAcceptSessionConnected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoAcceptSessionConnected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAutoAcceptSessionConnected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAutoAcceptSessionConnected: usize,
    #[cfg(feature = "Foundation")]
    pub AdvertisementStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AdvertisementStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdvertisementStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdvertisementStatusChanged: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    ConnectAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub ConnectAsyncWithPin: unsafe extern "system" fn(this: *mut *mut Self, deviceinfo: *mut ::core::ffi::c_void, pin: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    ConnectAsyncWithPin: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiDirectServiceAdvertiserFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWiFiDirectServiceAdvertiser: unsafe extern "system" fn(this: *mut *mut Self, servicename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SessionInfo: usize,
}
#[repr(C)]
pub struct IWiFiDirectServiceProvisioningInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedConfigurationMethod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectServiceConfigurationMethod) -> ::windows_sys::core::HRESULT,
    pub IsGroupFormationNeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiDirectServiceRemotePortAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub EndpointPairs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))]
    EndpointPairs: usize,
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectServiceIPProtocol) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiDirectServiceSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub ServiceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectServiceSessionStatus) -> ::windows_sys::core::HRESULT,
    pub ErrorStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectServiceSessionErrorStatus) -> ::windows_sys::core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AdvertisementId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ServiceAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SessionAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub GetConnectionEndpointPairs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))]
    GetConnectionEndpointPairs: usize,
    #[cfg(feature = "Foundation")]
    pub SessionStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionStatusChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub AddStreamSocketListenerAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))]
    AddStreamSocketListenerAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub AddDatagramSocketAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))]
    AddDatagramSocketAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemotePortAdded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemotePortAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemotePortAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemotePortAdded: usize,
}
#[repr(C)]
pub struct IWiFiDirectServiceSessionDeferredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub DeferredSessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DeferredSessionInfo: usize,
}
#[repr(C)]
pub struct IWiFiDirectServiceSessionRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
    pub ProvisioningInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SessionInfo: usize,
}
#[repr(C)]
pub struct IWiFiDirectServiceSessionRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetSessionRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiDirectServiceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetSelector: unsafe extern "system" fn(this: *mut *mut Self, servicename: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetSelectorWithFilter: unsafe extern "system" fn(this: *mut *mut Self, servicename: ::windows_sys::core::HSTRING, serviceinfofilter: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetSelectorWithFilter: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
pub type WiFiDirectService = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceAdvertisementStatus(pub i32);
impl WiFiDirectServiceAdvertisementStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiDirectServiceAdvertisementStatus {}
impl ::core::clone::Clone for WiFiDirectServiceAdvertisementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WiFiDirectServiceAdvertiser = *mut ::core::ffi::c_void;
pub type WiFiDirectServiceAutoAcceptSessionConnectedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceConfigurationMethod(pub i32);
impl WiFiDirectServiceConfigurationMethod {
    pub const Default: Self = Self(0i32);
    pub const PinDisplay: Self = Self(1i32);
    pub const PinEntry: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectServiceConfigurationMethod {}
impl ::core::clone::Clone for WiFiDirectServiceConfigurationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceError(pub i32);
impl WiFiDirectServiceError {
    pub const Success: Self = Self(0i32);
    pub const RadioNotAvailable: Self = Self(1i32);
    pub const ResourceInUse: Self = Self(2i32);
    pub const UnsupportedHardware: Self = Self(3i32);
    pub const NoHardware: Self = Self(4i32);
}
impl ::core::marker::Copy for WiFiDirectServiceError {}
impl ::core::clone::Clone for WiFiDirectServiceError {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceIPProtocol(pub i32);
impl WiFiDirectServiceIPProtocol {
    pub const Tcp: Self = Self(6i32);
    pub const Udp: Self = Self(17i32);
}
impl ::core::marker::Copy for WiFiDirectServiceIPProtocol {}
impl ::core::clone::Clone for WiFiDirectServiceIPProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WiFiDirectServiceProvisioningInfo = *mut ::core::ffi::c_void;
pub type WiFiDirectServiceRemotePortAddedEventArgs = *mut ::core::ffi::c_void;
pub type WiFiDirectServiceSession = *mut ::core::ffi::c_void;
pub type WiFiDirectServiceSessionDeferredEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceSessionErrorStatus(pub i32);
impl WiFiDirectServiceSessionErrorStatus {
    pub const Ok: Self = Self(0i32);
    pub const Disassociated: Self = Self(1i32);
    pub const LocalClose: Self = Self(2i32);
    pub const RemoteClose: Self = Self(3i32);
    pub const SystemFailure: Self = Self(4i32);
    pub const NoResponseFromRemote: Self = Self(5i32);
}
impl ::core::marker::Copy for WiFiDirectServiceSessionErrorStatus {}
impl ::core::clone::Clone for WiFiDirectServiceSessionErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WiFiDirectServiceSessionRequest = *mut ::core::ffi::c_void;
pub type WiFiDirectServiceSessionRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceSessionStatus(pub i32);
impl WiFiDirectServiceSessionStatus {
    pub const Closed: Self = Self(0i32);
    pub const Initiated: Self = Self(1i32);
    pub const Requested: Self = Self(2i32);
    pub const Open: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiDirectServiceSessionStatus {}
impl ::core::clone::Clone for WiFiDirectServiceSessionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceStatus(pub i32);
impl WiFiDirectServiceStatus {
    pub const Available: Self = Self(0i32);
    pub const Busy: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectServiceStatus {}
impl ::core::clone::Clone for WiFiDirectServiceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
