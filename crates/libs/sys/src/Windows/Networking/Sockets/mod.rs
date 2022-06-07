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
impl ::windows_sys::core::Interface for IControlChannelTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2098475431, data2: 61078, data3: 16616, data4: [161, 153, 135, 3, 205, 150, 158, 195] };
}
#[repr(C)]
pub struct IControlChannelTrigger2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsWakeFromLowPowerSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IControlChannelTrigger2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2936066615, data2: 20926, data3: 17684, data4: [151, 37, 53, 86, 225, 135, 149, 128] };
}
#[repr(C)]
pub struct IControlChannelTriggerEventDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub ControlChannelTrigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IControlChannelTriggerEventDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 456581191, data2: 35259, data3: 16950, data4: [150, 172, 113, 208, 18, 187, 72, 105] };
}
#[repr(C)]
pub struct IControlChannelTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateControlChannelTrigger: unsafe extern "system" fn(this: *mut *mut Self, channelid: ::windows_sys::core::HSTRING, serverkeepaliveintervalinminutes: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateControlChannelTriggerEx: unsafe extern "system" fn(this: *mut *mut Self, channelid: ::windows_sys::core::HSTRING, serverkeepaliveintervalinminutes: u32, resourcerequesttype: ControlChannelTriggerResourceType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IControlChannelTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3662380272, data2: 36209, data3: 17519, data4: [136, 195, 185, 81, 132, 162, 214, 205] };
}
#[repr(C)]
pub struct IControlChannelTriggerResetEventDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResetReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ControlChannelTriggerResetReason) -> ::windows_sys::core::HRESULT,
    pub HardwareSlotReset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SoftwareSlotReset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IControlChannelTriggerResetEventDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1750139790, data2: 36548, data3: 17150, data4: [155, 178, 33, 233, 27, 123, 252, 177] };
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
impl ::windows_sys::core::Interface for IDatagramSocket {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2145541051, data2: 50108, data3: 18039, data4: [132, 70, 202, 40, 164, 101, 163, 175] };
}
#[repr(C)]
pub struct IDatagramSocket2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub BindServiceNameAndAdapterAsync: unsafe extern "system" fn(this: *mut *mut Self, localservicename: ::windows_sys::core::HSTRING, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity")))]
    BindServiceNameAndAdapterAsync: usize,
}
impl ::windows_sys::core::Interface for IDatagramSocket2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3627787092, data2: 39581, data3: 16773, data4: [162, 10, 20, 36, 201, 194, 167, 205] };
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
impl ::windows_sys::core::Interface for IDatagramSocket3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 928272137, data2: 43922, data3: 17158, data4: [154, 193, 12, 56, 18, 131, 217, 198] };
}
#[repr(C)]
pub struct IDatagramSocketControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub QualityOfService: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketQualityOfService) -> ::windows_sys::core::HRESULT,
    pub SetQualityOfService: unsafe extern "system" fn(this: *mut *mut Self, value: SocketQualityOfService) -> ::windows_sys::core::HRESULT,
    pub OutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetOutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDatagramSocketControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1387020078, data2: 13466, data3: 16693, data4: [187, 88, 183, 155, 38, 71, 211, 144] };
}
#[repr(C)]
pub struct IDatagramSocketControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetInboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub DontFragment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDontFragment: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDatagramSocketControl2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 871028162, data2: 38812, data3: 17429, data4: [130, 161, 60, 250, 246, 70, 193, 146] };
}
#[repr(C)]
pub struct IDatagramSocketControl3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MulticastOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetMulticastOnly: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDatagramSocketControl3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3572204118, data2: 8045, data3: 17816, data4: [155, 87, 212, 42, 0, 29, 243, 73] };
}
#[repr(C)]
pub struct IDatagramSocketInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDatagramSocketInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1595561626, data2: 22011, data3: 18637, data4: [151, 6, 122, 151, 79, 123, 21, 133] };
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
impl ::windows_sys::core::Interface for IDatagramSocketMessageReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2653805730, data2: 5906, data3: 19684, data4: [177, 121, 140, 101, 44, 109, 16, 126] };
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
impl ::windows_sys::core::Interface for IDatagramSocketStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3922078446, data2: 5268, data3: 18977, data4: [187, 126, 133, 137, 252, 117, 29, 157] };
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
impl ::windows_sys::core::Interface for IMessageWebSocket {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 863141128, data2: 13525, data3: 18246, data4: [173, 123, 141, 222, 91, 194, 239, 136] };
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
impl ::windows_sys::core::Interface for IMessageWebSocket2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3201355495, data2: 63944, data3: 17418, data4: [154, 213, 115, 114, 129, 217, 116, 46] };
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
impl ::windows_sys::core::Interface for IMessageWebSocket3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1507450619, data2: 29103, data3: 17225, data4: [132, 135, 145, 31, 207, 104, 21, 151] };
}
#[repr(C)]
pub struct IMessageWebSocketControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxMessageSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxMessageSize: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub MessageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketMessageType) -> ::windows_sys::core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(this: *mut *mut Self, value: SocketMessageType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMessageWebSocketControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2165848202, data2: 50729, data3: 20234, data4: [128, 251, 129, 252, 5, 83, 136, 98] };
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
impl ::windows_sys::core::Interface for IMessageWebSocketControl2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3809466257, data2: 2060, data3: 16394, data4: [167, 18, 39, 223, 169, 231, 68, 216] };
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
impl ::windows_sys::core::Interface for IMessageWebSocketMessageReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1200366252, data2: 19531, data3: 17133, data4: [158, 215, 30, 249, 249, 79, 163, 213] };
}
#[repr(C)]
pub struct IMessageWebSocketMessageReceivedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsMessageComplete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMessageWebSocketMessageReceivedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2311980797, data2: 56687, data3: 18951, data4: [135, 249, 249, 235, 77, 137, 216, 61] };
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
impl ::windows_sys::core::Interface for IServerMessageWebSocket {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3819737664, data2: 33083, data3: 24317, data4: [126, 17, 174, 35, 5, 252, 119, 241] };
}
#[repr(C)]
pub struct IServerMessageWebSocketControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub MessageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketMessageType) -> ::windows_sys::core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(this: *mut *mut Self, value: SocketMessageType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IServerMessageWebSocketControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1774383185, data2: 7199, data3: 22650, data4: [69, 25, 33, 129, 97, 1, 146, 183] };
}
#[repr(C)]
pub struct IServerMessageWebSocketInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub BandwidthStatistics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BandwidthStatistics) -> ::windows_sys::core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LocalAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IServerMessageWebSocketInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4231181407, data2: 17480, data3: 21765, data4: [108, 201, 9, 175, 168, 145, 95, 93] };
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
impl ::windows_sys::core::Interface for IServerStreamWebSocket {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 753753023, data2: 29942, data3: 21988, data4: [121, 223, 145, 50, 104, 13, 254, 232] };
}
#[repr(C)]
pub struct IServerStreamWebSocketInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub BandwidthStatistics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BandwidthStatistics) -> ::windows_sys::core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LocalAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IServerStreamWebSocketInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4231181407, data2: 17480, data3: 21765, data4: [108, 201, 9, 171, 168, 145, 95, 93] };
}
#[repr(C)]
pub struct ISocketActivityContext {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
impl ::windows_sys::core::Interface for ISocketActivityContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1135627620, data2: 19589, data3: 17302, data4: [166, 55, 29, 151, 63, 110, 189, 73] };
}
#[repr(C)]
pub struct ISocketActivityContextFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for ISocketActivityContextFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3114255299, data2: 2188, data3: 17288, data4: [131, 174, 37, 37, 19, 142, 4, 154] };
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
impl ::windows_sys::core::Interface for ISocketActivityInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2374648548, data2: 43134, data3: 19316, data4: [153, 104, 24, 91, 37, 17, 222, 254] };
}
#[repr(C)]
pub struct ISocketActivityInformationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AllSockets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllSockets: usize,
}
impl ::windows_sys::core::Interface for ISocketActivityInformationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2238755962, data2: 32381, data3: 18230, data4: [128, 65, 19, 39, 166, 84, 60, 86] };
}
#[repr(C)]
pub struct ISocketActivityTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketActivityTriggerReason) -> ::windows_sys::core::HRESULT,
    pub SocketInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISocketActivityTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1173620391, data2: 64671, data3: 20353, data4: [172, 173, 53, 95, 239, 81, 230, 123] };
}
#[repr(C)]
pub struct ISocketErrorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, hresult: i32, result__: *mut SocketErrorStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISocketErrorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2189637620, data2: 32086, data3: 19854, data4: [183, 180, 160, 125, 215, 193, 188, 169] };
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
impl ::windows_sys::core::Interface for IStreamSocket {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1772236019, data2: 64635, data3: 18519, data4: [175, 56, 246, 231, 222, 106, 91, 73] };
}
#[repr(C)]
pub struct IStreamSocket2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub ConnectWithProtectionLevelAndAdapterAsync: unsafe extern "system" fn(this: *mut *mut Self, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::windows_sys::core::HSTRING, protectionlevel: SocketProtectionLevel, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity")))]
    ConnectWithProtectionLevelAndAdapterAsync: usize,
}
impl ::windows_sys::core::Interface for IStreamSocket2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 701556085, data2: 62228, data3: 19721, data4: [173, 240, 15, 189, 150, 127, 189, 159] };
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
impl ::windows_sys::core::Interface for IStreamSocket3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1061358336, data2: 40232, data3: 18516, data4: [186, 195, 35, 1, 148, 30, 194, 35] };
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
impl ::windows_sys::core::Interface for IStreamSocketControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4263882225, data2: 37547, data3: 19187, data4: [153, 146, 15, 76, 133, 227, 108, 196] };
}
#[repr(C)]
pub struct IStreamSocketControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub IgnorableServerCertificateErrors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    IgnorableServerCertificateErrors: usize,
}
impl ::windows_sys::core::Interface for IStreamSocketControl2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3268450902, data2: 1551, data3: 17601, data4: [184, 226, 31, 191, 96, 189, 98, 197] };
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
impl ::windows_sys::core::Interface for IStreamSocketControl3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3312075852, data2: 20084, data3: 16446, data4: [137, 76, 179, 28, 174, 92, 115, 66] };
}
#[repr(C)]
pub struct IStreamSocketControl4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MinProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub SetMinProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, value: SocketProtectionLevel) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStreamSocketControl4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2521705277, data2: 60455, data3: 18568, data4: [179, 206, 199, 75, 65, 132, 35, 173] };
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
impl ::windows_sys::core::Interface for IStreamSocketInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 998288944, data2: 24168, data3: 16901, data4: [136, 240, 220, 133, 210, 226, 93, 237] };
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
impl ::windows_sys::core::Interface for IStreamSocketInformation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 314737746, data2: 19420, data3: 20196, data4: [151, 106, 207, 19, 14, 157, 146, 227] };
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
impl ::windows_sys::core::Interface for IStreamSocketListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4283511863, data2: 57247, data3: 19952, data4: [191, 130, 14, 197, 215, 179, 90, 174] };
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
impl ::windows_sys::core::Interface for IStreamSocketListener2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1703788862, data2: 47934, data3: 17496, data4: [178, 50, 237, 16, 136, 105, 75, 152] };
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
impl ::windows_sys::core::Interface for IStreamSocketListener3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1201152028, data2: 48632, data3: 18713, data4: [133, 66, 40, 212, 80, 231, 69, 7] };
}
#[repr(C)]
pub struct IStreamSocketListenerConnectionReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Socket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStreamSocketListenerConnectionReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 205991593, data2: 14143, data3: 17531, data4: [133, 177, 221, 212, 84, 136, 3, 186] };
}
#[repr(C)]
pub struct IStreamSocketListenerControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub QualityOfService: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocketQualityOfService) -> ::windows_sys::core::HRESULT,
    pub SetQualityOfService: unsafe extern "system" fn(this: *mut *mut Self, value: SocketQualityOfService) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStreamSocketListenerControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 551077238, data2: 36234, data3: 19898, data4: [151, 34, 161, 108, 77, 152, 73, 128] };
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
impl ::windows_sys::core::Interface for IStreamSocketListenerControl2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2492184165, data2: 11326, data3: 16459, data4: [184, 176, 142, 178, 73, 162, 176, 161] };
}
#[repr(C)]
pub struct IStreamSocketListenerInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalPort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStreamSocketListenerInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3861620783, data2: 42554, data3: 17163, data4: [191, 98, 41, 233, 62, 86, 51, 180] };
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
impl ::windows_sys::core::Interface for IStreamSocketStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2753608778, data2: 28206, data3: 19189, data4: [181, 86, 53, 90, 224, 205, 79, 41] };
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
impl ::windows_sys::core::Interface for IStreamWebSocket {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3175762392, data2: 45705, data3: 17851, data4: [151, 235, 199, 82, 82, 5, 168, 67] };
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
impl ::windows_sys::core::Interface for IStreamWebSocket2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2857175243, data2: 37877, data3: 18040, data4: [130, 54, 87, 204, 229, 65, 126, 213] };
}
#[repr(C)]
pub struct IStreamWebSocketControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub NoDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetNoDelay: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStreamWebSocketControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3035920561, data2: 42074, data3: 18651, data4: [149, 58, 100, 91, 125, 150, 76, 7] };
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
impl ::windows_sys::core::Interface for IStreamWebSocketControl2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 559783806, data2: 64088, data3: 16602, data4: [159, 17, 164, 141, 175, 233, 80, 55] };
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
impl ::windows_sys::core::Interface for IWebSocket {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4168563055, data2: 39345, data3: 19992, data4: [188, 8, 133, 12, 154, 223, 21, 110] };
}
#[repr(C)]
pub struct IWebSocketClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Code: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebSocketClosedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3468135687, data2: 53416, data3: 18179, data4: [160, 145, 200, 194, 192, 145, 91, 195] };
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
impl ::windows_sys::core::Interface for IWebSocketControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 784645571, data2: 55717, data3: 17754, data4: [152, 17, 222, 36, 212, 83, 55, 233] };
}
#[repr(C)]
pub struct IWebSocketControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub IgnorableServerCertificateErrors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    IgnorableServerCertificateErrors: usize,
}
impl ::windows_sys::core::Interface for IWebSocketControl2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2042871299, data2: 62154, data3: 17950, data4: [175, 78, 150, 101, 188, 45, 6, 32] };
}
#[repr(C)]
pub struct IWebSocketErrorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Web")]
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, hresult: i32, result__: *mut super::super::Web::WebErrorStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web"))]
    GetStatus: usize,
}
impl ::windows_sys::core::Interface for IWebSocketErrorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 667808603, data2: 8033, data3: 18185, data4: [142, 2, 97, 40, 58, 218, 78, 157] };
}
#[repr(C)]
pub struct IWebSocketInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BandwidthStatistics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BandwidthStatistics) -> ::windows_sys::core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebSocketInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1577181974, data2: 51498, data3: 18341, data4: [178, 95, 7, 132, 118, 57, 209, 129] };
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
impl ::windows_sys::core::Interface for IWebSocketInformation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3458021838, data2: 41399, data3: 19779, data4: [130, 105, 141, 91, 152, 27, 212, 122] };
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
impl ::windows_sys::core::Interface for IWebSocketServerCustomValidationRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4293918280, data2: 554, data3: 19127, data4: [139, 54, 225, 10, 244, 100, 14, 107] };
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
