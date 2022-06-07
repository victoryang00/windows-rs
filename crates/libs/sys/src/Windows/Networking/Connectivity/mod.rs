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
impl ::windows_sys::core::Interface for IAttributedNetworkUsage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4150898745, data2: 60578, data3: 17899, data4: [173, 225, 176, 54, 139, 117, 108, 73] };
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
impl ::windows_sys::core::Interface for ICellularApnContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1873095156, data2: 61437, data3: 17730, data4: [154, 178, 112, 91, 191, 148, 148, 58] };
}
#[repr(C)]
pub struct ICellularApnContext2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProfileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetProfileName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICellularApnContext2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1991306010, data2: 44105, data3: 17232, data4: [177, 229, 220, 71, 99, 188, 105, 199] };
}
#[repr(C)]
pub struct IConnectionCost {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkCostType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkCostType) -> ::windows_sys::core::HRESULT,
    pub Roaming: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub OverDataLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ApproachingDataLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConnectionCost {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3134707753, data2: 13334, data3: 19216, data4: [162, 2, 186, 192, 176, 117, 189, 174] };
}
#[repr(C)]
pub struct IConnectionCost2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundDataUsageRestricted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConnectionCost2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2383493637, data2: 57865, data3: 17737, data4: [187, 37, 94, 13, 182, 145, 203, 5] };
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
impl ::windows_sys::core::Interface for IConnectionProfile {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1908020284, data2: 22926, data3: 18896, data4: [132, 235, 143, 235, 174, 220, 193, 149] };
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
impl ::windows_sys::core::Interface for IConnectionProfile2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3791933765, data2: 19615, data3: 16396, data4: [145, 80, 126, 199, 214, 226, 136, 138] };
}
#[repr(C)]
pub struct IConnectionProfile3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAttributedNetworkUsageAsync: unsafe extern "system" fn(this: *mut *mut Self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAttributedNetworkUsageAsync: usize,
}
impl ::windows_sys::core::Interface for IConnectionProfile3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1468802344, data2: 19673, data3: 16737, data4: [128, 69, 32, 28, 253, 91, 17, 92] };
}
#[repr(C)]
pub struct IConnectionProfile4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetProviderNetworkUsageAsync: unsafe extern "system" fn(this: *mut *mut Self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetProviderNetworkUsageAsync: usize,
}
impl ::windows_sys::core::Interface for IConnectionProfile4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2049786573, data2: 33248, data3: 19174, data4: [171, 237, 171, 156, 161, 62, 183, 20] };
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
impl ::windows_sys::core::Interface for IConnectionProfile5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2234916551, data2: 40051, data3: 19424, data4: [143, 20, 87, 142, 236, 113, 238, 14] };
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
impl ::windows_sys::core::Interface for IConnectionProfileFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 541883592, data2: 48429, data3: 20109, data4: [164, 179, 69, 94, 195, 55, 56, 138] };
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
impl ::windows_sys::core::Interface for IConnectionProfileFilter2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3439759073, data2: 50172, data3: 20397, data4: [157, 220, 89, 63, 170, 75, 120, 133] };
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
impl ::windows_sys::core::Interface for IConnectionProfileFilter3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 178915776, data2: 20500, data3: 17532, data4: [136, 9, 174, 228, 203, 10, 249, 74] };
}
#[repr(C)]
pub struct IConnectionSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConnectionSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4287651148, data2: 63547, data3: 16816, data4: [138, 12, 20, 98, 217, 197, 107, 115] };
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
impl ::windows_sys::core::Interface for IConnectivityInterval {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1336557567, data2: 26438, data3: 18468, data4: [169, 100, 238, 216, 232, 127, 135, 9] };
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
impl ::windows_sys::core::Interface for IConnectivityManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1361106097, data2: 20401, data3: 18608, data4: [175, 201, 66, 224, 9, 42, 129, 100] };
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
impl ::windows_sys::core::Interface for IDataPlanStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2541390732, data2: 14469, data3: 16627, data4: [136, 81, 66, 205, 43, 213, 104, 187] };
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
impl ::windows_sys::core::Interface for IDataPlanUsage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3105966381, data2: 15172, data3: 18431, data4: [179, 97, 190, 89, 230, 158, 209, 176] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IDataUsage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3242401235, data2: 45382, data3: 19769, data4: [185, 89, 12, 105, 176, 150, 197, 18] };
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
impl ::windows_sys::core::Interface for IIPInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3629204960, data2: 5007, data3: 18391, data4: [155, 58, 54, 187, 72, 140, 239, 51] };
}
#[repr(C)]
pub struct ILanIdentifier {
    pub base__: ::windows_sys::core::IInspectable,
    pub InfrastructureId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PortId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NetworkAdapterId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILanIdentifier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1219122090, data2: 4360, data3: 17734, data4: [166, 203, 154, 116, 218, 75, 123, 160] };
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
impl ::windows_sys::core::Interface for ILanIdentifierData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2806940611, data2: 54841, data3: 17854, data4: [163, 106, 196, 228, 174, 175, 109, 155] };
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
impl ::windows_sys::core::Interface for INetworkAdapter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 995372547, data2: 21384, data3: 18796, data4: [168, 163, 175, 253, 57, 174, 194, 230] };
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
impl ::windows_sys::core::Interface for INetworkInformationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1349843025, data2: 38157, data3: 16741, data4: [156, 21, 54, 86, 25, 72, 30, 234] };
}
#[repr(C)]
pub struct INetworkInformationStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindConnectionProfilesAsync: unsafe extern "system" fn(this: *mut *mut Self, pprofilefilter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindConnectionProfilesAsync: usize,
}
impl ::windows_sys::core::Interface for INetworkInformationStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1167912212, data2: 10290, data3: 18870, data4: [186, 110, 226, 101, 240, 71, 134, 168] };
}
#[repr(C)]
pub struct INetworkItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetNetworkTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkTypes) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INetworkItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 29117753, data2: 62944, data3: 17767, data4: [162, 140, 66, 8, 12, 131, 27, 43] };
}
#[repr(C)]
pub struct INetworkSecuritySettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkAuthenticationType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkAuthenticationType) -> ::windows_sys::core::HRESULT,
    pub NetworkEncryptionType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkEncryptionType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INetworkSecuritySettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2090892941, data2: 37243, data3: 19295, data4: [184, 77, 40, 247, 165, 172, 84, 2] };
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
impl ::windows_sys::core::Interface for INetworkStateChangeEventDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 520942387, data2: 55206, data3: 17629, data4: [164, 233, 104, 124, 71, 107, 144, 61] };
}
#[repr(C)]
pub struct INetworkStateChangeEventDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasNewTetheringOperationalState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasNewTetheringClientCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INetworkStateChangeEventDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3594764520, data2: 12499, data3: 20330, data4: [173, 71, 106, 24, 115, 206, 179, 193] };
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
impl ::windows_sys::core::Interface for INetworkUsage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1239060430, data2: 39301, data3: 18727, data4: [191, 91, 7, 43, 92, 101, 248, 217] };
}
pub type IPInformation = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IProviderNetworkUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub BytesSent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub ProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProviderNetworkUsage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1590074884, data2: 31025, data3: 18632, data4: [184, 243, 70, 48, 15, 164, 39, 40] };
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
impl ::windows_sys::core::Interface for IProxyConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4013580468, data2: 36868, data3: 19926, data4: [183, 216, 179, 229, 2, 244, 170, 208] };
}
#[repr(C)]
pub struct IRoutePolicy {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HostName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HostNameType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::DomainNameType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRoutePolicy {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 296469676, data2: 4039, data3: 17124, data4: [135, 66, 86, 153, 35, 177, 202, 17] };
}
#[repr(C)]
pub struct IRoutePolicyFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateRoutePolicy: unsafe extern "system" fn(this: *mut *mut Self, connectionprofile: *mut ::core::ffi::c_void, hostname: *mut ::core::ffi::c_void, r#type: super::DomainNameType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRoutePolicyFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 906131763, data2: 41358, data3: 19893, data4: [166, 151, 245, 143, 167, 54, 78, 68] };
}
#[repr(C)]
pub struct IWlanConnectionProfileDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetConnectedSsid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWlanConnectionProfileDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1444976843, data2: 45914, data3: 19441, data4: [168, 132, 183, 85, 126, 136, 255, 134] };
}
#[repr(C)]
pub struct IWwanConnectionProfileDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub HomeProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AccessPointName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetNetworkRegistrationState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WwanNetworkRegistrationState) -> ::windows_sys::core::HRESULT,
    pub GetCurrentDataClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WwanDataClass) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWwanConnectionProfileDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 239970558, data2: 33631, data3: 19955, data4: [130, 253, 223, 85, 110, 188, 9, 239] };
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
impl ::windows_sys::core::Interface for IWwanConnectionProfileDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2054508254, data2: 41453, data3: 18610, data4: [142, 146, 180, 96, 3, 61, 82, 226] };
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
