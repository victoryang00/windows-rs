#[repr(C)]
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
pub struct BandwidthStatistics {
    pub OutboundBitsPerSecond: u64,
    pub InboundBitsPerSecond: u64,
    pub OutboundBitsPerSecondInstability: u64,
    pub InboundBitsPerSecondInstability: u64,
    pub OutboundBandwidthPeaked: bool,
    pub InboundBandwidthPeaked: bool,
}
impl ::core::marker::Copy for BandwidthStatistics {}
impl ::core::clone::Clone for BandwidthStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ControlChannelTrigger = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct ControlChannelTriggerResetReason(pub i32);
impl ControlChannelTriggerResetReason {
    pub const FastUserSwitched: Self = Self(0i32);
    pub const LowPowerExit: Self = Self(1i32);
    pub const QuietHoursExit: Self = Self(2i32);
    pub const ApplicationRestart: Self = Self(3i32);
}
impl ::core::marker::Copy for ControlChannelTriggerResetReason {}
impl ::core::clone::Clone for ControlChannelTriggerResetReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct ControlChannelTriggerResourceType(pub i32);
impl ControlChannelTriggerResourceType {
    pub const RequestSoftwareSlot: Self = Self(0i32);
    pub const RequestHardwareSlot: Self = Self(1i32);
}
impl ::core::marker::Copy for ControlChannelTriggerResourceType {}
impl ::core::clone::Clone for ControlChannelTriggerResourceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct ControlChannelTriggerStatus(pub i32);
impl ControlChannelTriggerStatus {
    pub const HardwareSlotRequested: Self = Self(0i32);
    pub const SoftwareSlotAllocated: Self = Self(1i32);
    pub const HardwareSlotAllocated: Self = Self(2i32);
    pub const PolicyError: Self = Self(3i32);
    pub const SystemError: Self = Self(4i32);
    pub const TransportDisconnected: Self = Self(5i32);
    pub const ServiceUnavailable: Self = Self(6i32);
}
impl ::core::marker::Copy for ControlChannelTriggerStatus {}
impl ::core::clone::Clone for ControlChannelTriggerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DatagramSocket = *mut ::core::ffi::c_void;
pub type DatagramSocketControl = *mut ::core::ffi::c_void;
pub type DatagramSocketInformation = *mut ::core::ffi::c_void;
pub type DatagramSocketMessageReceivedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IControlChannelTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub ControlChannelTriggerId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ServerKeepAliveIntervalInMinutes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetServerKeepAliveIntervalInMinutes: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CurrentKeepAliveIntervalInMinutes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TransportObject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Background")]
    pub KeepAliveTrigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    KeepAliveTrigger: usize,
    #[cfg(feature = "ApplicationModel_Background")]
    pub PushNotificationTrigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    PushNotificationTrigger: usize,
    pub UsingTransport: unsafe extern "system" fn(this: *mut *mut Self, transport: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WaitForPushEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ControlChannelTriggerStatus) -> ::windows_sys::core::HRESULT,
    pub DecreaseNetworkKeepAliveInterval: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub FlushTransport: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlChannelTrigger2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsWakeFromLowPowerSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlChannelTriggerEventDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub ControlChannelTrigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlChannelTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateControlChannelTrigger: unsafe extern "system" fn(this: *mut *mut Self, channelid: ::windows_sys::core::HSTRING, serverkeepaliveintervalinminutes: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateControlChannelTriggerEx: unsafe extern "system" fn(this: *mut *mut Self, channelid: ::windows_sys::core::HSTRING, serverkeepaliveintervalinminutes: u32, resourcerequesttype: ControlChannelTriggerResourceType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IControlChannelTriggerResetEventDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResetReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ControlChannelTriggerResetReason) -> ::windows_sys::core::HRESULT,
    pub HardwareSlotReset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SoftwareSlotReset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatagramSocket {
    pub base__: ::windows_sys::core::IInspectable,
    pub Control: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Information: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut *mut Self, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectWithEndpointPairAsync: unsafe extern "system" fn(this: *mut *mut Self, endpointpair: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectWithEndpointPairAsync: usize,
    #[cfg(feature = "Foundation")]
    pub BindServiceNameAsync: unsafe extern "system" fn(this: *mut *mut Self, localservicename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BindServiceNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub BindEndpointAsync: unsafe extern "system" fn(this: *mut *mut Self, localhostname: *mut ::core::ffi::c_void, localservicename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BindEndpointAsync: usize,
    pub JoinMulticastGroup: unsafe extern "system" fn(this: *mut *mut Self, host: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetOutputStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetOutputStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetOutputStreamWithEndpointPairAsync: unsafe extern "system" fn(this: *mut *mut Self, endpointpair: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetOutputStreamWithEndpointPairAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
}
#[repr(C)]
pub struct IDatagramSocket2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub BindServiceNameAndAdapterAsync: unsafe extern "system" fn(this: *mut *mut Self, localservicename: ::windows_sys::core::HSTRING, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity")))]
    BindServiceNameAndAdapterAsync: usize,
}
#[repr(C)]
pub struct IDatagramSocket3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CancelIOAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CancelIOAsync: usize,
    pub EnableTransferOwnership: unsafe extern "system" fn(this: *mut *mut Self, taskid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub EnableTransferOwnershipWithConnectedStandbyAction: unsafe extern "system" fn(this: *mut *mut Self, taskid: ::windows_sys::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows_sys::core::HRESULT,
    pub TransferOwnership: unsafe extern "system" fn(this: *mut *mut Self, socketid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TransferOwnershipWithContext: unsafe extern "system" fn(this: *mut *mut Self, socketid: ::windows_sys::core::HSTRING, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TransferOwnershipWithContextAndKeepAliveTime: unsafe extern "system" fn(this: *mut *mut Self, socketid: ::windows_sys::core::HSTRING, data: *mut ::core::ffi::c_void, keepalivetime: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransferOwnershipWithContextAndKeepAliveTime: usize,
}
#[repr(C)]
pub struct IDatagramSocketControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub QualityOfService: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketQualityOfService) -> ::windows_sys::core::HRESULT,
    pub SetQualityOfService: unsafe extern "system" fn(this: *mut *mut Self, value: SocketQualityOfService) -> ::windows_sys::core::HRESULT,
    pub OutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetOutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatagramSocketControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetInboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub DontFragment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDontFragment: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatagramSocketControl3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MulticastOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetMulticastOnly: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatagramSocketInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDatagramSocketMessageReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LocalAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetDataReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetDataReader: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetDataStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetDataStream: usize,
}
#[repr(C)]
pub struct IDatagramSocketStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEndpointPairsAsync: unsafe extern "system" fn(this: *mut *mut Self, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEndpointPairsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEndpointPairsWithSortOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::windows_sys::core::HSTRING, sortoptions: super::HostNameSortOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEndpointPairsWithSortOptionsAsync: usize,
}
#[repr(C)]
pub struct IMessageWebSocket {
    pub base__: ::windows_sys::core::IInspectable,
    pub Control: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Information: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
}
#[repr(C)]
pub struct IMessageWebSocket2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ServerCustomValidationRequested: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServerCustomValidationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServerCustomValidationRequested: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServerCustomValidationRequested: usize,
}
#[repr(C)]
pub struct IMessageWebSocket3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendNonfinalFrameAsync: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendNonfinalFrameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendFinalFrameAsync: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendFinalFrameAsync: usize,
}
#[repr(C)]
pub struct IMessageWebSocketControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxMessageSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxMessageSize: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub MessageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketMessageType) -> ::windows_sys::core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(this: *mut *mut Self, value: SocketMessageType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMessageWebSocketControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DesiredUnsolicitedPongInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredUnsolicitedPongInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredUnsolicitedPongInterval: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredUnsolicitedPongInterval: usize,
    #[cfg(feature = "Foundation")]
    pub ActualUnsolicitedPongInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActualUnsolicitedPongInterval: usize,
    pub ReceiveMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MessageWebSocketReceiveMode) -> ::windows_sys::core::HRESULT,
    pub SetReceiveMode: unsafe extern "system" fn(this: *mut *mut Self, value: MessageWebSocketReceiveMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ClientCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ClientCertificate: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub SetClientCertificate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    SetClientCertificate: usize,
}
#[repr(C)]
pub struct IMessageWebSocketMessageReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub MessageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketMessageType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetDataReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetDataReader: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetDataStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetDataStream: usize,
}
#[repr(C)]
pub struct IMessageWebSocketMessageReceivedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsMessageComplete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServerMessageWebSocket {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
    pub Control: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Information: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    pub CloseWithStatus: unsafe extern "system" fn(this: *mut *mut Self, code: u16, reason: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServerMessageWebSocketControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub MessageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketMessageType) -> ::windows_sys::core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(this: *mut *mut Self, value: SocketMessageType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServerMessageWebSocketInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub BandwidthStatistics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BandwidthStatistics) -> ::windows_sys::core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LocalAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServerStreamWebSocket {
    pub base__: ::windows_sys::core::IInspectable,
    pub Information: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    pub CloseWithStatus: unsafe extern "system" fn(this: *mut *mut Self, code: u16, reason: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServerStreamWebSocketInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub BandwidthStatistics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BandwidthStatistics) -> ::windows_sys::core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LocalAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISocketActivityContext {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
#[repr(C)]
pub struct ISocketActivityContextFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
#[repr(C)]
pub struct ISocketActivityInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SocketKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketActivityKind) -> ::windows_sys::core::HRESULT,
    pub Context: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DatagramSocket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StreamSocket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StreamSocketListener: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISocketActivityInformationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AllSockets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllSockets: usize,
}
#[repr(C)]
pub struct ISocketActivityTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketActivityTriggerReason) -> ::windows_sys::core::HRESULT,
    pub SocketInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISocketErrorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, hresult: i32, result__: *mut SocketErrorStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStreamSocket {
    pub base__: ::windows_sys::core::IInspectable,
    pub Control: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Information: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectWithEndpointPairAsync: unsafe extern "system" fn(this: *mut *mut Self, endpointpair: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectWithEndpointPairAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut *mut Self, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectWithEndpointPairAndProtectionLevelAsync: unsafe extern "system" fn(this: *mut *mut Self, endpointpair: *mut ::core::ffi::c_void, protectionlevel: SocketProtectionLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectWithEndpointPairAndProtectionLevelAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectWithProtectionLevelAsync: unsafe extern "system" fn(this: *mut *mut Self, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::windows_sys::core::HSTRING, protectionlevel: SocketProtectionLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectWithProtectionLevelAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpgradeToSslAsync: unsafe extern "system" fn(this: *mut *mut Self, protectionlevel: SocketProtectionLevel, validationhostname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpgradeToSslAsync: usize,
}
#[repr(C)]
pub struct IStreamSocket2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub ConnectWithProtectionLevelAndAdapterAsync: unsafe extern "system" fn(this: *mut *mut Self, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::windows_sys::core::HSTRING, protectionlevel: SocketProtectionLevel, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity")))]
    ConnectWithProtectionLevelAndAdapterAsync: usize,
}
#[repr(C)]
pub struct IStreamSocket3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CancelIOAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CancelIOAsync: usize,
    pub EnableTransferOwnership: unsafe extern "system" fn(this: *mut *mut Self, taskid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub EnableTransferOwnershipWithConnectedStandbyAction: unsafe extern "system" fn(this: *mut *mut Self, taskid: ::windows_sys::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows_sys::core::HRESULT,
    pub TransferOwnership: unsafe extern "system" fn(this: *mut *mut Self, socketid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TransferOwnershipWithContext: unsafe extern "system" fn(this: *mut *mut Self, socketid: ::windows_sys::core::HSTRING, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TransferOwnershipWithContextAndKeepAliveTime: unsafe extern "system" fn(this: *mut *mut Self, socketid: ::windows_sys::core::HSTRING, data: *mut ::core::ffi::c_void, keepalivetime: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransferOwnershipWithContextAndKeepAliveTime: usize,
}
#[repr(C)]
pub struct IStreamSocketControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub NoDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetNoDelay: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub KeepAlive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetKeepAlive: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub OutboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetOutboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub QualityOfService: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketQualityOfService) -> ::windows_sys::core::HRESULT,
    pub SetQualityOfService: unsafe extern "system" fn(this: *mut *mut Self, value: SocketQualityOfService) -> ::windows_sys::core::HRESULT,
    pub OutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetOutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStreamSocketControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub IgnorableServerCertificateErrors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    IgnorableServerCertificateErrors: usize,
}
#[repr(C)]
pub struct IStreamSocketControl3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SerializeConnectionAttempts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSerializeConnectionAttempts: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ClientCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ClientCertificate: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub SetClientCertificate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    SetClientCertificate: usize,
}
#[repr(C)]
pub struct IStreamSocketControl4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MinProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub SetMinProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, value: SocketProtectionLevel) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStreamSocketInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteHostName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoteAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoteServiceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RoundTripTimeStatistics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RoundTripTimeStatistics) -> ::windows_sys::core::HRESULT,
    pub BandwidthStatistics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BandwidthStatistics) -> ::windows_sys::core::HRESULT,
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketProtectionLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SessionKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SessionKey: usize,
}
#[repr(C)]
pub struct IStreamSocketInformation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketSslErrorSeverity) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
}
#[repr(C)]
pub struct IStreamSocketListener {
    pub base__: ::windows_sys::core::IInspectable,
    pub Control: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Information: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BindServiceNameAsync: unsafe extern "system" fn(this: *mut *mut Self, localservicename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BindServiceNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub BindEndpointAsync: unsafe extern "system" fn(this: *mut *mut Self, localhostname: *mut ::core::ffi::c_void, localservicename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BindEndpointAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionReceived: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionReceived: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionReceived: usize,
}
#[repr(C)]
pub struct IStreamSocketListener2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BindServiceNameWithProtectionLevelAsync: unsafe extern "system" fn(this: *mut *mut Self, localservicename: ::windows_sys::core::HSTRING, protectionlevel: SocketProtectionLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BindServiceNameWithProtectionLevelAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub BindServiceNameWithProtectionLevelAndAdapterAsync: unsafe extern "system" fn(this: *mut *mut Self, localservicename: ::windows_sys::core::HSTRING, protectionlevel: SocketProtectionLevel, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity")))]
    BindServiceNameWithProtectionLevelAndAdapterAsync: usize,
}
#[repr(C)]
pub struct IStreamSocketListener3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CancelIOAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CancelIOAsync: usize,
    pub EnableTransferOwnership: unsafe extern "system" fn(this: *mut *mut Self, taskid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub EnableTransferOwnershipWithConnectedStandbyAction: unsafe extern "system" fn(this: *mut *mut Self, taskid: ::windows_sys::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows_sys::core::HRESULT,
    pub TransferOwnership: unsafe extern "system" fn(this: *mut *mut Self, socketid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TransferOwnershipWithContext: unsafe extern "system" fn(this: *mut *mut Self, socketid: ::windows_sys::core::HSTRING, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStreamSocketListenerConnectionReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Socket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStreamSocketListenerControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub QualityOfService: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketQualityOfService) -> ::windows_sys::core::HRESULT,
    pub SetQualityOfService: unsafe extern "system" fn(this: *mut *mut Self, value: SocketQualityOfService) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStreamSocketListenerControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub NoDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetNoDelay: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub KeepAlive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetKeepAlive: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub OutboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetOutboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub OutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetOutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStreamSocketListenerInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalPort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStreamSocketStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEndpointPairsAsync: unsafe extern "system" fn(this: *mut *mut Self, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEndpointPairsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEndpointPairsWithSortOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::windows_sys::core::HSTRING, sortoptions: super::HostNameSortOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEndpointPairsWithSortOptionsAsync: usize,
}
#[repr(C)]
pub struct IStreamWebSocket {
    pub base__: ::windows_sys::core::IInspectable,
    pub Control: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Information: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
}
#[repr(C)]
pub struct IStreamWebSocket2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ServerCustomValidationRequested: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServerCustomValidationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServerCustomValidationRequested: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServerCustomValidationRequested: usize,
}
#[repr(C)]
pub struct IStreamWebSocketControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub NoDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetNoDelay: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStreamWebSocketControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DesiredUnsolicitedPongInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredUnsolicitedPongInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredUnsolicitedPongInterval: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredUnsolicitedPongInterval: usize,
    #[cfg(feature = "Foundation")]
    pub ActualUnsolicitedPongInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActualUnsolicitedPongInterval: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ClientCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ClientCertificate: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub SetClientCertificate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    SetClientCertificate: usize,
}
#[repr(C)]
pub struct IWebSocket {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut *mut Self, headername: ::windows_sys::core::HSTRING, headervalue: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    pub CloseWithStatus: unsafe extern "system" fn(this: *mut *mut Self, code: u16, reason: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebSocketClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Code: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebSocketControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub OutboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetOutboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub ServerCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetServerCredential: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ProxyCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetProxyCredential: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetProxyCredential: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedProtocols: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedProtocols: usize,
}
#[repr(C)]
pub struct IWebSocketControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub IgnorableServerCertificateErrors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    IgnorableServerCertificateErrors: usize,
}
#[repr(C)]
pub struct IWebSocketErrorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Web")]
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, hresult: i32, result__: *mut super::super::Web::WebErrorStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web"))]
    GetStatus: usize,
}
#[repr(C)]
pub struct IWebSocketInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BandwidthStatistics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BandwidthStatistics) -> ::windows_sys::core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebSocketInformation2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketSslErrorSeverity) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
}
#[repr(C)]
pub struct IWebSocketServerCustomValidationRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketSslErrorSeverity) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
    pub Reject: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
pub type MessageWebSocket = *mut ::core::ffi::c_void;
pub type MessageWebSocketControl = *mut ::core::ffi::c_void;
pub type MessageWebSocketInformation = *mut ::core::ffi::c_void;
pub type MessageWebSocketMessageReceivedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct MessageWebSocketReceiveMode(pub i32);
impl MessageWebSocketReceiveMode {
    pub const FullMessage: Self = Self(0i32);
    pub const PartialMessage: Self = Self(1i32);
}
impl ::core::marker::Copy for MessageWebSocketReceiveMode {}
impl ::core::clone::Clone for MessageWebSocketReceiveMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
pub struct RoundTripTimeStatistics {
    pub Variance: u32,
    pub Max: u32,
    pub Min: u32,
    pub Sum: u32,
}
impl ::core::marker::Copy for RoundTripTimeStatistics {}
impl ::core::clone::Clone for RoundTripTimeStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ServerMessageWebSocket = *mut ::core::ffi::c_void;
pub type ServerMessageWebSocketControl = *mut ::core::ffi::c_void;
pub type ServerMessageWebSocketInformation = *mut ::core::ffi::c_void;
pub type ServerStreamWebSocket = *mut ::core::ffi::c_void;
pub type ServerStreamWebSocketInformation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct SocketActivityConnectedStandbyAction(pub i32);
impl SocketActivityConnectedStandbyAction {
    pub const DoNotWake: Self = Self(0i32);
    pub const Wake: Self = Self(1i32);
}
impl ::core::marker::Copy for SocketActivityConnectedStandbyAction {}
impl ::core::clone::Clone for SocketActivityConnectedStandbyAction {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SocketActivityContext = *mut ::core::ffi::c_void;
pub type SocketActivityInformation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct SocketActivityKind(pub i32);
impl SocketActivityKind {
    pub const None: Self = Self(0i32);
    pub const StreamSocketListener: Self = Self(1i32);
    pub const DatagramSocket: Self = Self(2i32);
    pub const StreamSocket: Self = Self(3i32);
}
impl ::core::marker::Copy for SocketActivityKind {}
impl ::core::clone::Clone for SocketActivityKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SocketActivityTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct SocketActivityTriggerReason(pub i32);
impl SocketActivityTriggerReason {
    pub const None: Self = Self(0i32);
    pub const SocketActivity: Self = Self(1i32);
    pub const ConnectionAccepted: Self = Self(2i32);
    pub const KeepAliveTimerExpired: Self = Self(3i32);
    pub const SocketClosed: Self = Self(4i32);
}
impl ::core::marker::Copy for SocketActivityTriggerReason {}
impl ::core::clone::Clone for SocketActivityTriggerReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct SocketErrorStatus(pub i32);
impl SocketErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const OperationAborted: Self = Self(1i32);
    pub const HttpInvalidServerResponse: Self = Self(2i32);
    pub const ConnectionTimedOut: Self = Self(3i32);
    pub const AddressFamilyNotSupported: Self = Self(4i32);
    pub const SocketTypeNotSupported: Self = Self(5i32);
    pub const HostNotFound: Self = Self(6i32);
    pub const NoDataRecordOfRequestedType: Self = Self(7i32);
    pub const NonAuthoritativeHostNotFound: Self = Self(8i32);
    pub const ClassTypeNotFound: Self = Self(9i32);
    pub const AddressAlreadyInUse: Self = Self(10i32);
    pub const CannotAssignRequestedAddress: Self = Self(11i32);
    pub const ConnectionRefused: Self = Self(12i32);
    pub const NetworkIsUnreachable: Self = Self(13i32);
    pub const UnreachableHost: Self = Self(14i32);
    pub const NetworkIsDown: Self = Self(15i32);
    pub const NetworkDroppedConnectionOnReset: Self = Self(16i32);
    pub const SoftwareCausedConnectionAbort: Self = Self(17i32);
    pub const ConnectionResetByPeer: Self = Self(18i32);
    pub const HostIsDown: Self = Self(19i32);
    pub const NoAddressesFound: Self = Self(20i32);
    pub const TooManyOpenFiles: Self = Self(21i32);
    pub const MessageTooLong: Self = Self(22i32);
    pub const CertificateExpired: Self = Self(23i32);
    pub const CertificateUntrustedRoot: Self = Self(24i32);
    pub const CertificateCommonNameIsIncorrect: Self = Self(25i32);
    pub const CertificateWrongUsage: Self = Self(26i32);
    pub const CertificateRevoked: Self = Self(27i32);
    pub const CertificateNoRevocationCheck: Self = Self(28i32);
    pub const CertificateRevocationServerOffline: Self = Self(29i32);
    pub const CertificateIsInvalid: Self = Self(30i32);
}
impl ::core::marker::Copy for SocketErrorStatus {}
impl ::core::clone::Clone for SocketErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct SocketMessageType(pub i32);
impl SocketMessageType {
    pub const Binary: Self = Self(0i32);
    pub const Utf8: Self = Self(1i32);
}
impl ::core::marker::Copy for SocketMessageType {}
impl ::core::clone::Clone for SocketMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct SocketProtectionLevel(pub i32);
impl SocketProtectionLevel {
    pub const PlainSocket: Self = Self(0i32);
    pub const Ssl: Self = Self(1i32);
    pub const SslAllowNullEncryption: Self = Self(2i32);
    pub const BluetoothEncryptionAllowNullAuthentication: Self = Self(3i32);
    pub const BluetoothEncryptionWithAuthentication: Self = Self(4i32);
    pub const Ssl3AllowWeakEncryption: Self = Self(5i32);
    pub const Tls10: Self = Self(6i32);
    pub const Tls11: Self = Self(7i32);
    pub const Tls12: Self = Self(8i32);
    pub const Unspecified: Self = Self(9i32);
}
impl ::core::marker::Copy for SocketProtectionLevel {}
impl ::core::clone::Clone for SocketProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct SocketQualityOfService(pub i32);
impl SocketQualityOfService {
    pub const Normal: Self = Self(0i32);
    pub const LowLatency: Self = Self(1i32);
}
impl ::core::marker::Copy for SocketQualityOfService {}
impl ::core::clone::Clone for SocketQualityOfService {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct SocketSslErrorSeverity(pub i32);
impl SocketSslErrorSeverity {
    pub const None: Self = Self(0i32);
    pub const Ignorable: Self = Self(1i32);
    pub const Fatal: Self = Self(2i32);
}
impl ::core::marker::Copy for SocketSslErrorSeverity {}
impl ::core::clone::Clone for SocketSslErrorSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StreamSocket = *mut ::core::ffi::c_void;
pub type StreamSocketControl = *mut ::core::ffi::c_void;
pub type StreamSocketInformation = *mut ::core::ffi::c_void;
pub type StreamSocketListener = *mut ::core::ffi::c_void;
pub type StreamSocketListenerConnectionReceivedEventArgs = *mut ::core::ffi::c_void;
pub type StreamSocketListenerControl = *mut ::core::ffi::c_void;
pub type StreamSocketListenerInformation = *mut ::core::ffi::c_void;
pub type StreamWebSocket = *mut ::core::ffi::c_void;
pub type StreamWebSocketControl = *mut ::core::ffi::c_void;
pub type StreamWebSocketInformation = *mut ::core::ffi::c_void;
pub type WebSocketClosedEventArgs = *mut ::core::ffi::c_void;
pub type WebSocketKeepAlive = *mut ::core::ffi::c_void;
pub type WebSocketServerCustomValidationRequestedEventArgs = *mut ::core::ffi::c_void;
