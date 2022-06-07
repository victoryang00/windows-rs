pub type BluetoothLEAdvertisement = *mut ::core::ffi::c_void;
pub type BluetoothLEAdvertisementBytePattern = *mut ::core::ffi::c_void;
pub type BluetoothLEAdvertisementDataSection = *mut ::core::ffi::c_void;
pub type BluetoothLEAdvertisementFilter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementFlags(pub u32);
impl BluetoothLEAdvertisementFlags {
    pub const None: Self = Self(0u32);
    pub const LimitedDiscoverableMode: Self = Self(1u32);
    pub const GeneralDiscoverableMode: Self = Self(2u32);
    pub const ClassicNotSupported: Self = Self(4u32);
    pub const DualModeControllerCapable: Self = Self(8u32);
    pub const DualModeHostCapable: Self = Self(16u32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementFlags {}
impl ::core::clone::Clone for BluetoothLEAdvertisementFlags {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BluetoothLEAdvertisementPublisher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherStatus(pub i32);
impl BluetoothLEAdvertisementPublisherStatus {
    pub const Created: Self = Self(0i32);
    pub const Waiting: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementPublisherStatus {}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BluetoothLEAdvertisementPublisherStatusChangedEventArgs = *mut ::core::ffi::c_void;
pub type BluetoothLEAdvertisementReceivedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementType(pub i32);
impl BluetoothLEAdvertisementType {
    pub const ConnectableUndirected: Self = Self(0i32);
    pub const ConnectableDirected: Self = Self(1i32);
    pub const ScannableUndirected: Self = Self(2i32);
    pub const NonConnectableUndirected: Self = Self(3i32);
    pub const ScanResponse: Self = Self(4i32);
    pub const Extended: Self = Self(5i32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementType {}
impl ::core::clone::Clone for BluetoothLEAdvertisementType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BluetoothLEAdvertisementWatcher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherStatus(pub i32);
impl BluetoothLEAdvertisementWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopping: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Aborted: Self = Self(4i32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementWatcherStatus {}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BluetoothLEAdvertisementWatcherStoppedEventArgs = *mut ::core::ffi::c_void;
pub type BluetoothLEManufacturerData = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEScanningMode(pub i32);
impl BluetoothLEScanningMode {
    pub const Passive: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl ::core::marker::Copy for BluetoothLEScanningMode {}
impl ::core::clone::Clone for BluetoothLEScanningMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IBluetoothLEAdvertisement {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Flags: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Flags: usize,
    #[cfg(feature = "Foundation")]
    pub SetFlags: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFlags: usize,
    pub LocalName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLocalName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ServiceUuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServiceUuids: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ManufacturerData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ManufacturerData: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DataSections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DataSections: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetManufacturerDataByCompanyId: unsafe extern "system" fn(this: *mut *mut Self, companyid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetManufacturerDataByCompanyId: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSectionsByType: unsafe extern "system" fn(this: *mut *mut Self, r#type: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSectionsByType: usize,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 107983543, data2: 13265, data3: 20093, data4: [131, 103, 207, 129, 208, 247, 150, 83] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementBytePattern {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetDataType: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementBytePattern {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4227520498, data2: 47557, data3: 18952, data4: [188, 81, 80, 47, 142, 246, 138, 121] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementBytePatternFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, datatype: u8, offset: i16, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementBytePatternFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3269610867, data2: 64860, data3: 20163, data4: [190, 42, 156, 166, 250, 17, 183, 189] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementDataSection {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetDataType: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementDataSection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3609277204, data2: 14915, data3: 16633, data4: [182, 240, 146, 191, 239, 195, 74, 227] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementDataSectionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, datatype: u8, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementDataSectionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3886287170, data2: 43077, data3: 16453, data4: [191, 126, 62, 153, 113, 219, 138, 107] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementDataTypesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Flags: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub IncompleteService16BitUuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub CompleteService16BitUuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub IncompleteService32BitUuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub CompleteService32BitUuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub IncompleteService128BitUuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub CompleteService128BitUuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ShortenedLocalName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub CompleteLocalName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub TxPowerLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SlaveConnectionIntervalRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ServiceSolicitation16BitUuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ServiceSolicitation32BitUuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ServiceSolicitation128BitUuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ServiceData16BitUuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ServiceData32BitUuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ServiceData128BitUuids: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub PublicTargetAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub RandomTargetAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Appearance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub AdvertisingInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ManufacturerSpecificData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementDataTypesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1001801519, data2: 1542, data3: 17227, data4: [167, 110, 116, 21, 159, 6, 132, 211] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementFilter {
    pub base__: ::windows_sys::core::IInspectable,
    pub Advertisement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAdvertisement: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BytePatterns: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BytePatterns: usize,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 320778451, data2: 53326, data3: 18353, data4: [131, 126, 73, 64, 91, 246, 248, 15] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisher {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BluetoothLEAdvertisementPublisherStatus) -> ::windows_sys::core::HRESULT,
    pub Advertisement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementPublisher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3454542073, data2: 55802, data3: 17366, data4: [162, 100, 221, 216, 183, 218, 139, 120] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisher2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreferredTransmitPowerLevelInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub SetPreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredTransmitPowerLevelInDBm: usize,
    pub UseExtendedAdvertisement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetUseExtendedAdvertisement: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAnonymous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAnonymous: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IncludeTransmitPowerLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIncludeTransmitPowerLevel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementPublisher2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4225455198, data2: 22257, data3: 20751, data4: [164, 52, 33, 127, 189, 158, 123, 210] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, advertisement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementPublisherFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1549731422, data2: 47203, data3: 18817, data4: [161, 175, 28, 84, 77, 139, 12, 13] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BluetoothLEAdvertisementPublisherStatus) -> ::windows_sys::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothError) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 163757471, data2: 11775, data3: 19235, data4: [134, 238, 13, 20, 251, 148, 174, 174] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SelectedTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedTransmitPowerLevelInDBm: usize,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2405595406, data2: 56456, data3: 23691, data4: [179, 78, 16, 179, 33, 133, 15, 136] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RawSignalStrengthInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i16) -> ::windows_sys::core::HRESULT,
    pub BluetoothAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub AdvertisementType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BluetoothLEAdvertisementType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Advertisement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 664305119, data2: 58774, data3: 16830, data4: [141, 67, 158, 103, 49, 212, 169, 19] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BluetoothAddressType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothAddressType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransmitPowerLevelInDBm: usize,
    pub IsAnonymous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsConnectable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsScannable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDirected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsScanResponse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementReceivedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 316262523, data2: 921, data3: 24334, data4: [163, 72, 83, 176, 43, 107, 22, 46] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MinSamplingInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSamplingInterval: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSamplingInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSamplingInterval: usize,
    #[cfg(feature = "Foundation")]
    pub MinOutOfRangeTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinOutOfRangeTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOutOfRangeTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOutOfRangeTimeout: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BluetoothLEAdvertisementWatcherStatus) -> ::windows_sys::core::HRESULT,
    pub ScanningMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BluetoothLEScanningMode) -> ::windows_sys::core::HRESULT,
    pub SetScanningMode: unsafe extern "system" fn(this: *mut *mut Self, value: BluetoothLEScanningMode) -> ::windows_sys::core::HRESULT,
    pub SignalStrengthFilter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSignalStrengthFilter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AdvertisementFilter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAdvertisementFilter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Received: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Received: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReceived: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2796303215, data2: 62419, data3: 17047, data4: [141, 108, 200, 30, 166, 98, 63, 64] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcher2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementWatcher2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 29304508, data2: 45412, data3: 22533, data4: [144, 163, 232, 167, 153, 127, 242, 37] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcherFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, advertisementfilter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementWatcherFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2595171670, data2: 14764, data3: 17726, data4: [179, 42, 133, 198, 87, 224, 23, 241] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothError) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementWatcherStoppedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3712022605, data2: 59321, data3: 17379, data4: [156, 4, 6, 133, 208, 133, 253, 140] };
}
#[repr(C)]
pub struct IBluetoothLEManufacturerData {
    pub base__: ::windows_sys::core::IInspectable,
    pub CompanyId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetCompanyId: unsafe extern "system" fn(this: *mut *mut Self, value: u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
}
impl ::windows_sys::core::Interface for IBluetoothLEManufacturerData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2435693080, data2: 26979, data3: 17715, data4: [176, 97, 70, 148, 218, 251, 52, 229] };
}
#[repr(C)]
pub struct IBluetoothLEManufacturerDataFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, companyid: u16, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IBluetoothLEManufacturerDataFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3231398392, data2: 12698, data3: 17438, data4: [141, 229, 102, 168, 30, 135, 122, 108] };
}
