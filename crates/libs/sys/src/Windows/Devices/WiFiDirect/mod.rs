#[cfg(feature = "Devices_WiFiDirect_Services")]
pub mod Services;
#[repr(C)]
pub struct IWiFiDirectAdvertisement {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub InformationElements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InformationElements: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetInformationElements: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetInformationElements: usize,
    pub ListenStateDiscoverability: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows_sys::core::HRESULT,
    pub SetListenStateDiscoverability: unsafe extern "system" fn(this: *mut *mut Self, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows_sys::core::HRESULT,
    pub IsAutonomousGroupOwnerEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAutonomousGroupOwnerEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub LegacySettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiDirectAdvertisement2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedConfigurationMethods: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedConfigurationMethods: usize,
}
#[repr(C)]
pub struct IWiFiDirectAdvertisementPublisher {
    pub base__: ::windows_sys::core::IInspectable,
    pub Advertisement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows_sys::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectError) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiDirectConnectionListener {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ConnectionRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionRequested: usize,
}
#[repr(C)]
pub struct IWiFiDirectConnectionParameters {
    pub base__: ::windows_sys::core::IInspectable,
    pub GroupOwnerIntent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetGroupOwnerIntent: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiDirectConnectionParameters2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub PreferenceOrderedConfigurationMethods: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PreferenceOrderedConfigurationMethods: usize,
    pub PreferredPairingProcedure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectPairingProcedure) -> ::windows_sys::core::HRESULT,
    pub SetPreferredPairingProcedure: unsafe extern "system" fn(this: *mut *mut Self, value: WiFiDirectPairingProcedure) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiDirectConnectionParametersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub GetDevicePairingKinds: unsafe extern "system" fn(this: *mut *mut Self, configurationmethod: WiFiDirectConfigurationMethod, result__: *mut super::Enumeration::DevicePairingKinds) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    GetDevicePairingKinds: usize,
}
#[repr(C)]
pub struct IWiFiDirectConnectionRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
#[repr(C)]
pub struct IWiFiDirectConnectionRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetConnectionRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWiFiDirectDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WiFiDirectConnectionStatus) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectionStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionStatusChanged: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub GetConnectionEndpointPairs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))]
    GetConnectionEndpointPairs: usize,
}
#[repr(C)]
pub struct IWiFiDirectDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[repr(C)]
pub struct IWiFiDirectDeviceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, r#type: WiFiDirectDeviceSelectorType, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, connectionparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[repr(C)]
pub struct IWiFiDirectInformationElement {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Oui: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Oui: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetOui: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetOui: usize,
    pub OuiType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetOuiType: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetValue: usize,
}
#[repr(C)]
pub struct IWiFiDirectInformationElementStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    CreateFromBuffer: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))]
    pub CreateFromDeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, deviceinformation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation_Collections")))]
    CreateFromDeviceInformation: usize,
}
#[repr(C)]
pub struct IWiFiDirectLegacySettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Ssid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSsid: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub Passphrase: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Passphrase: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetPassphrase: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetPassphrase: usize,
}
pub type WiFiDirectAdvertisement = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectAdvertisementListenStateDiscoverability(pub i32);
impl WiFiDirectAdvertisementListenStateDiscoverability {
    pub const None: Self = Self(0i32);
    pub const Normal: Self = Self(1i32);
    pub const Intensive: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectAdvertisementListenStateDiscoverability {}
impl ::core::clone::Clone for WiFiDirectAdvertisementListenStateDiscoverability {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WiFiDirectAdvertisementPublisher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisherStatus(pub i32);
impl WiFiDirectAdvertisementPublisherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiDirectAdvertisementPublisherStatus {}
impl ::core::clone::Clone for WiFiDirectAdvertisementPublisherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WiFiDirectAdvertisementPublisherStatusChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectConfigurationMethod(pub i32);
impl WiFiDirectConfigurationMethod {
    pub const ProvidePin: Self = Self(0i32);
    pub const DisplayPin: Self = Self(1i32);
    pub const PushButton: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectConfigurationMethod {}
impl ::core::clone::Clone for WiFiDirectConfigurationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WiFiDirectConnectionListener = *mut ::core::ffi::c_void;
pub type WiFiDirectConnectionParameters = *mut ::core::ffi::c_void;
pub type WiFiDirectConnectionRequest = *mut ::core::ffi::c_void;
pub type WiFiDirectConnectionRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectConnectionStatus(pub i32);
impl WiFiDirectConnectionStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiDirectConnectionStatus {}
impl ::core::clone::Clone for WiFiDirectConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WiFiDirectDevice = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectDeviceSelectorType(pub i32);
impl WiFiDirectDeviceSelectorType {
    pub const DeviceInterface: Self = Self(0i32);
    pub const AssociationEndpoint: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiDirectDeviceSelectorType {}
impl ::core::clone::Clone for WiFiDirectDeviceSelectorType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectError(pub i32);
impl WiFiDirectError {
    pub const Success: Self = Self(0i32);
    pub const RadioNotAvailable: Self = Self(1i32);
    pub const ResourceInUse: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectError {}
impl ::core::clone::Clone for WiFiDirectError {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WiFiDirectInformationElement = *mut ::core::ffi::c_void;
pub type WiFiDirectLegacySettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectPairingProcedure(pub i32);
impl WiFiDirectPairingProcedure {
    pub const GroupOwnerNegotiation: Self = Self(0i32);
    pub const Invitation: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiDirectPairingProcedure {}
impl ::core::clone::Clone for WiFiDirectPairingProcedure {
    fn clone(&self) -> Self {
        *self
    }
}
