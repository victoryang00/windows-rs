pub type GattCharacteristic = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattCharacteristicProperties(pub u32);
impl GattCharacteristicProperties {
    pub const None: Self = Self(0u32);
    pub const Broadcast: Self = Self(1u32);
    pub const Read: Self = Self(2u32);
    pub const WriteWithoutResponse: Self = Self(4u32);
    pub const Write: Self = Self(8u32);
    pub const Notify: Self = Self(16u32);
    pub const Indicate: Self = Self(32u32);
    pub const AuthenticatedSignedWrites: Self = Self(64u32);
    pub const ExtendedProperties: Self = Self(128u32);
    pub const ReliableWrites: Self = Self(256u32);
    pub const WritableAuxiliaries: Self = Self(512u32);
}
impl ::core::marker::Copy for GattCharacteristicProperties {}
impl ::core::clone::Clone for GattCharacteristicProperties {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GattCharacteristicsResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattClientCharacteristicConfigurationDescriptorValue(pub i32);
impl GattClientCharacteristicConfigurationDescriptorValue {
    pub const None: Self = Self(0i32);
    pub const Notify: Self = Self(1i32);
    pub const Indicate: Self = Self(2i32);
}
impl ::core::marker::Copy for GattClientCharacteristicConfigurationDescriptorValue {}
impl ::core::clone::Clone for GattClientCharacteristicConfigurationDescriptorValue {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GattClientNotificationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattCommunicationStatus(pub i32);
impl GattCommunicationStatus {
    pub const Success: Self = Self(0i32);
    pub const Unreachable: Self = Self(1i32);
    pub const ProtocolError: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
}
impl ::core::marker::Copy for GattCommunicationStatus {}
impl ::core::clone::Clone for GattCommunicationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GattDescriptor = *mut ::core::ffi::c_void;
pub type GattDescriptorsResult = *mut ::core::ffi::c_void;
pub type GattDeviceService = *mut ::core::ffi::c_void;
pub type GattDeviceServicesResult = *mut ::core::ffi::c_void;
pub type GattLocalCharacteristic = *mut ::core::ffi::c_void;
pub type GattLocalCharacteristicParameters = *mut ::core::ffi::c_void;
pub type GattLocalCharacteristicResult = *mut ::core::ffi::c_void;
pub type GattLocalDescriptor = *mut ::core::ffi::c_void;
pub type GattLocalDescriptorParameters = *mut ::core::ffi::c_void;
pub type GattLocalDescriptorResult = *mut ::core::ffi::c_void;
pub type GattLocalService = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattOpenStatus(pub i32);
impl GattOpenStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AlreadyOpened: Self = Self(2i32);
    pub const NotFound: Self = Self(3i32);
    pub const SharingViolation: Self = Self(4i32);
    pub const AccessDenied: Self = Self(5i32);
}
impl ::core::marker::Copy for GattOpenStatus {}
impl ::core::clone::Clone for GattOpenStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GattPresentationFormat = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattProtectionLevel(pub i32);
impl GattProtectionLevel {
    pub const Plain: Self = Self(0i32);
    pub const AuthenticationRequired: Self = Self(1i32);
    pub const EncryptionRequired: Self = Self(2i32);
    pub const EncryptionAndAuthenticationRequired: Self = Self(3i32);
}
impl ::core::marker::Copy for GattProtectionLevel {}
impl ::core::clone::Clone for GattProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GattReadClientCharacteristicConfigurationDescriptorResult = *mut ::core::ffi::c_void;
pub type GattReadRequest = *mut ::core::ffi::c_void;
pub type GattReadRequestedEventArgs = *mut ::core::ffi::c_void;
pub type GattReadResult = *mut ::core::ffi::c_void;
pub type GattReliableWriteTransaction = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattRequestState(pub i32);
impl GattRequestState {
    pub const Pending: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for GattRequestState {}
impl ::core::clone::Clone for GattRequestState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GattRequestStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type GattServiceProvider = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattServiceProviderAdvertisementStatus(pub i32);
impl GattServiceProviderAdvertisementStatus {
    pub const Created: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
    pub const StartedWithoutAllAdvertisementData: Self = Self(4i32);
}
impl ::core::marker::Copy for GattServiceProviderAdvertisementStatus {}
impl ::core::clone::Clone for GattServiceProviderAdvertisementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GattServiceProviderAdvertisementStatusChangedEventArgs = *mut ::core::ffi::c_void;
pub type GattServiceProviderAdvertisingParameters = *mut ::core::ffi::c_void;
pub type GattServiceProviderResult = *mut ::core::ffi::c_void;
pub type GattSession = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattSessionStatus(pub i32);
impl GattSessionStatus {
    pub const Closed: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
}
impl ::core::marker::Copy for GattSessionStatus {}
impl ::core::clone::Clone for GattSessionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GattSessionStatusChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattSharingMode(pub i32);
impl GattSharingMode {
    pub const Unspecified: Self = Self(0i32);
    pub const Exclusive: Self = Self(1i32);
    pub const SharedReadOnly: Self = Self(2i32);
    pub const SharedReadAndWrite: Self = Self(3i32);
}
impl ::core::marker::Copy for GattSharingMode {}
impl ::core::clone::Clone for GattSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GattSubscribedClient = *mut ::core::ffi::c_void;
pub type GattValueChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattWriteOption(pub i32);
impl GattWriteOption {
    pub const WriteWithResponse: Self = Self(0i32);
    pub const WriteWithoutResponse: Self = Self(1i32);
}
impl ::core::marker::Copy for GattWriteOption {}
impl ::core::clone::Clone for GattWriteOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GattWriteRequest = *mut ::core::ffi::c_void;
pub type GattWriteRequestedEventArgs = *mut ::core::ffi::c_void;
pub type GattWriteResult = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IGattCharacteristic {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetDescriptors: unsafe extern "system" fn(this: *mut *mut Self, descriptoruuid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetDescriptors: usize,
    pub CharacteristicProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattCharacteristicProperties) -> ::windows_sys::core::HRESULT,
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub SetProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, value: GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub UserDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Uuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AttributeHandle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresentationFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresentationFormats: usize,
    #[cfg(feature = "Foundation")]
    pub ReadValueAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadValueAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReadValueWithCacheModeAsync: unsafe extern "system" fn(this: *mut *mut Self, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadValueWithCacheModeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteValueAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteValueAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteValueWithOptionAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, writeoption: GattWriteOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteValueWithOptionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReadClientCharacteristicConfigurationDescriptorAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadClientCharacteristicConfigurationDescriptorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub WriteClientCharacteristicConfigurationDescriptorAsync: unsafe extern "system" fn(this: *mut *mut Self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteClientCharacteristicConfigurationDescriptorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ValueChanged: unsafe extern "system" fn(this: *mut *mut Self, valuechangedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveValueChanged: unsafe extern "system" fn(this: *mut *mut Self, valuechangedeventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveValueChanged: usize,
}
impl ::windows_sys::core::Interface for IGattCharacteristic {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1506496705, data2: 22836, data3: 20328, data4: [161, 152, 235, 134, 79, 164, 78, 107] };
}
#[repr(C)]
pub struct IGattCharacteristic2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Service: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetAllDescriptors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetAllDescriptors: usize,
}
impl ::windows_sys::core::Interface for IGattCharacteristic2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2920985976, data2: 60422, data3: 18276, data4: [183, 128, 152, 53, 161, 211, 93, 110] };
}
#[repr(C)]
pub struct IGattCharacteristic3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDescriptorsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDescriptorsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDescriptorsWithCacheModeAsync: unsafe extern "system" fn(this: *mut *mut Self, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDescriptorsWithCacheModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDescriptorsForUuidAsync: unsafe extern "system" fn(this: *mut *mut Self, descriptoruuid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDescriptorsForUuidAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDescriptorsForUuidWithCacheModeAsync: unsafe extern "system" fn(this: *mut *mut Self, descriptoruuid: ::windows_sys::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDescriptorsForUuidWithCacheModeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteValueWithResultAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteValueWithResultAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteValueWithResultAndOptionAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, writeoption: GattWriteOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteValueWithResultAndOptionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub WriteClientCharacteristicConfigurationDescriptorWithResultAsync: unsafe extern "system" fn(this: *mut *mut Self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteClientCharacteristicConfigurationDescriptorWithResultAsync: usize,
}
impl ::windows_sys::core::Interface for IGattCharacteristic3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1060922942, data2: 37844, data3: 16491, data4: [184, 23, 219, 129, 248, 237, 83, 179] };
}
#[repr(C)]
pub struct IGattCharacteristicStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub ConvertShortIdToUuid: unsafe extern "system" fn(this: *mut *mut Self, shortid: u16, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertShortIdToUuid: usize,
}
impl ::windows_sys::core::Interface for IGattCharacteristicStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1506496707, data2: 22836, data3: 20328, data4: [161, 152, 235, 134, 79, 164, 78, 107] };
}
#[repr(C)]
pub struct IGattCharacteristicUuidsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub BatteryLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub BloodPressureFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub BloodPressureMeasurement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub BodySensorLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CscFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CscMeasurement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GlucoseFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GlucoseMeasurement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GlucoseMeasurementContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub HeartRateControlPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub HeartRateMeasurement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub IntermediateCuffPressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub IntermediateTemperature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub MeasurementInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub RecordAccessControlPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub RscFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub RscMeasurement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SCControlPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SensorLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub TemperatureMeasurement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub TemperatureType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattCharacteristicUuidsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1492796806, data2: 45534, data3: 18188, data4: [183, 222, 13, 17, 255, 68, 244, 183] };
}
#[repr(C)]
pub struct IGattCharacteristicUuidsStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlertCategoryId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AlertCategoryIdBitMask: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AlertLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AlertNotificationControlPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AlertStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GapAppearance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub BootKeyboardInputReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub BootKeyboardOutputReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub BootMouseInputReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CurrentTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CyclingPowerControlPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CyclingPowerFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CyclingPowerMeasurement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CyclingPowerVector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub DateTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub DayDateTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub DayOfWeek: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GapDeviceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub DstOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ExactTime256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub FirmwareRevisionString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub HardwareRevisionString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub HidControlPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub HidInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Ieee1107320601RegulatoryCertificationDataList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub LnControlPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub LnFeature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub LocalTimeInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub LocationAndSpeed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ManufacturerNameString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ModelNumberString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Navigation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub NewAlert: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GapPeripheralPreferredConnectionParameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GapPeripheralPrivacyFlag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PnpId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PositionQuality: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ProtocolMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GapReconnectionAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ReferenceTimeInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Report: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ReportMap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub RingerControlPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub RingerSetting: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ScanIntervalWindow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ScanRefresh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SerialNumberString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GattServiceChanged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SoftwareRevisionString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SupportedNewAlertCategory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SupportUnreadAlertCategory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub TimeAccuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub TimeSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub TimeUpdateControlPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub TimeUpdateState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub TimeWithDst: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub TimeZone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub TxPowerLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub UnreadAlertStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattCharacteristicUuidsStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408269861, data2: 54382, data3: 18988, data4: [156, 63, 237, 109, 234, 41, 231, 190] };
}
#[repr(C)]
pub struct IGattCharacteristicsResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattCommunicationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
impl ::windows_sys::core::Interface for IGattCharacteristicsResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 294949980, data2: 45655, data3: 20286, data4: [157, 183, 246, 139, 201, 169, 174, 242] };
}
#[repr(C)]
pub struct IGattClientNotificationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub SubscribedClient: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattCommunicationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
}
impl ::windows_sys::core::Interface for IGattClientNotificationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1349342617, data2: 274, data3: 16794, data4: [142, 59, 174, 33, 175, 171, 210, 194] };
}
#[repr(C)]
pub struct IGattClientNotificationResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BytesSent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattClientNotificationResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2410595479, data2: 17888, data3: 18814, data4: [149, 130, 41, 161, 254, 40, 26, 213] };
}
#[repr(C)]
pub struct IGattDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub SetProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, value: GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub Uuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AttributeHandle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadValueAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadValueAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReadValueWithCacheModeAsync: unsafe extern "system" fn(this: *mut *mut Self, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadValueWithCacheModeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteValueAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteValueAsync: usize,
}
impl ::windows_sys::core::Interface for IGattDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2449825579, data2: 32900, data3: 17220, data4: [180, 194, 40, 77, 225, 154, 133, 6] };
}
#[repr(C)]
pub struct IGattDescriptor2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteValueWithResultAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteValueWithResultAsync: usize,
}
impl ::windows_sys::core::Interface for IGattDescriptor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2404793657, data2: 54832, data3: 16492, data4: [186, 17, 16, 205, 209, 107, 14, 94] };
}
#[repr(C)]
pub struct IGattDescriptorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub ConvertShortIdToUuid: unsafe extern "system" fn(this: *mut *mut Self, shortid: u16, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertShortIdToUuid: usize,
}
impl ::windows_sys::core::Interface for IGattDescriptorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2449825581, data2: 32900, data3: 17220, data4: [180, 194, 40, 77, 225, 154, 133, 6] };
}
#[repr(C)]
pub struct IGattDescriptorUuidsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CharacteristicAggregateFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CharacteristicExtendedProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CharacteristicPresentationFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CharacteristicUserDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ClientCharacteristicConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ServerCharacteristicConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattDescriptorUuidsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2801296078, data2: 40188, data3: 17137, data4: [145, 133, 255, 55, 183, 81, 129, 211] };
}
#[repr(C)]
pub struct IGattDescriptorsResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattCommunicationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Descriptors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Descriptors: usize,
}
impl ::windows_sys::core::Interface for IGattDescriptorsResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2613088755, data2: 38375, data3: 17545, data4: [141, 37, 255, 129, 149, 90, 87, 185] };
}
#[repr(C)]
pub struct IGattDeviceService {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetCharacteristics: unsafe extern "system" fn(this: *mut *mut Self, characteristicuuid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetCharacteristics: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetIncludedServices: unsafe extern "system" fn(this: *mut *mut Self, serviceuuid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetIncludedServices: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Uuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AttributeHandle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattDeviceService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2893773829, data2: 45884, data3: 18383, data4: [153, 15, 107, 143, 85, 119, 223, 113] };
}
#[repr(C)]
pub struct IGattDeviceService2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Device: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ParentServices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ParentServices: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetAllCharacteristics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetAllCharacteristics: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetAllIncludedServices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetAllIncludedServices: usize,
}
impl ::windows_sys::core::Interface for IGattDeviceService2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4233384459, data2: 2829, data3: 18184, data4: [186, 224, 159, 253, 148, 137, 188, 89] };
}
#[repr(C)]
pub struct IGattDeviceService3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceAccessInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceAccessInformation: usize,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattSharingMode) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut *mut Self, sharingmode: GattSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCharacteristicsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCharacteristicsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCharacteristicsWithCacheModeAsync: unsafe extern "system" fn(this: *mut *mut Self, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCharacteristicsWithCacheModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCharacteristicsForUuidAsync: unsafe extern "system" fn(this: *mut *mut Self, characteristicuuid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCharacteristicsForUuidAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCharacteristicsForUuidWithCacheModeAsync: unsafe extern "system" fn(this: *mut *mut Self, characteristicuuid: ::windows_sys::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCharacteristicsForUuidWithCacheModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIncludedServicesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIncludedServicesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIncludedServicesWithCacheModeAsync: unsafe extern "system" fn(this: *mut *mut Self, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIncludedServicesWithCacheModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIncludedServicesForUuidAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceuuid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIncludedServicesForUuidAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIncludedServicesForUuidWithCacheModeAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceuuid: ::windows_sys::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIncludedServicesForUuidWithCacheModeAsync: usize,
}
impl ::windows_sys::core::Interface for IGattDeviceService3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2996021584, data2: 3155, data3: 17276, data4: [169, 179, 92, 50, 16, 198, 229, 105] };
}
#[repr(C)]
pub struct IGattDeviceServiceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelectorFromUuid: unsafe extern "system" fn(this: *mut *mut Self, serviceuuid: ::windows_sys::core::GUID, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub GetDeviceSelectorFromShortId: unsafe extern "system" fn(this: *mut *mut Self, serviceshortid: u16, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDeviceSelectorFromShortId: usize,
    #[cfg(feature = "deprecated")]
    pub ConvertShortIdToUuid: unsafe extern "system" fn(this: *mut *mut Self, shortid: u16, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertShortIdToUuid: usize,
}
impl ::windows_sys::core::Interface for IGattDeviceServiceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 426573858, data2: 64173, data3: 17884, data4: [174, 91, 42, 195, 24, 78, 132, 219] };
}
#[repr(C)]
pub struct IGattDeviceServiceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdWithSharingModeAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, sharingmode: GattSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdWithSharingModeAsync: usize,
    pub GetDeviceSelectorForBluetoothDeviceId: unsafe extern "system" fn(this: *mut *mut Self, bluetoothdeviceid: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceIdWithCacheMode: unsafe extern "system" fn(this: *mut *mut Self, bluetoothdeviceid: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceIdAndUuid: unsafe extern "system" fn(this: *mut *mut Self, bluetoothdeviceid: *mut ::core::ffi::c_void, serviceuuid: ::windows_sys::core::GUID, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode: unsafe extern "system" fn(this: *mut *mut Self, bluetoothdeviceid: *mut ::core::ffi::c_void, serviceuuid: ::windows_sys::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattDeviceServiceStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 100931694, data2: 9382, data3: 19213, data4: [162, 242, 48, 204, 1, 84, 93, 37] };
}
#[repr(C)]
pub struct IGattDeviceServicesResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattCommunicationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Services: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Services: usize,
}
impl ::windows_sys::core::Interface for IGattDeviceServicesResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 387830766, data2: 365, data3: 16797, data4: [131, 138, 87, 108, 244, 117, 163, 216] };
}
#[repr(C)]
pub struct IGattLocalCharacteristic {
    pub base__: ::windows_sys::core::IInspectable,
    pub Uuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub StaticValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StaticValue: usize,
    pub CharacteristicProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattCharacteristicProperties) -> ::windows_sys::core::HRESULT,
    pub ReadProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub WriteProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateDescriptorAsync: unsafe extern "system" fn(this: *mut *mut Self, descriptoruuid: ::windows_sys::core::GUID, parameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDescriptorAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Descriptors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Descriptors: usize,
    pub UserDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresentationFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresentationFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SubscribedClients: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SubscribedClients: usize,
    #[cfg(feature = "Foundation")]
    pub SubscribedClientsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubscribedClientsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSubscribedClientsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSubscribedClientsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ReadRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadRequested: usize,
    #[cfg(feature = "Foundation")]
    pub WriteRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWriteRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWriteRequested: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub NotifyValueAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    NotifyValueAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub NotifyValueForSubscribedClientAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, subscribedclient: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    NotifyValueForSubscribedClientAsync: usize,
}
impl ::windows_sys::core::Interface for IGattLocalCharacteristic {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2933798765, data2: 21522, data3: 19828, data4: [146, 168, 141, 235, 133, 38, 130, 156] };
}
#[repr(C)]
pub struct IGattLocalCharacteristicParameters {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub SetStaticValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStaticValue: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StaticValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StaticValue: usize,
    pub SetCharacteristicProperties: unsafe extern "system" fn(this: *mut *mut Self, value: GattCharacteristicProperties) -> ::windows_sys::core::HRESULT,
    pub CharacteristicProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattCharacteristicProperties) -> ::windows_sys::core::HRESULT,
    pub SetReadProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, value: GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub ReadProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub SetWriteProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, value: GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub WriteProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub SetUserDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresentationFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresentationFormats: usize,
}
impl ::windows_sys::core::Interface for IGattLocalCharacteristicParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4210507188, data2: 19711, data3: 17607, data4: [132, 69, 4, 14, 110, 173, 0, 99] };
}
#[repr(C)]
pub struct IGattLocalCharacteristicResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Characteristic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothError) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattLocalCharacteristicResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2037767835, data2: 368, data3: 17303, data4: [150, 102, 146, 248, 99, 241, 46, 230] };
}
#[repr(C)]
pub struct IGattLocalDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub Uuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub StaticValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StaticValue: usize,
    pub ReadProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub WriteProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadRequested: usize,
    #[cfg(feature = "Foundation")]
    pub WriteRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWriteRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWriteRequested: usize,
}
impl ::windows_sys::core::Interface for IGattLocalDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4102995462, data2: 30877, data3: 19019, data4: [134, 82, 189, 1, 123, 93, 47, 198] };
}
#[repr(C)]
pub struct IGattLocalDescriptorParameters {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub SetStaticValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStaticValue: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StaticValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StaticValue: usize,
    pub SetReadProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, value: GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub ReadProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub SetWriteProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, value: GattProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub WriteProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattProtectionLevel) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattLocalDescriptorParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1608441450, data2: 62401, data3: 19302, data4: [140, 75, 227, 210, 41, 59, 64, 233] };
}
#[repr(C)]
pub struct IGattLocalDescriptorResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Descriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothError) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattLocalDescriptorResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 928485822, data2: 12831, data3: 17254, data4: [191, 193, 59, 198, 184, 44, 121, 248] };
}
#[repr(C)]
pub struct IGattLocalService {
    pub base__: ::windows_sys::core::IInspectable,
    pub Uuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateCharacteristicAsync: unsafe extern "system" fn(this: *mut *mut Self, characteristicuuid: ::windows_sys::core::GUID, parameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateCharacteristicAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
impl ::windows_sys::core::Interface for IGattLocalService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4111721048, data2: 63479, data3: 18690, data4: [184, 3, 87, 252, 199, 214, 254, 131] };
}
#[repr(C)]
pub struct IGattPresentationFormat {
    pub base__: ::windows_sys::core::IInspectable,
    pub FormatType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Exponent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Unit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Namespace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattPresentationFormat {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 426573857, data2: 64173, data3: 17884, data4: [174, 91, 42, 195, 24, 78, 132, 219] };
}
#[repr(C)]
pub struct IGattPresentationFormatStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub BluetoothSigAssignedNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattPresentationFormatStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 426573856, data2: 64173, data3: 17884, data4: [174, 91, 42, 195, 24, 78, 132, 219] };
}
#[repr(C)]
pub struct IGattPresentationFormatStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromParts: unsafe extern "system" fn(this: *mut *mut Self, formattype: u8, exponent: i32, unit: u16, namespaceid: u8, description: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattPresentationFormatStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2848069395, data2: 47151, data3: 17246, data4: [182, 52, 33, 253, 133, 164, 60, 7] };
}
#[repr(C)]
pub struct IGattPresentationFormatTypesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Boolean: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Bit2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Nibble: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub UInt8: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub UInt12: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub UInt16: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub UInt24: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub UInt32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub UInt48: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub UInt64: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub UInt128: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SInt8: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SInt12: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SInt16: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SInt24: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SInt32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SInt48: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SInt64: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SInt128: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Float32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Float64: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SFloat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Float: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub DUInt16: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Utf8: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Utf16: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Struct: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattPresentationFormatTypesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4210145802, data2: 12474, data3: 16540, data4: [190, 247, 207, 251, 109, 3, 184, 251] };
}
#[repr(C)]
pub struct IGattProtocolErrorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub InvalidHandle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ReadNotPermitted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub WriteNotPermitted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub InvalidPdu: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub InsufficientAuthentication: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub RequestNotSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub InvalidOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub InsufficientAuthorization: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub PrepareQueueFull: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub AttributeNotFound: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub AttributeNotLong: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub InsufficientEncryptionKeySize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub InvalidAttributeValueLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub UnlikelyError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub InsufficientEncryption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub UnsupportedGroupType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub InsufficientResources: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattProtocolErrorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3393635781, data2: 3788, data3: 18441, data4: [190, 163, 207, 121, 188, 153, 30, 55] };
}
#[repr(C)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattCommunicationStatus) -> ::windows_sys::core::HRESULT,
    pub ClientCharacteristicConfigurationDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattClientCharacteristicConfigurationDescriptorValue) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattReadClientCharacteristicConfigurationDescriptorResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1671851785, data2: 6890, data3: 19532, data4: [165, 15, 151, 186, 228, 116, 179, 72] };
}
#[repr(C)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
}
impl ::windows_sys::core::Interface for IGattReadClientCharacteristicConfigurationDescriptorResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 468821405, data2: 47693, data3: 17954, data4: [134, 81, 244, 238, 21, 13, 10, 93] };
}
#[repr(C)]
pub struct IGattReadRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattRequestState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RespondWithValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RespondWithValue: usize,
    pub RespondWithProtocolError: unsafe extern "system" fn(this: *mut *mut Self, protocolerror: u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattReadRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4057818421, data2: 27341, data3: 17062, data4: [164, 187, 215, 137, 218, 224, 4, 62] };
}
#[repr(C)]
pub struct IGattReadRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
    #[cfg(feature = "Foundation")]
    pub GetRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRequestAsync: usize,
}
impl ::windows_sys::core::Interface for IGattReadRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2471064131, data2: 62364, data3: 18507, data4: [138, 182, 153, 107, 164, 134, 207, 163] };
}
#[repr(C)]
pub struct IGattReadResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattCommunicationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
impl ::windows_sys::core::Interface for IGattReadResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1671851784, data2: 6890, data3: 19532, data4: [165, 15, 151, 186, 228, 116, 179, 72] };
}
#[repr(C)]
pub struct IGattReadResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
}
impl ::windows_sys::core::Interface for IGattReadResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2702135456, data2: 64323, data3: 18607, data4: [186, 170, 99, 138, 92, 99, 41, 254] };
}
#[repr(C)]
pub struct IGattReliableWriteTransaction {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub WriteValue: unsafe extern "system" fn(this: *mut *mut Self, characteristic: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    WriteValue: usize,
    #[cfg(feature = "Foundation")]
    pub CommitAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommitAsync: usize,
}
impl ::windows_sys::core::Interface for IGattReliableWriteTransaction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1671851783, data2: 6890, data3: 19532, data4: [165, 15, 151, 186, 228, 116, 179, 72] };
}
#[repr(C)]
pub struct IGattReliableWriteTransaction2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CommitWithResultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommitWithResultAsync: usize,
}
impl ::windows_sys::core::Interface for IGattReliableWriteTransaction2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1360083335, data2: 61202, data3: 17967, data4: [159, 178, 161, 164, 58, 103, 148, 22] };
}
#[repr(C)]
pub struct IGattRequestStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattRequestState) -> ::windows_sys::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothError) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattRequestStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3895777580, data2: 10174, data3: 17587, data4: [157, 13, 79, 198, 232, 8, 221, 63] };
}
#[repr(C)]
pub struct IGattServiceProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Service: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AdvertisementStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattServiceProviderAdvertisementStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AdvertisementStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AdvertisementStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdvertisementStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdvertisementStatusChanged: usize,
    pub StartAdvertising: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StartAdvertisingWithParameters: unsafe extern "system" fn(this: *mut *mut Self, parameters: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StopAdvertising: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattServiceProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2015540173, data2: 10377, data3: 20358, data4: [160, 81, 63, 10, 237, 28, 39, 96] };
}
#[repr(C)]
pub struct IGattServiceProviderAdvertisementStatusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothError) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattServiceProviderAdvertisementStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattServiceProviderAdvertisementStatusChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1504029285, data2: 64033, data3: 20476, data4: [177, 85, 4, 217, 40, 1, 38, 134] };
}
#[repr(C)]
pub struct IGattServiceProviderAdvertisingParameters {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsConnectable: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsConnectable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDiscoverable: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsDiscoverable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattServiceProviderAdvertisingParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3805163947, data2: 25365, data3: 19490, data4: [155, 215, 120, 29, 188, 61, 141, 130] };
}
#[repr(C)]
pub struct IGattServiceProviderAdvertisingParameters2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub SetServiceData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetServiceData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ServiceData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ServiceData: usize,
}
impl ::windows_sys::core::Interface for IGattServiceProviderAdvertisingParameters2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4285023885, data2: 51858, data3: 17460, data4: [151, 67, 14, 144, 152, 138, 216, 121] };
}
#[repr(C)]
pub struct IGattServiceProviderResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothError) -> ::windows_sys::core::HRESULT,
    pub ServiceProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattServiceProviderResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1984337624, data2: 50494, data3: 17036, data4: [138, 72, 103, 175, 224, 44, 58, 230] };
}
#[repr(C)]
pub struct IGattServiceProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceuuid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
impl ::windows_sys::core::Interface for IGattServiceProviderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 830029923, data2: 21078, data3: 16468, data4: [164, 244, 123, 190, 119, 85, 165, 126] };
}
#[repr(C)]
pub struct IGattServiceUuidsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Battery: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub BloodPressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CyclingSpeedAndCadence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GenericAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GenericAttribute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Glucose: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub HealthThermometer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub HeartRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub RunningSpeedAndCadence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattServiceUuidsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1841655896, data2: 39610, data3: 17431, data4: [184, 242, 220, 224, 22, 211, 78, 226] };
}
#[repr(C)]
pub struct IGattServiceUuidsStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlertNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CurrentTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CyclingPower: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub DeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub HumanInterfaceDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ImmediateAlert: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub LinkLoss: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub LocationAndNavigation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub NextDstChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PhoneAlertStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ReferenceTimeUpdate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ScanParameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub TxPower: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattServiceUuidsStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3534656757, data2: 15637, data3: 20345, data4: [156, 12, 234, 175, 166, 117, 21, 92] };
}
#[repr(C)]
pub struct IGattSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanMaintainConnection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetMaintainConnection: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MaintainConnection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MaxPduSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SessionStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattSessionStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxPduSizeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPduSizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMaxPduSizeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMaxPduSizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SessionStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionStatusChanged: usize,
}
impl ::windows_sys::core::Interface for IGattSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3527102787, data2: 57422, data3: 19492, data4: [153, 156, 156, 37, 111, 152, 86, 177] };
}
#[repr(C)]
pub struct IGattSessionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromDeviceIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromDeviceIdAsync: usize,
}
impl ::windows_sys::core::Interface for IGattSessionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 778418524, data2: 21407, data3: 19895, data4: [130, 168, 115, 189, 187, 247, 62, 191] };
}
#[repr(C)]
pub struct IGattSessionStatusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothError) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattSessionStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattSessionStatusChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1980086062, data2: 33663, data3: 16460, data4: [171, 52, 49, 99, 243, 157, 223, 50] };
}
#[repr(C)]
pub struct IGattSubscribedClient {
    pub base__: ::windows_sys::core::IInspectable,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxNotificationSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxNotificationSizeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxNotificationSizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMaxNotificationSizeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMaxNotificationSizeChanged: usize,
}
impl ::windows_sys::core::Interface for IGattSubscribedClient {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1936625665, data2: 5540, data3: 20162, data4: [146, 72, 227, 242, 13, 70, 59, 233] };
}
#[repr(C)]
pub struct IGattValueChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CharacteristicValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CharacteristicValue: usize,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
impl ::windows_sys::core::Interface for IGattValueChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3525040980, data2: 1763, data3: 20184, data4: [162, 99, 172, 250, 200, 186, 115, 19] };
}
#[repr(C)]
pub struct IGattWriteRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Option: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattWriteOption) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattRequestState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    pub Respond: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RespondWithProtocolError: unsafe extern "system" fn(this: *mut *mut Self, protocolerror: u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattWriteRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2931206637, data2: 56879, data3: 20418, data4: [169, 168, 148, 234, 120, 68, 241, 61] };
}
#[repr(C)]
pub struct IGattWriteRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
    #[cfg(feature = "Foundation")]
    pub GetRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRequestAsync: usize,
}
impl ::windows_sys::core::Interface for IGattWriteRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 770476990, data2: 42810, data3: 18202, data4: [148, 213, 3, 125, 234, 221, 8, 6] };
}
#[repr(C)]
pub struct IGattWriteResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GattCommunicationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
}
impl ::windows_sys::core::Interface for IGattWriteResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1234296241, data2: 52011, data3: 17655, data4: [153, 252, 210, 154, 40, 113, 220, 155] };
}
