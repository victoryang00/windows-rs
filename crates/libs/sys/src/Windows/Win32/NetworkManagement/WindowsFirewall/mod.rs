#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
    pub fn NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername: ::windows_sys::core::PCWSTR, netisoerror: *mut NETISO_ERROR_TYPE) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationEnumAppContainers(flags: u32, pdwnumpublicappcs: *mut u32, pppublicappcs: *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationFreeAppContainers(ppublicappcs: *const INET_FIREWALL_APP_CONTAINER) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut u32, appcontainersids: *mut *mut super::super::Security::SID_AND_ATTRIBUTES) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationRegisterForAppContainerChanges(flags: u32, callback: PAC_CHANGES_CALLBACK_FN, context: *const ::core::ffi::c_void, registrationobject: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs: u32, appcontainersids: *const super::super::Security::SID_AND_ATTRIBUTES) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetworkIsolationSetupAppContainerBinaries(applicationcontainersid: super::super::Foundation::PSID, packagefullname: ::windows_sys::core::PCWSTR, packagefolder: ::windows_sys::core::PCWSTR, displayname: ::windows_sys::core::PCWSTR, bbinariesfullycomputed: super::super::Foundation::BOOL, binaries: *const ::windows_sys::core::PWSTR, binariescount: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject: super::super::Foundation::HANDLE) -> u32;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type ICS_TARGETTYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const ICSTT_NAME: ICS_TARGETTYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const ICSTT_IPADDRESS: ICS_TARGETTYPE = 1i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDynamicPortMapping {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ExternalIPAddress: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExternalIPAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoteHost: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoteHost: usize,
    pub ExternalPort: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Protocol: usize,
    pub InternalPort: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InternalClient: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InternalClient: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    pub LeaseDuration: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RenewLease: unsafe extern "system" fn(this: *mut *mut Self, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EditInternalClient: unsafe extern "system" fn(this: *mut *mut Self, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EditInternalClient: usize,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self, vb: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EditDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EditDescription: usize,
    pub EditInternalPort: unsafe extern "system" fn(this: *mut *mut Self, linternalport: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDynamicPortMappingCollection {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdpm: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lleaseduration: i32, ppdpm: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
}
#[repr(C)]
pub struct IEnumNetConnection {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumNetSharingEveryConnection {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumNetSharingPortMapping {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumNetSharingPrivateConnection {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumNetSharingPublicConnection {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INATEventManager {
    pub base__: super::super::System::Com::IDispatch,
    pub SetExternalIPAddressCallback: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNumberOfEntriesCallback: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INATExternalIPAddressCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub NewExternalIPAddress: unsafe extern "system" fn(this: *mut *mut Self, bstrnewexternalipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NewExternalIPAddress: usize,
}
#[repr(C)]
pub struct INATNumberOfEntriesCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub NewNumberOfEntries: unsafe extern "system" fn(this: *mut *mut Self, lnewnumberofentries: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub struct INET_FIREWALL_AC_BINARIES {
    pub count: u32,
    pub binaries: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for INET_FIREWALL_AC_BINARIES {}
impl ::core::clone::Clone for INET_FIREWALL_AC_BINARIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct INET_FIREWALL_AC_CAPABILITIES {
    pub count: u32,
    pub capabilities: *mut super::super::Security::SID_AND_ATTRIBUTES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_AC_CAPABILITIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_AC_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct INET_FIREWALL_AC_CHANGE {
    pub changeType: INET_FIREWALL_AC_CHANGE_TYPE,
    pub createType: INET_FIREWALL_AC_CREATION_TYPE,
    pub appContainerSid: *mut super::super::Security::SID,
    pub userSid: *mut super::super::Security::SID,
    pub displayName: ::windows_sys::core::PWSTR,
    pub Anonymous: INET_FIREWALL_AC_CHANGE_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_AC_CHANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_AC_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union INET_FIREWALL_AC_CHANGE_0 {
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_AC_CHANGE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_AC_CHANGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type INET_FIREWALL_AC_CHANGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const INET_FIREWALL_AC_CHANGE_INVALID: INET_FIREWALL_AC_CHANGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const INET_FIREWALL_AC_CHANGE_CREATE: INET_FIREWALL_AC_CHANGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const INET_FIREWALL_AC_CHANGE_DELETE: INET_FIREWALL_AC_CHANGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const INET_FIREWALL_AC_CHANGE_MAX: INET_FIREWALL_AC_CHANGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type INET_FIREWALL_AC_CREATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const INET_FIREWALL_AC_NONE: INET_FIREWALL_AC_CREATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const INET_FIREWALL_AC_PACKAGE_ID_ONLY: INET_FIREWALL_AC_CREATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const INET_FIREWALL_AC_BINARY: INET_FIREWALL_AC_CREATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const INET_FIREWALL_AC_MAX: INET_FIREWALL_AC_CREATION_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct INET_FIREWALL_APP_CONTAINER {
    pub appContainerSid: *mut super::super::Security::SID,
    pub userSid: *mut super::super::Security::SID,
    pub appContainerName: ::windows_sys::core::PWSTR,
    pub displayName: ::windows_sys::core::PWSTR,
    pub description: ::windows_sys::core::PWSTR,
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
    pub workingDirectory: ::windows_sys::core::PWSTR,
    pub packageFullName: ::windows_sys::core::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_APP_CONTAINER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_APP_CONTAINER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct INetConnection {
    pub base__: ::windows_sys::core::IUnknown,
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Duplicate: unsafe extern "system" fn(this: *mut *mut Self, pszwduplicatename: ::windows_sys::core::PCWSTR, ppcon: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(this: *mut *mut Self, ppprops: *mut *mut NETCON_PROPERTIES) -> ::windows_sys::core::HRESULT,
    pub GetUiObjectClassId: unsafe extern "system" fn(this: *mut *mut Self, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut *mut Self, pszwnewname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INetConnectionConnectUi {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetConnection: unsafe extern "system" fn(this: *mut *mut Self, pcon: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Connect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Disconnect: usize,
}
#[repr(C)]
pub struct INetConnectionManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub EnumConnections: unsafe extern "system" fn(this: *mut *mut Self, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetConnectionProps {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Guid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Guid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceName: unsafe extern "system" fn(this: *mut *mut Self, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceName: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut NETCON_STATUS) -> ::windows_sys::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut *mut Self, pmediatype: *mut NETCON_MEDIATYPE) -> ::windows_sys::core::HRESULT,
    pub Characteristics: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwAuthorizedApplication {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProcessImageFileName: unsafe extern "system" fn(this: *mut *mut Self, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProcessImageFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProcessImageFileName: unsafe extern "system" fn(this: *mut *mut Self, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProcessImageFileName: usize,
    pub IpVersion: unsafe extern "system" fn(this: *mut *mut Self, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_sys::core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(this: *mut *mut Self, ipversion: NET_FW_IP_VERSION) -> ::windows_sys::core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut *mut Self, scope: *mut NET_FW_SCOPE) -> ::windows_sys::core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut *mut Self, scope: NET_FW_SCOPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut *mut Self, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoteAddresses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut *mut Self, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRemoteAddresses: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwAuthorizedApplications {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, app: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, app: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwIcmpSettings {
    pub base__: super::super::System::Com::IDispatch,
    pub AllowOutboundDestinationUnreachable: unsafe extern "system" fn(this: *mut *mut Self, allow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowOutboundDestinationUnreachable: unsafe extern "system" fn(this: *mut *mut Self, allow: i16) -> ::windows_sys::core::HRESULT,
    pub AllowRedirect: unsafe extern "system" fn(this: *mut *mut Self, allow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowRedirect: unsafe extern "system" fn(this: *mut *mut Self, allow: i16) -> ::windows_sys::core::HRESULT,
    pub AllowInboundEchoRequest: unsafe extern "system" fn(this: *mut *mut Self, allow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowInboundEchoRequest: unsafe extern "system" fn(this: *mut *mut Self, allow: i16) -> ::windows_sys::core::HRESULT,
    pub AllowOutboundTimeExceeded: unsafe extern "system" fn(this: *mut *mut Self, allow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowOutboundTimeExceeded: unsafe extern "system" fn(this: *mut *mut Self, allow: i16) -> ::windows_sys::core::HRESULT,
    pub AllowOutboundParameterProblem: unsafe extern "system" fn(this: *mut *mut Self, allow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowOutboundParameterProblem: unsafe extern "system" fn(this: *mut *mut Self, allow: i16) -> ::windows_sys::core::HRESULT,
    pub AllowOutboundSourceQuench: unsafe extern "system" fn(this: *mut *mut Self, allow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowOutboundSourceQuench: unsafe extern "system" fn(this: *mut *mut Self, allow: i16) -> ::windows_sys::core::HRESULT,
    pub AllowInboundRouterRequest: unsafe extern "system" fn(this: *mut *mut Self, allow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowInboundRouterRequest: unsafe extern "system" fn(this: *mut *mut Self, allow: i16) -> ::windows_sys::core::HRESULT,
    pub AllowInboundTimestampRequest: unsafe extern "system" fn(this: *mut *mut Self, allow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowInboundTimestampRequest: unsafe extern "system" fn(this: *mut *mut Self, allow: i16) -> ::windows_sys::core::HRESULT,
    pub AllowInboundMaskRequest: unsafe extern "system" fn(this: *mut *mut Self, allow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowInboundMaskRequest: unsafe extern "system" fn(this: *mut *mut Self, allow: i16) -> ::windows_sys::core::HRESULT,
    pub AllowOutboundPacketTooBig: unsafe extern "system" fn(this: *mut *mut Self, allow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAllowOutboundPacketTooBig: unsafe extern "system" fn(this: *mut *mut Self, allow: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwMgr {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub LocalPolicy: unsafe extern "system" fn(this: *mut *mut Self, localpolicy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LocalPolicy: usize,
    pub CurrentProfileType: unsafe extern "system" fn(this: *mut *mut Self, profiletype: *mut NET_FW_PROFILE_TYPE) -> ::windows_sys::core::HRESULT,
    pub RestoreDefaults: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub IsPortAllowed: unsafe extern "system" fn(this: *mut *mut Self, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    IsPortAllowed: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub IsIcmpTypeAllowed: unsafe extern "system" fn(this: *mut *mut Self, ipversion: NET_FW_IP_VERSION, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: u8, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    IsIcmpTypeAllowed: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwOpenPort {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    pub IpVersion: unsafe extern "system" fn(this: *mut *mut Self, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_sys::core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(this: *mut *mut Self, ipversion: NET_FW_IP_VERSION) -> ::windows_sys::core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> ::windows_sys::core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(this: *mut *mut Self, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_sys::core::HRESULT,
    pub Port: unsafe extern "system" fn(this: *mut *mut Self, portnumber: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut *mut Self, portnumber: i32) -> ::windows_sys::core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut *mut Self, scope: *mut NET_FW_SCOPE) -> ::windows_sys::core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut *mut Self, scope: NET_FW_SCOPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut *mut Self, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoteAddresses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut *mut Self, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRemoteAddresses: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    pub BuiltIn: unsafe extern "system" fn(this: *mut *mut Self, builtin: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwOpenPorts {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, port: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwPolicy {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentProfile: unsafe extern "system" fn(this: *mut *mut Self, profile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentProfile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProfileByType: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE, profile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProfileByType: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwPolicy2 {
    pub base__: super::super::System::Com::IDispatch,
    pub CurrentProfileTypes: unsafe extern "system" fn(this: *mut *mut Self, profiletypesbitmask: *mut i32) -> ::windows_sys::core::HRESULT,
    pub get_FirewallEnabled: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub put_FirewallEnabled: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_ExcludedInterfaces: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_ExcludedInterfaces: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub put_ExcludedInterfaces: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    put_ExcludedInterfaces: usize,
    pub get_BlockAllInboundTraffic: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, block: *mut i16) -> ::windows_sys::core::HRESULT,
    pub put_BlockAllInboundTraffic: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows_sys::core::HRESULT,
    pub get_NotificationsDisabled: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub put_NotificationsDisabled: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows_sys::core::HRESULT,
    pub get_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub put_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Rules: unsafe extern "system" fn(this: *mut *mut Self, rules: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ServiceRestriction: unsafe extern "system" fn(this: *mut *mut Self, servicerestriction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ServiceRestriction: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableRuleGroup: unsafe extern "system" fn(this: *mut *mut Self, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enable: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableRuleGroup: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRuleGroupEnabled: unsafe extern "system" fn(this: *mut *mut Self, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRuleGroupEnabled: usize,
    pub RestoreLocalFirewallDefaults: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub get_DefaultInboundAction: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows_sys::core::HRESULT,
    pub put_DefaultInboundAction: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_sys::core::HRESULT,
    pub get_DefaultOutboundAction: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows_sys::core::HRESULT,
    pub put_DefaultOutboundAction: unsafe extern "system" fn(this: *mut *mut Self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_IsRuleGroupCurrentlyEnabled: unsafe extern "system" fn(this: *mut *mut Self, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_IsRuleGroupCurrentlyEnabled: usize,
    pub LocalPolicyModifyState: unsafe extern "system" fn(this: *mut *mut Self, modifystate: *mut NET_FW_MODIFY_STATE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwProduct {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RuleCategories: unsafe extern "system" fn(this: *mut *mut Self, rulecategories: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RuleCategories: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetRuleCategories: unsafe extern "system" fn(this: *mut *mut Self, rulecategories: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetRuleCategories: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, displayname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, displayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PathToSignedProductExe: unsafe extern "system" fn(this: *mut *mut Self, path: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathToSignedProductExe: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwProducts {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Register: unsafe extern "system" fn(this: *mut *mut Self, product: *mut ::core::ffi::c_void, registration: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Register: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, product: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwProfile {
    pub base__: super::super::System::Com::IDispatch,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut NET_FW_PROFILE_TYPE) -> ::windows_sys::core::HRESULT,
    pub FirewallEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetFirewallEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    pub ExceptionsNotAllowed: unsafe extern "system" fn(this: *mut *mut Self, notallowed: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetExceptionsNotAllowed: unsafe extern "system" fn(this: *mut *mut Self, notallowed: i16) -> ::windows_sys::core::HRESULT,
    pub NotificationsDisabled: unsafe extern "system" fn(this: *mut *mut Self, disabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetNotificationsDisabled: unsafe extern "system" fn(this: *mut *mut Self, disabled: i16) -> ::windows_sys::core::HRESULT,
    pub UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(this: *mut *mut Self, disabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(this: *mut *mut Self, disabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoteAdminSettings: unsafe extern "system" fn(this: *mut *mut Self, remoteadminsettings: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoteAdminSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IcmpSettings: unsafe extern "system" fn(this: *mut *mut Self, icmpsettings: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IcmpSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GloballyOpenPorts: unsafe extern "system" fn(this: *mut *mut Self, openports: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GloballyOpenPorts: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Services: unsafe extern "system" fn(this: *mut *mut Self, services: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Services: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AuthorizedApplications: unsafe extern "system" fn(this: *mut *mut Self, apps: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AuthorizedApplications: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwRemoteAdminSettings {
    pub base__: super::super::System::Com::IDispatch,
    pub IpVersion: unsafe extern "system" fn(this: *mut *mut Self, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_sys::core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(this: *mut *mut Self, ipversion: NET_FW_IP_VERSION) -> ::windows_sys::core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut *mut Self, scope: *mut NET_FW_SCOPE) -> ::windows_sys::core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut *mut Self, scope: NET_FW_SCOPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut *mut Self, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoteAddresses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut *mut Self, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRemoteAddresses: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwRule {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, desc: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, desc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplicationName: unsafe extern "system" fn(this: *mut *mut Self, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplicationName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetApplicationName: unsafe extern "system" fn(this: *mut *mut Self, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetApplicationName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceName: unsafe extern "system" fn(this: *mut *mut Self, servicename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceName: unsafe extern "system" fn(this: *mut *mut Self, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceName: usize,
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, protocol: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(this: *mut *mut Self, protocol: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LocalPorts: unsafe extern "system" fn(this: *mut *mut Self, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LocalPorts: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLocalPorts: unsafe extern "system" fn(this: *mut *mut Self, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLocalPorts: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemotePorts: unsafe extern "system" fn(this: *mut *mut Self, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemotePorts: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRemotePorts: unsafe extern "system" fn(this: *mut *mut Self, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRemotePorts: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LocalAddresses: unsafe extern "system" fn(this: *mut *mut Self, localaddrs: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LocalAddresses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLocalAddresses: unsafe extern "system" fn(this: *mut *mut Self, localaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLocalAddresses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut *mut Self, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoteAddresses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut *mut Self, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRemoteAddresses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IcmpTypesAndCodes: unsafe extern "system" fn(this: *mut *mut Self, icmptypesandcodes: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IcmpTypesAndCodes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIcmpTypesAndCodes: unsafe extern "system" fn(this: *mut *mut Self, icmptypesandcodes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIcmpTypesAndCodes: usize,
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows_sys::core::HRESULT,
    pub SetDirection: unsafe extern "system" fn(this: *mut *mut Self, dir: NET_FW_RULE_DIRECTION) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Interfaces: unsafe extern "system" fn(this: *mut *mut Self, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Interfaces: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetInterfaces: unsafe extern "system" fn(this: *mut *mut Self, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetInterfaces: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InterfaceTypes: unsafe extern "system" fn(this: *mut *mut Self, interfacetypes: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InterfaceTypes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInterfaceTypes: unsafe extern "system" fn(this: *mut *mut Self, interfacetypes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInterfaceTypes: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Grouping: unsafe extern "system" fn(this: *mut *mut Self, context: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Grouping: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGrouping: unsafe extern "system" fn(this: *mut *mut Self, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGrouping: usize,
    pub Profiles: unsafe extern "system" fn(this: *mut *mut Self, profiletypesbitmask: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetProfiles: unsafe extern "system" fn(this: *mut *mut Self, profiletypesbitmask: i32) -> ::windows_sys::core::HRESULT,
    pub EdgeTraversal: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEdgeTraversal: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    pub Action: unsafe extern "system" fn(this: *mut *mut Self, action: *mut NET_FW_ACTION) -> ::windows_sys::core::HRESULT,
    pub SetAction: unsafe extern "system" fn(this: *mut *mut Self, action: NET_FW_ACTION) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwRule2 {
    pub base__: INetFwRule,
    pub EdgeTraversalOptions: unsafe extern "system" fn(this: *mut *mut Self, loptions: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetEdgeTraversalOptions: unsafe extern "system" fn(this: *mut *mut Self, loptions: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwRule3 {
    pub base__: INetFwRule2,
    #[cfg(feature = "Win32_Foundation")]
    pub LocalAppPackageId: unsafe extern "system" fn(this: *mut *mut Self, wszpackageid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LocalAppPackageId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLocalAppPackageId: unsafe extern "system" fn(this: *mut *mut Self, wszpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLocalAppPackageId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LocalUserOwner: unsafe extern "system" fn(this: *mut *mut Self, wszuserowner: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LocalUserOwner: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLocalUserOwner: unsafe extern "system" fn(this: *mut *mut Self, wszuserowner: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLocalUserOwner: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LocalUserAuthorizedList: unsafe extern "system" fn(this: *mut *mut Self, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LocalUserAuthorizedList: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLocalUserAuthorizedList: unsafe extern "system" fn(this: *mut *mut Self, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLocalUserAuthorizedList: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoteUserAuthorizedList: unsafe extern "system" fn(this: *mut *mut Self, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoteUserAuthorizedList: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRemoteUserAuthorizedList: unsafe extern "system" fn(this: *mut *mut Self, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRemoteUserAuthorizedList: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoteMachineAuthorizedList: unsafe extern "system" fn(this: *mut *mut Self, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoteMachineAuthorizedList: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRemoteMachineAuthorizedList: unsafe extern "system" fn(this: *mut *mut Self, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRemoteMachineAuthorizedList: usize,
    pub SecureFlags: unsafe extern "system" fn(this: *mut *mut Self, loptions: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSecureFlags: unsafe extern "system" fn(this: *mut *mut Self, loptions: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwRules {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, rule: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwService {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut NET_FW_SERVICE_TYPE) -> ::windows_sys::core::HRESULT,
    pub Customized: unsafe extern "system" fn(this: *mut *mut Self, customized: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IpVersion: unsafe extern "system" fn(this: *mut *mut Self, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_sys::core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(this: *mut *mut Self, ipversion: NET_FW_IP_VERSION) -> ::windows_sys::core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut *mut Self, scope: *mut NET_FW_SCOPE) -> ::windows_sys::core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut *mut Self, scope: NET_FW_SCOPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut *mut Self, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoteAddresses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut *mut Self, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRemoteAddresses: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GloballyOpenPorts: unsafe extern "system" fn(this: *mut *mut Self, openports: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GloballyOpenPorts: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwServiceRestriction {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub RestrictService: unsafe extern "system" fn(this: *mut *mut Self, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, restrictservice: i16, servicesidrestricted: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RestrictService: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceRestricted: unsafe extern "system" fn(this: *mut *mut Self, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, servicerestricted: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceRestricted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Rules: unsafe extern "system" fn(this: *mut *mut Self, rules: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rules: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwServices {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, svctype: NET_FW_SERVICE_TYPE, service: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingConfiguration {
    pub base__: super::super::System::Com::IDispatch,
    pub SharingEnabled: unsafe extern "system" fn(this: *mut *mut Self, pbenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SharingConnectionType: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut SHARINGCONNECTIONTYPE) -> ::windows_sys::core::HRESULT,
    pub DisableSharing: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnableSharing: unsafe extern "system" fn(this: *mut *mut Self, r#type: SHARINGCONNECTIONTYPE) -> ::windows_sys::core::HRESULT,
    pub InternetFirewallEnabled: unsafe extern "system" fn(this: *mut *mut Self, pbenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub DisableInternetFirewall: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnableInternetFirewall: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_EnumPortMappings: unsafe extern "system" fn(this: *mut *mut Self, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_EnumPortMappings: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddPortMapping: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, etargettype: ICS_TARGETTYPE, ppmapping: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddPortMapping: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemovePortMapping: unsafe extern "system" fn(this: *mut *mut Self, pmapping: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemovePortMapping: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingEveryConnectionCollection {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingManager {
    pub base__: super::super::System::Com::IDispatch,
    pub SharingInstalled: unsafe extern "system" fn(this: *mut *mut Self, pbinstalled: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_EnumPublicConnections: unsafe extern "system" fn(this: *mut *mut Self, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_EnumPublicConnections: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_EnumPrivateConnections: unsafe extern "system" fn(this: *mut *mut Self, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_EnumPrivateConnections: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_INetSharingConfigurationForINetConnection: unsafe extern "system" fn(this: *mut *mut Self, pnetconnection: *mut ::core::ffi::c_void, ppnetsharingconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_INetSharingConfigurationForINetConnection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumEveryConnection: unsafe extern "system" fn(this: *mut *mut Self, ppcoll: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumEveryConnection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_NetConnectionProps: unsafe extern "system" fn(this: *mut *mut Self, pnetconnection: *mut ::core::ffi::c_void, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_NetConnectionProps: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingPortMapping {
    pub base__: super::super::System::Com::IDispatch,
    pub Disable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppnspmp: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingPortMappingCollection {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingPortMappingProps {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub IPProtocol: unsafe extern "system" fn(this: *mut *mut Self, pucipprot: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ExternalPort: unsafe extern "system" fn(this: *mut *mut Self, pusport: *mut i32) -> ::windows_sys::core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(this: *mut *mut Self, pusport: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, pdwoptions: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, pbstrtargetname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetIPAddress: unsafe extern "system" fn(this: *mut *mut Self, pbstrtargetipaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetIPAddress: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, pbool: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingPrivateConnectionCollection {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingPublicConnectionCollection {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IStaticPortMapping {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ExternalIPAddress: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExternalIPAddress: usize,
    pub ExternalPort: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Protocol: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InternalClient: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InternalClient: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EditInternalClient: unsafe extern "system" fn(this: *mut *mut Self, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EditInternalClient: usize,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self, vb: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EditDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EditDescription: usize,
    pub EditInternalPort: unsafe extern "system" fn(this: *mut *mut Self, linternalport: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IStaticPortMappingCollection {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPNAT {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub StaticPortMappingCollection: unsafe extern "system" fn(this: *mut *mut Self, ppspms: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    StaticPortMappingCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DynamicPortMappingCollection: unsafe extern "system" fn(this: *mut *mut Self, ppdpms: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DynamicPortMappingCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NATEventManager: unsafe extern "system" fn(this: *mut *mut Self, ppnem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NATEventManager: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NETCONMGR_ENUM_FLAGS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCME_DEFAULT: NETCONMGR_ENUM_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCME_HIDDEN: NETCONMGR_ENUM_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NETCONUI_CONNECT_FLAGS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCUC_DEFAULT: NETCONUI_CONNECT_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCUC_NO_UI: NETCONUI_CONNECT_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCUC_ENABLE_DISABLE: NETCONUI_CONNECT_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NETCON_CHARACTERISTIC_FLAGS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_NONE: NETCON_CHARACTERISTIC_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_ALL_USERS: NETCON_CHARACTERISTIC_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_ALLOW_DUPLICATION: NETCON_CHARACTERISTIC_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_ALLOW_REMOVAL: NETCON_CHARACTERISTIC_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_ALLOW_RENAME: NETCON_CHARACTERISTIC_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_INCOMING_ONLY: NETCON_CHARACTERISTIC_FLAGS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_OUTGOING_ONLY: NETCON_CHARACTERISTIC_FLAGS = 64i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_BRANDED: NETCON_CHARACTERISTIC_FLAGS = 128i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_SHARED: NETCON_CHARACTERISTIC_FLAGS = 256i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_BRIDGED: NETCON_CHARACTERISTIC_FLAGS = 512i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_FIREWALLED: NETCON_CHARACTERISTIC_FLAGS = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_DEFAULT: NETCON_CHARACTERISTIC_FLAGS = 2048i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_HOMENET_CAPABLE: NETCON_CHARACTERISTIC_FLAGS = 4096i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_SHARED_PRIVATE: NETCON_CHARACTERISTIC_FLAGS = 8192i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_QUARANTINED: NETCON_CHARACTERISTIC_FLAGS = 16384i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_RESERVED: NETCON_CHARACTERISTIC_FLAGS = 32768i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_HOSTED_NETWORK: NETCON_CHARACTERISTIC_FLAGS = 65536i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_VIRTUAL_STATION: NETCON_CHARACTERISTIC_FLAGS = 131072i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_WIFI_DIRECT: NETCON_CHARACTERISTIC_FLAGS = 262144i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_BLUETOOTH_MASK: NETCON_CHARACTERISTIC_FLAGS = 983040i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCCF_LAN_MASK: NETCON_CHARACTERISTIC_FLAGS = 15728640i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NETCON_MAX_NAME_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NETCON_MEDIATYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCM_NONE: NETCON_MEDIATYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCM_DIRECT: NETCON_MEDIATYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCM_ISDN: NETCON_MEDIATYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCM_LAN: NETCON_MEDIATYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCM_PHONE: NETCON_MEDIATYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCM_TUNNEL: NETCON_MEDIATYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCM_PPPOE: NETCON_MEDIATYPE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCM_BRIDGE: NETCON_MEDIATYPE = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCM_SHAREDACCESSHOST_LAN: NETCON_MEDIATYPE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCM_SHAREDACCESSHOST_RAS: NETCON_MEDIATYPE = 9i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub struct NETCON_PROPERTIES {
    pub guidId: ::windows_sys::core::GUID,
    pub pszwName: ::windows_sys::core::PWSTR,
    pub pszwDeviceName: ::windows_sys::core::PWSTR,
    pub Status: NETCON_STATUS,
    pub MediaType: NETCON_MEDIATYPE,
    pub dwCharacter: u32,
    pub clsidThisObject: ::windows_sys::core::GUID,
    pub clsidUiObject: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for NETCON_PROPERTIES {}
impl ::core::clone::Clone for NETCON_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NETCON_STATUS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_DISCONNECTED: NETCON_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_CONNECTING: NETCON_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_CONNECTED: NETCON_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_DISCONNECTING: NETCON_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_HARDWARE_NOT_PRESENT: NETCON_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_HARDWARE_DISABLED: NETCON_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_HARDWARE_MALFUNCTION: NETCON_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_MEDIA_DISCONNECTED: NETCON_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_AUTHENTICATING: NETCON_STATUS = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_AUTHENTICATION_SUCCEEDED: NETCON_STATUS = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_AUTHENTICATION_FAILED: NETCON_STATUS = 10i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_INVALID_ADDRESS: NETCON_STATUS = 11i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_CREDENTIALS_REQUIRED: NETCON_STATUS = 12i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_ACTION_REQUIRED: NETCON_STATUS = 13i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_ACTION_REQUIRED_RETRY: NETCON_STATUS = 14i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCS_CONNECT_FAILED: NETCON_STATUS = 15i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NETCON_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCT_DIRECT_CONNECT: NETCON_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCT_INBOUND: NETCON_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCT_INTERNET: NETCON_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCT_LAN: NETCON_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCT_PHONE: NETCON_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCT_TUNNEL: NETCON_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NCT_BRIDGE: NETCON_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NETISO_ERROR_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NETISO_ERROR_TYPE_NONE: NETISO_ERROR_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NETISO_ERROR_TYPE_PRIVATE_NETWORK: NETISO_ERROR_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT: NETISO_ERROR_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT_SERVER: NETISO_ERROR_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NETISO_ERROR_TYPE_MAX: NETISO_ERROR_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NETISO_FLAG = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NETISO_FLAG_FORCE_COMPUTE_BINARIES: NETISO_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NETISO_FLAG_MAX: NETISO_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NETISO_GEID_FOR_WDAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NET_FW_ACTION = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_ACTION_BLOCK: NET_FW_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_ACTION_ALLOW: NET_FW_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_ACTION_MAX: NET_FW_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NET_FW_AUTHENTICATE_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_AUTHENTICATE_NONE: NET_FW_AUTHENTICATE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_AUTHENTICATE_NO_ENCAPSULATION: NET_FW_AUTHENTICATE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_AUTHENTICATE_WITH_INTEGRITY: NET_FW_AUTHENTICATE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_AUTHENTICATE_AND_NEGOTIATE_ENCRYPTION: NET_FW_AUTHENTICATE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_AUTHENTICATE_AND_ENCRYPT: NET_FW_AUTHENTICATE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NET_FW_EDGE_TRAVERSAL_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DENY: NET_FW_EDGE_TRAVERSAL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_EDGE_TRAVERSAL_TYPE_ALLOW: NET_FW_EDGE_TRAVERSAL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_APP: NET_FW_EDGE_TRAVERSAL_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_USER: NET_FW_EDGE_TRAVERSAL_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NET_FW_IP_PROTOCOL = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_IP_PROTOCOL_TCP: NET_FW_IP_PROTOCOL = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_IP_PROTOCOL_UDP: NET_FW_IP_PROTOCOL = 17i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_IP_PROTOCOL_ANY: NET_FW_IP_PROTOCOL = 256i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NET_FW_IP_VERSION = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_IP_VERSION_V4: NET_FW_IP_VERSION = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_IP_VERSION_V6: NET_FW_IP_VERSION = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_IP_VERSION_ANY: NET_FW_IP_VERSION = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_IP_VERSION_MAX: NET_FW_IP_VERSION = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NET_FW_MODIFY_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_MODIFY_STATE_OK: NET_FW_MODIFY_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_MODIFY_STATE_GP_OVERRIDE: NET_FW_MODIFY_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_MODIFY_STATE_INBOUND_BLOCKED: NET_FW_MODIFY_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NET_FW_POLICY_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_POLICY_GROUP: NET_FW_POLICY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_POLICY_LOCAL: NET_FW_POLICY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_POLICY_EFFECTIVE: NET_FW_POLICY_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_POLICY_TYPE_MAX: NET_FW_POLICY_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NET_FW_PROFILE_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_PROFILE_DOMAIN: NET_FW_PROFILE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_PROFILE_STANDARD: NET_FW_PROFILE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_PROFILE_CURRENT: NET_FW_PROFILE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_PROFILE_TYPE_MAX: NET_FW_PROFILE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NET_FW_PROFILE_TYPE2 = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_PROFILE2_DOMAIN: NET_FW_PROFILE_TYPE2 = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_PROFILE2_PRIVATE: NET_FW_PROFILE_TYPE2 = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_PROFILE2_PUBLIC: NET_FW_PROFILE_TYPE2 = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_PROFILE2_ALL: NET_FW_PROFILE_TYPE2 = 2147483647i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NET_FW_RULE_CATEGORY = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_RULE_CATEGORY_BOOT: NET_FW_RULE_CATEGORY = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_RULE_CATEGORY_STEALTH: NET_FW_RULE_CATEGORY = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_RULE_CATEGORY_FIREWALL: NET_FW_RULE_CATEGORY = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_RULE_CATEGORY_CONSEC: NET_FW_RULE_CATEGORY = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_RULE_CATEGORY_MAX: NET_FW_RULE_CATEGORY = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NET_FW_RULE_DIRECTION = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_RULE_DIR_IN: NET_FW_RULE_DIRECTION = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_RULE_DIR_OUT: NET_FW_RULE_DIRECTION = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_RULE_DIR_MAX: NET_FW_RULE_DIRECTION = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NET_FW_SCOPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_SCOPE_ALL: NET_FW_SCOPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_SCOPE_LOCAL_SUBNET: NET_FW_SCOPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_SCOPE_CUSTOM: NET_FW_SCOPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_SCOPE_MAX: NET_FW_SCOPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type NET_FW_SERVICE_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_SERVICE_FILE_AND_PRINT: NET_FW_SERVICE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_SERVICE_UPNP: NET_FW_SERVICE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_SERVICE_REMOTE_DESKTOP: NET_FW_SERVICE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_SERVICE_NONE: NET_FW_SERVICE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const NET_FW_SERVICE_TYPE_MAX: NET_FW_SERVICE_TYPE = 4i32;
pub const NetFwAuthorizedApplication: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3969402547, data2: 10082, data3: 19051, data4: [162, 20, 106, 203, 96, 52, 98, 210] };
pub const NetFwMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 810346818, data2: 28217, data3: 16600, data4: [148, 58, 185, 19, 196, 12, 156, 212] };
pub const NetFwOpenPort: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 212157894, data2: 14253, data3: 19052, data4: [191, 146, 159, 118, 16, 6, 126, 245] };
pub const NetFwPolicy2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3803433343, data2: 27361, data3: 16812, data4: [129, 122, 246, 249, 33, 102, 215, 221] };
pub const NetFwProduct: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2641649368, data2: 50452, data3: 19741, data4: [191, 66, 117, 31, 237, 45, 90, 199] };
pub const NetFwProducts: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3424192411, data2: 33394, data3: 19827, data4: [187, 112, 205, 181, 51, 82, 123, 97] };
pub const NetFwRule: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 744211518, data2: 13161, data3: 19507, data4: [171, 12, 190, 148, 105, 103, 122, 244] };
pub const NetSharingManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1550041517, data2: 14678, data3: 20472, data4: [132, 134, 64, 3, 71, 88, 49, 91] };
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type PAC_CHANGES_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, pchange: *const INET_FIREWALL_AC_CHANGE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type PFN_FWADDDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddress: *const _tag_FW_DYNAMIC_KEYWORD_ADDRESS0) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type PFN_FWDELETEDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows_sys::core::GUID) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows_sys::core::GUID, dynamickeywordaddressdata: *mut *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0 = ::core::option::Option<unsafe extern "system" fn(flags: u32, dynamickeywordaddressdata: *mut *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressdata: *const _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWUPDATEDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows_sys::core::GUID, updatedaddresses: ::windows_sys::core::PCWSTR, append: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type PNETISO_EDP_ID_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, wszenterpriseid: ::windows_sys::core::PCWSTR, dwerr: u32)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type SHARINGCONNECTIONTYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const ICSSHARINGTYPE_PUBLIC: SHARINGCONNECTIONTYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const ICSSHARINGTYPE_PRIVATE: SHARINGCONNECTIONTYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type SHARINGCONNECTION_ENUM_FLAGS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const ICSSC_DEFAULT: SHARINGCONNECTION_ENUM_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const ICSSC_ENABLED: SHARINGCONNECTION_ENUM_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const S_OBJECT_NO_LONGER_VALID: ::windows_sys::core::HRESULT = 2i32;
pub const UPnPNAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2921201834, data2: 16341, data3: 16444, data4: [138, 39, 43, 189, 195, 12, 208, 225] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    pub id: ::windows_sys::core::GUID,
    pub keyword: ::windows_sys::core::PCWSTR,
    pub flags: u32,
    pub addresses: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {}
impl ::core::clone::Clone for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    pub dynamicKeywordAddress: _tag_FW_DYNAMIC_KEYWORD_ADDRESS0,
    pub next: *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0,
    pub schemaVersion: u16,
    pub originType: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE,
}
impl ::core::marker::Copy for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {}
impl ::core::clone::Clone for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_NON_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_ALL: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub type _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const FW_DYNAMIC_KEYWORD_ORIGIN_INVALID: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const FW_DYNAMIC_KEYWORD_ORIGIN_LOCAL: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFirewall\"`*"]
pub const FW_DYNAMIC_KEYWORD_ORIGIN_MDM: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 2i32;
