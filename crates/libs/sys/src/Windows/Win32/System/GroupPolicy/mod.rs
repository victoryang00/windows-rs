#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BrowseForGPO(lpbrowseinfo: *mut GPOBROWSEINFO) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub fn CommandLineFromMsiDescriptor(descriptor: ::windows_sys::core::PCWSTR, commandline: ::windows_sys::core::PWSTR, commandlinelength: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateGPOLink(lpgpo: ::windows_sys::core::PCWSTR, lpcontainer: ::windows_sys::core::PCWSTR, fhighpriority: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub fn DeleteAllGPOLinks(lpcontainer: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub fn DeleteGPOLink(lpgpo: ::windows_sys::core::PCWSTR, lpcontainer: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnterCriticalPolicySection(bmachine: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub fn ExportRSoPData(lpnamespace: ::windows_sys::core::PCWSTR, lpfilename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeGPOListA(pgpolist: *const GROUP_POLICY_OBJECTA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeGPOListW(pgpolist: *const GROUP_POLICY_OBJECTW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GenerateGPNotification(bmachine: super::super::Foundation::BOOL, lpwszmgmtproduct: ::windows_sys::core::PCWSTR, dwmgmtproductoptions: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppliedGPOListA(dwflags: u32, pmachinename: ::windows_sys::core::PCSTR, psiduser: super::super::Foundation::PSID, pguidextension: *const ::windows_sys::core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> u32;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppliedGPOListW(dwflags: u32, pmachinename: ::windows_sys::core::PCWSTR, psiduser: super::super::Foundation::PSID, pguidextension: *const ::windows_sys::core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> u32;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGPOListA(htoken: super::super::Foundation::HANDLE, lpname: ::windows_sys::core::PCSTR, lphostname: ::windows_sys::core::PCSTR, lpcomputername: ::windows_sys::core::PCSTR, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGPOListW(htoken: super::super::Foundation::HANDLE, lpname: ::windows_sys::core::PCWSTR, lphostname: ::windows_sys::core::PCWSTR, lpcomputername: ::windows_sys::core::PCWSTR, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub fn GetLocalManagedApplicationData(productcode: ::windows_sys::core::PCWSTR, displayname: *mut ::windows_sys::core::PWSTR, supporturl: *mut ::windows_sys::core::PWSTR);
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocalManagedApplications(buserapps: super::super::Foundation::BOOL, pdwapps: *mut u32, prglocalapps: *mut *mut LOCALMANAGEDAPPLICATION) -> u32;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_UI_Shell\"`*"]
    #[cfg(feature = "Win32_UI_Shell")]
    pub fn GetManagedApplicationCategories(dwreserved: u32, pappcategory: *mut super::super::UI::Shell::APPCATEGORYINFOLIST) -> u32;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetManagedApplications(pcategory: *const ::windows_sys::core::GUID, dwqueryflags: u32, dwinfolevel: u32, pdwapps: *mut u32, prgmanagedapps: *mut *mut MANAGEDAPPLICATION) -> u32;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub fn ImportRSoPData(lpnamespace: ::windows_sys::core::PCWSTR, lpfilename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub fn InstallApplication(pinstallinfo: *const INSTALLDATA) -> u32;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LeaveCriticalPolicySection(hsection: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub fn ProcessGroupPolicyCompleted(extensionid: *const ::windows_sys::core::GUID, pasynchandle: usize, dwstatus: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub fn ProcessGroupPolicyCompletedEx(extensionid: *const ::windows_sys::core::GUID, pasynchandle: usize, dwstatus: u32, rsopstatus: ::windows_sys::core::HRESULT) -> u32;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshPolicy(bmachine: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshPolicyEx(bmachine: super::super::Foundation::BOOL, dwoptions: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterGPNotification(hevent: super::super::Foundation::HANDLE, bmachine: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RsopAccessCheckByType(psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, pprincipalselfsid: super::super::Foundation::PSID, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pobjecttypelist: *const super::super::Security::OBJECT_TYPE_LIST, objecttypelistlength: u32, pgenericmapping: *const super::super::Security::GENERIC_MAPPING, pprivilegeset: *const super::super::Security::PRIVILEGE_SET, pdwprivilegesetlength: *const u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub fn RsopFileAccessCheck(pszfilename: ::windows_sys::core::PCWSTR, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_System_Wmi\"`*"]
    #[cfg(feature = "Win32_System_Wmi")]
    pub fn RsopResetPolicySettingStatus(dwflags: u32, pservices: *mut *mut super::Wmi::IWbemServices, psettinginstance: *mut *mut super::Wmi::IWbemClassObject) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Wmi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Wmi"))]
    pub fn RsopSetPolicySettingStatus(dwflags: u32, pservices: *mut *mut super::Wmi::IWbemServices, psettinginstance: *mut *mut super::Wmi::IWbemClassObject, ninfo: u32, pstatus: *const POLICYSETTINGSTATUSINFO) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
    pub fn UninstallApplication(productcode: ::windows_sys::core::PCWSTR, dwstatus: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterGPNotification(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type APPSTATE = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const ABSENT: APPSTATE = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const ASSIGNED: APPSTATE = 1i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PUBLISHED: APPSTATE = 2i32;
pub const CLSID_GPESnapIn: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2411771700, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const CLSID_GroupPolicyObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3931121442, data2: 41533, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const CLSID_RSOPSnapIn: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1841528907, data2: 29202, data3: 17805, data4: [173, 176, 154, 7, 226, 174, 31, 162] };
pub type CriticalPolicySectionHandle = isize;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_ASSUME_COMP_WQLFILTER_TRUE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_ASSUME_SLOW_LINK: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_ASSUME_USER_WQLFILTER_TRUE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_FORCE_CREATENAMESPACE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_LOOPBACK_MERGE: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_LOOPBACK_REPLACE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_NO_COMPUTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_NO_CSE_INVOKE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_NO_GPO_FILTER: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_NO_USER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FLAG_PLANNING_MODE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPC_BLOCK_POLICY: u32 = 1u32;
pub const GPM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4117317384, data2: 35070, data3: 19253, data4: [186, 191, 229, 97, 98, 213, 251, 200] };
pub const GPMAsyncCancel: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 925341353, data2: 30444, data3: 18333, data4: [173, 108, 85, 99, 24, 237, 95, 157] };
pub const GPMBackup: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3977925816, data2: 24314, data3: 18474, data4: [147, 192, 138, 216, 111, 13, 104, 195] };
pub const GPMBackupCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3952018267, data2: 28891, data3: 19103, data4: [150, 118, 55, 194, 89, 148, 233, 220] };
pub const GPMBackupDir: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4242843037, data2: 3873, data3: 19194, data4: [184, 89, 230, 208, 198, 44, 209, 12] };
pub const GPMBackupDirEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3904936074, data2: 52995, data3: 19547, data4: [139, 226, 42, 169, 173, 50, 170, 218] };
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GPMBackupType = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeGPO: GPMBackupType = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeStarterGPO: GPMBackupType = 1i32;
pub const GPMCSECollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3482499112, data2: 11588, data3: 19297, data4: [177, 10, 179, 39, 175, 212, 45, 168] };
pub const GPMClientSideExtension: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3248678670, data2: 26012, data3: 19226, data4: [148, 11, 248, 139, 10, 249, 200, 164] };
pub const GPMConstants: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 945154176, data2: 52638, data3: 19724, data4: [158, 175, 21, 121, 40, 58, 24, 136] };
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GPMDestinationOption = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opDestinationSameAsSource: GPMDestinationOption = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opDestinationNone: GPMDestinationOption = 1i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opDestinationByRelativeName: GPMDestinationOption = 2i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opDestinationSet: GPMDestinationOption = 3i32;
pub const GPMDomain: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1896415678, data2: 4176, data3: 19633, data4: [131, 138, 197, 207, 242, 89, 225, 131] };
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GPMEntryType = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeUser: GPMEntryType = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeComputer: GPMEntryType = 1i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeLocalGroup: GPMEntryType = 2i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeGlobalGroup: GPMEntryType = 3i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeUniversalGroup: GPMEntryType = 4i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeUNCPath: GPMEntryType = 5i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeUnknown: GPMEntryType = 6i32;
pub const GPMGPO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3536726420, data2: 22965, data3: 16484, data4: [181, 129, 77, 104, 72, 106, 22, 196] };
pub const GPMGPOCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2047177509, data2: 33581, data3: 19939, data4: [164, 31, 199, 128, 67, 106, 78, 9] };
pub const GPMGPOLink: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3252656256, data2: 21251, data3: 17094, data4: [138, 60, 4, 136, 225, 191, 115, 100] };
pub const GPMGPOLinksCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4142749722, data2: 18853, data3: 18402, data4: [183, 113, 253, 141, 192, 43, 98, 89] };
pub const GPMMapEntry: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2358727251, data2: 21553, data3: 17521, data4: [179, 93, 6, 38, 201, 40, 37, 138] };
pub const GPMMapEntryCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 217537883, data2: 41889, data3: 19541, data4: [180, 254, 158, 20, 156, 65, 246, 109] };
pub const GPMMigrationTable: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1437548611, data2: 10758, data3: 20338, data4: [171, 239, 99, 27, 68, 7, 156, 118] };
pub const GPMPermission: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1483842570, data2: 59840, data3: 18156, data4: [145, 62, 148, 78, 249, 34, 90, 148] };
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GPMPermissionType = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permGPOApply: GPMPermissionType = 65536i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permGPORead: GPMPermissionType = 65792i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permGPOEdit: GPMPermissionType = 65793i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permGPOEditSecurityAndDelete: GPMPermissionType = 65794i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permGPOCustom: GPMPermissionType = 65795i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permWMIFilterEdit: GPMPermissionType = 131072i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permWMIFilterFullControl: GPMPermissionType = 131073i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permWMIFilterCustom: GPMPermissionType = 131074i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMLink: GPMPermissionType = 1835008i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMLogging: GPMPermissionType = 1573120i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMPlanning: GPMPermissionType = 1573376i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMWMICreate: GPMPermissionType = 1049344i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMWMIFullControl: GPMPermissionType = 1049345i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMGPOCreate: GPMPermissionType = 1049600i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permStarterGPORead: GPMPermissionType = 197888i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permStarterGPOEdit: GPMPermissionType = 197889i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permStarterGPOFullControl: GPMPermissionType = 197890i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permStarterGPOCustom: GPMPermissionType = 197891i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const permSOMStarterGPOCreate: GPMPermissionType = 1049856i32;
pub const GPMRSOP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1218120879, data2: 40642, data3: 20151, data4: [145, 245, 182, 247, 29, 67, 218, 140] };
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GPMRSOPMode = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const rsopUnknown: GPMRSOPMode = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const rsopPlanning: GPMRSOPMode = 1i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const rsopLogging: GPMRSOPMode = 2i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GPMReportType = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const repXML: GPMReportType = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const repHTML: GPMReportType = 1i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const repInfraXML: GPMReportType = 2i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const repInfraRefreshXML: GPMReportType = 3i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const repClientHealthXML: GPMReportType = 4i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const repClientHealthRefreshXML: GPMReportType = 5i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GPMReportingOptions = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opReportLegacy: GPMReportingOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opReportComments: GPMReportingOptions = 1i32;
pub const GPMResult: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2450528960, data2: 37511, data3: 16902, data4: [163, 178, 75, 219, 115, 210, 37, 246] };
pub const GPMSOM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 853098412, data2: 17678, data3: 17615, data4: [130, 156, 139, 34, 255, 107, 218, 225] };
pub const GPMSOMCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 616689991, data2: 14112, data3: 20315, data4: [169, 195, 6, 180, 228, 249, 49, 210] };
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GPMSOMType = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const somSite: GPMSOMType = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const somDomain: GPMSOMType = 1i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const somOU: GPMSOMType = 2i32;
pub const GPMSearchCriteria: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 397068838, data2: 23776, data3: 17658, data4: [140, 192, 82, 89, 230, 72, 53, 102] };
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GPMSearchOperation = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opEquals: GPMSearchOperation = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opContains: GPMSearchOperation = 1i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opNotContains: GPMSearchOperation = 2i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const opNotEquals: GPMSearchOperation = 3i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GPMSearchProperty = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoPermissions: GPMSearchProperty = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoEffectivePermissions: GPMSearchProperty = 1i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoDisplayName: GPMSearchProperty = 2i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoWMIFilter: GPMSearchProperty = 3i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoID: GPMSearchProperty = 4i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoComputerExtensions: GPMSearchProperty = 5i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoUserExtensions: GPMSearchProperty = 6i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const somLinks: GPMSearchProperty = 7i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const gpoDomain: GPMSearchProperty = 8i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const backupMostRecent: GPMSearchProperty = 9i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const starterGPOPermissions: GPMSearchProperty = 10i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const starterGPOEffectivePermissions: GPMSearchProperty = 11i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const starterGPODisplayName: GPMSearchProperty = 12i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const starterGPOID: GPMSearchProperty = 13i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const starterGPODomain: GPMSearchProperty = 14i32;
pub const GPMSecurityInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1417305743, data2: 37218, data3: 17686, data4: [164, 223, 157, 219, 150, 134, 216, 70] };
pub const GPMSitesContainer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 580869186, data2: 34092, data3: 19248, data4: [148, 95, 197, 34, 190, 155, 211, 134] };
pub const GPMStarterGPOBackup: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 949895178, data2: 55535, data3: 17755, data4: [168, 97, 95, 156, 163, 74, 106, 2] };
pub const GPMStarterGPOBackupCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3881739677, data2: 6891, data3: 19637, data4: [167, 138, 40, 29, 170, 88, 36, 6] };
pub const GPMStarterGPOCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2197334667, data2: 18874, data3: 17330, data4: [149, 110, 51, 151, 249, 185, 76, 58] };
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GPMStarterGPOType = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeSystem: GPMStarterGPOType = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const typeCustom: GPMStarterGPOType = 1i32;
pub const GPMStatusMessage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1266142356, data2: 53845, data3: 16539, data4: [188, 98, 55, 8, 129, 113, 90, 25] };
pub const GPMStatusMsgCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 673506494, data2: 19404, data3: 19628, data4: [158, 96, 14, 62, 215, 241, 36, 150] };
pub const GPMTemplate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3975271508, data2: 29146, data3: 20015, data4: [168, 192, 129, 133, 70, 89, 17, 217] };
pub const GPMTrustee: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3309989901, data2: 6582, data3: 16913, data4: [188, 176, 232, 226, 71, 94, 71, 30] };
pub const GPMWMIFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1650935256, data2: 3562, data3: 16482, data4: [191, 96, 207, 197, 177, 202, 18, 134] };
pub const GPMWMIFilterCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1960602920, data2: 59424, data3: 18390, data4: [160, 184, 240, 141, 147, 215, 250, 51] };
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPM_DONOTUSE_W2KDC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPM_DONOT_VALIDATEDC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPM_MIGRATIONTABLE_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPM_PROCESS_SECURITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPM_USE_ANYDC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPM_USE_PDC: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GPOBROWSEINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpTitle: ::windows_sys::core::PWSTR,
    pub lpInitialOU: ::windows_sys::core::PWSTR,
    pub lpDSPath: ::windows_sys::core::PWSTR,
    pub dwDSPathSize: u32,
    pub lpName: ::windows_sys::core::PWSTR,
    pub dwNameSize: u32,
    pub gpoType: GROUP_POLICY_OBJECT_TYPE,
    pub gpoHint: GROUP_POLICY_HINT_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GPOBROWSEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GPOBROWSEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_DISABLENEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_INITTOALL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_NOCOMPUTERS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_NODSGPOS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_NOUSERGPOS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_OPENBUTTON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_BROWSE_SENDAPPLYONEDIT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_FLAG_DISABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_FLAG_FORCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_ASYNC_FOREGROUND: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_BACKGROUND: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_FORCED_REFRESH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_LINKTRANSITION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_LOGRSOP_TRANSITION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_MACHINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_NOCHANGES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_SAFEMODE_BOOT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_SLOWLINK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_INFO_FLAG_VERBOSE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GPO_LINK = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPLinkUnknown: GPO_LINK = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPLinkMachine: GPO_LINK = 1i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPLinkSite: GPO_LINK = 2i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPLinkDomain: GPO_LINK = 3i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPLinkOrganizationalUnit: GPO_LINK = 4i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_LIST_FLAG_MACHINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_LIST_FLAG_NO_SECURITYFILTERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_LIST_FLAG_NO_WMIFILTERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_LIST_FLAG_SITEONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_OPEN_LOAD_REGISTRY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_OPEN_READ_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_OPTION_DISABLE_MACHINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_OPTION_DISABLE_USER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_SECTION_MACHINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_SECTION_ROOT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPO_SECTION_USER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_DLLNAME: &str = "DllName";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_ENABLEASYNCHRONOUSPROCESSING: &str = "EnableAsynchronousProcessing";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_MAXNOGPOLISTCHANGESINTERVAL: &str = "MaxNoGPOListChangesInterval";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_NOBACKGROUNDPOLICY: &str = "NoBackgroundPolicy";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_NOGPOLISTCHANGES: &str = "NoGPOListChanges";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_NOMACHINEPOLICY: &str = "NoMachinePolicy";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_NOSLOWLINK: &str = "NoSlowLink";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_NOTIFYLINKTRANSITION: &str = "NotifyLinkTransition";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_NOUSERPOLICY: &str = "NoUserPolicy";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_PERUSERLOCALSETTINGS: &str = "PerUserLocalSettings";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_PROCESSGROUPPOLICY: &str = "ProcessGroupPolicy";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GP_REQUIRESSUCCESSFULREGISTRY: &str = "RequiresSuccessfulRegistry";
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GROUP_POLICY_HINT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPHintUnknown: GROUP_POLICY_HINT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPHintMachine: GROUP_POLICY_HINT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPHintSite: GROUP_POLICY_HINT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPHintDomain: GROUP_POLICY_HINT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPHintOrganizationalUnit: GROUP_POLICY_HINT_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_POLICY_OBJECTA {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: ::windows_sys::core::PSTR,
    pub lpFileSysPath: ::windows_sys::core::PSTR,
    pub lpDisplayName: ::windows_sys::core::PSTR,
    pub szGPOName: [super::super::Foundation::CHAR; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTA,
    pub pPrev: *mut GROUP_POLICY_OBJECTA,
    pub lpExtensions: ::windows_sys::core::PSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: ::windows_sys::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_POLICY_OBJECTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_POLICY_OBJECTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_POLICY_OBJECTW {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: ::windows_sys::core::PWSTR,
    pub lpFileSysPath: ::windows_sys::core::PWSTR,
    pub lpDisplayName: ::windows_sys::core::PWSTR,
    pub szGPOName: [u16; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTW,
    pub pPrev: *mut GROUP_POLICY_OBJECTW,
    pub lpExtensions: ::windows_sys::core::PWSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_POLICY_OBJECTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_POLICY_OBJECTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type GROUP_POLICY_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPOTypeLocal: GROUP_POLICY_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPOTypeRemote: GROUP_POLICY_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPOTypeDS: GROUP_POLICY_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPOTypeLocalUser: GROUP_POLICY_OBJECT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const GPOTypeLocalGroup: GROUP_POLICY_OBJECT_TYPE = 4i32;
#[repr(C)]
pub struct IGPEInformation {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PWSTR, cchmaxlength: i32) -> ::windows_sys::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PWSTR, cchmaxlength: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Registry")]
    pub GetRegistryKey: unsafe extern "system" fn(this: *mut *mut Self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    GetRegistryKey: usize,
    pub GetDSPath: unsafe extern "system" fn(this: *mut *mut Self, dwsection: u32, pszpath: ::windows_sys::core::PWSTR, cchmaxpath: i32) -> ::windows_sys::core::HRESULT,
    pub GetFileSysPath: unsafe extern "system" fn(this: *mut *mut Self, dwsection: u32, pszpath: ::windows_sys::core::PWSTR, cchmaxpath: i32) -> ::windows_sys::core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(this: *mut *mut Self, dwoptions: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_sys::core::HRESULT,
    pub GetHint: unsafe extern "system" fn(this: *mut *mut Self, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PolicyChanged: unsafe extern "system" fn(this: *mut *mut Self, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows_sys::core::GUID, pguidsnapin: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PolicyChanged: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPM {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetDomain: unsafe extern "system" fn(this: *mut *mut Self, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, pigpmdomain: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetDomain: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetBackupDir: unsafe extern "system" fn(this: *mut *mut Self, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pigpmbackupdir: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetBackupDir: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetSitesContainer: unsafe extern "system" fn(this: *mut *mut Self, bstrforest: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, ppigpmsitescontainer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetSitesContainer: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetRSOP: unsafe extern "system" fn(this: *mut *mut Self, gpmrsopmode: GPMRSOPMode, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, ppigpmrsop: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetRSOP: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePermission: unsafe extern "system" fn(this: *mut *mut Self, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, perm: GPMPermissionType, binheritable: i16, ppperm: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePermission: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSearchCriteria: unsafe extern "system" fn(this: *mut *mut Self, ppigpmsearchcriteria: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSearchCriteria: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateTrustee: unsafe extern "system" fn(this: *mut *mut Self, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmtrustee: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateTrustee: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClientSideExtensions: unsafe extern "system" fn(this: *mut *mut Self, ppigpmcsecollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClientSideExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetConstants: unsafe extern "system" fn(this: *mut *mut Self, ppigpmconstants: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetConstants: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetMigrationTable: unsafe extern "system" fn(this: *mut *mut Self, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmigrationtable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetMigrationTable: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateMigrationTable: unsafe extern "system" fn(this: *mut *mut Self, ppmigrationtable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateMigrationTable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeReporting: unsafe extern "system" fn(this: *mut *mut Self, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeReporting: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPM2 {
    pub base__: IGPM,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetBackupDirEx: unsafe extern "system" fn(this: *mut *mut Self, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backupdirtype: GPMBackupType, ppigpmbackupdirex: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetBackupDirEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeReportingEx: unsafe extern "system" fn(this: *mut *mut Self, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, reportingoptions: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeReportingEx: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMAsyncCancel {
    pub base__: super::Com::IDispatch,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMAsyncProgress {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows_sys::core::HRESULT, presult: *const super::Com::VARIANT, ppigpmstatusmsgcollection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Status: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMBackup {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ID: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GPOID: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GPOID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GPODomain: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GPODomain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GPODisplayName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GPODisplayName: usize,
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Comment: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Comment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BackupDir: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BackupDir: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut *mut Self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GenerateReport: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut *mut Self, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GenerateReportToFile: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMBackupCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppigpmbackup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMBackupDir {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub BackupDirectory: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BackupDirectory: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetBackup: unsafe extern "system" fn(this: *mut *mut Self, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbackup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetBackup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchBackups: unsafe extern "system" fn(this: *mut *mut Self, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmbackupcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchBackups: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMBackupDirEx {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub BackupDir: unsafe extern "system" fn(this: *mut *mut Self, pbstrbackupdir: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BackupDir: usize,
    pub BackupType: unsafe extern "system" fn(this: *mut *mut Self, pgpmbackuptype: *mut GPMBackupType) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetBackup: unsafe extern "system" fn(this: *mut *mut Self, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarbackup: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetBackup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SearchBackups: unsafe extern "system" fn(this: *mut *mut Self, pigpmsearchcriteria: *mut ::core::ffi::c_void, pvarbackupcollection: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SearchBackups: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMCSECollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppigpmcses: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMClientSideExtension {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ID: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    pub IsUserEnabled: unsafe extern "system" fn(this: *mut *mut Self, pvbenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsComputerEnabled: unsafe extern "system" fn(this: *mut *mut Self, pvbenabled: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMConstants {
    pub base__: super::Com::IDispatch,
    pub PermGPOApply: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermGPORead: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermGPOEdit: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermGPOEditSecurityAndDelete: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermGPOCustom: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermWMIFilterEdit: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermWMIFilterFullControl: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermWMIFilterCustom: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermSOMLink: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermSOMLogging: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermSOMPlanning: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermSOMGPOCreate: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermSOMWMICreate: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermSOMWMIFullControl: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyGPOPermissions: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyGPOEffectivePermissions: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyGPODisplayName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyGPOWMIFilter: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyGPOID: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyGPOComputerExtensions: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyGPOUserExtensions: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchPropertySOMLinks: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyGPODomain: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyBackupMostRecent: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchOpEquals: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchOperation) -> ::windows_sys::core::HRESULT,
    pub SearchOpContains: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchOperation) -> ::windows_sys::core::HRESULT,
    pub SearchOpNotContains: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchOperation) -> ::windows_sys::core::HRESULT,
    pub SearchOpNotEquals: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchOperation) -> ::windows_sys::core::HRESULT,
    pub UsePDC: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub UseAnyDC: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DoNotUseW2KDC: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SOMSite: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSOMType) -> ::windows_sys::core::HRESULT,
    pub SOMDomain: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSOMType) -> ::windows_sys::core::HRESULT,
    pub SOMOU: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSOMType) -> ::windows_sys::core::HRESULT,
    pub get_SecurityFlags: unsafe extern "system" fn(this: *mut *mut Self, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DoNotValidateDC: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ReportHTML: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMReportType) -> ::windows_sys::core::HRESULT,
    pub ReportXML: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMReportType) -> ::windows_sys::core::HRESULT,
    pub RSOPModeUnknown: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMRSOPMode) -> ::windows_sys::core::HRESULT,
    pub RSOPModePlanning: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMRSOPMode) -> ::windows_sys::core::HRESULT,
    pub RSOPModeLogging: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMRSOPMode) -> ::windows_sys::core::HRESULT,
    pub EntryTypeUser: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMEntryType) -> ::windows_sys::core::HRESULT,
    pub EntryTypeComputer: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMEntryType) -> ::windows_sys::core::HRESULT,
    pub EntryTypeLocalGroup: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMEntryType) -> ::windows_sys::core::HRESULT,
    pub EntryTypeGlobalGroup: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMEntryType) -> ::windows_sys::core::HRESULT,
    pub EntryTypeUniversalGroup: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMEntryType) -> ::windows_sys::core::HRESULT,
    pub EntryTypeUNCPath: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMEntryType) -> ::windows_sys::core::HRESULT,
    pub EntryTypeUnknown: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMEntryType) -> ::windows_sys::core::HRESULT,
    pub DestinationOptionSameAsSource: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMDestinationOption) -> ::windows_sys::core::HRESULT,
    pub DestinationOptionNone: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMDestinationOption) -> ::windows_sys::core::HRESULT,
    pub DestinationOptionByRelativeName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMDestinationOption) -> ::windows_sys::core::HRESULT,
    pub DestinationOptionSet: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMDestinationOption) -> ::windows_sys::core::HRESULT,
    pub MigrationTableOnly: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ProcessSecurity: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RsopLoggingNoComputer: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RsopLoggingNoUser: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RsopPlanningAssumeSlowLink: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub get_RsopPlanningLoopbackOption: unsafe extern "system" fn(this: *mut *mut Self, vbmerge: i16, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RsopPlanningAssumeUserWQLFilterTrue: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RsopPlanningAssumeCompWQLFilterTrue: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMConstants2 {
    pub base__: IGPMConstants,
    pub BackupTypeGPO: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMBackupType) -> ::windows_sys::core::HRESULT,
    pub BackupTypeStarterGPO: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMBackupType) -> ::windows_sys::core::HRESULT,
    pub StarterGPOTypeSystem: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMStarterGPOType) -> ::windows_sys::core::HRESULT,
    pub StarterGPOTypeCustom: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMStarterGPOType) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyStarterGPOPermissions: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyStarterGPOEffectivePermissions: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyStarterGPODisplayName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyStarterGPOID: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub SearchPropertyStarterGPODomain: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSearchProperty) -> ::windows_sys::core::HRESULT,
    pub PermStarterGPORead: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermStarterGPOEdit: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermStarterGPOFullControl: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub PermStarterGPOCustom: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    pub ReportLegacy: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMReportingOptions) -> ::windows_sys::core::HRESULT,
    pub ReportComments: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMReportingOptions) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMDomain {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub DomainController: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DomainController: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Domain: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Domain: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateGPO: unsafe extern "system" fn(this: *mut *mut Self, ppnewgpo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateGPO: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetGPO: unsafe extern "system" fn(this: *mut *mut Self, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgpo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetGPO: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchGPOs: unsafe extern "system" fn(this: *mut *mut Self, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmgpocollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchGPOs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RestoreGPO: unsafe extern "system" fn(this: *mut *mut Self, pigpmbackup: *mut ::core::ffi::c_void, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RestoreGPO: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetSOM: unsafe extern "system" fn(this: *mut *mut Self, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetSOM: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchSOMs: unsafe extern "system" fn(this: *mut *mut Self, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmsomcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchSOMs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetWMIFilter: unsafe extern "system" fn(this: *mut *mut Self, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwmifilter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetWMIFilter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchWMIFilters: unsafe extern "system" fn(this: *mut *mut Self, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmwmifiltercollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchWMIFilters: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMDomain2 {
    pub base__: IGPMDomain,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStarterGPO: unsafe extern "system" fn(this: *mut *mut Self, ppnewtemplate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStarterGPO: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateGPOFromStarterGPO: unsafe extern "system" fn(this: *mut *mut Self, pgpotemplate: *mut ::core::ffi::c_void, ppnewgpo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateGPOFromStarterGPO: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetStarterGPO: unsafe extern "system" fn(this: *mut *mut Self, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptemplate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetStarterGPO: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchStarterGPOs: unsafe extern "system" fn(this: *mut *mut Self, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmtemplatecollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchStarterGPOs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LoadStarterGPO: unsafe extern "system" fn(this: *mut *mut Self, bstrloadfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LoadStarterGPO: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RestoreStarterGPO: unsafe extern "system" fn(this: *mut *mut Self, pigpmtmplbackup: *mut ::core::ffi::c_void, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RestoreStarterGPO: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMDomain3 {
    pub base__: IGPMDomain2,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut *mut Self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GenerateReport: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InfrastructureDC: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InfrastructureDC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInfrastructureDC: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInfrastructureDC: usize,
    pub SetInfrastructureFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMGPO {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ID: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DomainName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DomainName: usize,
    pub CreationTime: unsafe extern "system" fn(this: *mut *mut Self, pdate: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ModificationTime: unsafe extern "system" fn(this: *mut *mut Self, pdate: *mut f64) -> ::windows_sys::core::HRESULT,
    pub UserDSVersionNumber: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ComputerDSVersionNumber: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub UserSysvolVersionNumber: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ComputerSysvolVersionNumber: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWMIFilter: unsafe extern "system" fn(this: *mut *mut Self, ppigpmwmifilter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWMIFilter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWMIFilter: unsafe extern "system" fn(this: *mut *mut Self, pigpmwmifilter: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWMIFilter: usize,
    pub SetUserEnabled: unsafe extern "system" fn(this: *mut *mut Self, vbenabled: i16) -> ::windows_sys::core::HRESULT,
    pub SetComputerEnabled: unsafe extern "system" fn(this: *mut *mut Self, vbenabled: i16) -> ::windows_sys::core::HRESULT,
    pub IsUserEnabled: unsafe extern "system" fn(this: *mut *mut Self, pvbenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsComputerEnabled: unsafe extern "system" fn(this: *mut *mut Self, pvbenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut *mut Self, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut *mut Self, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityInfo: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Backup: unsafe extern "system" fn(this: *mut *mut Self, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Backup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Import: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, pigpmbackup: *mut ::core::ffi::c_void, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Import: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut *mut Self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GenerateReport: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut *mut Self, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GenerateReportToFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CopyTo: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, pigpmdomain: *mut ::core::ffi::c_void, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CopyTo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, psd: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, ppsd: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityDescriptor: usize,
    pub IsACLConsistent: unsafe extern "system" fn(this: *mut *mut Self, pvbconsistent: *mut i16) -> ::windows_sys::core::HRESULT,
    pub MakeACLConsistent: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMGPO2 {
    pub base__: IGPMGPO,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMGPO3 {
    pub base__: IGPMGPO2,
    #[cfg(feature = "Win32_Foundation")]
    pub InfrastructureDC: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InfrastructureDC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInfrastructureDC: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInfrastructureDC: usize,
    pub SetInfrastructureFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMGPOCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppigpmgpos: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMGPOLink {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub GPOID: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GPOID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GPODomain: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GPODomain: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    pub Enforced: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnforced: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    pub SOMLinkOrder: unsafe extern "system" fn(this: *mut *mut Self, lval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SOM: unsafe extern "system" fn(this: *mut *mut Self, ppigpmsom: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SOM: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMGPOLinksCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppigpmlinks: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMMapEntry {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Source: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Destination: unsafe extern "system" fn(this: *mut *mut Self, pbstrdestination: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Destination: usize,
    pub DestinationOption: unsafe extern "system" fn(this: *mut *mut Self, pgpmdestoption: *mut GPMDestinationOption) -> ::windows_sys::core::HRESULT,
    pub EntryType: unsafe extern "system" fn(this: *mut *mut Self, pgpmentrytype: *mut GPMEntryType) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMMapEntryCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMMigrationTable {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut *mut Self, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, var: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddEntry: unsafe extern "system" fn(this: *mut *mut Self, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, gpmentrytype: GPMEntryType, pvardestination: *const super::Com::VARIANT, ppentry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddEntry: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetEntry: unsafe extern "system" fn(this: *mut *mut Self, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppentry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetEntry: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteEntry: unsafe extern "system" fn(this: *mut *mut Self, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteEntry: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub UpdateDestination: unsafe extern "system" fn(this: *mut *mut Self, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardestination: *const super::Com::VARIANT, ppentry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    UpdateDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Validate: unsafe extern "system" fn(this: *mut *mut Self, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Validate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEntries: unsafe extern "system" fn(this: *mut *mut Self, ppentries: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEntries: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMPermission {
    pub base__: super::Com::IDispatch,
    pub Inherited: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Inheritable: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Denied: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Permission: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMPermissionType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Trustee: unsafe extern "system" fn(this: *mut *mut Self, ppigpmtrustee: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Trustee: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMRSOP {
    pub base__: super::Com::IDispatch,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMRSOPMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Namespace: unsafe extern "system" fn(this: *mut *mut Self, bstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Namespace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLoggingComputer: unsafe extern "system" fn(this: *mut *mut Self, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLoggingComputer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LoggingComputer: unsafe extern "system" fn(this: *mut *mut Self, bstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoggingComputer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLoggingUser: unsafe extern "system" fn(this: *mut *mut Self, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLoggingUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LoggingUser: unsafe extern "system" fn(this: *mut *mut Self, bstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoggingUser: usize,
    pub SetLoggingFlags: unsafe extern "system" fn(this: *mut *mut Self, lval: i32) -> ::windows_sys::core::HRESULT,
    pub LoggingFlags: unsafe extern "system" fn(this: *mut *mut Self, lval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPlanningFlags: unsafe extern "system" fn(this: *mut *mut Self, lval: i32) -> ::windows_sys::core::HRESULT,
    pub PlanningFlags: unsafe extern "system" fn(this: *mut *mut Self, lval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPlanningDomainController: unsafe extern "system" fn(this: *mut *mut Self, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPlanningDomainController: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PlanningDomainController: unsafe extern "system" fn(this: *mut *mut Self, bstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlanningDomainController: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPlanningSiteName: unsafe extern "system" fn(this: *mut *mut Self, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPlanningSiteName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PlanningSiteName: unsafe extern "system" fn(this: *mut *mut Self, bstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlanningSiteName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPlanningUser: unsafe extern "system" fn(this: *mut *mut Self, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPlanningUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PlanningUser: unsafe extern "system" fn(this: *mut *mut Self, bstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlanningUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPlanningUserSOM: unsafe extern "system" fn(this: *mut *mut Self, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPlanningUserSOM: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PlanningUserSOM: unsafe extern "system" fn(this: *mut *mut Self, bstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlanningUserSOM: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPlanningUserWMIFilters: unsafe extern "system" fn(this: *mut *mut Self, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPlanningUserWMIFilters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PlanningUserWMIFilters: unsafe extern "system" fn(this: *mut *mut Self, varval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PlanningUserWMIFilters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPlanningUserSecurityGroups: unsafe extern "system" fn(this: *mut *mut Self, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPlanningUserSecurityGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PlanningUserSecurityGroups: unsafe extern "system" fn(this: *mut *mut Self, varval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PlanningUserSecurityGroups: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPlanningComputer: unsafe extern "system" fn(this: *mut *mut Self, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPlanningComputer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PlanningComputer: unsafe extern "system" fn(this: *mut *mut Self, bstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlanningComputer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPlanningComputerSOM: unsafe extern "system" fn(this: *mut *mut Self, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPlanningComputerSOM: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PlanningComputerSOM: unsafe extern "system" fn(this: *mut *mut Self, bstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlanningComputerSOM: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPlanningComputerWMIFilters: unsafe extern "system" fn(this: *mut *mut Self, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPlanningComputerWMIFilters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PlanningComputerWMIFilters: unsafe extern "system" fn(this: *mut *mut Self, varval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PlanningComputerWMIFilters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPlanningComputerSecurityGroups: unsafe extern "system" fn(this: *mut *mut Self, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPlanningComputerSecurityGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PlanningComputerSecurityGroups: unsafe extern "system" fn(this: *mut *mut Self, varval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PlanningComputerSecurityGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LoggingEnumerateUsers: unsafe extern "system" fn(this: *mut *mut Self, varval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LoggingEnumerateUsers: usize,
    pub CreateQueryResults: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReleaseQueryResults: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut *mut Self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GenerateReport: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut *mut Self, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GenerateReportToFile: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMResult {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, ppigpmstatusmsgcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Result: usize,
    pub OverallStatus: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMSOM {
    pub base__: super::Com::IDispatch,
    pub GPOInheritanceBlocked: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetGPOInheritanceBlocked: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateGPOLink: unsafe extern "system" fn(this: *mut *mut Self, llinkpos: i32, pgpo: *mut ::core::ffi::c_void, ppnewgpolink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateGPOLink: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMSOMType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetGPOLinks: unsafe extern "system" fn(this: *mut *mut Self, ppgpolinks: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetGPOLinks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInheritedGPOLinks: unsafe extern "system" fn(this: *mut *mut Self, ppgpolinks: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInheritedGPOLinks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut *mut Self, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut *mut Self, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityInfo: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMSOMCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppigpmsom: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMSearchCriteria {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMSecurityInfo {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pperm: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, pperm: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Remove: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveTrustee: unsafe extern "system" fn(this: *mut *mut Self, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveTrustee: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMSitesContainer {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub DomainController: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DomainController: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Domain: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Domain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Forest: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Forest: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetSite: unsafe extern "system" fn(this: *mut *mut Self, bstrsitename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetSite: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchSites: unsafe extern "system" fn(this: *mut *mut Self, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmsomcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchSites: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMStarterGPO {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Author: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Author: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Product: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Product: usize,
    pub CreationTime: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ID: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ID: usize,
    pub ModifiedTime: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut GPMStarterGPOType) -> ::windows_sys::core::HRESULT,
    pub ComputerVersion: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut u16) -> ::windows_sys::core::HRESULT,
    pub UserVersion: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StarterGPOVersion: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StarterGPOVersion: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Save: unsafe extern "system" fn(this: *mut *mut Self, bstrsavefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, bsaveassystem: i16, bstrlanguage: *const super::Com::VARIANT, bstrauthor: *const super::Com::VARIANT, bstrproduct: *const super::Com::VARIANT, bstruniqueid: *const super::Com::VARIANT, bstrversion: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Save: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Backup: unsafe extern "system" fn(this: *mut *mut Self, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Backup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CopyTo: unsafe extern "system" fn(this: *mut *mut Self, pvarnewdisplayname: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CopyTo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut *mut Self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GenerateReport: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut *mut Self, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GenerateReportToFile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut *mut Self, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut *mut Self, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityInfo: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMStarterGPOBackup {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub BackupDir: unsafe extern "system" fn(this: *mut *mut Self, pbstrbackupdir: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BackupDir: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Comment: unsafe extern "system" fn(this: *mut *mut Self, pbstrcomment: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Comment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, pbstrdisplayname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Domain: unsafe extern "system" fn(this: *mut *mut Self, pbstrtemplatedomain: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Domain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StarterGPOID: unsafe extern "system" fn(this: *mut *mut Self, pbstrtemplateid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StarterGPOID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ID: unsafe extern "system" fn(this: *mut *mut Self, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ID: usize,
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, ptimestamp: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut GPMStarterGPOType) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut *mut Self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GenerateReport: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut *mut Self, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GenerateReportToFile: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMStarterGPOBackupCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppigpmtmplbackup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMStarterGPOCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppigpmtemplates: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMStatusMessage {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ObjectPath: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ObjectPath: usize,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtensionName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtensionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SettingsName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SettingsName: usize,
    pub OperationCode: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Message: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMStatusMsgCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMTrustee {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub TrusteeSid: unsafe extern "system" fn(this: *mut *mut Self, bstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TrusteeSid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TrusteeName: unsafe extern "system" fn(this: *mut *mut Self, bstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TrusteeName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TrusteeDomain: unsafe extern "system" fn(this: *mut *mut Self, bstrval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TrusteeDomain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TrusteeDSPath: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TrusteeDSPath: usize,
    pub TrusteeType: unsafe extern "system" fn(this: *mut *mut Self, lval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMWMIFilter {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetQueryList: unsafe extern "system" fn(this: *mut *mut Self, pqrylist: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetQueryList: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut *mut Self, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut *mut Self, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityInfo: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGPMWMIFilterCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[repr(C)]
pub struct IGroupPolicyObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub New: unsafe extern "system" fn(this: *mut *mut Self, pszdomainname: ::windows_sys::core::PCWSTR, pszdisplayname: ::windows_sys::core::PCWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub OpenDSGPO: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub OpenLocalMachineGPO: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub OpenRemoteMachineGPO: unsafe extern "system" fn(this: *mut *mut Self, pszcomputername: ::windows_sys::core::PCWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut *mut Self, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows_sys::core::GUID, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PWSTR, cchmaxlength: i32) -> ::windows_sys::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PWSTR, cchmaxlength: i32) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PWSTR, cchmaxlength: i32) -> ::windows_sys::core::HRESULT,
    pub GetDSPath: unsafe extern "system" fn(this: *mut *mut Self, dwsection: u32, pszpath: ::windows_sys::core::PWSTR, cchmaxpath: i32) -> ::windows_sys::core::HRESULT,
    pub GetFileSysPath: unsafe extern "system" fn(this: *mut *mut Self, dwsection: u32, pszpath: ::windows_sys::core::PWSTR, cchmaxpath: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Registry")]
    pub GetRegistryKey: unsafe extern "system" fn(this: *mut *mut Self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    GetRegistryKey: usize,
    pub GetOptions: unsafe extern "system" fn(this: *mut *mut Self, dwoptions: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(this: *mut *mut Self, dwoptions: u32, dwmask: u32) -> ::windows_sys::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_sys::core::HRESULT,
    pub GetMachineName: unsafe extern "system" fn(this: *mut *mut Self, pszname: ::windows_sys::core::PWSTR, cchmaxlength: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Controls")]
    pub GetPropertySheetPages: unsafe extern "system" fn(this: *mut *mut Self, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    GetPropertySheetPages: usize,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub struct INSTALLDATA {
    pub Type: INSTALLSPECTYPE,
    pub Spec: INSTALLSPEC,
}
impl ::core::marker::Copy for INSTALLDATA {}
impl ::core::clone::Clone for INSTALLDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub union INSTALLSPEC {
    pub AppName: INSTALLSPEC_0,
    pub FileExt: ::windows_sys::core::PWSTR,
    pub ProgId: ::windows_sys::core::PWSTR,
    pub COMClass: INSTALLSPEC_1,
}
impl ::core::marker::Copy for INSTALLSPEC {}
impl ::core::clone::Clone for INSTALLSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub struct INSTALLSPEC_0 {
    pub Name: ::windows_sys::core::PWSTR,
    pub GPOId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for INSTALLSPEC_0 {}
impl ::core::clone::Clone for INSTALLSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub struct INSTALLSPEC_1 {
    pub Clsid: ::windows_sys::core::GUID,
    pub ClsCtx: u32,
}
impl ::core::marker::Copy for INSTALLSPEC_1 {}
impl ::core::clone::Clone for INSTALLSPEC_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type INSTALLSPECTYPE = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const APPNAME: INSTALLSPECTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const FILEEXT: INSTALLSPECTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PROGID: INSTALLSPECTYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const COMCLASS: INSTALLSPECTYPE = 4i32;
#[repr(C)]
pub struct IRSOPInformation {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetNamespace: unsafe extern "system" fn(this: *mut *mut Self, dwsection: u32, pszname: ::windows_sys::core::PWSTR, cchmaxlength: i32) -> ::windows_sys::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetEventLogEntryText: unsafe extern "system" fn(this: *mut *mut Self, pszeventsource: ::windows_sys::core::PCWSTR, pszeventlogname: ::windows_sys::core::PCWSTR, pszeventtime: ::windows_sys::core::PCWSTR, dweventid: u32, ppsztext: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub struct LOCALMANAGEDAPPLICATION {
    pub pszDeploymentName: ::windows_sys::core::PWSTR,
    pub pszPolicyName: ::windows_sys::core::PWSTR,
    pub pszProductId: ::windows_sys::core::PWSTR,
    pub dwState: u32,
}
impl ::core::marker::Copy for LOCALMANAGEDAPPLICATION {}
impl ::core::clone::Clone for LOCALMANAGEDAPPLICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_ASSIGNED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_ORPHANED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_POLICYREMOVE_ORPHAN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_POLICYREMOVE_UNINSTALL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_PUBLISHED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_UNINSTALLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const LOCALSTATE_UNINSTALL_UNMANAGED: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MANAGEDAPPLICATION {
    pub pszPackageName: ::windows_sys::core::PWSTR,
    pub pszPublisher: ::windows_sys::core::PWSTR,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwRevision: u32,
    pub GpoId: ::windows_sys::core::GUID,
    pub pszPolicyName: ::windows_sys::core::PWSTR,
    pub ProductId: ::windows_sys::core::GUID,
    pub Language: u16,
    pub pszOwner: ::windows_sys::core::PWSTR,
    pub pszCompany: ::windows_sys::core::PWSTR,
    pub pszComments: ::windows_sys::core::PWSTR,
    pub pszContact: ::windows_sys::core::PWSTR,
    pub pszSupportUrl: ::windows_sys::core::PWSTR,
    pub dwPathType: u32,
    pub bInstalled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MANAGEDAPPLICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MANAGEDAPPLICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const MANAGED_APPS_FROMCATEGORY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const MANAGED_APPS_INFOLEVEL_DEFAULT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const MANAGED_APPS_USERAPPLICATIONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const MANAGED_APPTYPE_SETUPEXE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const MANAGED_APPTYPE_UNSUPPORTED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const MANAGED_APPTYPE_WINDOWSINSTALLER: u32 = 1u32;
pub const NODEID_Machine: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2411771703, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const NODEID_MachineSWSettings: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2411771706, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const NODEID_RSOPMachine: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3175881262, data2: 2938, data3: 19042, data4: [166, 176, 192, 87, 117, 57, 201, 126] };
pub const NODEID_RSOPMachineSWSettings: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1786128190, data2: 60302, data3: 17883, data4: [148, 197, 37, 102, 58, 95, 44, 26] };
pub const NODEID_RSOPUser: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2877765199, data2: 3308, data3: 19672, data4: [155, 248, 137, 143, 52, 98, 143, 184] };
pub const NODEID_RSOPUserSWSettings: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3844889827, data2: 64807, data3: 17410, data4: [132, 222, 217, 165, 242, 133, 137, 16] };
pub const NODEID_User: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2411771704, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const NODEID_UserSWSettings: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2411771708, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Wmi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
pub type PFNGENERATEGROUPPOLICY = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, pbabort: *mut super::super::Foundation::BOOL, pwszsite: ::windows_sys::core::PCWSTR, pcomputertarget: *const RSOP_TARGET, pusertarget: *const RSOP_TARGET) -> u32>;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PFNPROCESSGROUPPOLICY = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK) -> u32>;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`, `\"Win32_System_Wmi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_System_Wmi"))]
pub type PFNPROCESSGROUPPOLICYEX = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK, pwbemservices: *mut *mut super::Wmi::IWbemServices, prsopstatus: *mut ::windows_sys::core::HRESULT) -> u32>;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNSTATUSMESSAGECALLBACK = ::core::option::Option<unsafe extern "system" fn(bverbose: super::super::Foundation::BOOL, lpmessage: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PI_APPLYPOLICY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PI_NOUI: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICYSETTINGSTATUSINFO {
    pub szKey: ::windows_sys::core::PWSTR,
    pub szEventSource: ::windows_sys::core::PWSTR,
    pub szEventLogName: ::windows_sys::core::PWSTR,
    pub dwEventID: u32,
    pub dwErrorCode: u32,
    pub status: SETTINGSTATUS,
    pub timeLogged: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICYSETTINGSTATUSINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICYSETTINGSTATUSINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PT_MANDATORY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PT_ROAMING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PT_ROAMING_PREEXISTING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const PT_TEMPORARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RP_FORCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RP_SYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_COMPUTER_ACCESS_DENIED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_INFO_FLAG_DIAGNOSTIC_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_NO_COMPUTER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_NO_USER: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_PLANNING_ASSUME_COMP_WQLFILTER_TRUE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_PLANNING_ASSUME_LOOPBACK_MERGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_PLANNING_ASSUME_LOOPBACK_REPLACE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_PLANNING_ASSUME_SLOW_LINK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_PLANNING_ASSUME_USER_WQLFILTER_TRUE: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Wmi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
pub struct RSOP_TARGET {
    pub pwszAccountName: ::windows_sys::core::PWSTR,
    pub pwszNewSOM: ::windows_sys::core::PWSTR,
    pub psaSecurityGroups: *mut super::Com::SAFEARRAY,
    pub pRsopToken: *mut ::core::ffi::c_void,
    pub pGPOList: *mut GROUP_POLICY_OBJECTA,
    pub pWbemServices: *mut *mut *mut *mut super::Wmi::IWbemServices,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::marker::Copy for RSOP_TARGET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::clone::Clone for RSOP_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_TEMPNAMESPACE_EXISTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOP_USER_ACCESS_DENIED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub type SETTINGSTATUS = i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOPUnspecified: SETTINGSTATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOPApplied: SETTINGSTATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOPIgnored: SETTINGSTATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOPFailed: SETTINGSTATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_GroupPolicy\"`*"]
pub const RSOPSubsettingFailed: SETTINGSTATUS = 4i32;
