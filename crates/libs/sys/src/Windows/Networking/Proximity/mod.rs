pub type ConnectionRequestedEventArgs = *mut ::core::ffi::c_void;
pub type DeviceArrivedEventHandler = *mut ::core::ffi::c_void;
pub type DeviceDepartedEventHandler = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IConnectionRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PeerInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConnectionRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3949498798, data2: 20254, data3: 19558, data4: [189, 13, 70, 146, 74, 148, 46, 8] };
}
#[repr(C)]
pub struct IPeerFinderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowBluetooth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowBluetooth: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowInfrastructure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowInfrastructure: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowWiFiDirect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowWiFiDirect: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SupportedDiscoveryTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PeerDiscoveryTypes) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AlternateIdentities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AlternateIdentities: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StartWithMessage: unsafe extern "system" fn(this: *mut *mut Self, peermessage: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TriggeredConnectionStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TriggeredConnectionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTriggeredConnectionStateChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTriggeredConnectionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionRequested: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionRequested: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllPeersAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllPeersAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut *mut Self, peerinformation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))]
    ConnectAsync: usize,
}
impl ::windows_sys::core::Interface for IPeerFinderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2437626721, data2: 63201, data3: 18372, data4: [161, 76, 20, 138, 25, 3, 208, 198] };
}
#[repr(C)]
pub struct IPeerFinderStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Role: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PeerRole) -> ::windows_sys::core::HRESULT,
    pub SetRole: unsafe extern "system" fn(this: *mut *mut Self, value: PeerRole) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub DiscoveryData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DiscoveryData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetDiscoveryData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetDiscoveryData: usize,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPeerFinderStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3605478501, data2: 64976, data3: 19211, data4: [147, 18, 134, 100, 8, 147, 93, 130] };
}
#[repr(C)]
pub struct IPeerInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPeerInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 537022216, data2: 40959, data3: 17908, data4: [182, 233, 64, 139, 46, 190, 243, 115] };
}
#[repr(C)]
pub struct IPeerInformation3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub DiscoveryData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DiscoveryData: usize,
}
impl ::windows_sys::core::Interface for IPeerInformation3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2987352362, data2: 56272, data3: 16632, data4: [149, 189, 45, 66, 9, 199, 131, 111] };
}
#[repr(C)]
pub struct IPeerInformationWithHostAndService {
    pub base__: ::windows_sys::core::IInspectable,
    pub HostName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ServiceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPeerInformationWithHostAndService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3972517037, data2: 7024, data3: 20107, data4: [146, 219, 187, 231, 129, 65, 147, 8] };
}
#[repr(C)]
pub struct IPeerWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PeerWatcherStatus) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPeerWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1022239224, data2: 12198, data3: 18041, data4: [150, 145, 3, 201, 74, 66, 15, 52] };
}
#[repr(C)]
pub struct IProximityDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub SubscribeForMessage: unsafe extern "system" fn(this: *mut *mut Self, messagetype: ::windows_sys::core::HSTRING, messagereceivedhandler: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub PublishMessage: unsafe extern "system" fn(this: *mut *mut Self, messagetype: ::windows_sys::core::HSTRING, message: ::windows_sys::core::HSTRING, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub PublishMessageWithCallback: unsafe extern "system" fn(this: *mut *mut Self, messagetype: ::windows_sys::core::HSTRING, message: ::windows_sys::core::HSTRING, messagetransmittedhandler: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PublishBinaryMessage: unsafe extern "system" fn(this: *mut *mut Self, messagetype: ::windows_sys::core::HSTRING, message: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublishBinaryMessage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PublishBinaryMessageWithCallback: unsafe extern "system" fn(this: *mut *mut Self, messagetype: ::windows_sys::core::HSTRING, message: *mut ::core::ffi::c_void, messagetransmittedhandler: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublishBinaryMessageWithCallback: usize,
    #[cfg(feature = "Foundation")]
    pub PublishUriMessage: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PublishUriMessage: usize,
    #[cfg(feature = "Foundation")]
    pub PublishUriMessageWithCallback: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, messagetransmittedhandler: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PublishUriMessageWithCallback: usize,
    pub StopSubscribingForMessage: unsafe extern "system" fn(this: *mut *mut Self, subscriptionid: i64) -> ::windows_sys::core::HRESULT,
    pub StopPublishingMessage: unsafe extern "system" fn(this: *mut *mut Self, messageid: i64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeviceArrived: unsafe extern "system" fn(this: *mut *mut Self, arrivedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceArrived: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceArrived: usize,
    #[cfg(feature = "Foundation")]
    pub DeviceDeparted: unsafe extern "system" fn(this: *mut *mut Self, departedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceDeparted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceDeparted: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceDeparted: usize,
    pub MaxMessageBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub BitsPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProximityDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4020806994, data2: 63201, data3: 17193, data4: [160, 252, 171, 107, 15, 210, 130, 98] };
}
#[repr(C)]
pub struct IProximityDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProximityDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2437652509, data2: 63201, data3: 18372, data4: [161, 76, 20, 138, 25, 3, 208, 198] };
}
#[repr(C)]
pub struct IProximityMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub MessageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SubscriptionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    pub DataAsString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProximityMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4020963202, data2: 63201, data3: 18037, data4: [160, 69, 216, 227, 32, 194, 72, 8] };
}
#[repr(C)]
pub struct ITriggeredConnectionStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TriggeredConnectState) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub Socket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    Socket: usize,
}
impl ::windows_sys::core::Interface for ITriggeredConnectionStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3332866221, data2: 63201, data3: 19796, data4: [150, 226, 51, 246, 32, 188, 168, 138] };
}
pub type MessageReceivedHandler = *mut ::core::ffi::c_void;
pub type MessageTransmittedHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct PeerDiscoveryTypes(pub u32);
impl PeerDiscoveryTypes {
    pub const None: Self = Self(0u32);
    pub const Browse: Self = Self(1u32);
    pub const Triggered: Self = Self(2u32);
}
impl ::core::marker::Copy for PeerDiscoveryTypes {}
impl ::core::clone::Clone for PeerDiscoveryTypes {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PeerInformation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct PeerRole(pub i32);
impl PeerRole {
    pub const Peer: Self = Self(0i32);
    pub const Host: Self = Self(1i32);
    pub const Client: Self = Self(2i32);
}
impl ::core::marker::Copy for PeerRole {}
impl ::core::clone::Clone for PeerRole {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PeerWatcher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct PeerWatcherStatus(pub i32);
impl PeerWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for PeerWatcherStatus {}
impl ::core::clone::Clone for PeerWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ProximityDevice = *mut ::core::ffi::c_void;
pub type ProximityMessage = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct TriggeredConnectState(pub i32);
impl TriggeredConnectState {
    pub const PeerFound: Self = Self(0i32);
    pub const Listening: Self = Self(1i32);
    pub const Connecting: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
    pub const Canceled: Self = Self(4i32);
    pub const Failed: Self = Self(5i32);
}
impl ::core::marker::Copy for TriggeredConnectState {}
impl ::core::clone::Clone for TriggeredConnectState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TriggeredConnectionStateChangedEventArgs = *mut ::core::ffi::c_void;
