#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CanSendToFaxRecipient() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxAbort(faxhandle: super::super::Foundation::HANDLE, jobid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxAccessCheck(faxhandle: super::super::Foundation::HANDLE, accessmask: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxClose(faxhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxCompleteJobParamsA(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxCompleteJobParamsW(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxConnectFaxServerA(machinename: ::windows_sys::core::PCSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxConnectFaxServerW(machinename: ::windows_sys::core::PCWSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnableRoutingMethodA(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_sys::core::PCSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnableRoutingMethodW(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_sys::core::PCWSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumJobsA(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumJobsW(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumPortsA(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumPortsW(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumRoutingMethodsA(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumRoutingMethodsW(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub fn FaxFreeBuffer(buffer: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetDeviceStatusA(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetDeviceStatusW(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPageData(faxhandle: super::super::Foundation::HANDLE, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetRoutingInfoA(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_sys::core::PCSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetRoutingInfoW(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_sys::core::PCWSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxInitializeEventQueue(faxhandle: super::super::Foundation::HANDLE, completionport: super::super::Foundation::HANDLE, completionkey: usize, hwnd: super::super::Foundation::HWND, messagestart: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxOpenPort(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxPrintCoverPageA(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxPrintCoverPageW(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxRegisterRoutingExtensionW(faxhandle: super::super::Foundation::HANDLE, extensionname: ::windows_sys::core::PCWSTR, friendlyname: ::windows_sys::core::PCWSTR, imagename: ::windows_sys::core::PCWSTR, callback: PFAX_ROUTING_INSTALLATION_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxRegisterServiceProviderW(deviceprovider: ::windows_sys::core::PCWSTR, friendlyname: ::windows_sys::core::PCWSTR, imagename: ::windows_sys::core::PCWSTR, tspname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentA(faxhandle: super::super::Foundation::HANDLE, filename: ::windows_sys::core::PCSTR, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentForBroadcastA(faxhandle: super::super::Foundation::HANDLE, filename: ::windows_sys::core::PCSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKA, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentForBroadcastW(faxhandle: super::super::Foundation::HANDLE, filename: ::windows_sys::core::PCWSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentW(faxhandle: super::super::Foundation::HANDLE, filename: ::windows_sys::core::PCWSTR, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetRoutingInfoA(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_sys::core::PCSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetRoutingInfoW(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_sys::core::PCWSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxStartPrintJobA(printername: ::windows_sys::core::PCSTR, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxStartPrintJobW(printername: ::windows_sys::core::PCWSTR, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxUnregisterServiceProviderW(deviceprovider: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub fn SendToFaxRecipient(sndmode: SendToMode, lpfilename: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StiCreateInstanceW(hinst: super::super::Foundation::HINSTANCE, dwver: u32, ppsti: *mut *mut *mut IStillImageW, punkouter: *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_DEVICE_ID: &str = "FAXSRV_DeviceID";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_FSP_GUID: &str = "FAXSRV_FSPGuid";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_ROUTEEXT_NAME: &str = "FAXSRV_RoutingExtName";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_ROUTING_METHOD_GUID: &str = "FAXSRV_RoutingMethodGuid";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_SERVER_NAME: &str = "FAXSRV_ServerName";
pub const CLSID_Sti: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3005479136, data2: 11880, data3: 4560, data4: [144, 234, 0, 170, 0, 96, 248, 108] };
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WIA_DeviceType: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 1809653702, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WIA_USDClassId: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 1809653702, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXDEVRECEIVE_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXDEVREPORTSTATUS_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAXROUTE_ENABLE = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const QUERY_STATUS: FAXROUTE_ENABLE = -1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STATUS_DISABLE: FAXROUTE_ENABLE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STATUS_ENABLE: FAXROUTE_ENABLE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_ACCESS_RIGHTS_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farSUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farSUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farSUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farQUERY_JOBS: FAX_ACCESS_RIGHTS_ENUM = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farMANAGE_JOBS: FAX_ACCESS_RIGHTS_ENUM = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farQUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM = 32i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farMANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM = 64i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farQUERY_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = 128i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farMANAGE_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = 256i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farQUERY_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = 512i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farMANAGE_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = 1024i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_ACCESS_RIGHTS_ENUM_2 = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2SUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM_2 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2SUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM_2 = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2SUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM_2 = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2QUERY_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2MANAGE_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2QUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = 32i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2MANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = 64i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2QUERY_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = 128i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2MANAGE_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = 256i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2MANAGE_RECEIVE_FOLDER: FAX_ACCESS_RIGHTS_ENUM_2 = 512i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_ACCOUNT_EVENTS_TYPE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetNONE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetIN_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetOUT_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetIN_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetOUT_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetFXSSVC_ENDED: FAX_ACCOUNT_EVENTS_TYPE_ENUM = 16i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_CONFIGURATIONA {
    pub SizeOfStruct: u32,
    pub Retries: u32,
    pub RetryDelay: u32,
    pub DirtyDays: u32,
    pub Branding: super::super::Foundation::BOOL,
    pub UseDeviceTsid: super::super::Foundation::BOOL,
    pub ServerCp: super::super::Foundation::BOOL,
    pub PauseServerQueue: super::super::Foundation::BOOL,
    pub StartCheapTime: FAX_TIME,
    pub StopCheapTime: FAX_TIME,
    pub ArchiveOutgoingFaxes: super::super::Foundation::BOOL,
    pub ArchiveDirectory: ::windows_sys::core::PCSTR,
    pub Reserved: ::windows_sys::core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_CONFIGURATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_CONFIGURATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_CONFIGURATIONW {
    pub SizeOfStruct: u32,
    pub Retries: u32,
    pub RetryDelay: u32,
    pub DirtyDays: u32,
    pub Branding: super::super::Foundation::BOOL,
    pub UseDeviceTsid: super::super::Foundation::BOOL,
    pub ServerCp: super::super::Foundation::BOOL,
    pub PauseServerQueue: super::super::Foundation::BOOL,
    pub StartCheapTime: FAX_TIME,
    pub StopCheapTime: FAX_TIME,
    pub ArchiveOutgoingFaxes: super::super::Foundation::BOOL,
    pub ArchiveDirectory: ::windows_sys::core::PCWSTR,
    pub Reserved: ::windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_CONFIGURATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_CONFIGURATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_CONFIG_QUERY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_CONFIG_SET: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct FAX_CONTEXT_INFOA {
    pub SizeOfStruct: u32,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub ServerName: [super::super::Foundation::CHAR; 16],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for FAX_CONTEXT_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for FAX_CONTEXT_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct FAX_CONTEXT_INFOW {
    pub SizeOfStruct: u32,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub ServerName: [u16; 16],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for FAX_CONTEXT_INFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for FAX_CONTEXT_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_COVERPAGE_INFOA {
    pub SizeOfStruct: u32,
    pub CoverPageName: ::windows_sys::core::PCSTR,
    pub UseServerCoverPage: super::super::Foundation::BOOL,
    pub RecName: ::windows_sys::core::PCSTR,
    pub RecFaxNumber: ::windows_sys::core::PCSTR,
    pub RecCompany: ::windows_sys::core::PCSTR,
    pub RecStreetAddress: ::windows_sys::core::PCSTR,
    pub RecCity: ::windows_sys::core::PCSTR,
    pub RecState: ::windows_sys::core::PCSTR,
    pub RecZip: ::windows_sys::core::PCSTR,
    pub RecCountry: ::windows_sys::core::PCSTR,
    pub RecTitle: ::windows_sys::core::PCSTR,
    pub RecDepartment: ::windows_sys::core::PCSTR,
    pub RecOfficeLocation: ::windows_sys::core::PCSTR,
    pub RecHomePhone: ::windows_sys::core::PCSTR,
    pub RecOfficePhone: ::windows_sys::core::PCSTR,
    pub SdrName: ::windows_sys::core::PCSTR,
    pub SdrFaxNumber: ::windows_sys::core::PCSTR,
    pub SdrCompany: ::windows_sys::core::PCSTR,
    pub SdrAddress: ::windows_sys::core::PCSTR,
    pub SdrTitle: ::windows_sys::core::PCSTR,
    pub SdrDepartment: ::windows_sys::core::PCSTR,
    pub SdrOfficeLocation: ::windows_sys::core::PCSTR,
    pub SdrHomePhone: ::windows_sys::core::PCSTR,
    pub SdrOfficePhone: ::windows_sys::core::PCSTR,
    pub Note: ::windows_sys::core::PCSTR,
    pub Subject: ::windows_sys::core::PCSTR,
    pub TimeSent: super::super::Foundation::SYSTEMTIME,
    pub PageCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_COVERPAGE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_COVERPAGE_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_COVERPAGE_INFOW {
    pub SizeOfStruct: u32,
    pub CoverPageName: ::windows_sys::core::PCWSTR,
    pub UseServerCoverPage: super::super::Foundation::BOOL,
    pub RecName: ::windows_sys::core::PCWSTR,
    pub RecFaxNumber: ::windows_sys::core::PCWSTR,
    pub RecCompany: ::windows_sys::core::PCWSTR,
    pub RecStreetAddress: ::windows_sys::core::PCWSTR,
    pub RecCity: ::windows_sys::core::PCWSTR,
    pub RecState: ::windows_sys::core::PCWSTR,
    pub RecZip: ::windows_sys::core::PCWSTR,
    pub RecCountry: ::windows_sys::core::PCWSTR,
    pub RecTitle: ::windows_sys::core::PCWSTR,
    pub RecDepartment: ::windows_sys::core::PCWSTR,
    pub RecOfficeLocation: ::windows_sys::core::PCWSTR,
    pub RecHomePhone: ::windows_sys::core::PCWSTR,
    pub RecOfficePhone: ::windows_sys::core::PCWSTR,
    pub SdrName: ::windows_sys::core::PCWSTR,
    pub SdrFaxNumber: ::windows_sys::core::PCWSTR,
    pub SdrCompany: ::windows_sys::core::PCWSTR,
    pub SdrAddress: ::windows_sys::core::PCWSTR,
    pub SdrTitle: ::windows_sys::core::PCWSTR,
    pub SdrDepartment: ::windows_sys::core::PCWSTR,
    pub SdrOfficeLocation: ::windows_sys::core::PCWSTR,
    pub SdrHomePhone: ::windows_sys::core::PCWSTR,
    pub SdrOfficePhone: ::windows_sys::core::PCWSTR,
    pub Note: ::windows_sys::core::PCWSTR,
    pub Subject: ::windows_sys::core::PCWSTR,
    pub TimeSent: super::super::Foundation::SYSTEMTIME,
    pub PageCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_COVERPAGE_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_COVERPAGE_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_COVERPAGE_TYPE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fcptNONE: FAX_COVERPAGE_TYPE_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fcptLOCAL: FAX_COVERPAGE_TYPE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fcptSERVER: FAX_COVERPAGE_TYPE_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_DEVICE_RECEIVE_MODE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fdrmNO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fdrmAUTO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fdrmMANUAL_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_DEVICE_STATUSA {
    pub SizeOfStruct: u32,
    pub CallerId: ::windows_sys::core::PCSTR,
    pub Csid: ::windows_sys::core::PCSTR,
    pub CurrentPage: u32,
    pub DeviceId: u32,
    pub DeviceName: ::windows_sys::core::PCSTR,
    pub DocumentName: ::windows_sys::core::PCSTR,
    pub JobType: u32,
    pub PhoneNumber: ::windows_sys::core::PCSTR,
    pub RoutingString: ::windows_sys::core::PCSTR,
    pub SenderName: ::windows_sys::core::PCSTR,
    pub RecipientName: ::windows_sys::core::PCSTR,
    pub Size: u32,
    pub StartTime: super::super::Foundation::FILETIME,
    pub Status: u32,
    pub StatusString: ::windows_sys::core::PCSTR,
    pub SubmittedTime: super::super::Foundation::FILETIME,
    pub TotalPages: u32,
    pub Tsid: ::windows_sys::core::PCSTR,
    pub UserName: ::windows_sys::core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_DEVICE_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_DEVICE_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_DEVICE_STATUSW {
    pub SizeOfStruct: u32,
    pub CallerId: ::windows_sys::core::PCWSTR,
    pub Csid: ::windows_sys::core::PCWSTR,
    pub CurrentPage: u32,
    pub DeviceId: u32,
    pub DeviceName: ::windows_sys::core::PCWSTR,
    pub DocumentName: ::windows_sys::core::PCWSTR,
    pub JobType: u32,
    pub PhoneNumber: ::windows_sys::core::PCWSTR,
    pub RoutingString: ::windows_sys::core::PCWSTR,
    pub SenderName: ::windows_sys::core::PCWSTR,
    pub RecipientName: ::windows_sys::core::PCWSTR,
    pub Size: u32,
    pub StartTime: super::super::Foundation::FILETIME,
    pub Status: u32,
    pub StatusString: ::windows_sys::core::PCWSTR,
    pub SubmittedTime: super::super::Foundation::FILETIME,
    pub TotalPages: u32,
    pub Tsid: ::windows_sys::core::PCWSTR,
    pub UserName: ::windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_DEVICE_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_DEVICE_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_DEV_STATUS {
    pub SizeOfStruct: u32,
    pub StatusId: u32,
    pub StringId: u32,
    pub PageCount: u32,
    pub CSI: ::windows_sys::core::PWSTR,
    pub CallerId: ::windows_sys::core::PWSTR,
    pub RoutingInfo: ::windows_sys::core::PWSTR,
    pub ErrorCode: u32,
    pub Reserved: [u32; 3],
}
impl ::core::marker::Copy for FAX_DEV_STATUS {}
impl ::core::clone::Clone for FAX_DEV_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_ENUM_DELIVERY_REPORT_TYPES = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DRT_NONE: FAX_ENUM_DELIVERY_REPORT_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DRT_EMAIL: FAX_ENUM_DELIVERY_REPORT_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DRT_INBOX: FAX_ENUM_DELIVERY_REPORT_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_ENUM_DEVICE_ID_SOURCE = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DEV_ID_SRC_FAX: FAX_ENUM_DEVICE_ID_SOURCE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DEV_ID_SRC_TAPI: FAX_ENUM_DEVICE_ID_SOURCE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_ENUM_JOB_COMMANDS = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JC_UNKNOWN: FAX_ENUM_JOB_COMMANDS = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JC_DELETE: FAX_ENUM_JOB_COMMANDS = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JC_PAUSE: FAX_ENUM_JOB_COMMANDS = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JC_RESUME: FAX_ENUM_JOB_COMMANDS = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_ENUM_JOB_SEND_ATTRIBUTES = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JSA_NOW: FAX_ENUM_JOB_SEND_ATTRIBUTES = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JSA_SPECIFIC_TIME: FAX_ENUM_JOB_SEND_ATTRIBUTES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JSA_DISCOUNT_PERIOD: FAX_ENUM_JOB_SEND_ATTRIBUTES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_ENUM_LOG_CATEGORIES = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_CATEGORY_INIT: FAX_ENUM_LOG_CATEGORIES = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_CATEGORY_OUTBOUND: FAX_ENUM_LOG_CATEGORIES = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_CATEGORY_INBOUND: FAX_ENUM_LOG_CATEGORIES = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_CATEGORY_UNKNOWN: FAX_ENUM_LOG_CATEGORIES = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_ENUM_LOG_LEVELS = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_LEVEL_NONE: FAX_ENUM_LOG_LEVELS = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_LEVEL_MIN: FAX_ENUM_LOG_LEVELS = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_LEVEL_MED: FAX_ENUM_LOG_LEVELS = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_LEVEL_MAX: FAX_ENUM_LOG_LEVELS = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_ENUM_PORT_OPEN_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const PORT_OPEN_QUERY: FAX_ENUM_PORT_OPEN_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const PORT_OPEN_MODIFY: FAX_ENUM_PORT_OPEN_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_BAD_GROUP_CONFIGURATION: i32 = 7003i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_DEVICE_NUM_LIMIT_EXCEEDED: i32 = 7010i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_DIRECTORY_IN_USE: i32 = 7007i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_END: i32 = 7013i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_FILE_ACCESS_DENIED: i32 = 7008i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_GROUP_IN_USE: i32 = 7004i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_GROUP_NOT_FOUND: i32 = 7002i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_MESSAGE_NOT_FOUND: i32 = 7009i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_NOT_NTFS: i32 = 7006i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_NOT_SUPPORTED_ON_THIS_SKU: i32 = 7011i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_RECIPIENTS_LIMIT: i32 = 7013i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_RULE_NOT_FOUND: i32 = 7005i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_SRV_OUTOFMEMORY: i32 = 7001i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_START: i32 = 7001i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_VERSION_MISMATCH: i32 = 7012i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_EVENTA {
    pub SizeOfStruct: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub DeviceId: u32,
    pub EventId: u32,
    pub JobId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_EVENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_EVENTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_EVENTW {
    pub SizeOfStruct: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub DeviceId: u32,
    pub EventId: u32,
    pub JobId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_EVENTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_EVENTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_BAD_GROUP_CONFIGURATION: ::windows_sys::core::HRESULT = -2147214501i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_DEVICE_NUM_LIMIT_EXCEEDED: ::windows_sys::core::HRESULT = -2147214494i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_DIRECTORY_IN_USE: ::windows_sys::core::HRESULT = -2147214497i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_FILE_ACCESS_DENIED: ::windows_sys::core::HRESULT = -2147214496i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_GROUP_IN_USE: ::windows_sys::core::HRESULT = -2147214500i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_GROUP_NOT_FOUND: ::windows_sys::core::HRESULT = -2147214502i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_MESSAGE_NOT_FOUND: ::windows_sys::core::HRESULT = -2147214495i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_NOT_NTFS: ::windows_sys::core::HRESULT = -2147214498i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_NOT_SUPPORTED_ON_THIS_SKU: ::windows_sys::core::HRESULT = -2147214493i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_RECIPIENTS_LIMIT: ::windows_sys::core::HRESULT = -2147214491i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_RULE_NOT_FOUND: ::windows_sys::core::HRESULT = -2147214499i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_SRV_OUTOFMEMORY: ::windows_sys::core::HRESULT = -2147214503i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_VERSION_MISMATCH: ::windows_sys::core::HRESULT = -2147214492i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_GLOBAL_ROUTING_INFOA {
    pub SizeOfStruct: u32,
    pub Priority: u32,
    pub Guid: ::windows_sys::core::PCSTR,
    pub FriendlyName: ::windows_sys::core::PCSTR,
    pub FunctionName: ::windows_sys::core::PCSTR,
    pub ExtensionImageName: ::windows_sys::core::PCSTR,
    pub ExtensionFriendlyName: ::windows_sys::core::PCSTR,
}
impl ::core::marker::Copy for FAX_GLOBAL_ROUTING_INFOA {}
impl ::core::clone::Clone for FAX_GLOBAL_ROUTING_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_GLOBAL_ROUTING_INFOW {
    pub SizeOfStruct: u32,
    pub Priority: u32,
    pub Guid: ::windows_sys::core::PCWSTR,
    pub FriendlyName: ::windows_sys::core::PCWSTR,
    pub FunctionName: ::windows_sys::core::PCWSTR,
    pub ExtensionImageName: ::windows_sys::core::PCWSTR,
    pub ExtensionFriendlyName: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for FAX_GLOBAL_ROUTING_INFOW {}
impl ::core::clone::Clone for FAX_GLOBAL_ROUTING_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_GROUP_STATUS_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fgsALL_DEV_VALID: FAX_GROUP_STATUS_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fgsEMPTY: FAX_GROUP_STATUS_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fgsALL_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fgsSOME_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_ENTRYA {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub UserName: ::windows_sys::core::PCSTR,
    pub JobType: u32,
    pub QueueStatus: u32,
    pub Status: u32,
    pub Size: u32,
    pub PageCount: u32,
    pub RecipientNumber: ::windows_sys::core::PCSTR,
    pub RecipientName: ::windows_sys::core::PCSTR,
    pub Tsid: ::windows_sys::core::PCSTR,
    pub SenderName: ::windows_sys::core::PCSTR,
    pub SenderCompany: ::windows_sys::core::PCSTR,
    pub SenderDept: ::windows_sys::core::PCSTR,
    pub BillingCode: ::windows_sys::core::PCSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: ::windows_sys::core::PCSTR,
    pub DocumentName: ::windows_sys::core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_JOB_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_JOB_ENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_ENTRYW {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub UserName: ::windows_sys::core::PCWSTR,
    pub JobType: u32,
    pub QueueStatus: u32,
    pub Status: u32,
    pub Size: u32,
    pub PageCount: u32,
    pub RecipientNumber: ::windows_sys::core::PCWSTR,
    pub RecipientName: ::windows_sys::core::PCWSTR,
    pub Tsid: ::windows_sys::core::PCWSTR,
    pub SenderName: ::windows_sys::core::PCWSTR,
    pub SenderCompany: ::windows_sys::core::PCWSTR,
    pub SenderDept: ::windows_sys::core::PCWSTR,
    pub BillingCode: ::windows_sys::core::PCWSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: ::windows_sys::core::PCWSTR,
    pub DocumentName: ::windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_JOB_ENTRYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_JOB_ENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_JOB_EXTENDED_STATUS_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesNONE: FAX_JOB_EXTENDED_STATUS_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesDISCONNECTED: FAX_JOB_EXTENDED_STATUS_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesINITIALIZING: FAX_JOB_EXTENDED_STATUS_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesDIALING: FAX_JOB_EXTENDED_STATUS_ENUM = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesTRANSMITTING: FAX_JOB_EXTENDED_STATUS_ENUM = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesANSWERED: FAX_JOB_EXTENDED_STATUS_ENUM = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesRECEIVING: FAX_JOB_EXTENDED_STATUS_ENUM = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesLINE_UNAVAILABLE: FAX_JOB_EXTENDED_STATUS_ENUM = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesBUSY: FAX_JOB_EXTENDED_STATUS_ENUM = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesNO_ANSWER: FAX_JOB_EXTENDED_STATUS_ENUM = 9i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesBAD_ADDRESS: FAX_JOB_EXTENDED_STATUS_ENUM = 10i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesNO_DIAL_TONE: FAX_JOB_EXTENDED_STATUS_ENUM = 11i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesFATAL_ERROR: FAX_JOB_EXTENDED_STATUS_ENUM = 12i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesCALL_DELAYED: FAX_JOB_EXTENDED_STATUS_ENUM = 13i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesCALL_BLACKLISTED: FAX_JOB_EXTENDED_STATUS_ENUM = 14i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesNOT_FAX_CALL: FAX_JOB_EXTENDED_STATUS_ENUM = 15i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesPARTIALLY_RECEIVED: FAX_JOB_EXTENDED_STATUS_ENUM = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesHANDLED: FAX_JOB_EXTENDED_STATUS_ENUM = 17i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesCALL_COMPLETED: FAX_JOB_EXTENDED_STATUS_ENUM = 18i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesCALL_ABORTED: FAX_JOB_EXTENDED_STATUS_ENUM = 19i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesPROPRIETARY: FAX_JOB_EXTENDED_STATUS_ENUM = 16777216i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_JOB_MANAGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_JOB_OPERATIONS_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoVIEW: FAX_JOB_OPERATIONS_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoPAUSE: FAX_JOB_OPERATIONS_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoRESUME: FAX_JOB_OPERATIONS_ENUM = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoRESTART: FAX_JOB_OPERATIONS_ENUM = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoDELETE: FAX_JOB_OPERATIONS_ENUM = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoRECIPIENT_INFO: FAX_JOB_OPERATIONS_ENUM = 32i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoSENDER_INFO: FAX_JOB_OPERATIONS_ENUM = 64i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_PARAMA {
    pub SizeOfStruct: u32,
    pub RecipientNumber: ::windows_sys::core::PCSTR,
    pub RecipientName: ::windows_sys::core::PCSTR,
    pub Tsid: ::windows_sys::core::PCSTR,
    pub SenderName: ::windows_sys::core::PCSTR,
    pub SenderCompany: ::windows_sys::core::PCSTR,
    pub SenderDept: ::windows_sys::core::PCSTR,
    pub BillingCode: ::windows_sys::core::PCSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: ::windows_sys::core::PCSTR,
    pub DocumentName: ::windows_sys::core::PCSTR,
    pub CallHandle: u32,
    pub Reserved: [usize; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_JOB_PARAMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_JOB_PARAMA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_PARAMW {
    pub SizeOfStruct: u32,
    pub RecipientNumber: ::windows_sys::core::PCWSTR,
    pub RecipientName: ::windows_sys::core::PCWSTR,
    pub Tsid: ::windows_sys::core::PCWSTR,
    pub SenderName: ::windows_sys::core::PCWSTR,
    pub SenderCompany: ::windows_sys::core::PCWSTR,
    pub SenderDept: ::windows_sys::core::PCWSTR,
    pub BillingCode: ::windows_sys::core::PCWSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: ::windows_sys::core::PCWSTR,
    pub DocumentName: ::windows_sys::core::PCWSTR,
    pub CallHandle: u32,
    pub Reserved: [usize; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_JOB_PARAMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_JOB_PARAMW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_JOB_QUERY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_JOB_STATUS_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsPENDING: FAX_JOB_STATUS_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsINPROGRESS: FAX_JOB_STATUS_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsFAILED: FAX_JOB_STATUS_ENUM = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsPAUSED: FAX_JOB_STATUS_ENUM = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsNOLINE: FAX_JOB_STATUS_ENUM = 32i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsRETRYING: FAX_JOB_STATUS_ENUM = 64i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsRETRIES_EXCEEDED: FAX_JOB_STATUS_ENUM = 128i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsCOMPLETED: FAX_JOB_STATUS_ENUM = 256i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsCANCELED: FAX_JOB_STATUS_ENUM = 512i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsCANCELING: FAX_JOB_STATUS_ENUM = 1024i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsROUTING: FAX_JOB_STATUS_ENUM = 2048i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_JOB_SUBMIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_JOB_TYPE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjtSEND: FAX_JOB_TYPE_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjtRECEIVE: FAX_JOB_TYPE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjtROUTING: FAX_JOB_TYPE_ENUM = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_LOG_CATEGORYA {
    pub Name: ::windows_sys::core::PCSTR,
    pub Category: u32,
    pub Level: u32,
}
impl ::core::marker::Copy for FAX_LOG_CATEGORYA {}
impl ::core::clone::Clone for FAX_LOG_CATEGORYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_LOG_CATEGORYW {
    pub Name: ::windows_sys::core::PCWSTR,
    pub Category: u32,
    pub Level: u32,
}
impl ::core::marker::Copy for FAX_LOG_CATEGORYW {}
impl ::core::clone::Clone for FAX_LOG_CATEGORYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_LOG_LEVEL_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fllNONE: FAX_LOG_LEVEL_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fllMIN: FAX_LOG_LEVEL_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fllMED: FAX_LOG_LEVEL_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fllMAX: FAX_LOG_LEVEL_ENUM = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_PORT_INFOA {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub State: u32,
    pub Flags: u32,
    pub Rings: u32,
    pub Priority: u32,
    pub DeviceName: ::windows_sys::core::PCSTR,
    pub Tsid: ::windows_sys::core::PCSTR,
    pub Csid: ::windows_sys::core::PCSTR,
}
impl ::core::marker::Copy for FAX_PORT_INFOA {}
impl ::core::clone::Clone for FAX_PORT_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_PORT_INFOW {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub State: u32,
    pub Flags: u32,
    pub Rings: u32,
    pub Priority: u32,
    pub DeviceName: ::windows_sys::core::PCWSTR,
    pub Tsid: ::windows_sys::core::PCWSTR,
    pub Csid: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for FAX_PORT_INFOW {}
impl ::core::clone::Clone for FAX_PORT_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_PORT_QUERY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_PORT_SET: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_PRINT_INFOA {
    pub SizeOfStruct: u32,
    pub DocName: ::windows_sys::core::PCSTR,
    pub RecipientName: ::windows_sys::core::PCSTR,
    pub RecipientNumber: ::windows_sys::core::PCSTR,
    pub SenderName: ::windows_sys::core::PCSTR,
    pub SenderCompany: ::windows_sys::core::PCSTR,
    pub SenderDept: ::windows_sys::core::PCSTR,
    pub SenderBillingCode: ::windows_sys::core::PCSTR,
    pub Reserved: ::windows_sys::core::PCSTR,
    pub DrEmailAddress: ::windows_sys::core::PCSTR,
    pub OutputFileName: ::windows_sys::core::PCSTR,
}
impl ::core::marker::Copy for FAX_PRINT_INFOA {}
impl ::core::clone::Clone for FAX_PRINT_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_PRINT_INFOW {
    pub SizeOfStruct: u32,
    pub DocName: ::windows_sys::core::PCWSTR,
    pub RecipientName: ::windows_sys::core::PCWSTR,
    pub RecipientNumber: ::windows_sys::core::PCWSTR,
    pub SenderName: ::windows_sys::core::PCWSTR,
    pub SenderCompany: ::windows_sys::core::PCWSTR,
    pub SenderDept: ::windows_sys::core::PCWSTR,
    pub SenderBillingCode: ::windows_sys::core::PCWSTR,
    pub Reserved: ::windows_sys::core::PCWSTR,
    pub DrEmailAddress: ::windows_sys::core::PCWSTR,
    pub OutputFileName: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for FAX_PRINT_INFOW {}
impl ::core::clone::Clone for FAX_PRINT_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_PRIORITY_TYPE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fptLOW: FAX_PRIORITY_TYPE_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fptNORMAL: FAX_PRIORITY_TYPE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fptHIGH: FAX_PRIORITY_TYPE_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_PROVIDER_STATUS_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsSUCCESS: FAX_PROVIDER_STATUS_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsSERVER_ERROR: FAX_PROVIDER_STATUS_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsBAD_GUID: FAX_PROVIDER_STATUS_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsBAD_VERSION: FAX_PROVIDER_STATUS_ENUM = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsCANT_LOAD: FAX_PROVIDER_STATUS_ENUM = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsCANT_LINK: FAX_PROVIDER_STATUS_ENUM = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsCANT_INIT: FAX_PROVIDER_STATUS_ENUM = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_RECEIPT_TYPE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frtNONE: FAX_RECEIPT_TYPE_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frtMAIL: FAX_RECEIPT_TYPE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frtMSGBOX: FAX_RECEIPT_TYPE_ENUM = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_RECEIVE {
    pub SizeOfStruct: u32,
    pub FileName: ::windows_sys::core::PWSTR,
    pub ReceiverName: ::windows_sys::core::PWSTR,
    pub ReceiverNumber: ::windows_sys::core::PWSTR,
    pub Reserved: [u32; 4],
}
impl ::core::marker::Copy for FAX_RECEIVE {}
impl ::core::clone::Clone for FAX_RECEIVE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_ROUTE {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub ElapsedTime: u64,
    pub ReceiveTime: u64,
    pub PageCount: u32,
    pub Csid: ::windows_sys::core::PCWSTR,
    pub Tsid: ::windows_sys::core::PCWSTR,
    pub CallerId: ::windows_sys::core::PCWSTR,
    pub RoutingInfo: ::windows_sys::core::PCWSTR,
    pub ReceiverName: ::windows_sys::core::PCWSTR,
    pub ReceiverNumber: ::windows_sys::core::PCWSTR,
    pub DeviceName: ::windows_sys::core::PCWSTR,
    pub DeviceId: u32,
    pub RoutingInfoData: *mut u8,
    pub RoutingInfoDataSize: u32,
}
impl ::core::marker::Copy for FAX_ROUTE {}
impl ::core::clone::Clone for FAX_ROUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_ROUTE_CALLBACKROUTINES {
    pub SizeOfStruct: u32,
    pub FaxRouteAddFile: PFAXROUTEADDFILE,
    pub FaxRouteDeleteFile: PFAXROUTEDELETEFILE,
    pub FaxRouteGetFile: PFAXROUTEGETFILE,
    pub FaxRouteEnumFiles: PFAXROUTEENUMFILES,
    pub FaxRouteModifyRoutingData: PFAXROUTEMODIFYROUTINGDATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_ROUTE_CALLBACKROUTINES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_ROUTE_CALLBACKROUTINES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_ROUTING_METHODA {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub Enabled: super::super::Foundation::BOOL,
    pub DeviceName: ::windows_sys::core::PCSTR,
    pub Guid: ::windows_sys::core::PCSTR,
    pub FriendlyName: ::windows_sys::core::PCSTR,
    pub FunctionName: ::windows_sys::core::PCSTR,
    pub ExtensionImageName: ::windows_sys::core::PCSTR,
    pub ExtensionFriendlyName: ::windows_sys::core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_ROUTING_METHODA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_ROUTING_METHODA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_ROUTING_METHODW {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub Enabled: super::super::Foundation::BOOL,
    pub DeviceName: ::windows_sys::core::PCWSTR,
    pub Guid: ::windows_sys::core::PCWSTR,
    pub FriendlyName: ::windows_sys::core::PCWSTR,
    pub FunctionName: ::windows_sys::core::PCWSTR,
    pub ExtensionImageName: ::windows_sys::core::PCWSTR,
    pub ExtensionFriendlyName: ::windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_ROUTING_METHODW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_ROUTING_METHODW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_ROUTING_RULE_CODE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frrcANY_CODE: FAX_ROUTING_RULE_CODE_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_RULE_STATUS_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsVALID: FAX_RULE_STATUS_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsEMPTY_GROUP: FAX_RULE_STATUS_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsALL_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsSOME_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsBAD_DEVICE: FAX_RULE_STATUS_ENUM = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_SCHEDULE_TYPE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fstNOW: FAX_SCHEDULE_TYPE_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fstSPECIFIC_TIME: FAX_SCHEDULE_TYPE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fstDISCOUNT_PERIOD: FAX_SCHEDULE_TYPE_ENUM = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_SEND {
    pub SizeOfStruct: u32,
    pub FileName: ::windows_sys::core::PWSTR,
    pub CallerName: ::windows_sys::core::PWSTR,
    pub CallerNumber: ::windows_sys::core::PWSTR,
    pub ReceiverName: ::windows_sys::core::PWSTR,
    pub ReceiverNumber: ::windows_sys::core::PWSTR,
    pub Branding: super::super::Foundation::BOOL,
    pub CallHandle: u32,
    pub Reserved: [u32; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_SEND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_SEND {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_SERVER_APIVERSION_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsAPI_VERSION_0: FAX_SERVER_APIVERSION_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsAPI_VERSION_1: FAX_SERVER_APIVERSION_ENUM = 65536i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsAPI_VERSION_2: FAX_SERVER_APIVERSION_ENUM = 131072i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsAPI_VERSION_3: FAX_SERVER_APIVERSION_ENUM = 196608i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_SERVER_EVENTS_TYPE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetNONE: FAX_SERVER_EVENTS_TYPE_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetIN_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetOUT_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetCONFIG: FAX_SERVER_EVENTS_TYPE_ENUM = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetACTIVITY: FAX_SERVER_EVENTS_TYPE_ENUM = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetQUEUE_STATE: FAX_SERVER_EVENTS_TYPE_ENUM = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetIN_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = 32i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetOUT_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = 64i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetFXSSVC_ENDED: FAX_SERVER_EVENTS_TYPE_ENUM = 128i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetDEVICE_STATUS: FAX_SERVER_EVENTS_TYPE_ENUM = 256i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetINCOMING_CALL: FAX_SERVER_EVENTS_TYPE_ENUM = 512i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type FAX_SMTP_AUTHENTICATION_TYPE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsatANONYMOUS: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsatBASIC: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsatNTLM: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_TIME {
    pub Hour: u16,
    pub Minute: u16,
}
impl ::core::marker::Copy for FAX_TIME {}
impl ::core::clone::Clone for FAX_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_ABORTING: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_ANSWERED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_BAD_ADDRESS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_BUSY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_CALL_BLACKLISTED: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_CALL_DELAYED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_COMPLETED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_DELETED: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_DIALING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_DISCONNECTED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_FATAL_ERROR: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_FAXSVC_ENDED: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_FAXSVC_STARTED: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_HANDLED: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_IDLE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_INITIALIZING: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_JOB_QUEUED: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_LINE_UNAVAILABLE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_MODEM_POWERED_OFF: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_MODEM_POWERED_ON: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_NEVENTS: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_NOT_FAX_CALL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_NO_ANSWER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_NO_DIAL_TONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_RECEIVING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_RINGING: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_ROUTING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_SENDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPF_RECEIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPF_SEND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPF_VIRTUAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_ABORTING: u32 = 538968064u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_ANSWERED: u32 = 545259520u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_AVAILABLE: u32 = 537919488u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_BAD_ADDRESS: u32 = 536871168u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_BUSY: u32 = 536870976u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_CALL_BLACKLISTED: u32 = 536887296u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_CALL_DELAYED: u32 = 536879104u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_COMPLETED: u32 = 536870920u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_DIALING: u32 = 536870913u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_DISCONNECTED: u32 = 536871936u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_FATAL_ERROR: u32 = 536872960u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_HANDLED: u32 = 536870928u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_INITIALIZING: u32 = 536903680u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_NOT_FAX_CALL: u32 = 536875008u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_NO_ANSWER: u32 = 536871040u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_NO_DIAL_TONE: u32 = 536871424u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_OFFLINE: u32 = 536936448u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_RECEIVING: u32 = 536870916u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_RINGING: u32 = 537001984u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_ROUTING: u32 = 541065216u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_SENDING: u32 = 536870914u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_UNAVAILABLE: u32 = 536870944u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_ANSWERED: u32 = 545259520u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_BAD_ADDRESS: u32 = 536871168u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_BUSY: u32 = 536870976u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_CALL_BLACKLISTED: u32 = 536887296u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_CALL_DELAYED: u32 = 536879104u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_COMPLETED: u32 = 536870920u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_DIALING: u32 = 536870913u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_DISCONNECTED: u32 = 536871936u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_FATAL_ERROR: u32 = 536872960u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_HANDLED: u32 = 536870928u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_INITIALIZING: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_LINE_UNAVAILABLE: u32 = 536870944u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_NOT_FAX_CALL: u32 = 536875008u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_NO_ANSWER: u32 = 536871040u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_NO_DIAL_TONE: u32 = 536871424u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_RECEIVING: u32 = 536870916u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_TRANSMITTING: u32 = 536870914u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_USER_ABORT: u32 = 538968064u32;
pub const FaxAccount: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2816500863, data2: 17700, data3: 17508, data4: [165, 109, 185, 254, 102, 111, 113, 94] };
pub const FaxAccountFolders: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2235141961, data2: 49204, data3: 19007, data4: [130, 28, 219, 125, 104, 94, 129, 41] };
pub const FaxAccountIncomingArchive: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 347291061, data2: 19520, data3: 20175, data4: [158, 248, 163, 96, 203, 232, 9, 237] };
pub const FaxAccountIncomingQueue: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2614059156, data2: 46298, data3: 17908, data4: [184, 214, 221, 235, 33, 134, 101, 44] };
pub const FaxAccountOutgoingArchive: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2233367285, data2: 17210, data3: 18233, data4: [162, 223, 173, 36, 92, 44, 185, 142] };
pub const FaxAccountOutgoingQueue: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4276940539, data2: 49481, data3: 18618, data4: [186, 184, 183, 145, 225, 1, 246, 47] };
pub const FaxAccountSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4223810635, data2: 31200, data3: 17041, data4: [188, 86, 193, 46, 37, 59, 191, 58] };
pub const FaxAccounts: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3659502762, data2: 60972, data3: 18368, data4: [143, 79, 42, 33, 112, 117, 183, 110] };
pub const FaxActivity: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3488570638, data2: 59469, data3: 17966, data4: [170, 187, 135, 211, 30, 176, 79, 239] };
pub const FaxActivityLogging: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4037028174, data2: 15293, data3: 18616, data4: [143, 19, 140, 89, 26, 85, 189, 188] };
pub const FaxConfiguration: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1482109551, data2: 59315, data3: 16807, data4: [156, 25, 169, 27, 70, 62, 45, 86] };
pub const FaxDevice: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1508091314, data2: 54902, data3: 18507, data4: [166, 222, 114, 11, 250, 137, 181, 175] };
pub const FaxDeviceIds: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3452254698, data2: 29303, data3: 17934, data4: [141, 224, 72, 160, 165, 118, 13, 31] };
pub const FaxDeviceProvider: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 399448739, data2: 62955, data3: 18506, data4: [156, 154, 68, 64, 165, 186, 171, 252] };
pub const FaxDeviceProviders: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3952076648, data2: 34650, data3: 20319, data4: [130, 197, 3, 242, 58, 172, 27, 215] };
pub const FaxDevices: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1435099790, data2: 9163, data3: 18713, data4: [136, 8, 230, 16, 24, 70, 232, 13] };
pub const FaxDocument: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 255827857, data2: 51256, data3: 16734, data4: [164, 243, 62, 130, 140, 164, 69, 224] };
pub const FaxEventLogging: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2793736496, data2: 41206, data3: 19055, data4: [149, 183, 219, 46, 191, 61, 2, 227] };
pub const FaxFolders: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3276935639, data2: 22390, data3: 18635, data4: [175, 68, 195, 27, 227, 178, 207, 229] };
pub const FaxInboundRouting: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3892463853, data2: 44389, data3: 16920, data4: [129, 8, 153, 25, 36, 212, 231, 237] };
pub const FaxInboundRoutingExtension: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 494795601, data2: 29191, data3: 17462, data4: [160, 217, 36, 227, 46, 229, 105, 136] };
pub const FaxInboundRoutingExtensions: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 412764397, data2: 25148, data3: 19469, data4: [128, 242, 214, 108, 123, 158, 254, 194] };
pub const FaxInboundRoutingMethod: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1268766556, data2: 404, data3: 19314, data4: [156, 229, 2, 168, 32, 90, 199, 212] };
pub const FaxInboundRoutingMethods: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 637319018, data2: 46928, data3: 19330, data4: [146, 102, 251, 187, 174, 137, 34, 186] };
pub const FaxIncomingArchive: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2217133418, data2: 13729, data3: 19567, data4: [175, 147, 252, 149, 36, 34, 226, 194] };
pub const FaxIncomingJob: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3295875564, data2: 44594, data3: 16824, data4: [174, 75, 62, 174, 6, 41, 208, 201] };
pub const FaxIncomingJobs: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2713422403, data2: 34918, data3: 20407, data4: [161, 93, 98, 102, 200, 117, 165, 204] };
pub const FaxIncomingMessage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 422771959, data2: 40259, data3: 19802, data4: [137, 255, 3, 134, 27, 50, 23, 54] };
pub const FaxIncomingMessageIterator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1619583448, data2: 16328, data3: 17858, data4: [135, 177, 144, 154, 41, 96, 126, 169] };
pub const FaxIncomingQueue: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1762858775, data2: 62449, data3: 16611, data4: [128, 157, 166, 203, 247, 189, 133, 229] };
pub const FaxJobStatus: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2079466228, data2: 48781, data3: 17455, data4: [132, 29, 97, 50, 116, 36, 35, 187] };
pub const FaxLoggingOptions: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 469364390, data2: 60640, data3: 18309, data4: [161, 139, 222, 86, 233, 238, 249, 106] };
pub const FaxOutboundRouting: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3357227102, data2: 47209, data3: 19197, data4: [134, 192, 97, 100, 152, 237, 155, 226] };
pub const FaxOutboundRoutingGroup: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 34862048, data2: 26513, data3: 19831, data4: [162, 113, 4, 210, 53, 124, 80, 214] };
pub const FaxOutboundRoutingGroups: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3435045285, data2: 58036, data3: 19287, data4: [148, 33, 176, 75, 98, 137, 70, 75] };
pub const FaxOutboundRoutingRule: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1699344063, data2: 2257, data3: 18266, data4: [130, 139, 59, 241, 5, 149, 47, 160] };
pub const FaxOutboundRoutingRules: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3548757706, data2: 58916, data3: 17523, data4: [191, 170, 159, 64, 0, 131, 31, 84] };
pub const FaxOutgoingArchive: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1136821251, data2: 57423, data3: 18253, data4: [153, 12, 185, 70, 105, 20, 143, 89] };
pub const FaxOutgoingJob: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1908097692, data2: 3833, data3: 18709, data4: [190, 197, 165, 216, 151, 163, 233, 36] };
pub const FaxOutgoingJobs: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2462001772, data2: 14270, data3: 17402, data4: [163, 125, 203, 14, 95, 117, 59, 53] };
pub const FaxOutgoingMessage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2444534648, data2: 19160, data3: 19183, data4: [164, 220, 151, 217, 110, 147, 154, 58] };
pub const FaxOutgoingMessageIterator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2318542032, data2: 54027, data3: 18910, data4: [152, 19, 203, 56, 87, 144, 251, 187] };
pub const FaxOutgoingQueue: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1948325534, data2: 35907, data3: 19213, data4: [187, 22, 100, 92, 143, 164, 3, 87] };
pub const FaxReceiptOptions: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1770145915, data2: 8827, data3: 19606, data4: [166, 28, 36, 131, 72, 176, 90, 182] };
pub const FaxRecipient: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1623143169, data2: 32248, data3: 19416, data4: [145, 72, 123, 88, 1, 249, 239, 223] };
pub const FaxRecipients: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3936083795, data2: 4265, data3: 19791, data4: [160, 103, 99, 200, 248, 79, 1, 176] };
pub const FaxSecurity: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 281337310, data2: 44016, data3: 17375, data4: [150, 79, 127, 58, 194, 26, 76, 123] };
pub const FaxSecurity2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935413832, data2: 60553, data3: 19504, data4: [161, 39, 101, 110, 146, 227, 196, 234] };
pub const FaxSender: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 643663056, data2: 6224, data3: 17248, data4: [183, 200, 117, 139, 187, 95, 11, 150] };
pub const FaxServer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3450383536, data2: 36085, data3: 20332, data4: [155, 162, 89, 49, 212, 12, 140, 174] };
pub const GUID_DeviceArrivedLaunch: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1947049702, data2: 28913, data3: 4561, data4: [173, 16, 0, 160, 36, 56, 173, 72] };
pub const GUID_STIUserDefined1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3222189973, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_STIUserDefined2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3346721221, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_STIUserDefined3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3346721222, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_ScanFaxImage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3222189971, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_ScanImage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2797971221, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_ScanPrintImage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3024221221, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxAccount {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub AccountName: unsafe extern "system" fn(this: *mut *mut Self, pbstraccountname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AccountName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Folders: unsafe extern "system" fn(this: *mut *mut Self, ppfolders: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Folders: usize,
    pub ListenToAccountEvents: unsafe extern "system" fn(this: *mut *mut Self, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub RegisteredEvents: unsafe extern "system" fn(this: *mut *mut Self, pregisteredevents: *mut FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxAccount {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1750293299, data2: 24004, data3: 16518, data4: [190, 38, 183, 111, 155, 113, 16, 6] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxAccountFolders {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub OutgoingQueue: unsafe extern "system" fn(this: *mut *mut Self, pfaxoutgoingqueue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutgoingQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncomingQueue: unsafe extern "system" fn(this: *mut *mut Self, pfaxincomingqueue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncomingQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncomingArchive: unsafe extern "system" fn(this: *mut *mut Self, pfaxincomingarchive: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncomingArchive: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OutgoingArchive: unsafe extern "system" fn(this: *mut *mut Self, pfaxoutgoingarchive: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutgoingArchive: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxAccountFolders {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1684273309, data2: 9176, data3: 18089, data4: [143, 134, 196, 123, 119, 202, 121, 38] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxAccountIncomingArchive {
    pub base__: super::super::System::Com::IDispatch,
    pub SizeLow: unsafe extern "system" fn(this: *mut *mut Self, plsizelow: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(this: *mut *mut Self, plsizehigh: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessages: unsafe extern "system" fn(this: *mut *mut Self, lprefetchsize: i32, pfaxincomingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessages: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetMessage: unsafe extern "system" fn(this: *mut *mut Self, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetMessage: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxAccountIncomingArchive {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2829432559, data2: 57558, data3: 19182, data4: [149, 92, 145, 98, 91, 236, 157, 180] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxAccountIncomingQueue {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJobs: unsafe extern "system" fn(this: *mut *mut Self, pfaxincomingjobs: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJobs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetJob: unsafe extern "system" fn(this: *mut *mut Self, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingjob: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetJob: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxAccountIncomingQueue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3709087122, data2: 390, data3: 19093, data4: [160, 144, 203, 195, 234, 219, 166, 180] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxAccountNotify {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxAccountNotify {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 190733265, data2: 47273, data3: 18336, data4: [163, 35, 239, 74, 41, 59, 160, 106] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxAccountOutgoingArchive {
    pub base__: super::super::System::Com::IDispatch,
    pub SizeLow: unsafe extern "system" fn(this: *mut *mut Self, plsizelow: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(this: *mut *mut Self, plsizehigh: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessages: unsafe extern "system" fn(this: *mut *mut Self, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessages: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetMessage: unsafe extern "system" fn(this: *mut *mut Self, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetMessage: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxAccountOutgoingArchive {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1415776109, data2: 60436, data3: 18719, data4: [146, 110, 179, 206, 218, 94, 86, 98] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxAccountOutgoingQueue {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJobs: unsafe extern "system" fn(this: *mut *mut Self, pfaxoutgoingjobs: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJobs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetJob: unsafe extern "system" fn(this: *mut *mut Self, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingjob: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetJob: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxAccountOutgoingQueue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 252978409, data2: 61997, data3: 17747, data4: [183, 165, 13, 36, 189, 13, 126, 70] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxAccountSet {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAccounts: unsafe extern "system" fn(this: *mut *mut Self, ppfaxaccounts: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAccounts: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetAccount: unsafe extern "system" fn(this: *mut *mut Self, bstraccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxaccount: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetAccount: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddAccount: unsafe extern "system" fn(this: *mut *mut Self, bstraccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxaccount: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddAccount: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveAccount: unsafe extern "system" fn(this: *mut *mut Self, bstraccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveAccount: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxAccountSet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1948842926, data2: 33822, data3: 18360, data4: [134, 244, 34, 136, 148, 109, 202, 27] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxAccounts {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxaccount: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxAccounts {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2481619298, data2: 35815, data3: 17105, data4: [174, 123, 236, 116, 226, 217, 137, 218] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxActivity {
    pub base__: super::super::System::Com::IDispatch,
    pub IncomingMessages: unsafe extern "system" fn(this: *mut *mut Self, plincomingmessages: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RoutingMessages: unsafe extern "system" fn(this: *mut *mut Self, plroutingmessages: *mut i32) -> ::windows_sys::core::HRESULT,
    pub OutgoingMessages: unsafe extern "system" fn(this: *mut *mut Self, ploutgoingmessages: *mut i32) -> ::windows_sys::core::HRESULT,
    pub QueuedMessages: unsafe extern "system" fn(this: *mut *mut Self, plqueuedmessages: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxActivity {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1259368343, data2: 15861, data3: 16626, data4: [188, 60, 68, 203, 129, 21, 235, 223] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxActivityLogging {
    pub base__: super::super::System::Com::IDispatch,
    pub LogIncoming: unsafe extern "system" fn(this: *mut *mut Self, pblogincoming: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetLogIncoming: unsafe extern "system" fn(this: *mut *mut Self, blogincoming: i16) -> ::windows_sys::core::HRESULT,
    pub LogOutgoing: unsafe extern "system" fn(this: *mut *mut Self, pblogoutgoing: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetLogOutgoing: unsafe extern "system" fn(this: *mut *mut Self, blogoutgoing: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DatabasePath: unsafe extern "system" fn(this: *mut *mut Self, pbstrdatabasepath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DatabasePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDatabasePath: unsafe extern "system" fn(this: *mut *mut Self, bstrdatabasepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDatabasePath: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxActivityLogging {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 506005387, data2: 23145, data3: 18811, data4: [149, 146, 73, 183, 231, 250, 221, 181] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxConfiguration {
    pub base__: super::super::System::Com::IDispatch,
    pub UseArchive: unsafe extern "system" fn(this: *mut *mut Self, pbusearchive: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUseArchive: unsafe extern "system" fn(this: *mut *mut Self, busearchive: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ArchiveLocation: unsafe extern "system" fn(this: *mut *mut Self, pbstrarchivelocation: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ArchiveLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetArchiveLocation: unsafe extern "system" fn(this: *mut *mut Self, bstrarchivelocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetArchiveLocation: usize,
    pub SizeQuotaWarning: unsafe extern "system" fn(this: *mut *mut Self, pbsizequotawarning: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSizeQuotaWarning: unsafe extern "system" fn(this: *mut *mut Self, bsizequotawarning: i16) -> ::windows_sys::core::HRESULT,
    pub HighQuotaWaterMark: unsafe extern "system" fn(this: *mut *mut Self, plhighquotawatermark: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetHighQuotaWaterMark: unsafe extern "system" fn(this: *mut *mut Self, lhighquotawatermark: i32) -> ::windows_sys::core::HRESULT,
    pub LowQuotaWaterMark: unsafe extern "system" fn(this: *mut *mut Self, pllowquotawatermark: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLowQuotaWaterMark: unsafe extern "system" fn(this: *mut *mut Self, llowquotawatermark: i32) -> ::windows_sys::core::HRESULT,
    pub ArchiveAgeLimit: unsafe extern "system" fn(this: *mut *mut Self, plarchiveagelimit: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetArchiveAgeLimit: unsafe extern "system" fn(this: *mut *mut Self, larchiveagelimit: i32) -> ::windows_sys::core::HRESULT,
    pub ArchiveSizeLow: unsafe extern "system" fn(this: *mut *mut Self, plsizelow: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ArchiveSizeHigh: unsafe extern "system" fn(this: *mut *mut Self, plsizehigh: *mut i32) -> ::windows_sys::core::HRESULT,
    pub OutgoingQueueBlocked: unsafe extern "system" fn(this: *mut *mut Self, pboutgoingblocked: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetOutgoingQueueBlocked: unsafe extern "system" fn(this: *mut *mut Self, boutgoingblocked: i16) -> ::windows_sys::core::HRESULT,
    pub OutgoingQueuePaused: unsafe extern "system" fn(this: *mut *mut Self, pboutgoingpaused: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetOutgoingQueuePaused: unsafe extern "system" fn(this: *mut *mut Self, boutgoingpaused: i16) -> ::windows_sys::core::HRESULT,
    pub AllowPersonalCoverPages: unsafe extern "system" fn(this: *mut *mut Self, pballowpersonalcoverpages: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowPersonalCoverPages: unsafe extern "system" fn(this: *mut *mut Self, ballowpersonalcoverpages: i16) -> ::windows_sys::core::HRESULT,
    pub UseDeviceTSID: unsafe extern "system" fn(this: *mut *mut Self, pbusedevicetsid: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUseDeviceTSID: unsafe extern "system" fn(this: *mut *mut Self, busedevicetsid: i16) -> ::windows_sys::core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut *mut Self, plretries: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRetries: unsafe extern "system" fn(this: *mut *mut Self, lretries: i32) -> ::windows_sys::core::HRESULT,
    pub RetryDelay: unsafe extern "system" fn(this: *mut *mut Self, plretrydelay: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRetryDelay: unsafe extern "system" fn(this: *mut *mut Self, lretrydelay: i32) -> ::windows_sys::core::HRESULT,
    pub DiscountRateStart: unsafe extern "system" fn(this: *mut *mut Self, pdatediscountratestart: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDiscountRateStart: unsafe extern "system" fn(this: *mut *mut Self, datediscountratestart: f64) -> ::windows_sys::core::HRESULT,
    pub DiscountRateEnd: unsafe extern "system" fn(this: *mut *mut Self, pdatediscountrateend: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDiscountRateEnd: unsafe extern "system" fn(this: *mut *mut Self, datediscountrateend: f64) -> ::windows_sys::core::HRESULT,
    pub OutgoingQueueAgeLimit: unsafe extern "system" fn(this: *mut *mut Self, ploutgoingqueueagelimit: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetOutgoingQueueAgeLimit: unsafe extern "system" fn(this: *mut *mut Self, loutgoingqueueagelimit: i32) -> ::windows_sys::core::HRESULT,
    pub Branding: unsafe extern "system" fn(this: *mut *mut Self, pbbranding: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetBranding: unsafe extern "system" fn(this: *mut *mut Self, bbranding: i16) -> ::windows_sys::core::HRESULT,
    pub IncomingQueueBlocked: unsafe extern "system" fn(this: *mut *mut Self, pbincomingblocked: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIncomingQueueBlocked: unsafe extern "system" fn(this: *mut *mut Self, bincomingblocked: i16) -> ::windows_sys::core::HRESULT,
    pub AutoCreateAccountOnConnect: unsafe extern "system" fn(this: *mut *mut Self, pbautocreateaccountonconnect: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAutoCreateAccountOnConnect: unsafe extern "system" fn(this: *mut *mut Self, bautocreateaccountonconnect: i16) -> ::windows_sys::core::HRESULT,
    pub IncomingFaxesArePublic: unsafe extern "system" fn(this: *mut *mut Self, pbincomingfaxesarepublic: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIncomingFaxesArePublic: unsafe extern "system" fn(this: *mut *mut Self, bincomingfaxesarepublic: i16) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 284479735, data2: 2452, data3: 17731, data4: [171, 110, 80, 105, 73, 18, 140, 64] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxDevice {
    pub base__: super::super::System::Com::IDispatch,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, plid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceName: unsafe extern "system" fn(this: *mut *mut Self, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProviderUniqueName: unsafe extern "system" fn(this: *mut *mut Self, pbstrprovideruniquename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProviderUniqueName: usize,
    pub PoweredOff: unsafe extern "system" fn(this: *mut *mut Self, pbpoweredoff: *mut i16) -> ::windows_sys::core::HRESULT,
    pub ReceivingNow: unsafe extern "system" fn(this: *mut *mut Self, pbreceivingnow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SendingNow: unsafe extern "system" fn(this: *mut *mut Self, pbsendingnow: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub UsedRoutingMethods: unsafe extern "system" fn(this: *mut *mut Self, pvusedroutingmethods: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    UsedRoutingMethods: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    pub SendEnabled: unsafe extern "system" fn(this: *mut *mut Self, pbsendenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSendEnabled: unsafe extern "system" fn(this: *mut *mut Self, bsendenabled: i16) -> ::windows_sys::core::HRESULT,
    pub ReceiveMode: unsafe extern "system" fn(this: *mut *mut Self, preceivemode: *mut FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows_sys::core::HRESULT,
    pub SetReceiveMode: unsafe extern "system" fn(this: *mut *mut Self, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows_sys::core::HRESULT,
    pub RingsBeforeAnswer: unsafe extern "system" fn(this: *mut *mut Self, plringsbeforeanswer: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRingsBeforeAnswer: unsafe extern "system" fn(this: *mut *mut Self, lringsbeforeanswer: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCSID: unsafe extern "system" fn(this: *mut *mut Self, bstrcsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTSID: unsafe extern "system" fn(this: *mut *mut Self, bstrtsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTSID: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetExtensionProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvproperty: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetExtensionProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetExtensionProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetExtensionProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UseRoutingMethod: unsafe extern "system" fn(this: *mut *mut Self, bstrmethodguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, buse: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseRoutingMethod: usize,
    pub RingingNow: unsafe extern "system" fn(this: *mut *mut Self, pbringingnow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub AnswerCall: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1227910233, data2: 46382, data3: 18535, data4: [157, 244, 202, 88, 65, 201, 86, 208] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxDeviceIds {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pldeviceid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, ldeviceid: i32) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32) -> ::windows_sys::core::HRESULT,
    pub SetOrder: unsafe extern "system" fn(this: *mut *mut Self, ldeviceid: i32, lneworder: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxDeviceIds {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 789545279, data2: 19689, data3: 17470, data4: [140, 161, 115, 140, 250, 238, 225, 73] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxDeviceProvider {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pbstrfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImageName: unsafe extern "system" fn(this: *mut *mut Self, pbstrimagename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImageName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UniqueName: unsafe extern "system" fn(this: *mut *mut Self, pbstruniquename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UniqueName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TapiProviderName: unsafe extern "system" fn(this: *mut *mut Self, pbstrtapiprovidername: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TapiProviderName: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut *mut Self, plmajorversion: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut *mut Self, plminorversion: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MajorBuild: unsafe extern "system" fn(this: *mut *mut Self, plmajorbuild: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinorBuild: unsafe extern "system" fn(this: *mut *mut Self, plminorbuild: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Debug: unsafe extern "system" fn(this: *mut *mut Self, pbdebug: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows_sys::core::HRESULT,
    pub InitErrorCode: unsafe extern "system" fn(this: *mut *mut Self, pliniterrorcode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeviceIds: unsafe extern "system" fn(this: *mut *mut Self, pvdeviceids: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeviceIds: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxDeviceProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 688827491, data2: 33772, data3: 17564, data4: [132, 23, 241, 72, 223, 140, 104, 42] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxDeviceProviders {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxdeviceprovider: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxDeviceProviders {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2679598946, data2: 19582, data3: 17317, data4: [182, 253, 80, 40, 147, 247, 225, 62] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxDevices {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemById: unsafe extern "system" fn(this: *mut *mut Self, lid: i32, ppfaxdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemById: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxDevices {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2655418430, data2: 62287, data3: 18478, data4: [163, 96, 4, 22, 190, 203, 189, 150] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxDocument {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, pbstrbody: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Body: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBody: unsafe extern "system" fn(this: *mut *mut Self, bstrbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBody: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Sender: unsafe extern "system" fn(this: *mut *mut Self, ppfaxsender: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sender: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recipients: unsafe extern "system" fn(this: *mut *mut Self, ppfaxrecipients: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recipients: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CoverPage: unsafe extern "system" fn(this: *mut *mut Self, pbstrcoverpage: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CoverPage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCoverPage: unsafe extern "system" fn(this: *mut *mut Self, bstrcoverpage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCoverPage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubject: unsafe extern "system" fn(this: *mut *mut Self, bstrsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Note: unsafe extern "system" fn(this: *mut *mut Self, pbstrnote: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Note: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNote: unsafe extern "system" fn(this: *mut *mut Self, bstrnote: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNote: usize,
    pub ScheduleTime: unsafe extern "system" fn(this: *mut *mut Self, pdatescheduletime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetScheduleTime: unsafe extern "system" fn(this: *mut *mut Self, datescheduletime: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReceiptAddress: unsafe extern "system" fn(this: *mut *mut Self, pbstrreceiptaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReceiptAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReceiptAddress: unsafe extern "system" fn(this: *mut *mut Self, bstrreceiptaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReceiptAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DocumentName: unsafe extern "system" fn(this: *mut *mut Self, pbstrdocumentname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DocumentName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDocumentName: unsafe extern "system" fn(this: *mut *mut Self, bstrdocumentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDocumentName: usize,
    pub CallHandle: unsafe extern "system" fn(this: *mut *mut Self, plcallhandle: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCallHandle: unsafe extern "system" fn(this: *mut *mut Self, lcallhandle: i32) -> ::windows_sys::core::HRESULT,
    pub CoverPageType: unsafe extern "system" fn(this: *mut *mut Self, pcoverpagetype: *mut FAX_COVERPAGE_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub SetCoverPageType: unsafe extern "system" fn(this: *mut *mut Self, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub ScheduleType: unsafe extern "system" fn(this: *mut *mut Self, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub SetScheduleType: unsafe extern "system" fn(this: *mut *mut Self, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub ReceiptType: unsafe extern "system" fn(this: *mut *mut Self, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub SetReceiptType: unsafe extern "system" fn(this: *mut *mut Self, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub GroupBroadcastReceipts: unsafe extern "system" fn(this: *mut *mut Self, pbusegrouping: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetGroupBroadcastReceipts: unsafe extern "system" fn(this: *mut *mut Self, busegrouping: i16) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TapiConnection: unsafe extern "system" fn(this: *mut *mut Self, pptapiconnection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TapiConnection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_TapiConnection: unsafe extern "system" fn(this: *mut *mut Self, ptapiconnection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_TapiConnection: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut *mut Self, bstrfaxservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ConnectedSubmit: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ConnectedSubmit: usize,
    pub AttachFaxToReceipt: unsafe extern "system" fn(this: *mut *mut Self, pbattachfax: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAttachFaxToReceipt: unsafe extern "system" fn(this: *mut *mut Self, battachfax: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxDocument {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2986844742, data2: 2531, data3: 19022, data4: [167, 220, 254, 163, 29, 41, 69, 143] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxDocument2 {
    pub base__: IFaxDocument,
    #[cfg(feature = "Win32_Foundation")]
    pub SubmissionId: unsafe extern "system" fn(this: *mut *mut Self, pbstrsubmissionid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubmissionId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Bodies: unsafe extern "system" fn(this: *mut *mut Self, pvbodies: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Bodies: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetBodies: unsafe extern "system" fn(this: *mut *mut Self, vbodies: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetBodies: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit2: unsafe extern "system" fn(this: *mut *mut Self, bstrfaxservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ConnectedSubmit2: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ConnectedSubmit2: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxDocument2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3778311777, data2: 63983, data3: 19821, data4: [180, 165, 192, 160, 104, 182, 92, 255] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxEventLogging {
    pub base__: super::super::System::Com::IDispatch,
    pub InitEventsLevel: unsafe extern "system" fn(this: *mut *mut Self, piniteventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows_sys::core::HRESULT,
    pub SetInitEventsLevel: unsafe extern "system" fn(this: *mut *mut Self, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_sys::core::HRESULT,
    pub InboundEventsLevel: unsafe extern "system" fn(this: *mut *mut Self, pinboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows_sys::core::HRESULT,
    pub SetInboundEventsLevel: unsafe extern "system" fn(this: *mut *mut Self, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_sys::core::HRESULT,
    pub OutboundEventsLevel: unsafe extern "system" fn(this: *mut *mut Self, poutboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows_sys::core::HRESULT,
    pub SetOutboundEventsLevel: unsafe extern "system" fn(this: *mut *mut Self, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_sys::core::HRESULT,
    pub GeneralEventsLevel: unsafe extern "system" fn(this: *mut *mut Self, pgeneraleventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows_sys::core::HRESULT,
    pub SetGeneralEventsLevel: unsafe extern "system" fn(this: *mut *mut Self, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxEventLogging {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 142661989, data2: 8424, data3: 17124, data4: [142, 23, 148, 79, 25, 44, 170, 212] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxFolders {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub OutgoingQueue: unsafe extern "system" fn(this: *mut *mut Self, pfaxoutgoingqueue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutgoingQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncomingQueue: unsafe extern "system" fn(this: *mut *mut Self, pfaxincomingqueue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncomingQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncomingArchive: unsafe extern "system" fn(this: *mut *mut Self, pfaxincomingarchive: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncomingArchive: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OutgoingArchive: unsafe extern "system" fn(this: *mut *mut Self, pfaxoutgoingarchive: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutgoingArchive: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxFolders {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3705909928, data2: 42923, data3: 17084, data4: [157, 10, 49, 73, 69, 114, 97, 160] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxInboundRouting {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub GetExtensions: unsafe extern "system" fn(this: *mut *mut Self, pfaxinboundroutingextensions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMethods: unsafe extern "system" fn(this: *mut *mut Self, pfaxinboundroutingmethods: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMethods: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxInboundRouting {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2169029135, data2: 40274, data3: 17841, data4: [191, 150, 56, 252, 18, 113, 53, 39] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxInboundRoutingExtension {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pbstrfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImageName: unsafe extern "system" fn(this: *mut *mut Self, pbstrimagename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImageName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UniqueName: unsafe extern "system" fn(this: *mut *mut Self, pbstruniquename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UniqueName: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut *mut Self, plmajorversion: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut *mut Self, plminorversion: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MajorBuild: unsafe extern "system" fn(this: *mut *mut Self, plmajorbuild: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinorBuild: unsafe extern "system" fn(this: *mut *mut Self, plminorbuild: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Debug: unsafe extern "system" fn(this: *mut *mut Self, pbdebug: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows_sys::core::HRESULT,
    pub InitErrorCode: unsafe extern "system" fn(this: *mut *mut Self, pliniterrorcode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Methods: unsafe extern "system" fn(this: *mut *mut Self, pvmethods: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Methods: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxInboundRoutingExtension {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2287689224, data2: 49772, data3: 20217, data4: [175, 131, 81, 88, 10, 117, 11, 225] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxInboundRoutingExtensions {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxinboundroutingextension: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxInboundRoutingExtensions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 795645555, data2: 31526, data3: 17118, data4: [142, 176, 145, 93, 205, 42, 79, 76] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxInboundRoutingMethod {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GUID: unsafe extern "system" fn(this: *mut *mut Self, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GUID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FunctionName: unsafe extern "system" fn(this: *mut *mut Self, pbstrfunctionname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FunctionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtensionFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pbstrextensionfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtensionFriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtensionImageName: unsafe extern "system" fn(this: *mut *mut Self, pbstrextensionimagename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtensionImageName: usize,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, plpriority: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, lpriority: i32) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxInboundRoutingMethod {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1164968033, data2: 44445, data3: 18294, data4: [168, 196, 100, 6, 84, 146, 207, 75] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxInboundRoutingMethods {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxinboundroutingmethod: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxInboundRoutingMethods {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2017446416, data2: 35080, data3: 17523, data4: [157, 105, 246, 127, 190, 160, 198, 185] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxIncomingArchive {
    pub base__: super::super::System::Com::IDispatch,
    pub UseArchive: unsafe extern "system" fn(this: *mut *mut Self, pbusearchive: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUseArchive: unsafe extern "system" fn(this: *mut *mut Self, busearchive: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ArchiveFolder: unsafe extern "system" fn(this: *mut *mut Self, pbstrarchivefolder: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ArchiveFolder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetArchiveFolder: unsafe extern "system" fn(this: *mut *mut Self, bstrarchivefolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetArchiveFolder: usize,
    pub SizeQuotaWarning: unsafe extern "system" fn(this: *mut *mut Self, pbsizequotawarning: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSizeQuotaWarning: unsafe extern "system" fn(this: *mut *mut Self, bsizequotawarning: i16) -> ::windows_sys::core::HRESULT,
    pub HighQuotaWaterMark: unsafe extern "system" fn(this: *mut *mut Self, plhighquotawatermark: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetHighQuotaWaterMark: unsafe extern "system" fn(this: *mut *mut Self, lhighquotawatermark: i32) -> ::windows_sys::core::HRESULT,
    pub LowQuotaWaterMark: unsafe extern "system" fn(this: *mut *mut Self, pllowquotawatermark: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLowQuotaWaterMark: unsafe extern "system" fn(this: *mut *mut Self, llowquotawatermark: i32) -> ::windows_sys::core::HRESULT,
    pub AgeLimit: unsafe extern "system" fn(this: *mut *mut Self, plagelimit: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAgeLimit: unsafe extern "system" fn(this: *mut *mut Self, lagelimit: i32) -> ::windows_sys::core::HRESULT,
    pub SizeLow: unsafe extern "system" fn(this: *mut *mut Self, plsizelow: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(this: *mut *mut Self, plsizehigh: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessages: unsafe extern "system" fn(this: *mut *mut Self, lprefetchsize: i32, pfaxincomingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessages: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetMessage: unsafe extern "system" fn(this: *mut *mut Self, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetMessage: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxIncomingArchive {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1980116167, data2: 63252, data3: 20413, data4: [170, 6, 237, 110, 74, 75, 112, 243] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxIncomingJob {
    pub base__: super::super::System::Com::IDispatch,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, plsize: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    pub CurrentPage: unsafe extern "system" fn(this: *mut *mut Self, plcurrentpage: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, pldeviceid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows_sys::core::HRESULT,
    pub ExtendedStatusCode: unsafe extern "system" fn(this: *mut *mut Self, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut *mut Self, pbstrextendedstatus: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtendedStatus: usize,
    pub AvailableOperations: unsafe extern "system" fn(this: *mut *mut Self, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows_sys::core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut *mut Self, plretries: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut *mut Self, pdatetransmissionstart: *mut f64) -> ::windows_sys::core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut *mut Self, pdatetransmissionend: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CallerId: unsafe extern "system" fn(this: *mut *mut Self, pbstrcallerid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CallerId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RoutingInformation: unsafe extern "system" fn(this: *mut *mut Self, pbstrroutinginformation: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RoutingInformation: usize,
    pub JobType: unsafe extern "system" fn(this: *mut *mut Self, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTiff: unsafe extern "system" fn(this: *mut *mut Self, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTiff: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxIncomingJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 544549350, data2: 25930, data3: 18710, data4: [159, 136, 77, 35, 46, 232, 161, 7] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxIncomingJobs {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxincomingjob: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxIncomingJobs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 18810089, data2: 20438, data3: 19491, data4: [149, 19, 182, 182, 107, 178, 107, 233] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxIncomingMessage {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    pub Pages: unsafe extern "system" fn(this: *mut *mut Self, plpages: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, plsize: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceName: unsafe extern "system" fn(this: *mut *mut Self, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceName: usize,
    pub Retries: unsafe extern "system" fn(this: *mut *mut Self, plretries: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut *mut Self, pdatetransmissionstart: *mut f64) -> ::windows_sys::core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut *mut Self, pdatetransmissionend: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CallerId: unsafe extern "system" fn(this: *mut *mut Self, pbstrcallerid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CallerId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RoutingInformation: unsafe extern "system" fn(this: *mut *mut Self, pbstrroutinginformation: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RoutingInformation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTiff: unsafe extern "system" fn(this: *mut *mut Self, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTiff: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxIncomingMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2091616506, data2: 12025, data3: 18513, data4: [178, 243, 29, 20, 143, 237, 132, 71] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxIncomingMessage2 {
    pub base__: IFaxIncomingMessage,
    #[cfg(feature = "Win32_Foundation")]
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubject: unsafe extern "system" fn(this: *mut *mut Self, bstrsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SenderName: unsafe extern "system" fn(this: *mut *mut Self, pbstrsendername: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SenderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSenderName: unsafe extern "system" fn(this: *mut *mut Self, bstrsendername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSenderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SenderFaxNumber: unsafe extern "system" fn(this: *mut *mut Self, pbstrsenderfaxnumber: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SenderFaxNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSenderFaxNumber: unsafe extern "system" fn(this: *mut *mut Self, bstrsenderfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSenderFaxNumber: usize,
    pub HasCoverPage: unsafe extern "system" fn(this: *mut *mut Self, pbhascoverpage: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetHasCoverPage: unsafe extern "system" fn(this: *mut *mut Self, bhascoverpage: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Recipients: unsafe extern "system" fn(this: *mut *mut Self, pbstrrecipients: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Recipients: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRecipients: unsafe extern "system" fn(this: *mut *mut Self, bstrrecipients: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRecipients: usize,
    pub WasReAssigned: unsafe extern "system" fn(this: *mut *mut Self, pbwasreassigned: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, pbread: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetRead: unsafe extern "system" fn(this: *mut *mut Self, bread: i16) -> ::windows_sys::core::HRESULT,
    pub ReAssign: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxIncomingMessage2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4179657987, data2: 58044, data3: 18675, data4: [158, 192, 230, 35, 111, 155, 80, 154] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxIncomingMessageIterator {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, pfaxincomingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
    pub PrefetchSize: unsafe extern "system" fn(this: *mut *mut Self, plprefetchsize: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPrefetchSize: unsafe extern "system" fn(this: *mut *mut Self, lprefetchsize: i32) -> ::windows_sys::core::HRESULT,
    pub AtEOF: unsafe extern "system" fn(this: *mut *mut Self, pbeof: *mut i16) -> ::windows_sys::core::HRESULT,
    pub MoveFirst: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxIncomingMessageIterator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4252232900, data2: 28422, data3: 20306, data4: [130, 168, 247, 186, 6, 174, 49, 8] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxIncomingQueue {
    pub base__: super::super::System::Com::IDispatch,
    pub Blocked: unsafe extern "system" fn(this: *mut *mut Self, pbblocked: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetBlocked: unsafe extern "system" fn(this: *mut *mut Self, bblocked: i16) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJobs: unsafe extern "system" fn(this: *mut *mut Self, pfaxincomingjobs: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJobs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetJob: unsafe extern "system" fn(this: *mut *mut Self, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingjob: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetJob: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxIncomingQueue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2418959599, data2: 36824, data3: 19317, data4: [151, 37, 96, 20, 223, 22, 21, 69] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxJobStatus {
    pub base__: super::super::System::Com::IDispatch,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows_sys::core::HRESULT,
    pub Pages: unsafe extern "system" fn(this: *mut *mut Self, plpages: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, plsize: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CurrentPage: unsafe extern "system" fn(this: *mut *mut Self, plcurrentpage: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, pldeviceid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    pub ExtendedStatusCode: unsafe extern "system" fn(this: *mut *mut Self, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut *mut Self, pbstrextendedstatus: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtendedStatus: usize,
    pub AvailableOperations: unsafe extern "system" fn(this: *mut *mut Self, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows_sys::core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut *mut Self, plretries: *mut i32) -> ::windows_sys::core::HRESULT,
    pub JobType: unsafe extern "system" fn(this: *mut *mut Self, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub ScheduledTime: unsafe extern "system" fn(this: *mut *mut Self, pdatescheduledtime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut *mut Self, pdatetransmissionstart: *mut f64) -> ::windows_sys::core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut *mut Self, pdatetransmissionend: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CallerId: unsafe extern "system" fn(this: *mut *mut Self, pbstrcallerid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CallerId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RoutingInformation: unsafe extern "system" fn(this: *mut *mut Self, pbstrroutinginformation: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RoutingInformation: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxJobStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2340877445, data2: 64895, data3: 18468, data4: [136, 107, 64, 197, 202, 166, 23, 204] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxLoggingOptions {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub EventLogging: unsafe extern "system" fn(this: *mut *mut Self, pfaxeventlogging: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EventLogging: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActivityLogging: unsafe extern "system" fn(this: *mut *mut Self, pfaxactivitylogging: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActivityLogging: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxLoggingOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 887508921, data2: 27441, data3: 19762, data4: [139, 39, 210, 134, 192, 195, 54, 6] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxOutboundRouting {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub GetGroups: unsafe extern "system" fn(this: *mut *mut Self, pfaxoutboundroutinggroups: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRules: unsafe extern "system" fn(this: *mut *mut Self, pfaxoutboundroutingrules: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRules: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxOutboundRouting {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 635176356, data2: 39177, data3: 16829, data4: [169, 91, 126, 93, 29, 236, 29, 67] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxOutboundRoutingGroup {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut FAX_GROUP_STATUS_ENUM) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceIds: unsafe extern "system" fn(this: *mut *mut Self, pfaxdeviceids: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceIds: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxOutboundRoutingGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3395455393, data2: 32293, data3: 20359, data4: [154, 11, 147, 54, 87, 52, 150, 44] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxOutboundRoutingGroups {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxoutboundroutinggroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutboundroutinggroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxOutboundRoutingGroups {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 593280759, data2: 49886, data3: 19453, data4: [184, 218, 117, 9, 124, 130, 200, 127] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxOutboundRoutingRule {
    pub base__: super::super::System::Com::IDispatch,
    pub CountryCode: unsafe extern "system" fn(this: *mut *mut Self, plcountrycode: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AreaCode: unsafe extern "system" fn(this: *mut *mut Self, plareacode: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut FAX_RULE_STATUS_ENUM) -> ::windows_sys::core::HRESULT,
    pub UseDevice: unsafe extern "system" fn(this: *mut *mut Self, pbusedevice: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUseDevice: unsafe extern "system" fn(this: *mut *mut Self, busedevice: i16) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, pldeviceid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDeviceId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GroupName: unsafe extern "system" fn(this: *mut *mut Self, pbstrgroupname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GroupName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGroupName: unsafe extern "system" fn(this: *mut *mut Self, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGroupName: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxOutboundRoutingRule {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3791099349, data2: 1986, data3: 18079, data4: [176, 39, 172, 172, 194, 50, 25, 218] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxOutboundRoutingRules {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pfaxoutboundroutingrule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ItemByCountryAndArea: unsafe extern "system" fn(this: *mut *mut Self, lcountrycode: i32, lareacode: i32, pfaxoutboundroutingrule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ItemByCountryAndArea: usize,
    pub RemoveByCountryAndArea: unsafe extern "system" fn(this: *mut *mut Self, lcountrycode: i32, lareacode: i32) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, lcountrycode: i32, lareacode: i32, busedevice: i16, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldeviceid: i32, pfaxoutboundroutingrule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxOutboundRoutingRules {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3706692071, data2: 44669, data3: 20182, data4: [133, 33, 54, 158, 220, 202, 81, 32] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxOutgoingArchive {
    pub base__: super::super::System::Com::IDispatch,
    pub UseArchive: unsafe extern "system" fn(this: *mut *mut Self, pbusearchive: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUseArchive: unsafe extern "system" fn(this: *mut *mut Self, busearchive: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ArchiveFolder: unsafe extern "system" fn(this: *mut *mut Self, pbstrarchivefolder: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ArchiveFolder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetArchiveFolder: unsafe extern "system" fn(this: *mut *mut Self, bstrarchivefolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetArchiveFolder: usize,
    pub SizeQuotaWarning: unsafe extern "system" fn(this: *mut *mut Self, pbsizequotawarning: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSizeQuotaWarning: unsafe extern "system" fn(this: *mut *mut Self, bsizequotawarning: i16) -> ::windows_sys::core::HRESULT,
    pub HighQuotaWaterMark: unsafe extern "system" fn(this: *mut *mut Self, plhighquotawatermark: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetHighQuotaWaterMark: unsafe extern "system" fn(this: *mut *mut Self, lhighquotawatermark: i32) -> ::windows_sys::core::HRESULT,
    pub LowQuotaWaterMark: unsafe extern "system" fn(this: *mut *mut Self, pllowquotawatermark: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLowQuotaWaterMark: unsafe extern "system" fn(this: *mut *mut Self, llowquotawatermark: i32) -> ::windows_sys::core::HRESULT,
    pub AgeLimit: unsafe extern "system" fn(this: *mut *mut Self, plagelimit: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAgeLimit: unsafe extern "system" fn(this: *mut *mut Self, lagelimit: i32) -> ::windows_sys::core::HRESULT,
    pub SizeLow: unsafe extern "system" fn(this: *mut *mut Self, plsizelow: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(this: *mut *mut Self, plsizehigh: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessages: unsafe extern "system" fn(this: *mut *mut Self, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessages: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetMessage: unsafe extern "system" fn(this: *mut *mut Self, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetMessage: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxOutgoingArchive {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3384971072, data2: 36224, data3: 20051, data4: [129, 15, 154, 121, 145, 155, 73, 253] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxOutgoingJob {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DocumentName: unsafe extern "system" fn(this: *mut *mut Self, pbstrdocumentname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DocumentName: usize,
    pub Pages: unsafe extern "system" fn(this: *mut *mut Self, plpages: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, plsize: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SubmissionId: unsafe extern "system" fn(this: *mut *mut Self, pbstrsubmissionid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubmissionId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    pub OriginalScheduledTime: unsafe extern "system" fn(this: *mut *mut Self, pdateoriginalscheduledtime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SubmissionTime: unsafe extern "system" fn(this: *mut *mut Self, pdatesubmissiontime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ReceiptType: unsafe extern "system" fn(this: *mut *mut Self, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Sender: unsafe extern "system" fn(this: *mut *mut Self, ppfaxsender: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sender: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recipient: unsafe extern "system" fn(this: *mut *mut Self, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recipient: usize,
    pub CurrentPage: unsafe extern "system" fn(this: *mut *mut Self, plcurrentpage: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, pldeviceid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows_sys::core::HRESULT,
    pub ExtendedStatusCode: unsafe extern "system" fn(this: *mut *mut Self, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut *mut Self, pbstrextendedstatus: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtendedStatus: usize,
    pub AvailableOperations: unsafe extern "system" fn(this: *mut *mut Self, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows_sys::core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut *mut Self, plretries: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ScheduledTime: unsafe extern "system" fn(this: *mut *mut Self, pdatescheduledtime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut *mut Self, pdatetransmissionstart: *mut f64) -> ::windows_sys::core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut *mut Self, pdatetransmissionend: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    pub GroupBroadcastReceipts: unsafe extern "system" fn(this: *mut *mut Self, pbgroupbroadcastreceipts: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Restart: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTiff: unsafe extern "system" fn(this: *mut *mut Self, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTiff: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxOutgoingJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1666636461, data2: 26132, data3: 17795, data4: [191, 122, 58, 214, 123, 191, 199, 28] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxOutgoingJob2 {
    pub base__: IFaxOutgoingJob,
    pub HasCoverPage: unsafe extern "system" fn(this: *mut *mut Self, pbhascoverpage: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReceiptAddress: unsafe extern "system" fn(this: *mut *mut Self, pbstrreceiptaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReceiptAddress: usize,
    pub ScheduleType: unsafe extern "system" fn(this: *mut *mut Self, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxOutgoingJob2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1099599254, data2: 22944, data3: 18313, data4: [177, 118, 237, 243, 220, 143, 168, 247] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxOutgoingJobs {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxoutgoingjob: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxOutgoingJobs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 743889126, data2: 35887, data3: 17779, data4: [148, 76, 229, 5, 248, 245, 174, 237] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxOutgoingMessage {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub SubmissionId: unsafe extern "system" fn(this: *mut *mut Self, pbstrsubmissionid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubmissionId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DocumentName: unsafe extern "system" fn(this: *mut *mut Self, pbstrdocumentname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DocumentName: usize,
    pub Retries: unsafe extern "system" fn(this: *mut *mut Self, plretries: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Pages: unsafe extern "system" fn(this: *mut *mut Self, plpages: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, plsize: *mut i32) -> ::windows_sys::core::HRESULT,
    pub OriginalScheduledTime: unsafe extern "system" fn(this: *mut *mut Self, pdateoriginalscheduledtime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SubmissionTime: unsafe extern "system" fn(this: *mut *mut Self, pdatesubmissiontime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Sender: unsafe extern "system" fn(this: *mut *mut Self, ppfaxsender: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sender: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recipient: unsafe extern "system" fn(this: *mut *mut Self, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recipient: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceName: unsafe extern "system" fn(this: *mut *mut Self, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceName: usize,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut *mut Self, pdatetransmissionstart: *mut f64) -> ::windows_sys::core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut *mut Self, pdatetransmissionend: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTiff: unsafe extern "system" fn(this: *mut *mut Self, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTiff: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxOutgoingMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4041881054, data2: 51877, data3: 19068, data4: [130, 199, 43, 96, 186, 95, 43, 226] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxOutgoingMessage2 {
    pub base__: IFaxOutgoingMessage,
    pub HasCoverPage: unsafe extern "system" fn(this: *mut *mut Self, pbhascoverpage: *mut i16) -> ::windows_sys::core::HRESULT,
    pub ReceiptType: unsafe extern "system" fn(this: *mut *mut Self, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReceiptAddress: unsafe extern "system" fn(this: *mut *mut Self, pbstrreceiptaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReceiptAddress: usize,
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, pbread: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetRead: unsafe extern "system" fn(this: *mut *mut Self, bread: i16) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxOutgoingMessage2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3011376775, data2: 48264, data3: 19270, data4: [179, 190, 180, 88, 179, 234, 158, 127] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxOutgoingMessageIterator {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, pfaxoutgoingmessage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
    pub AtEOF: unsafe extern "system" fn(this: *mut *mut Self, pbeof: *mut i16) -> ::windows_sys::core::HRESULT,
    pub PrefetchSize: unsafe extern "system" fn(this: *mut *mut Self, plprefetchsize: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPrefetchSize: unsafe extern "system" fn(this: *mut *mut Self, lprefetchsize: i32) -> ::windows_sys::core::HRESULT,
    pub MoveFirst: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxOutgoingMessageIterator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4125908303, data2: 47168, data3: 17199, data4: [153, 128, 17, 47, 228, 42, 155, 122] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxOutgoingQueue {
    pub base__: super::super::System::Com::IDispatch,
    pub Blocked: unsafe extern "system" fn(this: *mut *mut Self, pbblocked: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetBlocked: unsafe extern "system" fn(this: *mut *mut Self, bblocked: i16) -> ::windows_sys::core::HRESULT,
    pub Paused: unsafe extern "system" fn(this: *mut *mut Self, pbpaused: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetPaused: unsafe extern "system" fn(this: *mut *mut Self, bpaused: i16) -> ::windows_sys::core::HRESULT,
    pub AllowPersonalCoverPages: unsafe extern "system" fn(this: *mut *mut Self, pballowpersonalcoverpages: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowPersonalCoverPages: unsafe extern "system" fn(this: *mut *mut Self, ballowpersonalcoverpages: i16) -> ::windows_sys::core::HRESULT,
    pub UseDeviceTSID: unsafe extern "system" fn(this: *mut *mut Self, pbusedevicetsid: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUseDeviceTSID: unsafe extern "system" fn(this: *mut *mut Self, busedevicetsid: i16) -> ::windows_sys::core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut *mut Self, plretries: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRetries: unsafe extern "system" fn(this: *mut *mut Self, lretries: i32) -> ::windows_sys::core::HRESULT,
    pub RetryDelay: unsafe extern "system" fn(this: *mut *mut Self, plretrydelay: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRetryDelay: unsafe extern "system" fn(this: *mut *mut Self, lretrydelay: i32) -> ::windows_sys::core::HRESULT,
    pub DiscountRateStart: unsafe extern "system" fn(this: *mut *mut Self, pdatediscountratestart: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDiscountRateStart: unsafe extern "system" fn(this: *mut *mut Self, datediscountratestart: f64) -> ::windows_sys::core::HRESULT,
    pub DiscountRateEnd: unsafe extern "system" fn(this: *mut *mut Self, pdatediscountrateend: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDiscountRateEnd: unsafe extern "system" fn(this: *mut *mut Self, datediscountrateend: f64) -> ::windows_sys::core::HRESULT,
    pub AgeLimit: unsafe extern "system" fn(this: *mut *mut Self, plagelimit: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAgeLimit: unsafe extern "system" fn(this: *mut *mut Self, lagelimit: i32) -> ::windows_sys::core::HRESULT,
    pub Branding: unsafe extern "system" fn(this: *mut *mut Self, pbbranding: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetBranding: unsafe extern "system" fn(this: *mut *mut Self, bbranding: i16) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJobs: unsafe extern "system" fn(this: *mut *mut Self, pfaxoutgoingjobs: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJobs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetJob: unsafe extern "system" fn(this: *mut *mut Self, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingjob: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetJob: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxOutgoingQueue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2159140644, data2: 55724, data3: 17203, data4: [179, 115, 72, 124, 237, 200, 12, 229] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxReceiptOptions {
    pub base__: super::super::System::Com::IDispatch,
    pub AuthenticationType: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(this: *mut *mut Self, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SMTPServer: unsafe extern "system" fn(this: *mut *mut Self, pbstrsmtpserver: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SMTPServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSMTPServer: unsafe extern "system" fn(this: *mut *mut Self, bstrsmtpserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSMTPServer: usize,
    pub SMTPPort: unsafe extern "system" fn(this: *mut *mut Self, plsmtpport: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSMTPPort: unsafe extern "system" fn(this: *mut *mut Self, lsmtpport: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SMTPSender: unsafe extern "system" fn(this: *mut *mut Self, pbstrsmtpsender: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SMTPSender: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSMTPSender: unsafe extern "system" fn(this: *mut *mut Self, bstrsmtpsender: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSMTPSender: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SMTPUser: unsafe extern "system" fn(this: *mut *mut Self, pbstrsmtpuser: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SMTPUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSMTPUser: unsafe extern "system" fn(this: *mut *mut Self, bstrsmtpuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSMTPUser: usize,
    pub AllowedReceipts: unsafe extern "system" fn(this: *mut *mut Self, pallowedreceipts: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub SetAllowedReceipts: unsafe extern "system" fn(this: *mut *mut Self, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SMTPPassword: unsafe extern "system" fn(this: *mut *mut Self, pbstrsmtppassword: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SMTPPassword: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSMTPPassword: unsafe extern "system" fn(this: *mut *mut Self, bstrsmtppassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSMTPPassword: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub UseForInboundRouting: unsafe extern "system" fn(this: *mut *mut Self, pbuseforinboundrouting: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUseForInboundRouting: unsafe extern "system" fn(this: *mut *mut Self, buseforinboundrouting: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxReceiptOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 932117227, data2: 24523, data3: 19195, data4: [178, 238, 225, 110, 128, 97, 68, 135] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxRecipient {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub FaxNumber: unsafe extern "system" fn(this: *mut *mut Self, pbstrfaxnumber: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FaxNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFaxNumber: unsafe extern "system" fn(this: *mut *mut Self, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFaxNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxRecipient {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2587730848, data2: 21389, data3: 17078, data4: [148, 68, 170, 165, 125, 12, 226, 188] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxRecipients {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrecipientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxRecipients {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3117014618, data2: 35150, data3: 17554, data4: [159, 163, 8, 198, 39, 193, 29, 93] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxSecurity {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Descriptor: unsafe extern "system" fn(this: *mut *mut Self, pvdescriptor: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Descriptor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDescriptor: unsafe extern "system" fn(this: *mut *mut Self, vdescriptor: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDescriptor: usize,
    pub GrantedRights: unsafe extern "system" fn(this: *mut *mut Self, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub InformationType: unsafe extern "system" fn(this: *mut *mut Self, plinformationtype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetInformationType: unsafe extern "system" fn(this: *mut *mut Self, linformationtype: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxSecurity {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2008352961, data2: 2496, data3: 18338, data4: [145, 235, 252, 231, 253, 242, 105, 14] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxSecurity2 {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Descriptor: unsafe extern "system" fn(this: *mut *mut Self, pvdescriptor: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Descriptor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDescriptor: unsafe extern "system" fn(this: *mut *mut Self, vdescriptor: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDescriptor: usize,
    pub GrantedRights: unsafe extern "system" fn(this: *mut *mut Self, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM_2) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub InformationType: unsafe extern "system" fn(this: *mut *mut Self, plinformationtype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetInformationType: unsafe extern "system" fn(this: *mut *mut Self, linformationtype: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxSecurity2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 400052724, data2: 53403, data3: 18684, data4: [153, 201, 143, 36, 196, 219, 154, 177] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxSender {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub BillingCode: unsafe extern "system" fn(this: *mut *mut Self, pbstrbillingcode: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BillingCode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBillingCode: unsafe extern "system" fn(this: *mut *mut Self, bstrbillingcode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBillingCode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub City: unsafe extern "system" fn(this: *mut *mut Self, pbstrcity: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    City: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCity: unsafe extern "system" fn(this: *mut *mut Self, bstrcity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Company: unsafe extern "system" fn(this: *mut *mut Self, pbstrcompany: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Company: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCompany: unsafe extern "system" fn(this: *mut *mut Self, bstrcompany: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCompany: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Country: unsafe extern "system" fn(this: *mut *mut Self, pbstrcountry: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Country: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCountry: unsafe extern "system" fn(this: *mut *mut Self, bstrcountry: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCountry: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Department: unsafe extern "system" fn(this: *mut *mut Self, pbstrdepartment: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Department: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDepartment: unsafe extern "system" fn(this: *mut *mut Self, bstrdepartment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDepartment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Email: unsafe extern "system" fn(this: *mut *mut Self, pbstremail: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Email: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEmail: unsafe extern "system" fn(this: *mut *mut Self, bstremail: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEmail: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FaxNumber: unsafe extern "system" fn(this: *mut *mut Self, pbstrfaxnumber: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FaxNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFaxNumber: unsafe extern "system" fn(this: *mut *mut Self, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFaxNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HomePhone: unsafe extern "system" fn(this: *mut *mut Self, pbstrhomephone: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HomePhone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHomePhone: unsafe extern "system" fn(this: *mut *mut Self, bstrhomephone: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHomePhone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut *mut Self, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTSID: unsafe extern "system" fn(this: *mut *mut Self, bstrtsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OfficePhone: unsafe extern "system" fn(this: *mut *mut Self, pbstrofficephone: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OfficePhone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOfficePhone: unsafe extern "system" fn(this: *mut *mut Self, bstrofficephone: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOfficePhone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OfficeLocation: unsafe extern "system" fn(this: *mut *mut Self, pbstrofficelocation: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OfficeLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOfficeLocation: unsafe extern "system" fn(this: *mut *mut Self, bstrofficelocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOfficeLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub State: unsafe extern "system" fn(this: *mut *mut Self, pbstrstate: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    State: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, bstrstate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StreetAddress: unsafe extern "system" fn(this: *mut *mut Self, pbstrstreetaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StreetAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStreetAddress: unsafe extern "system" fn(this: *mut *mut Self, bstrstreetaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStreetAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, pbstrtitle: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Title: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTitle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ZipCode: unsafe extern "system" fn(this: *mut *mut Self, pbstrzipcode: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ZipCode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetZipCode: unsafe extern "system" fn(this: *mut *mut Self, bstrzipcode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetZipCode: usize,
    pub LoadDefaultSender: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SaveDefaultSender: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxSender {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 226991485, data2: 62842, data3: 19654, data4: [166, 249, 62, 229, 213, 39, 180, 106] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxServer {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Connect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServerName: unsafe extern "system" fn(this: *mut *mut Self, pbstrservername: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServerName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDeviceProviders: unsafe extern "system" fn(this: *mut *mut Self, ppfaxdeviceproviders: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDeviceProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDevices: unsafe extern "system" fn(this: *mut *mut Self, ppfaxdevices: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDevices: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InboundRouting: unsafe extern "system" fn(this: *mut *mut Self, ppfaxinboundrouting: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InboundRouting: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Folders: unsafe extern "system" fn(this: *mut *mut Self, pfaxfolders: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Folders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LoggingOptions: unsafe extern "system" fn(this: *mut *mut Self, ppfaxloggingoptions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoggingOptions: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut *mut Self, plmajorversion: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut *mut Self, plminorversion: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MajorBuild: unsafe extern "system" fn(this: *mut *mut Self, plmajorbuild: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinorBuild: unsafe extern "system" fn(this: *mut *mut Self, plminorbuild: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Debug: unsafe extern "system" fn(this: *mut *mut Self, pbdebug: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Activity: unsafe extern "system" fn(this: *mut *mut Self, ppfaxactivity: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Activity: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OutboundRouting: unsafe extern "system" fn(this: *mut *mut Self, ppfaxoutboundrouting: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutboundRouting: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiptOptions: unsafe extern "system" fn(this: *mut *mut Self, ppfaxreceiptoptions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiptOptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security: unsafe extern "system" fn(this: *mut *mut Self, ppfaxsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetExtensionProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvproperty: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetExtensionProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetExtensionProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetExtensionProperty: usize,
    pub ListenToServerEvents: unsafe extern "system" fn(this: *mut *mut Self, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterDeviceProvider: unsafe extern "system" fn(this: *mut *mut Self, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrimagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, tspname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lfspiversion: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterDeviceProvider: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterDeviceProvider: unsafe extern "system" fn(this: *mut *mut Self, bstruniquename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterDeviceProvider: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RegisterInboundRoutingExtension: unsafe extern "system" fn(this: *mut *mut Self, bstrextensionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrimagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vmethods: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RegisterInboundRoutingExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterInboundRoutingExtension: unsafe extern "system" fn(this: *mut *mut Self, bstrextensionuniquename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterInboundRoutingExtension: usize,
    pub RegisteredEvents: unsafe extern "system" fn(this: *mut *mut Self, peventtypes: *mut FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows_sys::core::HRESULT,
    pub APIVersion: unsafe extern "system" fn(this: *mut *mut Self, papiversion: *mut FAX_SERVER_APIVERSION_ENUM) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxServer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1197171817, data2: 37029, data3: 18552, data4: [165, 119, 23, 168, 110, 142, 52, 98] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxServer2 {
    pub base__: IFaxServer,
    #[cfg(feature = "Win32_System_Com")]
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, ppfaxconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Configuration: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentAccount: unsafe extern "system" fn(this: *mut *mut Self, ppcurrentaccount: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentAccount: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub FaxAccountSet: unsafe extern "system" fn(this: *mut *mut Self, ppfaxaccountset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FaxAccountSet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security2: unsafe extern "system" fn(this: *mut *mut Self, ppfaxsecurity2: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security2: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxServer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1461513487, data2: 22025, data3: 20288, data4: [145, 118, 84, 126, 58, 114, 202, 124] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxServerNotify {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxServerNotify {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 771980071, data2: 53130, data3: 19133, data4: [177, 224, 87, 4, 148, 59, 234, 111] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFaxServerNotify2 {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFaxServerNotify2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1634511062, data2: 42874, data3: 16482, data4: [171, 253, 14, 71, 18, 65, 199, 170] };
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const IS_DIGITAL_CAMERA_STR: &str = "IsDigitalCamera";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const IS_DIGITAL_CAMERA_VAL: u32 = 1u32;
#[repr(C)]
pub struct IStiDevice {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: ::windows_sys::core::PCWSTR, dwversion: u32, dwmode: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut *mut Self, pdevcaps: *mut STI_DEV_CAPS) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows_sys::core::HRESULT,
    pub DeviceReset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(this: *mut *mut Self, pbuffer: *mut STI_DIAG) -> ::windows_sys::core::HRESULT,
    pub Escape: unsafe extern "system" fn(this: *mut *mut Self, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLastError: unsafe extern "system" fn(this: *mut *mut Self, pdwlastdeviceerror: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LockDevice: unsafe extern "system" fn(this: *mut *mut Self, dwtimeout: u32) -> ::windows_sys::core::HRESULT,
    pub UnLockDevice: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadData: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteData: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadCommand: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadCommand: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteCommand: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteCommand: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Subscribe: unsafe extern "system" fn(this: *mut *mut Self, lpsubsribe: *mut STISUBSCRIBE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subscribe: usize,
    pub GetLastNotificationData: unsafe extern "system" fn(this: *mut *mut Self, lpnotify: *mut STINOTIFY) -> ::windows_sys::core::HRESULT,
    pub UnSubscribe: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetLastErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStiDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1828346496, data2: 11720, data3: 4560, data4: [144, 234, 0, 170, 0, 96, 248, 108] };
}
#[repr(C)]
pub struct IStiDeviceControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, dwdevicetype: u32, dwmode: u32, pwszportname: ::windows_sys::core::PCWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadData: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteData: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadCommand: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadCommand: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteCommand: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteCommand: usize,
    pub RawDeviceControl: unsafe extern "system" fn(this: *mut *mut Self, escapefunction: u32, lpindata: *mut ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLastError: unsafe extern "system" fn(this: *mut *mut Self, lpdwlasterror: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMyDevicePortName: unsafe extern "system" fn(this: *mut *mut Self, lpszdevicepath: ::windows_sys::core::PWSTR, cwdevicepathsize: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMyDeviceHandle: unsafe extern "system" fn(this: *mut *mut Self, lph: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMyDeviceHandle: usize,
    pub GetMyDeviceOpenMode: unsafe extern "system" fn(this: *mut *mut Self, pdwopenmode: *mut u32) -> ::windows_sys::core::HRESULT,
    pub WriteToErrorLog: unsafe extern "system" fn(this: *mut *mut Self, dwmessagetype: u32, pszmessage: ::windows_sys::core::PCWSTR, dwerrorcode: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStiDeviceControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 311072864, data2: 21212, data3: 4560, data4: [158, 223, 68, 69, 83, 84, 0, 0] };
}
#[repr(C)]
pub struct IStiDeviceW(pub u8);
#[repr(C)]
pub struct IStiUSD {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Registry")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pheldcb: *mut ::core::ffi::c_void, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    Initialize: usize,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut *mut Self, pdevcaps: *mut STI_USD_CAPS) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows_sys::core::HRESULT,
    pub DeviceReset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(this: *mut *mut Self, pbuffer: *mut STI_DIAG) -> ::windows_sys::core::HRESULT,
    pub Escape: unsafe extern "system" fn(this: *mut *mut Self, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLastError: unsafe extern "system" fn(this: *mut *mut Self, pdwlastdeviceerror: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LockDevice: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub UnLockDevice: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadData: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteData: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadCommand: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadCommand: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteCommand: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteCommand: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNotificationHandle: unsafe extern "system" fn(this: *mut *mut Self, hevent: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNotificationHandle: usize,
    pub GetNotificationData: unsafe extern "system" fn(this: *mut *mut Self, lpnotify: *mut STINOTIFY) -> ::windows_sys::core::HRESULT,
    pub GetLastErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStiUSD {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 211530848, data2: 20908, data3: 4560, data4: [144, 234, 0, 170, 0, 96, 248, 108] };
}
#[repr(C)]
pub struct IStillImageW {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub GetDeviceList: unsafe extern "system" fn(this: *mut *mut Self, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(this: *mut *mut Self, pwszdevicename: ::windows_sys::core::PCWSTR, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDevice: unsafe extern "system" fn(this: *mut *mut Self, pwszdevicename: ::windows_sys::core::PCWSTR, dwmode: u32, pdevice: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeviceValue: unsafe extern "system" fn(this: *mut *mut Self, pwszdevicename: ::windows_sys::core::PCWSTR, pvaluename: ::windows_sys::core::PCWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDeviceValue: unsafe extern "system" fn(this: *mut *mut Self, pwszdevicename: ::windows_sys::core::PCWSTR, pvaluename: ::windows_sys::core::PCWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> ::windows_sys::core::HRESULT,
    pub GetSTILaunchInformation: unsafe extern "system" fn(this: *mut *mut Self, pwszdevicename: ::windows_sys::core::PWSTR, pdweventcode: *mut u32, pwszeventname: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub RegisterLaunchApplication: unsafe extern "system" fn(this: *mut *mut Self, pwszappname: ::windows_sys::core::PCWSTR, pwszcommandline: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub UnregisterLaunchApplication: unsafe extern "system" fn(this: *mut *mut Self, pwszappname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableHwNotifications: unsafe extern "system" fn(this: *mut *mut Self, pwszdevicename: ::windows_sys::core::PCWSTR, bnewstate: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableHwNotifications: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHwNotificationState: unsafe extern "system" fn(this: *mut *mut Self, pwszdevicename: ::windows_sys::core::PCWSTR, pbcurrentstate: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHwNotificationState: usize,
    pub RefreshDeviceBus: unsafe extern "system" fn(this: *mut *mut Self, pwszdevicename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub LaunchApplicationForDevice: unsafe extern "system" fn(this: *mut *mut Self, pwszdevicename: ::windows_sys::core::PCWSTR, pwszappname: ::windows_sys::core::PCWSTR, pstinotify: *const STINOTIFY) -> ::windows_sys::core::HRESULT,
    pub SetupDeviceParameters: unsafe extern "system" fn(this: *mut *mut Self, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows_sys::core::HRESULT,
    pub WriteToErrorLog: unsafe extern "system" fn(this: *mut *mut Self, dwmessagetype: u32, pszmessage: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStillImageW {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1679546496, data2: 11720, data3: 4560, data4: [144, 234, 0, 170, 0, 96, 248, 108] };
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_DELETING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_FAILED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_INPROGRESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_NOLINE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_PAUSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_PENDING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_RETRIES_EXCEEDED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_RETRYING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_FAIL_RECEIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_RECEIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_ROUTING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_SEND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const MAX_NOTIFICATION_DATA: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const MS_FAXROUTE_EMAIL_GUID: &str = "{6bbf7bfe-9af2-11d0-abf7-00c04fd91a4e}";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const MS_FAXROUTE_FOLDER_GUID: &str = "{92041a90-9af2-11d0-abf7-00c04fd91a4e}";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const MS_FAXROUTE_PRINTING_GUID: &str = "{aec1b37c-9af2-11d0-abf7-00c04fd91a4e}";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXABORT = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXACCESSCHECK = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, accessmask: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCLOSE = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCOMPLETEJOBPARAMSA = ::core::option::Option<unsafe extern "system" fn(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCOMPLETEJOBPARAMSW = ::core::option::Option<unsafe extern "system" fn(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCONNECTFAXSERVERA = ::core::option::Option<unsafe extern "system" fn(machinename: ::windows_sys::core::PCSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCONNECTFAXSERVERW = ::core::option::Option<unsafe extern "system" fn(machinename: ::windows_sys::core::PCWSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVABORTOPERATION = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type PFAXDEVCONFIGURE = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVENDJOB = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVINITIALIZE = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: super::super::Foundation::HANDLE, param2: *mut PFAX_LINECALLBACK, param3: PFAX_SERVICE_CALLBACK) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVRECEIVE = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: *mut FAX_RECEIVE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVREPORTSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_DEV_STATUS, param2: u32, param3: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVSEND = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_SEND, param2: PFAX_SEND_CALLBACK) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAXDEVSHUTDOWN = ::core::option::Option<unsafe extern "system" fn() -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVSTARTJOB = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: *mut super::super::Foundation::HANDLE, param3: super::super::Foundation::HANDLE, param4: usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVVIRTUALDEVICECREATION = ::core::option::Option<unsafe extern "system" fn(devicecount: *mut u32, devicenameprefix: ::windows_sys::core::PWSTR, deviceidprefix: *mut u32, completionport: super::super::Foundation::HANDLE, completionkey: usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENABLEROUTINGMETHODA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_sys::core::PCSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENABLEROUTINGMETHODW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_sys::core::PCWSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMGLOBALROUTINGINFOA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMGLOBALROUTINGINFOW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMJOBSA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMJOBSW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMPORTSA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMPORTSW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMROUTINGMETHODSA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMROUTINGMETHODSW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAXFREEBUFFER = ::core::option::Option<unsafe extern "system" fn(buffer: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETCONFIGURATIONA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETCONFIGURATIONW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETDEVICESTATUSA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETDEVICESTATUSW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETJOBA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETJOBW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETLOGGINGCATEGORIESA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETLOGGINGCATEGORIESW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETPAGEDATA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETPORTA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETPORTW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETROUTINGINFOA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_sys::core::PCSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETROUTINGINFOW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_sys::core::PCWSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXINITIALIZEEVENTQUEUE = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, completionport: super::super::Foundation::HANDLE, completionkey: usize, hwnd: super::super::Foundation::HWND, messagestart: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXOPENPORT = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXPRINTCOVERPAGEA = ::core::option::Option<unsafe extern "system" fn(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXPRINTCOVERPAGEW = ::core::option::Option<unsafe extern "system" fn(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXREGISTERROUTINGEXTENSIONW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, extensionname: ::windows_sys::core::PCWSTR, friendlyname: ::windows_sys::core::PCWSTR, imagename: ::windows_sys::core::PCWSTR, callback: PFAX_ROUTING_INSTALLATION_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXREGISTERSERVICEPROVIDERW = ::core::option::Option<unsafe extern "system" fn(deviceprovider: ::windows_sys::core::PCWSTR, friendlyname: ::windows_sys::core::PCWSTR, imagename: ::windows_sys::core::PCWSTR, tspname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAXROUTEADDFILE = ::core::option::Option<unsafe extern "system" fn(jobid: u32, filename: ::windows_sys::core::PCWSTR, guid: *mut ::windows_sys::core::GUID) -> i32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAXROUTEDELETEFILE = ::core::option::Option<unsafe extern "system" fn(jobid: u32, filename: ::windows_sys::core::PCWSTR) -> i32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEDEVICECHANGENOTIFICATION = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEDEVICEENABLE = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_sys::core::PCWSTR, param1: u32, param2: i32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEENUMFILE = ::core::option::Option<unsafe extern "system" fn(jobid: u32, guidowner: *mut ::windows_sys::core::GUID, guidcaller: *mut ::windows_sys::core::GUID, filename: ::windows_sys::core::PCWSTR, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEENUMFILES = ::core::option::Option<unsafe extern "system" fn(jobid: u32, guid: *mut ::windows_sys::core::GUID, fileenumerator: PFAXROUTEENUMFILE, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEGETFILE = ::core::option::Option<unsafe extern "system" fn(jobid: u32, index: u32, filenamebuffer: ::windows_sys::core::PWSTR, requiredsize: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEGETROUTINGINFO = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_sys::core::PCWSTR, param1: u32, param2: *mut u8, param3: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEINITIALIZE = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_ROUTE_CALLBACKROUTINES) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEMETHOD = ::core::option::Option<unsafe extern "system" fn(param0: *const FAX_ROUTE, param1: *mut *mut ::core::ffi::c_void, param2: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEMODIFYROUTINGDATA = ::core::option::Option<unsafe extern "system" fn(jobid: u32, routingguid: ::windows_sys::core::PCWSTR, routingdata: *mut u8, routingdatasize: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTESETROUTINGINFO = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_sys::core::PCWSTR, param1: u32, param2: *const u8, param3: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: ::windows_sys::core::PCSTR, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTFORBROADCASTA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: ::windows_sys::core::PCSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKA, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTFORBROADCASTW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: ::windows_sys::core::PCWSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: ::windows_sys::core::PCWSTR, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETCONFIGURATIONA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETCONFIGURATIONW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETGLOBALROUTINGINFOA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETGLOBALROUTINGINFOW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETJOBA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETJOBW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETLOGGINGCATEGORIESA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETLOGGINGCATEGORIESW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETPORTA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETPORTW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETROUTINGINFOA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_sys::core::PCSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETROUTINGINFOW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows_sys::core::PCWSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXSTARTPRINTJOBA = ::core::option::Option<unsafe extern "system" fn(printername: ::windows_sys::core::PCSTR, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXSTARTPRINTJOBW = ::core::option::Option<unsafe extern "system" fn(printername: ::windows_sys::core::PCWSTR, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXUNREGISTERSERVICEPROVIDERW = ::core::option::Option<unsafe extern "system" fn(deviceprovider: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAX_EXT_CONFIG_CHANGE = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: ::windows_sys::core::PCWSTR, param2: *mut u8, param3: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAX_EXT_FREE_BUFFER = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAX_EXT_GET_DATA = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: FAX_ENUM_DEVICE_ID_SOURCE, param2: ::windows_sys::core::PCWSTR, param3: *mut *mut u8, param4: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_INITIALIZE_CONFIG = ::core::option::Option<unsafe extern "system" fn(param0: PFAX_EXT_GET_DATA, param1: PFAX_EXT_SET_DATA, param2: PFAX_EXT_REGISTER_FOR_EVENTS, param3: PFAX_EXT_UNREGISTER_FOR_EVENTS, param4: PFAX_EXT_FREE_BUFFER) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_REGISTER_FOR_EVENTS = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HINSTANCE, param1: u32, param2: FAX_ENUM_DEVICE_ID_SOURCE, param3: ::windows_sys::core::PCWSTR, param4: PFAX_EXT_CONFIG_CHANGE) -> super::super::Foundation::HANDLE>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_SET_DATA = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HINSTANCE, param1: u32, param2: FAX_ENUM_DEVICE_ID_SOURCE, param3: ::windows_sys::core::PCWSTR, param4: *mut u8, param5: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_UNREGISTER_FOR_EVENTS = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_LINECALLBACK = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, hdevice: u32, dwmessage: u32, dwinstance: usize, dwparam1: usize, dwparam2: usize, dwparam3: usize)>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_RECIPIENT_CALLBACKA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, recipientnumber: u32, context: *mut ::core::ffi::c_void, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_RECIPIENT_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, recipientnumber: u32, context: *mut ::core::ffi::c_void, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_ROUTING_INSTALLATION_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, context: *mut ::core::ffi::c_void, methodname: ::windows_sys::core::PCWSTR, friendlyname: ::windows_sys::core::PCWSTR, functionname: ::windows_sys::core::PCWSTR, guid: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_SEND_CALLBACK = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, callhandle: u32, reserved1: u32, reserved2: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_SERVICE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, param1: usize, param2: usize, param3: usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_BAUDRATE: &str = "BaudRate";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_BAUDRATE_A: &str = "BaudRate";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DATA_W: &str = "DeviceData";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DEVICESUBTYPE_W: &str = "DeviceSubType";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DEVICETYPE_W: &str = "DeviceType";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DEVICE_NAME_W: &str = "DriverDesc";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DEV_NAME_W: &str = "DeviceName";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DRIVER_DESC_W: &str = "DriverDesc";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_FRIENDLY_NAME_W: &str = "FriendlyName";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_GENERIC_CAPS_W: &str = "Capabilities";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_GUID: &str = "GUID";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_GUID_W: &str = "GUID";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_HARDWARE: &str = "HardwareConfig";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_HARDWARE_W: &str = "HardwareConfig";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_LAUNCHABLE: &str = "Launchable";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_LAUNCHABLE_W: &str = "Launchable";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_LAUNCH_APPS: &str = "LaunchApplications";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_LAUNCH_APPS_W: &str = "LaunchApplications";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_SHUTDOWNDELAY: &str = "ShutdownIfUnusedDelay";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_SHUTDOWNDELAY_W: &str = "ShutdownIfUnusedDelay";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_TYPE_W: &str = "Type";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_VENDOR_NAME_W: &str = "Vendor";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIEDFL_ALLDEVICES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIEDFL_ATTACHEDONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = -2147023649i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_BADDRIVER: ::windows_sys::core::HRESULT = -2147024777i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_BETA_VERSION: ::windows_sys::core::HRESULT = -2147023743i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_DEVICENOTREG: i32 = -2147221164i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_DEVICE_LOCKED: ::windows_sys::core::HRESULT = -2147024863i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_DEVICE_NOTREADY: ::windows_sys::core::HRESULT = -2147024875i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_GENERIC: i32 = -2147467259i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_HANDLEEXISTS: ::windows_sys::core::HRESULT = -2147024713i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_INVALID_DEVICE_NAME: ::windows_sys::core::HRESULT = -2147024773i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_INVALID_HW_TYPE: ::windows_sys::core::HRESULT = -2147024883i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_INVALID_PARAM: i32 = -2147024809i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NEEDS_LOCK: ::windows_sys::core::HRESULT = -2147024738i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NOEVENTS: ::windows_sys::core::HRESULT = -2147024637i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NOINTERFACE: i32 = -2147467262i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NOTINITIALIZED: i32 = -2147024891i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2147024875i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_OBJECTNOTFOUND: ::windows_sys::core::HRESULT = -2147024894i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_OLD_VERSION: ::windows_sys::core::HRESULT = -2147023746i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_OUTOFMEMORY: i32 = -2147024882i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_READONLY: i32 = -2147024891i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_SHARING_VIOLATION: ::windows_sys::core::HRESULT = -2147024864i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_UNSUPPORTED: i32 = -2147467263i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STINOTIFY {
    pub dwSize: u32,
    pub guidNotificationCode: ::windows_sys::core::GUID,
    pub abNotificationData: [u8; 64],
}
impl ::core::marker::Copy for STINOTIFY {}
impl ::core::clone::Clone for STINOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STISUBSCRIBE {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFilter: u32,
    pub hWndNotify: super::super::Foundation::HWND,
    pub hEvent: super::super::Foundation::HANDLE,
    pub uiNotificationMessage: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STISUBSCRIBE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STISUBSCRIBE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ADD_DEVICE_BROADCAST_ACTION: &str = "Arrival";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ADD_DEVICE_BROADCAST_STRING: &str = "STI\\";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_CHANGENOEFFECT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_BOTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_FOR_MONITOR: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_STATUS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: u32,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: ::windows_sys::core::PWSTR,
    pub pszDeviceDescription: ::windows_sys::core::PWSTR,
    pub pszPortName: ::windows_sys::core::PWSTR,
    pub pszPropProvider: ::windows_sys::core::PWSTR,
    pub pszLocalName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for STI_DEVICE_INFORMATIONW {}
impl ::core::clone::Clone for STI_DEVICE_INFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type STI_DEVICE_MJ_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const StiDeviceTypeDefault: STI_DEVICE_MJ_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const StiDeviceTypeScanner: STI_DEVICE_MJ_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const StiDeviceTypeDigitalCamera: STI_DEVICE_MJ_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const StiDeviceTypeStreamingVideo: STI_DEVICE_MJ_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_DEVICE_STATUS {
    pub dwSize: u32,
    pub StatusMask: u32,
    pub dwOnlineState: u32,
    pub dwHardwareStatusCode: u32,
    pub dwEventHandlingState: u32,
    pub dwPollingInterval: u32,
}
impl ::core::marker::Copy for STI_DEVICE_STATUS {}
impl ::core::clone::Clone for STI_DEVICE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_DEFAULT_LAUNCHAPP: &str = "DefaultLaunchApp";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_DEFAULT_LAUNCHAPP_A: &str = "DefaultLaunchApp";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_DISABLE_NOTIFICATIONS: &str = "DisableNotifications";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_DISABLE_NOTIFICATIONS_A: &str = "DisableNotifications";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_ICM_PROFILE: &str = "ICMProfile";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_ICM_PROFILE_A: &str = "ICMProfile";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_ISIS_NAME: &str = "ISISDriverName";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_ISIS_NAME_A: &str = "ISISDriverName";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_TIMEOUT: &str = "PollTimeout";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_TIMEOUT_A: &str = "PollTimeout";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_TWAIN_NAME: &str = "TwainDS";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_TWAIN_NAME_A: &str = "TwainDS";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVSTATUS_EVENTS_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVSTATUS_ONLINE_STATE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_DEV_CAPS {
    pub dwGeneric: u32,
}
impl ::core::marker::Copy for STI_DEV_CAPS {}
impl ::core::clone::Clone for STI_DEV_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_DIAG {
    pub dwSize: u32,
    pub dwBasicDiagCode: u32,
    pub dwVendorDiagCode: u32,
    pub dwStatusMask: u32,
    pub sErrorInfo: _ERROR_INFOW,
}
impl ::core::marker::Copy for STI_DIAG {}
impl ::core::clone::Clone for STI_DIAG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DIAGCODE_HWPRESENCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ERROR_NO_ERROR: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_EVENTHANDLING_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_EVENTHANDLING_PENDING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_EVENTHANDLING_POLLING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_AUTO_PORTSELECT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_GENERATE_ARRIVALEVENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_NOTIFICATIONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_POLLING_NEEDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_SUBSET: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_WIA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_PARALLEL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_SCSI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_SERIAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_UNKNOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_USB: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_MAX_INTERNAL_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_NOTCONNECTED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_OK: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_BUSY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_INITIALIZING: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_IO_ACTIVE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_OFFLINE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_OPERATIONAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_PAPER_JAM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_PAPER_PROBLEM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_PAUSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_PENDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_POWER_SAVE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_TRANSFERRING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_USER_INTERVENTION: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_WARMING_UP: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_RAW_RESERVED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_REMOVE_DEVICE_BROADCAST_ACTION: &str = "Removal";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_REMOVE_DEVICE_BROADCAST_STRING: &str = "STI\\";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_SUBSCRIBE_FLAG_EVENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_SUBSCRIBE_FLAG_WINDOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_TRACE_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_TRACE_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_TRACE_WARNING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_UNICODE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_USD_CAPS {
    pub dwVersion: u32,
    pub dwGenericCaps: u32,
}
impl ::core::marker::Copy for STI_USD_CAPS {}
impl ::core::clone::Clone for STI_USD_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_USD_GENCAP_NATIVE_PUSHSUPPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION_FLAG_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION_FLAG_UNICODE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION_MIN_ALLOWED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION_REAL: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_WIA_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: u32,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: ::windows_sys::core::PWSTR,
    pub pszDeviceDescription: ::windows_sys::core::PWSTR,
    pub pszPortName: ::windows_sys::core::PWSTR,
    pub pszPropProvider: ::windows_sys::core::PWSTR,
    pub pszLocalName: ::windows_sys::core::PWSTR,
    pub pszUiDll: ::windows_sys::core::PWSTR,
    pub pszServer: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for STI_WIA_DEVICE_INFORMATIONW {}
impl ::core::clone::Clone for STI_WIA_DEVICE_INFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const SUPPORTS_MSCPLUS_STR: &str = "SupportsMSCPlus";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const SUPPORTS_MSCPLUS_VAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type SendToMode = i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const SEND_TO_FAX_RECIPIENT_ATTACHMENT: SendToMode = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const WIA_INCOMPAT_XP: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct _ERROR_INFOW {
    pub dwSize: u32,
    pub dwGenericError: u32,
    pub dwVendorError: u32,
    pub szExtendedErrorText: [u16; 255],
}
impl ::core::marker::Copy for _ERROR_INFOW {}
impl ::core::clone::Clone for _ERROR_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _IFaxAccountNotify {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingJobAdded: unsafe extern "system" fn(this: *mut *mut Self, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingJobAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingJobRemoved: unsafe extern "system" fn(this: *mut *mut Self, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingJobRemoved: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingJobChanged: unsafe extern "system" fn(this: *mut *mut Self, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingJobChanged: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingJobAdded: unsafe extern "system" fn(this: *mut *mut Self, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingJobAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingJobRemoved: unsafe extern "system" fn(this: *mut *mut Self, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingJobRemoved: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingJobChanged: unsafe extern "system" fn(this: *mut *mut Self, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingJobChanged: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingMessageAdded: unsafe extern "system" fn(this: *mut *mut Self, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, faddedtoreceivefolder: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingMessageAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingMessageRemoved: unsafe extern "system" fn(this: *mut *mut Self, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fremovedfromreceivefolder: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingMessageRemoved: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingMessageAdded: unsafe extern "system" fn(this: *mut *mut Self, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingMessageAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingMessageRemoved: unsafe extern "system" fn(this: *mut *mut Self, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingMessageRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnServerShutDown: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnServerShutDown: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _IFaxAccountNotify {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3115564161, data2: 44059, data3: 18163, data4: [179, 157, 10, 220, 48, 225, 183, 136] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _IFaxServerNotify2 {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingJobAdded: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingJobAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingJobRemoved: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingJobRemoved: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingJobChanged: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingJobChanged: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingJobAdded: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingJobAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingJobRemoved: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingJobRemoved: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingJobChanged: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingJobChanged: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingMessageAdded: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingMessageAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingMessageRemoved: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingMessageRemoved: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingMessageAdded: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingMessageAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingMessageRemoved: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingMessageRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnReceiptOptionsChange: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnReceiptOptionsChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnActivityLoggingConfigChange: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnActivityLoggingConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSecurityConfigChange: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSecurityConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnEventLoggingConfigChange: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnEventLoggingConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingQueueConfigChange: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingQueueConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingArchiveConfigChange: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingArchiveConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnIncomingArchiveConfigChange: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnIncomingArchiveConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnDevicesConfigChange: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnDevicesConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutboundRoutingGroupsConfigChange: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutboundRoutingGroupsConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutboundRoutingRulesConfigChange: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutboundRoutingRulesConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnServerActivityChange: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnServerActivityChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnQueuesStatusChange: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, boutgoingqueueblocked: i16, boutgoingqueuepaused: i16, bincomingqueueblocked: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnQueuesStatusChange: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnNewCall: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, lcallid: i32, ldeviceid: i32, bstrcallerid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnNewCall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnServerShutDown: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnServerShutDown: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnDeviceStatusChange: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void, ldeviceid: i32, bpoweredoff: i16, bsending: i16, breceiving: i16, bringing: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnDeviceStatusChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnGeneralServerConfigChanged: unsafe extern "system" fn(this: *mut *mut Self, pfaxserver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnGeneralServerConfigChanged: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _IFaxServerNotify2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3969673657, data2: 24551, data3: 18437, data4: [148, 103, 130, 252, 217, 106, 249, 3] };
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const lDEFAULT_PREFETCH_SIZE: i32 = 100i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const prv_DEFAULT_PREFETCH_SIZE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const wcharREASSIGN_RECIPIENTS_DELIMITER: u16 = 59u16;
