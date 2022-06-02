#[repr(C)]
pub struct IRfcommDeviceService {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking")]
    pub ConnectionHostName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    ConnectionHostName: usize,
    pub ConnectionServiceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ProtectionLevel: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub MaxProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    MaxProtectionLevel: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetSdpRawAttributesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetSdpRawAttributesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetSdpRawAttributesWithCacheModeAsync: unsafe extern "system" fn(this: *mut *mut Self, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetSdpRawAttributesWithCacheModeAsync: usize,
}
#[repr(C)]
pub struct IRfcommDeviceService2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRfcommDeviceService3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceAccessInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceAccessInformation: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    RequestAccessAsync: usize,
}
#[repr(C)]
pub struct IRfcommDeviceServiceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, serviceid: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRfcommDeviceServiceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelectorForBluetoothDevice: unsafe extern "system" fn(this: *mut *mut Self, bluetoothdevice: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceWithCacheMode: unsafe extern "system" fn(this: *mut *mut Self, bluetoothdevice: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceAndServiceId: unsafe extern "system" fn(this: *mut *mut Self, bluetoothdevice: *mut ::core::ffi::c_void, serviceid: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode: unsafe extern "system" fn(this: *mut *mut Self, bluetoothdevice: *mut ::core::ffi::c_void, serviceid: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRfcommDeviceServicesResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::BluetoothError) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Services: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Services: usize,
}
#[repr(C)]
pub struct IRfcommServiceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub Uuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AsShortId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AsString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRfcommServiceIdStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromUuid: unsafe extern "system" fn(this: *mut *mut Self, uuid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromShortId: unsafe extern "system" fn(this: *mut *mut Self, shortid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SerialPort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ObexObjectPush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ObexFileTransfer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PhoneBookAccessPce: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PhoneBookAccessPse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GenericFileTransfer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRfcommServiceProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub ServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub SdpRawAttributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    SdpRawAttributes: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub StartAdvertising: unsafe extern "system" fn(this: *mut *mut Self, listener: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    StartAdvertising: usize,
    pub StopAdvertising: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRfcommServiceProvider2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_Sockets")]
    pub StartAdvertisingWithRadioDiscoverability: unsafe extern "system" fn(this: *mut *mut Self, listener: *mut ::core::ffi::c_void, radiodiscoverable: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    StartAdvertisingWithRadioDiscoverability: usize,
}
#[repr(C)]
pub struct IRfcommServiceProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
pub type RfcommDeviceService = *mut ::core::ffi::c_void;
pub type RfcommDeviceServicesResult = *mut ::core::ffi::c_void;
pub type RfcommServiceId = *mut ::core::ffi::c_void;
pub type RfcommServiceProvider = *mut ::core::ffi::c_void;
