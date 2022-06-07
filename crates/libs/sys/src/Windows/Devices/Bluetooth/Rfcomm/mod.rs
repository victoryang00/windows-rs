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
impl ::windows_sys::core::Interface for IRfcommDeviceService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2927755039, data2: 50593, data3: 19520, data4: [140, 40, 243, 239, 214, 144, 98, 243] };
}
#[repr(C)]
pub struct IRfcommDeviceService2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRfcommDeviceService2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1399647508, data2: 60365, data3: 18942, data4: [191, 159, 64, 239, 198, 137, 178, 13] };
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
impl ::windows_sys::core::Interface for IRfcommDeviceService3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 472034534, data2: 56644, data3: 19747, data4: [134, 109, 143, 52, 134, 238, 100, 144] };
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
impl ::windows_sys::core::Interface for IRfcommDeviceServiceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2762033647, data2: 25197, data3: 16812, data4: [178, 83, 135, 172, 92, 39, 226, 138] };
}
#[repr(C)]
pub struct IRfcommDeviceServiceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelectorForBluetoothDevice: unsafe extern "system" fn(this: *mut *mut Self, bluetoothdevice: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceWithCacheMode: unsafe extern "system" fn(this: *mut *mut Self, bluetoothdevice: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceAndServiceId: unsafe extern "system" fn(this: *mut *mut Self, bluetoothdevice: *mut ::core::ffi::c_void, serviceid: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode: unsafe extern "system" fn(this: *mut *mut Self, bluetoothdevice: *mut ::core::ffi::c_void, serviceid: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRfcommDeviceServiceStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2861347273, data2: 59277, data3: 19428, data4: [128, 118, 10, 61, 135, 160, 160, 95] };
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
impl ::windows_sys::core::Interface for IRfcommDeviceServicesResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 994588812, data2: 31951, data3: 18574, data4: [150, 37, 210, 89, 165, 115, 45, 85] };
}
#[repr(C)]
pub struct IRfcommServiceId {
    pub base__: ::windows_sys::core::IInspectable,
    pub Uuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AsShortId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AsString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRfcommServiceId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 576885252, data2: 32258, data3: 16407, data4: [129, 54, 218, 27, 106, 27, 155, 191] };
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
impl ::windows_sys::core::Interface for IRfcommServiceIdStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 706191034, data2: 43381, data3: 18147, data4: [181, 107, 8, 255, 215, 131, 165, 254] };
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
impl ::windows_sys::core::Interface for IRfcommServiceProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3940285892, data2: 45558, data3: 17663, data4: [159, 124, 231, 168, 42, 184, 104, 33] };
}
#[repr(C)]
pub struct IRfcommServiceProvider2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_Sockets")]
    pub StartAdvertisingWithRadioDiscoverability: unsafe extern "system" fn(this: *mut *mut Self, listener: *mut ::core::ffi::c_void, radiodiscoverable: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    StartAdvertisingWithRadioDiscoverability: usize,
}
impl ::windows_sys::core::Interface for IRfcommServiceProvider2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1936449478, data2: 15489, data3: 19742, data4: [186, 242, 221, 187, 129, 40, 69, 18] };
}
#[repr(C)]
pub struct IRfcommServiceProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
impl ::windows_sys::core::Interface for IRfcommServiceProviderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2559083267, data2: 27082, data3: 16698, data4: [132, 247, 67, 68, 199, 41, 41, 151] };
}
pub type RfcommDeviceService = *mut ::core::ffi::c_void;
pub type RfcommDeviceServicesResult = *mut ::core::ffi::c_void;
pub type RfcommServiceId = *mut ::core::ffi::c_void;
pub type RfcommServiceProvider = *mut ::core::ffi::c_void;
