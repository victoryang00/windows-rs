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
#[repr(C)]
pub struct IDeviceAccessChangedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IDeviceAccessInformationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromDeviceClassId: unsafe extern "system" fn(this: *mut *mut Self, deviceclassid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromDeviceClass: unsafe extern "system" fn(this: *mut *mut Self, deviceclass: DeviceClass, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDeviceConnectionChangeTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDeviceDisconnectButtonClickedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IDeviceInformation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceInformationKind) -> ::windows_sys::core::HRESULT,
    pub Pairing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IDeviceInformationPairingStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryRegisterForAllInboundPairingRequests: unsafe extern "system" fn(this: *mut *mut Self, pairingkindssupported: DevicePairingKinds, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDeviceInformationPairingStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryRegisterForAllInboundPairingRequestsWithProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IDeviceInformationUpdate {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IDeviceInformationUpdate2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceInformationKind) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IDevicePairingRequestedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub AcceptWithPasswordCredential: unsafe extern "system" fn(this: *mut *mut Self, passwordcredential: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    AcceptWithPasswordCredential: usize,
}
#[repr(C)]
pub struct IDevicePairingResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DevicePairingResultStatus) -> ::windows_sys::core::HRESULT,
    pub ProtectionLevelUsed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DevicePairingProtectionLevel) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDevicePairingSettings {
    pub base__: ::windows_sys::core::IInspectable,
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
#[repr(C)]
pub struct IDeviceSelectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDeviceUnpairingResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceUnpairingResultStatus) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IDeviceWatcher2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections"))]
    pub GetBackgroundTrigger: unsafe extern "system" fn(this: *mut *mut Self, requestedeventkinds: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections")))]
    GetBackgroundTrigger: usize,
}
#[repr(C)]
pub struct IDeviceWatcherEvent {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceWatcherEventKind) -> ::windows_sys::core::HRESULT,
    pub DeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeviceInformationUpdate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDeviceWatcherTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceWatcherEvents: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceWatcherEvents: usize,
}
#[repr(C)]
pub struct IEnclosureLocation {
    pub base__: ::windows_sys::core::IInspectable,
    pub InDock: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub InLid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Panel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Panel) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnclosureLocation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RotationAngleInDegreesClockwise: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
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
