#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
    pub fn SwDeviceClose(hswdevice: HSWDEVICE);
    #[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SwDeviceCreate(pszenumeratorname: ::windows_sys::core::PCWSTR, pszparentdeviceinstance: ::windows_sys::core::PCWSTR, pcreateinfo: *const SW_DEVICE_CREATE_INFO, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, pcallback: SW_DEVICE_CREATE_CALLBACK, pcontext: *const ::core::ffi::c_void, phswdevice: *mut isize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
    pub fn SwDeviceGetLifetime(hswdevice: HSWDEVICE, plifetime: *mut SW_DEVICE_LIFETIME) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn SwDeviceInterfacePropertySet(hswdevice: HSWDEVICE, pszdeviceinterfaceid: ::windows_sys::core::PCWSTR, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SwDeviceInterfaceRegister(hswdevice: HSWDEVICE, pinterfaceclassguid: *const ::windows_sys::core::GUID, pszreferencestring: ::windows_sys::core::PCWSTR, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, fenabled: super::super::super::Foundation::BOOL, ppszdeviceinterfaceid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwDeviceInterfaceSetState(hswdevice: HSWDEVICE, pszdeviceinterfaceid: ::windows_sys::core::PCWSTR, fenabled: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn SwDevicePropertySet(hswdevice: HSWDEVICE, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
    pub fn SwDeviceSetLifetime(hswdevice: HSWDEVICE, lifetime: SW_DEVICE_LIFETIME) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
    pub fn SwMemFree(pmem: *const ::core::ffi::c_void);
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const ADDRESS_FAMILY_VALUE_NAME: &str = "AddressFamily";
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_ACTION_SPECIFIC_BASE: u32 = 600u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_ACTION_SPECIFIC_MAX: u32 = 899u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_DEVICE_INTERNAL_ERROR: u32 = 501u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_INVALID_ACTION: u32 = 401u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_INVALID_ARG: u32 = 402u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_INVALID_SEQUENCE_NUMBER: u32 = 403u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const FAULT_INVALID_VARIABLE: u32 = 404u32;
pub type HSWDEVICE = isize;
#[repr(C)]
pub struct IUPnPAddressFamilyControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut *mut Self, dwflags: i32) -> ::windows_sys::core::HRESULT,
    pub GetAddressFamily: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUPnPAddressFamilyControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3820970360, data2: 26958, data3: 17823, data4: [165, 166, 25, 30, 160, 255, 161, 199] };
}
#[repr(C)]
pub struct IUPnPAsyncResult {
    pub base__: ::windows_sys::core::IUnknown,
    pub AsyncOperationComplete: unsafe extern "system" fn(this: *mut *mut Self, ullrequestid: u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUPnPAsyncResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1298529544, data2: 53566, data3: 17012, data4: [156, 139, 221, 141, 2, 140, 134, 68] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPDescriptionDocument {
    pub base__: super::super::super::System::Com::IDispatch,
    pub ReadyState: unsafe extern "system" fn(this: *mut *mut Self, plreadystate: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Load: unsafe extern "system" fn(this: *mut *mut Self, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Load: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadAsync: unsafe extern "system" fn(this: *mut *mut Self, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadAsync: usize,
    pub LoadResult: unsafe extern "system" fn(this: *mut *mut Self, phrerror: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RootDevice: unsafe extern "system" fn(this: *mut *mut Self, ppudrootdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootDevice: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub DeviceByUDN: unsafe extern "system" fn(this: *mut *mut Self, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppuddevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    DeviceByUDN: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUPnPDescriptionDocument {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 298959282, data2: 32170, data3: 19614, data4: [149, 149, 127, 130, 237, 32, 109, 30] };
}
#[repr(C)]
pub struct IUPnPDescriptionDocumentCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub LoadComplete: unsafe extern "system" fn(this: *mut *mut Self, hrloadresult: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUPnPDescriptionDocumentCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2000243817, data2: 21638, data3: 16598, data4: [155, 195, 73, 145, 152, 62, 2, 218] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPDevice {
    pub base__: super::super::super::System::Com::IDispatch,
    pub IsRootDevice: unsafe extern "system" fn(this: *mut *mut Self, pvarb: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RootDevice: unsafe extern "system" fn(this: *mut *mut Self, ppudrootdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootDevice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ParentDevice: unsafe extern "system" fn(this: *mut *mut Self, ppuddeviceparent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParentDevice: usize,
    pub HasChildren: unsafe extern "system" fn(this: *mut *mut Self, pvarb: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, ppudchildren: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Children: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UniqueDeviceName: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UniqueDeviceName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Type: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PresentationURL: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PresentationURL: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ManufacturerName: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ManufacturerName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ManufacturerURL: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ManufacturerURL: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ModelName: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModelName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ModelNumber: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModelNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ModelURL: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModelURL: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UPC: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UPC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SerialNumber: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SerialNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IconURL: unsafe extern "system" fn(this: *mut *mut Self, bstrencodingformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IconURL: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Services: unsafe extern "system" fn(this: *mut *mut Self, ppusservices: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Services: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUPnPDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1027920081, data2: 39113, data3: 18569, data4: [172, 209, 249, 214, 116, 191, 34, 33] };
}
#[repr(C)]
pub struct IUPnPDeviceControl {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetServiceObject: unsafe extern "system" fn(this: *mut *mut Self, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdispservice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetServiceObject: usize,
}
impl ::windows_sys::core::Interface for IUPnPDeviceControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 541593786, data2: 29618, data3: 4564, data4: [191, 66, 0, 176, 208, 17, 139, 86] };
}
#[repr(C)]
pub struct IUPnPDeviceControlHttpHeaders {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAdditionalResponseHeaders: unsafe extern "system" fn(this: *mut *mut Self, bstrhttpresponseheaders: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAdditionalResponseHeaders: usize,
}
impl ::windows_sys::core::Interface for IUPnPDeviceControlHttpHeaders {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 541593787, data2: 29618, data3: 4564, data4: [191, 66, 0, 176, 208, 17, 139, 86] };
}
#[repr(C)]
pub struct IUPnPDeviceDocumentAccess {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDocumentURL: unsafe extern "system" fn(this: *mut *mut Self, pbstrdocument: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDocumentURL: usize,
}
impl ::windows_sys::core::Interface for IUPnPDeviceDocumentAccess {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3883345924, data2: 12935, data3: 16782, data4: [144, 114, 207, 43, 71, 35, 137, 129] };
}
#[repr(C)]
pub struct IUPnPDeviceDocumentAccessEx {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDocument: unsafe extern "system" fn(this: *mut *mut Self, pbstrdocument: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDocument: usize,
}
impl ::windows_sys::core::Interface for IUPnPDeviceDocumentAccessEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3300671568, data2: 24952, data3: 19409, data4: [164, 184, 99, 152, 50, 31, 50, 71] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPDeviceFinder {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub FindByType: unsafe extern "system" fn(this: *mut *mut Self, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, pdevices: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    FindByType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateAsyncFind: unsafe extern "system" fn(this: *mut *mut Self, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, punkdevicefindercallback: *mut ::core::ffi::c_void, plfinddata: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateAsyncFind: usize,
    pub StartAsyncFind: unsafe extern "system" fn(this: *mut *mut Self, lfinddata: i32) -> ::windows_sys::core::HRESULT,
    pub CancelAsyncFind: unsafe extern "system" fn(this: *mut *mut Self, lfinddata: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub FindByUDN: unsafe extern "system" fn(this: *mut *mut Self, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    FindByUDN: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUPnPDeviceFinder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2916760917, data2: 28530, data3: 17177, data4: [191, 249, 24, 96, 10, 83, 155, 16] };
}
#[repr(C)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceAddedWithInterface: unsafe extern "system" fn(this: *mut *mut Self, lfinddata: i32, pdevice: *mut ::core::ffi::c_void, pguidinterface: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceAddedWithInterface: usize,
}
impl ::windows_sys::core::Interface for IUPnPDeviceFinderAddCallbackWithInterface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2554199051, data2: 6038, data3: 17631, data4: [137, 117, 202, 84, 91, 98, 14, 229] };
}
#[repr(C)]
pub struct IUPnPDeviceFinderCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceAdded: unsafe extern "system" fn(this: *mut *mut Self, lfinddata: i32, pdevice: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceAdded: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceRemoved: unsafe extern "system" fn(this: *mut *mut Self, lfinddata: i32, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceRemoved: usize,
    pub SearchComplete: unsafe extern "system" fn(this: *mut *mut Self, lfinddata: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUPnPDeviceFinderCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1096456266, data2: 34995, data3: 18931, data4: [146, 175, 5, 8, 190, 223, 13, 108] };
}
#[repr(C)]
pub struct IUPnPDeviceProvider {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Start: usize,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUPnPDeviceProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 541593784, data2: 29618, data3: 4564, data4: [191, 66, 0, 176, 208, 17, 139, 86] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPDevices {
    pub base__: super::super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUPnPDevices {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4256959603, data2: 48547, data3: 19558, data4: [172, 79, 242, 217, 111, 218, 214, 140] };
}
#[repr(C)]
pub struct IUPnPEventSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnStateChanged: unsafe extern "system" fn(this: *mut *mut Self, cchanges: u32, rgdispidchanges: *const i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OnStateChangedSafe: unsafe extern "system" fn(this: *mut *mut Self, varsadispidchanges: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OnStateChangedSafe: usize,
}
impl ::windows_sys::core::Interface for IUPnPEventSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 541593780, data2: 29618, data3: 4564, data4: [191, 66, 0, 176, 208, 17, 139, 86] };
}
#[repr(C)]
pub struct IUPnPEventSource {
    pub base__: ::windows_sys::core::IUnknown,
    pub Advise: unsafe extern "system" fn(this: *mut *mut Self, pessubscriber: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut *mut Self, pessubscriber: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUPnPEventSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 541593781, data2: 29618, data3: 4564, data4: [191, 66, 0, 176, 208, 17, 139, 86] };
}
#[repr(C)]
pub struct IUPnPHttpHeaderControl {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub AddRequestHeaders: unsafe extern "system" fn(this: *mut *mut Self, bstrhttpheaders: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddRequestHeaders: usize,
}
impl ::windows_sys::core::Interface for IUPnPHttpHeaderControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 67481423, data2: 35676, data3: 17532, data4: [128, 242, 183, 89, 132, 163, 31, 60] };
}
#[repr(C)]
pub struct IUPnPRegistrar {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterDevice: unsafe extern "system" fn(this: *mut *mut Self, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterDevice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterRunningDevice: unsafe extern "system" fn(this: *mut *mut Self, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterRunningDevice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterDeviceProvider: unsafe extern "system" fn(this: *mut *mut Self, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogidproviderclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterDeviceProvider: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUniqueDeviceName: unsafe extern "system" fn(this: *mut *mut Self, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrtemplateudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrudn: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUniqueDeviceName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterDevice: unsafe extern "system" fn(this: *mut *mut Self, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fpermanent: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterDevice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterDeviceProvider: unsafe extern "system" fn(this: *mut *mut Self, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterDeviceProvider: usize,
}
impl ::windows_sys::core::Interface for IUPnPRegistrar {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 541593782, data2: 29618, data3: 4564, data4: [191, 66, 0, 176, 208, 17, 139, 86] };
}
#[repr(C)]
pub struct IUPnPRemoteEndpointInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDwordValue: unsafe extern "system" fn(this: *mut *mut Self, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdwvalue: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDwordValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStringValue: unsafe extern "system" fn(this: *mut *mut Self, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStringValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGuidValue: unsafe extern "system" fn(this: *mut *mut Self, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pguidvalue: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGuidValue: usize,
}
impl ::windows_sys::core::Interface for IUPnPRemoteEndpointInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3375282275, data2: 617, data3: 19199, data4: [156, 114, 117, 50, 27, 186, 41, 82] };
}
#[repr(C)]
pub struct IUPnPReregistrar {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub ReregisterDevice: unsafe extern "system" fn(this: *mut *mut Self, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReregisterDevice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReregisterRunningDevice: unsafe extern "system" fn(this: *mut *mut Self, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReregisterRunningDevice: usize,
}
impl ::windows_sys::core::Interface for IUPnPReregistrar {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 541593783, data2: 29618, data3: 4564, data4: [191, 66, 0, 176, 208, 17, 139, 86] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPService {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub QueryStateVariable: unsafe extern "system" fn(this: *mut *mut Self, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    QueryStateVariable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InvokeAction: unsafe extern "system" fn(this: *mut *mut Self, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InvokeAction: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceTypeIdentifier: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceTypeIdentifier: usize,
    pub AddCallback: unsafe extern "system" fn(this: *mut *mut Self, punkcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pbstrid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    pub LastTransportStatus: unsafe extern "system" fn(this: *mut *mut Self, plvalue: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUPnPService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2727674268, data2: 56421, data3: 18397, data4: [144, 220, 127, 233, 24, 161, 171, 68] };
}
#[repr(C)]
pub struct IUPnPServiceAsync {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BeginInvokeAction: unsafe extern "system" fn(this: *mut *mut Self, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BeginInvokeAction: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EndInvokeAction: unsafe extern "system" fn(this: *mut *mut Self, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EndInvokeAction: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginQueryStateVariable: unsafe extern "system" fn(this: *mut *mut Self, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginQueryStateVariable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EndQueryStateVariable: unsafe extern "system" fn(this: *mut *mut Self, ullrequestid: u64, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EndQueryStateVariable: usize,
    pub BeginSubscribeToEvents: unsafe extern "system" fn(this: *mut *mut Self, punkcallback: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_sys::core::HRESULT,
    pub EndSubscribeToEvents: unsafe extern "system" fn(this: *mut *mut Self, ullrequestid: u64) -> ::windows_sys::core::HRESULT,
    pub BeginSCPDDownload: unsafe extern "system" fn(this: *mut *mut Self, pasyncresult: *mut ::core::ffi::c_void, pullrequestid: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EndSCPDDownload: unsafe extern "system" fn(this: *mut *mut Self, ullrequestid: u64, pbstrscpddoc: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndSCPDDownload: usize,
    pub CancelAsyncOperation: unsafe extern "system" fn(this: *mut *mut Self, ullrequestid: u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUPnPServiceAsync {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 160160501, data2: 24257, data3: 18919, data4: [162, 96, 179, 161, 29, 216, 104, 12] };
}
#[repr(C)]
pub struct IUPnPServiceCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StateVariableChanged: unsafe extern "system" fn(this: *mut *mut Self, pus: *mut ::core::ffi::c_void, pcwszstatevarname: ::windows_sys::core::PCWSTR, vavalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StateVariableChanged: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ServiceInstanceDied: unsafe extern "system" fn(this: *mut *mut Self, pus: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ServiceInstanceDied: usize,
}
impl ::windows_sys::core::Interface for IUPnPServiceCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 838524073, data2: 43891, data3: 17995, data4: [182, 125, 92, 29, 15, 131, 200, 184] };
}
#[repr(C)]
pub struct IUPnPServiceDocumentAccess {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDocumentURL: unsafe extern "system" fn(this: *mut *mut Self, pbstrdocurl: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDocumentURL: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDocument: unsafe extern "system" fn(this: *mut *mut Self, pbstrdoc: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDocument: usize,
}
impl ::windows_sys::core::Interface for IUPnPServiceDocumentAccess {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 563107113, data2: 2654, data3: 17801, data4: [130, 93, 126, 109, 135, 234, 105, 152] };
}
#[repr(C)]
pub struct IUPnPServiceEnumProperty {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetServiceEnumProperty: unsafe extern "system" fn(this: *mut *mut Self, dwmask: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUPnPServiceEnumProperty {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 948386615, data2: 37307, data3: 18932, data4: [178, 73, 46, 142, 251, 184, 168, 22] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPServices {
    pub base__: super::super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IUPnPServices {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1066176158, data2: 39546, data3: 19912, data4: [188, 65, 255, 49, 250, 55, 73, 86] };
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const REMOTE_ADDRESS_VALUE_NAME: &str = "RemoteAddress";
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub type SW_DEVICE_CAPABILITIES = i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesNone: SW_DEVICE_CAPABILITIES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesRemovable: SW_DEVICE_CAPABILITIES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesSilentInstall: SW_DEVICE_CAPABILITIES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesNoDisplayInUI: SW_DEVICE_CAPABILITIES = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceCapabilitiesDriverRequired: SW_DEVICE_CAPABILITIES = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub type SW_DEVICE_CREATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hswdevice: HSWDEVICE, createresult: ::windows_sys::core::HRESULT, pcontext: *const ::core::ffi::c_void, pszdeviceinstanceid: ::windows_sys::core::PCWSTR)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SW_DEVICE_CREATE_INFO {
    pub cbSize: u32,
    pub pszInstanceId: ::windows_sys::core::PCWSTR,
    pub pszzHardwareIds: ::windows_sys::core::PCWSTR,
    pub pszzCompatibleIds: ::windows_sys::core::PCWSTR,
    pub pContainerId: *const ::windows_sys::core::GUID,
    pub CapabilityFlags: u32,
    pub pszDeviceDescription: ::windows_sys::core::PCWSTR,
    pub pszDeviceLocation: ::windows_sys::core::PCWSTR,
    pub pSecurityDescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for SW_DEVICE_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for SW_DEVICE_CREATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub type SW_DEVICE_LIFETIME = i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceLifetimeHandle: SW_DEVICE_LIFETIME = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceLifetimeParentPresent: SW_DEVICE_LIFETIME = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const SWDeviceLifetimeMax: SW_DEVICE_LIFETIME = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_ADDRESSFAMILY_BOTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_ADDRESSFAMILY_IPv4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_ADDRESSFAMILY_IPv6: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ACTION_REQUEST_FAILED: ::windows_sys::core::HRESULT = -2147220976i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ACTION_SPECIFIC_BASE: ::windows_sys::core::HRESULT = -2147220736i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_ELEMENT_EXPECTED: ::windows_sys::core::HRESULT = -2147220991i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_ERROR: ::windows_sys::core::HRESULT = -2147220972i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_NODE_INCOMPLETE: ::windows_sys::core::HRESULT = -2147220988i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_NOTREGISTERED: ::windows_sys::core::HRESULT = -2147180494i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_RUNNING: ::windows_sys::core::HRESULT = -2147180495i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DEVICE_TIMEOUT: ::windows_sys::core::HRESULT = -2147220969i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DUPLICATE_NOT_ALLOWED: ::windows_sys::core::HRESULT = -2147180511i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_DUPLICATE_SERVICE_ID: ::windows_sys::core::HRESULT = -2147180510i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ERROR_PROCESSING_RESPONSE: ::windows_sys::core::HRESULT = -2147220970i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_EVENT_SUBSCRIPTION_FAILED: ::windows_sys::core::HRESULT = -2147220223i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ICON_ELEMENT_EXPECTED: ::windows_sys::core::HRESULT = -2147220987i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ICON_NODE_INCOMPLETE: ::windows_sys::core::HRESULT = -2147220986i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_ACTION: ::windows_sys::core::HRESULT = -2147220985i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_ARGUMENTS: ::windows_sys::core::HRESULT = -2147220984i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_DESCRIPTION: ::windows_sys::core::HRESULT = -2147180509i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_DOCUMENT: ::windows_sys::core::HRESULT = -2147220224i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_ICON: ::windows_sys::core::HRESULT = -2147180507i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_ROOT_NAMESPACE: ::windows_sys::core::HRESULT = -2147180505i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_SERVICE: ::windows_sys::core::HRESULT = -2147180508i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_VARIABLE: ::windows_sys::core::HRESULT = -2147220973i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_INVALID_XML: ::windows_sys::core::HRESULT = -2147180506i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_OUT_OF_SYNC: ::windows_sys::core::HRESULT = -2147220983i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_PROTOCOL_ERROR: ::windows_sys::core::HRESULT = -2147220971i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_REQUIRED_ELEMENT_ERROR: ::windows_sys::core::HRESULT = -2147180512i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_ROOT_ELEMENT_EXPECTED: ::windows_sys::core::HRESULT = -2147220992i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_SERVICE_ELEMENT_EXPECTED: ::windows_sys::core::HRESULT = -2147220990i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_SERVICE_NODE_INCOMPLETE: ::windows_sys::core::HRESULT = -2147220989i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_SUFFIX_TOO_LONG: ::windows_sys::core::HRESULT = -2147180504i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_TRANSPORT_ERROR: ::windows_sys::core::HRESULT = -2147220975i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_URLBASE_PRESENT: ::windows_sys::core::HRESULT = -2147180503i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_VALUE_TOO_LONG: ::windows_sys::core::HRESULT = -2147180496i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_E_VARIABLE_VALUE_UNKNOWN: ::windows_sys::core::HRESULT = -2147220974i32;
#[doc = "*Required features: `\"Win32_Devices_Enumeration_Pnp\"`*"]
pub const UPNP_SERVICE_DELAY_SCPD_AND_SUBSCRIPTION: u32 = 1u32;
pub const UPnPDescriptionDocument: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 495622983, data2: 14888, data3: 19682, data4: [138, 75, 189, 52, 228, 91, 206, 235] };
pub const UPnPDescriptionDocumentEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 872220003, data2: 55322, data3: 17299, data4: [131, 204, 1, 149, 177, 218, 47, 145] };
pub const UPnPDevice: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2737132229, data2: 47713, data3: 17786, data4: [181, 154, 162, 86, 30, 18, 94, 51] };
pub const UPnPDeviceFinder: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3792199464, data2: 65207, data3: 16458, data4: [184, 231, 230, 89, 189, 234, 170, 2] };
pub const UPnPDeviceFinderEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 404444412, data2: 14347, data3: 19061, data4: [179, 241, 74, 196, 94, 150, 5, 176] };
pub const UPnPDevices: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3119009789, data2: 44348, data3: 16548, data4: [184, 53, 8, 130, 235, 203, 170, 168] };
pub const UPnPRegistrar: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 541593785, data2: 29618, data3: 4564, data4: [191, 66, 0, 176, 208, 17, 139, 86] };
pub const UPnPRemoteEndpointInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 777946345, data2: 16457, data3: 16964, data4: [183, 40, 45, 36, 34, 113, 87, 199] };
pub const UPnPService: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3324295829, data2: 64459, data3: 17417, data4: [140, 3, 140, 206, 236, 83, 62, 241] };
pub const UPnPServices: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3233565514, data2: 41990, data3: 20220, data4: [147, 47, 184, 84, 107, 129, 0, 204] };
