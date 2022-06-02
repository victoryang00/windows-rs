#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeAsyncRecvDone(hclientrequest: super::super::Foundation::HANDLE, action: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeDhcpAppendOption(preplypacket: *mut ::core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, boption: u8, boptionlen: u8, pvalue: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeDhcpAppendOptionRaw(preplypacket: *mut ::core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, ubufferlen: u16, pbuffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeDhcpGetOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, uinstance: u32, boption: u8, pboptionlen: *mut u8, ppoptionvalue: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeDhcpGetVendorOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, boption: u8, uinstance: u32, pboptionlen: *mut u8, ppoptionvalue: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeDhcpInitialize(precvpacket: *const ::core::ffi::c_void, urecvpacketlen: u32, preplypacket: *mut ::core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeDhcpIsValid(ppacket: *const ::core::ffi::c_void, upacketlen: u32, brequestpacket: super::super::Foundation::BOOL, pbpxeoptionpresent: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeDhcpv6AppendOption(preply: *mut ::core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, woptiontype: u16, cboption: u16, poption: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeDhcpv6AppendOptionRaw(preply: *mut ::core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, cbbuffer: u16, pbuffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeDhcpv6CreateRelayRepl(prelaymessages: *const PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages: u32, pinnerpacket: *const u8, cbinnerpacket: u32, preplybuffer: *mut ::core::ffi::c_void, cbreplybuffer: u32, pcbreplybuffer: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeDhcpv6GetOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, uinstance: u32, woption: u16, pwoptionlen: *mut u16, ppoptionvalue: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeDhcpv6GetVendorOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, dwenterprisenumber: u32, woption: u16, uinstance: u32, pwoptionlen: *mut u16, ppoptionvalue: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeDhcpv6Initialize(prequest: *const ::core::ffi::c_void, cbrequest: u32, preply: *mut ::core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeDhcpv6IsValid(ppacket: *const ::core::ffi::c_void, upacketlen: u32, brequestpacket: super::super::Foundation::BOOL, pbpxeoptionpresent: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeDhcpv6ParseRelayForw(prelayforwpacket: *const ::core::ffi::c_void, urelayforwpacketlen: u32, prelaymessages: *mut PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages: u32, pnrelaymessages: *mut u32, ppinnerpacket: *mut *mut u8, pcbinnerpacket: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeGetServerInfo(uinfotype: u32, pbuffer: *mut ::core::ffi::c_void, ubufferlen: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeGetServerInfoEx(uinfotype: u32, pbuffer: *mut ::core::ffi::c_void, ubufferlen: u32, pubufferused: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxePacketAllocate(hprovider: super::super::Foundation::HANDLE, hclientrequest: super::super::Foundation::HANDLE, usize: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxePacketFree(hprovider: super::super::Foundation::HANDLE, hclientrequest: super::super::Foundation::HANDLE, ppacket: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderEnumClose(henum: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderEnumFirst(phenum: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderEnumNext(henum: super::super::Foundation::HANDLE, ppprovider: *mut *mut PXE_PROVIDER) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderFreeInfo(pprovider: *const PXE_PROVIDER) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeProviderQueryIndex(pszprovidername: ::windows_sys::core::PCWSTR, puindex: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn PxeProviderRegister(pszprovidername: ::windows_sys::core::PCWSTR, pszmodulepath: ::windows_sys::core::PCWSTR, index: u32, biscritical: super::super::Foundation::BOOL, phproviderkey: *mut super::Registry::HKEY) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderSetAttribute(hprovider: super::super::Foundation::HANDLE, attribute: u32, pparameterbuffer: *const ::core::ffi::c_void, uparamlen: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn PxeProviderUnRegister(pszprovidername: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeRegisterCallback(hprovider: super::super::Foundation::HANDLE, callbacktype: u32, pcallbackfunction: *const ::core::ffi::c_void, pcontext: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeSendReply(hclientrequest: super::super::Foundation::HANDLE, ppacket: *const ::core::ffi::c_void, upacketlen: u32, paddress: *const PXE_ADDRESS) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeTrace(hprovider: super::super::Foundation::HANDLE, severity: u32, pszformat: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeTraceV(hprovider: super::super::Foundation::HANDLE, severity: u32, pszformat: ::windows_sys::core::PCWSTR, params: *const i8) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpAddOption(hhandle: super::super::Foundation::HANDLE, uoption: u32, uvaluelen: u32, pvalue: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpCloseHandle(hhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpGetOptionBuffer(hhandle: super::super::Foundation::HANDLE, ubufferlen: u32, pbuffer: *mut ::core::ffi::c_void, pubytes: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpInitialize(bpackettype: u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpParseInitialize(ppacket: *const ::core::ffi::c_void, upacketlen: u32, pbpackettype: *mut u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpParseInitializev6(ppacket: *const ::core::ffi::c_void, upacketlen: u32, pbpackettype: *mut u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpQueryOption(hhandle: super::super::Foundation::HANDLE, uoption: u32, uvaluelen: u32, pvalue: *mut ::core::ffi::c_void, pubytes: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliAuthorizeSession(hsession: super::super::Foundation::HANDLE, pcred: *const WDS_CLI_CRED) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliCancelTransfer(htransfer: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliClose(handle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliCreateSession(pwszserver: ::windows_sys::core::PCWSTR, pcred: *const WDS_CLI_CRED, phsession: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliFindFirstImage(hsession: super::super::Foundation::HANDLE, phfindhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliFindNextImage(handle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn WdsCliFreeStringArray(ppwszarray: *mut ::windows_sys::core::PWSTR, ulcount: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn WdsCliGetDriverQueryXml(pwszwindirpath: ::windows_sys::core::PCWSTR, ppwszdriverquery: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetEnumerationFlags(handle: super::super::Foundation::HANDLE, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageArchitecture(hifh: super::super::Foundation::HANDLE, pdwvalue: *mut CPU_ARCHITECTURE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageDescription(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageFiles(hifh: super::super::Foundation::HANDLE, pppwszfiles: *mut *mut ::windows_sys::core::PWSTR, pdwcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageGroup(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageHalName(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageHandleFromFindHandle(findhandle: super::super::Foundation::HANDLE, phimagehandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageHandleFromTransferHandle(htransfer: super::super::Foundation::HANDLE, phimagehandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageIndex(hifh: super::super::Foundation::HANDLE, pdwvalue: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageLanguage(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageLanguages(hifh: super::super::Foundation::HANDLE, pppszvalues: *mut *mut *mut i8, pdwnumvalues: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageLastModifiedTime(hifh: super::super::Foundation::HANDLE, ppsystimevalue: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageName(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageNamespace(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageParameter(hifh: super::super::Foundation::HANDLE, paramtype: WDS_CLI_IMAGE_PARAM_TYPE, presponse: *mut ::core::ffi::c_void, uresponselen: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImagePath(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageSize(hifh: super::super::Foundation::HANDLE, pullvalue: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageType(hifh: super::super::Foundation::HANDLE, pimagetype: *mut WDS_CLI_IMAGE_TYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageVersion(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetTransferSize(hifh: super::super::Foundation::HANDLE, pullvalue: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliInitializeLog(hsession: super::super::Foundation::HANDLE, ulclientarchitecture: CPU_ARCHITECTURE, pwszclientid: ::windows_sys::core::PCWSTR, pwszclientaddress: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliLog(hsession: super::super::Foundation::HANDLE, ulloglevel: u32, ulmessagecode: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliObtainDriverPackages(himage: super::super::Foundation::HANDLE, ppwszservername: *mut ::windows_sys::core::PWSTR, pppwszdriverpackages: *mut *mut ::windows_sys::core::PWSTR, pulcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliObtainDriverPackagesEx(hsession: super::super::Foundation::HANDLE, pwszmachineinfo: ::windows_sys::core::PCWSTR, ppwszservername: *mut ::windows_sys::core::PWSTR, pppwszdriverpackages: *mut *mut ::windows_sys::core::PWSTR, pulcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn WdsCliRegisterTrace(pfn: PFN_WdsCliTraceFunction) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn WdsCliSetTransferBufferSize(ulsizeinbytes: u32);
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliTransferFile(pwszserver: ::windows_sys::core::PCWSTR, pwsznamespace: ::windows_sys::core::PCWSTR, pwszremotefilepath: ::windows_sys::core::PCWSTR, pwszlocalfilepath: ::windows_sys::core::PCWSTR, dwflags: u32, dwreserved: u32, pfnwdsclicallback: PFN_WdsCliCallback, pvuserdata: *const ::core::ffi::c_void, phtransfer: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliTransferImage(himage: super::super::Foundation::HANDLE, pwszlocalpath: ::windows_sys::core::PCWSTR, dwflags: u32, dwreserved: u32, pfnwdsclicallback: PFN_WdsCliCallback, pvuserdata: *const ::core::ffi::c_void, phtransfer: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliWaitForTransfer(htransfer: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn WdsTransportClientAddRefBuffer(pvbuffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCancelSession(hsessionkey: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCancelSessionEx(hsessionkey: super::super::Foundation::HANDLE, dwerrorcode: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCloseSession(hsessionkey: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCompleteReceive(hsessionkey: super::super::Foundation::HANDLE, ulsize: u32, pulloffset: *const u64) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn WdsTransportClientInitialize() -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientInitializeSession(psessionrequest: *const WDS_TRANSPORTCLIENT_REQUEST, pcallerdata: *const ::core::ffi::c_void, hsessionkey: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientQueryStatus(hsessionkey: super::super::Foundation::HANDLE, pustatus: *mut u32, puerrorcode: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientRegisterCallback(hsessionkey: super::super::Foundation::HANDLE, callbackid: TRANSPORTCLIENT_CALLBACK_ID, pfncallback: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn WdsTransportClientReleaseBuffer(pvbuffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
    pub fn WdsTransportClientShutdown() -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientStartSession(hsessionkey: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientWaitForCompletion(hsessionkey: super::super::Foundation::HANDLE, utimeout: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerAllocateBuffer(hprovider: super::super::Foundation::HANDLE, ulbuffersize: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerCompleteRead(hprovider: super::super::Foundation::HANDLE, ulbytesread: u32, pvuserdata: *const ::core::ffi::c_void, hreadresult: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerFreeBuffer(hprovider: super::super::Foundation::HANDLE, pvbuffer: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerRegisterCallback(hprovider: super::super::Foundation::HANDLE, callbackid: TRANSPORTPROVIDER_CALLBACK_ID, pfncallback: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerTrace(hprovider: super::super::Foundation::HANDLE, severity: u32, pwszformat: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerTraceV(hprovider: super::super::Foundation::HANDLE, severity: u32, pwszformat: ::windows_sys::core::PCWSTR, params: *const i8) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type CPU_ARCHITECTURE = u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const CPU_ARCHITECTURE_AMD64: CPU_ARCHITECTURE = 9u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const CPU_ARCHITECTURE_IA64: CPU_ARCHITECTURE = 6u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const CPU_ARCHITECTURE_INTEL: CPU_ARCHITECTURE = 0u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_CP_CALLBACKS_NOT_REG: ::windows_sys::core::HRESULT = -1054801324i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_CP_CLOSE_INSTANCE_FAILED: ::windows_sys::core::HRESULT = -1054801320i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_CP_DLL_LOAD_FAILED: ::windows_sys::core::HRESULT = -1054801328i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_CP_DLL_LOAD_FAILED_CRITICAL: ::windows_sys::core::HRESULT = -1054801317i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_CP_INCOMPATIBLE_SERVER_VERSION: ::windows_sys::core::HRESULT = -1054801325i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_CP_INIT_FUNC_FAILED: ::windows_sys::core::HRESULT = -1054801326i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_CP_INIT_FUNC_MISSING: ::windows_sys::core::HRESULT = -1054801327i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_CP_MEMORY_LEAK: ::windows_sys::core::HRESULT = -1054801322i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_CP_OPEN_CONTENT_FAILED: ::windows_sys::core::HRESULT = -1054801319i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_CP_OPEN_INSTANCE_FAILED: ::windows_sys::core::HRESULT = -1054801321i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_CP_SHUTDOWN_FUNC_FAILED: ::windows_sys::core::HRESULT = -1054801323i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_DUPLICATE_MULTICAST_ADDR: ::windows_sys::core::HRESULT = -1054801406i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_NON_WDS_DUPLICATE_MULTICAST_ADDR: ::windows_sys::core::HRESULT = -1054801405i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_NSREG_CONTENT_PROVIDER_NOT_REG: ::windows_sys::core::HRESULT = -1054801151i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_NSREG_FAILURE: ::windows_sys::core::HRESULT = -1054801149i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_NSREG_NAMESPACE_EXISTS: ::windows_sys::core::HRESULT = -1054801150i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_NSREG_START_TIME_IN_PAST: ::windows_sys::core::HRESULT = -1054801152i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_E_PARAMETERS_READ_FAILED: ::windows_sys::core::HRESULT = -1054801407i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_S_PARAMETERS_READ: ::windows_sys::core::HRESULT = 1092682240i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const EVT_WDSMCS_W_CP_DLL_LOAD_FAILED_NOT_CRITICAL: ::windows_sys::core::HRESULT = -2128543142i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const FACILITY_WDSMCCLIENT: u32 = 290u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const FACILITY_WDSMCSERVER: u32 = 289u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const FACILITY_WDSTPTMGMT: u32 = 272u32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportCacheable {
    pub base__: super::Com::IDispatch,
    pub Dirty: unsafe extern "system" fn(this: *mut *mut Self, pbdirty: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Discard: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportClient {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransportsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Session: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pulid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbszname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MacAddress: unsafe extern "system" fn(this: *mut *mut Self, pbszmacaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MacAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IpAddress: unsafe extern "system" fn(this: *mut *mut Self, pbszipaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IpAddress: usize,
    pub PercentCompletion: unsafe extern "system" fn(this: *mut *mut Self, pulpercentcompletion: *mut u32) -> ::windows_sys::core::HRESULT,
    pub JoinDuration: unsafe extern "system" fn(this: *mut *mut Self, puljoinduration: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CpuUtilization: unsafe extern "system" fn(this: *mut *mut Self, pulcpuutilization: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MemoryUtilization: unsafe extern "system" fn(this: *mut *mut Self, pulmemoryutilization: *mut u32) -> ::windows_sys::core::HRESULT,
    pub NetworkUtilization: unsafe extern "system" fn(this: *mut *mut Self, pulnetworkutilization: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UserIdentity: unsafe extern "system" fn(this: *mut *mut Self, pbszuseridentity: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserIdentity: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pulcount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, ulindex: u32, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportConfigurationManager {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub ServicePolicy: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransportservicepolicy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ServicePolicy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DiagnosticsPolicy: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransportdiagnosticspolicy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DiagnosticsPolicy: usize,
    pub get_WdsTransportServicesRunning: unsafe extern "system" fn(this: *mut *mut Self, brealtimestatus: i16, pbservicesrunning: *mut i16) -> ::windows_sys::core::HRESULT,
    pub EnableWdsTransportServices: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DisableWdsTransportServices: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StartWdsTransportServices: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StopWdsTransportServices: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RestartWdsTransportServices: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyWdsTransportServices: unsafe extern "system" fn(this: *mut *mut Self, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportConfigurationManager2 {
    pub base__: IWdsTransportConfigurationManager,
    #[cfg(feature = "Win32_System_Com")]
    pub MulticastSessionPolicy: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransportmulticastsessionpolicy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MulticastSessionPolicy: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportContent {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Namespace: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransportnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Namespace: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pulid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbszname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RetrieveSessions: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransportsessions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RetrieveSessions: usize,
    pub Terminate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportContentProvider {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbszname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbszdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FilePath: unsafe extern "system" fn(this: *mut *mut Self, pbszfilepath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FilePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializationRoutine: unsafe extern "system" fn(this: *mut *mut Self, pbszinitializationroutine: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializationRoutine: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportDiagnosticsPolicy {
    pub base__: IWdsTransportCacheable,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, pbenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, benabled: i16) -> ::windows_sys::core::HRESULT,
    pub Components: unsafe extern "system" fn(this: *mut *mut Self, pulcomponents: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetComponents: unsafe extern "system" fn(this: *mut *mut Self, ulcomponents: u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportManager {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetWdsTransportServer: unsafe extern "system" fn(this: *mut *mut Self, bszservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportserver: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetWdsTransportServer: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportMulticastSessionPolicy {
    pub base__: IWdsTransportCacheable,
    pub SlowClientHandling: unsafe extern "system" fn(this: *mut *mut Self, pslowclienthandling: *mut WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows_sys::core::HRESULT,
    pub SetSlowClientHandling: unsafe extern "system" fn(this: *mut *mut Self, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows_sys::core::HRESULT,
    pub AutoDisconnectThreshold: unsafe extern "system" fn(this: *mut *mut Self, pulthreshold: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetAutoDisconnectThreshold: unsafe extern "system" fn(this: *mut *mut Self, ulthreshold: u32) -> ::windows_sys::core::HRESULT,
    pub MultistreamStreamCount: unsafe extern "system" fn(this: *mut *mut Self, pulstreamcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMultistreamStreamCount: unsafe extern "system" fn(this: *mut *mut Self, ulstreamcount: u32) -> ::windows_sys::core::HRESULT,
    pub SlowClientFallback: unsafe extern "system" fn(this: *mut *mut Self, pbclientfallback: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSlowClientFallback: unsafe extern "system" fn(this: *mut *mut Self, bclientfallback: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportNamespace {
    pub base__: super::Com::IDispatch,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pulid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbszname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pbszfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, bszfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pbszdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, bszdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ContentProvider: unsafe extern "system" fn(this: *mut *mut Self, pbszcontentprovider: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ContentProvider: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContentProvider: unsafe extern "system" fn(this: *mut *mut Self, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContentProvider: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, pbszconfiguration: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Configuration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetConfiguration: unsafe extern "system" fn(this: *mut *mut Self, bszconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetConfiguration: usize,
    pub Registered: unsafe extern "system" fn(this: *mut *mut Self, pbregistered: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Tombstoned: unsafe extern "system" fn(this: *mut *mut Self, pbtombstoned: *mut i16) -> ::windows_sys::core::HRESULT,
    pub TombstoneTime: unsafe extern "system" fn(this: *mut *mut Self, ptombstonetime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub TransmissionStarted: unsafe extern "system" fn(this: *mut *mut Self, pbtransmissionstarted: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Register: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Deregister: unsafe extern "system" fn(this: *mut *mut Self, bterminatesessions: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransportnamespaceclone: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RetrieveContents: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransportcontents: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RetrieveContents: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportNamespaceAutoCast {
    pub base__: IWdsTransportNamespace,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportNamespaceManager {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateNamespace: unsafe extern "system" fn(this: *mut *mut Self, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateNamespace: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RetrieveNamespace: unsafe extern "system" fn(this: *mut *mut Self, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RetrieveNamespace: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RetrieveNamespaces: unsafe extern "system" fn(this: *mut *mut Self, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bincludetombstones: i16, ppwdstransportnamespaces: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RetrieveNamespaces: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportNamespaceScheduledCast {
    pub base__: IWdsTransportNamespace,
    pub StartTransmission: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportNamespaceScheduledCastAutoStart {
    pub base__: IWdsTransportNamespaceScheduledCast,
    pub MinimumClients: unsafe extern "system" fn(this: *mut *mut Self, pulminimumclients: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMinimumClients: unsafe extern "system" fn(this: *mut *mut Self, ulminimumclients: u32) -> ::windows_sys::core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, pstarttime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(this: *mut *mut Self, starttime: f64) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportNamespaceScheduledCastManualStart {
    pub base__: IWdsTransportNamespaceScheduledCast,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportServer {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbszname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetupManager: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransportsetupmanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetupManager: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ConfigurationManager: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransportconfigurationmanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ConfigurationManager: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceManager: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransportnamespacemanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceManager: usize,
    pub DisconnectClient: unsafe extern "system" fn(this: *mut *mut Self, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportServer2 {
    pub base__: IWdsTransportServer,
    #[cfg(feature = "Win32_System_Com")]
    pub TftpManager: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransporttftpmanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TftpManager: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportServicePolicy {
    pub base__: IWdsTransportCacheable,
    pub get_IpAddressSource: unsafe extern "system" fn(this: *mut *mut Self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, psourcetype: *mut WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows_sys::core::HRESULT,
    pub put_IpAddressSource: unsafe extern "system" fn(this: *mut *mut Self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_StartIpAddress: unsafe extern "system" fn(this: *mut *mut Self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszstartipaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_StartIpAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_StartIpAddress: unsafe extern "system" fn(this: *mut *mut Self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_StartIpAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_EndIpAddress: unsafe extern "system" fn(this: *mut *mut Self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszendipaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_EndIpAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_EndIpAddress: unsafe extern "system" fn(this: *mut *mut Self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_EndIpAddress: usize,
    pub StartPort: unsafe extern "system" fn(this: *mut *mut Self, pulstartport: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetStartPort: unsafe extern "system" fn(this: *mut *mut Self, ulstartport: u32) -> ::windows_sys::core::HRESULT,
    pub EndPort: unsafe extern "system" fn(this: *mut *mut Self, pulendport: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetEndPort: unsafe extern "system" fn(this: *mut *mut Self, ulendport: u32) -> ::windows_sys::core::HRESULT,
    pub NetworkProfile: unsafe extern "system" fn(this: *mut *mut Self, pprofiletype: *mut WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows_sys::core::HRESULT,
    pub SetNetworkProfile: unsafe extern "system" fn(this: *mut *mut Self, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportServicePolicy2 {
    pub base__: IWdsTransportServicePolicy,
    pub UdpPortPolicy: unsafe extern "system" fn(this: *mut *mut Self, pudpportpolicy: *mut WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows_sys::core::HRESULT,
    pub SetUdpPortPolicy: unsafe extern "system" fn(this: *mut *mut Self, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows_sys::core::HRESULT,
    pub TftpMaximumBlockSize: unsafe extern "system" fn(this: *mut *mut Self, pultftpmaximumblocksize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTftpMaximumBlockSize: unsafe extern "system" fn(this: *mut *mut Self, ultftpmaximumblocksize: u32) -> ::windows_sys::core::HRESULT,
    pub EnableTftpVariableWindowExtension: unsafe extern "system" fn(this: *mut *mut Self, pbenabletftpvariablewindowextension: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnableTftpVariableWindowExtension: unsafe extern "system" fn(this: *mut *mut Self, benabletftpvariablewindowextension: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportSession {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransportcontent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Content: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pulid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NetworkInterfaceName: unsafe extern "system" fn(this: *mut *mut Self, pbsznetworkinterfacename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NetworkInterfaceName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub NetworkInterfaceAddress: unsafe extern "system" fn(this: *mut *mut Self, pbsznetworkinterfaceaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NetworkInterfaceAddress: usize,
    pub TransferRate: unsafe extern "system" fn(this: *mut *mut Self, pultransferrate: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MasterClientId: unsafe extern "system" fn(this: *mut *mut Self, pulmasterclientid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RetrieveClients: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransportclients: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RetrieveClients: usize,
    pub Terminate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportSetupManager {
    pub base__: super::Com::IDispatch,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, pullversion: *mut u64) -> ::windows_sys::core::HRESULT,
    pub InstalledFeatures: unsafe extern "system" fn(this: *mut *mut Self, pulinstalledfeatures: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Protocols: unsafe extern "system" fn(this: *mut *mut Self, pulprotocols: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterContentProvider: unsafe extern "system" fn(this: *mut *mut Self, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszinitializationroutine: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterContentProvider: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeregisterContentProvider: unsafe extern "system" fn(this: *mut *mut Self, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeregisterContentProvider: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportSetupManager2 {
    pub base__: IWdsTransportSetupManager,
    pub TftpCapabilities: unsafe extern "system" fn(this: *mut *mut Self, pultftpcapabilities: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ContentProviders: unsafe extern "system" fn(this: *mut *mut Self, ppprovidercollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ContentProviders: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportTftpClient {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub FileName: unsafe extern "system" fn(this: *mut *mut Self, pbszfilename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IpAddress: unsafe extern "system" fn(this: *mut *mut Self, pbszipaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IpAddress: usize,
    pub Timeout: unsafe extern "system" fn(this: *mut *mut Self, pultimeout: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CurrentFileOffset: unsafe extern "system" fn(this: *mut *mut Self, pul64currentoffset: *mut u64) -> ::windows_sys::core::HRESULT,
    pub FileSize: unsafe extern "system" fn(this: *mut *mut Self, pul64filesize: *mut u64) -> ::windows_sys::core::HRESULT,
    pub BlockSize: unsafe extern "system" fn(this: *mut *mut Self, pulblocksize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub WindowSize: unsafe extern "system" fn(this: *mut *mut Self, pulwindowsize: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWdsTransportTftpManager {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub RetrieveTftpClients: unsafe extern "system" fn(this: *mut *mut Self, ppwdstransporttftpclients: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RetrieveTftpClients: usize,
}
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const MC_SERVER_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type PFN_WDS_CLI_CALLBACK_MESSAGE_ID = u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_MSG_START: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = 0u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_MSG_COMPLETE: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_MSG_PROGRESS: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = 2u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_MSG_TEXT: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = 3u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsCliCallback = ::core::option::Option<unsafe extern "system" fn(dwmessageid: PFN_WDS_CLI_CALLBACK_MESSAGE_ID, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pvuserdata: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type PFN_WdsCliTraceFunction = ::core::option::Option<unsafe extern "system" fn(pwszformat: ::windows_sys::core::PCWSTR, params: *const i8)>;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientReceiveContents = ::core::option::Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::core::ffi::c_void, pcontents: *const ::core::ffi::c_void, ulsize: u32, pullcontentoffset: *const u64)>;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientReceiveMetadata = ::core::option::Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::core::ffi::c_void, pmetadata: *const ::core::ffi::c_void, ulsize: u32)>;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientSessionComplete = ::core::option::Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::core::ffi::c_void, dwerror: u32)>;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientSessionNegotiate = ::core::option::Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::core::ffi::c_void, pinfo: *const TRANSPORTCLIENT_SESSION_INFO, hnegotiatekey: super::super::Foundation::HANDLE)>;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientSessionStart = ::core::option::Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::core::ffi::c_void, ullfilesize: *const u64)>;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientSessionStartEx = ::core::option::Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::core::ffi::c_void, info: *const TRANSPORTCLIENT_SESSION_INFO)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub struct PXE_ADDRESS {
    pub uFlags: u32,
    pub Anonymous: PXE_ADDRESS_0,
    pub uAddrLen: u32,
    pub uPort: u16,
}
impl ::core::marker::Copy for PXE_ADDRESS {}
impl ::core::clone::Clone for PXE_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub union PXE_ADDRESS_0 {
    pub bAddress: [u8; 16],
    pub uIpAddress: u32,
}
impl ::core::marker::Copy for PXE_ADDRESS_0 {}
impl ::core::clone::Clone for PXE_ADDRESS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_ADDR_BROADCAST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_ADDR_USE_ADDR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_ADDR_USE_DHCP_RULES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_ADDR_USE_PORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_BA_CUSTOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_BA_IGNORE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_BA_NBP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_BA_REJECTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_CALLBACK_MAX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_CALLBACK_RECV_REQUEST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_CALLBACK_SERVICE_CONTROL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_CALLBACK_SHUTDOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_DHCPV6_CLIENT_PORT: u32 = 546u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub struct PXE_DHCPV6_MESSAGE {
    pub MessageType: u8,
    pub TransactionIDByte1: u8,
    pub TransactionIDByte2: u8,
    pub TransactionIDByte3: u8,
    pub Options: [PXE_DHCPV6_OPTION; 1],
}
impl ::core::marker::Copy for PXE_DHCPV6_MESSAGE {}
impl ::core::clone::Clone for PXE_DHCPV6_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub struct PXE_DHCPV6_MESSAGE_HEADER {
    pub MessageType: u8,
    pub Message: [u8; 1],
}
impl ::core::marker::Copy for PXE_DHCPV6_MESSAGE_HEADER {}
impl ::core::clone::Clone for PXE_DHCPV6_MESSAGE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub struct PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    pub pRelayMessage: *mut PXE_DHCPV6_RELAY_MESSAGE,
    pub cbRelayMessage: u32,
    pub pInterfaceIdOption: *mut ::core::ffi::c_void,
    pub cbInterfaceIdOption: u16,
}
impl ::core::marker::Copy for PXE_DHCPV6_NESTED_RELAY_MESSAGE {}
impl ::core::clone::Clone for PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub struct PXE_DHCPV6_OPTION {
    pub OptionCode: u16,
    pub DataLength: u16,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for PXE_DHCPV6_OPTION {}
impl ::core::clone::Clone for PXE_DHCPV6_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_DHCPV6_RELAY_HOP_COUNT_LIMIT: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub struct PXE_DHCPV6_RELAY_MESSAGE {
    pub MessageType: u8,
    pub HopCount: u8,
    pub LinkAddress: [u8; 16],
    pub PeerAddress: [u8; 16],
    pub Options: [PXE_DHCPV6_OPTION; 1],
}
impl ::core::marker::Copy for PXE_DHCPV6_RELAY_MESSAGE {}
impl ::core::clone::Clone for PXE_DHCPV6_RELAY_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_DHCPV6_SERVER_PORT: u32 = 547u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_DHCP_CLIENT_PORT: u32 = 68u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_DHCP_FILE_SIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_DHCP_HWAADR_SIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_DHCP_MAGIC_COOKIE_SIZE: u32 = 4u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub struct PXE_DHCP_MESSAGE {
    pub Operation: u8,
    pub HardwareAddressType: u8,
    pub HardwareAddressLength: u8,
    pub HopCount: u8,
    pub TransactionID: u32,
    pub SecondsSinceBoot: u16,
    pub Reserved: u16,
    pub ClientIpAddress: u32,
    pub YourIpAddress: u32,
    pub BootstrapServerAddress: u32,
    pub RelayAgentIpAddress: u32,
    pub HardwareAddress: [u8; 16],
    pub HostName: [u8; 64],
    pub BootFileName: [u8; 128],
    pub Anonymous: PXE_DHCP_MESSAGE_0,
    pub Option: PXE_DHCP_OPTION,
}
impl ::core::marker::Copy for PXE_DHCP_MESSAGE {}
impl ::core::clone::Clone for PXE_DHCP_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub union PXE_DHCP_MESSAGE_0 {
    pub bMagicCookie: [u8; 4],
    pub uMagicCookie: u32,
}
impl ::core::marker::Copy for PXE_DHCP_MESSAGE_0 {}
impl ::core::clone::Clone for PXE_DHCP_MESSAGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub struct PXE_DHCP_OPTION {
    pub OptionType: u8,
    pub OptionLength: u8,
    pub OptionValue: [u8; 1],
}
impl ::core::marker::Copy for PXE_DHCP_OPTION {}
impl ::core::clone::Clone for PXE_DHCP_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_DHCP_SERVER_PORT: u32 = 67u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_DHCP_SERVER_SIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_GSI_SERVER_DUID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_GSI_TRACE_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_MAX_ADDRESS: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PXE_PROVIDER {
    pub uSizeOfStruct: u32,
    pub pwszName: ::windows_sys::core::PWSTR,
    pub pwszFilePath: ::windows_sys::core::PWSTR,
    pub bIsCritical: super::super::Foundation::BOOL,
    pub uIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PXE_PROVIDER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PXE_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_PROV_ATTR_FILTER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_PROV_ATTR_FILTER_IPV6: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_PROV_ATTR_IPV6_CAPABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_PROV_FILTER_ALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_PROV_FILTER_DHCP_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_PROV_FILTER_PXE_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_REG_INDEX_BOTTOM: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_REG_INDEX_TOP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_SERVER_PORT: u32 = 4011u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_TRACE_ERROR: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_TRACE_FATAL: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_TRACE_INFO: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_TRACE_VERBOSE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const PXE_TRACE_WARNING: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type TRANSPORTCLIENT_CALLBACK_ID = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_SESSION_START: TRANSPORTCLIENT_CALLBACK_ID = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_RECEIVE_CONTENTS: TRANSPORTCLIENT_CALLBACK_ID = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_SESSION_COMPLETE: TRANSPORTCLIENT_CALLBACK_ID = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_RECEIVE_METADATA: TRANSPORTCLIENT_CALLBACK_ID = 3i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_SESSION_STARTEX: TRANSPORTCLIENT_CALLBACK_ID = 4i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_SESSION_NEGOTIATE: TRANSPORTCLIENT_CALLBACK_ID = 5i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_MAX_CALLBACKS: TRANSPORTCLIENT_CALLBACK_ID = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub struct TRANSPORTCLIENT_SESSION_INFO {
    pub ulStructureLength: u32,
    pub ullFileSize: u64,
    pub ulBlockSize: u32,
}
impl ::core::marker::Copy for TRANSPORTCLIENT_SESSION_INFO {}
impl ::core::clone::Clone for TRANSPORTCLIENT_SESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type TRANSPORTPROVIDER_CALLBACK_ID = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTPROVIDER_CREATE_INSTANCE: TRANSPORTPROVIDER_CALLBACK_ID = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTPROVIDER_COMPARE_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTPROVIDER_OPEN_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTPROVIDER_USER_ACCESS_CHECK: TRANSPORTPROVIDER_CALLBACK_ID = 3i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTPROVIDER_GET_CONTENT_SIZE: TRANSPORTPROVIDER_CALLBACK_ID = 4i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTPROVIDER_READ_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = 5i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTPROVIDER_CLOSE_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = 6i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTPROVIDER_CLOSE_INSTANCE: TRANSPORTPROVIDER_CALLBACK_ID = 7i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTPROVIDER_SHUTDOWN: TRANSPORTPROVIDER_CALLBACK_ID = 8i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTPROVIDER_DUMP_STATE: TRANSPORTPROVIDER_CALLBACK_ID = 9i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTPROVIDER_REFRESH_SETTINGS: TRANSPORTPROVIDER_CALLBACK_ID = 10i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTPROVIDER_GET_CONTENT_METADATA: TRANSPORTPROVIDER_CALLBACK_ID = 11i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTPROVIDER_MAX_CALLBACKS: TRANSPORTPROVIDER_CALLBACK_ID = 12i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const TRANSPORTPROVIDER_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPTVAL_ACTION_ABORT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPTVAL_ACTION_APPROVAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPTVAL_ACTION_REFERRAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPTVAL_NBP_VER_7: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPTVAL_NBP_VER_8: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPTVAL_PXE_PROMPT_NOPROMPT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPTVAL_PXE_PROMPT_OPTIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPTVAL_PXE_PROMPT_OPTOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPT_TYPE_BYTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPT_TYPE_IP4: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPT_TYPE_IP6: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPT_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPT_TYPE_STR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPT_TYPE_ULONG: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPT_TYPE_USHORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_OPT_TYPE_WSTR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_PK_TYPE_BCD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_PK_TYPE_DHCP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_PK_TYPE_DHCPV6: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSBP_PK_TYPE_WDSNBP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCCLIENT_CATEGORY: ::windows_sys::core::HRESULT = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCSERVER_CATEGORY: ::windows_sys::core::HRESULT = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_CLIENT_DOESNOT_SUPPORT_SECURITY_MODE: ::windows_sys::core::HRESULT = -1054801648i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_CLIENT_NOT_FOUND: ::windows_sys::core::HRESULT = -1054801660i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_CONTENT_NOT_FOUND: ::windows_sys::core::HRESULT = -1054801661i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_CONTENT_PROVIDER_NOT_FOUND: ::windows_sys::core::HRESULT = -1054801658i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_INCOMPATIBLE_VERSION: ::windows_sys::core::HRESULT = -1054801662i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_NAMESPACE_ALREADY_EXISTS: ::windows_sys::core::HRESULT = -1054801657i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_NAMESPACE_ALREADY_STARTED: ::windows_sys::core::HRESULT = -1054801655i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_NAMESPACE_NOT_FOUND: ::windows_sys::core::HRESULT = -1054801659i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_NAMESPACE_SHUTDOWN_IN_PROGRESS: ::windows_sys::core::HRESULT = -1054801656i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_NS_START_FAILED_NO_CLIENTS: ::windows_sys::core::HRESULT = -1054801654i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_PACKET_HAS_SECURITY: ::windows_sys::core::HRESULT = -1054801650i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_PACKET_NOT_CHECKSUMED: ::windows_sys::core::HRESULT = -1054801649i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_PACKET_NOT_HASHED: ::windows_sys::core::HRESULT = -1054801652i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_PACKET_NOT_SIGNED: ::windows_sys::core::HRESULT = -1054801651i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_REQCALLBACKS_NOT_REG: ::windows_sys::core::HRESULT = -1054801663i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_SESSION_SHUTDOWN_IN_PROGRESS: ::windows_sys::core::HRESULT = -1054801664i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSMCS_E_START_TIME_IN_PAST: ::windows_sys::core::HRESULT = -1054801653i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPC_E_ALREADY_COMPLETED: ::windows_sys::core::HRESULT = -1054735615i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPC_E_ALREADY_IN_LOWEST_SESSION: ::windows_sys::core::HRESULT = -1054735606i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPC_E_ALREADY_IN_PROGRESS: ::windows_sys::core::HRESULT = -1054735614i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPC_E_CALLBACKS_NOT_REG: ::windows_sys::core::HRESULT = -1054735616i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1054735605i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPC_E_KICKED_FAIL: ::windows_sys::core::HRESULT = -1054735609i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPC_E_KICKED_FALLBACK: ::windows_sys::core::HRESULT = -1054735610i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPC_E_KICKED_POLICY_NOT_MET: ::windows_sys::core::HRESULT = -1054735611i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPC_E_KICKED_UNKNOWN: ::windows_sys::core::HRESULT = -1054735608i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPC_E_MULTISTREAM_NOT_ENABLED: ::windows_sys::core::HRESULT = -1054735607i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPC_E_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -1054735612i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPC_E_NO_IP4_INTERFACE: ::windows_sys::core::HRESULT = -1054735604i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPC_E_UNKNOWN_ERROR: ::windows_sys::core::HRESULT = -1054735613i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE: ::windows_sys::core::HRESULT = -1054735603i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_CATEGORY: ::windows_sys::core::HRESULT = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_CANNOT_REFRESH_DIRTY_OBJECT: ::windows_sys::core::HRESULT = -1055915761i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_CANNOT_REINITIALIZE_OBJECT: ::windows_sys::core::HRESULT = -1055915767i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_CONTENT_PROVIDER_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = -1055915773i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_CONTENT_PROVIDER_NOT_REGISTERED: ::windows_sys::core::HRESULT = -1055915772i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_AUTO_DISCONNECT_THRESHOLD: ::windows_sys::core::HRESULT = -1055915748i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_CLASS: ::windows_sys::core::HRESULT = -1055915774i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_CONTENT_PROVIDER_NAME: ::windows_sys::core::HRESULT = -1055915771i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_DIAGNOSTICS_COMPONENTS: ::windows_sys::core::HRESULT = -1055915762i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_IPV4_MULTICAST_ADDRESS: ::windows_sys::core::HRESULT = -1055915753i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS: ::windows_sys::core::HRESULT = -1055915752i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS_SOURCE: ::windows_sys::core::HRESULT = -1055915750i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_IP_ADDRESS: ::windows_sys::core::HRESULT = -1055915754i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_MULTISTREAM_STREAM_COUNT: ::windows_sys::core::HRESULT = -1055915749i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_DATA: ::windows_sys::core::HRESULT = -1055915765i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_NAME: ::windows_sys::core::HRESULT = -1055915766i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_START_PARAMETERS: ::windows_sys::core::HRESULT = -1055915758i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_START_TIME: ::windows_sys::core::HRESULT = -1055915763i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_OPERATION: ::windows_sys::core::HRESULT = -1055915775i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_PROPERTY: ::windows_sys::core::HRESULT = -1055915776i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_SERVICE_IP_ADDRESS_RANGE: ::windows_sys::core::HRESULT = -1055915760i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_SERVICE_PORT_RANGE: ::windows_sys::core::HRESULT = -1055915759i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_SLOW_CLIENT_HANDLING_TYPE: ::windows_sys::core::HRESULT = -1055915746i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_INVALID_TFTP_MAX_BLOCKSIZE: ::windows_sys::core::HRESULT = -1055915741i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_IPV6_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1055915751i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_MULTICAST_SESSION_POLICY_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1055915747i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_NAMESPACE_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = -1055915769i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_NAMESPACE_NOT_ON_SERVER: ::windows_sys::core::HRESULT = -1055915756i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_NAMESPACE_NOT_REGISTERED: ::windows_sys::core::HRESULT = -1055915768i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_NAMESPACE_READ_ONLY: ::windows_sys::core::HRESULT = -1055915764i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_NAMESPACE_REMOVED_FROM_SERVER: ::windows_sys::core::HRESULT = -1055915755i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_NETWORK_PROFILES_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1055915745i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_TFTP_MAX_BLOCKSIZE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1055915743i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_TFTP_VAR_WINDOW_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1055915742i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_TRANSPORT_SERVER_ROLE_NOT_CONFIGURED: ::windows_sys::core::HRESULT = -1055915770i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_TRANSPORT_SERVER_UNAVAILABLE: ::windows_sys::core::HRESULT = -1055915757i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTPTMGMT_E_UDP_PORT_POLICY_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1055915744i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptDiagnosticsComponentPxe: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptDiagnosticsComponentTftp: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptDiagnosticsComponentImageServer: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptDiagnosticsComponentMulticast: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDSTRANSPORT_DISCONNECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptDisconnectUnknown: WDSTRANSPORT_DISCONNECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptDisconnectFallback: WDSTRANSPORT_DISCONNECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptDisconnectAbort: WDSTRANSPORT_DISCONNECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDSTRANSPORT_FEATURE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptFeatureAdminPack: WDSTRANSPORT_FEATURE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptFeatureTransportServer: WDSTRANSPORT_FEATURE_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptFeatureDeploymentServer: WDSTRANSPORT_FEATURE_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptIpAddressSourceUnknown: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptIpAddressSourceDhcp: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptIpAddressSourceRange: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDSTRANSPORT_IP_ADDRESS_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptIpAddressUnknown: WDSTRANSPORT_IP_ADDRESS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptIpAddressIpv4: WDSTRANSPORT_IP_ADDRESS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptIpAddressIpv6: WDSTRANSPORT_IP_ADDRESS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDSTRANSPORT_NAMESPACE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptNamespaceTypeUnknown: WDSTRANSPORT_NAMESPACE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptNamespaceTypeAutoCast: WDSTRANSPORT_NAMESPACE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptNamespaceTypeScheduledCastManualStart: WDSTRANSPORT_NAMESPACE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptNamespaceTypeScheduledCastAutoStart: WDSTRANSPORT_NAMESPACE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDSTRANSPORT_NETWORK_PROFILE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptNetworkProfileUnknown: WDSTRANSPORT_NETWORK_PROFILE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptNetworkProfileCustom: WDSTRANSPORT_NETWORK_PROFILE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptNetworkProfile10Mbps: WDSTRANSPORT_NETWORK_PROFILE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptNetworkProfile100Mbps: WDSTRANSPORT_NETWORK_PROFILE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptNetworkProfile1Gbps: WDSTRANSPORT_NETWORK_PROFILE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDSTRANSPORT_PROTOCOL_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptProtocolUnicast: WDSTRANSPORT_PROTOCOL_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptProtocolMulticast: WDSTRANSPORT_PROTOCOL_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDSTRANSPORT_RESOURCE_UTILIZATION_UNKNOWN: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDSTRANSPORT_SERVICE_NOTIFICATION = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptServiceNotifyUnknown: WDSTRANSPORT_SERVICE_NOTIFICATION = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptServiceNotifyReadSettings: WDSTRANSPORT_SERVICE_NOTIFICATION = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptSlowClientHandlingUnknown: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptSlowClientHandlingNone: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptSlowClientHandlingAutoDisconnect: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptSlowClientHandlingMultistream: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDSTRANSPORT_TFTP_CAPABILITY = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptTftpCapMaximumBlockSize: WDSTRANSPORT_TFTP_CAPABILITY = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptTftpCapVariableWindow: WDSTRANSPORT_TFTP_CAPABILITY = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDSTRANSPORT_UDP_PORT_POLICY = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptUdpPortPolicyDynamic: WDSTRANSPORT_UDP_PORT_POLICY = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsTptUdpPortPolicyFixed: WDSTRANSPORT_UDP_PORT_POLICY = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub struct WDS_CLI_CRED {
    pub pwszUserName: ::windows_sys::core::PCWSTR,
    pub pwszDomain: ::windows_sys::core::PCWSTR,
    pub pwszPassword: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WDS_CLI_CRED {}
impl ::core::clone::Clone for WDS_CLI_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDS_CLI_FIRMWARE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_FIRMWARE_UNKNOWN: WDS_CLI_FIRMWARE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_FIRMWARE_BIOS: WDS_CLI_FIRMWARE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_FIRMWARE_EFI: WDS_CLI_FIRMWARE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDS_CLI_IMAGE_PARAM_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_IMAGE_PARAM_UNKNOWN: WDS_CLI_IMAGE_PARAM_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_IMAGE_PARAM_SPARSE_FILE: WDS_CLI_IMAGE_PARAM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_IMAGE_PARAM_SUPPORTED_FIRMWARES: WDS_CLI_IMAGE_PARAM_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDS_CLI_IMAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_IMAGE_TYPE_UNKNOWN: WDS_CLI_IMAGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_IMAGE_TYPE_WIM: WDS_CLI_IMAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_IMAGE_TYPE_VHD: WDS_CLI_IMAGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_IMAGE_TYPE_VHDX: WDS_CLI_IMAGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_NO_SPARSE_FILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_CLI_TRANSFER_ASYNCHRONOUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_LEVEL_DISABLED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_LEVEL_ERROR: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_LEVEL_INFO: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_LEVEL_WARNING: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_APPLY_FINISHED: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_APPLY_FINISHED_2: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_APPLY_STARTED: i32 = 5i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_APPLY_STARTED_2: i32 = 15i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_DOMAINJOINERROR: i32 = 12i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_DOMAINJOINERROR_2: i32 = 17i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_DRIVER_PACKAGE_NOT_ACCESSIBLE: i32 = 18i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_ERROR: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_FINISHED: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_GENERIC_MESSAGE: i32 = 7i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED2: i32 = 22i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED3: i32 = 23i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_MAX_CODE: i32 = 24i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_END: i32 = 20i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_FAILURE: i32 = 21i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_START: i32 = 19i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_POST_ACTIONS_END: i32 = 14i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_POST_ACTIONS_START: i32 = 13i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_STARTED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_DOWNGRADE: i32 = 11i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_END: i32 = 10i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_START: i32 = 9i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_LOG_TYPE_CLIENT_UNATTEND_MODE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_MC_TRACE_ERROR: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_MC_TRACE_FATAL: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_MC_TRACE_INFO: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_MC_TRACE_VERBOSE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_MC_TRACE_WARNING: u32 = 262144u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WDS_TRANSPORTCLIENT_CALLBACKS {
    pub SessionStart: PFN_WdsTransportClientSessionStart,
    pub SessionStartEx: PFN_WdsTransportClientSessionStartEx,
    pub ReceiveContents: PFN_WdsTransportClientReceiveContents,
    pub ReceiveMetadata: PFN_WdsTransportClientReceiveMetadata,
    pub SessionComplete: PFN_WdsTransportClientSessionComplete,
    pub SessionNegotiate: PFN_WdsTransportClientSessionNegotiate,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WDS_TRANSPORTCLIENT_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WDS_TRANSPORTCLIENT_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_CURRENT_API_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_NO_CACHE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_PROTOCOL_MULTICAST: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub struct WDS_TRANSPORTCLIENT_REQUEST {
    pub ulLength: u32,
    pub ulApiVersion: u32,
    pub ulAuthLevel: WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL,
    pub pwszServer: ::windows_sys::core::PCWSTR,
    pub pwszNamespace: ::windows_sys::core::PCWSTR,
    pub pwszObjectName: ::windows_sys::core::PCWSTR,
    pub ulCacheSize: u32,
    pub ulProtocol: u32,
    pub pvProtocolData: *mut ::core::ffi::c_void,
    pub ulProtocolDataLength: u32,
}
impl ::core::marker::Copy for WDS_TRANSPORTCLIENT_REQUEST {}
impl ::core::clone::Clone for WDS_TRANSPORTCLIENT_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub type WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL = u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_AUTH: WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_NO_AUTH: WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL = 2u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_STATUS_FAILURE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_STATUS_IN_PROGRESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WDS_TRANSPORTCLIENT_STATUS_SUCCESS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    pub ulLength: u32,
    pub ulMcServerVersion: u32,
    pub hRegistryKey: super::Registry::HKEY,
    pub hProvider: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for WDS_TRANSPORTPROVIDER_INIT_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub struct WDS_TRANSPORTPROVIDER_SETTINGS {
    pub ulLength: u32,
    pub ulProviderVersion: u32,
}
impl ::core::marker::Copy for WDS_TRANSPORTPROVIDER_SETTINGS {}
impl ::core::clone::Clone for WDS_TRANSPORTPROVIDER_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsCliFlagEnumFilterFirmware: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_DeploymentServices\"`*"]
pub const WdsCliFlagEnumFilterVersion: i32 = 1i32;
pub const WdsTransportCacheable: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1884883734, data2: 61766, data3: 18109, data4: [189, 157, 74, 170, 144, 8, 75, 245] };
pub const WdsTransportClient: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1725089257, data2: 4086, data3: 18924, data4: [151, 51, 218, 251, 30, 1, 223, 28] };
pub const WdsTransportCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3354495753, data2: 14622, data3: 17262, data4: [177, 11, 195, 239, 70, 242, 195, 79] };
pub const WdsTransportConfigurationManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2269378164, data2: 36940, data3: 18378, data4: [133, 18, 53, 254, 152, 246, 176, 172] };
pub const WdsTransportContent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 176758759, data2: 19007, data3: 19557, data4: [182, 242, 20, 103, 97, 150, 121, 234] };
pub const WdsTransportContentProvider: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3770577951, data2: 23157, data3: 20153, data4: [138, 45, 94, 24, 155, 69, 243, 39] };
pub const WdsTransportDiagnosticsPolicy: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3946001377, data2: 42925, data3: 18165, data4: [128, 214, 107, 116, 2, 4, 229, 9] };
pub const WdsTransportManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4061471734, data2: 33660, data3: 19032, data4: [175, 153, 138, 126, 39, 248, 255, 89] };
pub const WdsTransportMulticastSessionPolicy: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1013695476, data2: 25624, data3: 18218, data4: [182, 241, 82, 212, 87, 25, 84, 55] };
pub const WdsTransportNamespace: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3627571048, data2: 1842, data3: 20161, data4: [149, 234, 22, 218, 88, 25, 8, 161] };
pub const WdsTransportNamespaceAutoCast: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2962355624, data2: 27289, data3: 18317, data4: [178, 59, 9, 232, 254, 224, 69, 116] };
pub const WdsTransportNamespaceManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4035763043, data2: 34270, data3: 18984, data4: [161, 169, 92, 163, 231, 239, 218, 115] };
pub const WdsTransportNamespaceScheduledCast: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3134986391, data2: 28709, data3: 17643, data4: [145, 8, 251, 97, 196, 5, 87, 146] };
pub const WdsTransportNamespaceScheduledCastAutoStart: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2702209106, data2: 4652, data3: 19329, data4: [155, 124, 56, 110, 104, 85, 56, 63] };
pub const WdsTransportNamespaceScheduledCastManualStart: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3554779818, data2: 51884, data3: 17934, data4: [185, 138, 71, 249, 243, 24, 161, 250] };
pub const WdsTransportServer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3927553603, data2: 19167, data3: 17427, data4: [148, 44, 20, 243, 121, 17, 135, 96] };
pub const WdsTransportServicePolicy: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1705831132, data2: 12043, data3: 20291, data4: [159, 77, 129, 24, 101, 216, 206, 173] };
pub const WdsTransportSession: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1956300000, data2: 26556, data3: 18243, data4: [191, 229, 202, 203, 31, 38, 245, 127] };
pub const WdsTransportSetupManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3351177901, data2: 40708, data3: 18723, data4: [159, 12, 251, 245, 43, 199, 89, 15] };
pub const WdsTransportTftpClient: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1345599781, data2: 31836, data3: 19596, data4: [150, 196, 173, 159, 165, 0, 95, 186] };
pub const WdsTransportTftpManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3370769570, data2: 12865, data3: 20045, data4: [184, 6, 188, 116, 1, 157, 254, 218] };
