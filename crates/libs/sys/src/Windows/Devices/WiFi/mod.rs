#[repr(C)]
pub struct IWiFiAdapter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_Connectivity")]
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    NetworkAdapter: usize,
    #[cfg(feature = "Foundation")]
    pub ScanAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScanAsync: usize,
    pub NetworkReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AvailableNetworksChanged: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AvailableNetworksChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAvailableNetworksChanged: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAvailableNetworksChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut *mut Self, availablenetwork: *mut ::core::ffi::c_void, reconnectionkind: WiFiReconnectionKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ConnectWithPasswordCredentialAsync: unsafe extern "system" fn(this: *mut *mut Self, availablenetwork: *mut ::core::ffi::c_void, reconnectionkind: WiFiReconnectionKind, passwordcredential: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ConnectWithPasswordCredentialAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ConnectWithPasswordCredentialAndSsidAsync: unsafe extern "system" fn(this: *mut *mut Self, availablenetwork: *mut ::core::ffi::c_void, reconnectionkind: WiFiReconnectionKind, passwordcredential: *mut ::core::ffi::c_void, ssid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ConnectWithPasswordCredentialAndSsidAsync: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiAdapter2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetWpsConfigurationAsync: unsafe extern "system" fn(this: *mut *mut Self, availablenetwork: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetWpsConfigurationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync: unsafe extern "system" fn(this: *mut *mut Self, availablenetwork: *mut ::core::ffi::c_void, reconnectionkind: WiFiReconnectionKind, passwordcredential: *mut ::core::ffi::c_void, ssid: ::windows_sys::core::HSTRING, connectionmethod: WiFiConnectionMethod, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync: usize,
}
#[repr(C)]
pub struct IWiFiAdapterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAdaptersAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAdaptersAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[repr(C)]
pub struct IWiFiAvailableNetwork {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uptime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uptime: usize,
    pub Ssid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Bssid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ChannelCenterFrequencyInKilohertz: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NetworkRssiInDecibelMilliwatts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SignalBars: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub NetworkKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiNetworkKind) -> ::windows_sys::core::HRESULT,
    pub PhyKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiPhyKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Networking_Connectivity")]
    pub SecuritySettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    SecuritySettings: usize,
    #[cfg(feature = "Foundation")]
    pub BeaconInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeaconInterval: usize,
    pub IsWiFiDirect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiConnectionResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiConnectionStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiNetworkReport {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AvailableNetworks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AvailableNetworks: usize,
}
#[repr(C)]
pub struct IWiFiWpsConfigurationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiWpsConfigurationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedWpsKinds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedWpsKinds: usize,
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiAccessStatus(pub i32);
impl WiFiAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiAccessStatus {}
impl ::core::clone::Clone for WiFiAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WiFiAdapter = *mut ::core::ffi::c_void;
pub type WiFiAvailableNetwork = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiConnectionMethod(pub i32);
impl WiFiConnectionMethod {
    pub const Default: Self = Self(0i32);
    pub const WpsPin: Self = Self(1i32);
    pub const WpsPushButton: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiConnectionMethod {}
impl ::core::clone::Clone for WiFiConnectionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WiFiConnectionResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiConnectionStatus(pub i32);
impl WiFiConnectionStatus {
    pub const UnspecifiedFailure: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AccessRevoked: Self = Self(2i32);
    pub const InvalidCredential: Self = Self(3i32);
    pub const NetworkNotAvailable: Self = Self(4i32);
    pub const Timeout: Self = Self(5i32);
    pub const UnsupportedAuthenticationProtocol: Self = Self(6i32);
}
impl ::core::marker::Copy for WiFiConnectionStatus {}
impl ::core::clone::Clone for WiFiConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiNetworkKind(pub i32);
impl WiFiNetworkKind {
    pub const Any: Self = Self(0i32);
    pub const Infrastructure: Self = Self(1i32);
    pub const Adhoc: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiNetworkKind {}
impl ::core::clone::Clone for WiFiNetworkKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WiFiNetworkReport = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiPhyKind(pub i32);
impl WiFiPhyKind {
    pub const Unknown: Self = Self(0i32);
    pub const Fhss: Self = Self(1i32);
    pub const Dsss: Self = Self(2i32);
    pub const IRBaseband: Self = Self(3i32);
    pub const Ofdm: Self = Self(4i32);
    pub const Hrdsss: Self = Self(5i32);
    pub const Erp: Self = Self(6i32);
    pub const HT: Self = Self(7i32);
    pub const Vht: Self = Self(8i32);
    pub const Dmg: Self = Self(9i32);
    pub const HE: Self = Self(10i32);
}
impl ::core::marker::Copy for WiFiPhyKind {}
impl ::core::clone::Clone for WiFiPhyKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiReconnectionKind(pub i32);
impl WiFiReconnectionKind {
    pub const Automatic: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiReconnectionKind {}
impl ::core::clone::Clone for WiFiReconnectionKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WiFiWpsConfigurationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiWpsConfigurationStatus(pub i32);
impl WiFiWpsConfigurationStatus {
    pub const UnspecifiedFailure: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const Timeout: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiWpsConfigurationStatus {}
impl ::core::clone::Clone for WiFiWpsConfigurationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiWpsKind(pub i32);
impl WiFiWpsKind {
    pub const Unknown: Self = Self(0i32);
    pub const Pin: Self = Self(1i32);
    pub const PushButton: Self = Self(2i32);
    pub const Nfc: Self = Self(3i32);
    pub const Ethernet: Self = Self(4i32);
    pub const Usb: Self = Self(5i32);
}
impl ::core::marker::Copy for WiFiWpsKind {}
impl ::core::clone::Clone for WiFiWpsKind {
    fn clone(&self) -> Self {
        *self
    }
}
