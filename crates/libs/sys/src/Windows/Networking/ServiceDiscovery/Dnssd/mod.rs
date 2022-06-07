pub type DnssdRegistrationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_ServiceDiscovery_Dnssd\"`*"]
#[repr(transparent)]
pub struct DnssdRegistrationStatus(pub i32);
impl DnssdRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const InvalidServiceName: Self = Self(1i32);
    pub const ServerError: Self = Self(2i32);
    pub const SecurityError: Self = Self(3i32);
}
impl ::core::marker::Copy for DnssdRegistrationStatus {}
impl ::core::clone::Clone for DnssdRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DnssdServiceInstance = *mut ::core::ffi::c_void;
pub type DnssdServiceInstanceCollection = *mut ::core::ffi::c_void;
pub type DnssdServiceWatcher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_ServiceDiscovery_Dnssd\"`*"]
#[repr(transparent)]
pub struct DnssdServiceWatcherStatus(pub i32);
impl DnssdServiceWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for DnssdServiceWatcherStatus {}
impl ::core::clone::Clone for DnssdServiceWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IDnssdRegistrationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DnssdRegistrationStatus) -> ::windows_sys::core::HRESULT,
    pub IPAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HasInstanceNameChanged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDnssdRegistrationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1031301842, data2: 58886, data3: 21328, data4: [115, 234, 126, 151, 240, 102, 22, 47] };
}
#[repr(C)]
pub struct IDnssdServiceInstance {
    pub base__: ::windows_sys::core::IInspectable,
    pub DnssdServiceInstanceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDnssdServiceInstanceName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HostName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHostName: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Port: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut *mut Self, value: u16) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, value: u16) -> ::windows_sys::core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(this: *mut *mut Self, value: u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TextAttributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TextAttributes: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub RegisterStreamSocketListenerAsync1: unsafe extern "system" fn(this: *mut *mut Self, socket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))]
    RegisterStreamSocketListenerAsync1: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    pub RegisterStreamSocketListenerAsync2: unsafe extern "system" fn(this: *mut *mut Self, socket: *mut ::core::ffi::c_void, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets")))]
    RegisterStreamSocketListenerAsync2: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub RegisterDatagramSocketAsync1: unsafe extern "system" fn(this: *mut *mut Self, socket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))]
    RegisterDatagramSocketAsync1: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    pub RegisterDatagramSocketAsync2: unsafe extern "system" fn(this: *mut *mut Self, socket: *mut ::core::ffi::c_void, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets")))]
    RegisterDatagramSocketAsync2: usize,
}
impl ::windows_sys::core::Interface for IDnssdServiceInstance {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3796294526, data2: 39077, data3: 19617, data4: [185, 228, 194, 83, 211, 60, 53, 255] };
}
#[repr(C)]
pub struct IDnssdServiceInstanceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, dnssdserviceinstancename: ::windows_sys::core::HSTRING, hostname: *mut ::core::ffi::c_void, port: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDnssdServiceInstanceFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1823498657, data2: 50296, data3: 17201, data4: [150, 132, 74, 242, 24, 108, 10, 43] };
}
#[repr(C)]
pub struct IDnssdServiceWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DnssdServiceWatcherStatus) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDnssdServiceWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3426015681, data2: 56189, data3: 19305, data4: [152, 61, 198, 248, 63, 32, 86, 130] };
}
