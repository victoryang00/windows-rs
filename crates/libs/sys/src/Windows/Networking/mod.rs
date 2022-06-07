#[cfg(feature = "Networking_BackgroundTransfer")]
pub mod BackgroundTransfer;
#[cfg(feature = "Networking_Connectivity")]
pub mod Connectivity;
#[cfg(feature = "Networking_NetworkOperators")]
pub mod NetworkOperators;
#[cfg(feature = "Networking_Proximity")]
pub mod Proximity;
#[cfg(feature = "Networking_PushNotifications")]
pub mod PushNotifications;
#[cfg(feature = "Networking_ServiceDiscovery")]
pub mod ServiceDiscovery;
#[cfg(feature = "Networking_Sockets")]
pub mod Sockets;
#[cfg(feature = "Networking_Vpn")]
pub mod Vpn;
#[cfg(feature = "Networking_XboxLive")]
pub mod XboxLive;
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
pub struct DomainNameType(pub i32);
impl DomainNameType {
    pub const Suffix: Self = Self(0i32);
    pub const FullyQualified: Self = Self(1i32);
}
impl ::core::marker::Copy for DomainNameType {}
impl ::core::clone::Clone for DomainNameType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EndpointPair = *mut ::core::ffi::c_void;
pub type HostName = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
pub struct HostNameSortOptions(pub u32);
impl HostNameSortOptions {
    pub const None: Self = Self(0u32);
    pub const OptimizeForLongConnections: Self = Self(2u32);
}
impl ::core::marker::Copy for HostNameSortOptions {}
impl ::core::clone::Clone for HostNameSortOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
pub struct HostNameType(pub i32);
impl HostNameType {
    pub const DomainName: Self = Self(0i32);
    pub const Ipv4: Self = Self(1i32);
    pub const Ipv6: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
}
impl ::core::marker::Copy for HostNameType {}
impl ::core::clone::Clone for HostNameType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IEndpointPair {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalHostName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLocalHostName: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LocalServiceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLocalServiceName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteHostName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRemoteHostName: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoteServiceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRemoteServiceName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEndpointPair {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 866167350, data2: 63738, data3: 19248, data4: [184, 86, 118, 81, 124, 59, 208, 109] };
}
#[repr(C)]
pub struct IEndpointPairFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateEndpointPair: unsafe extern "system" fn(this: *mut *mut Self, localhostname: *mut ::core::ffi::c_void, localservicename: ::windows_sys::core::HSTRING, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEndpointPairFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3054098801, data2: 25824, data3: 17451, data4: [170, 111, 204, 140, 143, 24, 31, 120] };
}
#[repr(C)]
pub struct IHostName {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_Connectivity")]
    pub IPInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    IPInformation: usize,
    pub RawName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CanonicalName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HostNameType) -> ::windows_sys::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut *mut Self, hostname: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHostName {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3213806253, data2: 60822, data3: 18855, data4: [144, 132, 212, 22, 202, 232, 141, 203] };
}
#[repr(C)]
pub struct IHostNameFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateHostName: unsafe extern "system" fn(this: *mut *mut Self, hostname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHostNameFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1166812141, data2: 28975, data3: 17782, data4: [173, 241, 194, 11, 44, 100, 53, 88] };
}
#[repr(C)]
pub struct IHostNameStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Compare: unsafe extern "system" fn(this: *mut *mut Self, value1: ::windows_sys::core::HSTRING, value2: ::windows_sys::core::HSTRING, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHostNameStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4136424639, data2: 41864, data3: 20107, data4: [145, 234, 84, 221, 109, 217, 1, 192] };
}
