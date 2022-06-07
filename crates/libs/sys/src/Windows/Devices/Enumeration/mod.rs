#[cfg(feature = "Devices_Enumeration_Pnp")]
pub mod Pnp;
pub type DeviceAccessChangedEventArgs = *mut ::core::ffi::c_void;
pub type DeviceAccessInformation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceAccessStatus(pub i32);
impl DeviceAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for DeviceAccessStatus {}
impl ::core::clone::Clone for DeviceAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceClass(pub i32);
impl DeviceClass {
    pub const All: Self = Self(0i32);
    pub const AudioCapture: Self = Self(1i32);
    pub const AudioRender: Self = Self(2i32);
    pub const PortableStorageDevice: Self = Self(3i32);
    pub const VideoCapture: Self = Self(4i32);
    pub const ImageScanner: Self = Self(5i32);
    pub const Location: Self = Self(6i32);
}
impl ::core::marker::Copy for DeviceClass {}
impl ::core::clone::Clone for DeviceClass {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DeviceConnectionChangeTriggerDetails = *mut ::core::ffi::c_void;
pub type DeviceDisconnectButtonClickedEventArgs = *mut ::core::ffi::c_void;
pub type DeviceInformation = *mut ::core::ffi::c_void;
pub type DeviceInformationCollection = *mut ::core::ffi::c_void;
pub type DeviceInformationCustomPairing = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceInformationKind(pub i32);
impl DeviceInformationKind {
    pub const Unknown: Self = Self(0i32);
    pub const DeviceInterface: Self = Self(1i32);
    pub const DeviceContainer: Self = Self(2i32);
    pub const Device: Self = Self(3i32);
    pub const DeviceInterfaceClass: Self = Self(4i32);
    pub const AssociationEndpoint: Self = Self(5i32);
    pub const AssociationEndpointContainer: Self = Self(6i32);
    pub const AssociationEndpointService: Self = Self(7i32);
    pub const DevicePanel: Self = Self(8i32);
}
impl ::core::marker::Copy for DeviceInformationKind {}
impl ::core::clone::Clone for DeviceInformationKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DeviceInformationPairing = *mut ::core::ffi::c_void;
pub type DeviceInformationUpdate = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DevicePairingKinds(pub u32);
impl DevicePairingKinds {
    pub const None: Self = Self(0u32);
    pub const ConfirmOnly: Self = Self(1u32);
    pub const DisplayPin: Self = Self(2u32);
    pub const ProvidePin: Self = Self(4u32);
    pub const ConfirmPinMatch: Self = Self(8u32);
    pub const ProvidePasswordCredential: Self = Self(16u32);
}
impl ::core::marker::Copy for DevicePairingKinds {}
impl ::core::clone::Clone for DevicePairingKinds {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DevicePairingProtectionLevel(pub i32);
impl DevicePairingProtectionLevel {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Encryption: Self = Self(2i32);
    pub const EncryptionAndAuthentication: Self = Self(3i32);
}
impl ::core::marker::Copy for DevicePairingProtectionLevel {}
impl ::core::clone::Clone for DevicePairingProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DevicePairingRequestedEventArgs = *mut ::core::ffi::c_void;
pub type DevicePairingResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DevicePairingResultStatus(pub i32);
impl DevicePairingResultStatus {
    pub const Paired: Self = Self(0i32);
    pub const NotReadyToPair: Self = Self(1i32);
    pub const NotPaired: Self = Self(2i32);
    pub const AlreadyPaired: Self = Self(3i32);
    pub const ConnectionRejected: Self = Self(4i32);
    pub const TooManyConnections: Self = Self(5i32);
    pub const HardwareFailure: Self = Self(6i32);
    pub const AuthenticationTimeout: Self = Self(7i32);
    pub const AuthenticationNotAllowed: Self = Self(8i32);
    pub const AuthenticationFailure: Self = Self(9i32);
    pub const NoSupportedProfiles: Self = Self(10i32);
    pub const ProtectionLevelCouldNotBeMet: Self = Self(11i32);
    pub const AccessDenied: Self = Self(12i32);
    pub const InvalidCeremonyData: Self = Self(13i32);
    pub const PairingCanceled: Self = Self(14i32);
    pub const OperationAlreadyInProgress: Self = Self(15i32);
    pub const RequiredHandlerNotRegistered: Self = Self(16i32);
    pub const RejectedByHandler: Self = Self(17i32);
    pub const RemoteDeviceHasAssociation: Self = Self(18i32);
    pub const Failed: Self = Self(19i32);
}
impl ::core::marker::Copy for DevicePairingResultStatus {}
impl ::core::clone::Clone for DevicePairingResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DevicePicker = *mut ::core::ffi::c_void;
pub type DevicePickerAppearance = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DevicePickerDisplayStatusOptions(pub u32);
impl DevicePickerDisplayStatusOptions {
    pub const None: Self = Self(0u32);
    pub const ShowProgress: Self = Self(1u32);
    pub const ShowDisconnectButton: Self = Self(2u32);
    pub const ShowRetryButton: Self = Self(4u32);
}
impl ::core::marker::Copy for DevicePickerDisplayStatusOptions {}
impl ::core::clone::Clone for DevicePickerDisplayStatusOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DevicePickerFilter = *mut ::core::ffi::c_void;
pub type DeviceSelectedEventArgs = *mut ::core::ffi::c_void;
pub type DeviceThumbnail = *mut ::core::ffi::c_void;
pub type DeviceUnpairingResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceUnpairingResultStatus(pub i32);
impl DeviceUnpairingResultStatus {
    pub const Unpaired: Self = Self(0i32);
    pub const AlreadyUnpaired: Self = Self(1i32);
    pub const OperationAlreadyInProgress: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
}
impl ::core::marker::Copy for DeviceUnpairingResultStatus {}
impl ::core::clone::Clone for DeviceUnpairingResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DeviceWatcher = *mut ::core::ffi::c_void;
pub type DeviceWatcherEvent = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceWatcherEventKind(pub i32);
impl DeviceWatcherEventKind {
    pub const Add: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Remove: Self = Self(2i32);
}
impl ::core::marker::Copy for DeviceWatcherEventKind {}
impl ::core::clone::Clone for DeviceWatcherEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceWatcherStatus(pub i32);
impl DeviceWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for DeviceWatcherStatus {}
impl ::core::clone::Clone for DeviceWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DeviceWatcherTriggerDetails = *mut ::core::ffi::c_void;
pub type EnclosureLocation = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDeviceAccessChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceAccessStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceAccessChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3738831820, data2: 20381, data3: 20312, data4: [157, 186, 169, 188, 128, 4, 8, 213] };
}
#[repr(C)]
pub struct IDeviceAccessChangedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceAccessChangedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2186424930, data2: 37707, data3: 19248, data4: [161, 120, 173, 195, 159, 47, 43, 227] };
}
#[repr(C)]
pub struct IDeviceAccessInformation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AccessChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccessChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccessChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccessChanged: usize,
    pub CurrentStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceAccessStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceAccessInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 195730035, data2: 28133, data3: 18709, data4: [141, 221, 154, 5, 84, 166, 245, 69] };
}
#[repr(C)]
pub struct IDeviceAccessInformationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromDeviceClassId: unsafe extern "system" fn(this: *mut *mut Self, deviceclassid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromDeviceClass: unsafe extern "system" fn(this: *mut *mut Self, deviceclass: DeviceClass, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceAccessInformationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1464587219, data2: 24368, data3: 17869, data4: [138, 148, 114, 79, 229, 151, 48, 132] };
}
#[repr(C)]
pub struct IDeviceConnectionChangeTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceConnectionChangeTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3092745228, data2: 48065, data3: 18507, data4: [191, 250, 123, 49, 220, 194, 0, 178] };
}
#[repr(C)]
pub struct IDeviceDisconnectButtonClickedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceDisconnectButtonClickedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2386867565, data2: 63746, data3: 18944, data4: [181, 54, 243, 121, 146, 230, 162, 167] };
}
#[repr(C)]
pub struct IDeviceInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub EnclosureLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, updateinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetThumbnailAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetGlyphThumbnailAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetGlyphThumbnailAsync: usize,
}
impl ::windows_sys::core::Interface for IDeviceInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2879454101, data2: 17304, data3: 18589, data4: [142, 68, 230, 19, 9, 39, 1, 31] };
}
#[repr(C)]
pub struct IDeviceInformation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceInformationKind) -> ::windows_sys::core::HRESULT,
    pub Pairing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceInformation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4048987704, data2: 31127, data3: 18649, data4: [161, 12, 38, 157, 70, 83, 63, 72] };
}
#[repr(C)]
pub struct IDeviceInformationCustomPairing {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PairAsync: unsafe extern "system" fn(this: *mut *mut Self, pairingkindssupported: DevicePairingKinds, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PairWithProtectionLevelAsync: unsafe extern "system" fn(this: *mut *mut Self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairWithProtectionLevelAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PairWithProtectionLevelAndSettingsAsync: unsafe extern "system" fn(this: *mut *mut Self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairWithProtectionLevelAndSettingsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PairingRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePairingRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePairingRequested: usize,
}
impl ::windows_sys::core::Interface for IDeviceInformationCustomPairing {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2232650754, data2: 20198, data3: 18708, data4: [131, 112, 16, 122, 57, 20, 76, 14] };
}
#[repr(C)]
pub struct IDeviceInformationPairing {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPaired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanPair: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PairAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PairWithProtectionLevelAsync: unsafe extern "system" fn(this: *mut *mut Self, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairWithProtectionLevelAsync: usize,
}
impl ::windows_sys::core::Interface for IDeviceInformationPairing {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 742877685, data2: 63108, data3: 16597, data4: [132, 105, 232, 219, 170, 183, 4, 133] };
}
#[repr(C)]
pub struct IDeviceInformationPairing2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DevicePairingProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub Custom: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PairWithProtectionLevelAndSettingsAsync: unsafe extern "system" fn(this: *mut *mut Self, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairWithProtectionLevelAndSettingsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnpairAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnpairAsync: usize,
}
impl ::windows_sys::core::Interface for IDeviceInformationPairing2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4135981821, data2: 2798, data3: 17192, data4: [133, 204, 28, 116, 43, 177, 121, 13] };
}
#[repr(C)]
pub struct IDeviceInformationPairingStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryRegisterForAllInboundPairingRequests: unsafe extern "system" fn(this: *mut *mut Self, pairingkindssupported: DevicePairingKinds, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceInformationPairingStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3910517768, data2: 14036, data3: 18849, data4: [191, 19, 81, 65, 115, 121, 155, 107] };
}
#[repr(C)]
pub struct IDeviceInformationPairingStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryRegisterForAllInboundPairingRequestsWithProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceInformationPairingStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 81679218, data2: 47031, data3: 18283, data4: [167, 79, 197, 131, 106, 112, 77, 152] };
}
#[repr(C)]
pub struct IDeviceInformationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateFromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIdAsyncAdditionalProperties: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, additionalproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIdAsyncAdditionalProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncDeviceClass: unsafe extern "system" fn(this: *mut *mut Self, deviceclass: DeviceClass, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncDeviceClass: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncAqsFilter: unsafe extern "system" fn(this: *mut *mut Self, aqsfilter: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncAqsFilter: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncAqsFilterAndAdditionalProperties: unsafe extern "system" fn(this: *mut *mut Self, aqsfilter: ::windows_sys::core::HSTRING, additionalproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncAqsFilterAndAdditionalProperties: usize,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWatcherDeviceClass: unsafe extern "system" fn(this: *mut *mut Self, deviceclass: DeviceClass, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWatcherAqsFilter: unsafe extern "system" fn(this: *mut *mut Self, aqsfilter: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherAqsFilterAndAdditionalProperties: unsafe extern "system" fn(this: *mut *mut Self, aqsfilter: ::windows_sys::core::HSTRING, additionalproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherAqsFilterAndAdditionalProperties: usize,
}
impl ::windows_sys::core::Interface for IDeviceInformationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3246329870, data2: 14918, data3: 19064, data4: [128, 19, 118, 157, 201, 185, 115, 144] };
}
#[repr(C)]
pub struct IDeviceInformationStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetAqsFilterFromDeviceClass: unsafe extern "system" fn(this: *mut *mut Self, deviceclass: DeviceClass, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIdAsyncWithKindAndAdditionalProperties: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, additionalproperties: *mut ::core::ffi::c_void, kind: DeviceInformationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIdAsyncWithKindAndAdditionalProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncWithKindAqsFilterAndAdditionalProperties: unsafe extern "system" fn(this: *mut *mut Self, aqsfilter: ::windows_sys::core::HSTRING, additionalproperties: *mut ::core::ffi::c_void, kind: DeviceInformationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncWithKindAqsFilterAndAdditionalProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherWithKindAqsFilterAndAdditionalProperties: unsafe extern "system" fn(this: *mut *mut Self, aqsfilter: ::windows_sys::core::HSTRING, additionalproperties: *mut ::core::ffi::c_void, kind: DeviceInformationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherWithKindAqsFilterAndAdditionalProperties: usize,
}
impl ::windows_sys::core::Interface for IDeviceInformationStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1228623668, data2: 43087, data3: 17917, data4: [145, 103, 21, 209, 203, 27, 209, 249] };
}
#[repr(C)]
pub struct IDeviceInformationUpdate {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
impl ::windows_sys::core::Interface for IDeviceInformationUpdate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2402374405, data2: 55666, data3: 17591, data4: [163, 126, 158, 130, 44, 120, 33, 59] };
}
#[repr(C)]
pub struct IDeviceInformationUpdate2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceInformationKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceInformationUpdate2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1570575500, data2: 43123, data3: 18526, data4: [186, 166, 170, 98, 7, 136, 227, 204] };
}
#[repr(C)]
pub struct IDevicePairingRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PairingKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DevicePairingKinds) -> ::windows_sys::core::HRESULT,
    pub Pin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub AcceptWithPin: unsafe extern "system" fn(this: *mut *mut Self, pin: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IDevicePairingRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4145544278, data2: 56939, data3: 18559, data4: [131, 118, 1, 128, 172, 166, 153, 99] };
}
#[repr(C)]
pub struct IDevicePairingRequestedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub AcceptWithPasswordCredential: unsafe extern "system" fn(this: *mut *mut Self, passwordcredential: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    AcceptWithPasswordCredential: usize,
}
impl ::windows_sys::core::Interface for IDevicePairingRequestedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3359068889, data2: 58579, data3: 19888, data4: [163, 96, 161, 5, 228, 55, 219, 220] };
}
#[repr(C)]
pub struct IDevicePairingResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DevicePairingResultStatus) -> ::windows_sys::core::HRESULT,
    pub ProtectionLevelUsed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DevicePairingProtectionLevel) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDevicePairingResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 120259263, data2: 56725, data3: 16421, data4: [155, 55, 222, 81, 173, 186, 55, 183] };
}
#[repr(C)]
pub struct IDevicePairingSettings {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IDevicePairingSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1210888828, data2: 33723, data3: 16910, data4: [190, 81, 102, 2, 178, 34, 222, 84] };
}
#[repr(C)]
pub struct IDevicePicker {
    pub base__: ::windows_sys::core::IInspectable,
    pub Filter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Appearance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestedProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestedProperties: usize,
    #[cfg(feature = "Foundation")]
    pub DeviceSelected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceSelected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceSelected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceSelected: usize,
    #[cfg(feature = "Foundation")]
    pub DisconnectButtonClicked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisconnectButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisconnectButtonClicked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisconnectButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub DevicePickerDismissed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DevicePickerDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDevicePickerDismissed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDevicePickerDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Show: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowWithPlacement: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowWithPlacement: usize,
    #[cfg(feature = "Foundation")]
    pub PickSingleDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleDeviceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub PickSingleDeviceAsyncWithPlacement: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    PickSingleDeviceAsyncWithPlacement: usize,
    pub Hide: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetDisplayStatus: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void, status: ::windows_sys::core::HSTRING, options: DevicePickerDisplayStatusOptions) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDevicePicker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2224650914, data2: 842, data3: 17472, data4: [136, 19, 125, 11, 212, 121, 191, 90] };
}
#[repr(C)]
pub struct IDevicePickerAppearance {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub ForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub AccentColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    AccentColor: usize,
    #[cfg(feature = "UI")]
    pub SetAccentColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetAccentColor: usize,
    #[cfg(feature = "UI")]
    pub SelectedForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SelectedForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetSelectedForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSelectedForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub SelectedBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SelectedBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetSelectedBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSelectedBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SelectedAccentColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SelectedAccentColor: usize,
    #[cfg(feature = "UI")]
    pub SetSelectedAccentColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSelectedAccentColor: usize,
}
impl ::windows_sys::core::Interface for IDevicePickerAppearance {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3868857030, data2: 58919, data3: 20184, data4: [155, 108, 70, 10, 244, 69, 229, 109] };
}
#[repr(C)]
pub struct IDevicePickerFilter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDeviceClasses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDeviceClasses: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDeviceSelectors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDeviceSelectors: usize,
}
impl ::windows_sys::core::Interface for IDevicePickerFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2447086242, data2: 22475, data3: 18673, data4: [155, 89, 165, 155, 122, 31, 2, 162] };
}
#[repr(C)]
pub struct IDeviceSelectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceSelectedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 647944926, data2: 7471, data3: 18752, data4: [132, 2, 65, 86, 184, 29, 60, 119] };
}
#[repr(C)]
pub struct IDeviceUnpairingResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceUnpairingResultStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceUnpairingResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1727285971, data2: 31193, data3: 17483, data4: [146, 207, 169, 46, 247, 37, 113, 199] };
}
#[repr(C)]
pub struct IDeviceWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceWatcherStatus) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3387603325, data2: 36715, data3: 20374, data4: [169, 244, 171, 200, 20, 226, 34, 113] };
}
#[repr(C)]
pub struct IDeviceWatcher2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections"))]
    pub GetBackgroundTrigger: unsafe extern "system" fn(this: *mut *mut Self, requestedeventkinds: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections")))]
    GetBackgroundTrigger: usize,
}
impl ::windows_sys::core::Interface for IDeviceWatcher2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4278732142, data2: 60692, data3: 18921, data4: [154, 105, 129, 23, 197, 74, 233, 113] };
}
#[repr(C)]
pub struct IDeviceWatcherEvent {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceWatcherEventKind) -> ::windows_sys::core::HRESULT,
    pub DeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeviceInformationUpdate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceWatcherEvent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1957338123, data2: 7613, data3: 18429, data4: [182, 53, 60, 197, 86, 208, 255, 139] };
}
#[repr(C)]
pub struct IDeviceWatcherTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceWatcherEvents: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceWatcherEvents: usize,
}
impl ::windows_sys::core::Interface for IDeviceWatcherTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 947945753, data2: 19639, data3: 20055, data4: [165, 109, 119, 109, 7, 203, 254, 249] };
}
#[repr(C)]
pub struct IEnclosureLocation {
    pub base__: ::windows_sys::core::IInspectable,
    pub InDock: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub InLid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Panel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Panel) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnclosureLocation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1110706727, data2: 22544, data3: 17820, data4: [170, 187, 198, 94, 31, 129, 62, 207] };
}
#[repr(C)]
pub struct IEnclosureLocation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RotationAngleInDegreesClockwise: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnclosureLocation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 679844187, data2: 57469, data3: 18525, data4: [138, 158, 189, 242, 154, 239, 79, 102] };
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct Panel(pub i32);
impl Panel {
    pub const Unknown: Self = Self(0i32);
    pub const Front: Self = Self(1i32);
    pub const Back: Self = Self(2i32);
    pub const Top: Self = Self(3i32);
    pub const Bottom: Self = Self(4i32);
    pub const Left: Self = Self(5i32);
    pub const Right: Self = Self(6i32);
}
impl ::core::marker::Copy for Panel {}
impl ::core::clone::Clone for Panel {
    fn clone(&self) -> Self {
        *self
    }
}
