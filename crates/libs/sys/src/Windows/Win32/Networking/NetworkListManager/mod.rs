#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEnumNetworkConnections {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumvar: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenumnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IEnumNetworkConnections {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3702521862, data2: 22287, data3: 19099, data4: [141, 105, 25, 159, 219, 165, 114, 59] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEnumNetworks {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumvar: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenumnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IEnumNetworks {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3702521859, data2: 22287, data3: 19099, data4: [141, 105, 25, 159, 219, 165, 114, 59] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetwork {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, psznetworkname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, sznetworknewname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, szdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    pub GetNetworkId: unsafe extern "system" fn(this: *mut *mut Self, pgdguidnetworkid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetDomainType: unsafe extern "system" fn(this: *mut *mut Self, pnetworktype: *mut NLM_DOMAIN_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworkConnections: unsafe extern "system" fn(this: *mut *mut Self, ppenumnetworkconnection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworkConnections: usize,
    pub GetTimeCreatedAndConnected: unsafe extern "system" fn(this: *mut *mut Self, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IsConnectedToInternet: unsafe extern "system" fn(this: *mut *mut Self, pbisconnected: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut *mut Self, pbisconnected: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetConnectivity: unsafe extern "system" fn(this: *mut *mut Self, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows_sys::core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(this: *mut *mut Self, pcategory: *mut NLM_NETWORK_CATEGORY) -> ::windows_sys::core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(this: *mut *mut Self, newcategory: NLM_NETWORK_CATEGORY) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for INetwork {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3702521858, data2: 22287, data3: 19099, data4: [141, 105, 25, 159, 219, 165, 114, 59] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetworkConnection {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetwork: unsafe extern "system" fn(this: *mut *mut Self, ppnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetwork: usize,
    pub IsConnectedToInternet: unsafe extern "system" fn(this: *mut *mut Self, pbisconnected: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut *mut Self, pbisconnected: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetConnectivity: unsafe extern "system" fn(this: *mut *mut Self, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows_sys::core::HRESULT,
    pub GetConnectionId: unsafe extern "system" fn(this: *mut *mut Self, pgdconnectionid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetAdapterId: unsafe extern "system" fn(this: *mut *mut Self, pgdadapterid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetDomainType: unsafe extern "system" fn(this: *mut *mut Self, pdomaintype: *mut NLM_DOMAIN_TYPE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for INetworkConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3702521861, data2: 22287, data3: 19099, data4: [141, 105, 25, 159, 219, 165, 114, 59] };
}
#[repr(C)]
pub struct INetworkConnectionCost {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCost: unsafe extern "system" fn(this: *mut *mut Self, pcost: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDataPlanStatus: unsafe extern "system" fn(this: *mut *mut Self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDataPlanStatus: usize,
}
impl ::windows_sys::core::Interface for INetworkConnectionCost {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3702521866, data2: 22287, data3: 19099, data4: [141, 105, 25, 159, 219, 165, 114, 59] };
}
#[repr(C)]
pub struct INetworkConnectionCostEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub ConnectionCostChanged: unsafe extern "system" fn(this: *mut *mut Self, connectionid: ::windows_sys::core::GUID, newcost: u32) -> ::windows_sys::core::HRESULT,
    pub ConnectionDataPlanStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, connectionid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INetworkConnectionCostEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3702521867, data2: 22287, data3: 19099, data4: [141, 105, 25, 159, 219, 165, 114, 59] };
}
#[repr(C)]
pub struct INetworkConnectionEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub NetworkConnectionConnectivityChanged: unsafe extern "system" fn(this: *mut *mut Self, connectionid: ::windows_sys::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows_sys::core::HRESULT,
    pub NetworkConnectionPropertyChanged: unsafe extern "system" fn(this: *mut *mut Self, connectionid: ::windows_sys::core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INetworkConnectionEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3702521863, data2: 22287, data3: 19099, data4: [141, 105, 25, 159, 219, 165, 114, 59] };
}
#[repr(C)]
pub struct INetworkCostManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCost: unsafe extern "system" fn(this: *mut *mut Self, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDataPlanStatus: unsafe extern "system" fn(this: *mut *mut Self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDataPlanStatus: usize,
    pub SetDestinationAddresses: unsafe extern "system" fn(this: *mut *mut Self, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: i16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INetworkCostManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3702521864, data2: 22287, data3: 19099, data4: [141, 105, 25, 159, 219, 165, 114, 59] };
}
#[repr(C)]
pub struct INetworkCostManagerEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub CostChanged: unsafe extern "system" fn(this: *mut *mut Self, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows_sys::core::HRESULT,
    pub DataPlanStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, pdestaddr: *const NLM_SOCKADDR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INetworkCostManagerEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3702521865, data2: 22287, data3: 19099, data4: [141, 105, 25, 159, 219, 165, 114, 59] };
}
#[repr(C)]
pub struct INetworkEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub NetworkAdded: unsafe extern "system" fn(this: *mut *mut Self, networkid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub NetworkDeleted: unsafe extern "system" fn(this: *mut *mut Self, networkid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub NetworkConnectivityChanged: unsafe extern "system" fn(this: *mut *mut Self, networkid: ::windows_sys::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows_sys::core::HRESULT,
    pub NetworkPropertyChanged: unsafe extern "system" fn(this: *mut *mut Self, networkid: ::windows_sys::core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INetworkEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3702521860, data2: 22287, data3: 19099, data4: [141, 105, 25, 159, 219, 165, 114, 59] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetworkListManager {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworks: unsafe extern "system" fn(this: *mut *mut Self, flags: NLM_ENUM_NETWORK, ppenumnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetwork: unsafe extern "system" fn(this: *mut *mut Self, gdnetworkid: ::windows_sys::core::GUID, ppnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetwork: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworkConnections: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworkConnections: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworkConnection: unsafe extern "system" fn(this: *mut *mut Self, gdnetworkconnectionid: ::windows_sys::core::GUID, ppnetworkconnection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworkConnection: usize,
    pub IsConnectedToInternet: unsafe extern "system" fn(this: *mut *mut Self, pbisconnected: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut *mut Self, pbisconnected: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetConnectivity: unsafe extern "system" fn(this: *mut *mut Self, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows_sys::core::HRESULT,
    pub SetSimulatedProfileInfo: unsafe extern "system" fn(this: *mut *mut Self, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows_sys::core::HRESULT,
    pub ClearSimulatedProfileInfo: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for INetworkListManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3702521856, data2: 22287, data3: 19099, data4: [141, 105, 25, 159, 219, 165, 114, 59] };
}
#[repr(C)]
pub struct INetworkListManagerEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub ConnectivityChanged: unsafe extern "system" fn(this: *mut *mut Self, newconnectivity: NLM_CONNECTIVITY) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INetworkListManagerEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3702521857, data2: 22287, data3: 19099, data4: [141, 105, 25, 159, 219, 165, 114, 59] };
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_AllowMerge: &str = "NA_AllowMerge";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_CategoryReadOnly: &str = "NA_CategoryReadOnly";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_CategorySetByPolicy: &str = "NA_CategorySetByPolicy";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_DescriptionReadOnly: &str = "NA_DescriptionReadOnly";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_DescriptionSetByPolicy: &str = "NA_DescriptionSetByPolicy";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_DomainAuthenticationFailed: &str = "NA_DomainAuthenticationFailed";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_IconReadOnly: &str = "NA_IconReadOnly";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_IconSetByPolicy: &str = "NA_IconSetByPolicy";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_InternetConnectivityV4: &str = "NA_InternetConnectivityV4";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_InternetConnectivityV6: &str = "NA_InternetConnectivityV6";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_NameReadOnly: &str = "NA_NameReadOnly";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_NameSetByPolicy: &str = "NA_NameSetByPolicy";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_NetworkClass: &str = "NA_NetworkClass";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub type NLM_CONNECTION_COST = i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_UNKNOWN: NLM_CONNECTION_COST = 0i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_UNRESTRICTED: NLM_CONNECTION_COST = 1i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_FIXED: NLM_CONNECTION_COST = 2i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_VARIABLE: NLM_CONNECTION_COST = 4i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_OVERDATALIMIT: NLM_CONNECTION_COST = 65536i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_CONGESTED: NLM_CONNECTION_COST = 131072i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_ROAMING: NLM_CONNECTION_COST = 262144i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_APPROACHINGDATALIMIT: NLM_CONNECTION_COST = 524288i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub type NLM_CONNECTION_PROPERTY_CHANGE = i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_PROPERTY_CHANGE_AUTHENTICATION: NLM_CONNECTION_PROPERTY_CHANGE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub type NLM_CONNECTIVITY = i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_DISCONNECTED: NLM_CONNECTIVITY = 0i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV4_NOTRAFFIC: NLM_CONNECTIVITY = 1i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV6_NOTRAFFIC: NLM_CONNECTIVITY = 2i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV4_SUBNET: NLM_CONNECTIVITY = 16i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV4_LOCALNETWORK: NLM_CONNECTIVITY = 32i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV4_INTERNET: NLM_CONNECTIVITY = 64i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV6_SUBNET: NLM_CONNECTIVITY = 256i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV6_LOCALNETWORK: NLM_CONNECTIVITY = 512i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV6_INTERNET: NLM_CONNECTIVITY = 1024i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NLM_DATAPLAN_STATUS {
    pub InterfaceGuid: ::windows_sys::core::GUID,
    pub UsageData: NLM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub NextBillingCycle: super::super::Foundation::FILETIME,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NLM_DATAPLAN_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NLM_DATAPLAN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub type NLM_DOMAIN_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_DOMAIN_TYPE_NON_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_DOMAIN_TYPE_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_DOMAIN_TYPE_DOMAIN_AUTHENTICATED: NLM_DOMAIN_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub type NLM_ENUM_NETWORK = i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_ENUM_NETWORK_CONNECTED: NLM_ENUM_NETWORK = 1i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_ENUM_NETWORK_DISCONNECTED: NLM_ENUM_NETWORK = 2i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_ENUM_NETWORK_ALL: NLM_ENUM_NETWORK = 3i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub type NLM_INTERNET_CONNECTIVITY = i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_INTERNET_CONNECTIVITY_WEBHIJACK: NLM_INTERNET_CONNECTIVITY = 1i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_INTERNET_CONNECTIVITY_PROXIED: NLM_INTERNET_CONNECTIVITY = 2i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_INTERNET_CONNECTIVITY_CORPORATE: NLM_INTERNET_CONNECTIVITY = 4i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_MAX_ADDRESS_LIST_SIZE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub type NLM_NETWORK_CATEGORY = i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_CATEGORY_PUBLIC: NLM_NETWORK_CATEGORY = 0i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_CATEGORY_PRIVATE: NLM_NETWORK_CATEGORY = 1i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED: NLM_NETWORK_CATEGORY = 2i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub type NLM_NETWORK_CLASS = i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_IDENTIFYING: NLM_NETWORK_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_IDENTIFIED: NLM_NETWORK_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_UNIDENTIFIED: NLM_NETWORK_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub type NLM_NETWORK_PROPERTY_CHANGE = i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_PROPERTY_CHANGE_CONNECTION: NLM_NETWORK_PROPERTY_CHANGE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_PROPERTY_CHANGE_DESCRIPTION: NLM_NETWORK_PROPERTY_CHANGE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_PROPERTY_CHANGE_NAME: NLM_NETWORK_PROPERTY_CHANGE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_PROPERTY_CHANGE_ICON: NLM_NETWORK_PROPERTY_CHANGE = 8i32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_PROPERTY_CHANGE_CATEGORY_VALUE: NLM_NETWORK_PROPERTY_CHANGE = 16i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub struct NLM_SIMULATED_PROFILE_INFO {
    pub ProfileName: [u16; 256],
    pub cost: NLM_CONNECTION_COST,
    pub UsageInMegabytes: u32,
    pub DataLimitInMegabytes: u32,
}
impl ::core::marker::Copy for NLM_SIMULATED_PROFILE_INFO {}
impl ::core::clone::Clone for NLM_SIMULATED_PROFILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub struct NLM_SOCKADDR {
    pub data: [u8; 128],
}
impl ::core::marker::Copy for NLM_SOCKADDR {}
impl ::core::clone::Clone for NLM_SOCKADDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NLM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NLM_USAGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NLM_USAGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NetworkListManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3702524929, data2: 22287, data3: 19099, data4: [141, 105, 25, 159, 219, 165, 114, 59] };
