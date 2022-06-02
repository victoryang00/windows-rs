#[repr(C)]
pub struct IUsbBulkInEndpointDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxPacketSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EndpointNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Pipe: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IUsbBulkOutEndpointDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxPacketSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EndpointNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Pipe: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IUsbConfigurationDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConfigurationValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub MaxPowerMilliamps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SelfPowered: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RemoteWakeup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUsbConfigurationDescriptorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, parsed: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IUsbDeviceClasses {
    pub base__: ::windows_sys::core::IInspectable,
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
#[repr(C)]
pub struct IUsbEndpointDescriptorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, parsed: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IUsbInterfaceDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClassCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SubclassCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ProtocolCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub AlternateSettingNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub InterfaceNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUsbInterfaceDescriptorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, parsed: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IUsbInterruptInEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub InterruptData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InterruptData: usize,
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
#[repr(C)]
pub struct IUsbSetupPacketFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateWithEightByteBuffer: unsafe extern "system" fn(this: *mut *mut Self, eightbytebuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateWithEightByteBuffer: usize,
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
