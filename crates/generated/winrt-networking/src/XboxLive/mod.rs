#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveDeviceAddress(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXboxLiveDeviceAddress {
    type Vtable = IXboxLiveDeviceAddress_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5bbd279_3c86_4b57_a31a_b9462408fd01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveDeviceAddress_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SnapshotChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSnapshotChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub GetSnapshotAsBase64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub GetSnapshotAsBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetSnapshotAsBuffer: usize,
    pub GetSnapshotAsBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, byteswritten: *mut u32) -> ::windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, otherdeviceaddress: ::windows_core::RawPtr, result__: *mut i32) -> ::windows_core::HRESULT,
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub NetworkAccessKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveNetworkAccessKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveDeviceAddressStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXboxLiveDeviceAddressStatics {
    type Vtable = IXboxLiveDeviceAddressStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5954a819_4a79_4931_827c_7f503e963263);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveDeviceAddressStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromSnapshotBase64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, base64: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub CreateFromSnapshotBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFromSnapshotBuffer: usize,
    pub CreateFromSnapshotBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MaxSnapshotBytesSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveEndpointPair(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXboxLiveEndpointPair {
    type Vtable = IXboxLiveEndpointPair_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e9a839b_813e_44e0_b87f_c87a093475e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPair_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetRemoteSocketAddressBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketAddress_array_size: u32, socketaddress: *mut u8) -> ::windows_core::HRESULT,
    pub GetLocalSocketAddressBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketAddress_array_size: u32, socketaddress: *mut u8) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairState) -> ::windows_core::HRESULT,
    pub Template: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoteDeviceAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LocalHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveEndpointPairCreationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXboxLiveEndpointPairCreationResult {
    type Vtable = IXboxLiveEndpointPairCreationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9a8bb95_2aab_4d1e_9794_33ecc0dcf0fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairCreationResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairCreationStatus) -> ::windows_core::HRESULT,
    pub IsExistingPathEvaluation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub EndpointPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveEndpointPairStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXboxLiveEndpointPairStateChangedEventArgs {
    type Vtable = IXboxLiveEndpointPairStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x592e3b55_de08_44e7_ac3b_b9b9a169583a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OldState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairState) -> ::windows_core::HRESULT,
    pub NewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveEndpointPairStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXboxLiveEndpointPairStatics {
    type Vtable = IXboxLiveEndpointPairStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64316b30_217a_4243_8ee1_6729281d27db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FindEndpointPairBySocketAddressBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localSocketAddress_array_size: u32, localsocketaddress: *const u8, remoteSocketAddress_array_size: u32, remotesocketaddress: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FindEndpointPairByHostNamesAndPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localhostname: ::windows_core::RawPtr, localport: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, remotehostname: ::windows_core::RawPtr, remoteport: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveEndpointPairTemplate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXboxLiveEndpointPairTemplate {
    type Vtable = IXboxLiveEndpointPairTemplate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b286ecf_3457_40ce_b9a1_c0cfe0213ea7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairTemplate_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InboundEndpointPairCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveInboundEndpointPairCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CreateEndpointPairDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateEndpointPairWithBehaviorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: ::windows_core::RawPtr, behaviors: XboxLiveEndpointPairCreationBehaviors, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateEndpointPairForPortsDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: ::windows_core::RawPtr, initiatorport: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, acceptorport: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateEndpointPairForPortsWithBehaviorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: ::windows_core::RawPtr, initiatorport: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, acceptorport: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, behaviors: XboxLiveEndpointPairCreationBehaviors, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SocketKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveSocketKind) -> ::windows_core::HRESULT,
    pub InitiatorBoundPortRangeLower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub InitiatorBoundPortRangeUpper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub AcceptorBoundPortRangeLower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub AcceptorBoundPortRangeUpper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub EndpointPairs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    EndpointPairs: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveEndpointPairTemplateStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXboxLiveEndpointPairTemplateStatics {
    type Vtable = IXboxLiveEndpointPairTemplateStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e13137b_737b_4a23_bc64_0870f75655ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairTemplateStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetTemplateByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Templates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Templates: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveInboundEndpointPairCreatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXboxLiveInboundEndpointPairCreatedEventArgs {
    type Vtable = IXboxLiveInboundEndpointPairCreatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc183b62_22ba_48d2_80de_c23968bd198b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveInboundEndpointPairCreatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EndpointPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveQualityOfServiceMeasurement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXboxLiveQualityOfServiceMeasurement {
    type Vtable = IXboxLiveQualityOfServiceMeasurement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d682bce_a5d6_47e6_a236_cfde5fbdf2ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServiceMeasurement_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MeasureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetMetricResultsForDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetMetricResultsForDevice: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetMetricResultsForMetric: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metric: XboxLiveQualityOfServiceMetric, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetMetricResultsForMetric: usize,
    pub GetMetricResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: ::windows_core::RawPtr, metric: XboxLiveQualityOfServiceMetric, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPrivatePayloadResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Metrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Metrics: usize,
    #[cfg(feature = "winrt-foundation")]
    pub DeviceAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DeviceAddresses: usize,
    pub ShouldRequestPrivatePayloads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShouldRequestPrivatePayloads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub TimeoutInMilliseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetTimeoutInMilliseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub NumberOfProbesToAttempt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetNumberOfProbesToAttempt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub NumberOfResultsPending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub MetricResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    MetricResults: usize,
    #[cfg(feature = "winrt-foundation")]
    pub PrivatePayloadResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PrivatePayloadResults: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveQualityOfServiceMeasurementStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXboxLiveQualityOfServiceMeasurementStatics {
    type Vtable = IXboxLiveQualityOfServiceMeasurementStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e352dca_23cf_440a_b077_5e30857a8234);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServiceMeasurementStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PublishPrivatePayloadBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, payload_array_size: u32, payload: *const u8) -> ::windows_core::HRESULT,
    pub ClearPrivatePayload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MaxSimultaneousProbeConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxSimultaneousProbeConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub IsSystemOutboundBandwidthConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSystemOutboundBandwidthConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsSystemInboundBandwidthConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSystemInboundBandwidthConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub PublishedPrivatePayload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    PublishedPrivatePayload: usize,
    #[cfg(feature = "winrt-storage")]
    pub SetPublishedPrivatePayload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetPublishedPrivatePayload: usize,
    pub MaxPrivatePayloadSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveQualityOfServiceMetricResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXboxLiveQualityOfServiceMetricResult {
    type Vtable = IXboxLiveQualityOfServiceMetricResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaeec53d1_3561_4782_b0cf_d3ae29d9fa87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServiceMetricResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveQualityOfServiceMeasurementStatus) -> ::windows_core::HRESULT,
    pub DeviceAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Metric: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveQualityOfServiceMetric) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveQualityOfServicePrivatePayloadResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXboxLiveQualityOfServicePrivatePayloadResult {
    type Vtable = IXboxLiveQualityOfServicePrivatePayloadResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a6302ae_6f38_41c0_9fcc_ea6cb978cafc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServicePrivatePayloadResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveQualityOfServiceMeasurementStatus) -> ::windows_core::HRESULT,
    pub DeviceAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Value: usize,
}
#[repr(transparent)]
pub struct XboxLiveDeviceAddress(::windows_core::IUnknown);
impl XboxLiveDeviceAddress {
    pub fn SnapshotChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<XboxLiveDeviceAddress, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SnapshotChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSnapshotChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSnapshotChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetSnapshotAsBase64(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetSnapshotAsBase64)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetSnapshotAsBuffer(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSnapshotAsBuffer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn GetSnapshotAsBytes(&self, buffer: &mut [u8], byteswritten: &mut u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetSnapshotAsBytes)(::windows_core::Interface::as_raw(this), buffer.len() as u32, ::core::mem::transmute_copy(&buffer), byteswritten).ok() }
    }
    pub fn Compare<'a, Param0: ::windows_core::IntoParam<'a, XboxLiveDeviceAddress>>(&self, otherdeviceaddress: Param0) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Compare)(::windows_core::Interface::as_raw(this), otherdeviceaddress.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn IsValid(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsValid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsLocal(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLocal)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn NetworkAccessKind(&self) -> ::windows_core::Result<XboxLiveNetworkAccessKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<XboxLiveNetworkAccessKind>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAccessKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveNetworkAccessKind>(result__)
        }
    }
    pub fn CreateFromSnapshotBase64<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(base64: Param0) -> ::windows_core::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromSnapshotBase64)(::windows_core::Interface::as_raw(this), base64.into_param().abi(), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFromSnapshotBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(buffer: Param0) -> ::windows_core::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromSnapshotBuffer)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        })
    }
    pub fn CreateFromSnapshotBytes(buffer: &[u8]) -> ::windows_core::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromSnapshotBytes)(::windows_core::Interface::as_raw(this), buffer.len() as u32, ::core::mem::transmute(buffer.as_ptr()), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        })
    }
    pub fn GetLocal() -> ::windows_core::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetLocal)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        })
    }
    pub fn MaxSnapshotBytesSize() -> ::windows_core::Result<u32> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxSnapshotBytesSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn IXboxLiveDeviceAddressStatics<R, F: FnOnce(&IXboxLiveDeviceAddressStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XboxLiveDeviceAddress, IXboxLiveDeviceAddressStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XboxLiveDeviceAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveDeviceAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveDeviceAddress {}
impl ::core::fmt::Debug for XboxLiveDeviceAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveDeviceAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveDeviceAddress {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveDeviceAddress;{f5bbd279-3c86-4b57-a31a-b9462408fd01})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XboxLiveDeviceAddress {
    type Vtable = IXboxLiveDeviceAddress_Vtbl;
    const IID: ::windows_core::GUID = <IXboxLiveDeviceAddress as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XboxLiveDeviceAddress {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveDeviceAddress";
}
impl ::core::convert::From<XboxLiveDeviceAddress> for ::windows_core::IUnknown {
    fn from(value: XboxLiveDeviceAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveDeviceAddress> for ::windows_core::IUnknown {
    fn from(value: &XboxLiveDeviceAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XboxLiveDeviceAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XboxLiveDeviceAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XboxLiveDeviceAddress> for ::windows_core::IInspectable {
    fn from(value: XboxLiveDeviceAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveDeviceAddress> for ::windows_core::IInspectable {
    fn from(value: &XboxLiveDeviceAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XboxLiveDeviceAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XboxLiveDeviceAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XboxLiveDeviceAddress {}
unsafe impl ::core::marker::Sync for XboxLiveDeviceAddress {}
#[repr(transparent)]
pub struct XboxLiveEndpointPair(::windows_core::IUnknown);
impl XboxLiveEndpointPair {
    pub fn StateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<XboxLiveEndpointPair, XboxLiveEndpointPairStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DeleteAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetRemoteSocketAddressBytes(&self, socketaddress: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetRemoteSocketAddressBytes)(::windows_core::Interface::as_raw(this), socketaddress.len() as u32, ::core::mem::transmute_copy(&socketaddress)).ok() }
    }
    pub fn GetLocalSocketAddressBytes(&self, socketaddress: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetLocalSocketAddressBytes)(::windows_core::Interface::as_raw(this), socketaddress.len() as u32, ::core::mem::transmute_copy(&socketaddress)).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<XboxLiveEndpointPairState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<XboxLiveEndpointPairState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPairState>(result__)
        }
    }
    pub fn Template(&self) -> ::windows_core::Result<XboxLiveEndpointPairTemplate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Template)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPairTemplate>(result__)
        }
    }
    pub fn RemoteDeviceAddress(&self) -> ::windows_core::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteDeviceAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        }
    }
    pub fn RemoteHostName(&self) -> ::windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteHostName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::HostName>(result__)
        }
    }
    pub fn RemotePort(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RemotePort)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn LocalHostName(&self) -> ::windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LocalHostName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::HostName>(result__)
        }
    }
    pub fn LocalPort(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LocalPort)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FindEndpointPairBySocketAddressBytes(localsocketaddress: &[u8], remotesocketaddress: &[u8]) -> ::windows_core::Result<XboxLiveEndpointPair> {
        Self::IXboxLiveEndpointPairStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindEndpointPairBySocketAddressBytes)(::windows_core::Interface::as_raw(this), localsocketaddress.len() as u32, ::core::mem::transmute(localsocketaddress.as_ptr()), remotesocketaddress.len() as u32, ::core::mem::transmute(remotesocketaddress.as_ptr()), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPair>(result__)
        })
    }
    pub fn FindEndpointPairByHostNamesAndPorts<'a, Param0: ::windows_core::IntoParam<'a, super::HostName>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, super::HostName>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(localhostname: Param0, localport: Param1, remotehostname: Param2, remoteport: Param3) -> ::windows_core::Result<XboxLiveEndpointPair> {
        Self::IXboxLiveEndpointPairStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindEndpointPairByHostNamesAndPorts)(::windows_core::Interface::as_raw(this), localhostname.into_param().abi(), localport.into_param().abi(), remotehostname.into_param().abi(), remoteport.into_param().abi(), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPair>(result__)
        })
    }
    pub fn IXboxLiveEndpointPairStatics<R, F: FnOnce(&IXboxLiveEndpointPairStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XboxLiveEndpointPair, IXboxLiveEndpointPairStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XboxLiveEndpointPair {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveEndpointPair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveEndpointPair {}
impl ::core::fmt::Debug for XboxLiveEndpointPair {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPair").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveEndpointPair {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveEndpointPair;{1e9a839b-813e-44e0-b87f-c87a093475e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XboxLiveEndpointPair {
    type Vtable = IXboxLiveEndpointPair_Vtbl;
    const IID: ::windows_core::GUID = <IXboxLiveEndpointPair as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XboxLiveEndpointPair {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPair";
}
impl ::core::convert::From<XboxLiveEndpointPair> for ::windows_core::IUnknown {
    fn from(value: XboxLiveEndpointPair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPair> for ::windows_core::IUnknown {
    fn from(value: &XboxLiveEndpointPair) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XboxLiveEndpointPair {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XboxLiveEndpointPair {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XboxLiveEndpointPair> for ::windows_core::IInspectable {
    fn from(value: XboxLiveEndpointPair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPair> for ::windows_core::IInspectable {
    fn from(value: &XboxLiveEndpointPair) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XboxLiveEndpointPair {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XboxLiveEndpointPair {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XboxLiveEndpointPair {}
unsafe impl ::core::marker::Sync for XboxLiveEndpointPair {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for XboxLiveEndpointPairCreationBehaviors {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XboxLiveEndpointPairCreationBehaviors {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveEndpointPairCreationBehaviors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairCreationBehaviors").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for XboxLiveEndpointPairCreationBehaviors {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for XboxLiveEndpointPairCreationBehaviors {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for XboxLiveEndpointPairCreationBehaviors {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for XboxLiveEndpointPairCreationBehaviors {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for XboxLiveEndpointPairCreationBehaviors {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveEndpointPairCreationBehaviors {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveEndpointPairCreationBehaviors;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct XboxLiveEndpointPairCreationResult(::windows_core::IUnknown);
impl XboxLiveEndpointPairCreationResult {
    pub fn DeviceAddress(&self) -> ::windows_core::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<XboxLiveEndpointPairCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<XboxLiveEndpointPairCreationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPairCreationStatus>(result__)
        }
    }
    pub fn IsExistingPathEvaluation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsExistingPathEvaluation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn EndpointPair(&self) -> ::windows_core::Result<XboxLiveEndpointPair> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EndpointPair)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPair>(result__)
        }
    }
}
impl ::core::clone::Clone for XboxLiveEndpointPairCreationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveEndpointPairCreationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveEndpointPairCreationResult {}
impl ::core::fmt::Debug for XboxLiveEndpointPairCreationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairCreationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveEndpointPairCreationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult;{d9a8bb95-2aab-4d1e-9794-33ecc0dcf0fe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XboxLiveEndpointPairCreationResult {
    type Vtable = IXboxLiveEndpointPairCreationResult_Vtbl;
    const IID: ::windows_core::GUID = <IXboxLiveEndpointPairCreationResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XboxLiveEndpointPairCreationResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult";
}
impl ::core::convert::From<XboxLiveEndpointPairCreationResult> for ::windows_core::IUnknown {
    fn from(value: XboxLiveEndpointPairCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairCreationResult> for ::windows_core::IUnknown {
    fn from(value: &XboxLiveEndpointPairCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XboxLiveEndpointPairCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XboxLiveEndpointPairCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XboxLiveEndpointPairCreationResult> for ::windows_core::IInspectable {
    fn from(value: XboxLiveEndpointPairCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairCreationResult> for ::windows_core::IInspectable {
    fn from(value: &XboxLiveEndpointPairCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XboxLiveEndpointPairCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XboxLiveEndpointPairCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XboxLiveEndpointPairCreationResult {}
unsafe impl ::core::marker::Sync for XboxLiveEndpointPairCreationResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for XboxLiveEndpointPairCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XboxLiveEndpointPairCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveEndpointPairCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveEndpointPairCreationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveEndpointPairCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for XboxLiveEndpointPairState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XboxLiveEndpointPairState {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveEndpointPairState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveEndpointPairState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveEndpointPairState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct XboxLiveEndpointPairStateChangedEventArgs(::windows_core::IUnknown);
impl XboxLiveEndpointPairStateChangedEventArgs {
    pub fn OldState(&self) -> ::windows_core::Result<XboxLiveEndpointPairState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<XboxLiveEndpointPairState>::zeroed();
            (::windows_core::Interface::vtable(this).OldState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPairState>(result__)
        }
    }
    pub fn NewState(&self) -> ::windows_core::Result<XboxLiveEndpointPairState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<XboxLiveEndpointPairState>::zeroed();
            (::windows_core::Interface::vtable(this).NewState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPairState>(result__)
        }
    }
}
impl ::core::clone::Clone for XboxLiveEndpointPairStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveEndpointPairStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveEndpointPairStateChangedEventArgs {}
impl ::core::fmt::Debug for XboxLiveEndpointPairStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveEndpointPairStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs;{592e3b55-de08-44e7-ac3b-b9b9a169583a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XboxLiveEndpointPairStateChangedEventArgs {
    type Vtable = IXboxLiveEndpointPairStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IXboxLiveEndpointPairStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XboxLiveEndpointPairStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs";
}
impl ::core::convert::From<XboxLiveEndpointPairStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XboxLiveEndpointPairStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XboxLiveEndpointPairStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XboxLiveEndpointPairStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XboxLiveEndpointPairStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XboxLiveEndpointPairStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XboxLiveEndpointPairStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for XboxLiveEndpointPairStateChangedEventArgs {}
#[repr(transparent)]
pub struct XboxLiveEndpointPairTemplate(::windows_core::IUnknown);
impl XboxLiveEndpointPairTemplate {
    pub fn InboundEndpointPairCreated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<XboxLiveEndpointPairTemplate, XboxLiveInboundEndpointPairCreatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).InboundEndpointPairCreated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveInboundEndpointPairCreated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveInboundEndpointPairCreated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CreateEndpointPairDefaultAsync<'a, Param0: ::windows_core::IntoParam<'a, XboxLiveDeviceAddress>>(&self, deviceaddress: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateEndpointPairDefaultAsync)(::windows_core::Interface::as_raw(this), deviceaddress.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>(result__)
        }
    }
    pub fn CreateEndpointPairWithBehaviorsAsync<'a, Param0: ::windows_core::IntoParam<'a, XboxLiveDeviceAddress>>(&self, deviceaddress: Param0, behaviors: XboxLiveEndpointPairCreationBehaviors) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateEndpointPairWithBehaviorsAsync)(::windows_core::Interface::as_raw(this), deviceaddress.into_param().abi(), behaviors, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>(result__)
        }
    }
    pub fn CreateEndpointPairForPortsDefaultAsync<'a, Param0: ::windows_core::IntoParam<'a, XboxLiveDeviceAddress>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, deviceaddress: Param0, initiatorport: Param1, acceptorport: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateEndpointPairForPortsDefaultAsync)(::windows_core::Interface::as_raw(this), deviceaddress.into_param().abi(), initiatorport.into_param().abi(), acceptorport.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>(result__)
        }
    }
    pub fn CreateEndpointPairForPortsWithBehaviorsAsync<'a, Param0: ::windows_core::IntoParam<'a, XboxLiveDeviceAddress>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, deviceaddress: Param0, initiatorport: Param1, acceptorport: Param2, behaviors: XboxLiveEndpointPairCreationBehaviors) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateEndpointPairForPortsWithBehaviorsAsync)(::windows_core::Interface::as_raw(this), deviceaddress.into_param().abi(), initiatorport.into_param().abi(), acceptorport.into_param().abi(), behaviors, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SocketKind(&self) -> ::windows_core::Result<XboxLiveSocketKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<XboxLiveSocketKind>::zeroed();
            (::windows_core::Interface::vtable(this).SocketKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveSocketKind>(result__)
        }
    }
    pub fn InitiatorBoundPortRangeLower(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).InitiatorBoundPortRangeLower)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn InitiatorBoundPortRangeUpper(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).InitiatorBoundPortRangeUpper)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn AcceptorBoundPortRangeLower(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).AcceptorBoundPortRangeLower)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn AcceptorBoundPortRangeUpper(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).AcceptorBoundPortRangeUpper)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn EndpointPairs(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<XboxLiveEndpointPair>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EndpointPairs)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<XboxLiveEndpointPair>>(result__)
        }
    }
    pub fn GetTemplateByName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<XboxLiveEndpointPairTemplate> {
        Self::IXboxLiveEndpointPairTemplateStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTemplateByName)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPairTemplate>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Templates() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<XboxLiveEndpointPairTemplate>> {
        Self::IXboxLiveEndpointPairTemplateStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Templates)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<XboxLiveEndpointPairTemplate>>(result__)
        })
    }
    pub fn IXboxLiveEndpointPairTemplateStatics<R, F: FnOnce(&IXboxLiveEndpointPairTemplateStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XboxLiveEndpointPairTemplate, IXboxLiveEndpointPairTemplateStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XboxLiveEndpointPairTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveEndpointPairTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveEndpointPairTemplate {}
impl ::core::fmt::Debug for XboxLiveEndpointPairTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairTemplate").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveEndpointPairTemplate {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate;{6b286ecf-3457-40ce-b9a1-c0cfe0213ea7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XboxLiveEndpointPairTemplate {
    type Vtable = IXboxLiveEndpointPairTemplate_Vtbl;
    const IID: ::windows_core::GUID = <IXboxLiveEndpointPairTemplate as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XboxLiveEndpointPairTemplate {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate";
}
impl ::core::convert::From<XboxLiveEndpointPairTemplate> for ::windows_core::IUnknown {
    fn from(value: XboxLiveEndpointPairTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairTemplate> for ::windows_core::IUnknown {
    fn from(value: &XboxLiveEndpointPairTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XboxLiveEndpointPairTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XboxLiveEndpointPairTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XboxLiveEndpointPairTemplate> for ::windows_core::IInspectable {
    fn from(value: XboxLiveEndpointPairTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairTemplate> for ::windows_core::IInspectable {
    fn from(value: &XboxLiveEndpointPairTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XboxLiveEndpointPairTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XboxLiveEndpointPairTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XboxLiveEndpointPairTemplate {}
unsafe impl ::core::marker::Sync for XboxLiveEndpointPairTemplate {}
#[repr(transparent)]
pub struct XboxLiveInboundEndpointPairCreatedEventArgs(::windows_core::IUnknown);
impl XboxLiveInboundEndpointPairCreatedEventArgs {
    pub fn EndpointPair(&self) -> ::windows_core::Result<XboxLiveEndpointPair> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EndpointPair)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPair>(result__)
        }
    }
}
impl ::core::clone::Clone for XboxLiveInboundEndpointPairCreatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveInboundEndpointPairCreatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveInboundEndpointPairCreatedEventArgs {}
impl ::core::fmt::Debug for XboxLiveInboundEndpointPairCreatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveInboundEndpointPairCreatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveInboundEndpointPairCreatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs;{dc183b62-22ba-48d2-80de-c23968bd198b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XboxLiveInboundEndpointPairCreatedEventArgs {
    type Vtable = IXboxLiveInboundEndpointPairCreatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IXboxLiveInboundEndpointPairCreatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XboxLiveInboundEndpointPairCreatedEventArgs {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs";
}
impl ::core::convert::From<XboxLiveInboundEndpointPairCreatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveInboundEndpointPairCreatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XboxLiveInboundEndpointPairCreatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XboxLiveInboundEndpointPairCreatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XboxLiveInboundEndpointPairCreatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveInboundEndpointPairCreatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XboxLiveInboundEndpointPairCreatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XboxLiveInboundEndpointPairCreatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XboxLiveInboundEndpointPairCreatedEventArgs {}
unsafe impl ::core::marker::Sync for XboxLiveInboundEndpointPairCreatedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for XboxLiveNetworkAccessKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XboxLiveNetworkAccessKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveNetworkAccessKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveNetworkAccessKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveNetworkAccessKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveNetworkAccessKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMeasurement(::windows_core::IUnknown);
impl XboxLiveQualityOfServiceMeasurement {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XboxLiveQualityOfServiceMeasurement, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn MeasureAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MeasureAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMetricResultsForDevice<'a, Param0: ::windows_core::IntoParam<'a, XboxLiveDeviceAddress>>(&self, deviceaddress: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMetricResultsForDevice)(::windows_core::Interface::as_raw(this), deviceaddress.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMetricResultsForMetric(&self, metric: XboxLiveQualityOfServiceMetric) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMetricResultsForMetric)(::windows_core::Interface::as_raw(this), metric, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>(result__)
        }
    }
    pub fn GetMetricResult<'a, Param0: ::windows_core::IntoParam<'a, XboxLiveDeviceAddress>>(&self, deviceaddress: Param0, metric: XboxLiveQualityOfServiceMetric) -> ::windows_core::Result<XboxLiveQualityOfServiceMetricResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMetricResult)(::windows_core::Interface::as_raw(this), deviceaddress.into_param().abi(), metric, result__.as_mut_ptr()).from_abi::<XboxLiveQualityOfServiceMetricResult>(result__)
        }
    }
    pub fn GetPrivatePayloadResult<'a, Param0: ::windows_core::IntoParam<'a, XboxLiveDeviceAddress>>(&self, deviceaddress: Param0) -> ::windows_core::Result<XboxLiveQualityOfServicePrivatePayloadResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPrivatePayloadResult)(::windows_core::Interface::as_raw(this), deviceaddress.into_param().abi(), result__.as_mut_ptr()).from_abi::<XboxLiveQualityOfServicePrivatePayloadResult>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Metrics(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<XboxLiveQualityOfServiceMetric>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Metrics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<XboxLiveQualityOfServiceMetric>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DeviceAddresses(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<XboxLiveDeviceAddress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceAddresses)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<XboxLiveDeviceAddress>>(result__)
        }
    }
    pub fn ShouldRequestPrivatePayloads(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShouldRequestPrivatePayloads)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldRequestPrivatePayloads(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShouldRequestPrivatePayloads)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TimeoutInMilliseconds(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TimeoutInMilliseconds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetTimeoutInMilliseconds(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimeoutInMilliseconds)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumberOfProbesToAttempt(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).NumberOfProbesToAttempt)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetNumberOfProbesToAttempt(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNumberOfProbesToAttempt)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumberOfResultsPending(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).NumberOfResultsPending)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn MetricResults(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MetricResults)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PrivatePayloadResults(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<XboxLiveQualityOfServicePrivatePayloadResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrivatePayloadResults)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<XboxLiveQualityOfServicePrivatePayloadResult>>(result__)
        }
    }
    pub fn PublishPrivatePayloadBytes(payload: &[u8]) -> ::windows_core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows_core::Interface::vtable(this).PublishPrivatePayloadBytes)(::windows_core::Interface::as_raw(this), payload.len() as u32, ::core::mem::transmute(payload.as_ptr())).ok() })
    }
    pub fn ClearPrivatePayload() -> ::windows_core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ClearPrivatePayload)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn MaxSimultaneousProbeConnections() -> ::windows_core::Result<u32> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxSimultaneousProbeConnections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn SetMaxSimultaneousProbeConnections(value: u32) -> ::windows_core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetMaxSimultaneousProbeConnections)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn IsSystemOutboundBandwidthConstrained() -> ::windows_core::Result<bool> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSystemOutboundBandwidthConstrained)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsSystemOutboundBandwidthConstrained(value: bool) -> ::windows_core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsSystemOutboundBandwidthConstrained)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn IsSystemInboundBandwidthConstrained() -> ::windows_core::Result<bool> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSystemInboundBandwidthConstrained)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsSystemInboundBandwidthConstrained(value: bool) -> ::windows_core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsSystemInboundBandwidthConstrained)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn PublishedPrivatePayload() -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PublishedPrivatePayload)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetPublishedPrivatePayload<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(value: Param0) -> ::windows_core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetPublishedPrivatePayload)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    pub fn MaxPrivatePayloadSize() -> ::windows_core::Result<u32> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxPrivatePayloadSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn IXboxLiveQualityOfServiceMeasurementStatics<R, F: FnOnce(&IXboxLiveQualityOfServiceMeasurementStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XboxLiveQualityOfServiceMeasurement, IXboxLiveQualityOfServiceMeasurementStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XboxLiveQualityOfServiceMeasurement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveQualityOfServiceMeasurement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveQualityOfServiceMeasurement {}
impl ::core::fmt::Debug for XboxLiveQualityOfServiceMeasurement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveQualityOfServiceMeasurement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveQualityOfServiceMeasurement {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement;{4d682bce-a5d6-47e6-a236-cfde5fbdf2ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XboxLiveQualityOfServiceMeasurement {
    type Vtable = IXboxLiveQualityOfServiceMeasurement_Vtbl;
    const IID: ::windows_core::GUID = <IXboxLiveQualityOfServiceMeasurement as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XboxLiveQualityOfServiceMeasurement {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement";
}
impl ::core::convert::From<XboxLiveQualityOfServiceMeasurement> for ::windows_core::IUnknown {
    fn from(value: XboxLiveQualityOfServiceMeasurement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMeasurement> for ::windows_core::IUnknown {
    fn from(value: &XboxLiveQualityOfServiceMeasurement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XboxLiveQualityOfServiceMeasurement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XboxLiveQualityOfServiceMeasurement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XboxLiveQualityOfServiceMeasurement> for ::windows_core::IInspectable {
    fn from(value: XboxLiveQualityOfServiceMeasurement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMeasurement> for ::windows_core::IInspectable {
    fn from(value: &XboxLiveQualityOfServiceMeasurement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XboxLiveQualityOfServiceMeasurement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XboxLiveQualityOfServiceMeasurement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XboxLiveQualityOfServiceMeasurement {}
unsafe impl ::core::marker::Sync for XboxLiveQualityOfServiceMeasurement {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for XboxLiveQualityOfServiceMeasurementStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XboxLiveQualityOfServiceMeasurementStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveQualityOfServiceMeasurementStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveQualityOfServiceMeasurementStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveQualityOfServiceMeasurementStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurementStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for XboxLiveQualityOfServiceMetric {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XboxLiveQualityOfServiceMetric {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveQualityOfServiceMetric {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveQualityOfServiceMetric").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveQualityOfServiceMetric {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetric;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMetricResult(::windows_core::IUnknown);
impl XboxLiveQualityOfServiceMetricResult {
    pub fn Status(&self) -> ::windows_core::Result<XboxLiveQualityOfServiceMeasurementStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<XboxLiveQualityOfServiceMeasurementStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveQualityOfServiceMeasurementStatus>(result__)
        }
    }
    pub fn DeviceAddress(&self) -> ::windows_core::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        }
    }
    pub fn Metric(&self) -> ::windows_core::Result<XboxLiveQualityOfServiceMetric> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<XboxLiveQualityOfServiceMetric>::zeroed();
            (::windows_core::Interface::vtable(this).Metric)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveQualityOfServiceMetric>(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for XboxLiveQualityOfServiceMetricResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveQualityOfServiceMetricResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveQualityOfServiceMetricResult {}
impl ::core::fmt::Debug for XboxLiveQualityOfServiceMetricResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveQualityOfServiceMetricResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveQualityOfServiceMetricResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult;{aeec53d1-3561-4782-b0cf-d3ae29d9fa87})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XboxLiveQualityOfServiceMetricResult {
    type Vtable = IXboxLiveQualityOfServiceMetricResult_Vtbl;
    const IID: ::windows_core::GUID = <IXboxLiveQualityOfServiceMetricResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XboxLiveQualityOfServiceMetricResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult";
}
impl ::core::convert::From<XboxLiveQualityOfServiceMetricResult> for ::windows_core::IUnknown {
    fn from(value: XboxLiveQualityOfServiceMetricResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMetricResult> for ::windows_core::IUnknown {
    fn from(value: &XboxLiveQualityOfServiceMetricResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XboxLiveQualityOfServiceMetricResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XboxLiveQualityOfServiceMetricResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XboxLiveQualityOfServiceMetricResult> for ::windows_core::IInspectable {
    fn from(value: XboxLiveQualityOfServiceMetricResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMetricResult> for ::windows_core::IInspectable {
    fn from(value: &XboxLiveQualityOfServiceMetricResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XboxLiveQualityOfServiceMetricResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XboxLiveQualityOfServiceMetricResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XboxLiveQualityOfServiceMetricResult {}
unsafe impl ::core::marker::Sync for XboxLiveQualityOfServiceMetricResult {}
#[repr(transparent)]
pub struct XboxLiveQualityOfServicePrivatePayloadResult(::windows_core::IUnknown);
impl XboxLiveQualityOfServicePrivatePayloadResult {
    pub fn Status(&self) -> ::windows_core::Result<XboxLiveQualityOfServiceMeasurementStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<XboxLiveQualityOfServiceMeasurementStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveQualityOfServiceMeasurementStatus>(result__)
        }
    }
    pub fn DeviceAddress(&self) -> ::windows_core::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Value(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for XboxLiveQualityOfServicePrivatePayloadResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveQualityOfServicePrivatePayloadResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveQualityOfServicePrivatePayloadResult {}
impl ::core::fmt::Debug for XboxLiveQualityOfServicePrivatePayloadResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveQualityOfServicePrivatePayloadResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveQualityOfServicePrivatePayloadResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult;{5a6302ae-6f38-41c0-9fcc-ea6cb978cafc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XboxLiveQualityOfServicePrivatePayloadResult {
    type Vtable = IXboxLiveQualityOfServicePrivatePayloadResult_Vtbl;
    const IID: ::windows_core::GUID = <IXboxLiveQualityOfServicePrivatePayloadResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XboxLiveQualityOfServicePrivatePayloadResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult";
}
impl ::core::convert::From<XboxLiveQualityOfServicePrivatePayloadResult> for ::windows_core::IUnknown {
    fn from(value: XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServicePrivatePayloadResult> for ::windows_core::IUnknown {
    fn from(value: &XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XboxLiveQualityOfServicePrivatePayloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XboxLiveQualityOfServicePrivatePayloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XboxLiveQualityOfServicePrivatePayloadResult> for ::windows_core::IInspectable {
    fn from(value: XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServicePrivatePayloadResult> for ::windows_core::IInspectable {
    fn from(value: &XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XboxLiveQualityOfServicePrivatePayloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XboxLiveQualityOfServicePrivatePayloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XboxLiveQualityOfServicePrivatePayloadResult {}
unsafe impl ::core::marker::Sync for XboxLiveQualityOfServicePrivatePayloadResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for XboxLiveSocketKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XboxLiveSocketKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveSocketKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveSocketKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XboxLiveSocketKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveSocketKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
