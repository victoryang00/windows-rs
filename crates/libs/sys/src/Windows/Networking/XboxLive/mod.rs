#[repr(C)]
pub struct IXboxLiveDeviceAddress {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SnapshotChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SnapshotChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSnapshotChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSnapshotChanged: usize,
    pub GetSnapshotAsBase64: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetSnapshotAsBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetSnapshotAsBuffer: usize,
    pub GetSnapshotAsBytes: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *mut u8, byteswritten: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Compare: unsafe extern "system" fn(this: *mut *mut Self, otherdeviceaddress: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsValid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub NetworkAccessKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut XboxLiveNetworkAccessKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXboxLiveDeviceAddressStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromSnapshotBase64: unsafe extern "system" fn(this: *mut *mut Self, base64: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromSnapshotBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromSnapshotBuffer: usize,
    pub CreateFromSnapshotBytes: unsafe extern "system" fn(this: *mut *mut Self, buffer_array_size: u32, buffer: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLocal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxSnapshotBytesSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXboxLiveEndpointPair {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    pub GetRemoteSocketAddressBytes: unsafe extern "system" fn(this: *mut *mut Self, socketAddress_array_size: u32, socketaddress: *mut u8) -> ::windows_sys::core::HRESULT,
    pub GetLocalSocketAddressBytes: unsafe extern "system" fn(this: *mut *mut Self, socketAddress_array_size: u32, socketaddress: *mut u8) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut XboxLiveEndpointPairState) -> ::windows_sys::core::HRESULT,
    pub Template: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoteDeviceAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoteHostName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LocalHostName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXboxLiveEndpointPairCreationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut XboxLiveEndpointPairCreationStatus) -> ::windows_sys::core::HRESULT,
    pub IsExistingPathEvaluation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub EndpointPair: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXboxLiveEndpointPairStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut XboxLiveEndpointPairState) -> ::windows_sys::core::HRESULT,
    pub NewState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut XboxLiveEndpointPairState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXboxLiveEndpointPairStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FindEndpointPairBySocketAddressBytes: unsafe extern "system" fn(this: *mut *mut Self, localSocketAddress_array_size: u32, localsocketaddress: *const u8, remoteSocketAddress_array_size: u32, remotesocketaddress: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindEndpointPairByHostNamesAndPorts: unsafe extern "system" fn(this: *mut *mut Self, localhostname: *mut ::core::ffi::c_void, localport: ::windows_sys::core::HSTRING, remotehostname: *mut ::core::ffi::c_void, remoteport: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXboxLiveEndpointPairTemplate {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub InboundEndpointPairCreated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InboundEndpointPairCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInboundEndpointPairCreated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInboundEndpointPairCreated: usize,
    #[cfg(feature = "Foundation")]
    pub CreateEndpointPairDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceaddress: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateEndpointPairDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateEndpointPairWithBehaviorsAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceaddress: *mut ::core::ffi::c_void, behaviors: XboxLiveEndpointPairCreationBehaviors, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateEndpointPairWithBehaviorsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateEndpointPairForPortsDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceaddress: *mut ::core::ffi::c_void, initiatorport: ::windows_sys::core::HSTRING, acceptorport: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateEndpointPairForPortsDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateEndpointPairForPortsWithBehaviorsAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceaddress: *mut ::core::ffi::c_void, initiatorport: ::windows_sys::core::HSTRING, acceptorport: ::windows_sys::core::HSTRING, behaviors: XboxLiveEndpointPairCreationBehaviors, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateEndpointPairForPortsWithBehaviorsAsync: usize,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SocketKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut XboxLiveSocketKind) -> ::windows_sys::core::HRESULT,
    pub InitiatorBoundPortRangeLower: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub InitiatorBoundPortRangeUpper: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub AcceptorBoundPortRangeLower: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub AcceptorBoundPortRangeUpper: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EndpointPairs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EndpointPairs: usize,
}
#[repr(C)]
pub struct IXboxLiveEndpointPairTemplateStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetTemplateByName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Templates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Templates: usize,
}
#[repr(C)]
pub struct IXboxLiveInboundEndpointPairCreatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub EndpointPair: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXboxLiveQualityOfServiceMeasurement {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MeasureAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MeasureAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMetricResultsForDevice: unsafe extern "system" fn(this: *mut *mut Self, deviceaddress: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMetricResultsForDevice: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMetricResultsForMetric: unsafe extern "system" fn(this: *mut *mut Self, metric: XboxLiveQualityOfServiceMetric, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMetricResultsForMetric: usize,
    pub GetMetricResult: unsafe extern "system" fn(this: *mut *mut Self, deviceaddress: *mut ::core::ffi::c_void, metric: XboxLiveQualityOfServiceMetric, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPrivatePayloadResult: unsafe extern "system" fn(this: *mut *mut Self, deviceaddress: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Metrics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metrics: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceAddresses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceAddresses: usize,
    pub ShouldRequestPrivatePayloads: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShouldRequestPrivatePayloads: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TimeoutInMilliseconds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTimeoutInMilliseconds: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub NumberOfProbesToAttempt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetNumberOfProbesToAttempt: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub NumberOfResultsPending: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MetricResults: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MetricResults: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PrivatePayloadResults: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrivatePayloadResults: usize,
}
#[repr(C)]
pub struct IXboxLiveQualityOfServiceMeasurementStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PublishPrivatePayloadBytes: unsafe extern "system" fn(this: *mut *mut Self, payload_array_size: u32, payload: *const u8) -> ::windows_sys::core::HRESULT,
    pub ClearPrivatePayload: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MaxSimultaneousProbeConnections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxSimultaneousProbeConnections: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub IsSystemOutboundBandwidthConstrained: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSystemOutboundBandwidthConstrained: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsSystemInboundBandwidthConstrained: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSystemInboundBandwidthConstrained: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PublishedPrivatePayload: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublishedPrivatePayload: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPublishedPrivatePayload: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPublishedPrivatePayload: usize,
    pub MaxPrivatePayloadSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXboxLiveQualityOfServiceMetricResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut XboxLiveQualityOfServiceMeasurementStatus) -> ::windows_sys::core::HRESULT,
    pub DeviceAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Metric: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut XboxLiveQualityOfServiceMetric) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXboxLiveQualityOfServicePrivatePayloadResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut XboxLiveQualityOfServiceMeasurementStatus) -> ::windows_sys::core::HRESULT,
    pub DeviceAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
pub type XboxLiveDeviceAddress = *mut ::core::ffi::c_void;
pub type XboxLiveEndpointPair = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveEndpointPairCreationBehaviors(pub u32);
impl XboxLiveEndpointPairCreationBehaviors {
    pub const None: Self = Self(0u32);
    pub const ReevaluatePath: Self = Self(1u32);
}
impl ::core::marker::Copy for XboxLiveEndpointPairCreationBehaviors {}
impl ::core::clone::Clone for XboxLiveEndpointPairCreationBehaviors {
    fn clone(&self) -> Self {
        *self
    }
}
pub type XboxLiveEndpointPairCreationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveEndpointPairCreationStatus(pub i32);
impl XboxLiveEndpointPairCreationStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoLocalNetworks: Self = Self(1i32);
    pub const NoCompatibleNetworkPaths: Self = Self(2i32);
    pub const LocalSystemNotAuthorized: Self = Self(3i32);
    pub const Canceled: Self = Self(4i32);
    pub const TimedOut: Self = Self(5i32);
    pub const RemoteSystemNotAuthorized: Self = Self(6i32);
    pub const RefusedDueToConfiguration: Self = Self(7i32);
    pub const UnexpectedInternalError: Self = Self(8i32);
}
impl ::core::marker::Copy for XboxLiveEndpointPairCreationStatus {}
impl ::core::clone::Clone for XboxLiveEndpointPairCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveEndpointPairState(pub i32);
impl XboxLiveEndpointPairState {
    pub const Invalid: Self = Self(0i32);
    pub const CreatingOutbound: Self = Self(1i32);
    pub const CreatingInbound: Self = Self(2i32);
    pub const Ready: Self = Self(3i32);
    pub const DeletingLocally: Self = Self(4i32);
    pub const RemoteEndpointTerminating: Self = Self(5i32);
    pub const Deleted: Self = Self(6i32);
}
impl ::core::marker::Copy for XboxLiveEndpointPairState {}
impl ::core::clone::Clone for XboxLiveEndpointPairState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type XboxLiveEndpointPairStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type XboxLiveEndpointPairTemplate = *mut ::core::ffi::c_void;
pub type XboxLiveInboundEndpointPairCreatedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveNetworkAccessKind(pub i32);
impl XboxLiveNetworkAccessKind {
    pub const Open: Self = Self(0i32);
    pub const Moderate: Self = Self(1i32);
    pub const Strict: Self = Self(2i32);
}
impl ::core::marker::Copy for XboxLiveNetworkAccessKind {}
impl ::core::clone::Clone for XboxLiveNetworkAccessKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type XboxLiveQualityOfServiceMeasurement = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMeasurementStatus(pub i32);
impl XboxLiveQualityOfServiceMeasurementStatus {
    pub const NotStarted: Self = Self(0i32);
    pub const InProgress: Self = Self(1i32);
    pub const InProgressWithProvisionalResults: Self = Self(2i32);
    pub const Succeeded: Self = Self(3i32);
    pub const NoLocalNetworks: Self = Self(4i32);
    pub const NoCompatibleNetworkPaths: Self = Self(5i32);
    pub const LocalSystemNotAuthorized: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const TimedOut: Self = Self(8i32);
    pub const RemoteSystemNotAuthorized: Self = Self(9i32);
    pub const RefusedDueToConfiguration: Self = Self(10i32);
    pub const UnexpectedInternalError: Self = Self(11i32);
}
impl ::core::marker::Copy for XboxLiveQualityOfServiceMeasurementStatus {}
impl ::core::clone::Clone for XboxLiveQualityOfServiceMeasurementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMetric(pub i32);
impl XboxLiveQualityOfServiceMetric {
    pub const AverageLatencyInMilliseconds: Self = Self(0i32);
    pub const MinLatencyInMilliseconds: Self = Self(1i32);
    pub const MaxLatencyInMilliseconds: Self = Self(2i32);
    pub const AverageOutboundBitsPerSecond: Self = Self(3i32);
    pub const MinOutboundBitsPerSecond: Self = Self(4i32);
    pub const MaxOutboundBitsPerSecond: Self = Self(5i32);
    pub const AverageInboundBitsPerSecond: Self = Self(6i32);
    pub const MinInboundBitsPerSecond: Self = Self(7i32);
    pub const MaxInboundBitsPerSecond: Self = Self(8i32);
}
impl ::core::marker::Copy for XboxLiveQualityOfServiceMetric {}
impl ::core::clone::Clone for XboxLiveQualityOfServiceMetric {
    fn clone(&self) -> Self {
        *self
    }
}
pub type XboxLiveQualityOfServiceMetricResult = *mut ::core::ffi::c_void;
pub type XboxLiveQualityOfServicePrivatePayloadResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveSocketKind(pub i32);
impl XboxLiveSocketKind {
    pub const None: Self = Self(0i32);
    pub const Datagram: Self = Self(1i32);
    pub const Stream: Self = Self(2i32);
}
impl ::core::marker::Copy for XboxLiveSocketKind {}
impl ::core::clone::Clone for XboxLiveSocketKind {
    fn clone(&self) -> Self {
        *self
    }
}
