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
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SelectedTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedTransmitPowerLevelInDBm: usize,
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
#[repr(C)]
pub struct IGattServiceProviderConnectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AllServices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllServices: usize,
}
#[repr(C)]
pub struct IGattServiceProviderTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
pub type RfcommConnectionTriggerDetails = *mut ::core::ffi::c_void;
pub type RfcommInboundConnectionInformation = *mut ::core::ffi::c_void;
pub type RfcommOutboundConnectionInformation = *mut ::core::ffi::c_void;
