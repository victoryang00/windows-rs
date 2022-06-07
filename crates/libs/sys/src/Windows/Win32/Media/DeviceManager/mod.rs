#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const ALLOW_OUTOFBAND_NOTIFICATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const DO_NOT_VIRTUALIZE_STORAGES_AS_DEVICES: u32 = 1u32;
pub const EVENT_WMDM_CONTENT_TRANSFER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 865901556, data2: 48382, data3: 20184, data4: [148, 223, 234, 248, 194, 106, 182, 27] };
#[repr(C)]
pub struct IComponentAuthenticate {
    pub base__: ::windows_sys::core::IUnknown,
    pub SACAuth: unsafe extern "system" fn(this: *mut *mut Self, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SACGetProtocols: unsafe extern "system" fn(this: *mut *mut Self, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IComponentAuthenticate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2844302336, data2: 27947, data3: 4563, data4: [132, 150, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IMDSPDevice {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PWSTR, nmaxchars: u32) -> ::windows_sys::core::HRESULT,
    pub GetManufacturer: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PWSTR, nmaxchars: u32) -> ::windows_sys::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut *mut Self, pdwversion: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, pdwtype: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut *mut Self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub GetPowerSource: unsafe extern "system" fn(this: *mut *mut Self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDeviceIcon: unsafe extern "system" fn(this: *mut *mut Self, hicon: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut *mut Self, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFormatSupport: unsafe extern "system" fn(this: *mut *mut Self, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_sys::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut *mut Self, pcommand: *mut OPAQUECOMMAND) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857938, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IMDSPDevice2 {
    pub base__: IMDSPDevice,
    pub GetStorage: unsafe extern "system" fn(this: *mut *mut Self, pszstoragename: ::windows_sys::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFormatSupport2: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFormatSupport2: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetSpecifyPropertyPages: unsafe extern "system" fn(this: *mut *mut Self, ppspecifyproppages: *mut *mut ::core::ffi::c_void, pppunknowns: *mut *mut *mut ::core::ffi::c_void, pcunks: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetSpecifyPropertyPages: usize,
    pub GetCanonicalName: unsafe extern "system" fn(this: *mut *mut Self, pwszpnpname: ::windows_sys::core::PWSTR, nmaxchars: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPDevice2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1108154029, data2: 51581, data3: 19968, data4: [130, 170, 0, 233, 244, 51, 93, 221] };
}
#[repr(C)]
pub struct IMDSPDevice3 {
    pub base__: IMDSPDevice2,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, pwszpropname: ::windows_sys::core::PCWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, pwszpropname: ::windows_sys::core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetFormatCapability: unsafe extern "system" fn(this: *mut *mut Self, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetFormatCapability: usize,
    pub DeviceIoControl: unsafe extern "system" fn(this: *mut *mut Self, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut *mut Self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows_sys::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPDevice3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 444831813, data2: 64597, data3: 18556, data4: [151, 111, 238, 56, 172, 14, 140, 78] };
}
#[repr(C)]
pub struct IMDSPDeviceControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDCStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut *mut Self, pdwcapabilitiesmask: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Record: unsafe extern "system" fn(this: *mut *mut Self, pformat: *const _WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32, noffset: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPDeviceControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857940, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IMDSPDirectTransfer {
    pub base__: ::windows_sys::core::IUnknown,
    pub TransferToDevice: unsafe extern "system" fn(this: *mut *mut Self, pwszsourcefilepath: ::windows_sys::core::PCWSTR, psourceoperation: *mut ::core::ffi::c_void, fuflags: u32, pwszdestinationname: ::windows_sys::core::PCWSTR, psourcemetadata: *mut ::core::ffi::c_void, ptransferprogress: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPDirectTransfer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3271448488, data2: 37636, data3: 18316, data4: [158, 228, 71, 227, 151, 185, 18, 215] };
}
#[repr(C)]
pub struct IMDSPEnumDevice {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppdevice: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPEnumDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857937, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IMDSPEnumStorage {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppstorage: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPEnumStorage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857941, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IMDSPObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32) -> ::windows_sys::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut *mut Self, fuflags: u32, dwoffset: u32) -> ::windows_sys::core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut *mut Self, pwsznewname: ::windows_sys::core::PCWSTR, pprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32, pprogress: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857944, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IMDSPObject2 {
    pub base__: IMDSPObject,
    pub ReadOnClearChannel: unsafe extern "system" fn(this: *mut *mut Self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub WriteOnClearChannel: unsafe extern "system" fn(this: *mut *mut Self, pdata: *const u8, pdwsize: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPObject2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1060425022, data2: 22791, data3: 17217, data4: [154, 249, 151, 244, 24, 124, 58, 165] };
}
#[repr(C)]
pub struct IMDSPObjectInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPlayLength: unsafe extern "system" fn(this: *mut *mut Self, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetPlayLength: unsafe extern "system" fn(this: *mut *mut Self, dwlength: u32) -> ::windows_sys::core::HRESULT,
    pub GetPlayOffset: unsafe extern "system" fn(this: *mut *mut Self, pdwoffset: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetPlayOffset: unsafe extern "system" fn(this: *mut *mut Self, dwoffset: u32) -> ::windows_sys::core::HRESULT,
    pub GetTotalLength: unsafe extern "system" fn(this: *mut *mut Self, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLastPlayPosition: unsafe extern "system" fn(this: *mut *mut Self, pdwlastpos: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLongestPlayPosition: unsafe extern "system" fn(this: *mut *mut Self, pdwlongestpos: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPObjectInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857945, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IMDSPRevoked {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRevocationURL: unsafe extern "system" fn(this: *mut *mut Self, ppwszrevocationurl: *mut ::windows_sys::core::PWSTR, pdwbufferlen: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPRevoked {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2766729940, data2: 16177, data3: 17997, data4: [181, 61, 79, 195, 53, 153, 129, 132] };
}
#[repr(C)]
pub struct IMDSPStorage {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAttributes: unsafe extern "system" fn(this: *mut *mut Self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    pub GetStorageGlobals: unsafe extern "system" fn(this: *mut *mut Self, ppstorageglobals: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(this: *mut *mut Self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PWSTR, nmaxchars: u32) -> ::windows_sys::core::HRESULT,
    pub GetDate: unsafe extern "system" fn(this: *mut *mut Self, pdatetimeutc: *mut WMDMDATETIME) -> ::windows_sys::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut *mut Self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetRights: unsafe extern "system" fn(this: *mut *mut Self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub CreateStorage: unsafe extern "system" fn(this: *mut *mut Self, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: ::windows_sys::core::PCWSTR, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut *mut Self, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut *mut Self, pcommand: *mut OPAQUECOMMAND) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPStorage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857942, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IMDSPStorage2 {
    pub base__: IMDSPStorage,
    pub GetStorage: unsafe extern "system" fn(this: *mut *mut Self, pszstoragename: ::windows_sys::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateStorage2: unsafe extern "system" fn(this: *mut *mut Self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: ::windows_sys::core::PCWSTR, qwfilesize: u64, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateStorage2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAttributes2: unsafe extern "system" fn(this: *mut *mut Self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAttributes2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributes2: unsafe extern "system" fn(this: *mut *mut Self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributes2: usize,
}
impl ::windows_sys::core::Interface for IMDSPStorage2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 173934501, data2: 25684, data3: 17489, data4: [156, 54, 28, 106, 231, 226, 177, 214] };
}
#[repr(C)]
pub struct IMDSPStorage3 {
    pub base__: IMDSPStorage2,
    pub GetMetadata: unsafe extern "system" fn(this: *mut *mut Self, pmetadata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(this: *mut *mut Self, pmetadata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPStorage3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1818663015, data2: 38893, data3: 19047, data4: [151, 6, 28, 85, 41, 210, 164, 20] };
}
#[repr(C)]
pub struct IMDSPStorage4 {
    pub base__: IMDSPStorage3,
    pub SetReferences: unsafe extern "system" fn(this: *mut *mut Self, dwrefs: u32, ppispstorage: *const *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetReferences: unsafe extern "system" fn(this: *mut *mut Self, pdwrefs: *mut u32, pppispstorage: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateStorageWithMetadata: unsafe extern "system" fn(this: *mut *mut Self, dwattributes: u32, pwszname: ::windows_sys::core::PCWSTR, pmetadata: *mut ::core::ffi::c_void, qwfilesize: u64, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSpecifiedMetadata: unsafe extern "system" fn(this: *mut *mut Self, cproperties: u32, ppwszpropnames: *const ::windows_sys::core::PWSTR, pmetadata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut *mut Self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows_sys::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut *mut Self, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPStorage4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 825471684, data2: 20828, data3: 18459, data4: [177, 206, 57, 50, 126, 203, 79, 116] };
}
#[repr(C)]
pub struct IMDSPStorageGlobals {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut *mut Self, pdwcapabilities: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut *mut Self, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub GetTotalSize: unsafe extern "system" fn(this: *mut *mut Self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetTotalFree: unsafe extern "system" fn(this: *mut *mut Self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetTotalBad: unsafe extern "system" fn(this: *mut *mut Self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut *mut Self, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRootStorage: unsafe extern "system" fn(this: *mut *mut Self, pproot: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDSPStorageGlobals {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857943, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IMDServiceProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDeviceCount: unsafe extern "system" fn(this: *mut *mut Self, pdwcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(this: *mut *mut Self, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDServiceProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857936, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IMDServiceProvider2 {
    pub base__: IMDServiceProvider,
    pub CreateDevice: unsafe extern "system" fn(this: *mut *mut Self, pwszdevicepath: ::windows_sys::core::PCWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDServiceProvider2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3002737847, data2: 52643, data3: 18068, data4: [152, 98, 65, 58, 225, 163, 72, 25] };
}
#[repr(C)]
pub struct IMDServiceProvider3 {
    pub base__: IMDServiceProvider2,
    pub SetDeviceEnumPreference: unsafe extern "system" fn(this: *mut *mut Self, dwenumpref: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMDServiceProvider3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1322335987, data2: 43377, data3: 19737, data4: [159, 81, 14, 24, 38, 178, 218, 87] };
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const IOCTL_MTP_CUSTOM_COMMAND: u32 = 827348045u32;
#[repr(C)]
pub struct ISCPSecureAuthenticate {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSecureQuery: unsafe extern "system" fn(this: *mut *mut Self, ppsecurequery: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISCPSecureAuthenticate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857935, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct ISCPSecureAuthenticate2 {
    pub base__: ISCPSecureAuthenticate,
    pub GetSCPSession: unsafe extern "system" fn(this: *mut *mut Self, ppscpsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISCPSecureAuthenticate2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3045117870, data2: 5746, data3: 18402, data4: [172, 170, 68, 187, 236, 188, 174, 91] };
}
#[repr(C)]
pub struct ISCPSecureExchange {
    pub base__: ::windows_sys::core::IUnknown,
    pub TransferContainerData: unsafe extern "system" fn(this: *mut *mut Self, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ObjectData: unsafe extern "system" fn(this: *mut *mut Self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub TransferComplete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISCPSecureExchange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857934, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct ISCPSecureExchange2 {
    pub base__: ISCPSecureExchange,
    pub TransferContainerData2: unsafe extern "system" fn(this: *mut *mut Self, pdata: *const u8, dwsize: u32, pprogresscallback: *mut ::core::ffi::c_void, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISCPSecureExchange2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1818426491, data2: 9872, data3: 18495, data4: [157, 68, 10, 32, 203, 53, 87, 124] };
}
#[repr(C)]
pub struct ISCPSecureExchange3 {
    pub base__: ISCPSecureExchange2,
    pub TransferContainerDataOnClearChannel: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: *mut ::core::ffi::c_void, pfureadyflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetObjectDataOnClearChannel: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TransferCompleteForDevice: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISCPSecureExchange3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2874046436, data2: 35080, data3: 19223, data4: [189, 42, 177, 219, 230, 221, 105, 225] };
}
#[repr(C)]
pub struct ISCPSecureQuery {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDataDemands: unsafe extern "system" fn(this: *mut *mut Self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ExamineData: unsafe extern "system" fn(this: *mut *mut Self, fuflags: u32, pwszextension: ::windows_sys::core::PCWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub MakeDecision: unsafe extern "system" fn(this: *mut *mut Self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub GetRights: unsafe extern "system" fn(this: *mut *mut Self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISCPSecureQuery {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857933, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct ISCPSecureQuery2 {
    pub base__: ISCPSecureQuery,
    pub MakeDecision2: unsafe extern "system" fn(this: *mut *mut Self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows_sys::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISCPSecureQuery2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3957423653, data2: 20439, data3: 17970, data4: [175, 70, 109, 147, 212, 252, 199, 46] };
}
#[repr(C)]
pub struct ISCPSecureQuery3 {
    pub base__: ISCPSecureQuery2,
    pub GetRightsOnClearChannel: unsafe extern "system" fn(this: *mut *mut Self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut ::core::ffi::c_void, pprogresscallback: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MakeDecisionOnClearChannel: unsafe extern "system" fn(this: *mut *mut Self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, pprogresscallback: *mut ::core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows_sys::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISCPSecureQuery3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3085816226, data2: 19883, data3: 18507, data4: [179, 197, 173, 57, 184, 180, 192, 177] };
}
#[repr(C)]
pub struct ISCPSession {
    pub base__: ::windows_sys::core::IUnknown,
    pub BeginSession: unsafe extern "system" fn(this: *mut *mut Self, pidevice: *mut ::core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> ::windows_sys::core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut *mut Self, pctx: *const u8, dwsizectx: u32) -> ::windows_sys::core::HRESULT,
    pub GetSecureQuery: unsafe extern "system" fn(this: *mut *mut Self, ppsecurequery: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISCPSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2292442861, data2: 61156, data3: 17945, data4: [187, 179, 253, 79, 182, 39, 21, 209] };
}
#[repr(C)]
pub struct IWMDMDevice {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PWSTR, nmaxchars: u32) -> ::windows_sys::core::HRESULT,
    pub GetManufacturer: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PWSTR, nmaxchars: u32) -> ::windows_sys::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut *mut Self, pdwversion: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, pdwtype: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut *mut Self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub GetPowerSource: unsafe extern "system" fn(this: *mut *mut Self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDeviceIcon: unsafe extern "system" fn(this: *mut *mut Self, hicon: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut *mut Self, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFormatSupport: unsafe extern "system" fn(this: *mut *mut Self, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_sys::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut *mut Self, pcommand: *mut OPAQUECOMMAND) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857922, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IWMDMDevice2 {
    pub base__: IWMDMDevice,
    pub GetStorage: unsafe extern "system" fn(this: *mut *mut Self, pszstoragename: ::windows_sys::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFormatSupport2: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFormatSupport2: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetSpecifyPropertyPages: unsafe extern "system" fn(this: *mut *mut Self, ppspecifyproppages: *mut *mut ::core::ffi::c_void, pppunknowns: *mut *mut *mut ::core::ffi::c_void, pcunks: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetSpecifyPropertyPages: usize,
    pub GetCanonicalName: unsafe extern "system" fn(this: *mut *mut Self, pwszpnpname: ::windows_sys::core::PWSTR, nmaxchars: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMDevice2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3813621047, data2: 40295, data3: 20417, data4: [146, 82, 98, 210, 139, 47, 139, 85] };
}
#[repr(C)]
pub struct IWMDMDevice3 {
    pub base__: IWMDMDevice2,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, pwszpropname: ::windows_sys::core::PCWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, pwszpropname: ::windows_sys::core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetFormatCapability: unsafe extern "system" fn(this: *mut *mut Self, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetFormatCapability: usize,
    pub DeviceIoControl: unsafe extern "system" fn(this: *mut *mut Self, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut *mut Self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows_sys::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMDevice3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1812194558, data2: 1499, data3: 19930, data4: [158, 60, 6, 35, 58, 109, 93, 101] };
}
#[repr(C)]
pub struct IWMDMDeviceControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut *mut Self, pdwcapabilitiesmask: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Record: unsafe extern "system" fn(this: *mut *mut Self, pformat: *const _WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32, noffset: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMDeviceControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857924, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IWMDMDeviceSession {
    pub base__: ::windows_sys::core::IUnknown,
    pub BeginSession: unsafe extern "system" fn(this: *mut *mut Self, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows_sys::core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut *mut Self, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMDeviceSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2192509541, data2: 40342, data3: 16684, data4: [131, 229, 60, 67, 228, 176, 108, 199] };
}
#[repr(C)]
pub struct IWMDMEnumDevice {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppdevice: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMEnumDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857921, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IWMDMEnumStorage {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppstorage: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMEnumStorage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857925, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IWMDMLogger {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enable: usize,
    pub GetLogFileName: unsafe extern "system" fn(this: *mut *mut Self, pszfilename: ::windows_sys::core::PSTR, nmaxchars: u32) -> ::windows_sys::core::HRESULT,
    pub SetLogFileName: unsafe extern "system" fn(this: *mut *mut Self, pszfilename: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT,
    pub LogString: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pszsrcname: ::windows_sys::core::PCSTR, pszlog: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT,
    pub LogDword: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pszsrcname: ::windows_sys::core::PCSTR, pszlogformat: ::windows_sys::core::PCSTR, dwlog: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetSizeParams: unsafe extern "system" fn(this: *mut *mut Self, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSizeParams: unsafe extern "system" fn(this: *mut *mut Self, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMLogger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 285880832, data2: 23161, data3: 4563, data4: [141, 120, 68, 69, 83, 84, 0, 0] };
}
#[repr(C)]
pub struct IWMDMMetaData {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddItem: unsafe extern "system" fn(this: *mut *mut Self, r#type: WMDM_TAG_DATATYPE, pwsztagname: ::windows_sys::core::PCWSTR, pvalue: *const u8, ilength: u32) -> ::windows_sys::core::HRESULT,
    pub QueryByName: unsafe extern "system" fn(this: *mut *mut Self, pwsztagname: ::windows_sys::core::PCWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub QueryByIndex: unsafe extern "system" fn(this: *mut *mut Self, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetItemCount: unsafe extern "system" fn(this: *mut *mut Self, icount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMMetaData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3963291235, data2: 2385, data3: 17930, data4: [154, 128, 13, 206, 237, 60, 4, 60] };
}
#[repr(C)]
pub struct IWMDMNotification {
    pub base__: ::windows_sys::core::IUnknown,
    pub WMDMMessage: unsafe extern "system" fn(this: *mut *mut Self, dwmessagetype: u32, pwszcanonicalname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMNotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1063163328, data2: 3907, data3: 20180, data4: [147, 210, 200, 154, 69, 213, 155, 129] };
}
#[repr(C)]
pub struct IWMDMObjectInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPlayLength: unsafe extern "system" fn(this: *mut *mut Self, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetPlayLength: unsafe extern "system" fn(this: *mut *mut Self, dwlength: u32) -> ::windows_sys::core::HRESULT,
    pub GetPlayOffset: unsafe extern "system" fn(this: *mut *mut Self, pdwoffset: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetPlayOffset: unsafe extern "system" fn(this: *mut *mut Self, dwoffset: u32) -> ::windows_sys::core::HRESULT,
    pub GetTotalLength: unsafe extern "system" fn(this: *mut *mut Self, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLastPlayPosition: unsafe extern "system" fn(this: *mut *mut Self, pdwlastpos: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLongestPlayPosition: unsafe extern "system" fn(this: *mut *mut Self, pdwlongestpos: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMObjectInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857929, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IWMDMOperation {
    pub base__: ::windows_sys::core::IUnknown,
    pub BeginRead: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BeginWrite: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetObjectName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PWSTR, nmaxchars: u32) -> ::windows_sys::core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR, nmaxchars: u32) -> ::windows_sys::core::HRESULT,
    pub GetObjectAttributes: unsafe extern "system" fn(this: *mut *mut Self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    pub SetObjectAttributes: unsafe extern "system" fn(this: *mut *mut Self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    pub GetObjectTotalSize: unsafe extern "system" fn(this: *mut *mut Self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetObjectTotalSize: unsafe extern "system" fn(this: *mut *mut Self, dwsize: u32, dwsizehigh: u32) -> ::windows_sys::core::HRESULT,
    pub TransferObjectData: unsafe extern "system" fn(this: *mut *mut Self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut *mut Self, phcompletioncode: *const ::windows_sys::core::HRESULT, pnewobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857931, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IWMDMOperation2 {
    pub base__: IWMDMOperation,
    #[cfg(feature = "Win32_Foundation")]
    pub SetObjectAttributes2: unsafe extern "system" fn(this: *mut *mut Self, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetObjectAttributes2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetObjectAttributes2: unsafe extern "system" fn(this: *mut *mut Self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetObjectAttributes2: usize,
}
impl ::windows_sys::core::Interface for IWMDMOperation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 860117832, data2: 32247, data3: 16988, data4: [173, 143, 15, 198, 216, 47, 159, 117] };
}
#[repr(C)]
pub struct IWMDMOperation3 {
    pub base__: IWMDMOperation,
    pub TransferObjectDataOnClearChannel: unsafe extern "system" fn(this: *mut *mut Self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMOperation3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3522802794, data2: 40104, data3: 18136, data4: [157, 15, 30, 201, 186, 229, 73, 25] };
}
#[repr(C)]
pub struct IWMDMProgress {
    pub base__: ::windows_sys::core::IUnknown,
    pub Begin: unsafe extern "system" fn(this: *mut *mut Self, dwestimatedticks: u32) -> ::windows_sys::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, dwtranspiredticks: u32) -> ::windows_sys::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMProgress {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857932, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IWMDMProgress2 {
    pub base__: IWMDMProgress,
    pub End2: unsafe extern "system" fn(this: *mut *mut Self, hrcompletioncode: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMProgress2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 977532240, data2: 45955, data3: 20114, data4: [176, 74, 230, 187, 198, 96, 254, 252] };
}
#[repr(C)]
pub struct IWMDMProgress3 {
    pub base__: IWMDMProgress2,
    pub Begin3: unsafe extern "system" fn(this: *mut *mut Self, eventid: ::windows_sys::core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows_sys::core::HRESULT,
    pub Progress3: unsafe extern "system" fn(this: *mut *mut Self, eventid: ::windows_sys::core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows_sys::core::HRESULT,
    pub End3: unsafe extern "system" fn(this: *mut *mut Self, eventid: ::windows_sys::core::GUID, hrcompletioncode: ::windows_sys::core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMProgress3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 568197579, data2: 15284, data3: 18729, data4: [178, 26, 23, 175, 63, 128, 246, 88] };
}
#[repr(C)]
pub struct IWMDMRevoked {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRevocationURL: unsafe extern "system" fn(this: *mut *mut Self, ppwszrevocationurl: *mut ::windows_sys::core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMRevoked {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3958165211, data2: 35054, data3: 20053, data4: [182, 164, 141, 159, 7, 214, 150, 170] };
}
#[repr(C)]
pub struct IWMDMStorage {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAttributes: unsafe extern "system" fn(this: *mut *mut Self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    pub GetStorageGlobals: unsafe extern "system" fn(this: *mut *mut Self, ppstorageglobals: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(this: *mut *mut Self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PWSTR, nmaxchars: u32) -> ::windows_sys::core::HRESULT,
    pub GetDate: unsafe extern "system" fn(this: *mut *mut Self, pdatetimeutc: *mut WMDMDATETIME) -> ::windows_sys::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut *mut Self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetRights: unsafe extern "system" fn(this: *mut *mut Self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut *mut Self, penumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut *mut Self, pcommand: *mut OPAQUECOMMAND) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMStorage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857926, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IWMDMStorage2 {
    pub base__: IWMDMStorage,
    pub GetStorage: unsafe extern "system" fn(this: *mut *mut Self, pszstoragename: ::windows_sys::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAttributes2: unsafe extern "system" fn(this: *mut *mut Self, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAttributes2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributes2: unsafe extern "system" fn(this: *mut *mut Self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributes2: usize,
}
impl ::windows_sys::core::Interface for IWMDMStorage2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 517316932, data2: 23765, data3: 18051, data4: [158, 255, 114, 203, 219, 45, 149, 51] };
}
#[repr(C)]
pub struct IWMDMStorage3 {
    pub base__: IWMDMStorage2,
    pub GetMetadata: unsafe extern "system" fn(this: *mut *mut Self, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(this: *mut *mut Self, pmetadata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateEmptyMetadataObject: unsafe extern "system" fn(this: *mut *mut Self, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEnumPreference: unsafe extern "system" fn(this: *mut *mut Self, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMStorage3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2540797674, data2: 37482, data3: 17998, data4: [150, 164, 36, 123, 2, 22, 2, 110] };
}
#[repr(C)]
pub struct IWMDMStorage4 {
    pub base__: IWMDMStorage3,
    pub SetReferences: unsafe extern "system" fn(this: *mut *mut Self, dwrefs: u32, ppiwmdmstorage: *const *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetReferences: unsafe extern "system" fn(this: *mut *mut Self, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRightsWithProgress: unsafe extern "system" fn(this: *mut *mut Self, piprogresscallback: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSpecifiedMetadata: unsafe extern "system" fn(this: *mut *mut Self, cproperties: u32, ppwszpropnames: *const ::windows_sys::core::PWSTR, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut *mut Self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows_sys::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut *mut Self, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMStorage4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3257252549, data2: 41018, data3: 16568, data4: [154, 35, 145, 207, 71, 140, 100, 166] };
}
#[repr(C)]
pub struct IWMDMStorageControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub Insert: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32, pwszfile: ::windows_sys::core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32, pwsznewname: ::windows_sys::core::PCWSTR, pprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32, pwszfile: ::windows_sys::core::PCWSTR, pprogress: *mut ::core::ffi::c_void, poperation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32, ptargetobject: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMStorageControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857928, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IWMDMStorageControl2 {
    pub base__: IWMDMStorageControl,
    pub Insert2: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32, pwszfilesource: ::windows_sys::core::PCWSTR, pwszfiledest: ::windows_sys::core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMStorageControl2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2536255112, data2: 48492, data3: 16677, data4: [142, 9, 132, 248, 55, 230, 55, 182] };
}
#[repr(C)]
pub struct IWMDMStorageControl3 {
    pub base__: IWMDMStorageControl2,
    pub Insert3: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32, futype: u32, pwszfilesource: ::windows_sys::core::PCWSTR, pwszfiledest: ::windows_sys::core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMStorageControl3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3005637477, data2: 54515, data3: 18070, data4: [141, 83, 189, 39, 236, 96, 153, 58] };
}
#[repr(C)]
pub struct IWMDMStorageGlobals {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut *mut Self, pdwcapabilities: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut *mut Self, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows_sys::core::HRESULT,
    pub GetTotalSize: unsafe extern "system" fn(this: *mut *mut Self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetTotalFree: unsafe extern "system" fn(this: *mut *mut Self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetTotalBad: unsafe extern "system" fn(this: *mut *mut Self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDMStorageGlobals {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857927, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IWMDeviceManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRevision: unsafe extern "system" fn(this: *mut *mut Self, pdwrevision: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDeviceCount: unsafe extern "system" fn(this: *mut *mut Self, pdwcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(this: *mut *mut Self, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDeviceManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 499857920, data2: 13293, data3: 4563, data4: [132, 112, 0, 192, 79, 121, 219, 192] };
}
#[repr(C)]
pub struct IWMDeviceManager2 {
    pub base__: IWMDeviceManager,
    pub GetDeviceFromCanonicalName: unsafe extern "system" fn(this: *mut *mut Self, pwszcanonicalname: ::windows_sys::core::PCWSTR, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumDevices2: unsafe extern "system" fn(this: *mut *mut Self, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Reinitialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDeviceManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2453557833, data2: 34609, data3: 19547, data4: [155, 28, 184, 182, 11, 110, 70, 175] };
}
#[repr(C)]
pub struct IWMDeviceManager3 {
    pub base__: IWMDeviceManager2,
    pub SetDeviceEnumPreference: unsafe extern "system" fn(this: *mut *mut Self, dwenumpref: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWMDeviceManager3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2937609281, data2: 4109, data3: 18157, data4: [190, 46, 156, 232, 196, 69, 148, 239] };
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_SEEK_BOF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_SEEK_CUR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_SEEK_EOF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_WRITE: u32 = 2u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct MTP_COMMAND_DATA_IN {
    pub OpCode: u16,
    pub NumParams: u32,
    pub Params: [u32; 5],
    pub NextPhase: u32,
    pub CommandWriteDataSize: u32,
    pub CommandWriteData: [u8; 1],
}
impl ::core::marker::Copy for MTP_COMMAND_DATA_IN {}
impl ::core::clone::Clone for MTP_COMMAND_DATA_IN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct MTP_COMMAND_DATA_OUT {
    pub ResponseCode: u16,
    pub NumParams: u32,
    pub Params: [u32; 5],
    pub CommandReadDataSize: u32,
    pub CommandReadData: [u8; 1],
}
impl ::core::marker::Copy for MTP_COMMAND_DATA_OUT {}
impl ::core::clone::Clone for MTP_COMMAND_DATA_OUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_COMMAND_MAX_PARAMS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_NEXTPHASE_NO_DATA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_NEXTPHASE_READ_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_NEXTPHASE_WRITE_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_RESPONSE_MAX_PARAMS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_RESPONSE_OK: u16 = 8193u16;
pub const MediaDevMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 632991105, data2: 13664, data3: 4563, data4: [132, 113, 0, 192, 79, 121, 219, 192] };
pub const MediaDevMgrClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1342442525, data2: 48575, data3: 18724, data4: [184, 115, 241, 77, 108, 91, 253, 102] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct OPAQUECOMMAND {
    pub guidCommand: ::windows_sys::core::GUID,
    pub dwDataLen: u32,
    pub pData: *mut u8,
    pub abMAC: [u8; 20],
}
impl ::core::marker::Copy for OPAQUECOMMAND {}
impl ::core::clone::Clone for OPAQUECOMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const RSA_KEY_LEN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_CERT_V1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_CERT_X509: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_MAC_LEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_PROTOCOL_V1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_PROTOCOL_WMDM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_SESSION_KEYLEN: u32 = 8u32;
pub const SCP_EVENTID_ACQSECURECLOCK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2250542281, data2: 19033, data3: 17378, data4: [145, 70, 72, 167, 243, 244, 20, 12] };
pub const SCP_EVENTID_DRMINFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 557699719, data2: 16850, data3: 17195, data4: [158, 63, 59, 79, 123, 53, 129, 221] };
pub const SCP_EVENTID_NEEDTOINDIV: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2275739591, data2: 46185, data3: 17286, data4: [185, 118, 213, 209, 206, 83, 138, 111] };
pub const SCP_PARAMID_DRMVERSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1104155997, data2: 31943, data3: 16919, data4: [173, 169, 0, 80, 116, 98, 77, 164] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct WMDMDATETIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
}
impl ::core::marker::Copy for WMDMDATETIME {}
impl ::core::clone::Clone for WMDMDATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub union WMDMDetermineMaxPropStringLen {
    pub sz001: [u16; 27],
    pub sz002: [u16; 31],
    pub sz003: [u16; 14],
    pub sz004: [u16; 16],
    pub sz005: [u16; 22],
    pub sz006: [u16; 14],
    pub sz007: [u16; 20],
    pub sz008: [u16; 20],
    pub sz009: [u16; 22],
    pub sz010: [u16; 11],
    pub sz011: [u16; 12],
    pub sz012: [u16; 17],
    pub sz013: [u16; 17],
    pub sz014: [u16; 16],
    pub sz015: [u16; 17],
    pub sz016: [u16; 11],
    pub sz017: [u16; 11],
    pub sz018: [u16; 15],
    pub sz019: [u16; 22],
    pub sz020: [u16; 20],
    pub sz021: [u16; 22],
    pub sz022: [u16; 21],
    pub sz023: [u16; 24],
    pub sz024: [u16; 20],
    pub sz025: [u16; 10],
    pub sz026: [u16; 14],
    pub sz027: [u16; 11],
    pub sz028: [u16; 11],
    pub sz029: [u16; 13],
    pub sz030: [u16; 17],
    pub sz031: [u16; 16],
    pub sz032: [u16; 17],
    pub sz033: [u16; 20],
    pub sz034: [u16; 19],
    pub sz035: [u16; 18],
    pub sz036: [u16; 18],
    pub sz037: [u16; 15],
    pub sz041: [u16; 14],
    pub sz043: [u16; 22],
    pub sz044: [u16; 16],
    pub sz045: [u16; 20],
    pub sz046: [u16; 14],
    pub sz047: [u16; 14],
    pub sz048: [u16; 12],
    pub sz049: [u16; 25],
    pub sz050: [u16; 26],
    pub sz051: [u16; 25],
    pub sz052: [u16; 16],
    pub sz053: [u16; 24],
    pub sz054: [u16; 15],
    pub sz055: [u16; 21],
    pub sz056: [u16; 16],
    pub sz057: [u16; 22],
    pub sz058: [u16; 14],
    pub sz059: [u16; 25],
    pub sz060: [u16; 18],
    pub sz061: [u16; 22],
    pub sz062: [u16; 26],
    pub sz063: [u16; 36],
    pub sz064: [u16; 23],
    pub sz065: [u16; 12],
    pub sz066: [u16; 24],
    pub sz067: [u16; 11],
    pub sz068: [u16; 12],
    pub sz069: [u16; 14],
    pub sz070: [u16; 20],
    pub sz071: [u16; 15],
    pub sz072: [u16; 14],
    pub sz073: [u16; 31],
    pub sz074: [u16; 24],
    pub sz075: [u16; 22],
    pub sz076: [u16; 24],
    pub sz077: [u16; 21],
    pub sz078: [u16; 27],
    pub sz079: [u16; 27],
    pub sz080: [u16; 20],
    pub sz081: [u16; 33],
    pub sz082: [u16; 21],
    pub sz083: [u16; 32],
    pub sz084: [u16; 26],
    pub sz085: [u16; 18],
    pub sz086: [u16; 30],
}
impl ::core::marker::Copy for WMDMDetermineMaxPropStringLen {}
impl ::core::clone::Clone for WMDMDetermineMaxPropStringLen {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WMDMDevice: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2155560159, data2: 13690, data3: 4563, data4: [132, 113, 0, 192, 79, 121, 219, 192] };
pub const WMDMDeviceEnum: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1125004719, data2: 14705, data3: 4563, data4: [132, 116, 0, 192, 79, 121, 219, 192] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct WMDMID {
    pub cbSize: u32,
    pub dwVendorID: u32,
    pub pID: [u8; 128],
    pub SerialNumberLength: u32,
}
impl ::core::marker::Copy for WMDMID {}
impl ::core::clone::Clone for WMDMID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMID_LENGTH: u32 = 128u32;
pub const WMDMLogger: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 285880834, data2: 23161, data3: 4563, data4: [141, 120, 68, 69, 83, 84, 0, 0] };
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDMMessage = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MSG_DEVICE_ARRIVAL: WMDMMessage = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MSG_DEVICE_REMOVAL: WMDMMessage = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MSG_MEDIA_ARRIVAL: WMDMMessage = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MSG_MEDIA_REMOVAL: WMDMMessage = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct WMDMMetadataView {
    pub pwszViewName: ::windows_sys::core::PWSTR,
    pub nDepth: u32,
    pub ppwszTags: *mut *mut u16,
}
impl ::core::marker::Copy for WMDMMetadataView {}
impl ::core::clone::Clone for WMDMMetadataView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct WMDMRIGHTS {
    pub cbSize: u32,
    pub dwContentType: u32,
    pub fuFlags: u32,
    pub fuRights: u32,
    pub dwAppSec: u32,
    pub dwPlaybackCount: u32,
    pub ExpirationDate: WMDMDATETIME,
}
impl ::core::marker::Copy for WMDMRIGHTS {}
impl ::core::clone::Clone for WMDMRIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WMDMStorage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2155560160, data2: 13690, data3: 4563, data4: [132, 113, 0, 192, 79, 121, 219, 192] };
pub const WMDMStorageEnum: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3946846779, data2: 15095, data3: 4563, data4: [132, 116, 0, 192, 79, 121, 219, 192] };
pub const WMDMStorageGlobal: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2155560161, data2: 13690, data3: 4563, data4: [132, 113, 0, 192, 79, 121, 219, 192] };
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_APP_REVOKED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_CONTENT_FILE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_CONTENT_FOLDER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_CONTENT_OPERATIONINTERFACE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANPAUSE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANPLAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANRECORD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANRESUME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANSEEK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANSTOP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANSTREAMPLAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANSTREAMRECORD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_HASSECURECLOCK: u32 = 256u32;
pub const WMDM_DEVICE_PROTOCOL_MSC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2765275756, data2: 43137, data3: 17595, data4: [189, 93, 31, 112, 60, 113, 247, 169] };
pub const WMDM_DEVICE_PROTOCOL_MTP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2543736037, data2: 2812, data3: 17924, data4: [141, 147, 220, 121, 138, 75, 207, 69] };
pub const WMDM_DEVICE_PROTOCOL_RAPI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 705818001, data2: 35983, data3: 16868, data4: [130, 209, 131, 134, 224, 3, 86, 28] };
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_DECODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_ENCODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_FILELISTRESYNC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_NONREENTRANT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_NONSDMI: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_PLAYBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_RECORD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_SDMI: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_STORAGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_VIEW_PREF_METADATAVIEW: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_VIRTUAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDM_ENUM_PROP_VALID_VALUES_FORM = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_ENUM_PROP_VALID_VALUES_ANY: WMDM_ENUM_PROP_VALID_VALUES_FORM = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_ENUM_PROP_VALID_VALUES_RANGE: WMDM_ENUM_PROP_VALID_VALUES_FORM = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_ENUM_PROP_VALID_VALUES_ENUM: WMDM_ENUM_PROP_VALID_VALUES_FORM = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_BUFFERTOOSMALL: i32 = -2147201016i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_BUSY: i32 = -2147201024i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_CALL_OUT_OF_SEQUENCE: i32 = -2147201017i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_CANTOPEN_PMSN_SERVICE_PIPE: i32 = -2147201005i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_INCORRECT_APPSEC: i32 = -2147201008i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_INCORRECT_RIGHTS: i32 = -2147201007i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_INTERFACEDEAD: i32 = -2147201023i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_INVALIDTYPE: i32 = -2147201022i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_LICENSE_EXPIRED: i32 = -2147201006i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_LICENSE_NOTEXIST: i32 = -2147201009i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_MAC_CHECK_FAILED: i32 = -2147201014i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_MOREDATA: i32 = -2147201015i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_NORIGHTS: i32 = -2147201018i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_NOTCERTIFIED: i32 = -2147201019i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_NOTSUPPORTED: i32 = -2147201020i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_PROCESSFAILED: i32 = -2147201021i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_REVOKED: i32 = -2147201010i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_SDMI_NOMORECOPIES: i32 = -2147201011i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_SDMI_TRIGGER: i32 = -2147201012i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_TOO_MANY_SESSIONS: i32 = -2147201005i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_USER_CANCELLED: i32 = -2147201013i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_AUDIO: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_AUDIOBOOK: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANDELETE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANMOVE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANPLAY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANREAD: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANRENAME: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_DATA: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_FILE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_FOLDER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_HIDDEN: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_LINK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_MUSIC: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_READONLY: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_SYSTEM: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_VIDEO: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_CREATE_OVERWRITE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDM_FIND_SCOPE = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FIND_SCOPE_GLOBAL: WMDM_FIND_SCOPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FIND_SCOPE_IMMEDIATE_CHILDREN: WMDM_FIND_SCOPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDM_FORMATCODE = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_NOTUSED: WMDM_FORMATCODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ALLIMAGES: WMDM_FORMATCODE = -1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINED: WMDM_FORMATCODE = 12288i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ASSOCIATION: WMDM_FORMATCODE = 12289i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_SCRIPT: WMDM_FORMATCODE = 12290i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_EXECUTABLE: WMDM_FORMATCODE = 12291i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_TEXT: WMDM_FORMATCODE = 12292i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_HTML: WMDM_FORMATCODE = 12293i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_DPOF: WMDM_FORMATCODE = 12294i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AIFF: WMDM_FORMATCODE = 12295i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WAVE: WMDM_FORMATCODE = 12296i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MP3: WMDM_FORMATCODE = 12297i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AVI: WMDM_FORMATCODE = 12298i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MPEG: WMDM_FORMATCODE = 12299i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ASF: WMDM_FORMATCODE = 12300i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_RESERVED_FIRST: WMDM_FORMATCODE = 12301i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_RESERVED_LAST: WMDM_FORMATCODE = 14335i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_UNDEFINED: WMDM_FORMATCODE = 14336i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_EXIF: WMDM_FORMATCODE = 14337i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_TIFFEP: WMDM_FORMATCODE = 14338i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_FLASHPIX: WMDM_FORMATCODE = 14339i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_BMP: WMDM_FORMATCODE = 14340i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_CIFF: WMDM_FORMATCODE = 14341i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_GIF: WMDM_FORMATCODE = 14343i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_JFIF: WMDM_FORMATCODE = 14344i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_PCD: WMDM_FORMATCODE = 14345i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_PICT: WMDM_FORMATCODE = 14346i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_PNG: WMDM_FORMATCODE = 14347i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_TIFF: WMDM_FORMATCODE = 14349i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_TIFFIT: WMDM_FORMATCODE = 14350i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_JP2: WMDM_FORMATCODE = 14351i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_JPX: WMDM_FORMATCODE = 14352i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_RESERVED_FIRST: WMDM_FORMATCODE = 14353i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_RESERVED_LAST: WMDM_FORMATCODE = 16383i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDFIRMWARE: WMDM_FORMATCODE = 47106i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WBMP: WMDM_FORMATCODE = 47107i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_JPEGXR: WMDM_FORMATCODE = 47108i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WINDOWSIMAGEFORMAT: WMDM_FORMATCODE = 47233i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDAUDIO: WMDM_FORMATCODE = 47360i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WMA: WMDM_FORMATCODE = 47361i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_OGG: WMDM_FORMATCODE = 47362i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AAC: WMDM_FORMATCODE = 47363i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AUDIBLE: WMDM_FORMATCODE = 47364i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_FLAC: WMDM_FORMATCODE = 47366i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_QCELP: WMDM_FORMATCODE = 47367i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AMR: WMDM_FORMATCODE = 47368i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDVIDEO: WMDM_FORMATCODE = 47488i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WMV: WMDM_FORMATCODE = 47489i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MP4: WMDM_FORMATCODE = 47490i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MP2: WMDM_FORMATCODE = 47491i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_3GP: WMDM_FORMATCODE = 47492i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_3G2: WMDM_FORMATCODE = 47493i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AVCHD: WMDM_FORMATCODE = 47494i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ATSCTS: WMDM_FORMATCODE = 47495i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_DVBTS: WMDM_FORMATCODE = 47496i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MKV: WMDM_FORMATCODE = 47497i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MKA: WMDM_FORMATCODE = 47498i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MK3D: WMDM_FORMATCODE = 47499i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDCOLLECTION: WMDM_FORMATCODE = 47616i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTMULTIMEDIAALBUM: WMDM_FORMATCODE = 47617i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTIMAGEALBUM: WMDM_FORMATCODE = 47618i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTAUDIOALBUM: WMDM_FORMATCODE = 47619i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTVIDEOALBUM: WMDM_FORMATCODE = 47620i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTAUDIOVIDEOPLAYLIST: WMDM_FORMATCODE = 47621i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTCONTACTGROUP: WMDM_FORMATCODE = 47622i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTMESSAGEFOLDER: WMDM_FORMATCODE = 47623i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTCHAPTEREDPRODUCTION: WMDM_FORMATCODE = 47624i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MEDIA_CAST: WMDM_FORMATCODE = 47627i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WPLPLAYLIST: WMDM_FORMATCODE = 47632i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_M3UPLAYLIST: WMDM_FORMATCODE = 47633i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MPLPLAYLIST: WMDM_FORMATCODE = 47634i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ASXPLAYLIST: WMDM_FORMATCODE = 47635i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_PLSPLAYLIST: WMDM_FORMATCODE = 47636i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDDOCUMENT: WMDM_FORMATCODE = 47744i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTDOCUMENT: WMDM_FORMATCODE = 47745i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_XMLDOCUMENT: WMDM_FORMATCODE = 47746i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MICROSOFTWORDDOCUMENT: WMDM_FORMATCODE = 47747i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MHTCOMPILEDHTMLDOCUMENT: WMDM_FORMATCODE = 47748i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MICROSOFTEXCELSPREADSHEET: WMDM_FORMATCODE = 47749i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MICROSOFTPOWERPOINTDOCUMENT: WMDM_FORMATCODE = 47750i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDMESSAGE: WMDM_FORMATCODE = 47872i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTMESSAGE: WMDM_FORMATCODE = 47873i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDCONTACT: WMDM_FORMATCODE = 48000i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTCONTACT: WMDM_FORMATCODE = 48001i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_VCARD2: WMDM_FORMATCODE = 48002i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_VCARD3: WMDM_FORMATCODE = 48003i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDCALENDARITEM: WMDM_FORMATCODE = 48640i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTCALENDARITEM: WMDM_FORMATCODE = 48641i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_VCALENDAR1: WMDM_FORMATCODE = 48642i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_VCALENDAR2: WMDM_FORMATCODE = 48643i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDWINDOWSEXECUTABLE: WMDM_FORMATCODE = 48768i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_M4A: WMDM_FORMATCODE = 1297101889i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_3GPA: WMDM_FORMATCODE = 860311617i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_3G2A: WMDM_FORMATCODE = 860303937i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_SECTION: WMDM_FORMATCODE = 48770i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_FORMAT_CAPABILITY {
    pub nPropConfig: u32,
    pub pConfigs: *mut WMDM_PROP_CONFIG,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_FORMAT_CAPABILITY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_FORMAT_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_GET_FORMAT_SUPPORT_AUDIO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_GET_FORMAT_SUPPORT_FILE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_GET_FORMAT_SUPPORT_VIDEO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_LOG_NOTIMESTAMP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_LOG_SEV_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_LOG_SEV_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_LOG_SEV_WARN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MAC_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_BLOCK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_PROGRESS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_QUERY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_RECURSIVE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_THREAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_TRANSFER_PROTECTED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_TRANSFER_UNPROTECTED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_CAP_BATTERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_CAP_EXTERNAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_IS_BATTERY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_IS_EXTERNAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_PERCENT_AVAILABLE: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_CONFIG {
    pub nPreference: u32,
    pub nPropDesc: u32,
    pub pPropDesc: *mut WMDM_PROP_DESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_DESC {
    pub pwszPropName: ::windows_sys::core::PWSTR,
    pub ValidValuesForm: WMDM_ENUM_PROP_VALID_VALUES_FORM,
    pub ValidValues: WMDM_PROP_DESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub union WMDM_PROP_DESC_0 {
    pub ValidValuesRange: WMDM_PROP_VALUES_RANGE,
    pub EnumeratedValidValues: WMDM_PROP_VALUES_ENUM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_DESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_VALUES_ENUM {
    pub cEnumValues: u32,
    pub pValues: *mut super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_VALUES_ENUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_VALUES_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_VALUES_RANGE {
    pub rangeMin: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeMax: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeStep: super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_VALUES_RANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_VALUES_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_COPY_TO_CD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_COPY_TO_NON_SDMI_DEVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_COPY_TO_SDMI_DEVICE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_EXPIRATIONDATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_FREESERIALIDS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_GROUPID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_NAMEDSERIALIDS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_PLAYBACKCOUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_PLAY_ON_PC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_DECIDE_DATA: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_DRMINFO_NOT_DRMPROTECTED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_DRMINFO_V1HEADER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_DRMINFO_V2HEADER: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_EXAMINE_DATA: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_EXAMINE_EXTENSION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_NO_MORE_CHANGES: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_PROTECTED_OUTPUT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_REVOKED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_RIGHTS_DATA: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_TRANSFER_OBJECTDATA: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_UNPROTECTED_OUTPUT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_BEGIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_CURRENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_END: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_REMOTECONTROL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_STREAMINGAUDIO: u32 = 2u32;
pub const WMDM_SERVICE_PROVIDER_VENDOR_MICROSOFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2112383085, data2: 30958, data3: 17386, data4: [164, 150, 198, 37, 172, 145, 204, 93] };
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDM_SESSION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_NONE: WMDM_SESSION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_TRANSFER_TO_DEVICE: WMDM_SESSION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_TRANSFER_FROM_DEVICE: WMDM_SESSION_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_DELETE: WMDM_SESSION_TYPE = 256i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_CUSTOM: WMDM_SESSION_TYPE = 4096i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SP_REVOKED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_BUSY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_PAUSED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_PLAYING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_RECORDING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_REMOTE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_STREAM: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICE_NOTPRESENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_READY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_APPENDING: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_DELETING: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_INSERTING: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_MOVING: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_READING: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_BROKEN: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_INITIALIZING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_NOTPRESENT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_NOTSUPPORTED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_UNFORMATTED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FILELIMITEXISTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FILESINFOLDERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FILESINROOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FOLDERLIMITEXISTS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FOLDERSINFOLDERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FOLDERSINROOT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_NOT_INITIALIZABLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECONTROL_INSERTAFTER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECONTROL_INSERTBEFORE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECONTROL_INSERTINTO: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_CANEDITMETADATA: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_FILESYSTEM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_FOLDERS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_HAS_FILES: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_HAS_FOLDERS: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_NONREMOVABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_REMOVABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_VIRTUAL: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_CONTAINS_DEFAULT: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDM_STORAGE_ENUM_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const ENUM_MODE_RAW: WMDM_STORAGE_ENUM_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const ENUM_MODE_USE_DEVICE_PREF: WMDM_STORAGE_ENUM_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const ENUM_MODE_METADATA_VIEWS: WMDM_STORAGE_ENUM_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_IS_DEFAULT: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_S_NOT_ALL_PROPERTIES_APPLIED: i32 = 282625i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_S_NOT_ALL_PROPERTIES_RETRIEVED: i32 = 282626i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDM_TAG_DATATYPE = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_DWORD: WMDM_TAG_DATATYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_STRING: WMDM_TAG_DATATYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_BINARY: WMDM_TAG_DATATYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_BOOL: WMDM_TAG_DATATYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_QWORD: WMDM_TAG_DATATYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_WORD: WMDM_TAG_DATATYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_GUID: WMDM_TAG_DATATYPE = 6i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_DATE: WMDM_TAG_DATATYPE = 7i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_WMDM_REVOKED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct WMFILECAPABILITIES {
    pub pwszMimeType: ::windows_sys::core::PWSTR,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for WMFILECAPABILITIES {}
impl ::core::clone::Clone for WMFILECAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct _BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}
impl ::core::marker::Copy for _BITMAPINFOHEADER {}
impl ::core::clone::Clone for _BITMAPINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct _VIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: _BITMAPINFOHEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _VIDEOINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct _WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}
impl ::core::marker::Copy for _WAVEFORMATEX {}
impl ::core::clone::Clone for _WAVEFORMATEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct __MACINFO {
    pub fUsed: super::super::Foundation::BOOL,
    pub abMacState: [u8; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for __MACINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for __MACINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszAudioWAVECodec: &str = "WMDM/AudioWAVECodec";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszVideoFourCCCodec: &str = "WMDM/VideoFourCCCodec";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumArt: &str = "WMDM/AlbumArt";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumArtist: &str = "WMDM/AlbumArtist";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverData: &str = "WMDM/AlbumCoverData";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverDuration: &str = "WMDM/AlbumCoverDuration";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverFormat: &str = "WMDM/AlbumCoverFormat";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverHeight: &str = "WMDM/AlbumCoverHeight";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverSize: &str = "WMDM/AlbumCoverSize";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverWidth: &str = "WMDM/AlbumCoverWidth";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumTitle: &str = "WMDM/AlbumTitle";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAudioBitDepth: &str = "WMDM/AudioBitDepth";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAuthor: &str = "WMDM/Author";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAuthorDate: &str = "WMDM/AuthorDate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBitRateType: &str = "WMDM/BitRateType";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBitrate: &str = "WMDM/Bitrate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBlockAlignment: &str = "WMDM/BlockAlignment";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBufferSize: &str = "WMDM/BufferSize";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBuyNow: &str = "WMDM/BuyNow";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMByteBookmark: &str = "WMDM/ByteBookmark";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMCategory: &str = "WMDM/Category";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMCodec: &str = "WMDM/Codec";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMCollectionID: &str = "WMDM/CollectionID";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMComposer: &str = "WMDM/Composer";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDRMId: &str = "WMDM/DRMId";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDataLength: &str = "WMDM/DataLength";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDataOffset: &str = "WMDM/DataOffset";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDataUnits: &str = "WMDM/DataUnits";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDescription: &str = "WMDM/Description";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDestinationURL: &str = "WMDM/DestinationURL";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceFirmwareVersion: &str = "WMDM/DeviceFirmwareVersion";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceFriendlyName: &str = "WMDM/DeviceFriendlyName";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceModelName: &str = "WMDM/DeviceModelName";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDevicePlayCount: &str = "WMDM/DevicePlayCount";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceProtocol: &str = "WMDM/DeviceProtocol";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceRevocationInfo: &str = "WMDM/DeviceRevocationInfo";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceServiceProviderVendor: &str = "WMDM/DeviceServiceProviderVendor";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceVendorExtension: &str = "WMDM/DeviceVendorExtension";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDuration: &str = "WMDM/Duration";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMEditor: &str = "WMDM/Editor";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMEncodingProfile: &str = "WMDM/EncodingProfile";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileAttributes: &str = "WMDM/FileAttributes";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileCreationDate: &str = "WMDM/FileCreationDate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileName: &str = "WMDM/FileName";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileSize: &str = "WMDM/FileSize";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFormatCode: &str = "WMDM/FormatCode";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFormatsSupported: &str = "WMDM/FormatsSupported";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFormatsSupportedAreOrdered: &str = "WMDM/FormatsSupportedAreOrdered";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFrameRate: &str = "WMDM/FrameRate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMGenre: &str = "WMDM/Genre";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMHeight: &str = "WMDM/Height";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMIsProtected: &str = "WMDM/IsProtected";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMIsRepeat: &str = "WMDM/IsRepeat";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMKeyFrameDistance: &str = "WMDM/KeyFrameDistance";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMLastModifiedDate: &str = "WMDM/LastModifiedDate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaClassSecondaryID: &str = "WMDM/MediaClassSecondaryID";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaCredits: &str = "WMDM/MediaCredits";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaGuid: &str = "WMDM/MediaGuid";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaOriginalBroadcastDateTime: &str = "WMDM/MediaOriginalBroadcastDateTime";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaOriginalChannel: &str = "WMDM/MediaOriginalChannel";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaStationName: &str = "WMDM/MediaStationName";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMetaGenre: &str = "WMDM/MetaGenre";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMNonConsumable: &str = "WMDM/NonConsumable";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMNumChannels: &str = "WMDM/NumChannels";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMObjectBookmark: &str = "WMDM/ObjectBookmark";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMOwner: &str = "WMDM/Owner";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMParentalRating: &str = "WMDM/ParentalRating";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMPersistentUniqueID: &str = "WMDM/PersistentUniqueID";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMPlayCount: &str = "WMDM/PlayCount";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMProviderCopyright: &str = "WMDM/ProviderCopyright";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMQualitySetting: &str = "WMDM/QualitySetting";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSampleRate: &str = "WMDM/SampleRate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMScanType: &str = "WMDM/ScanType";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSourceURL: &str = "WMDM/SourceURL";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSubTitle: &str = "WMDM/SubTitle";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSubTitleDescription: &str = "WMDM/SubTitleDescription";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSupportedDeviceProperties: &str = "WMDM/SupportedDeviceProperties";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSyncID: &str = "WMDM/SyncID";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSyncRelationshipID: &str = "WMDM/SyncRelationshipID";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSyncTime: &str = "WMDM/SyncTime";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTimeBookmark: &str = "WMDM/TimeBookmark";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTimeToLive: &str = "WMDM/TimeToLive";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTitle: &str = "WMDM/Title";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTotalBitrate: &str = "WMDM/TotalBitrate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTrack: &str = "WMDM/Track";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTrackMood: &str = "WMDM/TrackMood";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserEffectiveRating: &str = "WMDM/UserEffectiveRating";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserLastPlayTime: &str = "WMDM/UserLastPlayTime";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserRating: &str = "WMDM/UserRating";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserRatingOnDevice: &str = "WMDM/UserRatingOnDevice";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMVideoBitrate: &str = "WMDM/VideoBitrate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMWebmaster: &str = "WMDM/Webmaster";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMWidth: &str = "WMDM/Width";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMYear: &str = "WMDM/Year";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMediaClassPrimaryID: &str = "WMDM/MediaClassPrimaryID";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWPDPassthroughPropertyValues: &str = "WPD/PassthroughPropertyValues";
