pub type AttributedNetworkUsage = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct CellularApnAuthenticationType(pub i32);
impl CellularApnAuthenticationType {
    pub const None: Self = Self(0i32);
    pub const Pap: Self = Self(1i32);
    pub const Chap: Self = Self(2i32);
    pub const Mschapv2: Self = Self(3i32);
}
impl ::core::marker::Copy for CellularApnAuthenticationType {}
impl ::core::clone::Clone for CellularApnAuthenticationType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CellularApnContext = *mut ::core::ffi::c_void;
pub type ConnectionCost = *mut ::core::ffi::c_void;
pub type ConnectionProfile = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct ConnectionProfileDeleteStatus(pub i32);
impl ConnectionProfileDeleteStatus {
    pub const Success: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for ConnectionProfileDeleteStatus {}
impl ::core::clone::Clone for ConnectionProfileDeleteStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ConnectionProfileFilter = *mut ::core::ffi::c_void;
pub type ConnectionSession = *mut ::core::ffi::c_void;
pub type ConnectivityInterval = *mut ::core::ffi::c_void;
pub type DataPlanStatus = *mut ::core::ffi::c_void;
pub type DataPlanUsage = *mut ::core::ffi::c_void;
pub type DataUsage = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct DataUsageGranularity(pub i32);
impl DataUsageGranularity {
    pub const PerMinute: Self = Self(0i32);
    pub const PerHour: Self = Self(1i32);
    pub const PerDay: Self = Self(2i32);
    pub const Total: Self = Self(3i32);
}
impl ::core::marker::Copy for DataUsageGranularity {}
impl ::core::clone::Clone for DataUsageGranularity {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct DomainConnectivityLevel(pub i32);
impl DomainConnectivityLevel {
    pub const None: Self = Self(0i32);
    pub const Unauthenticated: Self = Self(1i32);
    pub const Authenticated: Self = Self(2i32);
}
impl ::core::marker::Copy for DomainConnectivityLevel {}
impl ::core::clone::Clone for DomainConnectivityLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAttributedNetworkUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub BytesSent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub AttributionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AttributionName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AttributionThumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AttributionThumbnail: usize,
}
#[repr(C)]
pub struct ICellularApnContext {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetProviderId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AccessPointName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAccessPointName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsCompressionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCompressionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CellularApnAuthenticationType) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(this: *mut *mut Self, value: CellularApnAuthenticationType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICellularApnContext2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProfileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetProfileName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConnectionCost {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkCostType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkCostType) -> ::windows_sys::core::HRESULT,
    pub Roaming: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub OverDataLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ApproachingDataLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConnectionCost2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundDataUsageRestricted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConnectionProfile {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProfileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetNetworkConnectivityLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkConnectivityLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNetworkNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNetworkNames: usize,
    pub GetConnectionCost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDataPlanStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetLocalUsage: unsafe extern "system" fn(this: *mut *mut Self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetLocalUsage: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetLocalUsagePerRoamingStates: unsafe extern "system" fn(this: *mut *mut Self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: RoamingStates, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetLocalUsagePerRoamingStates: usize,
    pub NetworkSecuritySettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConnectionProfile2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsWwanConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsWlanConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub WwanConnectionProfileDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WlanConnectionProfileDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ServiceProviderGuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceProviderGuid: usize,
    #[cfg(feature = "Foundation")]
    pub GetSignalBars: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSignalBars: usize,
    pub GetDomainConnectivityLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DomainConnectivityLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNetworkUsageAsync: unsafe extern "system" fn(this: *mut *mut Self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, granularity: DataUsageGranularity, states: NetworkUsageStates, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNetworkUsageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConnectivityIntervalsAsync: unsafe extern "system" fn(this: *mut *mut Self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConnectivityIntervalsAsync: usize,
}
#[repr(C)]
pub struct IConnectionProfile3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAttributedNetworkUsageAsync: unsafe extern "system" fn(this: *mut *mut Self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAttributedNetworkUsageAsync: usize,
}
#[repr(C)]
pub struct IConnectionProfile4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetProviderNetworkUsageAsync: unsafe extern "system" fn(this: *mut *mut Self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetProviderNetworkUsageAsync: usize,
}
#[repr(C)]
pub struct IConnectionProfile5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanDelete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryDeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteAsync: usize,
}
#[repr(C)]
pub struct IConnectionProfileFilter {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsConnected: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsWwanConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsWwanConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsWlanConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsWlanConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetNetworkCostType: unsafe extern "system" fn(this: *mut *mut Self, value: NetworkCostType) -> ::windows_sys::core::HRESULT,
    pub NetworkCostType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkCostType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetServiceProviderGuid: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetServiceProviderGuid: usize,
    #[cfg(feature = "Foundation")]
    pub ServiceProviderGuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceProviderGuid: usize,
}
#[repr(C)]
pub struct IConnectionProfileFilter2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetIsRoaming: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsRoaming: usize,
    #[cfg(feature = "Foundation")]
    pub IsRoaming: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsRoaming: usize,
    #[cfg(feature = "Foundation")]
    pub SetIsOverDataLimit: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsOverDataLimit: usize,
    #[cfg(feature = "Foundation")]
    pub IsOverDataLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsOverDataLimit: usize,
    #[cfg(feature = "Foundation")]
    pub SetIsBackgroundDataUsageRestricted: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsBackgroundDataUsageRestricted: usize,
    #[cfg(feature = "Foundation")]
    pub IsBackgroundDataUsageRestricted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsBackgroundDataUsageRestricted: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RawData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RawData: usize,
}
#[repr(C)]
pub struct IConnectionProfileFilter3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetPurposeGuid: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPurposeGuid: usize,
    #[cfg(feature = "Foundation")]
    pub PurposeGuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PurposeGuid: usize,
}
#[repr(C)]
pub struct IConnectionSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConnectivityInterval {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionDuration: usize,
}
#[repr(C)]
pub struct IConnectivityManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AcquireConnectionAsync: unsafe extern "system" fn(this: *mut *mut Self, cellularapncontext: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcquireConnectionAsync: usize,
    pub AddHttpRoutePolicy: unsafe extern "system" fn(this: *mut *mut Self, routepolicy: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveHttpRoutePolicy: unsafe extern "system" fn(this: *mut *mut Self, routepolicy: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataPlanStatus {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataPlanUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DataLimitInMegabytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataLimitInMegabytes: usize,
    #[cfg(feature = "Foundation")]
    pub InboundBitsPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InboundBitsPerSecond: usize,
    #[cfg(feature = "Foundation")]
    pub OutboundBitsPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OutboundBitsPerSecond: usize,
    #[cfg(feature = "Foundation")]
    pub NextBillingCycle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NextBillingCycle: usize,
    #[cfg(feature = "Foundation")]
    pub MaxTransferSizeInMegabytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxTransferSizeInMegabytes: usize,
}
#[repr(C)]
pub struct IDataPlanUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub MegabytesUsed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSyncTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSyncTime: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IDataUsage {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub BytesSent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BytesSent: usize,
    #[cfg(feature = "deprecated")]
    pub BytesReceived: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BytesReceived: usize,
}
#[repr(C)]
pub struct IIPInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PrefixLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrefixLength: usize,
}
#[repr(C)]
pub struct ILanIdentifier {
    pub base__: ::windows_sys::core::IInspectable,
    pub InfrastructureId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PortId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NetworkAdapterId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILanIdentifierData {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Value: usize,
}
#[repr(C)]
pub struct INetworkAdapter {
    pub base__: ::windows_sys::core::IInspectable,
    pub OutboundMaxBitsPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub InboundMaxBitsPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub IanaInterfaceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub NetworkItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NetworkAdapterId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetConnectedProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConnectedProfileAsync: usize,
}
#[repr(C)]
pub struct INetworkInformationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConnectionProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConnectionProfiles: usize,
    pub GetInternetConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetLanIdentifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetLanIdentifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetHostNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetHostNames: usize,
    #[cfg(feature = "Foundation")]
    pub GetProxyConfigurationAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetProxyConfigurationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSortedEndpointPairs: unsafe extern "system" fn(this: *mut *mut Self, destinationlist: *mut ::core::ffi::c_void, sortoptions: super::HostNameSortOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSortedEndpointPairs: usize,
    #[cfg(feature = "Foundation")]
    pub NetworkStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, networkstatushandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NetworkStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNetworkStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNetworkStatusChanged: usize,
}
#[repr(C)]
pub struct INetworkInformationStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindConnectionProfilesAsync: unsafe extern "system" fn(this: *mut *mut Self, pprofilefilter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindConnectionProfilesAsync: usize,
}
#[repr(C)]
pub struct INetworkItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetNetworkTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkTypes) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INetworkSecuritySettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkAuthenticationType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkAuthenticationType) -> ::windows_sys::core::HRESULT,
    pub NetworkEncryptionType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkEncryptionType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INetworkStateChangeEventDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasNewInternetConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasNewConnectionCost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasNewNetworkConnectivityLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasNewDomainConnectivityLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasNewHostNameList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasNewWwanRegistrationState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INetworkStateChangeEventDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasNewTetheringOperationalState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasNewTetheringClientCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INetworkUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub BytesSent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectionDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionDuration: usize,
}
pub type IPInformation = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IProviderNetworkUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub BytesSent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub ProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProxyConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ProxyUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProxyUris: usize,
    pub CanConnectDirectly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRoutePolicy {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HostName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HostNameType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::DomainNameType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRoutePolicyFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateRoutePolicy: unsafe extern "system" fn(this: *mut *mut Self, connectionprofile: *mut ::core::ffi::c_void, hostname: *mut ::core::ffi::c_void, r#type: super::DomainNameType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWlanConnectionProfileDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetConnectedSsid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWwanConnectionProfileDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub HomeProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AccessPointName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetNetworkRegistrationState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WwanNetworkRegistrationState) -> ::windows_sys::core::HRESULT,
    pub GetCurrentDataClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WwanDataClass) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWwanConnectionProfileDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IPKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WwanNetworkIPKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PurposeGuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PurposeGuids: usize,
}
pub type LanIdentifier = *mut ::core::ffi::c_void;
pub type LanIdentifierData = *mut ::core::ffi::c_void;
pub type NetworkAdapter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct NetworkAuthenticationType(pub i32);
impl NetworkAuthenticationType {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const Open80211: Self = Self(2i32);
    pub const SharedKey80211: Self = Self(3i32);
    pub const Wpa: Self = Self(4i32);
    pub const WpaPsk: Self = Self(5i32);
    pub const WpaNone: Self = Self(6i32);
    pub const Rsna: Self = Self(7i32);
    pub const RsnaPsk: Self = Self(8i32);
    pub const Ihv: Self = Self(9i32);
    pub const Wpa3: Self = Self(10i32);
    pub const Wpa3Enterprise192Bits: Self = Self(10i32);
    pub const Wpa3Sae: Self = Self(11i32);
    pub const Owe: Self = Self(12i32);
    pub const Wpa3Enterprise: Self = Self(13i32);
}
impl ::core::marker::Copy for NetworkAuthenticationType {}
impl ::core::clone::Clone for NetworkAuthenticationType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct NetworkConnectivityLevel(pub i32);
impl NetworkConnectivityLevel {
    pub const None: Self = Self(0i32);
    pub const LocalAccess: Self = Self(1i32);
    pub const ConstrainedInternetAccess: Self = Self(2i32);
    pub const InternetAccess: Self = Self(3i32);
}
impl ::core::marker::Copy for NetworkConnectivityLevel {}
impl ::core::clone::Clone for NetworkConnectivityLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct NetworkCostType(pub i32);
impl NetworkCostType {
    pub const Unknown: Self = Self(0i32);
    pub const Unrestricted: Self = Self(1i32);
    pub const Fixed: Self = Self(2i32);
    pub const Variable: Self = Self(3i32);
}
impl ::core::marker::Copy for NetworkCostType {}
impl ::core::clone::Clone for NetworkCostType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct NetworkEncryptionType(pub i32);
impl NetworkEncryptionType {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const Wep: Self = Self(2i32);
    pub const Wep40: Self = Self(3i32);
    pub const Wep104: Self = Self(4i32);
    pub const Tkip: Self = Self(5i32);
    pub const Ccmp: Self = Self(6i32);
    pub const WpaUseGroup: Self = Self(7i32);
    pub const RsnUseGroup: Self = Self(8i32);
    pub const Ihv: Self = Self(9i32);
    pub const Gcmp: Self = Self(10i32);
    pub const Gcmp256: Self = Self(11i32);
}
impl ::core::marker::Copy for NetworkEncryptionType {}
impl ::core::clone::Clone for NetworkEncryptionType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NetworkItem = *mut ::core::ffi::c_void;
pub type NetworkSecuritySettings = *mut ::core::ffi::c_void;
pub type NetworkStateChangeEventDetails = *mut ::core::ffi::c_void;
pub type NetworkStatusChangedEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct NetworkTypes(pub u32);
impl NetworkTypes {
    pub const None: Self = Self(0u32);
    pub const Internet: Self = Self(1u32);
    pub const PrivateNetwork: Self = Self(2u32);
}
impl ::core::marker::Copy for NetworkTypes {}
impl ::core::clone::Clone for NetworkTypes {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NetworkUsage = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
pub struct NetworkUsageStates {
    pub Roaming: TriStates,
    pub Shared: TriStates,
}
impl ::core::marker::Copy for NetworkUsageStates {}
impl ::core::clone::Clone for NetworkUsageStates {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ProviderNetworkUsage = *mut ::core::ffi::c_void;
pub type ProxyConfiguration = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct RoamingStates(pub u32);
impl RoamingStates {
    pub const None: Self = Self(0u32);
    pub const NotRoaming: Self = Self(1u32);
    pub const Roaming: Self = Self(2u32);
}
impl ::core::marker::Copy for RoamingStates {}
impl ::core::clone::Clone for RoamingStates {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RoutePolicy = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct TriStates(pub i32);
impl TriStates {
    pub const DoNotCare: Self = Self(0i32);
    pub const No: Self = Self(1i32);
    pub const Yes: Self = Self(2i32);
}
impl ::core::marker::Copy for TriStates {}
impl ::core::clone::Clone for TriStates {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WlanConnectionProfileDetails = *mut ::core::ffi::c_void;
pub type WwanConnectionProfileDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct WwanDataClass(pub u32);
impl WwanDataClass {
    pub const None: Self = Self(0u32);
    pub const Gprs: Self = Self(1u32);
    pub const Edge: Self = Self(2u32);
    pub const Umts: Self = Self(4u32);
    pub const Hsdpa: Self = Self(8u32);
    pub const Hsupa: Self = Self(16u32);
    pub const LteAdvanced: Self = Self(32u32);
    pub const Cdma1xRtt: Self = Self(65536u32);
    pub const Cdma1xEvdo: Self = Self(131072u32);
    pub const Cdma1xEvdoRevA: Self = Self(262144u32);
    pub const Cdma1xEvdv: Self = Self(524288u32);
    pub const Cdma3xRtt: Self = Self(1048576u32);
    pub const Cdma1xEvdoRevB: Self = Self(2097152u32);
    pub const CdmaUmb: Self = Self(4194304u32);
    pub const Custom: Self = Self(2147483648u32);
}
impl ::core::marker::Copy for WwanDataClass {}
impl ::core::clone::Clone for WwanDataClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct WwanNetworkIPKind(pub i32);
impl WwanNetworkIPKind {
    pub const None: Self = Self(0i32);
    pub const Ipv4: Self = Self(1i32);
    pub const Ipv6: Self = Self(2i32);
    pub const Ipv4v6: Self = Self(3i32);
    pub const Ipv4v6v4Xlat: Self = Self(4i32);
}
impl ::core::marker::Copy for WwanNetworkIPKind {}
impl ::core::clone::Clone for WwanNetworkIPKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct WwanNetworkRegistrationState(pub i32);
impl WwanNetworkRegistrationState {
    pub const None: Self = Self(0i32);
    pub const Deregistered: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Home: Self = Self(3i32);
    pub const Roaming: Self = Self(4i32);
    pub const Partner: Self = Self(5i32);
    pub const Denied: Self = Self(6i32);
}
impl ::core::marker::Copy for WwanNetworkRegistrationState {}
impl ::core::clone::Clone for WwanNetworkRegistrationState {
    fn clone(&self) -> Self {
        *self
    }
}
