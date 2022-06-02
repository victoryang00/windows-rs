#[cfg(feature = "Devices_Bluetooth_Advertisement")]
pub mod Advertisement;
#[cfg(feature = "Devices_Bluetooth_Background")]
pub mod Background;
#[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
pub mod GenericAttributeProfile;
#[cfg(feature = "Devices_Bluetooth_Rfcomm")]
pub mod Rfcomm;
pub type BluetoothAdapter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothAddressType(pub i32);
impl BluetoothAddressType {
    pub const Public: Self = Self(0i32);
    pub const Random: Self = Self(1i32);
    pub const Unspecified: Self = Self(2i32);
}
impl ::core::marker::Copy for BluetoothAddressType {}
impl ::core::clone::Clone for BluetoothAddressType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothCacheMode(pub i32);
impl BluetoothCacheMode {
    pub const Cached: Self = Self(0i32);
    pub const Uncached: Self = Self(1i32);
}
impl ::core::marker::Copy for BluetoothCacheMode {}
impl ::core::clone::Clone for BluetoothCacheMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BluetoothClassOfDevice = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothConnectionStatus(pub i32);
impl BluetoothConnectionStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
}
impl ::core::marker::Copy for BluetoothConnectionStatus {}
impl ::core::clone::Clone for BluetoothConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BluetoothDevice = *mut ::core::ffi::c_void;
pub type BluetoothDeviceId = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothError(pub i32);
impl BluetoothError {
    pub const Success: Self = Self(0i32);
    pub const RadioNotAvailable: Self = Self(1i32);
    pub const ResourceInUse: Self = Self(2i32);
    pub const DeviceNotConnected: Self = Self(3i32);
    pub const OtherError: Self = Self(4i32);
    pub const DisabledByPolicy: Self = Self(5i32);
    pub const NotSupported: Self = Self(6i32);
    pub const DisabledByUser: Self = Self(7i32);
    pub const ConsentRequired: Self = Self(8i32);
    pub const TransportNotSupported: Self = Self(9i32);
}
impl ::core::marker::Copy for BluetoothError {}
impl ::core::clone::Clone for BluetoothError {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BluetoothLEAppearance = *mut ::core::ffi::c_void;
pub type BluetoothLEConnectionParameters = *mut ::core::ffi::c_void;
pub type BluetoothLEConnectionPhy = *mut ::core::ffi::c_void;
pub type BluetoothLEConnectionPhyInfo = *mut ::core::ffi::c_void;
pub type BluetoothLEDevice = *mut ::core::ffi::c_void;
pub type BluetoothLEPreferredConnectionParameters = *mut ::core::ffi::c_void;
pub type BluetoothLEPreferredConnectionParametersRequest = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParametersRequestStatus(pub i32);
impl BluetoothLEPreferredConnectionParametersRequestStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const DeviceNotAvailable: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
}
impl ::core::marker::Copy for BluetoothLEPreferredConnectionParametersRequestStatus {}
impl ::core::clone::Clone for BluetoothLEPreferredConnectionParametersRequestStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothMajorClass(pub i32);
impl BluetoothMajorClass {
    pub const Miscellaneous: Self = Self(0i32);
    pub const Computer: Self = Self(1i32);
    pub const Phone: Self = Self(2i32);
    pub const NetworkAccessPoint: Self = Self(3i32);
    pub const AudioVideo: Self = Self(4i32);
    pub const Peripheral: Self = Self(5i32);
    pub const Imaging: Self = Self(6i32);
    pub const Wearable: Self = Self(7i32);
    pub const Toy: Self = Self(8i32);
    pub const Health: Self = Self(9i32);
}
impl ::core::marker::Copy for BluetoothMajorClass {}
impl ::core::clone::Clone for BluetoothMajorClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothMinorClass(pub i32);
impl BluetoothMinorClass {
    pub const Uncategorized: Self = Self(0i32);
    pub const ComputerDesktop: Self = Self(1i32);
    pub const ComputerServer: Self = Self(2i32);
    pub const ComputerLaptop: Self = Self(3i32);
    pub const ComputerHandheld: Self = Self(4i32);
    pub const ComputerPalmSize: Self = Self(5i32);
    pub const ComputerWearable: Self = Self(6i32);
    pub const ComputerTablet: Self = Self(7i32);
    pub const PhoneCellular: Self = Self(1i32);
    pub const PhoneCordless: Self = Self(2i32);
    pub const PhoneSmartPhone: Self = Self(3i32);
    pub const PhoneWired: Self = Self(4i32);
    pub const PhoneIsdn: Self = Self(5i32);
    pub const NetworkFullyAvailable: Self = Self(0i32);
    pub const NetworkUsed01To17Percent: Self = Self(8i32);
    pub const NetworkUsed17To33Percent: Self = Self(16i32);
    pub const NetworkUsed33To50Percent: Self = Self(24i32);
    pub const NetworkUsed50To67Percent: Self = Self(32i32);
    pub const NetworkUsed67To83Percent: Self = Self(40i32);
    pub const NetworkUsed83To99Percent: Self = Self(48i32);
    pub const NetworkNoServiceAvailable: Self = Self(56i32);
    pub const AudioVideoWearableHeadset: Self = Self(1i32);
    pub const AudioVideoHandsFree: Self = Self(2i32);
    pub const AudioVideoMicrophone: Self = Self(4i32);
    pub const AudioVideoLoudspeaker: Self = Self(5i32);
    pub const AudioVideoHeadphones: Self = Self(6i32);
    pub const AudioVideoPortableAudio: Self = Self(7i32);
    pub const AudioVideoCarAudio: Self = Self(8i32);
    pub const AudioVideoSetTopBox: Self = Self(9i32);
    pub const AudioVideoHifiAudioDevice: Self = Self(10i32);
    pub const AudioVideoVcr: Self = Self(11i32);
    pub const AudioVideoVideoCamera: Self = Self(12i32);
    pub const AudioVideoCamcorder: Self = Self(13i32);
    pub const AudioVideoVideoMonitor: Self = Self(14i32);
    pub const AudioVideoVideoDisplayAndLoudspeaker: Self = Self(15i32);
    pub const AudioVideoVideoConferencing: Self = Self(16i32);
    pub const AudioVideoGamingOrToy: Self = Self(18i32);
    pub const PeripheralJoystick: Self = Self(1i32);
    pub const PeripheralGamepad: Self = Self(2i32);
    pub const PeripheralRemoteControl: Self = Self(3i32);
    pub const PeripheralSensing: Self = Self(4i32);
    pub const PeripheralDigitizerTablet: Self = Self(5i32);
    pub const PeripheralCardReader: Self = Self(6i32);
    pub const PeripheralDigitalPen: Self = Self(7i32);
    pub const PeripheralHandheldScanner: Self = Self(8i32);
    pub const PeripheralHandheldGesture: Self = Self(9i32);
    pub const WearableWristwatch: Self = Self(1i32);
    pub const WearablePager: Self = Self(2i32);
    pub const WearableJacket: Self = Self(3i32);
    pub const WearableHelmet: Self = Self(4i32);
    pub const WearableGlasses: Self = Self(5i32);
    pub const ToyRobot: Self = Self(1i32);
    pub const ToyVehicle: Self = Self(2i32);
    pub const ToyDoll: Self = Self(3i32);
    pub const ToyController: Self = Self(4i32);
    pub const ToyGame: Self = Self(5i32);
    pub const HealthBloodPressureMonitor: Self = Self(1i32);
    pub const HealthThermometer: Self = Self(2i32);
    pub const HealthWeighingScale: Self = Self(3i32);
    pub const HealthGlucoseMeter: Self = Self(4i32);
    pub const HealthPulseOximeter: Self = Self(5i32);
    pub const HealthHeartRateMonitor: Self = Self(6i32);
    pub const HealthHealthDataDisplay: Self = Self(7i32);
    pub const HealthStepCounter: Self = Self(8i32);
    pub const HealthBodyCompositionAnalyzer: Self = Self(9i32);
    pub const HealthPeakFlowMonitor: Self = Self(10i32);
    pub const HealthMedicationMonitor: Self = Self(11i32);
    pub const HealthKneeProsthesis: Self = Self(12i32);
    pub const HealthAnkleProsthesis: Self = Self(13i32);
    pub const HealthGenericHealthManager: Self = Self(14i32);
    pub const HealthPersonalMobilityDevice: Self = Self(15i32);
}
impl ::core::marker::Copy for BluetoothMinorClass {}
impl ::core::clone::Clone for BluetoothMinorClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothServiceCapabilities(pub u32);
impl BluetoothServiceCapabilities {
    pub const None: Self = Self(0u32);
    pub const LimitedDiscoverableMode: Self = Self(1u32);
    pub const PositioningService: Self = Self(8u32);
    pub const NetworkingService: Self = Self(16u32);
    pub const RenderingService: Self = Self(32u32);
    pub const CapturingService: Self = Self(64u32);
    pub const ObjectTransferService: Self = Self(128u32);
    pub const AudioService: Self = Self(256u32);
    pub const TelephoneService: Self = Self(512u32);
    pub const InformationService: Self = Self(1024u32);
}
impl ::core::marker::Copy for BluetoothServiceCapabilities {}
impl ::core::clone::Clone for BluetoothServiceCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BluetoothSignalStrengthFilter = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBluetoothAdapter {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BluetoothAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub IsClassicSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsLowEnergySupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPeripheralRoleSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCentralRoleSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsAdvertisementOffloadSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Devices_Radios", feature = "Foundation"))]
    pub GetRadioAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Radios", feature = "Foundation")))]
    GetRadioAsync: usize,
}
#[repr(C)]
pub struct IBluetoothAdapter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreClassicSecureConnectionsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AreLowEnergySecureConnectionsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothAdapter3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsExtendedAdvertisingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MaxAdvertisementDataLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothAdapterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[repr(C)]
pub struct IBluetoothClassOfDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub RawValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MajorClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BluetoothMajorClass) -> ::windows_sys::core::HRESULT,
    pub MinorClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BluetoothMinorClass) -> ::windows_sys::core::HRESULT,
    pub ServiceCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BluetoothServiceCapabilities) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothClassOfDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromRawValue: unsafe extern "system" fn(this: *mut *mut Self, rawvalue: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromParts: unsafe extern "system" fn(this: *mut *mut Self, majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Networking")]
    pub HostName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    HostName: usize,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ClassOfDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub SdpRecords: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    SdpRecords: usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections", feature = "deprecated"))]
    pub RfcommServices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections", feature = "deprecated")))]
    RfcommServices: usize,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BluetoothConnectionStatus) -> ::windows_sys::core::HRESULT,
    pub BluetoothAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NameChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNameChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SdpRecordsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SdpRecordsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSdpRecordsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSdpRecordsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionStatusChanged: usize,
}
#[repr(C)]
pub struct IBluetoothDevice2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
#[repr(C)]
pub struct IBluetoothDevice3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceAccessInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceAccessInformation: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    RequestAccessAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub GetRfcommServicesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))]
    GetRfcommServicesAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub GetRfcommServicesWithCacheModeAsync: unsafe extern "system" fn(this: *mut *mut Self, cachemode: BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))]
    GetRfcommServicesWithCacheModeAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub GetRfcommServicesForIdAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))]
    GetRfcommServicesForIdAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub GetRfcommServicesForIdWithCacheModeAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceid: *mut ::core::ffi::c_void, cachemode: BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))]
    GetRfcommServicesForIdWithCacheModeAsync: usize,
}
#[repr(C)]
pub struct IBluetoothDevice4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BluetoothDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothDevice5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub WasSecureConnectionUsedForPairing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothDeviceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsClassicDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsLowEnergyDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothDeviceIdStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub FromHostNameAsync: unsafe extern "system" fn(this: *mut *mut Self, hostname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking")))]
    FromHostNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromBluetoothAddressAsync: unsafe extern "system" fn(this: *mut *mut Self, address: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromBluetoothAddressAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothDeviceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelectorFromPairingState: unsafe extern "system" fn(this: *mut *mut Self, pairingstate: bool, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromConnectionStatus: unsafe extern "system" fn(this: *mut *mut Self, connectionstatus: BluetoothConnectionStatus, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromDeviceName: unsafe extern "system" fn(this: *mut *mut Self, devicename: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromBluetoothAddress: unsafe extern "system" fn(this: *mut *mut Self, bluetoothaddress: u64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromClassOfDevice: unsafe extern "system" fn(this: *mut *mut Self, classofdevice: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEAppearance {
    pub base__: ::windows_sys::core::IInspectable,
    pub RawValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Category: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SubCategory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEAppearanceCategoriesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Uncategorized: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Phone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Computer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Watch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Clock: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Display: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub RemoteControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub EyeGlasses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Keyring: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub MediaPlayer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub BarcodeScanner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Thermometer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub HeartRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub BloodPressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub HumanInterfaceDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GlucoseMeter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub RunningWalking: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Cycling: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub PulseOximeter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub WeightScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub OutdoorSportActivity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEAppearanceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromRawValue: unsafe extern "system" fn(this: *mut *mut Self, rawvalue: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromParts: unsafe extern "system" fn(this: *mut *mut Self, appearancecategory: u16, appearancesubcategory: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEAppearanceSubcategoriesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Generic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SportsWatch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub ThermometerEar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub HeartRateBelt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub BloodPressureArm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub BloodPressureWrist: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Keyboard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Mouse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Joystick: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Gamepad: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub DigitizerTablet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub CardReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub DigitalPen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub BarcodeScanner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub RunningWalkingInShoe: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub RunningWalkingOnShoe: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub RunningWalkingOnHip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub CyclingComputer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub CyclingSpeedSensor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub CyclingCadenceSensor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub CyclingPowerSensor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub CyclingSpeedCadenceSensor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub OximeterFingertip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub OximeterWristWorn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub LocationDisplay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub LocationNavigationDisplay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub LocationPod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub LocationNavigationPod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEConnectionParameters {
    pub base__: ::windows_sys::core::IInspectable,
    pub LinkTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub ConnectionLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub ConnectionInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEConnectionPhy {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransmitInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReceiveInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEConnectionPhyInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsUncoded1MPhy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsUncoded2MPhy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCodedPhy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections", feature = "deprecated"))]
    pub GattServices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections", feature = "deprecated")))]
    GattServices: usize,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BluetoothConnectionStatus) -> ::windows_sys::core::HRESULT,
    pub BluetoothAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "deprecated"))]
    pub GetGattService: unsafe extern "system" fn(this: *mut *mut Self, serviceuuid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "deprecated")))]
    GetGattService: usize,
    #[cfg(feature = "Foundation")]
    pub NameChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNameChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub GattServicesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GattServicesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGattServicesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGattServicesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionStatusChanged: usize,
}
#[repr(C)]
pub struct IBluetoothLEDevice2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
    pub Appearance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BluetoothAddressType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BluetoothAddressType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEDevice3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceAccessInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceAccessInformation: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    RequestAccessAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub GetGattServicesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))]
    GetGattServicesAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub GetGattServicesWithCacheModeAsync: unsafe extern "system" fn(this: *mut *mut Self, cachemode: BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))]
    GetGattServicesWithCacheModeAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub GetGattServicesForUuidAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceuuid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))]
    GetGattServicesForUuidAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub GetGattServicesForUuidWithCacheModeAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceuuid: ::windows_sys::core::GUID, cachemode: BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))]
    GetGattServicesForUuidWithCacheModeAsync: usize,
}
#[repr(C)]
pub struct IBluetoothLEDevice4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BluetoothDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEDevice5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub WasSecureConnectionUsedForPairing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEDevice6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetConnectionParameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetConnectionPhy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RequestPreferredConnectionParameters: unsafe extern "system" fn(this: *mut *mut Self, preferredconnectionparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectionParametersChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionParametersChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionParametersChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionParametersChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionPhyChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionPhyChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionPhyChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionPhyChanged: usize,
}
#[repr(C)]
pub struct IBluetoothLEDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromBluetoothAddressAsync: unsafe extern "system" fn(this: *mut *mut Self, bluetoothaddress: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromBluetoothAddressAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEDeviceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelectorFromPairingState: unsafe extern "system" fn(this: *mut *mut Self, pairingstate: bool, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromConnectionStatus: unsafe extern "system" fn(this: *mut *mut Self, connectionstatus: BluetoothConnectionStatus, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromDeviceName: unsafe extern "system" fn(this: *mut *mut Self, devicename: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromBluetoothAddress: unsafe extern "system" fn(this: *mut *mut Self, bluetoothaddress: u64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType: unsafe extern "system" fn(this: *mut *mut Self, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromAppearance: unsafe extern "system" fn(this: *mut *mut Self, appearance: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromBluetoothAddressWithBluetoothAddressTypeAsync: unsafe extern "system" fn(this: *mut *mut Self, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromBluetoothAddressWithBluetoothAddressTypeAsync: usize,
}
#[repr(C)]
pub struct IBluetoothLEPreferredConnectionParameters {
    pub base__: ::windows_sys::core::IInspectable,
    pub LinkTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub ConnectionLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub MinConnectionInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub MaxConnectionInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEPreferredConnectionParametersRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BluetoothLEPreferredConnectionParametersRequestStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEPreferredConnectionParametersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Balanced: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ThroughputOptimized: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PowerOptimized: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothSignalStrengthFilter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub InRangeThresholdInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InRangeThresholdInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub SetInRangeThresholdInDBm: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInRangeThresholdInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub OutOfRangeThresholdInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OutOfRangeThresholdInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub SetOutOfRangeThresholdInDBm: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOutOfRangeThresholdInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub OutOfRangeTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OutOfRangeTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetOutOfRangeTimeout: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOutOfRangeTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SamplingInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SamplingInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetSamplingInterval: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSamplingInterval: usize,
}
#[repr(C)]
pub struct IBluetoothUuidHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromShortId: unsafe extern "system" fn(this: *mut *mut Self, shortid: u32, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryGetShortId: unsafe extern "system" fn(this: *mut *mut Self, uuid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetShortId: usize,
}
