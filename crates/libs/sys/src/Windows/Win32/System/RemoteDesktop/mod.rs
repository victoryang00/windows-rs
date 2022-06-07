#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProcessIdToSessionId(dwprocessid: u32, psessionid: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSCloseServer(hserver: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSConnectSessionA(logonid: u32, targetlogonid: u32, ppassword: ::windows_sys::core::PCSTR, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSConnectSessionW(logonid: u32, targetlogonid: u32, ppassword: ::windows_sys::core::PCWSTR, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSCreateListenerA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows_sys::core::PCSTR, pbuffer: *const WTSLISTENERCONFIGA, flag: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSCreateListenerW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows_sys::core::PCWSTR, pbuffer: *const WTSLISTENERCONFIGW, flag: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSDisconnectSession(hserver: super::super::Foundation::HANDLE, sessionid: u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnableChildSessions(benable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateListenersA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: *mut *mut i8, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateListenersW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: *mut *mut u16, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesA(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesExA(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut ::windows_sys::core::PSTR, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesExW(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut ::windows_sys::core::PWSTR, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesW(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateServersA(pdomainname: ::windows_sys::core::PCSTR, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateServersW(pdomainname: ::windows_sys::core::PCWSTR, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsA(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsExA(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1A, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsExW(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1W, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsW(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub fn WTSFreeMemory(pmemory: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSFreeMemoryExA(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const ::core::ffi::c_void, numberofentries: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSFreeMemoryExW(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const ::core::ffi::c_void, numberofentries: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub fn WTSGetActiveConsoleSessionId() -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSGetChildSessionId(psessionid: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSGetListenerSecurityA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows_sys::core::PCSTR, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSGetListenerSecurityW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows_sys::core::PCWSTR, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSIsChildSessionsEnabled(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSLogoffSession(hserver: super::super::Foundation::HANDLE, sessionid: u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerA(pservername: ::windows_sys::core::PCSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerExA(pservername: ::windows_sys::core::PCSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerExW(pservername: ::windows_sys::core::PCWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerW(pservername: ::windows_sys::core::PCWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryListenerConfigA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows_sys::core::PCSTR, pbuffer: *mut WTSLISTENERCONFIGA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryListenerConfigW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows_sys::core::PCWSTR, pbuffer: *mut WTSLISTENERCONFIGW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQuerySessionInformationA(hserver: super::super::Foundation::HANDLE, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut ::windows_sys::core::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQuerySessionInformationW(hserver: super::super::Foundation::HANDLE, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut ::windows_sys::core::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryUserConfigA(pservername: ::windows_sys::core::PCSTR, pusername: ::windows_sys::core::PCSTR, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut ::windows_sys::core::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryUserConfigW(pservername: ::windows_sys::core::PCWSTR, pusername: ::windows_sys::core::PCWSTR, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut ::windows_sys::core::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryUserToken(sessionid: u32, phtoken: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSRegisterSessionNotification(hwnd: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSRegisterSessionNotificationEx(hserver: super::super::Foundation::HANDLE, hwnd: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn WTSSendMessageA(hserver: super::super::Foundation::HANDLE, sessionid: u32, ptitle: ::windows_sys::core::PCSTR, titlelength: u32, pmessage: ::windows_sys::core::PCSTR, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn WTSSendMessageW(hserver: super::super::Foundation::HANDLE, sessionid: u32, ptitle: ::windows_sys::core::PCWSTR, titlelength: u32, pmessage: ::windows_sys::core::PCWSTR, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSSetListenerSecurityA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows_sys::core::PCSTR, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSSetListenerSecurityW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: ::windows_sys::core::PCWSTR, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSSetRenderHint(prenderhintid: *mut u64, hwndowner: super::super::Foundation::HWND, renderhinttype: u32, cbhintdatalength: u32, phintdata: *const u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSSetUserConfigA(pservername: ::windows_sys::core::PCSTR, pusername: ::windows_sys::core::PCSTR, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: ::windows_sys::core::PCSTR, datalength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSSetUserConfigW(pservername: ::windows_sys::core::PCWSTR, pusername: ::windows_sys::core::PCWSTR, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: ::windows_sys::core::PCWSTR, datalength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSShutdownSystem(hserver: super::super::Foundation::HANDLE, shutdownflag: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSStartRemoteControlSessionA(ptargetservername: ::windows_sys::core::PCSTR, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSStartRemoteControlSessionW(ptargetservername: ::windows_sys::core::PCWSTR, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSStopRemoteControlSession(logonid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSTerminateProcess(hserver: super::super::Foundation::HANDLE, processid: u32, exitcode: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSUnRegisterSessionNotification(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSUnRegisterSessionNotificationEx(hserver: super::super::Foundation::HANDLE, hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelClose(hchannelhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelOpen(hserver: super::super::Foundation::HANDLE, sessionid: u32, pvirtualname: ::windows_sys::core::PCSTR) -> HwtsVirtualChannelHandle;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
    pub fn WTSVirtualChannelOpenEx(sessionid: u32, pvirtualname: ::windows_sys::core::PCSTR, flags: u32) -> HwtsVirtualChannelHandle;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelPurgeInput(hchannelhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelPurgeOutput(hchannelhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelQuery(hchannelhandle: super::super::Foundation::HANDLE, param1: WTS_VIRTUAL_CLASS, ppbuffer: *mut *mut ::core::ffi::c_void, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelRead(hchannelhandle: super::super::Foundation::HANDLE, timeout: u32, buffer: ::windows_sys::core::PSTR, buffersize: u32, pbytesread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelWrite(hchannelhandle: super::super::Foundation::HANDLE, buffer: ::windows_sys::core::PCSTR, length: u32, pbyteswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSWaitSystemEvent(hserver: super::super::Foundation::HANDLE, eventmask: u32, peventflags: *mut u32) -> super::super::Foundation::BOOL;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AAAccountingData {
    pub userName: super::super::Foundation::BSTR,
    pub clientName: super::super::Foundation::BSTR,
    pub authType: AAAuthSchemes,
    pub resourceName: super::super::Foundation::BSTR,
    pub portNumber: i32,
    pub protocolName: super::super::Foundation::BSTR,
    pub numberOfBytesReceived: i32,
    pub numberOfBytesTransfered: i32,
    pub reasonForDisconnect: super::super::Foundation::BSTR,
    pub mainSessionId: ::windows_sys::core::GUID,
    pub subSessionId: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AAAccountingData {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AAAccountingData {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type AAAccountingDataType = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_MAIN_SESSION_CREATION: AAAccountingDataType = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_SUB_SESSION_CREATION: AAAccountingDataType = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_SUB_SESSION_CLOSED: AAAccountingDataType = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_MAIN_SESSION_CLOSED: AAAccountingDataType = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type AAAuthSchemes = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_MIN: AAAuthSchemes = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_BASIC: AAAuthSchemes = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_NTLM: AAAuthSchemes = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_SC: AAAuthSchemes = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_LOGGEDONCREDENTIALS: AAAuthSchemes = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_NEGOTIATE: AAAuthSchemes = 5i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_ANY: AAAuthSchemes = 6i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_COOKIE: AAAuthSchemes = 7i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_DIGEST: AAAuthSchemes = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_ORGID: AAAuthSchemes = 9i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_CONID: AAAuthSchemes = 10i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_SSPI_NTLM: AAAuthSchemes = 11i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_AUTH_MAX: AAAuthSchemes = 12i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type AATrustClassID = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_UNTRUSTED: AATrustClassID = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_TRUSTEDUSER_UNTRUSTEDCLIENT: AATrustClassID = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AA_TRUSTEDUSER_TRUSTEDCLIENT: AATrustClassID = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const ACQUIRE_TARGET_LOCK_TIMEOUT: u32 = 300000u32;
pub const ADsTSUserEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3806972646, data2: 7803, data3: 19342, data4: [186, 189, 233, 191, 98, 146, 172, 41] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct AE_CURRENT_POSITION {
    pub u64DevicePosition: u64,
    pub u64StreamPosition: u64,
    pub u64PaddingFrames: u64,
    pub hnsQPCPosition: i64,
    pub f32FramesPerSecond: f32,
    pub Flag: AE_POSITION_FLAGS,
}
impl ::core::marker::Copy for AE_CURRENT_POSITION {}
impl ::core::clone::Clone for AE_CURRENT_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type AE_POSITION_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const POSITION_INVALID: AE_POSITION_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const POSITION_DISCONTINUOUS: AE_POSITION_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const POSITION_CONTINUOUS: AE_POSITION_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const POSITION_QPC_ERROR: AE_POSITION_FLAGS = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct BITMAP_RENDERER_STATISTICS {
    pub dwFramesDelivered: u32,
    pub dwFramesDropped: u32,
}
impl ::core::marker::Copy for BITMAP_RENDERER_STATISTICS {}
impl ::core::clone::Clone for BITMAP_RENDERER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_BUFFER_SIZE: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_CHUNK_LENGTH: u32 = 1600u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANNEL_DEF {
    pub name: [super::super::Foundation::CHAR; 8],
    pub options: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANNEL_DEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANNEL_DEF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANNEL_ENTRY_POINTS {
    pub cbSize: u32,
    pub protocolVersion: u32,
    pub pVirtualChannelInit: PVIRTUALCHANNELINIT,
    pub pVirtualChannelOpen: PVIRTUALCHANNELOPEN,
    pub pVirtualChannelClose: PVIRTUALCHANNELCLOSE,
    pub pVirtualChannelWrite: PVIRTUALCHANNELWRITE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANNEL_ENTRY_POINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANNEL_ENTRY_POINTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_DATA_RECEIVED: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_DISCONNECTED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_INITIALIZED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_TERMINATED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_V1_CONNECTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_WRITE_CANCELLED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_EVENT_WRITE_COMPLETE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_FLAG_FAIL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_FLAG_FIRST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_FLAG_LAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_FLAG_MIDDLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_MAX_COUNT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_NAME_LEN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_COMPRESS: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_COMPRESS_RDP: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_ENCRYPT_CS: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_ENCRYPT_RDP: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_ENCRYPT_SC: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_INITIALIZED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_PRI_HIGH: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_PRI_LOW: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_PRI_MED: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_REMOTE_CONTROL_PERSISTENT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_OPTION_SHOW_PROTOCOL: u32 = 2097152u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct CHANNEL_PDU_HEADER {
    pub length: u32,
    pub flags: u32,
}
impl ::core::marker::Copy for CHANNEL_PDU_HEADER {}
impl ::core::clone::Clone for CHANNEL_PDU_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_ALREADY_CONNECTED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_ALREADY_INITIALIZED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_ALREADY_OPEN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_BAD_CHANNEL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_BAD_CHANNEL_HANDLE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_BAD_INIT_HANDLE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_BAD_PROC: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_INITIALIZATION_ERROR: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_INVALID_INSTANCE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NOT_CONNECTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NOT_INITIALIZED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NOT_IN_VIRTUALCHANNELENTRY: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NOT_OPEN: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NO_BUFFER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NO_MEMORY: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_NULL_DATA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_TOO_MANY_CHANNELS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_UNKNOWN_CHANNEL_NAME: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_UNSUPPORTED_VERSION: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CHANNEL_RC_ZERO_LENGTH: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CLIENTADDRESS_LENGTH: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CLIENTNAME_LENGTH: u32 = 20u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
impl ::core::marker::Copy for CLIENT_DISPLAY {}
impl ::core::clone::Clone for CLIENT_DISPLAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type CLIENT_MESSAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CLIENT_MESSAGE_CONNECTION_INVALID: CLIENT_MESSAGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CLIENT_MESSAGE_CONNECTION_STATUS: CLIENT_MESSAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CLIENT_MESSAGE_CONNECTION_ERROR: CLIENT_MESSAGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type CONNECTION_CHANGE_NOTIFICATION = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_INVALID: CONNECTION_CHANGE_NOTIFICATION = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_PENDING: CONNECTION_CHANGE_NOTIFICATION = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_FAILED: CONNECTION_CHANGE_NOTIFICATION = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_TIMEDOUT: CONNECTION_CHANGE_NOTIFICATION = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_SUCCEEDED: CONNECTION_CHANGE_NOTIFICATION = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_CANCELLED: CONNECTION_CHANGE_NOTIFICATION = 5i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_LB_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = 6i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_QUERY_PL_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = 7i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const CONNECTION_REQUEST_ORCH_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = 8i32;
pub const CONNECTION_PROPERTY_CURSOR_BLINK_DISABLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1259668864, data2: 65188, data3: 19772, data4: [157, 228, 116, 51, 166, 102, 24, 247] };
pub const CONNECTION_PROPERTY_IDLE_TIME_WARNING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1765769205, data2: 3150, data3: 19735, data4: [184, 224, 31, 112, 50, 94, 93, 88] };
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_ADMINMESSAGERECEIVED: u32 = 760u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_AUTORECONNECTED: u32 = 756u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_AUTORECONNECTING: u32 = 755u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_CONNECTED: u32 = 751u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_CONNECTING: u32 = 750u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_DIALOGDISMISSED: u32 = 758u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_DIALOGDISPLAYING: u32 = 757u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_DISCONNECTED: u32 = 753u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_KEYCOMBINATIONPRESSED: u32 = 761u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_LOGINCOMPLETED: u32 = 752u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_NETWORKSTATUSCHANGED: u32 = 759u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_REMOTEDESKTOPSIZECHANGED: u32 = 762u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_STATUSCHANGED: u32 = 754u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_AX_TOUCHPOINTERCURSORMOVED: u32 = 800u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_APPLY_SETTINGS: u32 = 722u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_ATTACH_EVENT: u32 = 706u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_CONNECT: u32 = 701u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DELETE_SAVED_CREDENTIALS: u32 = 704u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DETACH_EVENT: u32 = 707u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DISCONNECT: u32 = 702u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_EXECUTE_REMOTE_ACTION: u32 = 732u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_RDPPROPERTY: u32 = 721u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_SNAPSHOT: u32 = 733u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RECONNECT: u32 = 703u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RESUME_SCREEN_UPDATES: u32 = 731u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RETRIEVE_SETTINGS: u32 = 723u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SET_RDPPROPERTY: u32 = 720u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SUSPEND_SCREEN_UPDATES: u32 = 730u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_UPDATE_SESSION_DISPLAYSETTINGS: u32 = 705u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_ACTIONS: u32 = 711u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_SETTINGS: u32 = 710u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_ENABLED: u32 = 740u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_EVENTSENABLED: u32 = 741u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_POINTERSPEED: u32 = 742u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCH_POINTER: u32 = 712u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DOMAIN_LENGTH: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const FORCE_REJOIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const FORCE_REJOIN_IN_CLUSTERMODE: u32 = 3u32;
pub type HwtsVirtualChannelHandle = isize;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsTSUserEx {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub TerminalServicesProfilePath: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TerminalServicesProfilePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTerminalServicesProfilePath: unsafe extern "system" fn(this: *mut *mut Self, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTerminalServicesProfilePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TerminalServicesHomeDirectory: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TerminalServicesHomeDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTerminalServicesHomeDirectory: unsafe extern "system" fn(this: *mut *mut Self, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTerminalServicesHomeDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TerminalServicesHomeDrive: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TerminalServicesHomeDrive: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTerminalServicesHomeDrive: unsafe extern "system" fn(this: *mut *mut Self, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTerminalServicesHomeDrive: usize,
    pub AllowLogon: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAllowLogon: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub EnableRemoteControl: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetEnableRemoteControl: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub MaxDisconnectionTime: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxDisconnectionTime: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub MaxConnectionTime: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxConnectionTime: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub MaxIdleTime: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxIdleTime: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub ReconnectionAction: unsafe extern "system" fn(this: *mut *mut Self, pnewval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetReconnectionAction: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub BrokenConnectionAction: unsafe extern "system" fn(this: *mut *mut Self, pnewval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBrokenConnectionAction: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub ConnectClientDrivesAtLogon: unsafe extern "system" fn(this: *mut *mut Self, pnewval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetConnectClientDrivesAtLogon: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub ConnectClientPrintersAtLogon: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetConnectClientPrintersAtLogon: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub DefaultToMainPrinter: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDefaultToMainPrinter: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TerminalServicesWorkDirectory: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TerminalServicesWorkDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTerminalServicesWorkDirectory: unsafe extern "system" fn(this: *mut *mut Self, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTerminalServicesWorkDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TerminalServicesInitialProgram: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TerminalServicesInitialProgram: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTerminalServicesInitialProgram: unsafe extern "system" fn(this: *mut *mut Self, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTerminalServicesInitialProgram: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IADsTSUserEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3297971833, data2: 10633, data3: 17506, data4: [138, 96, 47, 207, 47, 41, 85, 239] };
}
#[repr(C)]
pub struct IAudioDeviceEndpoint {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetBuffer: unsafe extern "system" fn(this: *mut *mut Self, maxperiod: i64, u32latencycoefficient: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRTCaps: unsafe extern "system" fn(this: *mut *mut Self, pbisrtcapable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRTCaps: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEventDrivenCapable: unsafe extern "system" fn(this: *mut *mut Self, pbiseventcapable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEventDrivenCapable: usize,
    pub WriteExclusiveModeParametersToSharedMemory: unsafe extern "system" fn(this: *mut *mut Self, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioDeviceEndpoint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3566546778, data2: 41138, data3: 19652, data4: [139, 130, 147, 88, 72, 141, 216, 172] };
}
#[repr(C)]
pub struct IAudioEndpoint {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFrameFormat: unsafe extern "system" fn(this: *mut *mut Self, ppformat: *mut *mut super::super::Media::Audio::WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFrameFormat: usize,
    pub GetFramesPerPacket: unsafe extern "system" fn(this: *mut *mut Self, pframesperpacket: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(this: *mut *mut Self, platency: *mut i64) -> ::windows_sys::core::HRESULT,
    pub SetStreamFlags: unsafe extern "system" fn(this: *mut *mut Self, streamflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventHandle: unsafe extern "system" fn(this: *mut *mut Self, eventhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventHandle: usize,
}
impl ::windows_sys::core::Interface for IAudioEndpoint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 816420117, data2: 5415, data3: 17489, data4: [175, 159, 0, 197, 240, 35, 77, 175] };
}
#[repr(C)]
pub struct IAudioEndpointControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioEndpointControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3330586410, data2: 28148, data3: 18292, data4: [189, 249, 118, 183, 117, 9, 182, 83] };
}
#[repr(C)]
pub struct IAudioEndpointRT {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrentPadding: unsafe extern "system" fn(this: *mut *mut Self, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION),
    pub ProcessingComplete: unsafe extern "system" fn(this: *mut *mut Self),
    pub SetPinInactive: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetPinActive: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioEndpointRT {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3755081823, data2: 42725, data3: 19769, data4: [162, 101, 147, 154, 218, 159, 187, 77] };
}
#[repr(C)]
pub struct IAudioInputEndpointRT {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub GetInputDataPointer: unsafe extern "system" fn(this: *mut *mut Self, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    GetInputDataPointer: usize,
    pub ReleaseInputDataPointer: unsafe extern "system" fn(this: *mut *mut Self, u32framecount: u32, pdatapointer: usize),
    pub PulseEndpoint: unsafe extern "system" fn(this: *mut *mut Self),
}
impl ::windows_sys::core::Interface for IAudioInputEndpointRT {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2150017889, data2: 37554, data3: 17345, data4: [161, 223, 92, 55, 235, 208, 141, 130] };
}
#[repr(C)]
pub struct IAudioOutputEndpointRT {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOutputDataPointer: unsafe extern "system" fn(this: *mut *mut Self, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub ReleaseOutputDataPointer: unsafe extern "system" fn(this: *mut *mut Self, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    ReleaseOutputDataPointer: usize,
    pub PulseEndpoint: unsafe extern "system" fn(this: *mut *mut Self),
}
impl ::windows_sys::core::Interface for IAudioOutputEndpointRT {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2410219236, data2: 49948, data3: 20017, data4: [147, 46, 25, 166, 99, 133, 233, 170] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRemoteDesktopClient {
    pub base__: super::Com::IDispatch,
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reconnect: unsafe extern "system" fn(this: *mut *mut Self, width: u32, height: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Settings: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Settings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Actions: unsafe extern "system" fn(this: *mut *mut Self, actions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Actions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TouchPointer: unsafe extern "system" fn(this: *mut *mut Self, touchpointer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TouchPointer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteSavedCredentials: unsafe extern "system" fn(this: *mut *mut Self, servername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteSavedCredentials: usize,
    pub UpdateSessionDisplaySettings: unsafe extern "system" fn(this: *mut *mut Self, width: u32, height: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub attachEvent: unsafe extern "system" fn(this: *mut *mut Self, eventname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, callback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    attachEvent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub detachEvent: unsafe extern "system" fn(this: *mut *mut Self, eventname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, callback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    detachEvent: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IRemoteDesktopClient {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1473402472, data2: 25178, data3: 18693, data4: [190, 78, 48, 76, 170, 19, 248, 156] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRemoteDesktopClientActions {
    pub base__: super::Com::IDispatch,
    pub SuspendScreenUpdates: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ResumeScreenUpdates: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ExecuteRemoteAction: unsafe extern "system" fn(this: *mut *mut Self, remoteaction: RemoteActionType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSnapshot: unsafe extern "system" fn(this: *mut *mut Self, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32, snapshotdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSnapshot: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IRemoteDesktopClientActions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2102705230, data2: 4136, data3: 17876, data4: [139, 10, 185, 182, 191, 251, 161, 118] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRemoteDesktopClientSettings {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplySettings: unsafe extern "system" fn(this: *mut *mut Self, rdpfilecontents: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplySettings: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RetrieveSettings: unsafe extern "system" fn(this: *mut *mut Self, rdpfilecontents: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RetrieveSettings: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRdpProperty: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRdpProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetRdpProperty: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetRdpProperty: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IRemoteDesktopClientSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1218507431, data2: 10003, data3: 17183, data4: [187, 172, 111, 69, 88, 231, 214, 77] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRemoteDesktopClientTouchPointer {
    pub base__: super::Com::IDispatch,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEventsEnabled: unsafe extern "system" fn(this: *mut *mut Self, eventsenabled: i16) -> ::windows_sys::core::HRESULT,
    pub EventsEnabled: unsafe extern "system" fn(this: *mut *mut Self, eventsenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetPointerSpeed: unsafe extern "system" fn(this: *mut *mut Self, pointerspeed: u32) -> ::windows_sys::core::HRESULT,
    pub PointerSpeed: unsafe extern "system" fn(this: *mut *mut Self, pointerspeed: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IRemoteDesktopClientTouchPointer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 638501421, data2: 36028, data3: 17589, data4: [158, 136, 42, 55, 246, 201, 58, 233] };
}
#[repr(C)]
pub struct IRemoteSystemAdditionalInfoProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetAdditionalInfo: unsafe extern "system" fn(this: *mut *mut Self, deduplicationid: *mut ::windows_sys::core::HSTRING, riid: *const ::windows_sys::core::GUID, mapview: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemAdditionalInfoProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4004134239, data2: 60515, data3: 19751, data4: [175, 56, 232, 107, 29, 114, 146, 203] };
}
#[repr(C)]
pub struct ITSGAccountingEngine {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub DoAccounting: unsafe extern "system" fn(this: *mut *mut Self, accountingdatatype: AAAccountingDataType, accountingdata: ::core::mem::ManuallyDrop<AAAccountingData>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoAccounting: usize,
}
impl ::windows_sys::core::Interface for ITSGAccountingEngine {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1289920713, data2: 59508, data3: 20250, data4: [134, 244, 6, 187, 185, 17, 83, 56] };
}
#[repr(C)]
pub struct ITSGAuthenticateUserSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnUserAuthenticated: unsafe extern "system" fn(this: *mut *mut Self, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnUserAuthenticated: usize,
    pub OnUserAuthenticationFailed: unsafe extern "system" fn(this: *mut *mut Self, context: usize, genericerrorcode: ::windows_sys::core::HRESULT, specificerrorcode: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub ReauthenticateUser: unsafe extern "system" fn(this: *mut *mut Self, context: usize) -> ::windows_sys::core::HRESULT,
    pub DisconnectUser: unsafe extern "system" fn(this: *mut *mut Self, context: usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITSGAuthenticateUserSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 742272627, data2: 42882, data3: 18425, data4: [141, 251, 119, 238, 30, 210, 122, 3] };
}
#[repr(C)]
pub struct ITSGAuthenticationEngine {
    pub base__: ::windows_sys::core::IUnknown,
    pub AuthenticateUser: unsafe extern "system" fn(this: *mut *mut Self, mainsessionid: ::windows_sys::core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CancelAuthentication: unsafe extern "system" fn(this: *mut *mut Self, mainsessionid: ::windows_sys::core::GUID, context: usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITSGAuthenticationEngine {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2665735615, data2: 1195, data3: 18065, data4: [153, 140, 215, 246, 34, 50, 26, 86] };
}
#[repr(C)]
pub struct ITSGAuthorizeConnectionSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnConnectionAuthorized: unsafe extern "system" fn(this: *mut *mut Self, hrin: ::windows_sys::core::HRESULT, mainsessionid: ::windows_sys::core::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITSGAuthorizeConnectionSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3263090227, data2: 30593, data3: 17176, data4: [152, 239, 28, 242, 218, 123, 112, 5] };
}
#[repr(C)]
pub struct ITSGAuthorizeResourceSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnChannelAuthorized: unsafe extern "system" fn(this: *mut *mut Self, hrin: ::windows_sys::core::HRESULT, mainsessionid: ::windows_sys::core::GUID, subsessionid: i32, allowedresourcenames: *const super::super::Foundation::BSTR, numallowedresourcenames: u32, failedresourcenames: *const super::super::Foundation::BSTR, numfailedresourcenames: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnChannelAuthorized: usize,
}
impl ::windows_sys::core::Interface for ITSGAuthorizeResourceSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4275961044, data2: 64018, data3: 17461, data4: [174, 85, 122, 209, 169, 119, 154, 247] };
}
#[repr(C)]
pub struct ITSGPolicyEngine {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthorizeConnection: unsafe extern "system" fn(this: *mut *mut Self, mainsessionid: ::windows_sys::core::GUID, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authtype: AAAuthSchemes, clientmachineip: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, clientmachinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sohdata: *const u8, numsohbytes: u32, cookiedata: *const u8, numcookiebytes: u32, usertoken: super::super::Foundation::HANDLE_PTR, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthorizeConnection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthorizeResource: unsafe extern "system" fn(this: *mut *mut Self, mainsessionid: ::windows_sys::core::GUID, subsessionid: i32, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, resourcenames: *const super::super::Foundation::BSTR, numresources: u32, alternateresourcenames: *const super::super::Foundation::BSTR, numalternateresourcename: u32, portnumber: u32, operation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cookie: *const u8, numbytesincookie: u32, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthorizeResource: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsQuarantineEnabled: unsafe extern "system" fn(this: *mut *mut Self, quarantineenabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsQuarantineEnabled: usize,
}
impl ::windows_sys::core::Interface for ITSGPolicyEngine {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2344767240, data2: 25123, data3: 17140, data4: [165, 180, 142, 55, 205, 19, 91, 189] };
}
#[repr(C)]
pub struct ITsSbBaseNotifySink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnError: unsafe extern "system" fn(this: *mut *mut Self, hrerror: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnReportStatus: unsafe extern "system" fn(this: *mut *mut Self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbBaseNotifySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2156553527, data2: 4738, data3: 18825, data4: [158, 9, 244, 57, 56, 183, 23, 34] };
}
#[repr(C)]
pub struct ITsSbClientConnection {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub UserName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Domain: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Domain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitialProgram: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitialProgram: usize,
    pub LoadBalanceResult: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FarmName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FarmName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PutContext: unsafe extern "system" fn(this: *mut *mut Self, contextid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: ::core::mem::ManuallyDrop<super::Com::VARIANT>, existingcontext: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PutContext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetContext: unsafe extern "system" fn(this: *mut *mut Self, contextid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetContext: usize,
    pub Environment: unsafe extern "system" fn(this: *mut *mut Self, ppenvironment: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub get_ConnectionError: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SamUserAccount: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SamUserAccount: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub ClientConnectionPropertySet: unsafe extern "system" fn(this: *mut *mut Self, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    ClientConnectionPropertySet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFirstAssignment: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFirstAssignment: usize,
    pub RdFarmType: unsafe extern "system" fn(this: *mut *mut Self, prdfarmtype: *mut RD_FARM_TYPE) -> ::windows_sys::core::HRESULT,
    pub UserSidString: unsafe extern "system" fn(this: *mut *mut Self, pszusersidstring: *mut *mut i8) -> ::windows_sys::core::HRESULT,
    pub GetDisconnectedSession: unsafe extern "system" fn(this: *mut *mut Self, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbClientConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 411399321, data2: 44385, data3: 19227, data4: [183, 223, 203, 205, 65, 251, 131, 56] };
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
pub struct ITsSbClientConnectionPropertySet {
    pub base__: ITsSbPropertySet,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_sys::core::Interface for ITsSbClientConnectionPropertySet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3843659184, data2: 18134, data3: 4573, data4: [170, 33, 206, 220, 85, 216, 149, 147] };
}
#[repr(C)]
pub struct ITsSbEnvironment {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub ServerWeight: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub EnvironmentPropertySet: unsafe extern "system" fn(this: *mut *mut Self, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    EnvironmentPropertySet: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetEnvironmentPropertySet: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetEnvironmentPropertySet: usize,
}
impl ::windows_sys::core::Interface for ITsSbEnvironment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2357721079, data2: 48977, data3: 19036, data4: [135, 191, 142, 148, 251, 110, 34, 86] };
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
pub struct ITsSbEnvironmentPropertySet {
    pub base__: ITsSbPropertySet,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_sys::core::Interface for ITsSbEnvironmentPropertySet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3503406974, data2: 31439, data3: 4573, data4: [162, 67, 229, 17, 86, 216, 149, 147] };
}
#[repr(C)]
pub struct ITsSbFilterPluginStore {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SaveProperties: unsafe extern "system" fn(this: *mut *mut Self, ppropertyset: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SaveProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub EnumerateProperties: unsafe extern "system" fn(this: *mut *mut Self, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    EnumerateProperties: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteProperties: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteProperties: usize,
}
impl ::windows_sys::core::Interface for ITsSbFilterPluginStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2243185423, data2: 60792, data3: 16703, data4: [151, 2, 250, 109, 59, 94, 231, 85] };
}
#[repr(C)]
pub struct ITsSbGenericNotifySink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnCompleted: unsafe extern "system" fn(this: *mut *mut Self, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWaitTimeout: unsafe extern "system" fn(this: *mut *mut Self, pfttimeout: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWaitTimeout: usize,
}
impl ::windows_sys::core::Interface for ITsSbGenericNotifySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1280085071, data2: 12299, data3: 18093, data4: [145, 100, 132, 104, 167, 231, 86, 140] };
}
#[repr(C)]
pub struct ITsSbGlobalStore {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryTarget: unsafe extern "system" fn(this: *mut *mut Self, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QuerySessionBySessionId: unsafe extern "system" fn(this: *mut *mut Self, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QuerySessionBySessionId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumerateFarms: unsafe extern "system" fn(this: *mut *mut Self, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumerateFarms: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateTargets: unsafe extern "system" fn(this: *mut *mut Self, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, envname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateTargets: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateEnvironmentsByProvider: unsafe extern "system" fn(this: *mut *mut Self, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, ppval: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateEnvironmentsByProvider: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateSessions: unsafe extern "system" fn(this: *mut *mut Self, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateSessions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFarmProperty: unsafe extern "system" fn(this: *mut *mut Self, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarvalue: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFarmProperty: usize,
}
impl ::windows_sys::core::Interface for ITsSbGlobalStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2595622779, data2: 48498, data3: 19871, data4: [138, 58, 160, 234, 85, 116, 230, 53] };
}
#[repr(C)]
pub struct ITsSbLoadBalanceResult {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetName: usize,
}
impl ::windows_sys::core::Interface for ITsSbLoadBalanceResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 620607404, data2: 65190, data3: 4572, data4: [150, 114, 154, 137, 86, 216, 149, 147] };
}
#[repr(C)]
pub struct ITsSbLoadBalancing {
    pub base__: ITsSbPlugin,
    pub GetMostSuitableTarget: unsafe extern "system" fn(this: *mut *mut Self, pconnection: *mut ::core::ffi::c_void, plbsink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbLoadBalancing {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 607294068, data2: 40631, data3: 4572, data4: [174, 152, 242, 180, 86, 216, 149, 147] };
}
#[repr(C)]
pub struct ITsSbLoadBalancingNotifySink {
    pub base__: ITsSbBaseNotifySink,
    #[cfg(feature = "Win32_Foundation")]
    pub OnGetMostSuitableTarget: unsafe extern "system" fn(this: *mut *mut Self, plbresult: *mut ::core::ffi::c_void, fisnewconnection: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnGetMostSuitableTarget: usize,
}
impl ::windows_sys::core::Interface for ITsSbLoadBalancingNotifySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1602912919, data2: 12868, data3: 20074, data4: [149, 138, 39, 200, 34, 193, 225, 65] };
}
#[repr(C)]
pub struct ITsSbOrchestration {
    pub base__: ITsSbPlugin,
    pub PrepareTargetForConnect: unsafe extern "system" fn(this: *mut *mut Self, pconnection: *mut ::core::ffi::c_void, porchestrationnotifysink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbOrchestration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1694241138, data2: 40631, data3: 4572, data4: [139, 0, 58, 186, 86, 216, 149, 147] };
}
#[repr(C)]
pub struct ITsSbOrchestrationNotifySink {
    pub base__: ITsSbBaseNotifySink,
    pub OnReadyToConnect: unsafe extern "system" fn(this: *mut *mut Self, ptarget: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbOrchestrationNotifySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 918781281, data2: 37483, data3: 17455, data4: [188, 165, 17, 140, 109, 80, 220, 242] };
}
#[repr(C)]
pub struct ITsSbPlacement {
    pub base__: ITsSbPlugin,
    pub QueryEnvironmentForTarget: unsafe extern "system" fn(this: *mut *mut Self, pconnection: *mut ::core::ffi::c_void, pplacementsink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbPlacement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3668831839, data2: 27954, data3: 18446, data4: [158, 54, 221, 171, 35, 41, 240, 109] };
}
#[repr(C)]
pub struct ITsSbPlacementNotifySink {
    pub base__: ITsSbBaseNotifySink,
    pub OnQueryEnvironmentCompleted: unsafe extern "system" fn(this: *mut *mut Self, penvironment: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbPlacementNotifySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1755366535, data2: 11087, data3: 18114, data4: [148, 161, 108, 230, 133, 24, 54, 52] };
}
#[repr(C)]
pub struct ITsSbPlugin {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pprovider: *mut ::core::ffi::c_void, pnotifysink: *mut ::core::ffi::c_void, ppropertyset: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Initialize: usize,
    pub Terminate: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbPlugin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1221424134, data2: 51883, data3: 18015, data4: [165, 214, 186, 168, 99, 185, 234, 79] };
}
#[repr(C)]
pub struct ITsSbPluginNotifySink {
    pub base__: ITsSbBaseNotifySink,
    pub OnInitialized: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnTerminated: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbPluginNotifySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1155523339, data2: 50110, data3: 16629, data4: [191, 130, 122, 149, 187, 121, 90, 223] };
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
pub struct ITsSbPluginPropertySet {
    pub base__: ITsSbPropertySet,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_sys::core::Interface for ITsSbPluginPropertySet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2499833396, data2: 32511, data3: 19308, data4: [187, 64, 73, 164, 253, 167, 206, 166] };
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
pub struct ITsSbPropertySet {
    pub base__: super::Com::StructuredStorage::IPropertyBag,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_sys::core::Interface for ITsSbPropertySet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1543655793, data2: 47902, data3: 19375, data4: [162, 18, 109, 94, 151, 116, 179, 59] };
}
#[repr(C)]
pub struct ITsSbProvider {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTargetObject: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTargetObject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateLoadBalanceResultObject: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplbresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateLoadBalanceResultObject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSessionObject: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sessionid: u32, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSessionObject: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreatePluginPropertySet: unsafe extern "system" fn(this: *mut *mut Self, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreatePluginPropertySet: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreateTargetPropertySetObject: unsafe extern "system" fn(this: *mut *mut Self, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreateTargetPropertySetObject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateEnvironmentObject: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverweight: u32, ppenvironment: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateEnvironmentObject: usize,
    pub GetResourcePluginStore: unsafe extern "system" fn(this: *mut *mut Self, ppstore: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFilterPluginStore: unsafe extern "system" fn(this: *mut *mut Self, ppstore: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterForNotification: unsafe extern "system" fn(this: *mut *mut Self, notificationtype: u32, resourcetomonitor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppluginnotification: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterForNotification: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnRegisterForNotification: unsafe extern "system" fn(this: *mut *mut Self, notificationtype: u32, resourcetomonitor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnRegisterForNotification: usize,
    pub GetInstanceOfGlobalStore: unsafe extern "system" fn(this: *mut *mut Self, ppglobalstore: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreateEnvironmentPropertySetObject: unsafe extern "system" fn(this: *mut *mut Self, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreateEnvironmentPropertySetObject: usize,
}
impl ::windows_sys::core::Interface for ITsSbProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2275674511, data2: 28027, data3: 17629, data4: [188, 23, 140, 228, 78, 55, 13, 82] };
}
#[repr(C)]
pub struct ITsSbProvisioning {
    pub base__: ITsSbPlugin,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualMachines: unsafe extern "system" fn(this: *mut *mut Self, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualMachines: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PatchVirtualMachines: unsafe extern "system" fn(this: *mut *mut Self, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: *mut ::core::ffi::c_void, pvmpatchinfo: *const VM_PATCH_INFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PatchVirtualMachines: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteVirtualMachines: unsafe extern "system" fn(this: *mut *mut Self, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteVirtualMachines: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CancelJob: unsafe extern "system" fn(this: *mut *mut Self, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CancelJob: usize,
}
impl ::windows_sys::core::Interface for ITsSbProvisioning {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 795807163, data2: 40527, data3: 17963, data4: [156, 63, 252, 204, 61, 203, 98, 50] };
}
#[repr(C)]
pub struct ITsSbProvisioningPluginNotifySink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnJobCreated: unsafe extern "system" fn(this: *mut *mut Self, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnVirtualMachineStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: ::windows_sys::core::HRESULT, errordescr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnVirtualMachineStatusChanged: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnJobCompleted: unsafe extern "system" fn(this: *mut *mut Self, resultcode: ::windows_sys::core::HRESULT, resultdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnJobCompleted: usize,
    pub OnJobCancelled: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub LockVirtualMachine: unsafe extern "system" fn(this: *mut *mut Self, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnVirtualMachineHostStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, vmhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: ::windows_sys::core::HRESULT, errordescr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnVirtualMachineHostStatusChanged: usize,
}
impl ::windows_sys::core::Interface for ITsSbProvisioningPluginNotifySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2896722574, data2: 33163, data3: 17793, data4: [160, 50, 73, 195, 223, 185, 199, 1] };
}
#[repr(C)]
pub struct ITsSbResourceNotification {
    pub base__: ::windows_sys::core::IUnknown,
    pub NotifySessionChange: unsafe extern "system" fn(this: *mut *mut Self, changetype: TSSESSION_STATE, psession: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NotifyTargetChange: unsafe extern "system" fn(this: *mut *mut Self, targetchangetype: u32, ptarget: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NotifyClientConnectionStateChange: unsafe extern "system" fn(this: *mut *mut Self, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbResourceNotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1708386394, data2: 50075, data3: 4572, data4: [185, 45, 60, 210, 85, 216, 149, 147] };
}
#[repr(C)]
pub struct ITsSbResourceNotificationEx {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub NotifySessionChangeEx: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sessionid: u32, sessionstate: TSSESSION_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifySessionChangeEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub NotifyTargetChangeEx: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetchangetype: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifyTargetChangeEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub NotifyClientConnectionStateChangeEx: unsafe extern "system" fn(this: *mut *mut Self, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifyClientConnectionStateChangeEx: usize,
}
impl ::windows_sys::core::Interface for ITsSbResourceNotificationEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2829352926, data2: 51857, data3: 17618, data4: [184, 151, 58, 162, 138, 67, 178, 183] };
}
#[repr(C)]
pub struct ITsSbResourcePlugin {
    pub base__: ITsSbPlugin,
}
impl ::windows_sys::core::Interface for ITsSbResourcePlugin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3935155244, data2: 39149, data3: 17717, data4: [168, 139, 42, 22, 79, 53, 73, 15] };
}
#[repr(C)]
pub struct ITsSbResourcePluginStore {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryTarget: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QuerySessionBySessionId: unsafe extern "system" fn(this: *mut *mut Self, dwsessionid: u32, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QuerySessionBySessionId: usize,
    pub AddTargetToStore: unsafe extern "system" fn(this: *mut *mut Self, ptarget: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddSessionToStore: unsafe extern "system" fn(this: *mut *mut Self, psession: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddEnvironmentToStore: unsafe extern "system" fn(this: *mut *mut Self, penvironment: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveEnvironmentFromStore: unsafe extern "system" fn(this: *mut *mut Self, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bignoreowner: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveEnvironmentFromStore: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateFarms: unsafe extern "system" fn(this: *mut *mut Self, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateFarms: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryEnvironment: unsafe extern "system" fn(this: *mut *mut Self, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppenvironment: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryEnvironment: usize,
    pub EnumerateEnvironments: unsafe extern "system" fn(this: *mut *mut Self, pdwcount: *mut u32, pval: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveTarget: unsafe extern "system" fn(this: *mut *mut Self, ptarget: *mut ::core::ffi::c_void, bforcewrite: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveEnvironment: unsafe extern "system" fn(this: *mut *mut Self, penvironment: *mut ::core::ffi::c_void, bforcewrite: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveEnvironment: usize,
    pub SaveSession: unsafe extern "system" fn(this: *mut *mut Self, psession: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetTargetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetEnvironmentProperty: unsafe extern "system" fn(this: *mut *mut Self, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetEnvironmentProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTargetState: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newstate: TARGET_STATE, poldstate: *mut TARGET_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTargetState: usize,
    pub SetSessionState: unsafe extern "system" fn(this: *mut *mut Self, sbsession: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateTargets: unsafe extern "system" fn(this: *mut *mut Self, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, envname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateTargets: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateSessions: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateSessions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFarmProperty: unsafe extern "system" fn(this: *mut *mut Self, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarvalue: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFarmProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteTarget: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteTarget: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetTargetPropertyWithVersionCheck: unsafe extern "system" fn(this: *mut *mut Self, ptarget: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetTargetPropertyWithVersionCheck: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetEnvironmentPropertyWithVersionCheck: unsafe extern "system" fn(this: *mut *mut Self, penvironment: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetEnvironmentPropertyWithVersionCheck: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AcquireTargetLock: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwtimeout: u32, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AcquireTargetLock: usize,
    pub ReleaseTargetLock: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TestAndSetServerState: unsafe extern "system" fn(this: *mut *mut Self, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newstate: TARGET_STATE, teststate: TARGET_STATE, pinitstate: *mut TARGET_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TestAndSetServerState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServerWaitingToStart: unsafe extern "system" fn(this: *mut *mut Self, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, servername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServerWaitingToStart: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetServerState: unsafe extern "system" fn(this: *mut *mut Self, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pstate: *mut TARGET_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetServerState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServerDrainMode: unsafe extern "system" fn(this: *mut *mut Self, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, drainmode: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServerDrainMode: usize,
}
impl ::windows_sys::core::Interface for ITsSbResourcePluginStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1547236959, data2: 48369, data3: 16438, data4: [166, 191, 158, 60, 204, 174, 11, 99] };
}
#[repr(C)]
pub struct ITsSbServiceNotification {
    pub base__: ::windows_sys::core::IUnknown,
    pub NotifyServiceFailure: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyServiceSuccess: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbServiceNotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2261477550, data2: 34528, data3: 20311, data4: [138, 100, 187, 116, 6, 188, 85, 80] };
}
#[repr(C)]
pub struct ITsSbSession {
    pub base__: ::windows_sys::core::IUnknown,
    pub SessionId: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, targetname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTargetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Username: unsafe extern "system" fn(this: *mut *mut Self, username: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Username: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Domain: unsafe extern "system" fn(this: *mut *mut Self, domain: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Domain: usize,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, pstate: *mut TSSESSION_STATE) -> ::windows_sys::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, state: TSSESSION_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTime: unsafe extern "system" fn(this: *mut *mut Self, ptime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCreateTime: unsafe extern "system" fn(this: *mut *mut Self, time: super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCreateTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisconnectTime: unsafe extern "system" fn(this: *mut *mut Self, ptime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisconnectTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisconnectTime: unsafe extern "system" fn(this: *mut *mut Self, time: super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisconnectTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitialProgram: unsafe extern "system" fn(this: *mut *mut Self, app: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitialProgram: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInitialProgram: unsafe extern "system" fn(this: *mut *mut Self, application: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInitialProgram: usize,
    pub ClientDisplay: unsafe extern "system" fn(this: *mut *mut Self, pclientdisplay: *mut CLIENT_DISPLAY) -> ::windows_sys::core::HRESULT,
    pub SetClientDisplay: unsafe extern "system" fn(this: *mut *mut Self, pclientdisplay: CLIENT_DISPLAY) -> ::windows_sys::core::HRESULT,
    pub ProtocolType: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetProtocolType: unsafe extern "system" fn(this: *mut *mut Self, val: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3562252999, data2: 45528, data3: 19550, data4: [186, 52, 154, 251, 76, 140, 85, 16] };
}
#[repr(C)]
pub struct ITsSbTarget {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTargetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FarmName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FarmName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFarmName: unsafe extern "system" fn(this: *mut *mut Self, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFarmName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetFQDN: unsafe extern "system" fn(this: *mut *mut Self, targetfqdnname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetFQDN: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTargetFQDN: unsafe extern "system" fn(this: *mut *mut Self, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTargetFQDN: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetNetbios: unsafe extern "system" fn(this: *mut *mut Self, targetnetbiosname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetNetbios: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTargetNetbios: unsafe extern "system" fn(this: *mut *mut Self, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTargetNetbios: usize,
    pub get_IpAddresses: unsafe extern "system" fn(this: *mut *mut Self, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> ::windows_sys::core::HRESULT,
    pub put_IpAddresses: unsafe extern "system" fn(this: *mut *mut Self, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> ::windows_sys::core::HRESULT,
    pub TargetState: unsafe extern "system" fn(this: *mut *mut Self, pstate: *mut TARGET_STATE) -> ::windows_sys::core::HRESULT,
    pub SetTargetState: unsafe extern "system" fn(this: *mut *mut Self, state: TARGET_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub TargetPropertySet: unsafe extern "system" fn(this: *mut *mut Self, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    TargetPropertySet: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetTargetPropertySet: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetTargetPropertySet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnvironmentName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnvironmentName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnvironmentName: unsafe extern "system" fn(this: *mut *mut Self, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnvironmentName: usize,
    pub NumSessions: unsafe extern "system" fn(this: *mut *mut Self, pnumsessions: *mut u32) -> ::windows_sys::core::HRESULT,
    pub NumPendingConnections: unsafe extern "system" fn(this: *mut *mut Self, pnumpendingconnections: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TargetLoad: unsafe extern "system" fn(this: *mut *mut Self, ptargetload: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 375484108, data2: 10029, data3: 16669, data4: [179, 36, 18, 104, 147, 3, 56, 86] };
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
pub struct ITsSbTargetPropertySet {
    pub base__: ITsSbPropertySet,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_sys::core::Interface for ITsSbTargetPropertySet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4156401110, data2: 39244, data3: 19985, data4: [160, 121, 39, 99, 182, 24, 48, 172] };
}
#[repr(C)]
pub struct ITsSbTaskInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetId: unsafe extern "system" fn(this: *mut *mut Self, pname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, pstarttime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut *mut Self, pendtime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, pdeadline: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Deadline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Identifier: unsafe extern "system" fn(this: *mut *mut Self, pidentifier: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Identifier: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, plabel: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Context: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Context: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Plugin: unsafe extern "system" fn(this: *mut *mut Self, pplugin: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Plugin: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut RDV_TASK_STATUS) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITsSbTaskInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1379733635, data2: 35262, data3: 18653, data4: [153, 234, 4, 232, 47, 250, 114, 101] };
}
#[repr(C)]
pub struct ITsSbTaskPlugin {
    pub base__: ITsSbPlugin,
    pub InitializeTaskPlugin: unsafe extern "system" fn(this: *mut *mut Self, pitssbtaskpluginnotifysink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTaskQueue: unsafe extern "system" fn(this: *mut *mut Self, pszhostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sbtaskinfosize: u32, pitssbtaskinfo: *const *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTaskQueue: usize,
}
impl ::windows_sys::core::Interface for ITsSbTaskPlugin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4196593423, data2: 34565, data3: 16830, data4: [147, 188, 68, 189, 188, 241, 201, 196] };
}
#[repr(C)]
pub struct ITsSbTaskPluginNotifySink {
    pub base__: ITsSbBaseNotifySink,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnSetTaskTime: unsafe extern "system" fn(this: *mut *mut Self, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskstarttime: super::super::Foundation::FILETIME, taskendtime: super::super::Foundation::FILETIME, taskdeadline: super::super::Foundation::FILETIME, sztasklabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskplugin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnSetTaskTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDeleteTaskTime: unsafe extern "system" fn(this: *mut *mut Self, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDeleteTaskTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnUpdateTaskStatus: unsafe extern "system" fn(this: *mut *mut Self, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskstatus: RDV_TASK_STATUS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnUpdateTaskStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnReportTasks: unsafe extern "system" fn(this: *mut *mut Self, szhostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnReportTasks: usize,
}
impl ::windows_sys::core::Interface for ITsSbTaskPluginNotifySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1789888926, data2: 49900, data3: 17902, data4: [170, 55, 69, 230, 8, 149, 38, 26] };
}
#[repr(C)]
pub struct IWRdsEnhancedFastReconnectArbitrator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSessionForEnhancedFastReconnect: unsafe extern "system" fn(this: *mut *mut Self, psessionidarray: *const i32, dwsessioncount: u32, presultsessionid: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWRdsEnhancedFastReconnectArbitrator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1461235355, data2: 18418, data3: 18847, data4: [182, 52, 216, 23, 91, 213, 17, 49] };
}
#[repr(C)]
pub struct IWRdsGraphicsChannel {
    pub base__: ::windows_sys::core::IUnknown,
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, cbsize: u32, pbuffer: *const u8, pcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, pchannelevents: *mut ::core::ffi::c_void, popencontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWRdsGraphicsChannel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1749776907, data2: 60927, data3: 17325, data4: [213, 162, 74, 141, 83, 136, 244, 1] };
}
#[repr(C)]
pub struct IWRdsGraphicsChannelEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnDataReceived: unsafe extern "system" fn(this: *mut *mut Self, cbsize: u32, pbuffer: *const u8) -> ::windows_sys::core::HRESULT,
    pub OnClose: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnChannelOpened: unsafe extern "system" fn(this: *mut *mut Self, openresult: ::windows_sys::core::HRESULT, popencontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDataSent: unsafe extern "system" fn(this: *mut *mut Self, pwritecontext: *mut ::core::ffi::c_void, bcancelled: super::super::Foundation::BOOL, pbuffer: *const u8, cbbuffer: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDataSent: usize,
    pub OnMetricsUpdate: unsafe extern "system" fn(this: *mut *mut Self, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWRdsGraphicsChannelEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1743926924, data2: 54900, data3: 20398, data4: [102, 165, 210, 6, 40, 166, 64, 210] };
}
#[repr(C)]
pub struct IWRdsGraphicsChannelManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateChannel: unsafe extern "system" fn(this: *mut *mut Self, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType, ppvirtualchannel: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWRdsGraphicsChannelManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 265646425, data2: 59454, data3: 18282, data4: [168, 185, 74, 121, 118, 231, 30, 24] };
}
#[repr(C)]
pub struct IWRdsProtocolConnection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLogonErrorRedirector: unsafe extern "system" fn(this: *mut *mut Self, pplogonerrorredir: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AcceptConnection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetClientData: unsafe extern "system" fn(this: *mut *mut Self, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetClientData: usize,
    pub GetClientMonitorData: unsafe extern "system" fn(this: *mut *mut Self, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetUserCredentials: unsafe extern "system" fn(this: *mut *mut Self, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows_sys::core::HRESULT,
    pub GetLicenseConnection: unsafe extern "system" fn(this: *mut *mut Self, pplicenseconnection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AuthenticateClientToSession: unsafe extern "system" fn(this: *mut *mut Self, sessionid: *mut WTS_SESSION_ID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NotifySessionId: unsafe extern "system" fn(this: *mut *mut Self, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifySessionId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInputHandles: unsafe extern "system" fn(this: *mut *mut Self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInputHandles: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVideoHandle: unsafe extern "system" fn(this: *mut *mut Self, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVideoHandle: usize,
    pub ConnectNotify: unsafe extern "system" fn(this: *mut *mut Self, sessionid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUserAllowedToLogon: unsafe extern "system" fn(this: *mut *mut Self, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: ::windows_sys::core::PCWSTR, pusername: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUserAllowedToLogon: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SessionArbitrationEnumeration: unsafe extern "system" fn(this: *mut *mut Self, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SessionArbitrationEnumeration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LogonNotify: unsafe extern "system" fn(this: *mut *mut Self, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: ::windows_sys::core::PCWSTR, wszdomainname: ::windows_sys::core::PCWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogonNotify: usize,
    pub PreDisconnect: unsafe extern "system" fn(this: *mut *mut Self, disconnectreason: u32) -> ::windows_sys::core::HRESULT,
    pub DisconnectNotify: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetProtocolStatus: unsafe extern "system" fn(this: *mut *mut Self, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows_sys::core::HRESULT,
    pub GetLastInputTime: unsafe extern "system" fn(this: *mut *mut Self, plastinputtime: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, ulerror: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualChannel: unsafe extern "system" fn(this: *mut *mut Self, szendpointname: ::windows_sys::core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualChannel: usize,
    pub QueryProperty: unsafe extern "system" fn(this: *mut *mut Self, querytype: ::windows_sys::core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows_sys::core::HRESULT,
    pub GetShadowConnection: unsafe extern "system" fn(this: *mut *mut Self, ppshadowconnection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NotifyCommandProcessCreated: unsafe extern "system" fn(this: *mut *mut Self, sessionid: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWRdsProtocolConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 844028239, data2: 64943, data3: 20470, data4: [129, 168, 66, 171, 231, 85, 131, 11] };
}
#[repr(C)]
pub struct IWRdsProtocolConnectionCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnReady: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BrokenConnection: unsafe extern "system" fn(this: *mut *mut Self, reason: u32, source: u32) -> ::windows_sys::core::HRESULT,
    pub StopScreenUpdates: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RedrawWindow: unsafe extern "system" fn(this: *mut *mut Self, rect: *const WTS_SMALL_RECT) -> ::windows_sys::core::HRESULT,
    pub GetConnectionId: unsafe extern "system" fn(this: *mut *mut Self, pconnectionid: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWRdsProtocolConnectionCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4057400114, data2: 53360, data3: 20209, data4: [160, 136, 120, 49, 53, 54, 194, 214] };
}
#[repr(C)]
pub struct IWRdsProtocolConnectionSettings {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetConnectionSetting: unsafe extern "system" fn(this: *mut *mut Self, propertyid: ::windows_sys::core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> ::windows_sys::core::HRESULT,
    pub GetConnectionSetting: unsafe extern "system" fn(this: *mut *mut Self, propertyid: ::windows_sys::core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWRdsProtocolConnectionSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2214393299, data2: 63220, data3: 60052, data4: [156, 210, 50, 242, 128, 225, 229, 16] };
}
#[repr(C)]
pub struct IWRdsProtocolLicenseConnection {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestLicensingCapabilities: unsafe extern "system" fn(this: *mut *mut Self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestLicensingCapabilities: usize,
    pub SendClientLicense: unsafe extern "system" fn(this: *mut *mut Self, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows_sys::core::HRESULT,
    pub RequestClientLicense: unsafe extern "system" fn(this: *mut *mut Self, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ProtocolComplete: unsafe extern "system" fn(this: *mut *mut Self, ulcomplete: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWRdsProtocolLicenseConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 493491295, data2: 53397, data3: 17444, data4: [149, 122, 64, 127, 174, 130, 45, 132] };
}
#[repr(C)]
pub struct IWRdsProtocolListener {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSettings: unsafe extern "system" fn(this: *mut *mut Self, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL, pwrdslistenersettings: *mut WRDS_LISTENER_SETTINGS) -> ::windows_sys::core::HRESULT,
    pub StartListen: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StopListen: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWRdsProtocolListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4240184091, data2: 50822, data3: 17693, data4: [167, 115, 226, 121, 226, 48, 245, 64] };
}
#[repr(C)]
pub struct IWRdsProtocolListenerCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnConnected: unsafe extern "system" fn(this: *mut *mut Self, pconnection: *mut ::core::ffi::c_void, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS, pcallback: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnConnected: usize,
}
impl ::windows_sys::core::Interface for IWRdsProtocolListenerCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 984776283, data2: 17481, data3: 19905, data4: [183, 74, 145, 98, 29, 79, 233, 132] };
}
#[repr(C)]
pub struct IWRdsProtocolLogonErrorRedirector {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnBeginPainting: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RedirectStatus: unsafe extern "system" fn(this: *mut *mut Self, pszmessage: ::windows_sys::core::PCWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_sys::core::HRESULT,
    pub RedirectMessage: unsafe extern "system" fn(this: *mut *mut Self, pszcaption: ::windows_sys::core::PCWSTR, pszmessage: ::windows_sys::core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_sys::core::HRESULT,
    pub RedirectLogonError: unsafe extern "system" fn(this: *mut *mut Self, ntsstatus: i32, ntssubstatus: i32, pszcaption: ::windows_sys::core::PCWSTR, pszmessage: ::windows_sys::core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWRdsProtocolLogonErrorRedirector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1369434171, data2: 5162, data3: 16672, data4: [163, 213, 164, 5, 211, 21, 40, 26] };
}
#[repr(C)]
pub struct IWRdsProtocolManager {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, piwrdssettings: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub CreateListener: unsafe extern "system" fn(this: *mut *mut Self, wszlistenername: ::windows_sys::core::PCWSTR, pprotocollistener: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NotifyServiceStateChange: unsafe extern "system" fn(this: *mut *mut Self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows_sys::core::HRESULT,
    pub NotifySessionOfServiceStart: unsafe extern "system" fn(this: *mut *mut Self, sessionid: *const WTS_SESSION_ID) -> ::windows_sys::core::HRESULT,
    pub NotifySessionOfServiceStop: unsafe extern "system" fn(this: *mut *mut Self, sessionid: *const WTS_SESSION_ID) -> ::windows_sys::core::HRESULT,
    pub NotifySessionStateChange: unsafe extern "system" fn(this: *mut *mut Self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NotifySettingsChange: unsafe extern "system" fn(this: *mut *mut Self, pwrdssettings: *const WRDS_SETTINGS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifySettingsChange: usize,
    pub Uninitialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWRdsProtocolManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3698944359, data2: 15035, data3: 16589, data4: [164, 70, 16, 82, 118, 181, 137, 80] };
}
#[repr(C)]
pub struct IWRdsProtocolSettings {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSettings: unsafe extern "system" fn(this: *mut *mut Self, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSettings: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MergeSettings: unsafe extern "system" fn(this: *mut *mut Self, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MergeSettings: usize,
}
impl ::windows_sys::core::Interface for IWRdsProtocolSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1699371626, data2: 9552, data3: 18411, data4: [182, 247, 235, 214, 55, 71, 82, 101] };
}
#[repr(C)]
pub struct IWRdsProtocolShadowCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub StopShadow: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub InvokeTargetShadow: unsafe extern "system" fn(this: *mut *mut Self, ptargetservername: ::windows_sys::core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWRdsProtocolShadowCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3764813024, data2: 882, data3: 16598, data4: [173, 178, 160, 243, 50, 38, 116, 214] };
}
#[repr(C)]
pub struct IWRdsProtocolShadowConnection {
    pub base__: ::windows_sys::core::IUnknown,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, ptargetservername: ::windows_sys::core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DoTarget: unsafe extern "system" fn(this: *mut *mut Self, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWRdsProtocolShadowConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2598919398, data2: 51934, data3: 17736, data4: [143, 235, 153, 1, 101, 151, 246, 10] };
}
#[repr(C)]
pub struct IWRdsWddmIddProps {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetHardwareId: unsafe extern "system" fn(this: *mut *mut Self, pdisplaydriverhardwareid: ::windows_sys::core::PCWSTR, count: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDriverLoad: unsafe extern "system" fn(this: *mut *mut Self, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDriverLoad: usize,
    pub OnDriverUnload: unsafe extern "system" fn(this: *mut *mut Self, sessionid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableWddmIdd: unsafe extern "system" fn(this: *mut *mut Self, enabled: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableWddmIdd: usize,
}
impl ::windows_sys::core::Interface for IWRdsWddmIddProps {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 327343949, data2: 41609, data3: 17361, data4: [161, 132, 20, 71, 38, 249, 175, 144] };
}
#[repr(C)]
pub struct IWTSBitmapRenderService {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetMappedRenderer: unsafe extern "system" fn(this: *mut *mut Self, mappingid: u64, pmappedrenderercallback: *mut ::core::ffi::c_void, ppmappedrenderer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSBitmapRenderService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3929170065, data2: 1534, data3: 16577, data4: [180, 156, 61, 46, 244, 98, 106, 14] };
}
#[repr(C)]
pub struct IWTSBitmapRenderer {
    pub base__: ::windows_sys::core::IUnknown,
    pub Render: unsafe extern "system" fn(this: *mut *mut Self, imageformat: ::windows_sys::core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> ::windows_sys::core::HRESULT,
    pub GetRendererStatistics: unsafe extern "system" fn(this: *mut *mut Self, pstatistics: *mut BITMAP_RENDERER_STATISTICS) -> ::windows_sys::core::HRESULT,
    pub RemoveMapping: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSBitmapRenderer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1534774423, data2: 62409, data3: 18167, data4: [140, 91, 250, 104, 93, 52, 65, 177] };
}
#[repr(C)]
pub struct IWTSBitmapRendererCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTargetSizeChanged: unsafe extern "system" fn(this: *mut *mut Self, rcnewsize: super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTargetSizeChanged: usize,
}
impl ::windows_sys::core::Interface for IWTSBitmapRendererCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3615658638, data2: 65102, data3: 20087, data4: [174, 144, 156, 208, 179, 227, 179, 83] };
}
#[repr(C)]
pub struct IWTSListener {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetConfiguration: unsafe extern "system" fn(this: *mut *mut Self, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetConfiguration: usize,
}
impl ::windows_sys::core::Interface for IWTSListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2703426054, data2: 39481, data3: 19800, data4: [134, 116, 205, 180, 223, 244, 231, 59] };
}
#[repr(C)]
pub struct IWTSListenerCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnNewChannelConnection: unsafe extern "system" fn(this: *mut *mut Self, pchannel: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnNewChannelConnection: usize,
}
impl ::windows_sys::core::Interface for IWTSListenerCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2703426051, data2: 54951, data3: 4568, data4: [185, 253, 0, 11, 219, 209, 241, 152] };
}
#[repr(C)]
pub struct IWTSPlugin {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pchannelmgr: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Connected: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Disconnected: unsafe extern "system" fn(this: *mut *mut Self, dwdisconnectcode: u32) -> ::windows_sys::core::HRESULT,
    pub Terminated: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSPlugin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2703426049, data2: 5177, data3: 20066, data4: [164, 20, 25, 13, 10, 195, 212, 14] };
}
#[repr(C)]
pub struct IWTSPluginServiceProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetService: unsafe extern "system" fn(this: *mut *mut Self, serviceid: ::windows_sys::core::GUID, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSPluginServiceProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3554702179, data2: 2172, data3: 18284, data4: [134, 167, 219, 177, 95, 70, 221, 180] };
}
#[repr(C)]
pub struct IWTSProtocolConnection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLogonErrorRedirector: unsafe extern "system" fn(this: *mut *mut Self, pplogonerrorredir: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SendPolicyData: unsafe extern "system" fn(this: *mut *mut Self, ppolicydata: *const WTS_POLICY_DATA) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendPolicyData: usize,
    pub AcceptConnection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetClientData: unsafe extern "system" fn(this: *mut *mut Self, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetClientData: usize,
    pub GetUserCredentials: unsafe extern "system" fn(this: *mut *mut Self, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows_sys::core::HRESULT,
    pub GetLicenseConnection: unsafe extern "system" fn(this: *mut *mut Self, pplicenseconnection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AuthenticateClientToSession: unsafe extern "system" fn(this: *mut *mut Self, sessionid: *mut WTS_SESSION_ID) -> ::windows_sys::core::HRESULT,
    pub NotifySessionId: unsafe extern "system" fn(this: *mut *mut Self, sessionid: *const WTS_SESSION_ID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProtocolHandles: unsafe extern "system" fn(this: *mut *mut Self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProtocolHandles: usize,
    pub ConnectNotify: unsafe extern "system" fn(this: *mut *mut Self, sessionid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUserAllowedToLogon: unsafe extern "system" fn(this: *mut *mut Self, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: ::windows_sys::core::PCWSTR, pusername: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUserAllowedToLogon: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SessionArbitrationEnumeration: unsafe extern "system" fn(this: *mut *mut Self, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SessionArbitrationEnumeration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LogonNotify: unsafe extern "system" fn(this: *mut *mut Self, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: ::windows_sys::core::PCWSTR, wszdomainname: ::windows_sys::core::PCWSTR, sessionid: *const WTS_SESSION_ID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogonNotify: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUserData: unsafe extern "system" fn(this: *mut *mut Self, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUserData: usize,
    pub DisconnectNotify: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetProtocolStatus: unsafe extern "system" fn(this: *mut *mut Self, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows_sys::core::HRESULT,
    pub GetLastInputTime: unsafe extern "system" fn(this: *mut *mut Self, plastinputtime: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, ulerror: u32) -> ::windows_sys::core::HRESULT,
    pub SendBeep: unsafe extern "system" fn(this: *mut *mut Self, frequency: u32, duration: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualChannel: unsafe extern "system" fn(this: *mut *mut Self, szendpointname: ::windows_sys::core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualChannel: usize,
    pub QueryProperty: unsafe extern "system" fn(this: *mut *mut Self, querytype: ::windows_sys::core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows_sys::core::HRESULT,
    pub GetShadowConnection: unsafe extern "system" fn(this: *mut *mut Self, ppshadowconnection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSProtocolConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 587741029, data2: 37013, data3: 17992, data4: [152, 191, 239, 129, 201, 20, 3, 45] };
}
#[repr(C)]
pub struct IWTSProtocolConnectionCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnReady: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BrokenConnection: unsafe extern "system" fn(this: *mut *mut Self, reason: u32, source: u32) -> ::windows_sys::core::HRESULT,
    pub StopScreenUpdates: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RedrawWindow: unsafe extern "system" fn(this: *mut *mut Self, rect: *const WTS_SMALL_RECT) -> ::windows_sys::core::HRESULT,
    pub DisplayIOCtl: unsafe extern "system" fn(this: *mut *mut Self, displayioctl: *const WTS_DISPLAY_IOCTL) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSProtocolConnectionCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 587741029, data2: 30187, data3: 16894, data4: [180, 251, 224, 134, 36, 42, 250, 15] };
}
#[repr(C)]
pub struct IWTSProtocolLicenseConnection {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestLicensingCapabilities: unsafe extern "system" fn(this: *mut *mut Self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestLicensingCapabilities: usize,
    pub SendClientLicense: unsafe extern "system" fn(this: *mut *mut Self, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows_sys::core::HRESULT,
    pub RequestClientLicense: unsafe extern "system" fn(this: *mut *mut Self, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ProtocolComplete: unsafe extern "system" fn(this: *mut *mut Self, ulcomplete: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSProtocolLicenseConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 587741029, data2: 6028, data3: 16505, data4: [142, 74, 254, 166, 73, 106, 77, 112] };
}
#[repr(C)]
pub struct IWTSProtocolListener {
    pub base__: ::windows_sys::core::IUnknown,
    pub StartListen: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StopListen: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSProtocolListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 587741029, data2: 17904, data3: 17300, data4: [143, 105, 50, 178, 188, 14, 244, 202] };
}
#[repr(C)]
pub struct IWTSProtocolListenerCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnConnected: unsafe extern "system" fn(this: *mut *mut Self, pconnection: *mut ::core::ffi::c_void, pcallback: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSProtocolListenerCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 587741029, data2: 6701, data3: 19938, data4: [151, 222, 74, 53, 242, 96, 240, 179] };
}
#[repr(C)]
pub struct IWTSProtocolLogonErrorRedirector {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnBeginPainting: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RedirectStatus: unsafe extern "system" fn(this: *mut *mut Self, pszmessage: ::windows_sys::core::PCWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_sys::core::HRESULT,
    pub RedirectMessage: unsafe extern "system" fn(this: *mut *mut Self, pszcaption: ::windows_sys::core::PCWSTR, pszmessage: ::windows_sys::core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_sys::core::HRESULT,
    pub RedirectLogonError: unsafe extern "system" fn(this: *mut *mut Self, ntsstatus: i32, ntssubstatus: i32, pszcaption: ::windows_sys::core::PCWSTR, pszmessage: ::windows_sys::core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSProtocolLogonErrorRedirector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4254818727, data2: 10518, data3: 17959, data4: [141, 238, 67, 40, 113, 26, 214, 203] };
}
#[repr(C)]
pub struct IWTSProtocolManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateListener: unsafe extern "system" fn(this: *mut *mut Self, wszlistenername: ::windows_sys::core::PCWSTR, pprotocollistener: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NotifyServiceStateChange: unsafe extern "system" fn(this: *mut *mut Self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows_sys::core::HRESULT,
    pub NotifySessionOfServiceStart: unsafe extern "system" fn(this: *mut *mut Self, sessionid: *const WTS_SESSION_ID) -> ::windows_sys::core::HRESULT,
    pub NotifySessionOfServiceStop: unsafe extern "system" fn(this: *mut *mut Self, sessionid: *const WTS_SESSION_ID) -> ::windows_sys::core::HRESULT,
    pub NotifySessionStateChange: unsafe extern "system" fn(this: *mut *mut Self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSProtocolManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4192925388, data2: 60793, data3: 20225, data4: [130, 29, 31, 136, 27, 159, 102, 204] };
}
#[repr(C)]
pub struct IWTSProtocolShadowCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub StopShadow: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub InvokeTargetShadow: unsafe extern "system" fn(this: *mut *mut Self, ptargetservername: ::windows_sys::core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSProtocolShadowCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1345987844, data2: 43749, data3: 19121, data4: [147, 224, 109, 28, 75, 198, 247, 26] };
}
#[repr(C)]
pub struct IWTSProtocolShadowConnection {
    pub base__: ::windows_sys::core::IUnknown,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, ptargetservername: ::windows_sys::core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DoTarget: unsafe extern "system" fn(this: *mut *mut Self, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSProtocolShadowConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3996847124, data2: 14331, data3: 17771, data4: [186, 179, 109, 108, 213, 30, 19, 191] };
}
#[repr(C)]
pub struct IWTSSBPlugin {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, plugincapabilities: *mut u32) -> ::windows_sys::core::HRESULT,
    pub WTSSBX_MachineChangeNotification: unsafe extern "system" fn(this: *mut *mut Self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WTSSBX_SessionChangeNotification: unsafe extern "system" fn(this: *mut *mut Self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WTSSBX_SessionChangeNotification: usize,
    pub WTSSBX_GetMostSuitableServer: unsafe extern "system" fn(this: *mut *mut Self, username: ::windows_sys::core::PCWSTR, domainname: ::windows_sys::core::PCWSTR, applicationtype: ::windows_sys::core::PCWSTR, farmname: ::windows_sys::core::PCWSTR, pmachineid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Terminated: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WTSSBX_GetUserExternalSession: unsafe extern "system" fn(this: *mut *mut Self, username: ::windows_sys::core::PCWSTR, domainname: ::windows_sys::core::PCWSTR, applicationtype: ::windows_sys::core::PCWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSSBPlugin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3695492728, data2: 45453, data3: 17305, data4: [178, 16, 100, 27, 246, 122, 0, 44] };
}
#[repr(C)]
pub struct IWTSVirtualChannel {
    pub base__: ::windows_sys::core::IUnknown,
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, cbsize: u32, pbuffer: *const u8, preserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSVirtualChannel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2703426055, data2: 54951, data3: 4568, data4: [185, 253, 0, 11, 219, 209, 241, 152] };
}
#[repr(C)]
pub struct IWTSVirtualChannelCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnDataReceived: unsafe extern "system" fn(this: *mut *mut Self, cbsize: u32, pbuffer: *const u8) -> ::windows_sys::core::HRESULT,
    pub OnClose: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSVirtualChannelCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2703426052, data2: 54951, data3: 4568, data4: [185, 253, 0, 11, 219, 209, 241, 152] };
}
#[repr(C)]
pub struct IWTSVirtualChannelManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateListener: unsafe extern "system" fn(this: *mut *mut Self, pszchannelname: *const u8, uflags: u32, plistenercallback: *mut ::core::ffi::c_void, pplistener: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWTSVirtualChannelManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2703426053, data2: 54951, data3: 4568, data4: [185, 253, 0, 11, 219, 209, 241, 152] };
}
#[repr(C)]
pub struct IWorkspace {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWorkspaceNames: unsafe extern "system" fn(this: *mut *mut Self, psawkspnames: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWorkspaceNames: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub StartRemoteApplication: unsafe extern "system" fn(this: *mut *mut Self, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    StartRemoteApplication: usize,
    pub GetProcessId: unsafe extern "system" fn(this: *mut *mut Self, pulprocessid: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWorkspace {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3106061240, data2: 19541, data3: 20458, data4: [132, 150, 190, 176, 180, 66, 133, 229] };
}
#[repr(C)]
pub struct IWorkspace2 {
    pub base__: IWorkspace,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub StartRemoteApplicationEx: unsafe extern "system" fn(this: *mut *mut Self, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrequestingappid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrequestingappfamilyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, blaunchintoimmersiveclient: i16, bstrimmersiveclientactivationcontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    StartRemoteApplicationEx: usize,
}
impl ::windows_sys::core::Interface for IWorkspace2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2530793423, data2: 30782, data3: 17030, data4: [131, 76, 235, 192, 233, 95, 120, 60] };
}
#[repr(C)]
pub struct IWorkspace3 {
    pub base__: IWorkspace2,
    #[cfg(feature = "Win32_Foundation")]
    pub GetClaimsToken2: unsafe extern "system" fn(this: *mut *mut Self, bstrclaimshint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserhint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: super::super::Foundation::RECT, pbstraccesstoken: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetClaimsToken2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClaimsToken: unsafe extern "system" fn(this: *mut *mut Self, bstraccesstoken: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ullaccesstokenexpiration: u64, bstrrefreshtoken: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClaimsToken: usize,
}
impl ::windows_sys::core::Interface for IWorkspace3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 468500042, data2: 54868, data3: 16955, data4: [175, 235, 190, 141, 83, 44, 19, 198] };
}
#[repr(C)]
pub struct IWorkspaceClientExt {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetResourceId: unsafe extern "system" fn(this: *mut *mut Self, bstrworkspaceid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetResourceId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetResourceDisplayName: unsafe extern "system" fn(this: *mut *mut Self, bstrworkspacedisplayname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetResourceDisplayName: usize,
    pub IssueDisconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWorkspaceClientExt {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 314135284, data2: 16842, data3: 20257, data4: [168, 41, 166, 208, 125, 154, 22, 229] };
}
#[repr(C)]
pub struct IWorkspaceRegistration {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddResource: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    pub RemoveResource: unsafe extern "system" fn(this: *mut *mut Self, dwcookieconnection: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWorkspaceRegistration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3106061240, data2: 19541, data3: 20458, data4: [132, 150, 190, 176, 180, 66, 133, 230] };
}
#[repr(C)]
pub struct IWorkspaceRegistration2 {
    pub base__: IWorkspaceRegistration,
    #[cfg(feature = "Win32_Foundation")]
    pub AddResourceEx: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void, bstreventloguploadaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcookie: *mut u32, correlationid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddResourceEx: usize,
    pub RemoveResourceEx: unsafe extern "system" fn(this: *mut *mut Self, dwcookieconnection: u32, correlationid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWorkspaceRegistration2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3478779476, data2: 14779, data3: 17624, data4: [148, 208, 70, 53, 114, 137, 87, 233] };
}
#[repr(C)]
pub struct IWorkspaceReportMessage {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterErrorLogMessage: unsafe extern "system" fn(this: *mut *mut Self, bstrmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterErrorLogMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsErrorMessageRegistered: unsafe extern "system" fn(this: *mut *mut Self, bstrwkspid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrorcode: u32, pferrorexist: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsErrorMessageRegistered: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterErrorEvent: unsafe extern "system" fn(this: *mut *mut Self, bstrwkspid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrorcode: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterErrorEvent: usize,
}
impl ::windows_sys::core::Interface for IWorkspaceReportMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2814404409, data2: 20495, data3: 20108, data4: [153, 168, 43, 214, 149, 88, 153, 235] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWorkspaceResTypeRegistry {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub AddResourceType: unsafe extern "system" fn(this: *mut *mut Self, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlauncher: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddResourceType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteResourceType: unsafe extern "system" fn(this: *mut *mut Self, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteResourceType: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRegisteredFileExtensions: unsafe extern "system" fn(this: *mut *mut Self, fmachinewide: i16, psafileextensions: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRegisteredFileExtensions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetResourceTypeInfo: unsafe extern "system" fn(this: *mut *mut Self, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrlauncher: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetResourceTypeInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ModifyResourceType: unsafe extern "system" fn(this: *mut *mut Self, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlauncher: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModifyResourceType: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWorkspaceResTypeRegistry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 490900601, data2: 28206, data3: 17233, data4: [163, 97, 192, 64, 26, 3, 160, 186] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWorkspaceScriptable {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub DisconnectWorkspace: unsafe extern "system" fn(this: *mut *mut Self, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisconnectWorkspace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartWorkspace: unsafe extern "system" fn(this: *mut *mut Self, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartWorkspace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsWorkspaceCredentialSpecified: unsafe extern "system" fn(this: *mut *mut Self, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bcountunauthenticatedcredentials: i16, pbcredexist: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsWorkspaceCredentialSpecified: usize,
    pub IsWorkspaceSSOEnabled: unsafe extern "system" fn(this: *mut *mut Self, pbssoenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClearWorkspaceCredential: unsafe extern "system" fn(this: *mut *mut Self, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClearWorkspaceCredential: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnAuthenticated: unsafe extern "system" fn(this: *mut *mut Self, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnAuthenticated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisconnectWorkspaceByFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisconnectWorkspaceByFriendlyName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWorkspaceScriptable {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4025108898, data2: 56741, data3: 17053, data4: [143, 66, 178, 59, 146, 196, 195, 71] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWorkspaceScriptable2 {
    pub base__: IWorkspaceScriptable,
    #[cfg(feature = "Win32_Foundation")]
    pub StartWorkspaceEx: unsafe extern "system" fn(this: *mut *mut Self, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrredirectorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrappcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartWorkspaceEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ResourceDismissed: unsafe extern "system" fn(this: *mut *mut Self, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ResourceDismissed: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWorkspaceScriptable2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4025108898, data2: 56741, data3: 17053, data4: [143, 66, 179, 59, 162, 196, 195, 72] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWorkspaceScriptable3 {
    pub base__: IWorkspaceScriptable2,
    #[cfg(feature = "Win32_Foundation")]
    pub StartWorkspaceEx2:
        unsafe extern "system" fn(this: *mut *mut Self, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrredirectorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrappcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32, bstreventloguploadaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, correlationid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartWorkspaceEx2: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWorkspaceScriptable3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1394500882, data2: 11455, data3: 19410, data4: [128, 165, 217, 10, 113, 99, 106, 154] };
}
#[repr(C)]
pub struct ItsPubPlugin {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetResourceList: unsafe extern "system" fn(this: *mut *mut Self, userid: ::windows_sys::core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows_sys::core::HRESULT,
    pub GetResource: unsafe extern "system" fn(this: *mut *mut Self, alias: ::windows_sys::core::PCWSTR, flags: i32, resource: *mut pluginResource) -> ::windows_sys::core::HRESULT,
    pub GetCacheLastUpdateTime: unsafe extern "system" fn(this: *mut *mut Self, lastupdatetime: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub pluginName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    pluginName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub pluginVersion: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    pluginVersion: usize,
    pub ResolveResource: unsafe extern "system" fn(this: *mut *mut Self, resourcetype: *mut u32, resourcelocation: ::windows_sys::core::PWSTR, endpointname: ::windows_sys::core::PWSTR, userid: ::windows_sys::core::PCWSTR, alias: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ItsPubPlugin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1891650309, data2: 62279, data3: 16683, data4: [130, 47, 54, 201, 156, 84, 202, 69] };
}
#[repr(C)]
pub struct ItsPubPlugin2 {
    pub base__: ItsPubPlugin,
    pub GetResource2List: unsafe extern "system" fn(this: *mut *mut Self, userid: ::windows_sys::core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> ::windows_sys::core::HRESULT,
    pub GetResource2: unsafe extern "system" fn(this: *mut *mut Self, alias: ::windows_sys::core::PCWSTR, flags: i32, resource: *mut pluginResource2) -> ::windows_sys::core::HRESULT,
    pub ResolvePersonalDesktop: unsafe extern "system" fn(this: *mut *mut Self, userid: ::windows_sys::core::PCWSTR, poolid: ::windows_sys::core::PCWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub DeletePersonalDesktopAssignment: unsafe extern "system" fn(this: *mut *mut Self, userid: ::windows_sys::core::PCWSTR, poolid: ::windows_sys::core::PCWSTR, endpointname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ItsPubPlugin2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4199343128, data2: 43735, data3: 20166, data4: [186, 209, 10, 50, 27, 164, 101, 213] };
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KEEP_EXISTING_SESSIONS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type KeyCombinationType = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KeyCombinationHome: KeyCombinationType = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KeyCombinationLeft: KeyCombinationType = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KeyCombinationUp: KeyCombinationType = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KeyCombinationRight: KeyCombinationType = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KeyCombinationDown: KeyCombinationType = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const KeyCombinationScroll: KeyCombinationType = 5i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MAX_DATE_TIME_LENGTH: u32 = 56u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MAX_ELAPSED_TIME_LENGTH: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MAX_POLICY_ATTRIBUTES: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxAppName_Len: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxDomainName_Len: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxFQDN_Len: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxFarm_Len: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxNetBiosName_Len: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxNumOfExposed_IPs: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const MaxUserName_Len: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const NOTIFY_FOR_ALL_SESSIONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const NOTIFY_FOR_THIS_SESSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type PCHANNEL_INIT_EVENT_FN = ::core::option::Option<unsafe extern "system" fn(pinithandle: *mut ::core::ffi::c_void, event: u32, pdata: *mut ::core::ffi::c_void, datalength: u32)>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type PCHANNEL_OPEN_EVENT_FN = ::core::option::Option<unsafe extern "system" fn(openhandle: u32, event: u32, pdata: *mut ::core::ffi::c_void, datalength: u32, totallength: u32, dataflags: u32)>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PLUGIN_CAPABILITY_EXTERNAL_REDIRECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type PLUGIN_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const UNKNOWN_PLUGIN: PLUGIN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const POLICY_PLUGIN: PLUGIN_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RESOURCE_PLUGIN: PLUGIN_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const LOAD_BALANCING_PLUGIN: PLUGIN_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PLACEMENT_PLUGIN: PLUGIN_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const ORCHESTRATION_PLUGIN: PLUGIN_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PROVISIONING_PLUGIN: PLUGIN_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TASK_PLUGIN: PLUGIN_TYPE = 64i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PRODUCTINFO_COMPANYNAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PRODUCTINFO_PRODUCTID_LENGTH: u32 = 4u32;
pub const PROPERTY_DYNAMIC_TIME_ZONE_INFORMATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 215995022, data2: 53433, data3: 19487, data4: [165, 235, 109, 31, 108, 101, 53, 185] };
pub const PROPERTY_TYPE_ENABLE_UNIVERSAL_APPS_FOR_CUSTOM_SHELL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3979100122, data2: 13197, data3: 19775, data4: [129, 163, 231, 103, 49, 13, 144, 142] };
pub const PROPERTY_TYPE_GET_FAST_RECONNECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1645401943, data2: 67, data3: 18530, data4: [153, 195, 159, 48, 89, 172, 42, 59] };
pub const PROPERTY_TYPE_GET_FAST_RECONNECT_USER_SID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 427573882, data2: 309, data3: 19309, data4: [156, 94, 230, 87, 154, 10, 182, 37] };
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type PVIRTUALCHANNELCLOSE = ::core::option::Option<unsafe extern "system" fn(openhandle: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PVIRTUALCHANNELENTRY = ::core::option::Option<unsafe extern "system" fn(pentrypoints: *mut CHANNEL_ENTRY_POINTS) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PVIRTUALCHANNELINIT = ::core::option::Option<unsafe extern "system" fn(ppinithandle: *mut *mut ::core::ffi::c_void, pchannel: *mut CHANNEL_DEF, channelcount: i32, versionrequested: u32, pchanneliniteventproc: PCHANNEL_INIT_EVENT_FN) -> u32>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type PVIRTUALCHANNELOPEN = ::core::option::Option<unsafe extern "system" fn(pinithandle: *mut ::core::ffi::c_void, popenhandle: *mut u32, pchannelname: ::windows_sys::core::PCSTR, pchannelopeneventproc: PCHANNEL_OPEN_EVENT_FN) -> u32>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type PVIRTUALCHANNELWRITE = ::core::option::Option<unsafe extern "system" fn(openhandle: u32, pdata: *mut ::core::ffi::c_void, datalength: u32, puserdata: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type PasswordEncodingType = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PasswordEncodingUTF8: PasswordEncodingType = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PasswordEncodingUTF16LE: PasswordEncodingType = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PasswordEncodingUTF16BE: PasswordEncodingType = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type PolicyAttributeType = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const EnableAllRedirections: PolicyAttributeType = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DisableAllRedirections: PolicyAttributeType = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const DriveRedirectionDisabled: PolicyAttributeType = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PrinterRedirectionDisabled: PolicyAttributeType = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PortRedirectionDisabled: PolicyAttributeType = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const ClipboardRedirectionDisabled: PolicyAttributeType = 5i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const PnpRedirectionDisabled: PolicyAttributeType = 6i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const AllowOnlySDRServers: PolicyAttributeType = 7i32;
pub const RDCLIENT_BITMAP_RENDER_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3838576843, data2: 37934, data3: 19225, data4: [133, 4, 189, 90, 137, 167, 71, 245] };
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type RDV_TASK_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_UNKNOWN: RDV_TASK_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_SEARCHING: RDV_TASK_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_DOWNLOADING: RDV_TASK_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_APPLYING: RDV_TASK_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_REBOOTING: RDV_TASK_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_REBOOTED: RDV_TASK_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_SUCCESS: RDV_TASK_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_FAILED: RDV_TASK_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RDV_TASK_STATUS_TIMEOUT: RDV_TASK_STATUS = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type RD_FARM_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_RDSH: RD_FARM_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_TEMP_VM: RD_FARM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_MANUAL_PERSONAL_VM: RD_FARM_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_AUTO_PERSONAL_VM: RD_FARM_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_MANUAL_PERSONAL_RDSH: RD_FARM_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_AUTO_PERSONAL_RDSH: RD_FARM_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RD_FARM_TYPE_UNKNOWN: RD_FARM_TYPE = -1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const REMOTECONTROL_KBDALT_HOTKEY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const REMOTECONTROL_KBDCTRL_HOTKEY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const REMOTECONTROL_KBDSHIFT_HOTKEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RENDER_HINT_CLEAR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RENDER_HINT_MAPPEDWINDOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RENDER_HINT_VIDEO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RESERVED_FOR_LEGACY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RFX_CLIENT_ID_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RFX_GFX_MAX_SUPPORTED_MONITORS: u32 = 16u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RFX_GFX_MONITOR_INFO {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub physicalWidth: u32,
    pub physicalHeight: u32,
    pub orientation: u32,
    pub primary: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RFX_GFX_MONITOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RFX_GFX_MONITOR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
impl ::core::marker::Copy for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {}
impl ::core::clone::Clone for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub reserved: u32,
    pub monitorCount: u32,
    pub MonitorData: [RFX_GFX_MONITOR_INFO; 16],
    pub clientUniqueId: [u16; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
    pub ulBpp: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_INPUT_RESET {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub RedrawRect: RFX_GFX_RECT,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_DISCONNECT_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub DisconnectReason: u32,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DISCONNECT_NOTIFY {}
impl ::core::clone::Clone for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_HEADER {
    pub uMSGType: u16,
    pub cbSize: u16,
}
impl ::core::marker::Copy for RFX_GFX_MSG_HEADER {}
impl ::core::clone::Clone for RFX_GFX_MSG_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RFX_GFX_MSG_PREFIX: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RFX_GFX_MSG_PREFIX_MASK: u32 = 48u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_MSG_RDP_DATA {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub rdpData: [u8; 1],
}
impl ::core::marker::Copy for RFX_GFX_MSG_RDP_DATA {}
impl ::core::clone::Clone for RFX_GFX_MSG_RDP_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct RFX_GFX_RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl ::core::marker::Copy for RFX_GFX_RECT {}
impl ::core::clone::Clone for RFX_GFX_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RFX_RDP_MSG_PREFIX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type RemoteActionType = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RemoteActionCharms: RemoteActionType = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RemoteActionAppbar: RemoteActionType = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RemoteActionSnap: RemoteActionType = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RemoteActionStartScreen: RemoteActionType = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const RemoteActionAppSwitch: RemoteActionType = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SB_SYNCH_CONFLICT_MAX_WRITE_ATTEMPTS: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type SESSION_TIMEOUT_ACTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SESSION_TIMEOUT_ACTION_DISCONNECT: SESSION_TIMEOUT_ACTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SESSION_TIMEOUT_ACTION_SILENT_REAUTH: SESSION_TIMEOUT_ACTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SINGLE_SESSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type SnapshotEncodingType = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SnapshotEncodingDataUri: SnapshotEncodingType = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type SnapshotFormatType = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SnapshotFormatPng: SnapshotFormatType = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SnapshotFormatJpeg: SnapshotFormatType = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const SnapshotFormatBmp: SnapshotFormatType = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type TARGET_CHANGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_CHANGE_UNSPEC: TARGET_CHANGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_EXTERNALIP_CHANGED: TARGET_CHANGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_INTERNALIP_CHANGED: TARGET_CHANGE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_JOINED: TARGET_CHANGE_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_REMOVED: TARGET_CHANGE_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_STATE_CHANGED: TARGET_CHANGE_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_IDLE: TARGET_CHANGE_TYPE = 64i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PENDING: TARGET_CHANGE_TYPE = 128i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_INUSE: TARGET_CHANGE_TYPE = 256i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PATCH_STATE_CHANGED: TARGET_CHANGE_TYPE = 512i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_FARM_MEMBERSHIP_CHANGED: TARGET_CHANGE_TYPE = 1024i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type TARGET_OWNER = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const OWNER_UNKNOWN: TARGET_OWNER = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const OWNER_MS_TS_PLUGIN: TARGET_OWNER = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const OWNER_MS_VM_PLUGIN: TARGET_OWNER = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type TARGET_PATCH_STATE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PATCH_UNKNOWN: TARGET_PATCH_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PATCH_NOT_STARTED: TARGET_PATCH_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PATCH_IN_PROGRESS: TARGET_PATCH_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PATCH_COMPLETED: TARGET_PATCH_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_PATCH_FAILED: TARGET_PATCH_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type TARGET_STATE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_UNKNOWN: TARGET_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_INITIALIZING: TARGET_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_RUNNING: TARGET_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_DOWN: TARGET_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_HIBERNATED: TARGET_STATE = 5i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_CHECKED_OUT: TARGET_STATE = 6i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_STOPPED: TARGET_STATE = 7i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_INVALID: TARGET_STATE = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_STARTING: TARGET_STATE = 9i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_STOPPING: TARGET_STATE = 10i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TARGET_MAXSTATE: TARGET_STATE = 11i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type TARGET_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const UNKNOWN: TARGET_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const FARM: TARGET_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const NONFARM: TARGET_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_NEW: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_EXISTING: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type TSPUB_PLUGIN_PD_RESOLUTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSPUB_PLUGIN_PD_QUERY_OR_CREATE: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSPUB_PLUGIN_PD_QUERY_EXISTING: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type TSSB_NOTIFICATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSB_NOTIFY_INVALID: TSSB_NOTIFICATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSB_NOTIFY_TARGET_CHANGE: TSSB_NOTIFICATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSB_NOTIFY_SESSION_CHANGE: TSSB_NOTIFICATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSB_NOTIFY_CONNECTION_REQUEST_CHANGE: TSSB_NOTIFICATION_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type TSSD_AddrV46Type = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSD_ADDR_UNDEFINED: TSSD_AddrV46Type = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSD_ADDR_IPv4: TSSD_AddrV46Type = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TSSD_ADDR_IPv6: TSSD_AddrV46Type = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct TSSD_ConnectionPoint {
    pub ServerAddressB: [u8; 16],
    pub AddressType: TSSD_AddrV46Type,
    pub PortNumber: u16,
    pub AddressScope: u32,
}
impl ::core::marker::Copy for TSSD_ConnectionPoint {}
impl ::core::clone::Clone for TSSD_ConnectionPoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type TSSESSION_STATE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_INVALID: TSSESSION_STATE = -1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_ACTIVE: TSSESSION_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_CONNECTED: TSSESSION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_CONNECTQUERY: TSSESSION_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_SHADOW: TSSESSION_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_DISCONNECTED: TSSESSION_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_IDLE: TSSESSION_STATE = 5i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_LISTEN: TSSESSION_STATE = 6i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_RESET: TSSESSION_STATE = 7i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_DOWN: TSSESSION_STATE = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_INIT: TSSESSION_STATE = 9i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const STATE_MAX: TSSESSION_STATE = 10i32;
pub const TSUserExInterfaces: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 152100097, data2: 57228, data3: 4561, data4: [174, 39, 0, 192, 79, 163, 88, 19] };
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type TS_SB_SORT_BY = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TS_SB_SORT_BY_NONE: TS_SB_SORT_BY = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TS_SB_SORT_BY_NAME: TS_SB_SORT_BY = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TS_SB_SORT_BY_PROP: TS_SB_SORT_BY = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const TS_VC_LISTENER_STATIC_CHANNEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const USERNAME_LENGTH: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VALIDATIONINFORMATION_HARDWAREID_LENGTH: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VALIDATIONINFORMATION_LICENSE_LENGTH: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VIRTUAL_CHANNEL_VERSION_WIN2000: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type VM_HOST_NOTIFY_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_HOST_STATUS_INIT_PENDING: VM_HOST_NOTIFY_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_HOST_STATUS_INIT_IN_PROGRESS: VM_HOST_NOTIFY_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_HOST_STATUS_INIT_COMPLETE: VM_HOST_NOTIFY_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_HOST_STATUS_INIT_FAILED: VM_HOST_NOTIFY_STATUS = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct VM_NOTIFY_ENTRY {
    pub VmName: [u16; 128],
    pub VmHost: [u16; 128],
}
impl ::core::marker::Copy for VM_NOTIFY_ENTRY {}
impl ::core::clone::Clone for VM_NOTIFY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct VM_NOTIFY_INFO {
    pub dwNumEntries: u32,
    pub ppVmEntries: *mut *mut VM_NOTIFY_ENTRY,
}
impl ::core::marker::Copy for VM_NOTIFY_INFO {}
impl ::core::clone::Clone for VM_NOTIFY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type VM_NOTIFY_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_NOTIFY_STATUS_PENDING: VM_NOTIFY_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_NOTIFY_STATUS_IN_PROGRESS: VM_NOTIFY_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_NOTIFY_STATUS_COMPLETE: VM_NOTIFY_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_NOTIFY_STATUS_FAILED: VM_NOTIFY_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const VM_NOTIFY_STATUS_CANCELED: VM_NOTIFY_STATUS = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct VM_PATCH_INFO {
    pub dwNumEntries: u32,
    pub pVmNames: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for VM_PATCH_INFO {}
impl ::core::clone::Clone for VM_PATCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WINSTATIONNAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WKS_FLAG_CLEAR_CREDS_ON_LAST_RESOURCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WKS_FLAG_CREDS_AUTHENTICATED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WKS_FLAG_PASSWORD_ENCRYPTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_CLIENTADDRESS_LENGTH: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_CLIENTNAME_LENGTH: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WRDS_CONNECTION_SETTING {
    pub WRdsConnectionSettings1: WRDS_CONNECTION_SETTINGS_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_CONNECTION_SETTING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_CONNECTION_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_CONNECTION_SETTINGS {
    pub WRdsConnectionSettingLevel: WRDS_CONNECTION_SETTING_LEVEL,
    pub WRdsConnectionSetting: WRDS_CONNECTION_SETTING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_CONNECTION_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_CONNECTION_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_CONNECTION_SETTINGS_1 {
    pub fInheritInitialProgram: super::super::Foundation::BOOLEAN,
    pub fInheritColorDepth: super::super::Foundation::BOOLEAN,
    pub fHideTitleBar: super::super::Foundation::BOOLEAN,
    pub fInheritAutoLogon: super::super::Foundation::BOOLEAN,
    pub fMaximizeShell: super::super::Foundation::BOOLEAN,
    pub fDisablePNP: super::super::Foundation::BOOLEAN,
    pub fPasswordIsScPin: super::super::Foundation::BOOLEAN,
    pub fPromptForPassword: super::super::Foundation::BOOLEAN,
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fResetBroken: super::super::Foundation::BOOLEAN,
    pub fDisableEncryption: super::super::Foundation::BOOLEAN,
    pub fDisableAutoReconnect: super::super::Foundation::BOOLEAN,
    pub fDisableCtrlAltDel: super::super::Foundation::BOOLEAN,
    pub fDoubleClickDetect: super::super::Foundation::BOOLEAN,
    pub fEnableWindowsKey: super::super::Foundation::BOOLEAN,
    pub fUsingSavedCreds: super::super::Foundation::BOOLEAN,
    pub fMouse: super::super::Foundation::BOOLEAN,
    pub fNoAudioPlayback: super::super::Foundation::BOOLEAN,
    pub fRemoteConsoleAudio: super::super::Foundation::BOOLEAN,
    pub EncryptionLevel: u8,
    pub ColorDepth: u16,
    pub ProtocolType: u16,
    pub HRes: u16,
    pub VRes: u16,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub KeyboardLayout: u32,
    pub MaxConnectionTime: u32,
    pub MaxDisconnectionTime: u32,
    pub MaxIdleTime: u32,
    pub PerformanceFlags: u32,
    pub KeyboardType: u32,
    pub KeyboardSubType: u32,
    pub KeyboardFunctionKey: u32,
    pub ActiveInputLocale: u32,
    pub SerialNumber: u32,
    pub ClientAddressFamily: u32,
    pub ClientBuildNumber: u32,
    pub ClientSessionId: u32,
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub UserName: [u16; 256],
    pub Domain: [u16; 256],
    pub Password: [u16; 256],
    pub ProtocolName: [u16; 9],
    pub DisplayDriverName: [u16; 9],
    pub DisplayDeviceName: [u16; 20],
    pub imeFileName: [u16; 33],
    pub AudioDriverName: [u16; 9],
    pub ClientName: [u16; 21],
    pub ClientAddress: [u16; 31],
    pub ClientDirectory: [u16; 257],
    pub ClientDigProductId: [u16; 33],
    pub ClientSockAddress: WTS_SOCKADDR,
    pub ClientTimeZone: WTS_TIME_ZONE_INFORMATION,
    pub WRdsListenerSettings: WRDS_LISTENER_SETTINGS,
    pub EventLogActivityId: ::windows_sys::core::GUID,
    pub ContextSize: u32,
    pub ContextData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_CONNECTION_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_CONNECTION_SETTINGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WRDS_CONNECTION_SETTING_LEVEL = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_CONNECTION_SETTING_LEVEL_INVALID: WRDS_CONNECTION_SETTING_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_CONNECTION_SETTING_LEVEL_1: WRDS_CONNECTION_SETTING_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_DEVICE_NAME_LENGTH: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_DIRECTORY_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_DOMAIN_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_DRIVER_NAME_LENGTH: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: WTS_SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: WTS_SYSTEMTIME,
    pub DaylightBias: i32,
    pub TimeZoneKeyName: [u16; 128],
    pub DynamicDaylightTimeDisabled: u16,
}
impl ::core::marker::Copy for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_IMEFILENAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_INITIALPROGRAM_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub union WRDS_LISTENER_SETTING {
    pub WRdsListenerSettings1: WRDS_LISTENER_SETTINGS_1,
}
impl ::core::marker::Copy for WRDS_LISTENER_SETTING {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WRDS_LISTENER_SETTINGS {
    pub WRdsListenerSettingLevel: WRDS_LISTENER_SETTING_LEVEL,
    pub WRdsListenerSetting: WRDS_LISTENER_SETTING,
}
impl ::core::marker::Copy for WRDS_LISTENER_SETTINGS {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WRDS_LISTENER_SETTINGS_1 {
    pub MaxProtocolListenerConnectionCount: u32,
    pub SecurityDescriptorSize: u32,
    pub pSecurityDescriptor: *mut u8,
}
impl ::core::marker::Copy for WRDS_LISTENER_SETTINGS_1 {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTINGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WRDS_LISTENER_SETTING_LEVEL = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_LISTENER_SETTING_LEVEL_INVALID: WRDS_LISTENER_SETTING_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_LISTENER_SETTING_LEVEL_1: WRDS_LISTENER_SETTING_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_MAX_CACHE_RESERVED: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_MAX_COUNTERS: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_MAX_PROTOCOL_CACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_MAX_RESERVED: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PASSWORD_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_NOTHING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_THEMING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
pub const WRDS_SERVICE_ID_GRAPHICS_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3533258573, data2: 719, data3: 17024, data4: [140, 72, 22, 36, 180, 79, 135, 6] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WRDS_SETTING {
    pub WRdsSettings1: WRDS_SETTINGS_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_SETTING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_SETTINGS {
    pub WRdsSettingType: WRDS_SETTING_TYPE,
    pub WRdsSettingLevel: WRDS_SETTING_LEVEL,
    pub WRdsSetting: WRDS_SETTING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_SETTINGS_1 {
    pub WRdsDisableClipStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableClipValue: u32,
    pub WRdsDisableLPTStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableLPTValue: u32,
    pub WRdsDisableCcmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCcmValue: u32,
    pub WRdsDisableCdmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCdmValue: u32,
    pub WRdsDisableCpmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCpmValue: u32,
    pub WRdsDisablePnpStatus: WRDS_SETTING_STATUS,
    pub WRdsDisablePnpValue: u32,
    pub WRdsEncryptionLevelStatus: WRDS_SETTING_STATUS,
    pub WRdsEncryptionValue: u32,
    pub WRdsColorDepthStatus: WRDS_SETTING_STATUS,
    pub WRdsColorDepthValue: u32,
    pub WRdsDisableAutoReconnecetStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableAutoReconnecetValue: u32,
    pub WRdsDisableEncryptionStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableEncryptionValue: u32,
    pub WRdsResetBrokenStatus: WRDS_SETTING_STATUS,
    pub WRdsResetBrokenValue: u32,
    pub WRdsMaxIdleTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxIdleTimeValue: u32,
    pub WRdsMaxDisconnectTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxDisconnectTimeValue: u32,
    pub WRdsMaxConnectTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxConnectTimeValue: u32,
    pub WRdsKeepAliveStatus: WRDS_SETTING_STATUS,
    pub WRdsKeepAliveStartValue: super::super::Foundation::BOOLEAN,
    pub WRdsKeepAliveIntervalValue: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_SETTINGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WRDS_SETTING_LEVEL = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_LEVEL_INVALID: WRDS_SETTING_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_LEVEL_1: WRDS_SETTING_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WRDS_SETTING_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_STATUS_NOTAPPLICABLE: WRDS_SETTING_STATUS = -1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_STATUS_DISABLED: WRDS_SETTING_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_STATUS_ENABLED: WRDS_SETTING_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_STATUS_NOTCONFIGURED: WRDS_SETTING_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WRDS_SETTING_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_TYPE_INVALID: WRDS_SETTING_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_TYPE_MACHINE: WRDS_SETTING_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_TYPE_USER: WRDS_SETTING_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_SETTING_TYPE_SAM: WRDS_SETTING_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_USERNAME_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_VALUE_TYPE_BINARY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_VALUE_TYPE_GUID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_VALUE_TYPE_STRING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRDS_VALUE_TYPE_ULONG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WRdsGraphicsChannelType = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRdsGraphicsChannelType_GuaranteedDelivery: WRdsGraphicsChannelType = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRdsGraphicsChannelType_BestEffortDelivery: WRdsGraphicsChannelType = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WRdsGraphicsChannels_LossyChannelMaxMessageSize: u32 = 988u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSCLIENTA {
    pub ClientName: [super::super::Foundation::CHAR; 21],
    pub Domain: [super::super::Foundation::CHAR; 18],
    pub UserName: [super::super::Foundation::CHAR; 21],
    pub WorkDirectory: [super::super::Foundation::CHAR; 261],
    pub InitialProgram: [super::super::Foundation::CHAR; 261],
    pub EncryptionLevel: u8,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub ClientDirectory: [super::super::Foundation::CHAR; 261],
    pub ClientBuildNumber: u32,
    pub ClientHardwareId: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub DeviceId: [super::super::Foundation::CHAR; 261],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSCLIENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSCLIENTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSCLIENTW {
    pub ClientName: [u16; 21],
    pub Domain: [u16; 18],
    pub UserName: [u16; 21],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
    pub EncryptionLevel: u8,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub ClientDirectory: [u16; 261],
    pub ClientBuildNumber: u32,
    pub ClientHardwareId: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub DeviceId: [u16; 261],
}
impl ::core::marker::Copy for WTSCLIENTW {}
impl ::core::clone::Clone for WTSCLIENTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSCONFIGINFOA {
    pub version: u32,
    pub fConnectClientDrivesAtLogon: u32,
    pub fConnectPrinterAtLogon: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub ShadowSettings: u32,
    pub LogonUserName: [super::super::Foundation::CHAR; 21],
    pub LogonDomain: [super::super::Foundation::CHAR; 18],
    pub WorkDirectory: [super::super::Foundation::CHAR; 261],
    pub InitialProgram: [super::super::Foundation::CHAR; 261],
    pub ApplicationName: [super::super::Foundation::CHAR; 261],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSCONFIGINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSCONFIGINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSCONFIGINFOW {
    pub version: u32,
    pub fConnectClientDrivesAtLogon: u32,
    pub fConnectPrinterAtLogon: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub ShadowSettings: u32,
    pub LogonUserName: [u16; 21],
    pub LogonDomain: [u16; 18],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
    pub ApplicationName: [u16; 261],
}
impl ::core::marker::Copy for WTSCONFIGINFOW {}
impl ::core::clone::Clone for WTSCONFIGINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSINFOA {
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBy: u32,
    pub WinStationName: [super::super::Foundation::CHAR; 32],
    pub Domain: [super::super::Foundation::CHAR; 17],
    pub UserName: [super::super::Foundation::CHAR; 21],
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub LogonTime: i64,
    pub CurrentTime: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSINFOEXA {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_A,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSINFOEXW {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_W,
}
impl ::core::marker::Copy for WTSINFOEXW {}
impl ::core::clone::Clone for WTSINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSINFOEX_LEVEL1_A {
    pub SessionId: u32,
    pub SessionState: WTS_CONNECTSTATE_CLASS,
    pub SessionFlags: i32,
    pub WinStationName: [super::super::Foundation::CHAR; 33],
    pub UserName: [super::super::Foundation::CHAR; 21],
    pub DomainName: [super::super::Foundation::CHAR; 18],
    pub LogonTime: i64,
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub CurrentTime: i64,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSINFOEX_LEVEL1_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSINFOEX_LEVEL1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSINFOEX_LEVEL1_W {
    pub SessionId: u32,
    pub SessionState: WTS_CONNECTSTATE_CLASS,
    pub SessionFlags: i32,
    pub WinStationName: [u16; 33],
    pub UserName: [u16; 21],
    pub DomainName: [u16; 18],
    pub LogonTime: i64,
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub CurrentTime: i64,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
}
impl ::core::marker::Copy for WTSINFOEX_LEVEL1_W {}
impl ::core::clone::Clone for WTSINFOEX_LEVEL1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WTSINFOEX_LEVEL_A {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_A,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSINFOEX_LEVEL_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSINFOEX_LEVEL_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub union WTSINFOEX_LEVEL_W {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_W,
}
impl ::core::marker::Copy for WTSINFOEX_LEVEL_W {}
impl ::core::clone::Clone for WTSINFOEX_LEVEL_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSINFOW {
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
    pub WinStationName: [u16; 32],
    pub Domain: [u16; 17],
    pub UserName: [u16; 21],
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub LogonTime: i64,
    pub CurrentTime: i64,
}
impl ::core::marker::Copy for WTSINFOW {}
impl ::core::clone::Clone for WTSINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSLISTENERCONFIGA {
    pub version: u32,
    pub fEnableListener: u32,
    pub MaxConnectionCount: u32,
    pub fPromptForPassword: u32,
    pub fInheritColorDepth: u32,
    pub ColorDepth: u32,
    pub fInheritBrokenTimeoutSettings: u32,
    pub BrokenTimeoutSettings: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDriveRedirection: u32,
    pub fDisableComPortRedirection: u32,
    pub fDisableLPTPortRedirection: u32,
    pub fDisableClipboardRedirection: u32,
    pub fDisableAudioRedirection: u32,
    pub fDisablePNPRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub LanAdapter: u32,
    pub PortNumber: u32,
    pub fInheritShadowSettings: u32,
    pub ShadowSettings: u32,
    pub TimeoutSettingsConnection: u32,
    pub TimeoutSettingsDisconnection: u32,
    pub TimeoutSettingsIdle: u32,
    pub SecurityLayer: u32,
    pub MinEncryptionLevel: u32,
    pub UserAuthentication: u32,
    pub Comment: [super::super::Foundation::CHAR; 61],
    pub LogonUserName: [super::super::Foundation::CHAR; 21],
    pub LogonDomain: [super::super::Foundation::CHAR; 18],
    pub WorkDirectory: [super::super::Foundation::CHAR; 261],
    pub InitialProgram: [super::super::Foundation::CHAR; 261],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSLISTENERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSLISTENERCONFIGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSLISTENERCONFIGW {
    pub version: u32,
    pub fEnableListener: u32,
    pub MaxConnectionCount: u32,
    pub fPromptForPassword: u32,
    pub fInheritColorDepth: u32,
    pub ColorDepth: u32,
    pub fInheritBrokenTimeoutSettings: u32,
    pub BrokenTimeoutSettings: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDriveRedirection: u32,
    pub fDisableComPortRedirection: u32,
    pub fDisableLPTPortRedirection: u32,
    pub fDisableClipboardRedirection: u32,
    pub fDisableAudioRedirection: u32,
    pub fDisablePNPRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub LanAdapter: u32,
    pub PortNumber: u32,
    pub fInheritShadowSettings: u32,
    pub ShadowSettings: u32,
    pub TimeoutSettingsConnection: u32,
    pub TimeoutSettingsDisconnection: u32,
    pub TimeoutSettingsIdle: u32,
    pub SecurityLayer: u32,
    pub MinEncryptionLevel: u32,
    pub UserAuthentication: u32,
    pub Comment: [u16; 61],
    pub LogonUserName: [u16; 21],
    pub LogonDomain: [u16; 18],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
}
impl ::core::marker::Copy for WTSLISTENERCONFIGW {}
impl ::core::clone::Clone for WTSLISTENERCONFIGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTSSBX_ADDRESS_FAMILY = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_ADDRESS_FAMILY_AF_UNSPEC: WTSSBX_ADDRESS_FAMILY = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_ADDRESS_FAMILY_AF_INET: WTSSBX_ADDRESS_FAMILY = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_ADDRESS_FAMILY_AF_INET6: WTSSBX_ADDRESS_FAMILY = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_ADDRESS_FAMILY_AF_IPX: WTSSBX_ADDRESS_FAMILY = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_ADDRESS_FAMILY_AF_NETBIOS: WTSSBX_ADDRESS_FAMILY = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSSBX_IP_ADDRESS {
    pub AddressFamily: WTSSBX_ADDRESS_FAMILY,
    pub Address: [u8; 16],
    pub PortNumber: u16,
    pub dwScope: u32,
}
impl ::core::marker::Copy for WTSSBX_IP_ADDRESS {}
impl ::core::clone::Clone for WTSSBX_IP_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSSBX_MACHINE_CONNECT_INFO {
    pub wczMachineFQDN: [u16; 257],
    pub wczMachineNetBiosName: [u16; 17],
    pub dwNumOfIPAddr: u32,
    pub IPaddr: [WTSSBX_IP_ADDRESS; 12],
}
impl ::core::marker::Copy for WTSSBX_MACHINE_CONNECT_INFO {}
impl ::core::clone::Clone for WTSSBX_MACHINE_CONNECT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTSSBX_MACHINE_DRAIN = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_DRAIN_UNSPEC: WTSSBX_MACHINE_DRAIN = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_DRAIN_OFF: WTSSBX_MACHINE_DRAIN = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_DRAIN_ON: WTSSBX_MACHINE_DRAIN = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSSBX_MACHINE_INFO {
    pub ClientConnectInfo: WTSSBX_MACHINE_CONNECT_INFO,
    pub wczFarmName: [u16; 257],
    pub InternalIPAddress: WTSSBX_IP_ADDRESS,
    pub dwMaxSessionsLimit: u32,
    pub ServerWeight: u32,
    pub SingleSessionMode: WTSSBX_MACHINE_SESSION_MODE,
    pub InDrain: WTSSBX_MACHINE_DRAIN,
    pub MachineState: WTSSBX_MACHINE_STATE,
}
impl ::core::marker::Copy for WTSSBX_MACHINE_INFO {}
impl ::core::clone::Clone for WTSSBX_MACHINE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTSSBX_MACHINE_SESSION_MODE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_SESSION_MODE_UNSPEC: WTSSBX_MACHINE_SESSION_MODE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_SESSION_MODE_SINGLE: WTSSBX_MACHINE_SESSION_MODE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_SESSION_MODE_MULTIPLE: WTSSBX_MACHINE_SESSION_MODE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTSSBX_MACHINE_STATE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_STATE_UNSPEC: WTSSBX_MACHINE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_STATE_READY: WTSSBX_MACHINE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_MACHINE_STATE_SYNCHRONIZING: WTSSBX_MACHINE_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTSSBX_NOTIFICATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_NOTIFICATION_REMOVED: WTSSBX_NOTIFICATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_NOTIFICATION_CHANGED: WTSSBX_NOTIFICATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_NOTIFICATION_ADDED: WTSSBX_NOTIFICATION_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_NOTIFICATION_RESYNC: WTSSBX_NOTIFICATION_TYPE = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSSBX_SESSION_INFO {
    pub wszUserName: [u16; 105],
    pub wszDomainName: [u16; 257],
    pub ApplicationType: [u16; 257],
    pub dwSessionId: u32,
    pub CreateTime: super::super::Foundation::FILETIME,
    pub DisconnectTime: super::super::Foundation::FILETIME,
    pub SessionState: WTSSBX_SESSION_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSSBX_SESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSSBX_SESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTSSBX_SESSION_STATE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_SESSION_STATE_UNSPEC: WTSSBX_SESSION_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_SESSION_STATE_ACTIVE: WTSSBX_SESSION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSBX_SESSION_STATE_DISCONNECTED: WTSSBX_SESSION_STATE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSSESSION_NOTIFICATION {
    pub cbSize: u32,
    pub dwSessionId: u32,
}
impl ::core::marker::Copy for WTSSESSION_NOTIFICATION {}
impl ::core::clone::Clone for WTSSESSION_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSUSERCONFIGA {
    pub Source: u32,
    pub InheritInitialProgram: u32,
    pub AllowLogonTerminalServer: u32,
    pub TimeoutSettingsConnections: u32,
    pub TimeoutSettingsDisconnections: u32,
    pub TimeoutSettingsIdle: u32,
    pub DeviceClientDrives: u32,
    pub DeviceClientPrinters: u32,
    pub ClientDefaultPrinter: u32,
    pub BrokenTimeoutSettings: u32,
    pub ReconnectSettings: u32,
    pub ShadowingSettings: u32,
    pub TerminalServerRemoteHomeDir: u32,
    pub InitialProgram: [super::super::Foundation::CHAR; 261],
    pub WorkDirectory: [super::super::Foundation::CHAR; 261],
    pub TerminalServerProfilePath: [super::super::Foundation::CHAR; 261],
    pub TerminalServerHomeDir: [super::super::Foundation::CHAR; 261],
    pub TerminalServerHomeDirDrive: [super::super::Foundation::CHAR; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSUSERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSUSERCONFIGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTSUSERCONFIGW {
    pub Source: u32,
    pub InheritInitialProgram: u32,
    pub AllowLogonTerminalServer: u32,
    pub TimeoutSettingsConnections: u32,
    pub TimeoutSettingsDisconnections: u32,
    pub TimeoutSettingsIdle: u32,
    pub DeviceClientDrives: u32,
    pub DeviceClientPrinters: u32,
    pub ClientDefaultPrinter: u32,
    pub BrokenTimeoutSettings: u32,
    pub ReconnectSettings: u32,
    pub ShadowingSettings: u32,
    pub TerminalServerRemoteHomeDir: u32,
    pub InitialProgram: [u16; 261],
    pub WorkDirectory: [u16; 261],
    pub TerminalServerProfilePath: [u16; 261],
    pub TerminalServerHomeDir: [u16; 261],
    pub TerminalServerHomeDirDrive: [u16; 4],
}
impl ::core::marker::Copy for WTSUSERCONFIGW {}
impl ::core::clone::Clone for WTSUSERCONFIGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_CACHE_STATS {
    pub Specific: u32,
    pub Data: WTS_CACHE_STATS_UN,
    pub ProtocolType: u16,
    pub Length: u16,
}
impl ::core::marker::Copy for WTS_CACHE_STATS {}
impl ::core::clone::Clone for WTS_CACHE_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub union WTS_CACHE_STATS_UN {
    pub ProtocolCache: [WTS_PROTOCOL_CACHE; 4],
    pub TShareCacheStats: u32,
    pub Reserved: [u32; 20],
}
impl ::core::marker::Copy for WTS_CACHE_STATS_UN {}
impl ::core::clone::Clone for WTS_CACHE_STATS_UN {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTS_CERT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CERT_TYPE_INVALID: WTS_CERT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CERT_TYPE_PROPRIETORY: WTS_CERT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CERT_TYPE_X509: WTS_CERT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_NO_COMPRESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_HIGH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_LOW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_MED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_REAL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CLIENTADDRESS_LENGTH: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CLIENTNAME_LENGTH: u32 = 20u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_CLIENT_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl ::core::marker::Copy for WTS_CLIENT_ADDRESS {}
impl ::core::clone::Clone for WTS_CLIENT_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_CLIENT_DATA {
    pub fDisableCtrlAltDel: super::super::Foundation::BOOLEAN,
    pub fDoubleClickDetect: super::super::Foundation::BOOLEAN,
    pub fEnableWindowsKey: super::super::Foundation::BOOLEAN,
    pub fHideTitleBar: super::super::Foundation::BOOLEAN,
    pub fInheritAutoLogon: super::super::Foundation::BOOL,
    pub fPromptForPassword: super::super::Foundation::BOOLEAN,
    pub fUsingSavedCreds: super::super::Foundation::BOOLEAN,
    pub Domain: [u16; 256],
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub fPasswordIsScPin: super::super::Foundation::BOOLEAN,
    pub fInheritInitialProgram: super::super::Foundation::BOOL,
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub fMaximizeShell: super::super::Foundation::BOOLEAN,
    pub EncryptionLevel: u8,
    pub PerformanceFlags: u32,
    pub ProtocolName: [u16; 9],
    pub ProtocolType: u16,
    pub fInheritColorDepth: super::super::Foundation::BOOL,
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub DisplayDriverName: [u16; 9],
    pub DisplayDeviceName: [u16; 20],
    pub fMouse: super::super::Foundation::BOOLEAN,
    pub KeyboardLayout: u32,
    pub KeyboardType: u32,
    pub KeyboardSubType: u32,
    pub KeyboardFunctionKey: u32,
    pub imeFileName: [u16; 33],
    pub ActiveInputLocale: u32,
    pub fNoAudioPlayback: super::super::Foundation::BOOLEAN,
    pub fRemoteConsoleAudio: super::super::Foundation::BOOLEAN,
    pub AudioDriverName: [u16; 9],
    pub ClientTimeZone: WTS_TIME_ZONE_INFORMATION,
    pub ClientName: [u16; 21],
    pub SerialNumber: u32,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub ClientSockAddress: WTS_SOCKADDR,
    pub ClientDirectory: [u16; 257],
    pub ClientBuildNumber: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub ClientSessionId: u32,
    pub ClientDigProductId: [u16; 33],
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fDisablePNP: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_CLIENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_CLIENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
impl ::core::marker::Copy for WTS_CLIENT_DISPLAY {}
impl ::core::clone::Clone for WTS_CLIENT_DISPLAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_COMMENT_LENGTH: u32 = 60u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTS_CONFIG_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigInitialProgram: WTS_CONFIG_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigWorkingDirectory: WTS_CONFIG_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigfInheritInitialProgram: WTS_CONFIG_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigfAllowLogonTerminalServer: WTS_CONFIG_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigTimeoutSettingsConnections: WTS_CONFIG_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigTimeoutSettingsDisconnections: WTS_CONFIG_CLASS = 5i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigTimeoutSettingsIdle: WTS_CONFIG_CLASS = 6i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigfDeviceClientDrives: WTS_CONFIG_CLASS = 7i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigfDeviceClientPrinters: WTS_CONFIG_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigfDeviceClientDefaultPrinter: WTS_CONFIG_CLASS = 9i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigBrokenTimeoutSettings: WTS_CONFIG_CLASS = 10i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigReconnectSettings: WTS_CONFIG_CLASS = 11i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigModemCallbackSettings: WTS_CONFIG_CLASS = 12i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigModemCallbackPhoneNumber: WTS_CONFIG_CLASS = 13i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigShadowingSettings: WTS_CONFIG_CLASS = 14i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigTerminalServerProfilePath: WTS_CONFIG_CLASS = 15i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigTerminalServerHomeDir: WTS_CONFIG_CLASS = 16i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigTerminalServerHomeDirDrive: WTS_CONFIG_CLASS = 17i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigfTerminalServerRemoteHomeDir: WTS_CONFIG_CLASS = 18i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigUser: WTS_CONFIG_CLASS = 19i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTS_CONFIG_SOURCE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserConfigSourceSAM: WTS_CONFIG_SOURCE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTS_CONNECTSTATE_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSActive: WTS_CONNECTSTATE_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSConnected: WTS_CONNECTSTATE_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSConnectQuery: WTS_CONNECTSTATE_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSShadow: WTS_CONNECTSTATE_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSDisconnected: WTS_CONNECTSTATE_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSIdle: WTS_CONNECTSTATE_CLASS = 5i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSListen: WTS_CONNECTSTATE_CLASS = 6i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSReset: WTS_CONNECTSTATE_CLASS = 7i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSDown: WTS_CONNECTSTATE_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSInit: WTS_CONNECTSTATE_CLASS = 9i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_CURRENT_SESSION: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DEVICE_NAME_LENGTH: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DIRECTORY_LENGTH: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_DISPLAY_IOCTL {
    pub pDisplayIOCtlData: [u8; 256],
    pub cbDisplayIOCtlData: u32,
}
impl ::core::marker::Copy for WTS_DISPLAY_IOCTL {}
impl ::core::clone::Clone for WTS_DISPLAY_IOCTL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DOMAIN_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DRIVER_NAME_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DRIVE_LENGTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_ALL: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_CONNECT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_CREATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_DELETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_DISCONNECT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_FLUSH: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_LICENSE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_LOGOFF: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_LOGON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_RENAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_EVENT_STATECHANGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_IMEFILENAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTS_INFO_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSInitialProgram: WTS_INFO_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSApplicationName: WTS_INFO_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSWorkingDirectory: WTS_INFO_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSOEMId: WTS_INFO_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSessionId: WTS_INFO_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSUserName: WTS_INFO_CLASS = 5i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSWinStationName: WTS_INFO_CLASS = 6i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSDomainName: WTS_INFO_CLASS = 7i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSConnectState: WTS_INFO_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientBuildNumber: WTS_INFO_CLASS = 9i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientName: WTS_INFO_CLASS = 10i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientDirectory: WTS_INFO_CLASS = 11i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientProductId: WTS_INFO_CLASS = 12i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientHardwareId: WTS_INFO_CLASS = 13i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientAddress: WTS_INFO_CLASS = 14i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientDisplay: WTS_INFO_CLASS = 15i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientProtocolType: WTS_INFO_CLASS = 16i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSIdleTime: WTS_INFO_CLASS = 17i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSLogonTime: WTS_INFO_CLASS = 18i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSIncomingBytes: WTS_INFO_CLASS = 19i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSOutgoingBytes: WTS_INFO_CLASS = 20i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSIncomingFrames: WTS_INFO_CLASS = 21i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSOutgoingFrames: WTS_INFO_CLASS = 22i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSClientInfo: WTS_INFO_CLASS = 23i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSessionInfo: WTS_INFO_CLASS = 24i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSessionInfoEx: WTS_INFO_CLASS = 25i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSConfigInfo: WTS_INFO_CLASS = 26i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSValidationInfo: WTS_INFO_CLASS = 27i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSSessionAddressV4: WTS_INFO_CLASS = 28i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSIsRemoteSession: WTS_INFO_CLASS = 29i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_INITIALPROGRAM_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_LICENSE_CAPABILITIES {
    pub KeyExchangeAlg: u32,
    pub ProtocolVer: u32,
    pub fAuthenticateServer: super::super::Foundation::BOOL,
    pub CertType: WTS_CERT_TYPE,
    pub cbClientName: u32,
    pub rgbClientName: [u8; 42],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_LICENSE_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_LICENSE_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LISTENER_CREATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LISTENER_NAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LISTENER_UPDATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LOGON_ERR_INVALID: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LOGON_ERR_NOT_HANDLED: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LOGON_ERR_HANDLED_SHOW: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = 3i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW_START_OVER: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_MAX_CACHE_RESERVED: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_MAX_COUNTERS: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_MAX_PROTOCOL_CACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_MAX_RESERVED: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PASSWORD_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_NOTHING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_THEMING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_POLICY_DATA {
    pub fDisableEncryption: super::super::Foundation::BOOLEAN,
    pub fDisableAutoReconnect: super::super::Foundation::BOOLEAN,
    pub ColorDepth: u32,
    pub MinEncryptionLevel: u8,
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fDisablePNPRedir: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_POLICY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_POLICY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFOA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: ::windows_sys::core::PSTR,
    pub pUserSid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFOW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: ::windows_sys::core::PWSTR,
    pub pUserSid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFO_EXA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: ::windows_sys::core::PSTR,
    pub pUserSid: super::super::Foundation::PSID,
    pub NumberOfThreads: u32,
    pub HandleCount: u32,
    pub PagefileUsage: u32,
    pub PeakPagefileUsage: u32,
    pub WorkingSetSize: u32,
    pub PeakWorkingSetSize: u32,
    pub UserTime: i64,
    pub KernelTime: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFO_EXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFO_EXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFO_EXW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: ::windows_sys::core::PWSTR,
    pub pUserSid: super::super::Foundation::PSID,
    pub NumberOfThreads: u32,
    pub HandleCount: u32,
    pub PagefileUsage: u32,
    pub PeakPagefileUsage: u32,
    pub WorkingSetSize: u32,
    pub PeakWorkingSetSize: u32,
    pub UserTime: i64,
    pub KernelTime: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFO_EXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFO_EXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROCESS_INFO_LEVEL_0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROCESS_INFO_LEVEL_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROPERTY_DEFAULT_CONFIG: &str = "DefaultConfig";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_PROPERTY_VALUE {
    pub Type: u16,
    pub u: WTS_PROPERTY_VALUE_0,
}
impl ::core::marker::Copy for WTS_PROPERTY_VALUE {}
impl ::core::clone::Clone for WTS_PROPERTY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub union WTS_PROPERTY_VALUE_0 {
    pub ulVal: u32,
    pub strVal: WTS_PROPERTY_VALUE_0_1,
    pub bVal: WTS_PROPERTY_VALUE_0_0,
    pub guidVal: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for WTS_PROPERTY_VALUE_0 {}
impl ::core::clone::Clone for WTS_PROPERTY_VALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_PROPERTY_VALUE_0_0 {
    pub size: u32,
    pub pbVal: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for WTS_PROPERTY_VALUE_0_0 {}
impl ::core::clone::Clone for WTS_PROPERTY_VALUE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_PROPERTY_VALUE_0_1 {
    pub size: u32,
    pub pstrVal: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WTS_PROPERTY_VALUE_0_1 {}
impl ::core::clone::Clone for WTS_PROPERTY_VALUE_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_PROTOCOL_CACHE {
    pub CacheReads: u32,
    pub CacheHits: u32,
}
impl ::core::marker::Copy for WTS_PROTOCOL_CACHE {}
impl ::core::clone::Clone for WTS_PROTOCOL_CACHE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_PROTOCOL_COUNTERS {
    pub WdBytes: u32,
    pub WdFrames: u32,
    pub WaitForOutBuf: u32,
    pub Frames: u32,
    pub Bytes: u32,
    pub CompressedBytes: u32,
    pub CompressFlushes: u32,
    pub Errors: u32,
    pub Timeouts: u32,
    pub AsyncFramingError: u32,
    pub AsyncOverrunError: u32,
    pub AsyncOverflowError: u32,
    pub AsyncParityError: u32,
    pub TdErrors: u32,
    pub ProtocolType: u16,
    pub Length: u16,
    pub Specific: u16,
    pub Reserved: [u32; 100],
}
impl ::core::marker::Copy for WTS_PROTOCOL_COUNTERS {}
impl ::core::clone::Clone for WTS_PROTOCOL_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_PROTOCOL_STATUS {
    pub Output: WTS_PROTOCOL_COUNTERS,
    pub Input: WTS_PROTOCOL_COUNTERS,
    pub Cache: WTS_CACHE_STATS,
    pub AsyncSignal: u32,
    pub AsyncSignalMask: u32,
    pub Counters: [i64; 100],
}
impl ::core::marker::Copy for WTS_PROTOCOL_STATUS {}
impl ::core::clone::Clone for WTS_PROTOCOL_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROTOCOL_TYPE_CONSOLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROTOCOL_TYPE_ICA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_PROTOCOL_TYPE_RDP: u32 = 2u32;
pub const WTS_QUERY_ALLOWED_INITIAL_APP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3346864944, data2: 23521, data3: 19563, data4: [160, 225, 189, 109, 46, 92, 159, 204] };
pub const WTS_QUERY_AUDIOENUM_DLL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2616523415, data2: 51331, data3: 19498, data4: [128, 171, 90, 57, 201, 175, 0, 219] };
pub const WTS_QUERY_LOGON_SCREEN_SIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2341343207, data2: 2052, data3: 18958, data4: [178, 121, 134, 96, 177, 223, 0, 73] };
pub const WTS_QUERY_MF_FORMAT_SUPPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1099340496, data2: 25394, data3: 19912, data4: [149, 213, 219, 116, 158, 47, 29, 148] };
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTS_RCM_DRAIN_STATE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DRAIN_STATE_NONE: WTS_RCM_DRAIN_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DRAIN_IN_DRAIN: WTS_RCM_DRAIN_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_DRAIN_NOT_IN_DRAIN: WTS_RCM_DRAIN_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTS_RCM_SERVICE_STATE = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SERVICE_NONE: WTS_RCM_SERVICE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SERVICE_START: WTS_RCM_SERVICE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SERVICE_STOP: WTS_RCM_SERVICE_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_CONNECT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_DISCONNECT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_GUEST_ACCESS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_LOGOFF: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_LOGON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_MESSAGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_QUERY_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_REMOTE_CONTROL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_RESET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_SET_INFORMATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SECURITY_VIRTUAL_CHANNELS: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SERVER_INFOA {
    pub pServerName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for WTS_SERVER_INFOA {}
impl ::core::clone::Clone for WTS_SERVER_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SERVER_INFOW {
    pub pServerName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WTS_SERVER_INFOW {}
impl ::core::clone::Clone for WTS_SERVER_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SERVICE_STATE {
    pub RcmServiceState: WTS_RCM_SERVICE_STATE,
    pub RcmDrainState: WTS_RCM_DRAIN_STATE,
}
impl ::core::marker::Copy for WTS_SERVICE_STATE {}
impl ::core::clone::Clone for WTS_SERVICE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SESSIONSTATE_LOCK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SESSIONSTATE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_SESSIONSTATE_UNLOCK: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SESSION_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl ::core::marker::Copy for WTS_SESSION_ADDRESS {}
impl ::core::clone::Clone for WTS_SESSION_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SESSION_ID {
    pub SessionUniqueGuid: ::windows_sys::core::GUID,
    pub SessionId: u32,
}
impl ::core::marker::Copy for WTS_SESSION_ID {}
impl ::core::clone::Clone for WTS_SESSION_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SESSION_INFOA {
    pub SessionId: u32,
    pub pWinStationName: ::windows_sys::core::PSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
impl ::core::marker::Copy for WTS_SESSION_INFOA {}
impl ::core::clone::Clone for WTS_SESSION_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SESSION_INFOW {
    pub SessionId: u32,
    pub pWinStationName: ::windows_sys::core::PWSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
impl ::core::marker::Copy for WTS_SESSION_INFOW {}
impl ::core::clone::Clone for WTS_SESSION_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SESSION_INFO_1A {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: ::windows_sys::core::PSTR,
    pub pHostName: ::windows_sys::core::PSTR,
    pub pUserName: ::windows_sys::core::PSTR,
    pub pDomainName: ::windows_sys::core::PSTR,
    pub pFarmName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for WTS_SESSION_INFO_1A {}
impl ::core::clone::Clone for WTS_SESSION_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SESSION_INFO_1W {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: ::windows_sys::core::PWSTR,
    pub pHostName: ::windows_sys::core::PWSTR,
    pub pUserName: ::windows_sys::core::PWSTR,
    pub pDomainName: ::windows_sys::core::PWSTR,
    pub pFarmName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WTS_SESSION_INFO_1W {}
impl ::core::clone::Clone for WTS_SESSION_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl ::core::marker::Copy for WTS_SMALL_RECT {}
impl ::core::clone::Clone for WTS_SMALL_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SOCKADDR {
    pub sin_family: u16,
    pub u: WTS_SOCKADDR_0,
}
impl ::core::marker::Copy for WTS_SOCKADDR {}
impl ::core::clone::Clone for WTS_SOCKADDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub union WTS_SOCKADDR_0 {
    pub ipv4: WTS_SOCKADDR_0_0,
    pub ipv6: WTS_SOCKADDR_0_1,
}
impl ::core::marker::Copy for WTS_SOCKADDR_0 {}
impl ::core::clone::Clone for WTS_SOCKADDR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SOCKADDR_0_0 {
    pub sin_port: u16,
    pub IN_ADDR: u32,
    pub sin_zero: [u8; 8],
}
impl ::core::marker::Copy for WTS_SOCKADDR_0_0 {}
impl ::core::clone::Clone for WTS_SOCKADDR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SOCKADDR_0_1 {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl ::core::marker::Copy for WTS_SOCKADDR_0_1 {}
impl ::core::clone::Clone for WTS_SOCKADDR_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_SYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
impl ::core::marker::Copy for WTS_SYSTEMTIME {}
impl ::core::clone::Clone for WTS_SYSTEMTIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: WTS_SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: WTS_SYSTEMTIME,
    pub DaylightBias: i32,
}
impl ::core::marker::Copy for WTS_TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for WTS_TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTS_TYPE_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSTypeProcessInfoLevel0: WTS_TYPE_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSTypeProcessInfoLevel1: WTS_TYPE_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSTypeSessionInfoLevel1: WTS_TYPE_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_USERNAME_LENGTH: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_USER_CREDENTIAL {
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub Domain: [u16; 256],
}
impl ::core::marker::Copy for WTS_USER_CREDENTIAL {}
impl ::core::clone::Clone for WTS_USER_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_USER_DATA {
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub UserTimeZone: WTS_TIME_ZONE_INFORMATION,
}
impl ::core::marker::Copy for WTS_USER_DATA {}
impl ::core::clone::Clone for WTS_USER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_VALIDATION_INFORMATIONA {
    pub ProductInfo: _WTS_PRODUCT_INFOA,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_VALIDATION_INFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_VALIDATION_INFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct WTS_VALIDATION_INFORMATIONW {
    pub ProductInfo: _WTS_PRODUCT_INFOW,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
impl ::core::marker::Copy for WTS_VALIDATION_INFORMATIONW {}
impl ::core::clone::Clone for WTS_VALIDATION_INFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_VALUE_TYPE_BINARY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_VALUE_TYPE_GUID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_VALUE_TYPE_STRING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_VALUE_TYPE_ULONG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub type WTS_VIRTUAL_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSVirtualClientData: WTS_VIRTUAL_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTSVirtualFileHandle: WTS_VIRTUAL_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_WSD_FASTREBOOT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_WSD_LOGOFF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_WSD_POWEROFF: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_WSD_REBOOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub const WTS_WSD_SHUTDOWN: u32 = 2u32;
pub const Workspace: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1327365286, data2: 15021, data3: 18657, data4: [132, 6, 75, 194, 26, 80, 29, 124] };
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _ITSWkspEvents {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _ITSWkspEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3106061240, data2: 19541, data3: 20458, data4: [132, 150, 190, 176, 180, 66, 133, 233] };
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct _WTS_PRODUCT_INFOA {
    pub CompanyName: [super::super::Foundation::CHAR; 256],
    pub ProductID: [super::super::Foundation::CHAR; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _WTS_PRODUCT_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _WTS_PRODUCT_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct _WTS_PRODUCT_INFOW {
    pub CompanyName: [u16; 256],
    pub ProductID: [u16; 4],
}
impl ::core::marker::Copy for _WTS_PRODUCT_INFOW {}
impl ::core::clone::Clone for _WTS_PRODUCT_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct pluginResource {
    pub alias: [u16; 256],
    pub name: [u16; 256],
    pub resourceFileContents: ::windows_sys::core::PWSTR,
    pub fileExtension: [u16; 256],
    pub resourcePluginType: [u16; 256],
    pub isDiscoverable: u8,
    pub resourceType: i32,
    pub pceIconSize: u32,
    pub iconContents: *mut u8,
    pub pcePluginBlobSize: u32,
    pub blobContents: *mut u8,
}
impl ::core::marker::Copy for pluginResource {}
impl ::core::clone::Clone for pluginResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct pluginResource2 {
    pub resourceV1: pluginResource,
    pub pceFileAssocListSize: u32,
    pub fileAssocList: *mut pluginResource2FileAssociation,
    pub securityDescriptor: ::windows_sys::core::PWSTR,
    pub pceFolderListSize: u32,
    pub folderList: *mut *mut u16,
}
impl ::core::marker::Copy for pluginResource2 {}
impl ::core::clone::Clone for pluginResource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteDesktop\"`*"]
pub struct pluginResource2FileAssociation {
    pub extName: [u16; 256],
    pub primaryHandler: u8,
    pub pceIconSize: u32,
    pub iconContents: *mut u8,
}
impl ::core::marker::Copy for pluginResource2FileAssociation {}
impl ::core::clone::Clone for pluginResource2FileAssociation {
    fn clone(&self) -> Self {
        *self
    }
}
