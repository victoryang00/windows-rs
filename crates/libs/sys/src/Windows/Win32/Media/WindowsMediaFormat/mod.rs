#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
    pub fn WMCreateBackupRestorer(pcallback: *mut *mut ::windows_sys::core::IUnknown, ppbackup: *mut *mut *mut IWMLicenseBackup) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
    pub fn WMCreateEditor(ppeditor: *mut *mut *mut IWMMetadataEditor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
    pub fn WMCreateIndexer(ppindexer: *mut *mut *mut IWMIndexer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
    pub fn WMCreateProfileManager(ppprofilemanager: *mut *mut *mut IWMProfileManager) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
    pub fn WMCreateReader(punkcert: *mut *mut ::windows_sys::core::IUnknown, dwrights: u32, ppreader: *mut *mut *mut IWMReader) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
    pub fn WMCreateSyncReader(punkcert: *mut *mut ::windows_sys::core::IUnknown, dwrights: u32, ppsyncreader: *mut *mut *mut IWMSyncReader) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
    pub fn WMCreateWriter(punkcert: *mut *mut ::windows_sys::core::IUnknown, ppwriter: *mut *mut *mut IWMWriter) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
    pub fn WMCreateWriterFileSink(ppsink: *mut *mut *mut IWMWriterFileSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
    pub fn WMCreateWriterNetworkSink(ppsink: *mut *mut *mut IWMWriterNetworkSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
    pub fn WMCreateWriterPushSink(ppsink: *mut *mut *mut IWMWriterPushSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WMIsContentProtected(pwszfilename: ::windows_sys::core::PCWSTR, pfisprotected: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct AM_WMT_EVENT_DATA {
    pub hrStatus: ::windows_sys::core::HRESULT,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for AM_WMT_EVENT_DATA {}
impl ::core::clone::Clone for AM_WMT_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLSID_ClientNetManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3440550862, data2: 40002, data3: 4562, data4: [190, 237, 0, 96, 8, 47, 32, 84] };
pub const CLSID_WMBandwidthSharing_Exclusive: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2942329002, data2: 20887, data3: 4562, data4: [182, 175, 0, 192, 79, 217, 8, 233] };
pub const CLSID_WMBandwidthSharing_Partial: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2942329003, data2: 20887, data3: 4562, data4: [182, 175, 0, 192, 79, 217, 8, 233] };
pub const CLSID_WMMUTEX_Bitrate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3605146113, data2: 13786, data3: 4561, data4: [144, 52, 0, 160, 201, 3, 73, 190] };
pub const CLSID_WMMUTEX_Language: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3605146112, data2: 13786, data3: 4561, data4: [144, 52, 0, 160, 201, 3, 73, 190] };
pub const CLSID_WMMUTEX_Presentation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3605146114, data2: 13786, data3: 4561, data4: [144, 52, 0, 160, 201, 3, 73, 190] };
pub const CLSID_WMMUTEX_Unknown: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3605146115, data2: 13786, data3: 4561, data4: [144, 52, 0, 160, 201, 3, 73, 190] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_COPY_OPL {
    pub wMinimumCopyLevel: u16,
    pub oplIdIncludes: DRM_OPL_OUTPUT_IDS,
    pub oplIdExcludes: DRM_OPL_OUTPUT_IDS,
}
impl ::core::marker::Copy for DRM_COPY_OPL {}
impl ::core::clone::Clone for DRM_COPY_OPL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    pub wCompressedDigitalVideo: u16,
    pub wUncompressedDigitalVideo: u16,
    pub wAnalogVideo: u16,
    pub wCompressedDigitalAudio: u16,
    pub wUncompressedDigitalAudio: u16,
}
impl ::core::marker::Copy for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {}
impl ::core::clone::Clone for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_OPL_OUTPUT_IDS {
    pub cIds: u16,
    pub rgIds: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for DRM_OPL_OUTPUT_IDS {}
impl ::core::clone::Clone for DRM_OPL_OUTPUT_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const DRM_OPL_TYPES: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_OUTPUT_PROTECTION {
    pub guidId: ::windows_sys::core::GUID,
    pub bConfigData: u8,
}
impl ::core::marker::Copy for DRM_OUTPUT_PROTECTION {}
impl ::core::clone::Clone for DRM_OUTPUT_PROTECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_PLAY_OPL {
    pub minOPL: DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS,
    pub oplIdReserved: DRM_OPL_OUTPUT_IDS,
    pub vopi: DRM_VIDEO_OUTPUT_PROTECTION_IDS,
}
impl ::core::marker::Copy for DRM_PLAY_OPL {}
impl ::core::clone::Clone for DRM_PLAY_OPL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_VAL16 {
    pub val: [u8; 16],
}
impl ::core::marker::Copy for DRM_VAL16 {}
impl ::core::clone::Clone for DRM_VAL16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    pub cEntries: u16,
    pub rgVop: *mut DRM_OUTPUT_PROTECTION,
}
impl ::core::marker::Copy for DRM_VIDEO_OUTPUT_PROTECTION_IDS {}
impl ::core::clone::Clone for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct INSNetSourceCreator {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CreateNetSource: unsafe extern "system" fn(this: *mut *mut Self, pszstreamname: ::windows_sys::core::PCWSTR, pmonitor: *mut ::core::ffi::c_void, pdata: *const u8, pusercontext: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, qwcontext: u64) -> ::windows_sys::core::HRESULT,
    pub GetNetSourceProperties: unsafe extern "system" fn(this: *mut *mut Self, pszstreamname: ::windows_sys::core::PCWSTR, pppropertiesnode: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNetSourceSharedNamespace: unsafe extern "system" fn(this: *mut *mut Self, ppsharednamespace: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetNetSourceAdminInterface: unsafe extern "system" fn(this: *mut *mut Self, pszstreamname: ::windows_sys::core::PCWSTR, pval: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetNetSourceAdminInterface: usize,
    pub GetNumProtocolsSupported: unsafe extern "system" fn(this: *mut *mut Self, pcprotocols: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetProtocolName: unsafe extern "system" fn(this: *mut *mut Self, dwprotocolnum: u32, pwszprotocolname: ::windows_sys::core::PWSTR, pcchprotocolname: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INSSBuffer {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLength: unsafe extern "system" fn(this: *mut *mut Self, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut *mut Self, dwlength: u32) -> ::windows_sys::core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(this: *mut *mut Self, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(this: *mut *mut Self, ppdwbuffer: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub GetBufferAndLength: unsafe extern "system" fn(this: *mut *mut Self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INSSBuffer2 {
    pub base__: INSSBuffer,
    pub GetSampleProperties: unsafe extern "system" fn(this: *mut *mut Self, cbproperties: u32, pbproperties: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetSampleProperties: unsafe extern "system" fn(this: *mut *mut Self, cbproperties: u32, pbproperties: *const u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INSSBuffer3 {
    pub base__: INSSBuffer2,
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, guidbufferproperty: ::windows_sys::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows_sys::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, guidbufferproperty: ::windows_sys::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INSSBuffer4 {
    pub base__: INSSBuffer3,
    pub GetPropertyCount: unsafe extern "system" fn(this: *mut *mut Self, pcbufferproperties: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPropertyByIndex: unsafe extern "system" fn(this: *mut *mut Self, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows_sys::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMAddressAccess {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetAccessEntryCount: unsafe extern "system" fn(this: *mut *mut Self, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAccessEntry: unsafe extern "system" fn(this: *mut *mut Self, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows_sys::core::HRESULT,
    pub AddAccessEntry: unsafe extern "system" fn(this: *mut *mut Self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows_sys::core::HRESULT,
    pub RemoveAccessEntry: unsafe extern "system" fn(this: *mut *mut Self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMAddressAccess2 {
    pub base__: IWMAddressAccess,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAccessEntryEx: unsafe extern "system" fn(this: *mut *mut Self, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut super::super::Foundation::BSTR, pbstrmask: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAccessEntryEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddAccessEntryEx: unsafe extern "system" fn(this: *mut *mut Self, aetype: WM_AETYPE, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddAccessEntryEx: usize,
}
#[repr(C)]
pub struct IWMAuthorizer {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCertCount: unsafe extern "system" fn(this: *mut *mut Self, pccerts: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetCert: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub GetSharedData: unsafe extern "system" fn(this: *mut *mut Self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMBackupRestoreProps {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPropCount: unsafe extern "system" fn(this: *mut *mut Self, pcprops: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetPropByIndex: unsafe extern "system" fn(this: *mut *mut Self, windex: u16, pwszname: ::windows_sys::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetPropByName: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetProp: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_sys::core::HRESULT,
    pub RemoveProp: unsafe extern "system" fn(this: *mut *mut Self, pcwszname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub RemoveAllProps: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMBandwidthSharing {
    pub base__: IWMStreamList,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, pguidtype: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, guidtype: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetBandwidth: unsafe extern "system" fn(this: *mut *mut Self, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBandwidth: unsafe extern "system" fn(this: *mut *mut Self, dwbitrate: u32, msbufferwindow: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMClientConnections {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetClientCount: unsafe extern "system" fn(this: *mut *mut Self, pcclients: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetClientProperties: unsafe extern "system" fn(this: *mut *mut Self, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMClientConnections2 {
    pub base__: IWMClientConnections,
    pub GetClientInfo: unsafe extern "system" fn(this: *mut *mut Self, dwclientnum: u32, pwsznetworkaddress: ::windows_sys::core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: ::windows_sys::core::PWSTR, pcchport: *mut u32, pwszdnsname: ::windows_sys::core::PWSTR, pcchdnsname: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMCodecInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCodecInfoCount: unsafe extern "system" fn(this: *mut *mut Self, guidtype: *const ::windows_sys::core::GUID, pccodecs: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetCodecFormatCount: unsafe extern "system" fn(this: *mut *mut Self, guidtype: *const ::windows_sys::core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetCodecFormat: unsafe extern "system" fn(this: *mut *mut Self, guidtype: *const ::windows_sys::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMCodecInfo2 {
    pub base__: IWMCodecInfo,
    pub GetCodecName: unsafe extern "system" fn(this: *mut *mut Self, guidtype: *const ::windows_sys::core::GUID, dwcodecindex: u32, wszname: ::windows_sys::core::PWSTR, pcchname: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetCodecFormatDesc: unsafe extern "system" fn(this: *mut *mut Self, guidtype: *const ::windows_sys::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut *mut ::core::ffi::c_void, wszdesc: ::windows_sys::core::PWSTR, pcchdesc: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMCodecInfo3 {
    pub base__: IWMCodecInfo2,
    pub GetCodecFormatProp: unsafe extern "system" fn(this: *mut *mut Self, guidtype: *const ::windows_sys::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: ::windows_sys::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetCodecProp: unsafe extern "system" fn(this: *mut *mut Self, guidtype: *const ::windows_sys::core::GUID, dwcodecindex: u32, pszname: ::windows_sys::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCodecEnumerationSetting: unsafe extern "system" fn(this: *mut *mut Self, guidtype: *const ::windows_sys::core::GUID, dwcodecindex: u32, pszname: ::windows_sys::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows_sys::core::HRESULT,
    pub GetCodecEnumerationSetting: unsafe extern "system" fn(this: *mut *mut Self, guidtype: *const ::windows_sys::core::GUID, dwcodecindex: u32, pszname: ::windows_sys::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMCredentialCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub AcquireCredentials: unsafe extern "system" fn(this: *mut *mut Self, pwszrealm: ::windows_sys::core::PCWSTR, pwszsite: ::windows_sys::core::PCWSTR, pwszuser: ::windows_sys::core::PWSTR, cchuser: u32, pwszpassword: ::windows_sys::core::PWSTR, cchpassword: u32, hrstatus: ::windows_sys::core::HRESULT, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMDRMEditor {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDRMProperty: unsafe extern "system" fn(this: *mut *mut Self, pwstrname: ::windows_sys::core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMDRMMessageParser {
    pub base__: ::windows_sys::core::IUnknown,
    pub ParseRegistrationReqMsg: unsafe extern "system" fn(this: *mut *mut Self, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut *mut ::core::ffi::c_void, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ParseLicenseRequestMsg: unsafe extern "system" fn(this: *mut *mut Self, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut *mut ::core::ffi::c_void, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ParseLicenseRequestMsg: usize,
}
#[repr(C)]
pub struct IWMDRMReader {
    pub base__: ::windows_sys::core::IUnknown,
    pub AcquireLicense: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub CancelLicenseAcquisition: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Individualize: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub CancelIndividualization: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MonitorLicenseAcquisition: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CancelMonitorLicenseAcquisition: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetDRMProperty: unsafe extern "system" fn(this: *mut *mut Self, pwstrname: ::windows_sys::core::PCWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_sys::core::HRESULT,
    pub GetDRMProperty: unsafe extern "system" fn(this: *mut *mut Self, pwstrname: ::windows_sys::core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMDRMReader2 {
    pub base__: IWMDRMReader,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEvaluateOutputLevelLicenses: unsafe extern "system" fn(this: *mut *mut Self, fevaluate: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEvaluateOutputLevelLicenses: usize,
    pub GetPlayOutputLevels: unsafe extern "system" fn(this: *mut *mut Self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetCopyOutputLevels: unsafe extern "system" fn(this: *mut *mut Self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TryNextLicense: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMDRMReader3 {
    pub base__: IWMDRMReader2,
    pub GetInclusionList: unsafe extern "system" fn(this: *mut *mut Self, ppguids: *mut *mut ::windows_sys::core::GUID, pcguids: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMDRMTranscryptionManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateTranscryptor: unsafe extern "system" fn(this: *mut *mut Self, pptranscryptor: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMDRMTranscryptor {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub Seek: unsafe extern "system" fn(this: *mut *mut Self, hnstime: u64) -> ::windows_sys::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, pbdata: *const u8, pcbdata: *const u32) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMDRMTranscryptor2 {
    pub base__: IWMDRMTranscryptor,
    #[cfg(feature = "Win32_Foundation")]
    pub SeekEx: unsafe extern "system" fn(this: *mut *mut Self, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SeekEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ZeroAdjustTimestamps: unsafe extern "system" fn(this: *mut *mut Self, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ZeroAdjustTimestamps: usize,
    pub GetSeekStartTime: unsafe extern "system" fn(this: *mut *mut Self, pcnstime: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut *mut Self, pcnsduration: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMDRMWriter {
    pub base__: ::windows_sys::core::IUnknown,
    pub GenerateKeySeed: unsafe extern "system" fn(this: *mut *mut Self, pwszkeyseed: ::windows_sys::core::PWSTR, pcwchlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GenerateKeyID: unsafe extern "system" fn(this: *mut *mut Self, pwszkeyid: ::windows_sys::core::PWSTR, pcwchlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GenerateSigningKeyPair: unsafe extern "system" fn(this: *mut *mut Self, pwszprivkey: ::windows_sys::core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows_sys::core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDRMAttribute: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pszname: ::windows_sys::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMDRMWriter2 {
    pub base__: IWMDRMWriter,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWMDRMNetEncryption: unsafe extern "system" fn(this: *mut *mut Self, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWMDRMNetEncryption: usize,
}
#[repr(C)]
pub struct IWMDRMWriter3 {
    pub base__: IWMDRMWriter2,
    pub SetProtectStreamSamples: unsafe extern "system" fn(this: *mut *mut Self, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMDeviceRegistration {
    pub base__: ::windows_sys::core::IUnknown,
    pub RegisterDevice: unsafe extern "system" fn(this: *mut *mut Self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnregisterDevice: unsafe extern "system" fn(this: *mut *mut Self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows_sys::core::HRESULT,
    pub GetRegistrationStats: unsafe extern "system" fn(this: *mut *mut Self, dwregistertype: u32, pcregistereddevices: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetFirstRegisteredDevice: unsafe extern "system" fn(this: *mut *mut Self, dwregistertype: u32, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNextRegisteredDevice: unsafe extern "system" fn(this: *mut *mut Self, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRegisteredDeviceByID: unsafe extern "system" fn(this: *mut *mut Self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMGetSecureChannel {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPeerSecureChannelInterface: unsafe extern "system" fn(this: *mut *mut Self, pppeer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMHeaderInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetAttributeCount: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pcattributes: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetAttributeByIndex: unsafe extern "system" fn(this: *mut *mut Self, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows_sys::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetAttributeByName: unsafe extern "system" fn(this: *mut *mut Self, pwstreamnum: *mut u16, pszname: ::windows_sys::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pszname: ::windows_sys::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_sys::core::HRESULT,
    pub GetMarkerCount: unsafe extern "system" fn(this: *mut *mut Self, pcmarkers: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetMarker: unsafe extern "system" fn(this: *mut *mut Self, windex: u16, pwszmarkername: ::windows_sys::core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows_sys::core::HRESULT,
    pub AddMarker: unsafe extern "system" fn(this: *mut *mut Self, pwszmarkername: ::windows_sys::core::PCWSTR, cnsmarkertime: u64) -> ::windows_sys::core::HRESULT,
    pub RemoveMarker: unsafe extern "system" fn(this: *mut *mut Self, windex: u16) -> ::windows_sys::core::HRESULT,
    pub GetScriptCount: unsafe extern "system" fn(this: *mut *mut Self, pcscripts: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetScript: unsafe extern "system" fn(this: *mut *mut Self, windex: u16, pwsztype: ::windows_sys::core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows_sys::core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows_sys::core::HRESULT,
    pub AddScript: unsafe extern "system" fn(this: *mut *mut Self, pwsztype: ::windows_sys::core::PCWSTR, pwszcommand: ::windows_sys::core::PCWSTR, cnsscripttime: u64) -> ::windows_sys::core::HRESULT,
    pub RemoveScript: unsafe extern "system" fn(this: *mut *mut Self, windex: u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMHeaderInfo2 {
    pub base__: IWMHeaderInfo,
    pub GetCodecInfoCount: unsafe extern "system" fn(this: *mut *mut Self, pccodecinfos: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetCodecInfo: unsafe extern "system" fn(this: *mut *mut Self, windex: u32, pcchname: *mut u16, pwszname: ::windows_sys::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows_sys::core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMHeaderInfo3 {
    pub base__: IWMHeaderInfo2,
    pub GetAttributeCountEx: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pcattributes: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetAttributeIndices: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pwszname: ::windows_sys::core::PCWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetAttributeByIndexEx: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, windex: u16, pwszname: ::windows_sys::core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ModifyAttribute: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows_sys::core::HRESULT,
    pub AddAttribute: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pszname: ::windows_sys::core::PCWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows_sys::core::HRESULT,
    pub DeleteAttribute: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, windex: u16) -> ::windows_sys::core::HRESULT,
    pub AddCodecInfo: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR, pwszdescription: ::windows_sys::core::PCWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMIStreamProps {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMImageInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetImageCount: unsafe extern "system" fn(this: *mut *mut Self, pcimages: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetImage: unsafe extern "system" fn(this: *mut *mut Self, windex: u32, pcchmimetype: *mut u16, pwszmimetype: ::windows_sys::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows_sys::core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMIndexer {
    pub base__: ::windows_sys::core::IUnknown,
    pub StartIndexing: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PCWSTR, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMIndexer2 {
    pub base__: IWMIndexer,
    pub Configure: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMInputMediaProps {
    pub base__: IWMMediaProps,
    pub GetConnectionName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PWSTR, pcchname: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetGroupName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PWSTR, pcchname: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMLanguageList {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLanguageCount: unsafe extern "system" fn(this: *mut *mut Self, pwcount: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetLanguageDetails: unsafe extern "system" fn(this: *mut *mut Self, windex: u16, pwszlanguagestring: ::windows_sys::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_sys::core::HRESULT,
    pub AddLanguageByRFC1766String: unsafe extern "system" fn(this: *mut *mut Self, pwszlanguagestring: ::windows_sys::core::PCWSTR, pwindex: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMLicenseBackup {
    pub base__: ::windows_sys::core::IUnknown,
    pub BackupLicenses: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CancelLicenseBackup: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMLicenseRestore {
    pub base__: ::windows_sys::core::IUnknown,
    pub RestoreLicenses: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CancelLicenseRestore: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMLicenseRevocationAgent {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLRBChallenge: unsafe extern "system" fn(this: *mut *mut Self, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ProcessLRB: unsafe extern "system" fn(this: *mut *mut Self, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMMediaProps {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, pguidtype: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMediaType: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMediaType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMediaType: unsafe extern "system" fn(this: *mut *mut Self, ptype: *const WM_MEDIA_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMediaType: usize,
}
#[repr(C)]
pub struct IWMMetadataEditor {
    pub base__: ::windows_sys::core::IUnknown,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, pwszfilename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMMetadataEditor2 {
    pub base__: IWMMetadataEditor,
    pub OpenEx: unsafe extern "system" fn(this: *mut *mut Self, pwszfilename: ::windows_sys::core::PCWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMMutualExclusion {
    pub base__: IWMStreamList,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, pguidtype: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, guidtype: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMMutualExclusion2 {
    pub base__: IWMMutualExclusion,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PWSTR, pcchname: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetRecordCount: unsafe extern "system" fn(this: *mut *mut Self, pwrecordcount: *mut u16) -> ::windows_sys::core::HRESULT,
    pub AddRecord: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RemoveRecord: unsafe extern "system" fn(this: *mut *mut Self, wrecordnumber: u16) -> ::windows_sys::core::HRESULT,
    pub GetRecordName: unsafe extern "system" fn(this: *mut *mut Self, wrecordnumber: u16, pwszrecordname: ::windows_sys::core::PWSTR, pcchrecordname: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetRecordName: unsafe extern "system" fn(this: *mut *mut Self, wrecordnumber: u16, pwszrecordname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetStreamsForRecord: unsafe extern "system" fn(this: *mut *mut Self, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows_sys::core::HRESULT,
    pub AddStreamForRecord: unsafe extern "system" fn(this: *mut *mut Self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows_sys::core::HRESULT,
    pub RemoveStreamForRecord: unsafe extern "system" fn(this: *mut *mut Self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMOutputMediaProps {
    pub base__: IWMMediaProps,
    pub GetStreamGroupName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PWSTR, pcchname: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetConnectionName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PWSTR, pcchname: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMPacketSize {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetMaxPacketSize: unsafe extern "system" fn(this: *mut *mut Self, pdwmaxpacketsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxPacketSize: unsafe extern "system" fn(this: *mut *mut Self, dwmaxpacketsize: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMPacketSize2 {
    pub base__: IWMPacketSize,
    pub GetMinPacketSize: unsafe extern "system" fn(this: *mut *mut Self, pdwminpacketsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMinPacketSize: unsafe extern "system" fn(this: *mut *mut Self, dwminpacketsize: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMPlayerHook {
    pub base__: ::windows_sys::core::IUnknown,
    pub PreDecode: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMPlayerTimestampHook {
    pub base__: ::windows_sys::core::IUnknown,
    pub MapTimestamp: unsafe extern "system" fn(this: *mut *mut Self, rtin: i64, prtout: *mut i64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMProfile {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetVersion: unsafe extern "system" fn(this: *mut *mut Self, pdwversion: *mut WMT_VERSION) -> ::windows_sys::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PWSTR, pcchname: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, pwszdescription: ::windows_sys::core::PWSTR, pcchdescription: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, pwszdescription: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetStreamCount: unsafe extern "system" fn(this: *mut *mut Self, pcstreams: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, dwstreamindex: u32, ppconfig: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStreamByNumber: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, ppconfig: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(this: *mut *mut Self, pconfig: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveStreamByNumber: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16) -> ::windows_sys::core::HRESULT,
    pub AddStream: unsafe extern "system" fn(this: *mut *mut Self, pconfig: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReconfigStream: unsafe extern "system" fn(this: *mut *mut Self, pconfig: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateNewStream: unsafe extern "system" fn(this: *mut *mut Self, guidstreamtype: *const ::windows_sys::core::GUID, ppconfig: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMutualExclusionCount: unsafe extern "system" fn(this: *mut *mut Self, pcme: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMutualExclusion: unsafe extern "system" fn(this: *mut *mut Self, dwmeindex: u32, ppme: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveMutualExclusion: unsafe extern "system" fn(this: *mut *mut Self, pme: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddMutualExclusion: unsafe extern "system" fn(this: *mut *mut Self, pme: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateNewMutualExclusion: unsafe extern "system" fn(this: *mut *mut Self, ppme: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMProfile2 {
    pub base__: IWMProfile,
    pub GetProfileID: unsafe extern "system" fn(this: *mut *mut Self, pguidid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMProfile3 {
    pub base__: IWMProfile2,
    pub GetStorageFormat: unsafe extern "system" fn(this: *mut *mut Self, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> ::windows_sys::core::HRESULT,
    pub SetStorageFormat: unsafe extern "system" fn(this: *mut *mut Self, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows_sys::core::HRESULT,
    pub GetBandwidthSharingCount: unsafe extern "system" fn(this: *mut *mut Self, pcbs: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetBandwidthSharing: unsafe extern "system" fn(this: *mut *mut Self, dwbsindex: u32, ppbs: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveBandwidthSharing: unsafe extern "system" fn(this: *mut *mut Self, pbs: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddBandwidthSharing: unsafe extern "system" fn(this: *mut *mut Self, pbs: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateNewBandwidthSharing: unsafe extern "system" fn(this: *mut *mut Self, ppbs: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStreamPrioritization: unsafe extern "system" fn(this: *mut *mut Self, ppsp: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetStreamPrioritization: unsafe extern "system" fn(this: *mut *mut Self, psp: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveStreamPrioritization: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CreateNewStreamPrioritization: unsafe extern "system" fn(this: *mut *mut Self, ppsp: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetExpectedPacketCount: unsafe extern "system" fn(this: *mut *mut Self, msduration: u64, pcpackets: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMProfileManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateEmptyProfile: unsafe extern "system" fn(this: *mut *mut Self, dwversion: WMT_VERSION, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LoadProfileByID: unsafe extern "system" fn(this: *mut *mut Self, guidprofile: *const ::windows_sys::core::GUID, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LoadProfileByData: unsafe extern "system" fn(this: *mut *mut Self, pwszprofile: ::windows_sys::core::PCWSTR, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SaveProfile: unsafe extern "system" fn(this: *mut *mut Self, piwmprofile: *mut ::core::ffi::c_void, pwszprofile: ::windows_sys::core::PCWSTR, pdwlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSystemProfileCount: unsafe extern "system" fn(this: *mut *mut Self, pcprofiles: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LoadSystemProfile: unsafe extern "system" fn(this: *mut *mut Self, dwprofileindex: u32, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMProfileManager2 {
    pub base__: IWMProfileManager,
    pub GetSystemProfileVersion: unsafe extern "system" fn(this: *mut *mut Self, pdwversion: *mut WMT_VERSION) -> ::windows_sys::core::HRESULT,
    pub SetSystemProfileVersion: unsafe extern "system" fn(this: *mut *mut Self, dwversion: WMT_VERSION) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMProfileManagerLanguage {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetUserLanguageID: unsafe extern "system" fn(this: *mut *mut Self, wlangid: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetUserLanguageID: unsafe extern "system" fn(this: *mut *mut Self, wlangid: u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMPropertyVault {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPropertyCount: unsafe extern "system" fn(this: *mut *mut Self, pdwcount: *const u32) -> ::windows_sys::core::HRESULT,
    pub GetPropertyByName: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PCWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows_sys::core::HRESULT,
    pub GetPropertyByIndex: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32, pszname: ::windows_sys::core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CopyPropertiesFrom: unsafe extern "system" fn(this: *mut *mut Self, piwmpropertyvault: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMProximityDetection {
    pub base__: ::windows_sys::core::IUnknown,
    pub StartDetection: unsafe extern "system" fn(this: *mut *mut Self, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReader {
    pub base__: ::windows_sys::core::IUnknown,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PCWSTR, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut *mut Self, pcoutputs: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetOutputProps: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOutputProps: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, poutput: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOutputFormatCount: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnumber: u32, pcformats: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetOutputFormat: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnumber: u32, dwformatnumber: u32, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderAccelerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCodecInterface: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, riid: *const ::windows_sys::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Notify: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Notify: usize,
}
#[repr(C)]
pub struct IWMReaderAdvanced {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserProvidedClock: unsafe extern "system" fn(this: *mut *mut Self, fuserclock: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserProvidedClock: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUserProvidedClock: unsafe extern "system" fn(this: *mut *mut Self, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUserProvidedClock: usize,
    pub DeliverTime: unsafe extern "system" fn(this: *mut *mut Self, cnstime: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetManualStreamSelection: unsafe extern "system" fn(this: *mut *mut Self, fselection: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetManualStreamSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetManualStreamSelection: unsafe extern "system" fn(this: *mut *mut Self, pfselection: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetManualStreamSelection: usize,
    pub SetStreamsSelected: unsafe extern "system" fn(this: *mut *mut Self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_sys::core::HRESULT,
    pub GetStreamSelected: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReceiveSelectionCallbacks: unsafe extern "system" fn(this: *mut *mut Self, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReceiveSelectionCallbacks: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReceiveSelectionCallbacks: unsafe extern "system" fn(this: *mut *mut Self, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReceiveSelectionCallbacks: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReceiveStreamSamples: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReceiveStreamSamples: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReceiveStreamSamples: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReceiveStreamSamples: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllocateForOutput: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllocateForOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllocateForOutput: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllocateForOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllocateForStream: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllocateForStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllocateForStream: unsafe extern "system" fn(this: *mut *mut Self, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllocateForStream: usize,
    pub GetStatistics: unsafe extern "system" fn(this: *mut *mut Self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientInfo: unsafe extern "system" fn(this: *mut *mut Self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientInfo: usize,
    pub GetMaxOutputSampleSize: unsafe extern "system" fn(this: *mut *mut Self, dwoutput: u32, pcbmax: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMaxStreamSampleSize: unsafe extern "system" fn(this: *mut *mut Self, wstream: u16, pcbmax: *mut u32) -> ::windows_sys::core::HRESULT,
    pub NotifyLateDelivery: unsafe extern "system" fn(this: *mut *mut Self, cnslateness: u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderAdvanced2 {
    pub base__: IWMReaderAdvanced,
    pub SetPlayMode: unsafe extern "system" fn(this: *mut *mut Self, mode: WMT_PLAY_MODE) -> ::windows_sys::core::HRESULT,
    pub GetPlayMode: unsafe extern "system" fn(this: *mut *mut Self, pmode: *mut WMT_PLAY_MODE) -> ::windows_sys::core::HRESULT,
    pub GetBufferProgress: unsafe extern "system" fn(this: *mut *mut Self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetDownloadProgress: unsafe extern "system" fn(this: *mut *mut Self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetSaveAsProgress: unsafe extern "system" fn(this: *mut *mut Self, pdwpercent: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SaveFileAs: unsafe extern "system" fn(this: *mut *mut Self, pwszfilename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetProtocolName: unsafe extern "system" fn(this: *mut *mut Self, pwszprotocol: ::windows_sys::core::PWSTR, pcchprotocol: *mut u32) -> ::windows_sys::core::HRESULT,
    pub StartAtMarker: unsafe extern "system" fn(this: *mut *mut Self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOutputSetting: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, pszname: ::windows_sys::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetOutputSetting: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, pszname: ::windows_sys::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_sys::core::HRESULT,
    pub Preroll: unsafe extern "system" fn(this: *mut *mut Self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLogClientID: unsafe extern "system" fn(this: *mut *mut Self, flogclientid: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLogClientID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLogClientID: unsafe extern "system" fn(this: *mut *mut Self, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLogClientID: usize,
    pub StopBuffering: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenStream: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenStream: usize,
}
#[repr(C)]
pub struct IWMReaderAdvanced3 {
    pub base__: IWMReaderAdvanced2,
    pub StopNetStreaming: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StartAtPosition: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderAdvanced4 {
    pub base__: IWMReaderAdvanced3,
    pub GetLanguageCount: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows_sys::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetMaxSpeedFactor: unsafe extern "system" fn(this: *mut *mut Self, pdblfactor: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUsingFastCache: unsafe extern "system" fn(this: *mut *mut Self, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUsingFastCache: usize,
    pub AddLogParam: unsafe extern "system" fn(this: *mut *mut Self, wsznamespace: ::windows_sys::core::PCWSTR, wszname: ::windows_sys::core::PCWSTR, wszvalue: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SendLogParams: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CanSaveFileAs: unsafe extern "system" fn(this: *mut *mut Self, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanSaveFileAs: usize,
    pub CancelSaveFileAs: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetURL: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PWSTR, pcchurl: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderAdvanced5 {
    pub base__: IWMReaderAdvanced4,
    pub SetPlayerHook: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, phook: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderAdvanced6 {
    pub base__: IWMReaderAdvanced5,
    pub SetProtectStreamSamples: unsafe extern "system" fn(this: *mut *mut Self, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderAllocatorEx {
    pub base__: ::windows_sys::core::IUnknown,
    pub AllocateForStreamEx: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AllocateForOutputEx: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderCallback {
    pub base__: IWMStatusCallback,
    pub OnSample: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderCallbackAdvanced {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnStreamSample: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnTime: unsafe extern "system" fn(this: *mut *mut Self, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnStreamSelection: unsafe extern "system" fn(this: *mut *mut Self, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnOutputPropsChanged: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnOutputPropsChanged: usize,
    pub AllocateForStream: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AllocateForOutput: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderNetworkConfig {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetBufferingTime: unsafe extern "system" fn(this: *mut *mut Self, pcnsbufferingtime: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetBufferingTime: unsafe extern "system" fn(this: *mut *mut Self, cnsbufferingtime: u64) -> ::windows_sys::core::HRESULT,
    pub GetUDPPortRanges: unsafe extern "system" fn(this: *mut *mut Self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetUDPPortRanges: unsafe extern "system" fn(this: *mut *mut Self, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows_sys::core::HRESULT,
    pub GetProxySettings: unsafe extern "system" fn(this: *mut *mut Self, pwszprotocol: ::windows_sys::core::PCWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows_sys::core::HRESULT,
    pub SetProxySettings: unsafe extern "system" fn(this: *mut *mut Self, pwszprotocol: ::windows_sys::core::PCWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows_sys::core::HRESULT,
    pub GetProxyHostName: unsafe extern "system" fn(this: *mut *mut Self, pwszprotocol: ::windows_sys::core::PCWSTR, pwszhostname: ::windows_sys::core::PWSTR, pcchhostname: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetProxyHostName: unsafe extern "system" fn(this: *mut *mut Self, pwszprotocol: ::windows_sys::core::PCWSTR, pwszhostname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetProxyPort: unsafe extern "system" fn(this: *mut *mut Self, pwszprotocol: ::windows_sys::core::PCWSTR, pdwport: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetProxyPort: unsafe extern "system" fn(this: *mut *mut Self, pwszprotocol: ::windows_sys::core::PCWSTR, dwport: u32) -> ::windows_sys::core::HRESULT,
    pub GetProxyExceptionList: unsafe extern "system" fn(this: *mut *mut Self, pwszprotocol: ::windows_sys::core::PCWSTR, pwszexceptionlist: ::windows_sys::core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetProxyExceptionList: unsafe extern "system" fn(this: *mut *mut Self, pwszprotocol: ::windows_sys::core::PCWSTR, pwszexceptionlist: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProxyBypassForLocal: unsafe extern "system" fn(this: *mut *mut Self, pwszprotocol: ::windows_sys::core::PCWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProxyBypassForLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProxyBypassForLocal: unsafe extern "system" fn(this: *mut *mut Self, pwszprotocol: ::windows_sys::core::PCWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProxyBypassForLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForceRerunAutoProxyDetection: unsafe extern "system" fn(this: *mut *mut Self, pfforcererundetection: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForceRerunAutoProxyDetection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetForceRerunAutoProxyDetection: unsafe extern "system" fn(this: *mut *mut Self, fforcererundetection: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetForceRerunAutoProxyDetection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableMulticast: unsafe extern "system" fn(this: *mut *mut Self, pfenablemulticast: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableMulticast: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableMulticast: unsafe extern "system" fn(this: *mut *mut Self, fenablemulticast: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableMulticast: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableHTTP: unsafe extern "system" fn(this: *mut *mut Self, pfenablehttp: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableHTTP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableHTTP: unsafe extern "system" fn(this: *mut *mut Self, fenablehttp: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableHTTP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableUDP: unsafe extern "system" fn(this: *mut *mut Self, pfenableudp: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableUDP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableUDP: unsafe extern "system" fn(this: *mut *mut Self, fenableudp: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableUDP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableTCP: unsafe extern "system" fn(this: *mut *mut Self, pfenabletcp: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableTCP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableTCP: unsafe extern "system" fn(this: *mut *mut Self, fenabletcp: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableTCP: usize,
    pub ResetProtocolRollover: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetConnectionBandwidth: unsafe extern "system" fn(this: *mut *mut Self, pdwconnectionbandwidth: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetConnectionBandwidth: unsafe extern "system" fn(this: *mut *mut Self, dwconnectionbandwidth: u32) -> ::windows_sys::core::HRESULT,
    pub GetNumProtocolsSupported: unsafe extern "system" fn(this: *mut *mut Self, pcprotocols: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSupportedProtocolName: unsafe extern "system" fn(this: *mut *mut Self, dwprotocolnum: u32, pwszprotocolname: ::windows_sys::core::PWSTR, pcchprotocolname: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AddLoggingUrl: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetLoggingUrl: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32, pwszurl: ::windows_sys::core::PWSTR, pcchurl: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLoggingUrlCount: unsafe extern "system" fn(this: *mut *mut Self, pdwurlcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ResetLoggingUrlList: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderNetworkConfig2 {
    pub base__: IWMReaderNetworkConfig,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableContentCaching: unsafe extern "system" fn(this: *mut *mut Self, pfenablecontentcaching: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableContentCaching: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableContentCaching: unsafe extern "system" fn(this: *mut *mut Self, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableContentCaching: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableFastCache: unsafe extern "system" fn(this: *mut *mut Self, pfenablefastcache: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableFastCache: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableFastCache: unsafe extern "system" fn(this: *mut *mut Self, fenablefastcache: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableFastCache: usize,
    pub GetAcceleratedStreamingDuration: unsafe extern "system" fn(this: *mut *mut Self, pcnsaccelduration: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetAcceleratedStreamingDuration: unsafe extern "system" fn(this: *mut *mut Self, cnsaccelduration: u64) -> ::windows_sys::core::HRESULT,
    pub GetAutoReconnectLimit: unsafe extern "system" fn(this: *mut *mut Self, pdwautoreconnectlimit: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetAutoReconnectLimit: unsafe extern "system" fn(this: *mut *mut Self, dwautoreconnectlimit: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableResends: unsafe extern "system" fn(this: *mut *mut Self, pfenableresends: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableResends: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableResends: unsafe extern "system" fn(this: *mut *mut Self, fenableresends: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableResends: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableThinning: unsafe extern "system" fn(this: *mut *mut Self, pfenablethinning: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableThinning: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableThinning: unsafe extern "system" fn(this: *mut *mut Self, fenablethinning: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableThinning: usize,
    pub GetMaxNetPacketSize: unsafe extern "system" fn(this: *mut *mut Self, pdwmaxnetpacketsize: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderPlaylistBurn {
    pub base__: ::windows_sys::core::IUnknown,
    pub InitPlaylistBurn: unsafe extern "system" fn(this: *mut *mut Self, cfiles: u32, ppwszfilenames: *const ::windows_sys::core::PWSTR, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetInitResults: unsafe extern "system" fn(this: *mut *mut Self, cfiles: u32, phrstati: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EndPlaylistBurn: unsafe extern "system" fn(this: *mut *mut Self, hrburnresult: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderStreamClock {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetTime: unsafe extern "system" fn(this: *mut *mut Self, pcnsnow: *const u64) -> ::windows_sys::core::HRESULT,
    pub SetTimer: unsafe extern "system" fn(this: *mut *mut Self, cnswhen: u64, pvparam: *const ::core::ffi::c_void, pdwtimerid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub KillTimer: unsafe extern "system" fn(this: *mut *mut Self, dwtimerid: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderTimecode {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetTimecodeRangeCount: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pwrangecount: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetTimecodeRangeBounds: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMReaderTypeNegotiation {
    pub base__: ::windows_sys::core::IUnknown,
    pub TryOutputProps: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, poutput: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMRegisterCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub Advise: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMRegisteredDevice {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDeviceSerialNumber: unsafe extern "system" fn(this: *mut *mut Self, pserialnumber: *mut DRM_VAL16) -> ::windows_sys::core::HRESULT,
    pub GetDeviceCertificate: unsafe extern "system" fn(this: *mut *mut Self, ppcertificate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeviceType: unsafe extern "system" fn(this: *mut *mut Self, pdwtype: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAttributeCount: unsafe extern "system" fn(this: *mut *mut Self, pcattributes: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributeByIndex: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32, pbstrname: *mut super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributeByIndex: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributeByName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributeByName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAttributeByName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAttributeByName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Approve: unsafe extern "system" fn(this: *mut *mut Self, fapprove: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Approve: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsValid: unsafe extern "system" fn(this: *mut *mut Self, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsValid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsApproved: unsafe extern "system" fn(this: *mut *mut Self, pfapproved: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsApproved: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsWmdrmCompliant: unsafe extern "system" fn(this: *mut *mut Self, pfcompliant: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsWmdrmCompliant: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOpened: unsafe extern "system" fn(this: *mut *mut Self, pfopened: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOpened: usize,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMSBufferAllocator {
    pub base__: ::windows_sys::core::IUnknown,
    pub AllocateBuffer: unsafe extern "system" fn(this: *mut *mut Self, dwmaxbuffersize: u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AllocatePageSizeBuffer: unsafe extern "system" fn(this: *mut *mut Self, dwmaxbuffersize: u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMSInternalAdminNetSource {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, psharednamespace: *mut ::core::ffi::c_void, pnamespacenode: *mut ::core::ffi::c_void, pnetsourcecreator: *mut ::core::ffi::c_void, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub GetNetSourceCreator: unsafe extern "system" fn(this: *mut *mut Self, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredentials: unsafe extern "system" fn(this: *mut *mut Self, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredentials: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCredentials: unsafe extern "system" fn(this: *mut *mut Self, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCredentials: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteCredentials: unsafe extern "system" fn(this: *mut *mut Self, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteCredentials: usize,
    pub GetCredentialFlags: unsafe extern "system" fn(this: *mut *mut Self, lpdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCredentialFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindProxyForURL: unsafe extern "system" fn(this: *mut *mut Self, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindProxyForURL: usize,
    pub RegisterProxyFailure: unsafe extern "system" fn(this: *mut *mut Self, hrparam: ::windows_sys::core::HRESULT, dwproxycontext: u32) -> ::windows_sys::core::HRESULT,
    pub ShutdownProxyContext: unsafe extern "system" fn(this: *mut *mut Self, dwproxycontext: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUsingIE: unsafe extern "system" fn(this: *mut *mut Self, dwproxycontext: u32, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUsingIE: usize,
}
#[repr(C)]
pub struct IWMSInternalAdminNetSource2 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredentialsEx: unsafe extern "system" fn(this: *mut *mut Self, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredentialsEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCredentialsEx: unsafe extern "system" fn(this: *mut *mut Self, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCredentialsEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteCredentialsEx: unsafe extern "system" fn(this: *mut *mut Self, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteCredentialsEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FindProxyForURLEx: unsafe extern "system" fn(this: *mut *mut Self, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindProxyForURLEx: usize,
}
#[repr(C)]
pub struct IWMSInternalAdminNetSource3 {
    pub base__: IWMSInternalAdminNetSource2,
    pub GetNetSourceCreator2: unsafe extern "system" fn(this: *mut *mut Self, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindProxyForURLEx2: unsafe extern "system" fn(this: *mut *mut Self, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindProxyForURLEx2: usize,
    pub RegisterProxyFailure2: unsafe extern "system" fn(this: *mut *mut Self, hrparam: ::windows_sys::core::HRESULT, qwproxycontext: u64) -> ::windows_sys::core::HRESULT,
    pub ShutdownProxyContext2: unsafe extern "system" fn(this: *mut *mut Self, qwproxycontext: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUsingIE2: unsafe extern "system" fn(this: *mut *mut Self, qwproxycontext: u64, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUsingIE2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredentialsEx2: unsafe extern "system" fn(this: *mut *mut Self, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredentialsEx2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCredentialsEx2: unsafe extern "system" fn(this: *mut *mut Self, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCredentialsEx2: usize,
}
#[repr(C)]
pub struct IWMSecureChannel {
    pub base__: IWMAuthorizer,
    pub WMSC_AddCertificate: unsafe extern "system" fn(this: *mut *mut Self, pcert: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WMSC_AddSignature: unsafe extern "system" fn(this: *mut *mut Self, pbcertsig: *const u8, cbcertsig: u32) -> ::windows_sys::core::HRESULT,
    pub WMSC_Connect: unsafe extern "system" fn(this: *mut *mut Self, potherside: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WMSC_IsConnected: unsafe extern "system" fn(this: *mut *mut Self, pfisconnected: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WMSC_IsConnected: usize,
    pub WMSC_Disconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WMSC_GetValidCertificate: unsafe extern "system" fn(this: *mut *mut Self, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows_sys::core::HRESULT,
    pub WMSC_Encrypt: unsafe extern "system" fn(this: *mut *mut Self, pbdata: *const u8, cbdata: u32) -> ::windows_sys::core::HRESULT,
    pub WMSC_Decrypt: unsafe extern "system" fn(this: *mut *mut Self, pbdata: *const u8, cbdata: u32) -> ::windows_sys::core::HRESULT,
    pub WMSC_Lock: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WMSC_Unlock: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WMSC_SetSharedData: unsafe extern "system" fn(this: *mut *mut Self, dwcertindex: u32, pbshareddata: *const u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMStatusCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnStatus: unsafe extern "system" fn(this: *mut *mut Self, status: WMT_STATUS, hr: ::windows_sys::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMStreamConfig {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetStreamType: unsafe extern "system" fn(this: *mut *mut Self, pguidstreamtype: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetStreamNumber: unsafe extern "system" fn(this: *mut *mut Self, pwstreamnum: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetStreamNumber: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16) -> ::windows_sys::core::HRESULT,
    pub GetStreamName: unsafe extern "system" fn(this: *mut *mut Self, pwszstreamname: ::windows_sys::core::PWSTR, pcchstreamname: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetStreamName: unsafe extern "system" fn(this: *mut *mut Self, pwszstreamname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetConnectionName: unsafe extern "system" fn(this: *mut *mut Self, pwszinputname: ::windows_sys::core::PWSTR, pcchinputname: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetConnectionName: unsafe extern "system" fn(this: *mut *mut Self, pwszinputname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetBitrate: unsafe extern "system" fn(this: *mut *mut Self, pdwbitrate: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBitrate: unsafe extern "system" fn(this: *mut *mut Self, pdwbitrate: u32) -> ::windows_sys::core::HRESULT,
    pub GetBufferWindow: unsafe extern "system" fn(this: *mut *mut Self, pmsbufferwindow: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBufferWindow: unsafe extern "system" fn(this: *mut *mut Self, msbufferwindow: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMStreamConfig2 {
    pub base__: IWMStreamConfig,
    pub GetTransportType: unsafe extern "system" fn(this: *mut *mut Self, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows_sys::core::HRESULT,
    pub SetTransportType: unsafe extern "system" fn(this: *mut *mut Self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows_sys::core::HRESULT,
    pub AddDataUnitExtension: unsafe extern "system" fn(this: *mut *mut Self, guidextensionsystemid: ::windows_sys::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows_sys::core::HRESULT,
    pub GetDataUnitExtensionCount: unsafe extern "system" fn(this: *mut *mut Self, pcdataunitextensions: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetDataUnitExtension: unsafe extern "system" fn(this: *mut *mut Self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows_sys::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows_sys::core::HRESULT,
    pub RemoveAllDataUnitExtensions: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMStreamConfig3 {
    pub base__: IWMStreamConfig2,
    pub GetLanguage: unsafe extern "system" fn(this: *mut *mut Self, pwszlanguagestring: ::windows_sys::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut *mut Self, pwszlanguagestring: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMStreamList {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetStreams: unsafe extern "system" fn(this: *mut *mut Self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows_sys::core::HRESULT,
    pub AddStream: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16) -> ::windows_sys::core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMStreamPrioritization {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPriorityRecords: unsafe extern "system" fn(this: *mut *mut Self, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPriorityRecords: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPriorityRecords: unsafe extern "system" fn(this: *mut *mut Self, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPriorityRecords: usize,
}
#[repr(C)]
pub struct IWMSyncReader {
    pub base__: ::windows_sys::core::IUnknown,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, pwszfilename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetRange: unsafe extern "system" fn(this: *mut *mut Self, cnsstarttime: u64, cnsduration: i64) -> ::windows_sys::core::HRESULT,
    pub SetRangeByFrame: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows_sys::core::HRESULT,
    pub GetNextSample: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, ppsample: *mut *mut ::core::ffi::c_void, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetStreamsSelected: unsafe extern "system" fn(this: *mut *mut Self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_sys::core::HRESULT,
    pub GetStreamSelected: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReadStreamSamples: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReadStreamSamples: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReadStreamSamples: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReadStreamSamples: usize,
    pub GetOutputSetting: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, pszname: ::windows_sys::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetOutputSetting: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, pszname: ::windows_sys::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_sys::core::HRESULT,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut *mut Self, pcoutputs: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetOutputProps: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOutputProps: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, poutput: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOutputFormatCount: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, pcformats: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetOutputFormat: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOutputNumberForStream: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStreamNumberForOutput: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetMaxOutputSampleSize: unsafe extern "system" fn(this: *mut *mut Self, dwoutput: u32, pcbmax: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMaxStreamSampleSize: unsafe extern "system" fn(this: *mut *mut Self, wstream: u16, pcbmax: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenStream: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenStream: usize,
}
#[repr(C)]
pub struct IWMSyncReader2 {
    pub base__: IWMSyncReader,
    pub SetRangeByTimecode: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows_sys::core::HRESULT,
    pub SetRangeByFrameEx: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64, pcnsstarttime: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetAllocateForOutput: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, pallocator: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAllocateForOutput: unsafe extern "system" fn(this: *mut *mut Self, dwoutputnum: u32, ppallocator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAllocateForStream: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pallocator: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAllocateForStream: unsafe extern "system" fn(this: *mut *mut Self, dwsreamnum: u16, ppallocator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMVideoMediaProps {
    pub base__: IWMMediaProps,
    pub GetMaxKeyFrameSpacing: unsafe extern "system" fn(this: *mut *mut Self, plltime: *mut i64) -> ::windows_sys::core::HRESULT,
    pub SetMaxKeyFrameSpacing: unsafe extern "system" fn(this: *mut *mut Self, lltime: i64) -> ::windows_sys::core::HRESULT,
    pub GetQuality: unsafe extern "system" fn(this: *mut *mut Self, pdwquality: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetQuality: unsafe extern "system" fn(this: *mut *mut Self, dwquality: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMWatermarkInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetWatermarkEntryCount: unsafe extern "system" fn(this: *mut *mut Self, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetWatermarkEntry: unsafe extern "system" fn(this: *mut *mut Self, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMWriter {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetProfileByID: unsafe extern "system" fn(this: *mut *mut Self, guidprofile: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetProfile: unsafe extern "system" fn(this: *mut *mut Self, pprofile: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOutputFilename: unsafe extern "system" fn(this: *mut *mut Self, pwszfilename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetInputCount: unsafe extern "system" fn(this: *mut *mut Self, pcinputs: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetInputProps: unsafe extern "system" fn(this: *mut *mut Self, dwinputnum: u32, ppinput: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetInputProps: unsafe extern "system" fn(this: *mut *mut Self, dwinputnum: u32, pinput: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetInputFormatCount: unsafe extern "system" fn(this: *mut *mut Self, dwinputnumber: u32, pcformats: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetInputFormat: unsafe extern "system" fn(this: *mut *mut Self, dwinputnumber: u32, dwformatnumber: u32, pprops: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BeginWriting: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EndWriting: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub AllocateSample: unsafe extern "system" fn(this: *mut *mut Self, dwsamplesize: u32, ppsample: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WriteSample: unsafe extern "system" fn(this: *mut *mut Self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMWriterAdvanced {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSinkCount: unsafe extern "system" fn(this: *mut *mut Self, pcsinks: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSink: unsafe extern "system" fn(this: *mut *mut Self, dwsinknum: u32, ppsink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddSink: unsafe extern "system" fn(this: *mut *mut Self, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveSink: unsafe extern "system" fn(this: *mut *mut Self, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WriteStreamSample: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLiveSource: unsafe extern "system" fn(this: *mut *mut Self, fislivesource: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLiveSource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRealTime: unsafe extern "system" fn(this: *mut *mut Self, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRealTime: usize,
    pub GetWriterTime: unsafe extern "system" fn(this: *mut *mut Self, pcnscurrenttime: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows_sys::core::HRESULT,
    pub SetSyncTolerance: unsafe extern "system" fn(this: *mut *mut Self, mswindow: u32) -> ::windows_sys::core::HRESULT,
    pub GetSyncTolerance: unsafe extern "system" fn(this: *mut *mut Self, pmswindow: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMWriterAdvanced2 {
    pub base__: IWMWriterAdvanced,
    pub GetInputSetting: unsafe extern "system" fn(this: *mut *mut Self, dwinputnum: u32, pszname: ::windows_sys::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetInputSetting: unsafe extern "system" fn(this: *mut *mut Self, dwinputnum: u32, pszname: ::windows_sys::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMWriterAdvanced3 {
    pub base__: IWMWriterAdvanced2,
    pub GetStatisticsEx: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows_sys::core::HRESULT,
    pub SetNonBlocking: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMWriterFileSink {
    pub base__: IWMWriterSink,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, pwszfilename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMWriterFileSink2 {
    pub base__: IWMWriterFileSink,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, cnsstarttime: u64) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self, cnsstoptime: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsStopped: unsafe extern "system" fn(this: *mut *mut Self, pfstopped: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsStopped: usize,
    pub GetFileDuration: unsafe extern "system" fn(this: *mut *mut Self, pcnsduration: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetFileSize: unsafe extern "system" fn(this: *mut *mut Self, pcbfile: *mut u64) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsClosed: unsafe extern "system" fn(this: *mut *mut Self, pfclosed: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsClosed: usize,
}
#[repr(C)]
pub struct IWMWriterFileSink3 {
    pub base__: IWMWriterFileSink2,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAutoIndexing: unsafe extern "system" fn(this: *mut *mut Self, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAutoIndexing: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAutoIndexing: unsafe extern "system" fn(this: *mut *mut Self, pfautoindexing: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAutoIndexing: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetControlStream: unsafe extern "system" fn(this: *mut *mut Self, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetControlStream: usize,
    pub GetMode: unsafe extern "system" fn(this: *mut *mut Self, pdwfilesinkmode: *mut u32) -> ::windows_sys::core::HRESULT,
    pub OnDataUnitEx: unsafe extern "system" fn(this: *mut *mut Self, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUnbufferedIO: unsafe extern "system" fn(this: *mut *mut Self, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUnbufferedIO: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUnbufferedIO: unsafe extern "system" fn(this: *mut *mut Self, pfunbufferedio: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUnbufferedIO: usize,
    pub CompleteOperations: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMWriterNetworkSink {
    pub base__: IWMWriterSink,
    pub SetMaximumClients: unsafe extern "system" fn(this: *mut *mut Self, dwmaxclients: u32) -> ::windows_sys::core::HRESULT,
    pub GetMaximumClients: unsafe extern "system" fn(this: *mut *mut Self, pdwmaxclients: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetNetworkProtocol: unsafe extern "system" fn(this: *mut *mut Self, protocol: WMT_NET_PROTOCOL) -> ::windows_sys::core::HRESULT,
    pub GetNetworkProtocol: unsafe extern "system" fn(this: *mut *mut Self, pprotocol: *mut WMT_NET_PROTOCOL) -> ::windows_sys::core::HRESULT,
    pub GetHostURL: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PWSTR, pcchurl: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, pdwportnum: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMWriterPostView {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetPostViewCallback: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void, pvcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReceivePostViewSamples: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReceivePostViewSamples: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReceivePostViewSamples: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, pfreceivepostviewsamples: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReceivePostViewSamples: usize,
    pub GetPostViewProps: unsafe extern "system" fn(this: *mut *mut Self, wstreamnumber: u16, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPostViewProps: unsafe extern "system" fn(this: *mut *mut Self, wstreamnumber: u16, poutput: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPostViewFormatCount: unsafe extern "system" fn(this: *mut *mut Self, wstreamnumber: u16, pcformats: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPostViewFormat: unsafe extern "system" fn(this: *mut *mut Self, wstreamnumber: u16, dwformatnumber: u32, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllocateForPostView: unsafe extern "system" fn(this: *mut *mut Self, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllocateForPostView: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllocateForPostView: unsafe extern "system" fn(this: *mut *mut Self, wstreamnumber: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllocateForPostView: usize,
}
#[repr(C)]
pub struct IWMWriterPostViewCallback {
    pub base__: IWMStatusCallback,
    pub OnPostViewSample: unsafe extern "system" fn(this: *mut *mut Self, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AllocateForPostView: unsafe extern "system" fn(this: *mut *mut Self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMWriterPreprocess {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetMaxPreprocessingPasses: unsafe extern "system" fn(this: *mut *mut Self, dwinputnum: u32, dwflags: u32, pdwmaxnumpasses: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetNumPreprocessingPasses: unsafe extern "system" fn(this: *mut *mut Self, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows_sys::core::HRESULT,
    pub BeginPreprocessingPass: unsafe extern "system" fn(this: *mut *mut Self, dwinputnum: u32, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub PreprocessSample: unsafe extern "system" fn(this: *mut *mut Self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EndPreprocessingPass: unsafe extern "system" fn(this: *mut *mut Self, dwinputnum: u32, dwflags: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMWriterPushSink {
    pub base__: IWMWriterSink,
    #[cfg(feature = "Win32_Foundation")]
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, pwszurl: ::windows_sys::core::PCWSTR, pwsztemplateurl: ::windows_sys::core::PCWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Connect: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWMWriterSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnHeader: unsafe extern "system" fn(this: *mut *mut Self, pheader: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRealTime: unsafe extern "system" fn(this: *mut *mut Self, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRealTime: usize,
    pub AllocateDataUnit: unsafe extern "system" fn(this: *mut *mut Self, cbdataunit: u32, ppdataunit: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnDataUnit: unsafe extern "system" fn(this: *mut *mut Self, pdataunit: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnEndWriting: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type NETSOURCE_URLCREDPOLICY_SETTINGS = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const NETSOURCE_URLCREDPOLICY_SETTING_SILENTLOGONOK: NETSOURCE_URLCREDPOLICY_SETTINGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const NETSOURCE_URLCREDPOLICY_SETTING_MUSTPROMPTUSER: NETSOURCE_URLCREDPOLICY_SETTINGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const NETSOURCE_URLCREDPOLICY_SETTING_ANONYMOUSONLY: NETSOURCE_URLCREDPOLICY_SETTINGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WEBSTREAM_SAMPLE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WEBSTREAM_SAMPLE_TYPE_FILE: WEBSTREAM_SAMPLE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WEBSTREAM_SAMPLE_TYPE_RENDER: WEBSTREAM_SAMPLE_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMDRM_IMPORT_INIT_STRUCT {
    pub dwVersion: u32,
    pub cbEncryptedSessionKeyMessage: u32,
    pub pbEncryptedSessionKeyMessage: *mut u8,
    pub cbEncryptedKeyMessage: u32,
    pub pbEncryptedKeyMessage: *mut u8,
}
impl ::core::marker::Copy for WMDRM_IMPORT_INIT_STRUCT {}
impl ::core::clone::Clone for WMDRM_IMPORT_INIT_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMDRM_IMPORT_INIT_STRUCT_DEFINED: u32 = 1u32;
pub const WMFORMAT_MPEG2Video: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272803, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const WMFORMAT_Script: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1552224498, data2: 57022, data3: 19623, data4: [187, 165, 240, 122, 16, 79, 141, 255] };
pub const WMFORMAT_VideoInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 89694080, data2: 50006, data3: 4558, data4: [191, 1, 0, 170, 0, 85, 89, 90] };
pub const WMFORMAT_WaveFormatEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 89694081, data2: 50006, data3: 4558, data4: [191, 1, 0, 170, 0, 85, 89, 90] };
pub const WMFORMAT_WebStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3659426579, data2: 33625, data3: 16464, data4: [179, 152, 56, 142, 150, 91, 240, 12] };
pub const WMMEDIASUBTYPE_ACELPnet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 304, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_Base: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 0, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_DRM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 9, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_I420: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 808596553, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_IYUV: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1448433993, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_M4S2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 844313677, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_MP3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 85, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_MP43: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 859066445, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_MP4S: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1395937357, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_MPEG2_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765272614, data2: 56134, data3: 4559, data4: [180, 209, 0, 128, 95, 108, 187, 234] };
pub const WMMEDIASUBTYPE_MSS1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 827544397, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_MSS2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 844321613, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_P422: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 842150992, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_PCM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_RGB1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804472, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_RGB24: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804477, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_RGB32: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804478, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_RGB4: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804473, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_RGB555: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804476, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_RGB565: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804475, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_RGB8: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3828804474, data2: 21071, data3: 4558, data4: [159, 83, 0, 32, 175, 11, 167, 112] };
pub const WMMEDIASUBTYPE_UYVY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1498831189, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_VIDEOIMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 491406834, data2: 58870, data3: 19268, data4: [131, 136, 240, 174, 92, 14, 12, 55] };
pub const WMMEDIASUBTYPE_WMAudioV2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 353, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMAudioV7: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 353, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMAudioV8: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 353, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMAudioV9: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 354, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMAudio_Lossless: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 355, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMSP1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 10, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMSP2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 11, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMV1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 827739479, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMV2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 844516695, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMV3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 861293911, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMVA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1096174935, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WMVP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1347833175, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WVC1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 826496599, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WVP2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 844125783, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_WebStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2002933716, data2: 50727, data3: 16843, data4: [143, 129, 122, 199, 255, 28, 64, 204] };
pub const WMMEDIASUBTYPE_YUY2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 844715353, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_YV12: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 842094169, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_YVU9: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 961893977, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIASUBTYPE_YVYU: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1431918169, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIATYPE_Audio: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935963489, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIATYPE_FileTransfer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3655628153, data2: 37646, data3: 17447, data4: [173, 252, 173, 128, 242, 144, 228, 112] };
pub const WMMEDIATYPE_Image: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 883232728, data2: 35493, data3: 17286, data4: [129, 254, 160, 239, 224, 72, 142, 49] };
pub const WMMEDIATYPE_Script: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935895908, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
pub const WMMEDIATYPE_Text: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2612666023, data2: 23218, data3: 18473, data4: [186, 87, 9, 64, 32, 155, 207, 62] };
pub const WMMEDIATYPE_Video: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935960438, data2: 0, data3: 16, data4: [128, 0, 0, 170, 0, 56, 155, 113] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMMPEG2VIDEOINFO {
    pub hdr: WMVIDEOINFOHEADER2,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub dwProfile: u32,
    pub dwLevel: u32,
    pub dwFlags: u32,
    pub dwSequenceHeader: [u32; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WMMPEG2VIDEOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WMMPEG2VIDEOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMSCRIPTFORMAT {
    pub scriptType: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for WMSCRIPTFORMAT {}
impl ::core::clone::Clone for WMSCRIPTFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WMSCRIPTTYPE_TwoStrings: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2196998768, data2: 49823, data3: 4561, data4: [151, 173, 0, 160, 201, 94, 168, 80] };
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_ATTR_DATATYPE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_DWORD: WMT_ATTR_DATATYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_STRING: WMT_ATTR_DATATYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_BINARY: WMT_ATTR_DATATYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_BOOL: WMT_ATTR_DATATYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_QWORD: WMT_ATTR_DATATYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_WORD: WMT_ATTR_DATATYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TYPE_GUID: WMT_ATTR_DATATYPE = 6i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_ATTR_IMAGETYPE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IMAGETYPE_BITMAP: WMT_ATTR_IMAGETYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IMAGETYPE_JPEG: WMT_ATTR_IMAGETYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IMAGETYPE_GIF: WMT_ATTR_IMAGETYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_BUFFER_SEGMENT {
    pub pBuffer: *mut *mut *mut *mut INSSBuffer,
    pub cbOffset: u32,
    pub cbLength: u32,
}
impl ::core::marker::Copy for WMT_BUFFER_SEGMENT {}
impl ::core::clone::Clone for WMT_BUFFER_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_CODEC_INFO_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CODECINFO_AUDIO: WMT_CODEC_INFO_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CODECINFO_VIDEO: WMT_CODEC_INFO_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CODECINFO_UNKNOWN: WMT_CODEC_INFO_TYPE = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_COLORSPACEINFO_EXTENSION_DATA {
    pub ucColorPrimaries: u8,
    pub ucColorTransferChar: u8,
    pub ucColorMatrixCoef: u8,
}
impl ::core::marker::Copy for WMT_COLORSPACEINFO_EXTENSION_DATA {}
impl ::core::clone::Clone for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_CREDENTIAL_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CREDENTIAL_SAVE: WMT_CREDENTIAL_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CREDENTIAL_DONT_CACHE: WMT_CREDENTIAL_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CREDENTIAL_CLEAR_TEXT: WMT_CREDENTIAL_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CREDENTIAL_PROXY: WMT_CREDENTIAL_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CREDENTIAL_ENCRYPT: WMT_CREDENTIAL_FLAGS = 16i32;
pub const WMT_DMOCATEGORY_AUDIO_WATERMARK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1696734298, data2: 64117, data3: 19257, data4: [181, 12, 6, 195, 54, 182, 163, 239] };
pub const WMT_DMOCATEGORY_VIDEO_WATERMARK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 410831138, data2: 36604, data3: 17412, data4: [157, 175, 99, 244, 131, 13, 241, 188] };
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_DRMLA_TRUST = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_DRMLA_UNTRUSTED: WMT_DRMLA_TRUST = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_DRMLA_TRUSTED: WMT_DRMLA_TRUST = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_DRMLA_TAMPERED: WMT_DRMLA_TRUST = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_FILESINK_DATA_UNIT {
    pub packetHeaderBuffer: WMT_BUFFER_SEGMENT,
    pub cPayloads: u32,
    pub pPayloadHeaderBuffers: *mut WMT_BUFFER_SEGMENT,
    pub cPayloadDataFragments: u32,
    pub pPayloadDataFragments: *mut WMT_PAYLOAD_FRAGMENT,
}
impl ::core::marker::Copy for WMT_FILESINK_DATA_UNIT {}
impl ::core::clone::Clone for WMT_FILESINK_DATA_UNIT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_FILESINK_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_FM_SINGLE_BUFFERS: WMT_FILESINK_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_FM_FILESINK_DATA_UNITS: WMT_FILESINK_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_FM_FILESINK_UNBUFFERED: WMT_FILESINK_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_IMAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_NONE: WMT_IMAGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_BITMAP: WMT_IMAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_JPEG: WMT_IMAGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_GIF: WMT_IMAGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_INDEXER_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_PRESENTATION_TIME: WMT_INDEXER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_FRAME_NUMBERS: WMT_INDEXER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_TIMECODE: WMT_INDEXER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_INDEX_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_NEAREST_DATA_UNIT: WMT_INDEX_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_NEAREST_OBJECT: WMT_INDEX_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_IT_NEAREST_CLEAN_POINT: WMT_INDEX_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_MUSICSPEECH_CLASS_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_MS_CLASS_MUSIC: WMT_MUSICSPEECH_CLASS_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_MS_CLASS_SPEECH: WMT_MUSICSPEECH_CLASS_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_MS_CLASS_MIXED: WMT_MUSICSPEECH_CLASS_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_NET_PROTOCOL = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROTOCOL_HTTP: WMT_NET_PROTOCOL = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_OFFSET_FORMAT = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OFFSET_FORMAT_100NS: WMT_OFFSET_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OFFSET_FORMAT_FRAME_NUMBERS: WMT_OFFSET_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OFFSET_FORMAT_PLAYLIST_OFFSET: WMT_OFFSET_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OFFSET_FORMAT_TIMECODE: WMT_OFFSET_FORMAT = 3i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OFFSET_FORMAT_100NS_APPROXIMATE: WMT_OFFSET_FORMAT = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_PAYLOAD_FRAGMENT {
    pub dwPayloadIndex: u32,
    pub segmentData: WMT_BUFFER_SEGMENT,
}
impl ::core::marker::Copy for WMT_PAYLOAD_FRAGMENT {}
impl ::core::clone::Clone for WMT_PAYLOAD_FRAGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_PLAY_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PLAY_MODE_AUTOSELECT: WMT_PLAY_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PLAY_MODE_LOCAL: WMT_PLAY_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PLAY_MODE_DOWNLOAD: WMT_PLAY_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PLAY_MODE_STREAMING: WMT_PLAY_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_PROXY_SETTINGS = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXY_SETTING_NONE: WMT_PROXY_SETTINGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXY_SETTING_MANUAL: WMT_PROXY_SETTINGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXY_SETTING_AUTO: WMT_PROXY_SETTINGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXY_SETTING_BROWSER: WMT_PROXY_SETTINGS = 3i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXY_SETTING_MAX: WMT_PROXY_SETTINGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_RIGHTS = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_PLAYBACK: WMT_RIGHTS = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_COPY_TO_NON_SDMI_DEVICE: WMT_RIGHTS = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_COPY_TO_CD: WMT_RIGHTS = 8i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_COPY_TO_SDMI_DEVICE: WMT_RIGHTS = 16i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_ONE_TIME: WMT_RIGHTS = 32i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_SAVE_STREAM_PROTECTED: WMT_RIGHTS = 64i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_COPY: WMT_RIGHTS = 128i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_COLLABORATIVE_PLAY: WMT_RIGHTS = 256i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_SDMI_TRIGGER: WMT_RIGHTS = 65536i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RIGHT_SDMI_NOMORECOPIES: WMT_RIGHTS = 131072i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_STATUS = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_ERROR: WMT_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OPENED: WMT_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_BUFFERING_START: WMT_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_BUFFERING_STOP: WMT_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_EOF: WMT_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_END_OF_FILE: WMT_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_END_OF_SEGMENT: WMT_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_END_OF_STREAMING: WMT_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_LOCATING: WMT_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CONNECTING: WMT_STATUS = 8i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_NO_RIGHTS: WMT_STATUS = 9i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_MISSING_CODEC: WMT_STATUS = 10i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_STARTED: WMT_STATUS = 11i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_STOPPED: WMT_STATUS = 12i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLOSED: WMT_STATUS = 13i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_STRIDING: WMT_STATUS = 14i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TIMER: WMT_STATUS = 15i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_INDEX_PROGRESS: WMT_STATUS = 16i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_SAVEAS_START: WMT_STATUS = 17i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_SAVEAS_STOP: WMT_STATUS = 18i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_NEW_SOURCEFLAGS: WMT_STATUS = 19i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_NEW_METADATA: WMT_STATUS = 20i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_BACKUPRESTORE_BEGIN: WMT_STATUS = 21i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_SOURCE_SWITCH: WMT_STATUS = 22i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_ACQUIRE_LICENSE: WMT_STATUS = 23i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_INDIVIDUALIZE: WMT_STATUS = 24i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_NEEDS_INDIVIDUALIZATION: WMT_STATUS = 25i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_NO_RIGHTS_EX: WMT_STATUS = 26i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_BACKUPRESTORE_END: WMT_STATUS = 27i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_BACKUPRESTORE_CONNECTING: WMT_STATUS = 28i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_BACKUPRESTORE_DISCONNECTING: WMT_STATUS = 29i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_ERROR_WITHURL: WMT_STATUS = 30i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RESTRICTED_LICENSE: WMT_STATUS = 31i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLIENT_CONNECT: WMT_STATUS = 32i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLIENT_DISCONNECT: WMT_STATUS = 33i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_NATIVE_OUTPUT_PROPS_CHANGED: WMT_STATUS = 34i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RECONNECT_START: WMT_STATUS = 35i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_RECONNECT_END: WMT_STATUS = 36i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLIENT_CONNECT_EX: WMT_STATUS = 37i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLIENT_DISCONNECT_EX: WMT_STATUS = 38i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_SET_FEC_SPAN: WMT_STATUS = 39i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PREROLL_READY: WMT_STATUS = 40i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PREROLL_COMPLETE: WMT_STATUS = 41i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLIENT_PROPERTIES: WMT_STATUS = 42i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_LICENSEURL_SIGNATURE_STATE: WMT_STATUS = 43i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_INIT_PLAYLIST_BURN: WMT_STATUS = 44i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TRANSCRYPTOR_INIT: WMT_STATUS = 45i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TRANSCRYPTOR_SEEKED: WMT_STATUS = 46i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TRANSCRYPTOR_READ: WMT_STATUS = 47i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TRANSCRYPTOR_CLOSED: WMT_STATUS = 48i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXIMITY_RESULT: WMT_STATUS = 49i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_PROXIMITY_COMPLETED: WMT_STATUS = 50i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CONTENT_ENABLER: WMT_STATUS = 51i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_STORAGE_FORMAT = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_Storage_Format_MP3: WMT_STORAGE_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_Storage_Format_V1: WMT_STORAGE_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_STREAM_SELECTION = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_OFF: WMT_STREAM_SELECTION = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_CLEANPOINT_ONLY: WMT_STREAM_SELECTION = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_ON: WMT_STREAM_SELECTION = 2i32;
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_TIMECODE_EXTENSION_DATA {
    pub wRange: u16,
    pub dwTimecode: u32,
    pub dwUserbits: u32,
    pub dwAmFlags: u32,
}
impl ::core::marker::Copy for WMT_TIMECODE_EXTENSION_DATA {}
impl ::core::clone::Clone for WMT_TIMECODE_EXTENSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_TIMECODE_FRAMERATE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TIMECODE_FRAMERATE_30: WMT_TIMECODE_FRAMERATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TIMECODE_FRAMERATE_30DROP: WMT_TIMECODE_FRAMERATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TIMECODE_FRAMERATE_25: WMT_TIMECODE_FRAMERATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_TIMECODE_FRAMERATE_24: WMT_TIMECODE_FRAMERATE = 3i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_TRANSPORT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_Transport_Type_Unreliable: WMT_TRANSPORT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_Transport_Type_Reliable: WMT_TRANSPORT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_VERSION = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VER_4_0: WMT_VERSION = 262144i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VER_7_0: WMT_VERSION = 458752i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VER_8_0: WMT_VERSION = 524288i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VER_9_0: WMT_VERSION = 589824i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_INTEGER_DENOMINATOR: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER: u32 = 491406834u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER_2: u32 = 491406835u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_VIDEOIMAGE_SAMPLE {
    pub dwMagic: u32,
    pub cbStruct: u32,
    pub dwControlFlags: u32,
    pub dwInputFlagsCur: u32,
    pub lCurMotionXtoX: i32,
    pub lCurMotionYtoX: i32,
    pub lCurMotionXoffset: i32,
    pub lCurMotionXtoY: i32,
    pub lCurMotionYtoY: i32,
    pub lCurMotionYoffset: i32,
    pub lCurBlendCoef1: i32,
    pub lCurBlendCoef2: i32,
    pub dwInputFlagsPrev: u32,
    pub lPrevMotionXtoX: i32,
    pub lPrevMotionYtoX: i32,
    pub lPrevMotionXoffset: i32,
    pub lPrevMotionXtoY: i32,
    pub lPrevMotionYtoY: i32,
    pub lPrevMotionYoffset: i32,
    pub lPrevBlendCoef1: i32,
    pub lPrevBlendCoef2: i32,
}
impl ::core::marker::Copy for WMT_VIDEOIMAGE_SAMPLE {}
impl ::core::clone::Clone for WMT_VIDEOIMAGE_SAMPLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WMT_VIDEOIMAGE_SAMPLE2 {
    pub dwMagic: u32,
    pub dwStructSize: u32,
    pub dwControlFlags: u32,
    pub dwViewportWidth: u32,
    pub dwViewportHeight: u32,
    pub dwCurrImageWidth: u32,
    pub dwCurrImageHeight: u32,
    pub fCurrRegionX0: f32,
    pub fCurrRegionY0: f32,
    pub fCurrRegionWidth: f32,
    pub fCurrRegionHeight: f32,
    pub fCurrBlendCoef: f32,
    pub dwPrevImageWidth: u32,
    pub dwPrevImageHeight: u32,
    pub fPrevRegionX0: f32,
    pub fPrevRegionY0: f32,
    pub fPrevRegionWidth: f32,
    pub fPrevRegionHeight: f32,
    pub fPrevBlendCoef: f32,
    pub dwEffectType: u32,
    pub dwNumEffectParas: u32,
    pub fEffectPara0: f32,
    pub fEffectPara1: f32,
    pub fEffectPara2: f32,
    pub fEffectPara3: f32,
    pub fEffectPara4: f32,
    pub bKeepPrevImage: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WMT_VIDEOIMAGE_SAMPLE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WMT_VIDEOIMAGE_SAMPLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_ADV_BLENDING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_BLENDING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_INPUT_FRAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_MOTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_OUTPUT_FRAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_ROTATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_USES_CURRENT_INPUT_FRAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_USES_PREVIOUS_INPUT_FRAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_BOW_TIE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_CIRCLE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_CROSS_FADE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_DIAGONAL: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_DIAMOND: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_FADE_TO_COLOR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_FILLED_V: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_FLIP: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_INSET: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_IRIS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_PAGE_ROLL: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_RECTANGLE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_REVEAL: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_SLIDE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_SPLIT: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_STAR: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_WHEEL: u32 = 31u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_WATERMARK_ENTRY {
    pub wmetType: WMT_WATERMARK_ENTRY_TYPE,
    pub clsid: ::windows_sys::core::GUID,
    pub cbDisplayName: u32,
    pub pwszDisplayName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WMT_WATERMARK_ENTRY {}
impl ::core::clone::Clone for WMT_WATERMARK_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WMT_WATERMARK_ENTRY_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_WMETYPE_AUDIO: WMT_WATERMARK_ENTRY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WMT_WMETYPE_VIDEO: WMT_WATERMARK_ENTRY_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_WEBSTREAM_FORMAT {
    pub cbSize: u16,
    pub cbSampleHeaderFixedData: u16,
    pub wVersion: u16,
    pub wReserved: u16,
}
impl ::core::marker::Copy for WMT_WEBSTREAM_FORMAT {}
impl ::core::clone::Clone for WMT_WEBSTREAM_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WMT_WEBSTREAM_SAMPLE_HEADER {
    pub cbLength: u16,
    pub wPart: u16,
    pub cTotalParts: u16,
    pub wSampleType: u16,
    pub wszURL: [u16; 1],
}
impl ::core::marker::Copy for WMT_WEBSTREAM_SAMPLE_HEADER {}
impl ::core::clone::Clone for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMVIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WMVIDEOINFOHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WMVIDEOINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMVIDEOINFOHEADER2 {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub dwInterlaceFlags: u32,
    pub dwCopyProtectFlags: u32,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WMVIDEOINFOHEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WMVIDEOINFOHEADER2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_ADDRESS_ACCESSENTRY {
    pub dwIPAddress: u32,
    pub dwMask: u32,
}
impl ::core::marker::Copy for WM_ADDRESS_ACCESSENTRY {}
impl ::core::clone::Clone for WM_ADDRESS_ACCESSENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WM_AETYPE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_AETYPE_INCLUDE: WM_AETYPE = 105i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_AETYPE_EXCLUDE: WM_AETYPE = 101i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_CLIENT_PROPERTIES {
    pub dwIPAddress: u32,
    pub dwPort: u32,
}
impl ::core::marker::Copy for WM_CLIENT_PROPERTIES {}
impl ::core::clone::Clone for WM_CLIENT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_CLIENT_PROPERTIES_EX {
    pub cbSize: u32,
    pub pwszIPAddress: ::windows_sys::core::PCWSTR,
    pub pwszPort: ::windows_sys::core::PCWSTR,
    pub pwszDNSName: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WM_CLIENT_PROPERTIES_EX {}
impl ::core::clone::Clone for WM_CLIENT_PROPERTIES_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_CL_INTERLACED420: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_CL_PROGRESSIVE420: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_CT_BOTTOM_FIELD_FIRST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_CT_INTERLACED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_CT_REPEAT_FIRST_FIELD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_CT_TOP_FIELD_FIRST: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WM_DM_INTERLACED_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_NOTINTERLACED: WM_DM_INTERLACED_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_DEINTERLACE_NORMAL: WM_DM_INTERLACED_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_DEINTERLACE_HALFSIZE: WM_DM_INTERLACED_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_DEINTERLACE_HALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_DEINTERLACE_INVERSETELECINE: WM_DM_INTERLACED_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_DEINTERLACE_VERTICALHALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WM_DM_IT_FIRST_FRAME_COHERENCY = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_DISABLE_COHERENT_MODE: WM_DM_IT_FIRST_FRAME_COHERENCY = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = 3i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = 4i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = 5i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = 6i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = 7i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = 8i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = 9i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = 10i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_LEAKY_BUCKET_PAIR {
    pub dwBitrate: u32,
    pub msBufferWindow: u32,
}
impl ::core::marker::Copy for WM_LEAKY_BUCKET_PAIR {}
impl ::core::clone::Clone for WM_LEAKY_BUCKET_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_MAX_STREAMS: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_MAX_VIDEO_STREAMS: u32 = 63u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_MEDIA_TYPE {
    pub majortype: ::windows_sys::core::GUID,
    pub subtype: ::windows_sys::core::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: ::windows_sys::core::GUID,
    pub pUnk: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_MEDIA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_PICTURE {
    pub pwszMIMEType: ::windows_sys::core::PWSTR,
    pub bPictureType: u8,
    pub pwszDescription: ::windows_sys::core::PWSTR,
    pub dwDataLen: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for WM_PICTURE {}
impl ::core::clone::Clone for WM_PICTURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WM_PLAYBACK_DRC_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_PLAYBACK_DRC_HIGH: WM_PLAYBACK_DRC_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_PLAYBACK_DRC_MEDIUM: WM_PLAYBACK_DRC_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_PLAYBACK_DRC_LOW: WM_PLAYBACK_DRC_LEVEL = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_PORT_NUMBER_RANGE {
    pub wPortBegin: u16,
    pub wPortEnd: u16,
}
impl ::core::marker::Copy for WM_PORT_NUMBER_RANGE {}
impl ::core::clone::Clone for WM_PORT_NUMBER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_READER_CLIENTINFO {
    pub cbSize: u32,
    pub wszLang: ::windows_sys::core::PWSTR,
    pub wszBrowserUserAgent: ::windows_sys::core::PWSTR,
    pub wszBrowserWebPage: ::windows_sys::core::PWSTR,
    pub qwReserved: u64,
    pub pReserved: *mut super::super::Foundation::LPARAM,
    pub wszHostExe: ::windows_sys::core::PWSTR,
    pub qwHostVersion: u64,
    pub wszPlayerUserAgent: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_READER_CLIENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_READER_CLIENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_READER_STATISTICS {
    pub cbSize: u32,
    pub dwBandwidth: u32,
    pub cPacketsReceived: u32,
    pub cPacketsRecovered: u32,
    pub cPacketsLost: u32,
    pub wQuality: u16,
}
impl ::core::marker::Copy for WM_READER_STATISTICS {}
impl ::core::clone::Clone for WM_READER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WM_SFEX_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SFEX_NOTASYNCPOINT: WM_SFEX_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SFEX_DATALOSS: WM_SFEX_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type WM_SF_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SF_CLEANPOINT: WM_SF_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SF_DISCONTINUITY: WM_SF_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SF_DATALOSS: WM_SF_TYPE = 4i32;
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_STREAM_PRIORITY_RECORD {
    pub wStreamNumber: u16,
    pub fMandatory: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_STREAM_PRIORITY_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_STREAM_PRIORITY_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_STREAM_TYPE_INFO {
    pub guidMajorType: ::windows_sys::core::GUID,
    pub cbFormat: u32,
}
impl ::core::marker::Copy for WM_STREAM_TYPE_INFO {}
impl ::core::clone::Clone for WM_STREAM_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_SYNCHRONISED_LYRICS {
    pub bTimeStampFormat: u8,
    pub bContentType: u8,
    pub pwszContentDescriptor: ::windows_sys::core::PWSTR,
    pub dwLyricsLen: u32,
    pub pbLyrics: *mut u8,
}
impl ::core::marker::Copy for WM_SYNCHRONISED_LYRICS {}
impl ::core::clone::Clone for WM_SYNCHRONISED_LYRICS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WM_SampleExtensionGUID_ChromaLocation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1281019040, data2: 37494, data3: 19244, data4: [158, 76, 160, 237, 239, 221, 33, 126] };
pub const WM_SampleExtensionGUID_ColorSpaceInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4154120790, data2: 12523, data3: 20267, data4: [159, 122, 242, 75, 19, 154, 17, 87] };
pub const WM_SampleExtensionGUID_ContentType: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3583040544, data2: 1980, data3: 17260, data4: [156, 247, 243, 187, 251, 241, 164, 220] };
pub const WM_SampleExtensionGUID_FileName: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3781553166, data2: 6637, data3: 17879, data4: [180, 167, 37, 203, 209, 226, 142, 155] };
pub const WM_SampleExtensionGUID_OutputCleanPoint: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4146740335, data2: 28340, data3: 20156, data4: [177, 146, 9, 173, 151, 89, 232, 40] };
pub const WM_SampleExtensionGUID_PixelAspectRatio: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 455009620, data2: 63978, data3: 19400, data4: [130, 26, 55, 107, 116, 228, 196, 184] };
pub const WM_SampleExtensionGUID_SampleDuration: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3334313040, data2: 34431, data3: 18695, data4: [131, 163, 199, 121, 33, 183, 51, 173] };
pub const WM_SampleExtensionGUID_SampleProtectionSalt: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1409539822, data2: 47598, data3: 17295, data4: [170, 131, 56, 4, 153, 126, 86, 157] };
pub const WM_SampleExtensionGUID_Timecode: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 966104556, data2: 34407, data3: 20013, data4: [143, 219, 152, 129, 76, 231, 108, 30] };
pub const WM_SampleExtensionGUID_UserDataInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1932244218, data2: 30910, data3: 17737, data4: [153, 189, 2, 219, 26, 85, 183, 168] };
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtension_ChromaLocation_Size: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtension_ColorSpaceInfo_Size: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtension_ContentType_Size: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtension_PixelAspectRatio_Size: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtension_SampleDuration_Size: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const WM_SampleExtension_Timecode_Size: u32 = 14u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_USER_TEXT {
    pub pwszDescription: ::windows_sys::core::PWSTR,
    pub pwszText: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WM_USER_TEXT {}
impl ::core::clone::Clone for WM_USER_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_USER_WEB_URL {
    pub pwszDescription: ::windows_sys::core::PWSTR,
    pub pwszURL: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WM_USER_WEB_URL {}
impl ::core::clone::Clone for WM_USER_WEB_URL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_WRITER_STATISTICS {
    pub qwSampleCount: u64,
    pub qwByteCount: u64,
    pub qwDroppedSampleCount: u64,
    pub qwDroppedByteCount: u64,
    pub dwCurrentBitrate: u32,
    pub dwAverageBitrate: u32,
    pub dwExpectedBitrate: u32,
    pub dwCurrentSampleRate: u32,
    pub dwAverageSampleRate: u32,
    pub dwExpectedSampleRate: u32,
}
impl ::core::marker::Copy for WM_WRITER_STATISTICS {}
impl ::core::clone::Clone for WM_WRITER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub struct WM_WRITER_STATISTICS_EX {
    pub dwBitratePlusOverhead: u32,
    pub dwCurrentSampleDropRateInQueue: u32,
    pub dwCurrentSampleDropRateInCodec: u32,
    pub dwCurrentSampleDropRateInMultiplexer: u32,
    pub dwTotalSampleDropsInQueue: u32,
    pub dwTotalSampleDropsInCodec: u32,
    pub dwTotalSampleDropsInMultiplexer: u32,
}
impl ::core::marker::Copy for WM_WRITER_STATISTICS_EX {}
impl ::core::clone::Clone for WM_WRITER_STATISTICS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub type _AM_ASFWRITERCONFIG_PARAM = i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const AM_CONFIGASFWRITER_PARAM_AUTOINDEX: _AM_ASFWRITERCONFIG_PARAM = 1i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const AM_CONFIGASFWRITER_PARAM_MULTIPASS: _AM_ASFWRITERCONFIG_PARAM = 2i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const AM_CONFIGASFWRITER_PARAM_DONTCOMPRESS: _AM_ASFWRITERCONFIG_PARAM = 3i32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_dwWMContentAttributes: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_dwWMNSCAttributes: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_dwWMSpecialAttributes: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszASFLeakyBucketPairs: &str = "ASFLeakyBucketPairs";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszAllowInterlacedOutput: &str = "AllowInterlacedOutput";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszAverageLevel: &str = "AverageLevel";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszBufferAverage: &str = "Buffer Average";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszComplexity: &str = "_COMPLEXITYEX";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszComplexityLive: &str = "_COMPLEXITYEXLIVE";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszComplexityMax: &str = "_COMPLEXITYEXMAX";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszComplexityOffline: &str = "_COMPLEXITYEXOFFLINE";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszDecoderComplexityRequested: &str = "_DECODERCOMPLEXITYPROFILE";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszDedicatedDeliveryThread: &str = "DedicatedDeliveryThread";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszDeinterlaceMode: &str = "DeinterlaceMode";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszDeliverOnReceive: &str = "DeliverOnReceive";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszDeviceConformanceTemplate: &str = "DeviceConformanceTemplate";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszDynamicRangeControl: &str = "DynamicRangeControl";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszEDL: &str = "_EDL";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszEarlyDataDelivery: &str = "EarlyDataDelivery";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszEnableDiscreteOutput: &str = "EnableDiscreteOutput";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszEnableFrameInterpolation: &str = "EnableFrameInterpolation";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszEnableWMAProSPDIFOutput: &str = "EnableWMAProSPDIFOutput";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszFailSeekOnError: &str = "FailSeekOnError";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszFixedFrameRate: &str = "FixedFrameRate";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszFold6To2Channels3: &str = "Fold6To2Channels3";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszFoldToChannelsTemplate: &str = "Fold%luTo%luChannels%lu";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszInitialPatternForInverseTelecine: &str = "InitialPatternForInverseTelecine";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszInterlacedCoding: &str = "InterlacedCoding";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszIsVBRSupported: &str = "_ISVBRSUPPORTED";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszJPEGCompressionQuality: &str = "JPEGCompressionQuality";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszJustInTimeDecode: &str = "JustInTimeDecode";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszMixedClassMode: &str = "MixedClassMode";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszMusicClassMode: &str = "MusicClassMode";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszMusicSpeechClassMode: &str = "MusicSpeechClassMode";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszNeedsPreviousSample: &str = "NeedsPreviousSample";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszNumPasses: &str = "_PASSESUSED";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszOriginalSourceFormatTag: &str = "_SOURCEFORMATTAG";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszOriginalWaveFormat: &str = "_ORIGINALWAVEFORMAT";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszPeakValue: &str = "PeakValue";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszPermitSeeksBeyondEndOfStream: &str = "PermitSeeksBeyondEndOfStream";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszReloadIndexOnSeek: &str = "ReloadIndexOnSeek";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszScrambledAudio: &str = "ScrambledAudio";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSingleOutputBuffer: &str = "SingleOutputBuffer";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSoftwareScaling: &str = "SoftwareScaling";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSourceBufferTime: &str = "SourceBufferTime";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSourceMaxBytesAtOnce: &str = "SourceMaxBytesAtOnce";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSpeakerConfig: &str = "SpeakerConfig";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSpeechCaps: &str = "SpeechFormatCap";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszSpeechClassMode: &str = "SpeechClassMode";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszStreamLanguage: &str = "StreamLanguage";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszStreamNumIndexObjects: &str = "StreamNumIndexObjects";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszUsePacketAtSeekPoint: &str = "UsePacketAtSeekPoint";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszVBRBitrateMax: &str = "_RMAX";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszVBRBufferWindowMax: &str = "_BMAX";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszVBREnabled: &str = "_VBRENABLED";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszVBRPeak: &str = "VBR Peak";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszVBRQuality: &str = "_VBRQUALITY";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszVideoSampleDurations: &str = "VideoSampleDurations";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMADID: &str = "WM/ADID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMASFPacketCount: &str = "WM/ASFPacketCount";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMASFSecurityObjectsSize: &str = "WM/ASFSecurityObjectsSize";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAlbumArtist: &str = "WM/AlbumArtist";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAlbumArtistSort: &str = "WM/AlbumArtistSort";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAlbumCoverURL: &str = "WM/AlbumCoverURL";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAlbumTitle: &str = "WM/AlbumTitle";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAlbumTitleSort: &str = "WM/AlbumTitleSort";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAspectRatioX: &str = "AspectRatioX";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAspectRatioY: &str = "AspectRatioY";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAudioFileURL: &str = "WM/AudioFileURL";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAudioSourceURL: &str = "WM/AudioSourceURL";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAuthor: &str = "Author";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAuthorSort: &str = "AuthorSort";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMAuthorURL: &str = "WM/AuthorURL";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMBannerImageData: &str = "BannerImageData";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMBannerImageType: &str = "BannerImageType";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMBannerImageURL: &str = "BannerImageURL";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMBeatsPerMinute: &str = "WM/BeatsPerMinute";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMBitrate: &str = "Bitrate";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMBroadcast: &str = "Broadcast";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMCategory: &str = "WM/Category";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMCodec: &str = "WM/Codec";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMComposer: &str = "WM/Composer";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMComposerSort: &str = "WM/ComposerSort";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMConductor: &str = "WM/Conductor";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMContainerFormat: &str = "WM/ContainerFormat";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMContentDistributor: &str = "WM/ContentDistributor";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMContentGroupDescription: &str = "WM/ContentGroupDescription";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMCopyright: &str = "Copyright";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMCopyrightURL: &str = "CopyrightURL";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMCurrentBitrate: &str = "CurrentBitrate";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM: &str = "WM/DRM";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_ContentID: &str = "DRM_ContentID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_Flags: &str = "DRM_Flags";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_HeaderSignPrivKey: &str = "DRM_HeaderSignPrivKey";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_IndividualizedVersion: &str = "DRM_IndividualizedVersion";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_KeyID: &str = "DRM_KeyID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_KeySeed: &str = "DRM_KeySeed";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_LASignatureCert: &str = "DRM_LASignatureCert";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_LASignatureLicSrvCert: &str = "DRM_LASignatureLicSrvCert";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_LASignaturePrivKey: &str = "DRM_LASignaturePrivKey";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_LASignatureRootCert: &str = "DRM_LASignatureRootCert";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_Level: &str = "DRM_Level";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_LicenseAcqURL: &str = "DRM_LicenseAcqURL";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_SourceID: &str = "DRM_SourceID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDRM_V1LicenseAcqURL: &str = "DRM_V1LicenseAcqURL";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDVDID: &str = "WM/DVDID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDescription: &str = "Description";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDirector: &str = "WM/Director";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMDuration: &str = "Duration";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMEncodedBy: &str = "WM/EncodedBy";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMEncodingSettings: &str = "WM/EncodingSettings";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMEncodingTime: &str = "WM/EncodingTime";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMEpisodeNumber: &str = "WM/EpisodeNumber";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMFileSize: &str = "FileSize";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMGenre: &str = "WM/Genre";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMGenreID: &str = "WM/GenreID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasArbitraryDataStream: &str = "HasArbitraryDataStream";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasAttachedImages: &str = "HasAttachedImages";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasAudio: &str = "HasAudio";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasFileTransferStream: &str = "HasFileTransferStream";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasImage: &str = "HasImage";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasScript: &str = "HasScript";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMHasVideo: &str = "HasVideo";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMISAN: &str = "WM/ISAN";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMISRC: &str = "WM/ISRC";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMInitialKey: &str = "WM/InitialKey";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMIsCompilation: &str = "WM/IsCompilation";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMIsVBR: &str = "IsVBR";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMLanguage: &str = "WM/Language";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMLyrics: &str = "WM/Lyrics";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMLyrics_Synchronised: &str = "WM/Lyrics_Synchronised";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMCDI: &str = "WM/MCDI";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaClassPrimaryID: &str = "WM/MediaClassPrimaryID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaClassSecondaryID: &str = "WM/MediaClassSecondaryID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaCredits: &str = "WM/MediaCredits";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsDelay: &str = "WM/MediaIsDelay";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsFinale: &str = "WM/MediaIsFinale";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsLive: &str = "WM/MediaIsLive";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsPremiere: &str = "WM/MediaIsPremiere";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsRepeat: &str = "WM/MediaIsRepeat";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsSAP: &str = "WM/MediaIsSAP";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsStereo: &str = "WM/MediaIsStereo";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsSubtitled: &str = "WM/MediaIsSubtitled";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaIsTape: &str = "WM/MediaIsTape";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaNetworkAffiliation: &str = "WM/MediaNetworkAffiliation";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaOriginalBroadcastDateTime: &str = "WM/MediaOriginalBroadcastDateTime";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaOriginalChannel: &str = "WM/MediaOriginalChannel";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaStationCallSign: &str = "WM/MediaStationCallSign";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMediaStationName: &str = "WM/MediaStationName";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMModifiedBy: &str = "WM/ModifiedBy";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMMood: &str = "WM/Mood";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMNSCAddress: &str = "NSC_Address";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMNSCDescription: &str = "NSC_Description";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMNSCEmail: &str = "NSC_Email";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMNSCName: &str = "NSC_Name";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMNSCPhone: &str = "NSC_Phone";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMNumberOfFrames: &str = "NumberOfFrames";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOptimalBitrate: &str = "OptimalBitrate";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOriginalAlbumTitle: &str = "WM/OriginalAlbumTitle";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOriginalArtist: &str = "WM/OriginalArtist";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOriginalFilename: &str = "WM/OriginalFilename";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOriginalLyricist: &str = "WM/OriginalLyricist";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOriginalReleaseTime: &str = "WM/OriginalReleaseTime";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMOriginalReleaseYear: &str = "WM/OriginalReleaseYear";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMParentalRating: &str = "WM/ParentalRating";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMParentalRatingReason: &str = "WM/ParentalRatingReason";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPartOfSet: &str = "WM/PartOfSet";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPeakBitrate: &str = "WM/PeakBitrate";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPeriod: &str = "WM/Period";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPicture: &str = "WM/Picture";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPlaylistDelay: &str = "WM/PlaylistDelay";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProducer: &str = "WM/Producer";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPromotionURL: &str = "WM/PromotionURL";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProtected: &str = "Is_Protected";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProtectionType: &str = "WM/ProtectionType";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProvider: &str = "WM/Provider";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProviderCopyright: &str = "WM/ProviderCopyright";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProviderRating: &str = "WM/ProviderRating";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMProviderStyle: &str = "WM/ProviderStyle";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMPublisher: &str = "WM/Publisher";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMRadioStationName: &str = "WM/RadioStationName";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMRadioStationOwner: &str = "WM/RadioStationOwner";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMRating: &str = "Rating";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSeasonNumber: &str = "WM/SeasonNumber";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSeekable: &str = "Seekable";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSharedUserRating: &str = "WM/SharedUserRating";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSignature_Name: &str = "Signature_Name";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSkipBackward: &str = "Can_Skip_Backward";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSkipForward: &str = "Can_Skip_Forward";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMStreamTypeInfo: &str = "WM/StreamTypeInfo";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMStridable: &str = "Stridable";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSubTitle: &str = "WM/SubTitle";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSubTitleDescription: &str = "WM/SubTitleDescription";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMSubscriptionContentID: &str = "WM/SubscriptionContentID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMText: &str = "WM/Text";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMTitle: &str = "Title";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMTitleSort: &str = "TitleSort";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMToolName: &str = "WM/ToolName";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMToolVersion: &str = "WM/ToolVersion";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMTrack: &str = "WM/Track";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMTrackNumber: &str = "WM/TrackNumber";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMTrusted: &str = "Is_Trusted";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMUniqueFileIdentifier: &str = "WM/UniqueFileIdentifier";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMUse_Advanced_DRM: &str = "Use_Advanced_DRM";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMUse_DRM: &str = "Use_DRM";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMUserWebURL: &str = "WM/UserWebURL";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMVideoClosedCaptioning: &str = "WM/VideoClosedCaptioning";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMVideoFrameRate: &str = "WM/VideoFrameRate";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMVideoHeight: &str = "WM/VideoHeight";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMVideoWidth: &str = "WM/VideoWidth";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMADRCAverageReference: &str = "WM/WMADRCAverageReference";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMADRCAverageTarget: &str = "WM/WMADRCAverageTarget";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMADRCPeakReference: &str = "WM/WMADRCPeakReference";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMADRCPeakTarget: &str = "WM/WMADRCPeakTarget";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMCPDistributor: &str = "WM/WMCPDistributor";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMCPDistributorID: &str = "WM/WMCPDistributorID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMCollectionGroupID: &str = "WM/WMCollectionGroupID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMCollectionID: &str = "WM/WMCollectionID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMContentID: &str = "WM/WMContentID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMShadowFileSourceDRMType: &str = "WM/WMShadowFileSourceDRMType";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWMShadowFileSourceFileType: &str = "WM/WMShadowFileSourceFileType";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMWriter: &str = "WM/Writer";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWMYear: &str = "WM/Year";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWatermarkCLSID: &str = "WatermarkCLSID";
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`*"]
pub const g_wszWatermarkConfig: &str = "WatermarkConfig";
