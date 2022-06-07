#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct BluetoothEventTriggeringMode(pub i32);
impl BluetoothEventTriggeringMode {
    pub const Serial: Self = Self(0i32);
    pub const Batch: Self = Self(1i32);
    pub const KeepLatest: Self = Self(2i32);
}
impl ::core::marker::Copy for BluetoothEventTriggeringMode {}
impl ::core::clone::Clone for BluetoothEventTriggeringMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BluetoothLEAdvertisementPublisherTriggerDetails = *mut ::core::ffi::c_void;
pub type BluetoothLEAdvertisementWatcherTriggerDetails = *mut ::core::ffi::c_void;
pub type GattCharacteristicNotificationTriggerDetails = *mut ::core::ffi::c_void;
pub type GattServiceProviderConnection = *mut ::core::ffi::c_void;
pub type GattServiceProviderTriggerDetails = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Advertisement::BluetoothLEAdvertisementPublisherStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    Status: usize,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothError) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementPublisherTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1628359302, data2: 13440, data3: 16841, data4: [169, 24, 125, 218, 223, 32, 126, 0] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SelectedTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedTransmitPowerLevelInDBm: usize,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementPublisherTriggerDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3567505445, data2: 50689, data3: 17110, data4: [152, 41, 76, 203, 63, 92, 215, 127] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcherTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothError) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections"))]
    pub Advertisements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections")))]
    Advertisements: usize,
    pub SignalStrengthFilter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementWatcherTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2816170711, data2: 8791, data3: 20073, data4: [151, 132, 254, 230, 69, 193, 220, 224] };
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Characteristic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Characteristic: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
impl ::windows_sys::core::Interface for IGattCharacteristicNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2610969368, data2: 4076, data3: 17258, data4: [147, 177, 244, 108, 105, 117, 50, 162] };
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothError) -> ::windows_sys::core::HRESULT,
    pub EventTriggeringMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BluetoothEventTriggeringMode) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))]
    pub ValueChangedEvents: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections")))]
    ValueChangedEvents: usize,
}
impl ::windows_sys::core::Interface for IGattCharacteristicNotificationTriggerDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1920618716, data2: 38045, data3: 17738, data4: [177, 146, 152, 52, 103, 227, 213, 15] };
}
#[repr(C)]
pub struct IGattServiceProviderConnection {
    pub base__: ::windows_sys::core::IInspectable,
    pub TriggerId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Service: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Service: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattServiceProviderConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2141305273, data2: 12051, data3: 16565, data4: [149, 130, 142, 183, 142, 152, 239, 19] };
}
#[repr(C)]
pub struct IGattServiceProviderConnectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AllServices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllServices: usize,
}
impl ::windows_sys::core::Interface for IGattServiceProviderConnectionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1028693835, data2: 2830, data3: 17510, data4: [184, 205, 110, 189, 218, 31, 161, 125] };
}
#[repr(C)]
pub struct IGattServiceProviderTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGattServiceProviderTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2928412197, data2: 1535, data3: 19195, data4: [177, 106, 222, 149, 243, 207, 1, 88] };
}
#[repr(C)]
pub struct IRfcommConnectionTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_Sockets")]
    pub Socket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    Socket: usize,
    pub Incoming: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RemoteDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRfcommConnectionTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4179784525, data2: 11836, data3: 20220, data4: [171, 89, 252, 92, 249, 111, 151, 227] };
}
#[repr(C)]
pub struct IRfcommInboundConnectionInformation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub SdpRecord: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SdpRecord: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSdpRecord: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSdpRecord: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub LocalServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    LocalServiceId: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub SetLocalServiceId: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    SetLocalServiceId: usize,
    pub ServiceCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothServiceCapabilities) -> ::windows_sys::core::HRESULT,
    pub SetServiceCapabilities: unsafe extern "system" fn(this: *mut *mut Self, value: super::BluetoothServiceCapabilities) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRfcommInboundConnectionInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1832809896, data2: 21545, data3: 16473, data4: [146, 227, 30, 139, 101, 82, 135, 7] };
}
#[repr(C)]
pub struct IRfcommOutboundConnectionInformation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub RemoteServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    RemoteServiceId: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub SetRemoteServiceId: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    SetRemoteServiceId: usize,
}
impl ::windows_sys::core::Interface for IRfcommOutboundConnectionInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2962301563, data2: 62516, data3: 19632, data4: [153, 177, 74, 184, 206, 218, 237, 215] };
}
pub type RfcommConnectionTriggerDetails = *mut ::core::ffi::c_void;
pub type RfcommInboundConnectionInformation = *mut ::core::ffi::c_void;
pub type RfcommOutboundConnectionInformation = *mut ::core::ffi::c_void;
