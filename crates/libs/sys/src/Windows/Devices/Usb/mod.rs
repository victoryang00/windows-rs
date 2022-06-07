#[repr(C)]
pub struct IUsbBulkInEndpointDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxPacketSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EndpointNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Pipe: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbBulkInEndpointDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1013860422, data2: 1743, data3: 17065, data4: [157, 194, 151, 28, 27, 20, 182, 227] };
}
#[repr(C)]
pub struct IUsbBulkInPipe {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxTransferSizeBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EndpointDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClearStallAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearStallAsync: usize,
    pub SetReadOptions: unsafe extern "system" fn(this: *mut *mut Self, value: UsbReadOptions) -> ::windows_sys::core::HRESULT,
    pub ReadOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UsbReadOptions) -> ::windows_sys::core::HRESULT,
    pub FlushBuffer: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
}
impl ::windows_sys::core::Interface for IUsbBulkInPipe {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4028443963, data2: 17736, data3: 19792, data4: [179, 38, 216, 44, 218, 190, 18, 32] };
}
#[repr(C)]
pub struct IUsbBulkOutEndpointDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxPacketSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EndpointNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Pipe: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbBulkOutEndpointDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 673219706, data2: 65518, data3: 20320, data4: [155, 225, 149, 108, 172, 62, 203, 101] };
}
#[repr(C)]
pub struct IUsbBulkOutPipe {
    pub base__: ::windows_sys::core::IInspectable,
    pub EndpointDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClearStallAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearStallAsync: usize,
    pub SetWriteOptions: unsafe extern "system" fn(this: *mut *mut Self, value: UsbWriteOptions) -> ::windows_sys::core::HRESULT,
    pub WriteOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UsbWriteOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
}
impl ::windows_sys::core::Interface for IUsbBulkOutPipe {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2833903214, data2: 277, data3: 17834, data4: [139, 33, 55, 178, 37, 188, 206, 231] };
}
#[repr(C)]
pub struct IUsbConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub UsbInterfaces: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UsbInterfaces: usize,
    pub ConfigurationDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Descriptors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Descriptors: usize,
}
impl ::windows_sys::core::Interface for IUsbConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1746367529, data2: 13993, data3: 18135, data4: [184, 115, 252, 104, 146, 81, 236, 48] };
}
#[repr(C)]
pub struct IUsbConfigurationDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConfigurationValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub MaxPowerMilliamps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SelfPowered: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RemoteWakeup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbConfigurationDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4061621650, data2: 46146, data3: 16506, data4: [130, 7, 125, 100, 108, 3, 133, 243] };
}
#[repr(C)]
pub struct IUsbConfigurationDescriptorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, parsed: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbConfigurationDescriptorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1112337811, data2: 59200, data3: 16545, data4: [146, 189, 218, 18, 14, 160, 73, 20] };
}
#[repr(C)]
pub struct IUsbControlRequestType {
    pub base__: ::windows_sys::core::IInspectable,
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UsbTransferDirection) -> ::windows_sys::core::HRESULT,
    pub SetDirection: unsafe extern "system" fn(this: *mut *mut Self, value: UsbTransferDirection) -> ::windows_sys::core::HRESULT,
    pub ControlTransferType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UsbControlTransferType) -> ::windows_sys::core::HRESULT,
    pub SetControlTransferType: unsafe extern "system" fn(this: *mut *mut Self, value: UsbControlTransferType) -> ::windows_sys::core::HRESULT,
    pub Recipient: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UsbControlRecipient) -> ::windows_sys::core::HRESULT,
    pub SetRecipient: unsafe extern "system" fn(this: *mut *mut Self, value: UsbControlRecipient) -> ::windows_sys::core::HRESULT,
    pub AsByte: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetAsByte: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbControlRequestType {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2392090022, data2: 55101, data3: 18142, data4: [148, 190, 170, 231, 240, 124, 15, 92] };
}
#[repr(C)]
pub struct IUsbDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub DescriptorType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ReadDescriptorBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ReadDescriptorBuffer: usize,
}
impl ::windows_sys::core::Interface for IUsbDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 176812566, data2: 24477, data3: 18548, data4: [137, 4, 218, 154, 211, 245, 82, 143] };
}
#[repr(C)]
pub struct IUsbDevice {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendControlOutTransferAsync: unsafe extern "system" fn(this: *mut *mut Self, setuppacket: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendControlOutTransferAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendControlOutTransferAsyncNoBuffer: unsafe extern "system" fn(this: *mut *mut Self, setuppacket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendControlOutTransferAsyncNoBuffer: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendControlInTransferAsync: unsafe extern "system" fn(this: *mut *mut Self, setuppacket: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendControlInTransferAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendControlInTransferAsyncNoBuffer: unsafe extern "system" fn(this: *mut *mut Self, setuppacket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendControlInTransferAsyncNoBuffer: usize,
    pub DefaultInterface: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeviceDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1380563346, data2: 50262, data3: 17621, data4: [173, 94, 36, 245, 160, 137, 246, 59] };
}
#[repr(C)]
pub struct IUsbDeviceClass {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClassCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetClassCode: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SubclassCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubclassCode: usize,
    #[cfg(feature = "Foundation")]
    pub SetSubclassCode: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSubclassCode: usize,
    #[cfg(feature = "Foundation")]
    pub ProtocolCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolCode: usize,
    #[cfg(feature = "Foundation")]
    pub SetProtocolCode: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProtocolCode: usize,
}
impl ::windows_sys::core::Interface for IUsbDeviceClass {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 85541625, data2: 33886, data3: 18411, data4: [177, 42, 56, 242, 246, 23, 175, 231] };
}
#[repr(C)]
pub struct IUsbDeviceClasses {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IUsbDeviceClasses {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1752143197, data2: 39826, data3: 19248, data4: [151, 129, 194, 44, 85, 172, 53, 203] };
}
#[repr(C)]
pub struct IUsbDeviceClassesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CdcControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Physical: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PersonalHealthcare: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ActiveSync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PalmSync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeviceFirmwareUpdate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Irda: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Measurement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VendorSpecific: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbDeviceClassesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2987066663, data2: 50560, data3: 17817, data4: [161, 101, 152, 27, 79, 208, 50, 48] };
}
#[repr(C)]
pub struct IUsbDeviceDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub BcdUsb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxPacketSize0: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub VendorId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub BcdDeviceRevision: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub NumberOfConfigurations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbDeviceDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 524866038, data2: 47767, data3: 17186, data4: [185, 44, 181, 177, 137, 33, 101, 136] };
}
#[repr(C)]
pub struct IUsbDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, vendorid: u32, productid: u32, winusbinterfaceclass: ::windows_sys::core::GUID, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorGuidOnly: unsafe extern "system" fn(this: *mut *mut Self, winusbinterfaceclass: ::windows_sys::core::GUID, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorVidPidOnly: unsafe extern "system" fn(this: *mut *mut Self, vendorid: u32, productid: u32, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceClassSelector: unsafe extern "system" fn(this: *mut *mut Self, usbclass: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
impl ::windows_sys::core::Interface for IUsbDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 107709858, data2: 2487, data3: 17478, data4: [133, 2, 111, 230, 220, 170, 115, 9] };
}
#[repr(C)]
pub struct IUsbEndpointDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub EndpointNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UsbTransferDirection) -> ::windows_sys::core::HRESULT,
    pub EndpointType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UsbEndpointType) -> ::windows_sys::core::HRESULT,
    pub AsBulkInEndpointDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AsInterruptInEndpointDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AsBulkOutEndpointDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AsInterruptOutEndpointDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbEndpointDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1799906009, data2: 36343, data3: 19264, data4: [172, 131, 87, 143, 19, 159, 5, 117] };
}
#[repr(C)]
pub struct IUsbEndpointDescriptorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, parsed: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbEndpointDescriptorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3364925953, data2: 39530, data3: 18782, data4: [168, 44, 41, 91, 158, 112, 129, 6] };
}
#[repr(C)]
pub struct IUsbInterface {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub BulkInPipes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BulkInPipes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InterruptInPipes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InterruptInPipes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BulkOutPipes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BulkOutPipes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InterruptOutPipes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InterruptOutPipes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InterfaceSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InterfaceSettings: usize,
    pub InterfaceNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Descriptors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Descriptors: usize,
}
impl ::windows_sys::core::Interface for IUsbInterface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2687642517, data2: 32583, data3: 18603, data4: [167, 39, 103, 140, 37, 190, 33, 18] };
}
#[repr(C)]
pub struct IUsbInterfaceDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClassCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SubclassCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ProtocolCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub AlternateSettingNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub InterfaceNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbInterfaceDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 429289671, data2: 47086, data3: 20368, data4: [140, 213, 148, 162, 226, 87, 89, 138] };
}
#[repr(C)]
pub struct IUsbInterfaceDescriptorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, parsed: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbInterfaceDescriptorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3813318645, data2: 30678, data3: 18614, data4: [176, 190, 22, 198, 66, 35, 22, 254] };
}
#[repr(C)]
pub struct IUsbInterfaceSetting {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub BulkInEndpoints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BulkInEndpoints: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InterruptInEndpoints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InterruptInEndpoints: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BulkOutEndpoints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BulkOutEndpoints: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InterruptOutEndpoints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InterruptOutEndpoints: usize,
    pub Selected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectSettingAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectSettingAsync: usize,
    pub InterfaceDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Descriptors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Descriptors: usize,
}
impl ::windows_sys::core::Interface for IUsbInterfaceSetting {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 405257127, data2: 36263, data3: 19191, data4: [143, 76, 127, 48, 50, 231, 129, 245] };
}
#[repr(C)]
pub struct IUsbInterruptInEndpointDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxPacketSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EndpointNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Interval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Interval: usize,
    pub Pipe: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbInterruptInEndpointDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3226634599, data2: 51473, data3: 19514, data4: [134, 178, 65, 156, 45, 168, 144, 57] };
}
#[repr(C)]
pub struct IUsbInterruptInEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub InterruptData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InterruptData: usize,
}
impl ::windows_sys::core::Interface for IUsbInterruptInEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3081781394, data2: 5144, data3: 18742, data4: [130, 9, 41, 156, 245, 96, 85, 131] };
}
#[repr(C)]
pub struct IUsbInterruptInPipe {
    pub base__: ::windows_sys::core::IInspectable,
    pub EndpointDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClearStallAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearStallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DataReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataReceived: usize,
}
impl ::windows_sys::core::Interface for IUsbInterruptInPipe {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4194332950, data2: 34007, data3: 18631, data4: [138, 63, 76, 11, 35, 95, 46, 166] };
}
#[repr(C)]
pub struct IUsbInterruptOutEndpointDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxPacketSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EndpointNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Interval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Interval: usize,
    pub Pipe: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbInterruptOutEndpointDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3433033089, data2: 4298, data3: 17715, data4: [149, 45, 158, 39, 131, 65, 232, 15] };
}
#[repr(C)]
pub struct IUsbInterruptOutPipe {
    pub base__: ::windows_sys::core::IInspectable,
    pub EndpointDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClearStallAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearStallAsync: usize,
    pub SetWriteOptions: unsafe extern "system" fn(this: *mut *mut Self, value: UsbWriteOptions) -> ::windows_sys::core::HRESULT,
    pub WriteOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UsbWriteOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
}
impl ::windows_sys::core::Interface for IUsbInterruptOutPipe {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3917793449, data2: 43769, data3: 18896, data4: [185, 108, 246, 97, 171, 74, 127, 149] };
}
#[repr(C)]
pub struct IUsbSetupPacket {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRequestType: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetRequest: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Index: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUsbSetupPacket {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 273391922, data2: 51087, data3: 19537, data4: [182, 84, 228, 157, 2, 242, 203, 3] };
}
#[repr(C)]
pub struct IUsbSetupPacketFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateWithEightByteBuffer: unsafe extern "system" fn(this: *mut *mut Self, eightbytebuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateWithEightByteBuffer: usize,
}
impl ::windows_sys::core::Interface for IUsbSetupPacketFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3374677328, data2: 6958, data3: 19009, data4: [162, 167, 51, 143, 12, 239, 60, 20] };
}
pub type UsbBulkInEndpointDescriptor = *mut ::core::ffi::c_void;
pub type UsbBulkInPipe = *mut ::core::ffi::c_void;
pub type UsbBulkOutEndpointDescriptor = *mut ::core::ffi::c_void;
pub type UsbBulkOutPipe = *mut ::core::ffi::c_void;
pub type UsbConfiguration = *mut ::core::ffi::c_void;
pub type UsbConfigurationDescriptor = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Usb\"`*"]
#[repr(transparent)]
pub struct UsbControlRecipient(pub i32);
impl UsbControlRecipient {
    pub const Device: Self = Self(0i32);
    pub const SpecifiedInterface: Self = Self(1i32);
    pub const Endpoint: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
    pub const DefaultInterface: Self = Self(4i32);
}
impl ::core::marker::Copy for UsbControlRecipient {}
impl ::core::clone::Clone for UsbControlRecipient {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UsbControlRequestType = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Usb\"`*"]
#[repr(transparent)]
pub struct UsbControlTransferType(pub i32);
impl UsbControlTransferType {
    pub const Standard: Self = Self(0i32);
    pub const Class: Self = Self(1i32);
    pub const Vendor: Self = Self(2i32);
}
impl ::core::marker::Copy for UsbControlTransferType {}
impl ::core::clone::Clone for UsbControlTransferType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UsbDescriptor = *mut ::core::ffi::c_void;
pub type UsbDevice = *mut ::core::ffi::c_void;
pub type UsbDeviceClass = *mut ::core::ffi::c_void;
pub type UsbDeviceClasses = *mut ::core::ffi::c_void;
pub type UsbDeviceDescriptor = *mut ::core::ffi::c_void;
pub type UsbEndpointDescriptor = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Usb\"`*"]
#[repr(transparent)]
pub struct UsbEndpointType(pub i32);
impl UsbEndpointType {
    pub const Control: Self = Self(0i32);
    pub const Isochronous: Self = Self(1i32);
    pub const Bulk: Self = Self(2i32);
    pub const Interrupt: Self = Self(3i32);
}
impl ::core::marker::Copy for UsbEndpointType {}
impl ::core::clone::Clone for UsbEndpointType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UsbInterface = *mut ::core::ffi::c_void;
pub type UsbInterfaceDescriptor = *mut ::core::ffi::c_void;
pub type UsbInterfaceSetting = *mut ::core::ffi::c_void;
pub type UsbInterruptInEndpointDescriptor = *mut ::core::ffi::c_void;
pub type UsbInterruptInEventArgs = *mut ::core::ffi::c_void;
pub type UsbInterruptInPipe = *mut ::core::ffi::c_void;
pub type UsbInterruptOutEndpointDescriptor = *mut ::core::ffi::c_void;
pub type UsbInterruptOutPipe = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Usb\"`*"]
#[repr(transparent)]
pub struct UsbReadOptions(pub u32);
impl UsbReadOptions {
    pub const None: Self = Self(0u32);
    pub const AutoClearStall: Self = Self(1u32);
    pub const OverrideAutomaticBufferManagement: Self = Self(2u32);
    pub const IgnoreShortPacket: Self = Self(4u32);
    pub const AllowPartialReads: Self = Self(8u32);
}
impl ::core::marker::Copy for UsbReadOptions {}
impl ::core::clone::Clone for UsbReadOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UsbSetupPacket = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Usb\"`*"]
#[repr(transparent)]
pub struct UsbTransferDirection(pub i32);
impl UsbTransferDirection {
    pub const Out: Self = Self(0i32);
    pub const In: Self = Self(1i32);
}
impl ::core::marker::Copy for UsbTransferDirection {}
impl ::core::clone::Clone for UsbTransferDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Usb\"`*"]
#[repr(transparent)]
pub struct UsbWriteOptions(pub u32);
impl UsbWriteOptions {
    pub const None: Self = Self(0u32);
    pub const AutoClearStall: Self = Self(1u32);
    pub const ShortPacketTerminate: Self = Self(2u32);
}
impl ::core::marker::Copy for UsbWriteOptions {}
impl ::core::clone::Clone for UsbWriteOptions {
    fn clone(&self) -> Self {
        *self
    }
}
