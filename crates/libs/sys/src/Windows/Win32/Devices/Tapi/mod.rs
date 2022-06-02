#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn GetTnefStreamCodepage(lpstream: *mut *mut super::super::System::Com::IStream, lpulcodepage: *mut u32, lpulsubcodepage: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub fn OpenTnefStream(lpvsupport: *mut ::core::ffi::c_void, lpstream: *mut *mut super::super::System::Com::IStream, lpszstreamname: *const i8, ulflags: u32, lpmessage: *mut *mut super::super::System::AddressBook::IMessage, wkeyval: u16, lpptnef: *mut *mut *mut ITnef) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub fn OpenTnefStreamEx(lpvsupport: *mut ::core::ffi::c_void, lpstream: *mut *mut super::super::System::Com::IStream, lpszstreamname: *const i8, ulflags: u32, lpmessage: *mut *mut super::super::System::AddressBook::IMessage, wkeyval: u16, lpadressbook: *mut *mut super::super::System::AddressBook::IAddrBook, lpptnef: *mut *mut *mut ITnef) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineAccept(hcall: u32, lpsuseruserinfo: ::windows_sys::core::PCSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineAddProvider(lpszproviderfilename: ::windows_sys::core::PCSTR, hwndowner: super::super::Foundation::HWND, lpdwpermanentproviderid: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineAddProviderA(lpszproviderfilename: ::windows_sys::core::PCSTR, hwndowner: super::super::Foundation::HWND, lpdwpermanentproviderid: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineAddProviderW(lpszproviderfilename: ::windows_sys::core::PCWSTR, hwndowner: super::super::Foundation::HWND, lpdwpermanentproviderid: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineAddToConference(hconfcall: u32, hconsultcall: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineAgentSpecific(hline: u32, dwaddressid: u32, dwagentextensionidindex: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineAnswer(hcall: u32, lpsuseruserinfo: ::windows_sys::core::PCSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineBlindTransfer(hcall: u32, lpszdestaddress: ::windows_sys::core::PCSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineBlindTransferA(hcall: u32, lpszdestaddress: ::windows_sys::core::PCSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineBlindTransferW(hcall: u32, lpszdestaddressw: ::windows_sys::core::PCWSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineClose(hline: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineCompleteCall(hcall: u32, lpdwcompletionid: *mut u32, dwcompletionmode: u32, dwmessageid: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineCompleteTransfer(hcall: u32, hconsultcall: u32, lphconfcall: *mut u32, dwtransfermode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialog(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogA(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogEdit(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: ::windows_sys::core::PCSTR, lpdeviceconfigin: *const ::core::ffi::c_void, dwsize: u32, lpdeviceconfigout: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogEditA(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: ::windows_sys::core::PCSTR, lpdeviceconfigin: *const ::core::ffi::c_void, dwsize: u32, lpdeviceconfigout: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogEditW(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: ::windows_sys::core::PCWSTR, lpdeviceconfigin: *const ::core::ffi::c_void, dwsize: u32, lpdeviceconfigout: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogW(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigProvider(hwndowner: super::super::Foundation::HWND, dwpermanentproviderid: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineCreateAgentA(hline: u32, lpszagentid: ::windows_sys::core::PCSTR, lpszagentpin: ::windows_sys::core::PCSTR, lphagent: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineCreateAgentSessionA(hline: u32, hagent: u32, lpszagentpin: ::windows_sys::core::PCSTR, dwworkingaddressid: u32, lpgroupid: *mut ::windows_sys::core::GUID, lphagentsession: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineCreateAgentSessionW(hline: u32, hagent: u32, lpszagentpin: ::windows_sys::core::PCWSTR, dwworkingaddressid: u32, lpgroupid: *mut ::windows_sys::core::GUID, lphagentsession: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineCreateAgentW(hline: u32, lpszagentid: ::windows_sys::core::PCWSTR, lpszagentpin: ::windows_sys::core::PCWSTR, lphagent: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineDeallocateCall(hcall: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineDevSpecific(hline: u32, dwaddressid: u32, hcall: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineDevSpecificFeature(hline: u32, dwfeature: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineDial(hcall: u32, lpszdestaddress: ::windows_sys::core::PCSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineDialA(hcall: u32, lpszdestaddress: ::windows_sys::core::PCSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineDialW(hcall: u32, lpszdestaddress: ::windows_sys::core::PCWSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineDrop(hcall: u32, lpsuseruserinfo: ::windows_sys::core::PCSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineForward(hline: u32, balladdresses: u32, dwaddressid: u32, lpforwardlist: *const LINEFORWARDLIST, dwnumringsnoanswer: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineForwardA(hline: u32, balladdresses: u32, dwaddressid: u32, lpforwardlist: *const LINEFORWARDLIST, dwnumringsnoanswer: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineForwardW(hline: u32, balladdresses: u32, dwaddressid: u32, lpforwardlist: *const LINEFORWARDLIST, dwnumringsnoanswer: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGatherDigits(hcall: u32, dwdigitmodes: u32, lpsdigits: ::windows_sys::core::PSTR, dwnumdigits: u32, lpszterminationdigits: ::windows_sys::core::PCSTR, dwfirstdigittimeout: u32, dwinterdigittimeout: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGatherDigitsA(hcall: u32, dwdigitmodes: u32, lpsdigits: ::windows_sys::core::PSTR, dwnumdigits: u32, lpszterminationdigits: ::windows_sys::core::PCSTR, dwfirstdigittimeout: u32, dwinterdigittimeout: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGatherDigitsW(hcall: u32, dwdigitmodes: u32, lpsdigits: ::windows_sys::core::PWSTR, dwnumdigits: u32, lpszterminationdigits: ::windows_sys::core::PCWSTR, dwfirstdigittimeout: u32, dwinterdigittimeout: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGenerateDigits(hcall: u32, dwdigitmode: u32, lpszdigits: ::windows_sys::core::PCSTR, dwduration: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGenerateDigitsA(hcall: u32, dwdigitmode: u32, lpszdigits: ::windows_sys::core::PCSTR, dwduration: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGenerateDigitsW(hcall: u32, dwdigitmode: u32, lpszdigits: ::windows_sys::core::PCWSTR, dwduration: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGenerateTone(hcall: u32, dwtonemode: u32, dwduration: u32, dwnumtones: u32, lptones: *const LINEGENERATETONE) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAddressCaps(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwapiversion: u32, dwextversion: u32, lpaddresscaps: *mut LINEADDRESSCAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAddressCapsA(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwapiversion: u32, dwextversion: u32, lpaddresscaps: *mut LINEADDRESSCAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAddressCapsW(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwapiversion: u32, dwextversion: u32, lpaddresscaps: *mut LINEADDRESSCAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAddressID(hline: u32, lpdwaddressid: *mut u32, dwaddressmode: u32, lpsaddress: ::windows_sys::core::PCSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAddressIDA(hline: u32, lpdwaddressid: *mut u32, dwaddressmode: u32, lpsaddress: ::windows_sys::core::PCSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAddressIDW(hline: u32, lpdwaddressid: *mut u32, dwaddressmode: u32, lpsaddress: ::windows_sys::core::PCWSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAddressStatus(hline: u32, dwaddressid: u32, lpaddressstatus: *mut LINEADDRESSSTATUS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAddressStatusA(hline: u32, dwaddressid: u32, lpaddressstatus: *mut LINEADDRESSSTATUS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAddressStatusW(hline: u32, dwaddressid: u32, lpaddressstatus: *mut LINEADDRESSSTATUS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAgentActivityListA(hline: u32, dwaddressid: u32, lpagentactivitylist: *mut LINEAGENTACTIVITYLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAgentActivityListW(hline: u32, dwaddressid: u32, lpagentactivitylist: *mut LINEAGENTACTIVITYLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAgentCapsA(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwappapiversion: u32, lpagentcaps: *mut LINEAGENTCAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAgentCapsW(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwappapiversion: u32, lpagentcaps: *mut LINEAGENTCAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAgentGroupListA(hline: u32, dwaddressid: u32, lpagentgrouplist: *mut LINEAGENTGROUPLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAgentGroupListW(hline: u32, dwaddressid: u32, lpagentgrouplist: *mut LINEAGENTGROUPLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn lineGetAgentInfo(hline: u32, hagent: u32, lpagentinfo: *mut LINEAGENTINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn lineGetAgentSessionInfo(hline: u32, hagentsession: u32, lpagentsessioninfo: *mut LINEAGENTSESSIONINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAgentSessionList(hline: u32, hagent: u32, lpagentsessionlist: *mut LINEAGENTSESSIONLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAgentStatusA(hline: u32, dwaddressid: u32, lpagentstatus: *mut LINEAGENTSTATUS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAgentStatusW(hline: u32, dwaddressid: u32, lpagentstatus: *mut LINEAGENTSTATUS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAppPriority(lpszappfilename: ::windows_sys::core::PCSTR, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpextensionname: *mut VARSTRING, lpdwpriority: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAppPriorityA(lpszappfilename: ::windows_sys::core::PCSTR, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpextensionname: *mut VARSTRING, lpdwpriority: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetAppPriorityW(lpszappfilename: ::windows_sys::core::PCWSTR, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpextensionname: *mut VARSTRING, lpdwpriority: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetCallInfo(hcall: u32, lpcallinfo: *mut LINECALLINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetCallInfoA(hcall: u32, lpcallinfo: *mut LINECALLINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetCallInfoW(hcall: u32, lpcallinfo: *mut LINECALLINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetCallStatus(hcall: u32, lpcallstatus: *mut LINECALLSTATUS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetConfRelatedCalls(hcall: u32, lpcalllist: *mut LINECALLLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetCountry(dwcountryid: u32, dwapiversion: u32, lplinecountrylist: *mut LINECOUNTRYLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetCountryA(dwcountryid: u32, dwapiversion: u32, lplinecountrylist: *mut LINECOUNTRYLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetCountryW(dwcountryid: u32, dwapiversion: u32, lplinecountrylist: *mut LINECOUNTRYLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetDevCaps(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lplinedevcaps: *mut LINEDEVCAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetDevCapsA(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lplinedevcaps: *mut LINEDEVCAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetDevCapsW(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lplinedevcaps: *mut LINEDEVCAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetDevConfig(dwdeviceid: u32, lpdeviceconfig: *mut VARSTRING, lpszdeviceclass: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetDevConfigA(dwdeviceid: u32, lpdeviceconfig: *mut VARSTRING, lpszdeviceclass: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetDevConfigW(dwdeviceid: u32, lpdeviceconfig: *mut VARSTRING, lpszdeviceclass: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetGroupListA(hline: u32, lpgrouplist: *mut LINEAGENTGROUPLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetGroupListW(hline: u32, lpgrouplist: *mut LINEAGENTGROUPLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetID(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetIDA(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetIDW(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetIcon(dwdeviceid: u32, lpszdeviceclass: ::windows_sys::core::PCSTR, lphicon: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetIconA(dwdeviceid: u32, lpszdeviceclass: ::windows_sys::core::PCSTR, lphicon: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetIconW(dwdeviceid: u32, lpszdeviceclass: ::windows_sys::core::PCWSTR, lphicon: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetLineDevStatus(hline: u32, lplinedevstatus: *mut LINEDEVSTATUS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetLineDevStatusA(hline: u32, lplinedevstatus: *mut LINEDEVSTATUS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetLineDevStatusW(hline: u32, lplinedevstatus: *mut LINEDEVSTATUS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetMessage(hlineapp: u32, lpmessage: *mut LINEMESSAGE, dwtimeout: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetNewCalls(hline: u32, dwaddressid: u32, dwselect: u32, lpcalllist: *mut LINECALLLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetNumRings(hline: u32, dwaddressid: u32, lpdwnumrings: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetProviderList(dwapiversion: u32, lpproviderlist: *mut LINEPROVIDERLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetProviderListA(dwapiversion: u32, lpproviderlist: *mut LINEPROVIDERLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetProviderListW(dwapiversion: u32, lpproviderlist: *mut LINEPROVIDERLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetProxyStatus(hlineapp: u32, dwdeviceid: u32, dwappapiversion: u32, lplineproxyreqestlist: *mut LINEPROXYREQUESTLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetQueueInfo(hline: u32, dwqueueid: u32, lplinequeueinfo: *mut LINEQUEUEINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetQueueListA(hline: u32, lpgroupid: *mut ::windows_sys::core::GUID, lpqueuelist: *mut LINEQUEUELIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetQueueListW(hline: u32, lpgroupid: *mut ::windows_sys::core::GUID, lpqueuelist: *mut LINEQUEUELIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetRequest(hlineapp: u32, dwrequestmode: u32, lprequestbuffer: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetRequestA(hlineapp: u32, dwrequestmode: u32, lprequestbuffer: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetRequestW(hlineapp: u32, dwrequestmode: u32, lprequestbuffer: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetStatusMessages(hline: u32, lpdwlinestates: *mut u32, lpdwaddressstates: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetTranslateCaps(hlineapp: u32, dwapiversion: u32, lptranslatecaps: *mut LINETRANSLATECAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetTranslateCapsA(hlineapp: u32, dwapiversion: u32, lptranslatecaps: *mut LINETRANSLATECAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineGetTranslateCapsW(hlineapp: u32, dwapiversion: u32, lptranslatecaps: *mut LINETRANSLATECAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineHandoff(hcall: u32, lpszfilename: ::windows_sys::core::PCSTR, dwmediamode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineHandoffA(hcall: u32, lpszfilename: ::windows_sys::core::PCSTR, dwmediamode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineHandoffW(hcall: u32, lpszfilename: ::windows_sys::core::PCWSTR, dwmediamode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineHold(hcall: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineInitialize(lphlineapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: LINECALLBACK, lpszappname: ::windows_sys::core::PCSTR, lpdwnumdevs: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineInitializeExA(lphlineapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: LINECALLBACK, lpszfriendlyappname: ::windows_sys::core::PCSTR, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lplineinitializeexparams: *mut LINEINITIALIZEEXPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineInitializeExW(lphlineapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: LINECALLBACK, lpszfriendlyappname: ::windows_sys::core::PCWSTR, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lplineinitializeexparams: *mut LINEINITIALIZEEXPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineMakeCall(hline: u32, lphcall: *mut u32, lpszdestaddress: ::windows_sys::core::PCSTR, dwcountrycode: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineMakeCallA(hline: u32, lphcall: *mut u32, lpszdestaddress: ::windows_sys::core::PCSTR, dwcountrycode: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineMakeCallW(hline: u32, lphcall: *mut u32, lpszdestaddress: ::windows_sys::core::PCWSTR, dwcountrycode: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineMonitorDigits(hcall: u32, dwdigitmodes: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineMonitorMedia(hcall: u32, dwmediamodes: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineMonitorTones(hcall: u32, lptonelist: *const LINEMONITORTONE, dwnumentries: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineNegotiateAPIVersion(hlineapp: u32, dwdeviceid: u32, dwapilowversion: u32, dwapihighversion: u32, lpdwapiversion: *mut u32, lpextensionid: *mut LINEEXTENSIONID) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineNegotiateExtVersion(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextlowversion: u32, dwexthighversion: u32, lpdwextversion: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineOpen(hlineapp: u32, dwdeviceid: u32, lphline: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivileges: u32, dwmediamodes: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineOpenA(hlineapp: u32, dwdeviceid: u32, lphline: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivileges: u32, dwmediamodes: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineOpenW(hlineapp: u32, dwdeviceid: u32, lphline: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivileges: u32, dwmediamodes: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn linePark(hcall: u32, dwparkmode: u32, lpszdiraddress: ::windows_sys::core::PCSTR, lpnondiraddress: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineParkA(hcall: u32, dwparkmode: u32, lpszdiraddress: ::windows_sys::core::PCSTR, lpnondiraddress: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineParkW(hcall: u32, dwparkmode: u32, lpszdiraddress: ::windows_sys::core::PCWSTR, lpnondiraddress: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn linePickup(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: ::windows_sys::core::PCSTR, lpszgroupid: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn linePickupA(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: ::windows_sys::core::PCSTR, lpszgroupid: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn linePickupW(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: ::windows_sys::core::PCWSTR, lpszgroupid: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn linePrepareAddToConference(hconfcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn linePrepareAddToConferenceA(hconfcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn linePrepareAddToConferenceW(hconfcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineProxyMessage(hline: u32, hcall: u32, dwmsg: u32, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn lineProxyResponse(hline: u32, lpproxyrequest: *mut LINEPROXYREQUEST, dwresult: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineRedirect(hcall: u32, lpszdestaddress: ::windows_sys::core::PCSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineRedirectA(hcall: u32, lpszdestaddress: ::windows_sys::core::PCSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineRedirectW(hcall: u32, lpszdestaddress: ::windows_sys::core::PCWSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineRegisterRequestRecipient(hlineapp: u32, dwregistrationinstance: u32, dwrequestmode: u32, benable: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineReleaseUserUserInfo(hcall: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineRemoveFromConference(hcall: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineRemoveProvider(dwpermanentproviderid: u32, hwndowner: super::super::Foundation::HWND) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSecureCall(hcall: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSendUserUserInfo(hcall: u32, lpsuseruserinfo: ::windows_sys::core::PCSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetAgentActivity(hline: u32, dwaddressid: u32, dwactivityid: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetAgentGroup(hline: u32, dwaddressid: u32, lpagentgrouplist: *mut LINEAGENTGROUPLIST) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetAgentMeasurementPeriod(hline: u32, hagent: u32, dwmeasurementperiod: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetAgentSessionState(hline: u32, hagentsession: u32, dwagentsessionstate: u32, dwnextagentsessionstate: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetAgentState(hline: u32, dwaddressid: u32, dwagentstate: u32, dwnextagentstate: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetAgentStateEx(hline: u32, hagent: u32, dwagentstate: u32, dwnextagentstate: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetAppPriority(lpszappfilename: ::windows_sys::core::PCSTR, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpszextensionname: ::windows_sys::core::PCSTR, dwpriority: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetAppPriorityA(lpszappfilename: ::windows_sys::core::PCSTR, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpszextensionname: ::windows_sys::core::PCSTR, dwpriority: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetAppPriorityW(lpszappfilename: ::windows_sys::core::PCWSTR, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpszextensionname: ::windows_sys::core::PCWSTR, dwpriority: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetAppSpecific(hcall: u32, dwappspecific: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetCallData(hcall: u32, lpcalldata: *mut ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetCallParams(hcall: u32, dwbearermode: u32, dwminrate: u32, dwmaxrate: u32, lpdialparams: *const LINEDIALPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetCallPrivilege(hcall: u32, dwcallprivilege: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetCallQualityOfService(hcall: u32, lpsendingflowspec: *mut ::core::ffi::c_void, dwsendingflowspecsize: u32, lpreceivingflowspec: *mut ::core::ffi::c_void, dwreceivingflowspecsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetCallTreatment(hcall: u32, dwtreatment: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetCurrentLocation(hlineapp: u32, dwlocation: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetDevConfig(dwdeviceid: u32, lpdeviceconfig: *const ::core::ffi::c_void, dwsize: u32, lpszdeviceclass: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetDevConfigA(dwdeviceid: u32, lpdeviceconfig: *const ::core::ffi::c_void, dwsize: u32, lpszdeviceclass: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetDevConfigW(dwdeviceid: u32, lpdeviceconfig: *const ::core::ffi::c_void, dwsize: u32, lpszdeviceclass: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetLineDevStatus(hline: u32, dwstatustochange: u32, fstatus: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetMediaControl(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdigitlist: *const LINEMEDIACONTROLDIGIT, dwdigitnumentries: u32, lpmedialist: *const LINEMEDIACONTROLMEDIA, dwmedianumentries: u32, lptonelist: *const LINEMEDIACONTROLTONE, dwtonenumentries: u32, lpcallstatelist: *const LINEMEDIACONTROLCALLSTATE, dwcallstatenumentries: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetMediaMode(hcall: u32, dwmediamodes: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetNumRings(hline: u32, dwaddressid: u32, dwnumrings: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetQueueMeasurementPeriod(hline: u32, dwqueueid: u32, dwmeasurementperiod: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetStatusMessages(hline: u32, dwlinestates: u32, dwaddressstates: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetTerminal(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, dwterminalmodes: u32, dwterminalid: u32, benable: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetTollList(hlineapp: u32, dwdeviceid: u32, lpszaddressin: ::windows_sys::core::PCSTR, dwtolllistoption: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetTollListA(hlineapp: u32, dwdeviceid: u32, lpszaddressin: ::windows_sys::core::PCSTR, dwtolllistoption: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetTollListW(hlineapp: u32, dwdeviceid: u32, lpszaddressinw: ::windows_sys::core::PCWSTR, dwtolllistoption: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetupConference(hcall: u32, hline: u32, lphconfcall: *mut u32, lphconsultcall: *mut u32, dwnumparties: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetupConferenceA(hcall: u32, hline: u32, lphconfcall: *mut u32, lphconsultcall: *mut u32, dwnumparties: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetupConferenceW(hcall: u32, hline: u32, lphconfcall: *mut u32, lphconsultcall: *mut u32, dwnumparties: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetupTransfer(hcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetupTransferA(hcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSetupTransferW(hcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineShutdown(hlineapp: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineSwapHold(hactivecall: u32, hheldcall: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineTranslateAddress(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, lpszaddressin: ::windows_sys::core::PCSTR, dwcard: u32, dwtranslateoptions: u32, lptranslateoutput: *mut LINETRANSLATEOUTPUT) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineTranslateAddressA(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, lpszaddressin: ::windows_sys::core::PCSTR, dwcard: u32, dwtranslateoptions: u32, lptranslateoutput: *mut LINETRANSLATEOUTPUT) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineTranslateAddressW(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, lpszaddressin: ::windows_sys::core::PCWSTR, dwcard: u32, dwtranslateoptions: u32, lptranslateoutput: *mut LINETRANSLATEOUTPUT) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateDialog(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, hwndowner: super::super::Foundation::HWND, lpszaddressin: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateDialogA(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, hwndowner: super::super::Foundation::HWND, lpszaddressin: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateDialogW(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, hwndowner: super::super::Foundation::HWND, lpszaddressin: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineUncompleteCall(hline: u32, dwcompletionid: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineUnhold(hcall: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineUnpark(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineUnparkA(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn lineUnparkW(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneClose(hphone: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneConfigDialog(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneConfigDialogA(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneConfigDialogW(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneDevSpecific(hphone: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetButtonInfo(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *mut PHONEBUTTONINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetButtonInfoA(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *mut PHONEBUTTONINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetButtonInfoW(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *mut PHONEBUTTONINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetData(hphone: u32, dwdataid: u32, lpdata: *mut ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetDevCaps(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lpphonecaps: *mut PHONECAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetDevCapsA(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lpphonecaps: *mut PHONECAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetDevCapsW(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lpphonecaps: *mut PHONECAPS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetDisplay(hphone: u32, lpdisplay: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetGain(hphone: u32, dwhookswitchdev: u32, lpdwgain: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetHookSwitch(hphone: u32, lpdwhookswitchdevs: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetID(hphone: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetIDA(hphone: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetIDW(hphone: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetIcon(dwdeviceid: u32, lpszdeviceclass: ::windows_sys::core::PCSTR, lphicon: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetIconA(dwdeviceid: u32, lpszdeviceclass: ::windows_sys::core::PCSTR, lphicon: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetIconW(dwdeviceid: u32, lpszdeviceclass: ::windows_sys::core::PCWSTR, lphicon: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetLamp(hphone: u32, dwbuttonlampid: u32, lpdwlampmode: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetMessage(hphoneapp: u32, lpmessage: *mut PHONEMESSAGE, dwtimeout: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetRing(hphone: u32, lpdwringmode: *mut u32, lpdwvolume: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetStatus(hphone: u32, lpphonestatus: *mut PHONESTATUS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetStatusA(hphone: u32, lpphonestatus: *mut PHONESTATUS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetStatusMessages(hphone: u32, lpdwphonestates: *mut u32, lpdwbuttonmodes: *mut u32, lpdwbuttonstates: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetStatusW(hphone: u32, lpphonestatus: *mut PHONESTATUS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneGetVolume(hphone: u32, dwhookswitchdev: u32, lpdwvolume: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneInitialize(lphphoneapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: PHONECALLBACK, lpszappname: ::windows_sys::core::PCSTR, lpdwnumdevs: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneInitializeExA(lphphoneapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: PHONECALLBACK, lpszfriendlyappname: ::windows_sys::core::PCSTR, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lpphoneinitializeexparams: *mut PHONEINITIALIZEEXPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneInitializeExW(lphphoneapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: PHONECALLBACK, lpszfriendlyappname: ::windows_sys::core::PCWSTR, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lpphoneinitializeexparams: *mut PHONEINITIALIZEEXPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneNegotiateAPIVersion(hphoneapp: u32, dwdeviceid: u32, dwapilowversion: u32, dwapihighversion: u32, lpdwapiversion: *mut u32, lpextensionid: *mut PHONEEXTENSIONID) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneNegotiateExtVersion(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextlowversion: u32, dwexthighversion: u32, lpdwextversion: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneOpen(hphoneapp: u32, dwdeviceid: u32, lphphone: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivilege: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneSetButtonInfo(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *const PHONEBUTTONINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneSetButtonInfoA(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *const PHONEBUTTONINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneSetButtonInfoW(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *const PHONEBUTTONINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneSetData(hphone: u32, dwdataid: u32, lpdata: *const ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneSetDisplay(hphone: u32, dwrow: u32, dwcolumn: u32, lpsdisplay: ::windows_sys::core::PCSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneSetGain(hphone: u32, dwhookswitchdev: u32, dwgain: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneSetHookSwitch(hphone: u32, dwhookswitchdevs: u32, dwhookswitchmode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneSetLamp(hphone: u32, dwbuttonlampid: u32, dwlampmode: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneSetRing(hphone: u32, dwringmode: u32, dwvolume: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneSetStatusMessages(hphone: u32, dwphonestates: u32, dwbuttonmodes: u32, dwbuttonstates: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneSetVolume(hphone: u32, dwhookswitchdev: u32, dwvolume: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn phoneShutdown(hphoneapp: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn tapiGetLocationInfo(lpszcountrycode: ::windows_sys::core::PSTR, lpszcitycode: ::windows_sys::core::PSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn tapiGetLocationInfoA(lpszcountrycode: ::windows_sys::core::PSTR, lpszcitycode: ::windows_sys::core::PSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn tapiGetLocationInfoW(lpszcountrycodew: ::windows_sys::core::PWSTR, lpszcitycodew: ::windows_sys::core::PWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestDrop(hwnd: super::super::Foundation::HWND, wrequestid: super::super::Foundation::WPARAM) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn tapiRequestMakeCall(lpszdestaddress: ::windows_sys::core::PCSTR, lpszappname: ::windows_sys::core::PCSTR, lpszcalledparty: ::windows_sys::core::PCSTR, lpszcomment: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn tapiRequestMakeCallA(lpszdestaddress: ::windows_sys::core::PCSTR, lpszappname: ::windows_sys::core::PCSTR, lpszcalledparty: ::windows_sys::core::PCSTR, lpszcomment: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
    pub fn tapiRequestMakeCallW(lpszdestaddress: ::windows_sys::core::PCWSTR, lpszappname: ::windows_sys::core::PCWSTR, lpszcalledparty: ::windows_sys::core::PCWSTR, lpszcomment: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMediaCall(hwnd: super::super::Foundation::HWND, wrequestid: super::super::Foundation::WPARAM, lpszdeviceclass: ::windows_sys::core::PCSTR, lpdeviceid: ::windows_sys::core::PCSTR, dwsize: u32, dwsecure: u32, lpszdestaddress: ::windows_sys::core::PCSTR, lpszappname: ::windows_sys::core::PCSTR, lpszcalledparty: ::windows_sys::core::PCSTR, lpszcomment: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMediaCallA(hwnd: super::super::Foundation::HWND, wrequestid: super::super::Foundation::WPARAM, lpszdeviceclass: ::windows_sys::core::PCSTR, lpdeviceid: ::windows_sys::core::PCSTR, dwsize: u32, dwsecure: u32, lpszdestaddress: ::windows_sys::core::PCSTR, lpszappname: ::windows_sys::core::PCSTR, lpszcalledparty: ::windows_sys::core::PCSTR, lpszcomment: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMediaCallW(hwnd: super::super::Foundation::HWND, wrequestid: super::super::Foundation::WPARAM, lpszdeviceclass: ::windows_sys::core::PCWSTR, lpdeviceid: ::windows_sys::core::PCWSTR, dwsize: u32, dwsecure: u32, lpszdestaddress: ::windows_sys::core::PCWSTR, lpszappname: ::windows_sys::core::PCWSTR, lpszcalledparty: ::windows_sys::core::PCWSTR, lpszcomment: ::windows_sys::core::PCWSTR) -> i32;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type ACDGROUP_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACDGE_NEW_GROUP: ACDGROUP_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACDGE_GROUP_REMOVED: ACDGROUP_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type ACDQUEUE_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACDQE_NEW_QUEUE: ACDQUEUE_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACDQE_QUEUE_REMOVED: ACDQUEUE_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type ADDRESS_CAPABILITY = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_ADDRESSTYPES: ADDRESS_CAPABILITY = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_BEARERMODES: ADDRESS_CAPABILITY = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXACTIVECALLS: ADDRESS_CAPABILITY = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXONHOLDCALLS: ADDRESS_CAPABILITY = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXONHOLDPENDINGCALLS: ADDRESS_CAPABILITY = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXNUMCONFERENCE: ADDRESS_CAPABILITY = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXNUMTRANSCONF: ADDRESS_CAPABILITY = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MONITORDIGITSUPPORT: ADDRESS_CAPABILITY = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GENERATEDIGITSUPPORT: ADDRESS_CAPABILITY = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GENERATETONEMODES: ADDRESS_CAPABILITY = 9i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GENERATETONEMAXNUMFREQ: ADDRESS_CAPABILITY = 10i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MONITORTONEMAXNUMFREQ: ADDRESS_CAPABILITY = 11i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MONITORTONEMAXNUMENTRIES: ADDRESS_CAPABILITY = 12i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_DEVCAPFLAGS: ADDRESS_CAPABILITY = 13i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_ANSWERMODES: ADDRESS_CAPABILITY = 14i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_LINEFEATURES: ADDRESS_CAPABILITY = 15i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_SETTABLEDEVSTATUS: ADDRESS_CAPABILITY = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_PARKSUPPORT: ADDRESS_CAPABILITY = 17i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CALLERIDSUPPORT: ADDRESS_CAPABILITY = 18i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CALLEDIDSUPPORT: ADDRESS_CAPABILITY = 19i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CONNECTEDIDSUPPORT: ADDRESS_CAPABILITY = 20i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_REDIRECTIONIDSUPPORT: ADDRESS_CAPABILITY = 21i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_REDIRECTINGIDSUPPORT: ADDRESS_CAPABILITY = 22i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_ADDRESSCAPFLAGS: ADDRESS_CAPABILITY = 23i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CALLFEATURES1: ADDRESS_CAPABILITY = 24i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CALLFEATURES2: ADDRESS_CAPABILITY = 25i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_REMOVEFROMCONFCAPS: ADDRESS_CAPABILITY = 26i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_REMOVEFROMCONFSTATE: ADDRESS_CAPABILITY = 27i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_TRANSFERMODES: ADDRESS_CAPABILITY = 28i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_ADDRESSFEATURES: ADDRESS_CAPABILITY = 29i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_PREDICTIVEAUTOTRANSFERSTATES: ADDRESS_CAPABILITY = 30i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXCALLDATASIZE: ADDRESS_CAPABILITY = 31i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_LINEID: ADDRESS_CAPABILITY = 32i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_ADDRESSID: ADDRESS_CAPABILITY = 33i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_FORWARDMODES: ADDRESS_CAPABILITY = 34i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXFORWARDENTRIES: ADDRESS_CAPABILITY = 35i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXSPECIFICENTRIES: ADDRESS_CAPABILITY = 36i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MINFWDNUMRINGS: ADDRESS_CAPABILITY = 37i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXFWDNUMRINGS: ADDRESS_CAPABILITY = 38i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXCALLCOMPLETIONS: ADDRESS_CAPABILITY = 39i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CALLCOMPLETIONCONDITIONS: ADDRESS_CAPABILITY = 40i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CALLCOMPLETIONMODES: ADDRESS_CAPABILITY = 41i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_PERMANENTDEVICEID: ADDRESS_CAPABILITY = 42i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GATHERDIGITSMINTIMEOUT: ADDRESS_CAPABILITY = 43i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GATHERDIGITSMAXTIMEOUT: ADDRESS_CAPABILITY = 44i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GENERATEDIGITMINDURATION: ADDRESS_CAPABILITY = 45i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GENERATEDIGITMAXDURATION: ADDRESS_CAPABILITY = 46i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GENERATEDIGITDEFAULTDURATION: ADDRESS_CAPABILITY = 47i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type ADDRESS_CAPABILITY_STRING = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACS_PROTOCOL: ADDRESS_CAPABILITY_STRING = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACS_ADDRESSDEVICESPECIFIC: ADDRESS_CAPABILITY_STRING = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACS_LINEDEVICESPECIFIC: ADDRESS_CAPABILITY_STRING = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACS_PROVIDERSPECIFIC: ADDRESS_CAPABILITY_STRING = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACS_SWITCHSPECIFIC: ADDRESS_CAPABILITY_STRING = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACS_PERMANENTDEVICEGUID: ADDRESS_CAPABILITY_STRING = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type ADDRESS_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_STATE: ADDRESS_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_CAPSCHANGE: ADDRESS_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_RINGING: ADDRESS_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_CONFIGCHANGE: ADDRESS_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_FORWARD: ADDRESS_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_NEWTERMINAL: ADDRESS_EVENT = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_REMOVETERMINAL: ADDRESS_EVENT = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_MSGWAITON: ADDRESS_EVENT = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_MSGWAITOFF: ADDRESS_EVENT = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_LASTITEM: ADDRESS_EVENT = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type ADDRESS_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_INSERVICE: ADDRESS_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_OUTOFSERVICE: ADDRESS_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type AGENTHANDLER_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AHE_NEW_AGENTHANDLER: AGENTHANDLER_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AHE_AGENTHANDLER_REMOVED: AGENTHANDLER_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type AGENT_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_NOT_READY: AGENT_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_READY: AGENT_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_BUSY_ACD: AGENT_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_BUSY_INCOMING: AGENT_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_BUSY_OUTGOING: AGENT_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_UNKNOWN: AGENT_EVENT = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type AGENT_SESSION_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASE_NEW_SESSION: AGENT_SESSION_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASE_NOT_READY: AGENT_SESSION_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASE_READY: AGENT_SESSION_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASE_BUSY: AGENT_SESSION_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASE_WRAPUP: AGENT_SESSION_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASE_END: AGENT_SESSION_EVENT = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type AGENT_SESSION_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASST_NOT_READY: AGENT_SESSION_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASST_READY: AGENT_SESSION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASST_BUSY_ON_CALL: AGENT_SESSION_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASST_BUSY_WRAPUP: AGENT_SESSION_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASST_SESSION_ENDED: AGENT_SESSION_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type AGENT_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_NOT_READY: AGENT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_READY: AGENT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_BUSY_ACD: AGENT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_BUSY_INCOMING: AGENT_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_BUSY_OUTGOING: AGENT_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_UNKNOWN: AGENT_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type ASYNC_COMPLETION = ::core::option::Option<unsafe extern "system" fn(dwrequestid: u32, lresult: i32)>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type CALLHUB_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHE_CALLJOIN: CALLHUB_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHE_CALLLEAVE: CALLHUB_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHE_CALLHUBNEW: CALLHUB_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHE_CALLHUBIDLE: CALLHUB_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHE_LASTITEM: CALLHUB_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type CALLHUB_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHS_ACTIVE: CALLHUB_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHS_IDLE: CALLHUB_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type CALLINFOCHANGE_CAUSE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_OTHER: CALLINFOCHANGE_CAUSE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_DEVSPECIFIC: CALLINFOCHANGE_CAUSE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_BEARERMODE: CALLINFOCHANGE_CAUSE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_RATE: CALLINFOCHANGE_CAUSE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_APPSPECIFIC: CALLINFOCHANGE_CAUSE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_CALLID: CALLINFOCHANGE_CAUSE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_RELATEDCALLID: CALLINFOCHANGE_CAUSE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_ORIGIN: CALLINFOCHANGE_CAUSE = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_REASON: CALLINFOCHANGE_CAUSE = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_COMPLETIONID: CALLINFOCHANGE_CAUSE = 9i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_NUMOWNERINCR: CALLINFOCHANGE_CAUSE = 10i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_NUMOWNERDECR: CALLINFOCHANGE_CAUSE = 11i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_NUMMONITORS: CALLINFOCHANGE_CAUSE = 12i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_TRUNK: CALLINFOCHANGE_CAUSE = 13i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_CALLERID: CALLINFOCHANGE_CAUSE = 14i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_CALLEDID: CALLINFOCHANGE_CAUSE = 15i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_CONNECTEDID: CALLINFOCHANGE_CAUSE = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_REDIRECTIONID: CALLINFOCHANGE_CAUSE = 17i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_REDIRECTINGID: CALLINFOCHANGE_CAUSE = 18i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_USERUSERINFO: CALLINFOCHANGE_CAUSE = 19i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_HIGHLEVELCOMP: CALLINFOCHANGE_CAUSE = 20i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_LOWLEVELCOMP: CALLINFOCHANGE_CAUSE = 21i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_CHARGINGINFO: CALLINFOCHANGE_CAUSE = 22i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_TREATMENT: CALLINFOCHANGE_CAUSE = 23i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_CALLDATA: CALLINFOCHANGE_CAUSE = 24i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_PRIVILEGE: CALLINFOCHANGE_CAUSE = 25i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_MEDIATYPE: CALLINFOCHANGE_CAUSE = 26i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_LASTITEM: CALLINFOCHANGE_CAUSE = 26i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type CALLINFO_BUFFER = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIB_USERUSERINFO: CALLINFO_BUFFER = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIB_DEVSPECIFICBUFFER: CALLINFO_BUFFER = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIB_CALLDATABUFFER: CALLINFO_BUFFER = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIB_CHARGINGINFOBUFFER: CALLINFO_BUFFER = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIB_HIGHLEVELCOMPATIBILITYBUFFER: CALLINFO_BUFFER = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIB_LOWLEVELCOMPATIBILITYBUFFER: CALLINFO_BUFFER = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type CALLINFO_LONG = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_MEDIATYPESAVAILABLE: CALLINFO_LONG = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_BEARERMODE: CALLINFO_LONG = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_CALLERIDADDRESSTYPE: CALLINFO_LONG = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_CALLEDIDADDRESSTYPE: CALLINFO_LONG = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_CONNECTEDIDADDRESSTYPE: CALLINFO_LONG = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_REDIRECTIONIDADDRESSTYPE: CALLINFO_LONG = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_REDIRECTINGIDADDRESSTYPE: CALLINFO_LONG = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_ORIGIN: CALLINFO_LONG = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_REASON: CALLINFO_LONG = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_APPSPECIFIC: CALLINFO_LONG = 9i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_CALLPARAMSFLAGS: CALLINFO_LONG = 10i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_CALLTREATMENT: CALLINFO_LONG = 11i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_MINRATE: CALLINFO_LONG = 12i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_MAXRATE: CALLINFO_LONG = 13i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_COUNTRYCODE: CALLINFO_LONG = 14i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_CALLID: CALLINFO_LONG = 15i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_RELATEDCALLID: CALLINFO_LONG = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_COMPLETIONID: CALLINFO_LONG = 17i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_NUMBEROFOWNERS: CALLINFO_LONG = 18i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_NUMBEROFMONITORS: CALLINFO_LONG = 19i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_TRUNK: CALLINFO_LONG = 20i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_RATE: CALLINFO_LONG = 21i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_GENERATEDIGITDURATION: CALLINFO_LONG = 22i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_MONITORDIGITMODES: CALLINFO_LONG = 23i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_MONITORMEDIAMODES: CALLINFO_LONG = 24i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type CALLINFO_STRING = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CALLERIDNAME: CALLINFO_STRING = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CALLERIDNUMBER: CALLINFO_STRING = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CALLEDIDNAME: CALLINFO_STRING = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CALLEDIDNUMBER: CALLINFO_STRING = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CONNECTEDIDNAME: CALLINFO_STRING = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CONNECTEDIDNUMBER: CALLINFO_STRING = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_REDIRECTIONIDNAME: CALLINFO_STRING = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_REDIRECTIONIDNUMBER: CALLINFO_STRING = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_REDIRECTINGIDNAME: CALLINFO_STRING = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_REDIRECTINGIDNUMBER: CALLINFO_STRING = 9i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CALLEDPARTYFRIENDLYNAME: CALLINFO_STRING = 10i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_COMMENT: CALLINFO_STRING = 11i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_DISPLAYABLEADDRESS: CALLINFO_STRING = 12i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CALLINGPARTYID: CALLINFO_STRING = 13i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type CALL_MEDIA_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_NEW_STREAM: CALL_MEDIA_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_STREAM_FAIL: CALL_MEDIA_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_TERMINAL_FAIL: CALL_MEDIA_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_STREAM_NOT_USED: CALL_MEDIA_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_STREAM_ACTIVE: CALL_MEDIA_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_STREAM_INACTIVE: CALL_MEDIA_EVENT = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_LASTITEM: CALL_MEDIA_EVENT = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type CALL_MEDIA_EVENT_CAUSE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_UNKNOWN: CALL_MEDIA_EVENT_CAUSE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_BAD_DEVICE: CALL_MEDIA_EVENT_CAUSE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_CONNECT_FAIL: CALL_MEDIA_EVENT_CAUSE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_LOCAL_REQUEST: CALL_MEDIA_EVENT_CAUSE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_REMOTE_REQUEST: CALL_MEDIA_EVENT_CAUSE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_MEDIA_TIMEOUT: CALL_MEDIA_EVENT_CAUSE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_MEDIA_RECOVERED: CALL_MEDIA_EVENT_CAUSE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_QUALITY_OF_SERVICE: CALL_MEDIA_EVENT_CAUSE = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type CALL_NOTIFICATION_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CNE_OWNER: CALL_NOTIFICATION_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CNE_MONITOR: CALL_NOTIFICATION_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CNE_LASTITEM: CALL_NOTIFICATION_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type CALL_PRIVILEGE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CP_OWNER: CALL_PRIVILEGE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CP_MONITOR: CALL_PRIVILEGE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type CALL_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_IDLE: CALL_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_INPROGRESS: CALL_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_CONNECTED: CALL_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_DISCONNECTED: CALL_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_OFFERING: CALL_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_HOLD: CALL_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_QUEUED: CALL_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_LASTITEM: CALL_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type CALL_STATE_EVENT_CAUSE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_NONE: CALL_STATE_EVENT_CAUSE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_NORMAL: CALL_STATE_EVENT_CAUSE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_BUSY: CALL_STATE_EVENT_CAUSE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_BADADDRESS: CALL_STATE_EVENT_CAUSE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_NOANSWER: CALL_STATE_EVENT_CAUSE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_CANCELLED: CALL_STATE_EVENT_CAUSE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_REJECTED: CALL_STATE_EVENT_CAUSE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_FAILED: CALL_STATE_EVENT_CAUSE = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_BLOCKED: CALL_STATE_EVENT_CAUSE = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type DIRECTORY_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const OT_CONFERENCE: DIRECTORY_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const OT_USER: DIRECTORY_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type DIRECTORY_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const DT_NTDS: DIRECTORY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const DT_ILS: DIRECTORY_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type DISCONNECT_CODE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const DC_NORMAL: DISCONNECT_CODE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const DC_NOANSWER: DISCONNECT_CODE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const DC_REJECTED: DISCONNECT_CODE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const DISPIDMASK: u32 = 65535u32;
pub const DispatchMapper: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3911340694, data2: 51033, data3: 4561, data4: [160, 43, 0, 192, 79, 182, 128, 159] };
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type FINISH_MODE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FM_ASTRANSFER: FINISH_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FM_ASCONFERENCE: FINISH_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type FT_STATE_EVENT_CAUSE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FTEC_NORMAL: FT_STATE_EVENT_CAUSE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FTEC_END_OF_FILE: FT_STATE_EVENT_CAUSE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FTEC_READ_ERROR: FT_STATE_EVENT_CAUSE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FTEC_WRITE_ERROR: FT_STATE_EVENT_CAUSE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type FULLDUPLEX_SUPPORT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FDS_SUPPORTED: FULLDUPLEX_SUPPORT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FDS_NOTSUPPORTED: FULLDUPLEX_SUPPORT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FDS_UNKNOWN: FULLDUPLEX_SUPPORT = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const GETTNEFSTREAMCODEPAGE: &str = "GetTnefStreamCodePage";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HDRVCALL__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDRVCALL__ {}
impl ::core::clone::Clone for HDRVCALL__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HDRVDIALOGINSTANCE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDRVDIALOGINSTANCE__ {}
impl ::core::clone::Clone for HDRVDIALOGINSTANCE__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HDRVLINE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDRVLINE__ {}
impl ::core::clone::Clone for HDRVLINE__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HDRVMSPLINE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDRVMSPLINE__ {}
impl ::core::clone::Clone for HDRVMSPLINE__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HDRVPHONE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDRVPHONE__ {}
impl ::core::clone::Clone for HDRVPHONE__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HPROVIDER__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HPROVIDER__ {}
impl ::core::clone::Clone for HPROVIDER__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HTAPICALL__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HTAPICALL__ {}
impl ::core::clone::Clone for HTAPICALL__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HTAPILINE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HTAPILINE__ {}
impl ::core::clone::Clone for HTAPILINE__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HTAPIPHONE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HTAPIPHONE__ {}
impl ::core::clone::Clone for HTAPIPHONE__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPADDRESS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPADDRESSCAPABILITIES: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPADDRESSTRANSLATION: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPAGGREGATEDMSPADDRESSOBJ: u32 = 393216u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPAGGREGATEDMSPCALLOBJ: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPAPC: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPBASICCALLCONTROL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPCALLINFO: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPDIRECTORY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPDIROBJCONFERENCE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPDIROBJECT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPDIROBJUSER: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPFILETRACK: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPILSCONFIG: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPLEGACYADDRESSMEDIACONTROL: u32 = 327680u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPLEGACYCALLMEDIACONTROL: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPMEDIACONTROL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPMEDIAPLAYBACK: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPMEDIARECORD: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPMEDIASUPPORT: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPMULTITRACK: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPPHONE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPTAPI: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPTAPICALLCENTER: u32 = 131072u32;
#[repr(C)]
pub struct IEnumACDGroup {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumAddress {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumAgent {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumAgentHandler {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumAgentSession {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumBstr {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppstrings: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumCall {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumCallHub {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumCallingCard {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumDialableAddrs {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumDirectory {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumDirectoryObject {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, pval: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumLocation {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumMcastScope {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppscopes: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumPhone {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumPluggableSuperclassInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumPluggableTerminalClassInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumQueue {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumStream {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumSubStream {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumTerminal {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumTerminalClass {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, pelements: *mut ::windows_sys::core::GUID, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMcastAddressAllocation {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Scopes: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Scopes: usize,
    pub EnumerateScopes: unsafe extern "system" fn(this: *mut *mut Self, ppenummcastscope: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RequestAddress: unsafe extern "system" fn(this: *mut *mut Self, pscope: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, numaddresses: i32, ppleaseresponse: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RequestAddress: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RenewAddress: unsafe extern "system" fn(this: *mut *mut Self, lreserved: i32, prenewrequest: *mut ::core::ffi::c_void, pprenewresponse: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RenewAddress: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReleaseAddress: unsafe extern "system" fn(this: *mut *mut Self, preleaserequest: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReleaseAddress: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateLeaseInfo: unsafe extern "system" fn(this: *mut *mut Self, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const ::windows_sys::core::PWSTR, prequestid: ::windows_sys::core::PCWSTR, pserveraddress: ::windows_sys::core::PCWSTR, ppreleaserequest: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateLeaseInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateLeaseInfoFromVariant: unsafe extern "system" fn(this: *mut *mut Self, leasestarttime: f64, leasestoptime: f64, vaddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, prequestid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pserveraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppreleaserequest: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateLeaseInfoFromVariant: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMcastLeaseInfo {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestID: unsafe extern "system" fn(this: *mut *mut Self, pprequestid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestID: usize,
    pub LeaseStartTime: unsafe extern "system" fn(this: *mut *mut Self, ptime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLeaseStartTime: unsafe extern "system" fn(this: *mut *mut Self, time: f64) -> ::windows_sys::core::HRESULT,
    pub LeaseStopTime: unsafe extern "system" fn(this: *mut *mut Self, ptime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLeaseStopTime: unsafe extern "system" fn(this: *mut *mut Self, time: f64) -> ::windows_sys::core::HRESULT,
    pub AddressCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ServerAddress: unsafe extern "system" fn(this: *mut *mut Self, ppaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServerAddress: usize,
    pub TTL: unsafe extern "system" fn(this: *mut *mut Self, pttl: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Addresses: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Addresses: usize,
    pub EnumerateAddresses: unsafe extern "system" fn(this: *mut *mut Self, ppenumaddresses: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMcastScope {
    pub base__: super::super::System::Com::IDispatch,
    pub ScopeID: unsafe extern "system" fn(this: *mut *mut Self, pid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ServerID: unsafe extern "system" fn(this: *mut *mut Self, pid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub InterfaceID: unsafe extern "system" fn(this: *mut *mut Self, pid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ScopeDescription: unsafe extern "system" fn(this: *mut *mut Self, ppdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ScopeDescription: usize,
    pub TTL: unsafe extern "system" fn(this: *mut *mut Self, pttl: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const INITIALIZE_NEGOTIATION: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const INTERFACEMASK: u32 = 16711680u32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITACDGroup {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, ppname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub EnumerateQueues: unsafe extern "system" fn(this: *mut *mut Self, ppenumqueue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Queues: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Queues: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITACDGroupEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Group: unsafe extern "system" fn(this: *mut *mut Self, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Group: usize,
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, pevent: *mut ACDGROUP_EVENT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITAMMediaFormat {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub MediaFormat: unsafe extern "system" fn(this: *mut *mut Self, ppmt: *mut *mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    MediaFormat: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub SetMediaFormat: unsafe extern "system" fn(this: *mut *mut Self, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    SetMediaFormat: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITASRTerminalEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(this: *mut *mut Self, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, phrerrorcode: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddress {
    pub base__: super::super::System::Com::IDispatch,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, paddressstate: *mut ADDRESS_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddressName: unsafe extern "system" fn(this: *mut *mut Self, ppname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddressName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceProviderName: unsafe extern "system" fn(this: *mut *mut Self, ppname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceProviderName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TAPIObject: unsafe extern "system" fn(this: *mut *mut Self, pptapiobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TAPIObject: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateCall: unsafe extern "system" fn(this: *mut *mut Self, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, laddresstype: i32, lmediatypes: i32, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateCall: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Calls: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Calls: usize,
    pub EnumerateCalls: unsafe extern "system" fn(this: *mut *mut Self, ppcallenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DialableAddress: unsafe extern "system" fn(this: *mut *mut Self, pdialableaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DialableAddress: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateForwardInfoObject: unsafe extern "system" fn(this: *mut *mut Self, ppforwardinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateForwardInfoObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Forward: unsafe extern "system" fn(this: *mut *mut Self, pforwardinfo: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Forward: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentForwardInfo: unsafe extern "system" fn(this: *mut *mut Self, ppforwardinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentForwardInfo: usize,
    pub SetMessageWaiting: unsafe extern "system" fn(this: *mut *mut Self, fmessagewaiting: i16) -> ::windows_sys::core::HRESULT,
    pub MessageWaiting: unsafe extern "system" fn(this: *mut *mut Self, pfmessagewaiting: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDoNotDisturb: unsafe extern "system" fn(this: *mut *mut Self, fdonotdisturb: i16) -> ::windows_sys::core::HRESULT,
    pub DoNotDisturb: unsafe extern "system" fn(this: *mut *mut Self, pfdonotdisturb: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddress2 {
    pub base__: ITAddress,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Phones: unsafe extern "system" fn(this: *mut *mut Self, pphones: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Phones: usize,
    pub EnumeratePhones: unsafe extern "system" fn(this: *mut *mut Self, ppenumphone: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPhoneFromTerminal: unsafe extern "system" fn(this: *mut *mut Self, pterminal: *mut ::core::ffi::c_void, ppphone: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPhoneFromTerminal: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PreferredPhones: unsafe extern "system" fn(this: *mut *mut Self, pphones: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PreferredPhones: usize,
    pub EnumeratePreferredPhones: unsafe extern "system" fn(this: *mut *mut Self, ppenumphone: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub get_EventFilter: unsafe extern "system" fn(this: *mut *mut Self, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut i16) -> ::windows_sys::core::HRESULT,
    pub put_EventFilter: unsafe extern "system" fn(this: *mut *mut Self, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceSpecific: unsafe extern "system" fn(this: *mut *mut Self, pcall: *mut ::core::ffi::c_void, pparams: *const u8, dwsize: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceSpecific: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeviceSpecificVariant: unsafe extern "system" fn(this: *mut *mut Self, pcall: *mut ::core::ffi::c_void, vardevspecificbytearray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeviceSpecificVariant: usize,
    pub NegotiateExtVersion: unsafe extern "system" fn(this: *mut *mut Self, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddressCapabilities {
    pub base__: super::super::System::Com::IDispatch,
    pub get_AddressCapability: unsafe extern "system" fn(this: *mut *mut Self, addresscap: ADDRESS_CAPABILITY, plcapability: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AddressCapabilityString: unsafe extern "system" fn(this: *mut *mut Self, addresscapstring: ADDRESS_CAPABILITY_STRING, ppcapabilitystring: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AddressCapabilityString: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CallTreatments: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CallTreatments: usize,
    pub EnumerateCallTreatments: unsafe extern "system" fn(this: *mut *mut Self, ppenumcalltreatment: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CompletionMessages: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CompletionMessages: usize,
    pub EnumerateCompletionMessages: unsafe extern "system" fn(this: *mut *mut Self, ppenumcompletionmessage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeviceClasses: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeviceClasses: usize,
    pub EnumerateDeviceClasses: unsafe extern "system" fn(this: *mut *mut Self, ppenumdeviceclass: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddressDeviceSpecificEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub lParam1: unsafe extern "system" fn(this: *mut *mut Self, pparam1: *mut i32) -> ::windows_sys::core::HRESULT,
    pub lParam2: unsafe extern "system" fn(this: *mut *mut Self, pparam2: *mut i32) -> ::windows_sys::core::HRESULT,
    pub lParam3: unsafe extern "system" fn(this: *mut *mut Self, pparam3: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddressEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, pevent: *mut ADDRESS_EVENT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(this: *mut *mut Self, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddressTranslation {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub TranslateAddress: unsafe extern "system" fn(this: *mut *mut Self, paddresstotranslate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcard: i32, ltranslateoptions: i32, pptranslated: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    TranslateAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TranslateDialog: unsafe extern "system" fn(this: *mut *mut Self, hwndowner: isize, paddressin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TranslateDialog: usize,
    pub EnumerateLocations: unsafe extern "system" fn(this: *mut *mut Self, ppenumlocation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Locations: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Locations: usize,
    pub EnumerateCallingCards: unsafe extern "system" fn(this: *mut *mut Self, ppenumcallingcard: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CallingCards: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CallingCards: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddressTranslationInfo {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub DialableString: unsafe extern "system" fn(this: *mut *mut Self, ppdialablestring: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DialableString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayableString: unsafe extern "system" fn(this: *mut *mut Self, ppdisplayablestring: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayableString: usize,
    pub CurrentCountryCode: unsafe extern "system" fn(this: *mut *mut Self, countrycode: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DestinationCountryCode: unsafe extern "system" fn(this: *mut *mut Self, countrycode: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TranslationResults: unsafe extern "system" fn(this: *mut *mut Self, plresults: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAgent {
    pub base__: super::super::System::Com::IDispatch,
    pub EnumerateAgentSessions: unsafe extern "system" fn(this: *mut *mut Self, ppenumagentsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSession: unsafe extern "system" fn(this: *mut *mut Self, pacdgroup: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, ppagentsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSession: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateSessionWithPIN: unsafe extern "system" fn(this: *mut *mut Self, pacdgroup: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, ppin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppagentsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateSessionWithPIN: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ID: unsafe extern "system" fn(this: *mut *mut Self, ppid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, ppuser: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    User: usize,
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, agentstate: AGENT_STATE) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, pagentstate: *mut AGENT_STATE) -> ::windows_sys::core::HRESULT,
    pub SetMeasurementPeriod: unsafe extern "system" fn(this: *mut *mut Self, lperiod: i32) -> ::windows_sys::core::HRESULT,
    pub MeasurementPeriod: unsafe extern "system" fn(this: *mut *mut Self, plperiod: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OverallCallRate: unsafe extern "system" fn(this: *mut *mut Self, pcycallrate: *mut super::super::System::Com::CY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OverallCallRate: usize,
    pub NumberOfACDCalls: unsafe extern "system" fn(this: *mut *mut Self, plcalls: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NumberOfIncomingCalls: unsafe extern "system" fn(this: *mut *mut Self, plcalls: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NumberOfOutgoingCalls: unsafe extern "system" fn(this: *mut *mut Self, plcalls: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalACDTalkTime: unsafe extern "system" fn(this: *mut *mut Self, pltalktime: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalACDCallTime: unsafe extern "system" fn(this: *mut *mut Self, plcalltime: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalWrapUpTime: unsafe extern "system" fn(this: *mut *mut Self, plwrapuptime: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AgentSessions: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AgentSessions: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAgentEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Agent: unsafe extern "system" fn(this: *mut *mut Self, ppagent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Agent: usize,
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, pevent: *mut AGENT_EVENT) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAgentHandler {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, ppname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAgent: unsafe extern "system" fn(this: *mut *mut Self, ppagent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAgent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateAgentWithID: unsafe extern "system" fn(this: *mut *mut Self, pid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppagent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateAgentWithID: usize,
    pub EnumerateACDGroups: unsafe extern "system" fn(this: *mut *mut Self, ppenumacdgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumerateUsableAddresses: unsafe extern "system" fn(this: *mut *mut Self, ppenumaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ACDGroups: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ACDGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub UsableAddresses: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    UsableAddresses: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAgentHandlerEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub AgentHandler: unsafe extern "system" fn(this: *mut *mut Self, ppagenthandler: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AgentHandler: usize,
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, pevent: *mut AGENTHANDLER_EVENT) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAgentSession {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Agent: unsafe extern "system" fn(this: *mut *mut Self, ppagent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Agent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ACDGroup: unsafe extern "system" fn(this: *mut *mut Self, ppacdgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ACDGroup: usize,
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, sessionstate: AGENT_SESSION_STATE) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, psessionstate: *mut AGENT_SESSION_STATE) -> ::windows_sys::core::HRESULT,
    pub SessionStartTime: unsafe extern "system" fn(this: *mut *mut Self, pdatesessionstart: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SessionDuration: unsafe extern "system" fn(this: *mut *mut Self, plduration: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NumberOfCalls: unsafe extern "system" fn(this: *mut *mut Self, plcalls: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalTalkTime: unsafe extern "system" fn(this: *mut *mut Self, pltalktime: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AverageTalkTime: unsafe extern "system" fn(this: *mut *mut Self, pltalktime: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalCallTime: unsafe extern "system" fn(this: *mut *mut Self, plcalltime: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AverageCallTime: unsafe extern "system" fn(this: *mut *mut Self, plcalltime: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalWrapUpTime: unsafe extern "system" fn(this: *mut *mut Self, plwrapuptime: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AverageWrapUpTime: unsafe extern "system" fn(this: *mut *mut Self, plwrapuptime: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ACDCallRate: unsafe extern "system" fn(this: *mut *mut Self, pcycallrate: *mut super::super::System::Com::CY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ACDCallRate: usize,
    pub LongestTimeToAnswer: unsafe extern "system" fn(this: *mut *mut Self, planswertime: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AverageTimeToAnswer: unsafe extern "system" fn(this: *mut *mut Self, planswertime: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAgentSessionEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Session: usize,
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, pevent: *mut AGENT_SESSION_EVENT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITAllocatorProperties {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub SetAllocatorProperties: unsafe extern "system" fn(this: *mut *mut Self, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    SetAllocatorProperties: usize,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub GetAllocatorProperties: unsafe extern "system" fn(this: *mut *mut Self, pallocproperties: *mut super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    GetAllocatorProperties: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllocateBuffers: unsafe extern "system" fn(this: *mut *mut Self, ballocbuffers: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllocateBuffers: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllocateBuffers: unsafe extern "system" fn(this: *mut *mut Self, pballocbuffers: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllocateBuffers: usize,
    pub SetBufferSize: unsafe extern "system" fn(this: *mut *mut Self, buffersize: u32) -> ::windows_sys::core::HRESULT,
    pub GetBufferSize: unsafe extern "system" fn(this: *mut *mut Self, pbuffersize: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAutomatedPhoneControl {
    pub base__: super::super::System::Com::IDispatch,
    pub StartTone: unsafe extern "system" fn(this: *mut *mut Self, tone: PHONE_TONE, lduration: i32) -> ::windows_sys::core::HRESULT,
    pub StopTone: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Tone: unsafe extern "system" fn(this: *mut *mut Self, ptone: *mut PHONE_TONE) -> ::windows_sys::core::HRESULT,
    pub StartRinger: unsafe extern "system" fn(this: *mut *mut Self, lringmode: i32, lduration: i32) -> ::windows_sys::core::HRESULT,
    pub StopRinger: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Ringer: unsafe extern "system" fn(this: *mut *mut Self, pfringing: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetPhoneHandlingEnabled: unsafe extern "system" fn(this: *mut *mut Self, fenabled: i16) -> ::windows_sys::core::HRESULT,
    pub PhoneHandlingEnabled: unsafe extern "system" fn(this: *mut *mut Self, pfenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAutoEndOfNumberTimeout: unsafe extern "system" fn(this: *mut *mut Self, ltimeout: i32) -> ::windows_sys::core::HRESULT,
    pub AutoEndOfNumberTimeout: unsafe extern "system" fn(this: *mut *mut Self, pltimeout: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAutoDialtone: unsafe extern "system" fn(this: *mut *mut Self, fenabled: i16) -> ::windows_sys::core::HRESULT,
    pub AutoDialtone: unsafe extern "system" fn(this: *mut *mut Self, pfenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAutoStopTonesOnOnHook: unsafe extern "system" fn(this: *mut *mut Self, fenabled: i16) -> ::windows_sys::core::HRESULT,
    pub AutoStopTonesOnOnHook: unsafe extern "system" fn(this: *mut *mut Self, pfenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAutoStopRingOnOffHook: unsafe extern "system" fn(this: *mut *mut Self, fenabled: i16) -> ::windows_sys::core::HRESULT,
    pub AutoStopRingOnOffHook: unsafe extern "system" fn(this: *mut *mut Self, pfenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAutoKeypadTones: unsafe extern "system" fn(this: *mut *mut Self, fenabled: i16) -> ::windows_sys::core::HRESULT,
    pub AutoKeypadTones: unsafe extern "system" fn(this: *mut *mut Self, pfenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAutoKeypadTonesMinimumDuration: unsafe extern "system" fn(this: *mut *mut Self, lduration: i32) -> ::windows_sys::core::HRESULT,
    pub AutoKeypadTonesMinimumDuration: unsafe extern "system" fn(this: *mut *mut Self, plduration: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAutoVolumeControl: unsafe extern "system" fn(this: *mut *mut Self, fenabled: i16) -> ::windows_sys::core::HRESULT,
    pub AutoVolumeControl: unsafe extern "system" fn(this: *mut *mut Self, fenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAutoVolumeControlStep: unsafe extern "system" fn(this: *mut *mut Self, lstepsize: i32) -> ::windows_sys::core::HRESULT,
    pub AutoVolumeControlStep: unsafe extern "system" fn(this: *mut *mut Self, plstepsize: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAutoVolumeControlRepeatDelay: unsafe extern "system" fn(this: *mut *mut Self, ldelay: i32) -> ::windows_sys::core::HRESULT,
    pub AutoVolumeControlRepeatDelay: unsafe extern "system" fn(this: *mut *mut Self, pldelay: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAutoVolumeControlRepeatPeriod: unsafe extern "system" fn(this: *mut *mut Self, lperiod: i32) -> ::windows_sys::core::HRESULT,
    pub AutoVolumeControlRepeatPeriod: unsafe extern "system" fn(this: *mut *mut Self, plperiod: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectCall: unsafe extern "system" fn(this: *mut *mut Self, pcall: *mut ::core::ffi::c_void, fselectdefaultterminals: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectCall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UnselectCall: unsafe extern "system" fn(this: *mut *mut Self, pcall: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UnselectCall: usize,
    pub EnumerateSelectedCalls: unsafe extern "system" fn(this: *mut *mut Self, ppcallenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SelectedCalls: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SelectedCalls: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITBasicAudioTerminal {
    pub base__: super::super::System::Com::IDispatch,
    pub SetVolume: unsafe extern "system" fn(this: *mut *mut Self, lvolume: i32) -> ::windows_sys::core::HRESULT,
    pub Volume: unsafe extern "system" fn(this: *mut *mut Self, plvolume: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBalance: unsafe extern "system" fn(this: *mut *mut Self, lbalance: i32) -> ::windows_sys::core::HRESULT,
    pub Balance: unsafe extern "system" fn(this: *mut *mut Self, plbalance: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITBasicCallControl {
    pub base__: super::super::System::Com::IDispatch,
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, fsync: i16) -> ::windows_sys::core::HRESULT,
    pub Answer: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self, code: DISCONNECT_CODE) -> ::windows_sys::core::HRESULT,
    pub Hold: unsafe extern "system" fn(this: *mut *mut Self, fhold: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HandoffDirect: unsafe extern "system" fn(this: *mut *mut Self, papplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HandoffDirect: usize,
    pub HandoffIndirect: unsafe extern "system" fn(this: *mut *mut Self, lmediatype: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Conference: unsafe extern "system" fn(this: *mut *mut Self, pcall: *mut ::core::ffi::c_void, fsync: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Conference: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Transfer: unsafe extern "system" fn(this: *mut *mut Self, pcall: *mut ::core::ffi::c_void, fsync: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Transfer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BlindTransfer: unsafe extern "system" fn(this: *mut *mut Self, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BlindTransfer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SwapHold: unsafe extern "system" fn(this: *mut *mut Self, pcall: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SwapHold: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ParkDirect: unsafe extern "system" fn(this: *mut *mut Self, pparkaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ParkDirect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ParkIndirect: unsafe extern "system" fn(this: *mut *mut Self, ppnondiraddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ParkIndirect: usize,
    pub Unpark: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetQOS: unsafe extern "system" fn(this: *mut *mut Self, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Pickup: unsafe extern "system" fn(this: *mut *mut Self, pgroupid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Pickup: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Dial: unsafe extern "system" fn(this: *mut *mut Self, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Dial: usize,
    pub Finish: unsafe extern "system" fn(this: *mut *mut Self, finishmode: FINISH_MODE) -> ::windows_sys::core::HRESULT,
    pub RemoveFromConference: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITBasicCallControl2 {
    pub base__: ITBasicCallControl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RequestTerminal: unsafe extern "system" fn(this: *mut *mut Self, bstrterminalclassguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RequestTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectTerminalOnCall: unsafe extern "system" fn(this: *mut *mut Self, pterminal: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectTerminalOnCall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UnselectTerminalOnCall: unsafe extern "system" fn(this: *mut *mut Self, pterminal: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UnselectTerminalOnCall: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallHub {
    pub base__: super::super::System::Com::IDispatch,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnumerateCalls: unsafe extern "system" fn(this: *mut *mut Self, ppenumcall: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Calls: unsafe extern "system" fn(this: *mut *mut Self, pcalls: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Calls: usize,
    pub NumCalls: unsafe extern "system" fn(this: *mut *mut Self, plcalls: *mut i32) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, pstate: *mut CALLHUB_STATE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallHubEvent {
    pub base__: super::super::System::Com::IDispatch,
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, pevent: *mut CALLHUB_EVENT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CallHub: unsafe extern "system" fn(this: *mut *mut Self, ppcallhub: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CallHub: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallInfo {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    pub CallState: unsafe extern "system" fn(this: *mut *mut Self, pcallstate: *mut CALL_STATE) -> ::windows_sys::core::HRESULT,
    pub Privilege: unsafe extern "system" fn(this: *mut *mut Self, pprivilege: *mut CALL_PRIVILEGE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CallHub: unsafe extern "system" fn(this: *mut *mut Self, ppcallhub: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CallHub: usize,
    pub get_CallInfoLong: unsafe extern "system" fn(this: *mut *mut Self, callinfolong: CALLINFO_LONG, plcallinfolongval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub put_CallInfoLong: unsafe extern "system" fn(this: *mut *mut Self, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_CallInfoString: unsafe extern "system" fn(this: *mut *mut Self, callinfostring: CALLINFO_STRING, ppcallinfostring: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_CallInfoString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_CallInfoString: unsafe extern "system" fn(this: *mut *mut Self, callinfostring: CALLINFO_STRING, pcallinfostring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_CallInfoString: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_CallInfoBuffer: unsafe extern "system" fn(this: *mut *mut Self, callinfobuffer: CALLINFO_BUFFER, ppcallinfobuffer: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_CallInfoBuffer: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub put_CallInfoBuffer: unsafe extern "system" fn(this: *mut *mut Self, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    put_CallInfoBuffer: usize,
    pub GetCallInfoBuffer: unsafe extern "system" fn(this: *mut *mut Self, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetCallInfoBuffer: unsafe extern "system" fn(this: *mut *mut Self, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> ::windows_sys::core::HRESULT,
    pub ReleaseUserUserInfo: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallInfo2 {
    pub base__: ITCallInfo,
    pub get_EventFilter: unsafe extern "system" fn(this: *mut *mut Self, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut i16) -> ::windows_sys::core::HRESULT,
    pub put_EventFilter: unsafe extern "system" fn(this: *mut *mut Self, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallInfoChangeEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Cause: unsafe extern "system" fn(this: *mut *mut Self, pcic: *mut CALLINFOCHANGE_CAUSE) -> ::windows_sys::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut *mut Self, plcallbackinstance: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallMediaEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, pcallmediaevent: *mut CALL_MEDIA_EVENT) -> ::windows_sys::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, phrerror: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(this: *mut *mut Self, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Stream: unsafe extern "system" fn(this: *mut *mut Self, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Stream: usize,
    pub Cause: unsafe extern "system" fn(this: *mut *mut Self, pcause: *mut CALL_MEDIA_EVENT_CAUSE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallNotificationEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, pcallnotificationevent: *mut CALL_NOTIFICATION_EVENT) -> ::windows_sys::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut *mut Self, plcallbackinstance: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallStateEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, pcallstate: *mut CALL_STATE) -> ::windows_sys::core::HRESULT,
    pub Cause: unsafe extern "system" fn(this: *mut *mut Self, pcec: *mut CALL_STATE_EVENT_CAUSE) -> ::windows_sys::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut *mut Self, plcallbackinstance: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallingCard {
    pub base__: super::super::System::Com::IDispatch,
    pub PermanentCardID: unsafe extern "system" fn(this: *mut *mut Self, plcardid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NumberOfDigits: unsafe extern "system" fn(this: *mut *mut Self, pldigits: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, ploptions: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CardName: unsafe extern "system" fn(this: *mut *mut Self, ppcardname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CardName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SameAreaDialingRule: unsafe extern "system" fn(this: *mut *mut Self, pprule: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SameAreaDialingRule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LongDistanceDialingRule: unsafe extern "system" fn(this: *mut *mut Self, pprule: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LongDistanceDialingRule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InternationalDialingRule: unsafe extern "system" fn(this: *mut *mut Self, pprule: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InternationalDialingRule: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCollection {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, lcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCollection2 {
    pub base__: ITCollection,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pvariant: *const super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCustomTone {
    pub base__: super::super::System::Com::IDispatch,
    pub Frequency: unsafe extern "system" fn(this: *mut *mut Self, plfrequency: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetFrequency: unsafe extern "system" fn(this: *mut *mut Self, lfrequency: i32) -> ::windows_sys::core::HRESULT,
    pub CadenceOn: unsafe extern "system" fn(this: *mut *mut Self, plcadenceon: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCadenceOn: unsafe extern "system" fn(this: *mut *mut Self, cadenceon: i32) -> ::windows_sys::core::HRESULT,
    pub CadenceOff: unsafe extern "system" fn(this: *mut *mut Self, plcadenceoff: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCadenceOff: unsafe extern "system" fn(this: *mut *mut Self, lcadenceoff: i32) -> ::windows_sys::core::HRESULT,
    pub Volume: unsafe extern "system" fn(this: *mut *mut Self, plvolume: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut *mut Self, lvolume: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDetectTone {
    pub base__: super::super::System::Com::IDispatch,
    pub AppSpecific: unsafe extern "system" fn(this: *mut *mut Self, plappspecific: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut *mut Self, lappspecific: i32) -> ::windows_sys::core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, plduration: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, lduration: i32) -> ::windows_sys::core::HRESULT,
    pub get_Frequency: unsafe extern "system" fn(this: *mut *mut Self, index: i32, plfrequency: *mut i32) -> ::windows_sys::core::HRESULT,
    pub put_Frequency: unsafe extern "system" fn(this: *mut *mut Self, index: i32, lfrequency: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDigitDetectionEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Digit: unsafe extern "system" fn(this: *mut *mut Self, pucdigit: *mut u8) -> ::windows_sys::core::HRESULT,
    pub DigitMode: unsafe extern "system" fn(this: *mut *mut Self, pdigitmode: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TickCount: unsafe extern "system" fn(this: *mut *mut Self, pltickcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut *mut Self, plcallbackinstance: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDigitGenerationEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub GenerationTermination: unsafe extern "system" fn(this: *mut *mut Self, plgenerationtermination: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TickCount: unsafe extern "system" fn(this: *mut *mut Self, pltickcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut *mut Self, plcallbackinstance: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDigitsGatheredEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Digits: unsafe extern "system" fn(this: *mut *mut Self, ppdigits: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Digits: usize,
    pub GatherTermination: unsafe extern "system" fn(this: *mut *mut Self, pgathertermination: *mut TAPI_GATHERTERM) -> ::windows_sys::core::HRESULT,
    pub TickCount: unsafe extern "system" fn(this: *mut *mut Self, pltickcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut *mut Self, plcallbackinstance: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDirectory {
    pub base__: super::super::System::Com::IDispatch,
    pub DirectoryType: unsafe extern "system" fn(this: *mut *mut Self, pdirectorytype: *mut DIRECTORY_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, pname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    pub IsDynamic: unsafe extern "system" fn(this: *mut *mut Self, pfdynamic: *mut i16) -> ::windows_sys::core::HRESULT,
    pub DefaultObjectTTL: unsafe extern "system" fn(this: *mut *mut Self, pttl: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDefaultObjectTTL: unsafe extern "system" fn(this: *mut *mut Self, ttl: i32) -> ::windows_sys::core::HRESULT,
    pub EnableAutoRefresh: unsafe extern "system" fn(this: *mut *mut Self, fenable: i16) -> ::windows_sys::core::HRESULT,
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, fsecure: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Bind: unsafe extern "system" fn(this: *mut *mut Self, pdomainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Bind: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddDirectoryObject: unsafe extern "system" fn(this: *mut *mut Self, pdirectoryobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddDirectoryObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ModifyDirectoryObject: unsafe extern "system" fn(this: *mut *mut Self, pdirectoryobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModifyDirectoryObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RefreshDirectoryObject: unsafe extern "system" fn(this: *mut *mut Self, pdirectoryobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RefreshDirectoryObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteDirectoryObject: unsafe extern "system" fn(this: *mut *mut Self, pdirectoryobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteDirectoryObject: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_DirectoryObjects: unsafe extern "system" fn(this: *mut *mut Self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_DirectoryObjects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateDirectoryObjects: unsafe extern "system" fn(this: *mut *mut Self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppenumobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateDirectoryObjects: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDirectoryObject {
    pub base__: super::super::System::Com::IDispatch,
    pub ObjectType: unsafe extern "system" fn(this: *mut *mut Self, pobjecttype: *mut DIRECTORY_OBJECT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, ppname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_DialableAddrs: unsafe extern "system" fn(this: *mut *mut Self, dwaddresstype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_DialableAddrs: usize,
    pub EnumerateDialableAddrs: unsafe extern "system" fn(this: *mut *mut Self, dwaddresstype: u32, ppenumdialableaddrs: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, ppsecdes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SecurityDescriptor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, psecdes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityDescriptor: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDirectoryObjectConference {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Protocol: unsafe extern "system" fn(this: *mut *mut Self, ppprotocol: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Protocol: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Originator: unsafe extern "system" fn(this: *mut *mut Self, pporiginator: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Originator: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOriginator: unsafe extern "system" fn(this: *mut *mut Self, poriginator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOriginator: usize,
    pub AdvertisingScope: unsafe extern "system" fn(this: *mut *mut Self, padvertisingscope: *mut RND_ADVERTISING_SCOPE) -> ::windows_sys::core::HRESULT,
    pub SetAdvertisingScope: unsafe extern "system" fn(this: *mut *mut Self, advertisingscope: RND_ADVERTISING_SCOPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Url: unsafe extern "system" fn(this: *mut *mut Self, ppurl: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Url: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUrl: unsafe extern "system" fn(this: *mut *mut Self, purl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUrl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, ppdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, pdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    pub IsEncrypted: unsafe extern "system" fn(this: *mut *mut Self, pfencrypted: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIsEncrypted: unsafe extern "system" fn(this: *mut *mut Self, fencrypted: i16) -> ::windows_sys::core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, pdate: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(this: *mut *mut Self, date: f64) -> ::windows_sys::core::HRESULT,
    pub StopTime: unsafe extern "system" fn(this: *mut *mut Self, pdate: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetStopTime: unsafe extern "system" fn(this: *mut *mut Self, date: f64) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDirectoryObjectUser {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub IPPhonePrimary: unsafe extern "system" fn(this: *mut *mut Self, ppname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IPPhonePrimary: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIPPhonePrimary: unsafe extern "system" fn(this: *mut *mut Self, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIPPhonePrimary: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDispatchMapper {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryDispatchInterface: unsafe extern "system" fn(this: *mut *mut Self, piid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pinterfacetomap: *mut ::core::ffi::c_void, ppreturnedinterface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryDispatchInterface: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITFileTerminalEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(this: *mut *mut Self, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Track: unsafe extern "system" fn(this: *mut *mut Self, pptrackterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Track: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, pstate: *mut TERMINAL_MEDIA_STATE) -> ::windows_sys::core::HRESULT,
    pub Cause: unsafe extern "system" fn(this: *mut *mut Self, pcause: *mut FT_STATE_EVENT_CAUSE) -> ::windows_sys::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, phrerrorcode: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITFileTrack {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, ppmt: *mut *mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    Format: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub SetFormat: unsafe extern "system" fn(this: *mut *mut Self, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    SetFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ControllingTerminal: unsafe extern "system" fn(this: *mut *mut Self, ppcontrollingterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ControllingTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioFormatForScripting: unsafe extern "system" fn(this: *mut *mut Self, ppaudioformat: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioFormatForScripting: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAudioFormatForScripting: unsafe extern "system" fn(this: *mut *mut Self, paudioformat: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAudioFormatForScripting: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EmptyAudioFormatForScripting: unsafe extern "system" fn(this: *mut *mut Self, ppaudioformat: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EmptyAudioFormatForScripting: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITForwardInformation {
    pub base__: super::super::System::Com::IDispatch,
    pub SetNumRingsNoAnswer: unsafe extern "system" fn(this: *mut *mut Self, lnumrings: i32) -> ::windows_sys::core::HRESULT,
    pub NumRingsNoAnswer: unsafe extern "system" fn(this: *mut *mut Self, plnumrings: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetForwardType: unsafe extern "system" fn(this: *mut *mut Self, forwardtype: i32, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcalleraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetForwardType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_ForwardTypeDestination: unsafe extern "system" fn(this: *mut *mut Self, forwardtype: i32, ppdestaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_ForwardTypeDestination: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_ForwardTypeCaller: unsafe extern "system" fn(this: *mut *mut Self, forwardtype: i32, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_ForwardTypeCaller: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForwardType: unsafe extern "system" fn(this: *mut *mut Self, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForwardType: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITForwardInformation2 {
    pub base__: ITForwardInformation,
    #[cfg(feature = "Win32_Foundation")]
    pub SetForwardType2: unsafe extern "system" fn(this: *mut *mut Self, forwardtype: i32, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, destaddresstype: i32, pcalleraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, calleraddresstype: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetForwardType2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForwardType2: unsafe extern "system" fn(this: *mut *mut Self, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, pdestaddresstype: *mut i32, ppcalleraddress: *mut super::super::Foundation::BSTR, pcalleraddresstype: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForwardType2: usize,
    pub get_ForwardTypeDestinationAddressType: unsafe extern "system" fn(this: *mut *mut Self, forwardtype: i32, pdestaddresstype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub get_ForwardTypeCallerAddressType: unsafe extern "system" fn(this: *mut *mut Self, forwardtype: i32, pcalleraddresstype: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITILSConfig {
    pub base__: super::super::System::Com::IDispatch,
    pub Port: unsafe extern "system" fn(this: *mut *mut Self, pport: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut *mut Self, port: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITLegacyAddressMediaControl {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetID: unsafe extern "system" fn(this: *mut *mut Self, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDevConfig: unsafe extern "system" fn(this: *mut *mut Self, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDevConfig: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDevConfig: unsafe extern "system" fn(this: *mut *mut Self, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsize: u32, pdeviceconfig: *const u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDevConfig: usize,
}
#[repr(C)]
pub struct ITLegacyAddressMediaControl2 {
    pub base__: ITLegacyAddressMediaControl,
    #[cfg(feature = "Win32_Foundation")]
    pub ConfigDialog: unsafe extern "system" fn(this: *mut *mut Self, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConfigDialog: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConfigDialogEdit: unsafe extern "system" fn(this: *mut *mut Self, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConfigDialogEdit: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITLegacyCallMediaControl {
    pub base__: super::super::System::Com::IDispatch,
    pub DetectDigits: unsafe extern "system" fn(this: *mut *mut Self, digitmode: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GenerateDigits: unsafe extern "system" fn(this: *mut *mut Self, pdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, digitmode: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GenerateDigits: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetID: unsafe extern "system" fn(this: *mut *mut Self, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetID: usize,
    pub SetMediaType: unsafe extern "system" fn(this: *mut *mut Self, lmediatype: i32) -> ::windows_sys::core::HRESULT,
    pub MonitorMedia: unsafe extern "system" fn(this: *mut *mut Self, lmediatype: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITLegacyCallMediaControl2 {
    pub base__: ITLegacyCallMediaControl,
    #[cfg(feature = "Win32_Foundation")]
    pub GenerateDigits2: unsafe extern "system" fn(this: *mut *mut Self, pdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, digitmode: i32, lduration: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GenerateDigits2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GatherDigits: unsafe extern "system" fn(this: *mut *mut Self, digitmode: i32, lnumdigits: i32, pterminationdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lfirstdigittimeout: i32, linterdigittimeout: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GatherDigits: usize,
    pub DetectTones: unsafe extern "system" fn(this: *mut *mut Self, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DetectTonesByCollection: unsafe extern "system" fn(this: *mut *mut Self, pdetecttonecollection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DetectTonesByCollection: usize,
    pub GenerateTone: unsafe extern "system" fn(this: *mut *mut Self, tonemode: TAPI_TONEMODE, lduration: i32) -> ::windows_sys::core::HRESULT,
    pub GenerateCustomTones: unsafe extern "system" fn(this: *mut *mut Self, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GenerateCustomTonesByCollection: unsafe extern "system" fn(this: *mut *mut Self, pcustomtonecollection: *mut ::core::ffi::c_void, lduration: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GenerateCustomTonesByCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDetectToneObject: unsafe extern "system" fn(this: *mut *mut Self, ppdetecttone: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDetectToneObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateCustomToneObject: unsafe extern "system" fn(this: *mut *mut Self, ppcustomtone: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateCustomToneObject: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetIDAsVariant: unsafe extern "system" fn(this: *mut *mut Self, bstrdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardeviceid: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetIDAsVariant: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITLegacyWaveSupport {
    pub base__: super::super::System::Com::IDispatch,
    pub IsFullDuplex: unsafe extern "system" fn(this: *mut *mut Self, psupport: *mut FULLDUPLEX_SUPPORT) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITLocationInfo {
    pub base__: super::super::System::Com::IDispatch,
    pub PermanentLocationID: unsafe extern "system" fn(this: *mut *mut Self, pllocationid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CountryCode: unsafe extern "system" fn(this: *mut *mut Self, plcountrycode: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CountryID: unsafe extern "system" fn(this: *mut *mut Self, plcountryid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, ploptions: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PreferredCardID: unsafe extern "system" fn(this: *mut *mut Self, plcardid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LocationName: unsafe extern "system" fn(this: *mut *mut Self, pplocationname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LocationName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CityCode: unsafe extern "system" fn(this: *mut *mut Self, ppcode: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CityCode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LocalAccessCode: unsafe extern "system" fn(this: *mut *mut Self, ppcode: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LocalAccessCode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LongDistanceAccessCode: unsafe extern "system" fn(this: *mut *mut Self, ppcode: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LongDistanceAccessCode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TollPrefixList: unsafe extern "system" fn(this: *mut *mut Self, pptolllist: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TollPrefixList: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CancelCallWaitingCode: unsafe extern "system" fn(this: *mut *mut Self, ppcode: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CancelCallWaitingCode: usize,
}
#[repr(C)]
pub struct ITMSPAddress {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, hevent: *const i32) -> ::windows_sys::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CreateMSPCall: unsafe extern "system" fn(this: *mut *mut Self, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: *mut ::core::ffi::c_void, ppstreamcontrol: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShutdownMSPCall: unsafe extern "system" fn(this: *mut *mut Self, pstreamcontrol: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReceiveTSPData: unsafe extern "system" fn(this: *mut *mut Self, pmspcall: *mut ::core::ffi::c_void, pbuffer: *const u8, dwsize: u32) -> ::windows_sys::core::HRESULT,
    pub GetEvent: unsafe extern "system" fn(this: *mut *mut Self, pdwsize: *mut u32, peventbuffer: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITMediaControl {
    pub base__: super::super::System::Com::IDispatch,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MediaState: unsafe extern "system" fn(this: *mut *mut Self, pterminalmediastate: *mut TERMINAL_MEDIA_STATE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITMediaPlayback {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPlayList: unsafe extern "system" fn(this: *mut *mut Self, playlistvariant: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPlayList: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PlayList: unsafe extern "system" fn(this: *mut *mut Self, pplaylistvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PlayList: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITMediaRecord {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFileName: unsafe extern "system" fn(this: *mut *mut Self, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FileName: unsafe extern "system" fn(this: *mut *mut Self, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FileName: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITMediaSupport {
    pub base__: super::super::System::Com::IDispatch,
    pub MediaTypes: unsafe extern "system" fn(this: *mut *mut Self, plmediatypes: *mut i32) -> ::windows_sys::core::HRESULT,
    pub QueryMediaType: unsafe extern "system" fn(this: *mut *mut Self, lmediatype: i32, pfsupport: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITMultiTrackTerminal {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TrackTerminals: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TrackTerminals: usize,
    pub EnumerateTrackTerminals: unsafe extern "system" fn(this: *mut *mut Self, ppenumterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTrackTerminal: unsafe extern "system" fn(this: *mut *mut Self, mediatype: i32, terminaldirection: TERMINAL_DIRECTION, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTrackTerminal: usize,
    pub MediaTypesInUse: unsafe extern "system" fn(this: *mut *mut Self, plmediatypesinuse: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DirectionsInUse: unsafe extern "system" fn(this: *mut *mut Self, pldirectionsinused: *mut TERMINAL_DIRECTION) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveTrackTerminal: unsafe extern "system" fn(this: *mut *mut Self, ptrackterminaltoremove: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveTrackTerminal: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITPhone {
    pub base__: super::super::System::Com::IDispatch,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, privilege: PHONE_PRIVILEGE) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Addresses: unsafe extern "system" fn(this: *mut *mut Self, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Addresses: usize,
    pub EnumerateAddresses: unsafe extern "system" fn(this: *mut *mut Self, ppenumaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub get_PhoneCapsLong: unsafe extern "system" fn(this: *mut *mut Self, pclcap: PHONECAPS_LONG, plcapability: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_PhoneCapsString: unsafe extern "system" fn(this: *mut *mut Self, pcscap: PHONECAPS_STRING, ppcapability: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_PhoneCapsString: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Terminals: unsafe extern "system" fn(this: *mut *mut Self, paddress: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Terminals: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateTerminals: unsafe extern "system" fn(this: *mut *mut Self, paddress: *mut ::core::ffi::c_void, ppenumterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateTerminals: usize,
    pub get_ButtonMode: unsafe extern "system" fn(this: *mut *mut Self, lbuttonid: i32, pbuttonmode: *mut PHONE_BUTTON_MODE) -> ::windows_sys::core::HRESULT,
    pub put_ButtonMode: unsafe extern "system" fn(this: *mut *mut Self, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> ::windows_sys::core::HRESULT,
    pub get_ButtonFunction: unsafe extern "system" fn(this: *mut *mut Self, lbuttonid: i32, pbuttonfunction: *mut PHONE_BUTTON_FUNCTION) -> ::windows_sys::core::HRESULT,
    pub put_ButtonFunction: unsafe extern "system" fn(this: *mut *mut Self, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_ButtonText: unsafe extern "system" fn(this: *mut *mut Self, lbuttonid: i32, ppbuttontext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_ButtonText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_ButtonText: unsafe extern "system" fn(this: *mut *mut Self, lbuttonid: i32, bstrbuttontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_ButtonText: usize,
    pub get_ButtonState: unsafe extern "system" fn(this: *mut *mut Self, lbuttonid: i32, pbuttonstate: *mut PHONE_BUTTON_STATE) -> ::windows_sys::core::HRESULT,
    pub get_HookSwitchState: unsafe extern "system" fn(this: *mut *mut Self, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, phookswitchstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows_sys::core::HRESULT,
    pub put_HookSwitchState: unsafe extern "system" fn(this: *mut *mut Self, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> ::windows_sys::core::HRESULT,
    pub SetRingMode: unsafe extern "system" fn(this: *mut *mut Self, lringmode: i32) -> ::windows_sys::core::HRESULT,
    pub RingMode: unsafe extern "system" fn(this: *mut *mut Self, plringmode: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRingVolume: unsafe extern "system" fn(this: *mut *mut Self, lringvolume: i32) -> ::windows_sys::core::HRESULT,
    pub RingVolume: unsafe extern "system" fn(this: *mut *mut Self, plringvolume: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Privilege: unsafe extern "system" fn(this: *mut *mut Self, pprivilege: *mut PHONE_PRIVILEGE) -> ::windows_sys::core::HRESULT,
    pub GetPhoneCapsBuffer: unsafe extern "system" fn(this: *mut *mut Self, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_PhoneCapsBuffer: unsafe extern "system" fn(this: *mut *mut Self, pcbcaps: PHONECAPS_BUFFER, pvarbuffer: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_PhoneCapsBuffer: usize,
    pub get_LampMode: unsafe extern "system" fn(this: *mut *mut Self, llampid: i32, plampmode: *mut PHONE_LAMP_MODE) -> ::windows_sys::core::HRESULT,
    pub put_LampMode: unsafe extern "system" fn(this: *mut *mut Self, llampid: i32, lampmode: PHONE_LAMP_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Display: unsafe extern "system" fn(this: *mut *mut Self, pbstrdisplay: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Display: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplay: unsafe extern "system" fn(this: *mut *mut Self, lrow: i32, lcolumn: i32, bstrdisplay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplay: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PreferredAddresses: unsafe extern "system" fn(this: *mut *mut Self, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PreferredAddresses: usize,
    pub EnumeratePreferredAddresses: unsafe extern "system" fn(this: *mut *mut Self, ppenumaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeviceSpecific: unsafe extern "system" fn(this: *mut *mut Self, pparams: *const u8, dwsize: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeviceSpecificVariant: unsafe extern "system" fn(this: *mut *mut Self, vardevspecificbytearray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeviceSpecificVariant: usize,
    pub NegotiateExtVersion: unsafe extern "system" fn(this: *mut *mut Self, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITPhoneDeviceSpecificEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Phone: unsafe extern "system" fn(this: *mut *mut Self, ppphone: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Phone: usize,
    pub lParam1: unsafe extern "system" fn(this: *mut *mut Self, pparam1: *mut i32) -> ::windows_sys::core::HRESULT,
    pub lParam2: unsafe extern "system" fn(this: *mut *mut Self, pparam2: *mut i32) -> ::windows_sys::core::HRESULT,
    pub lParam3: unsafe extern "system" fn(this: *mut *mut Self, pparam3: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITPhoneEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Phone: unsafe extern "system" fn(this: *mut *mut Self, ppphone: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Phone: usize,
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, pevent: *mut PHONE_EVENT) -> ::windows_sys::core::HRESULT,
    pub ButtonState: unsafe extern "system" fn(this: *mut *mut Self, pstate: *mut PHONE_BUTTON_STATE) -> ::windows_sys::core::HRESULT,
    pub HookSwitchState: unsafe extern "system" fn(this: *mut *mut Self, pstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows_sys::core::HRESULT,
    pub HookSwitchDevice: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut PHONE_HOOK_SWITCH_DEVICE) -> ::windows_sys::core::HRESULT,
    pub RingMode: unsafe extern "system" fn(this: *mut *mut Self, plringmode: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ButtonLampId: unsafe extern "system" fn(this: *mut *mut Self, plbuttonlampid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NumberGathered: unsafe extern "system" fn(this: *mut *mut Self, ppnumber: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NumberGathered: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITPluggableTerminalClassInfo {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Company: unsafe extern "system" fn(this: *mut *mut Self, pcompany: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Company: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, pversion: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Version: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TerminalClass: unsafe extern "system" fn(this: *mut *mut Self, pterminalclass: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TerminalClass: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CLSID: unsafe extern "system" fn(this: *mut *mut Self, pclsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CLSID: usize,
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, pdirection: *mut TERMINAL_DIRECTION) -> ::windows_sys::core::HRESULT,
    pub MediaTypes: unsafe extern "system" fn(this: *mut *mut Self, pmediatypes: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITPluggableTerminalEventSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub FireEvent: unsafe extern "system" fn(this: *mut *mut Self, pmspeventinfo: *const MSP_EVENT_INFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FireEvent: usize,
}
#[repr(C)]
pub struct ITPluggableTerminalEventSinkRegistration {
    pub base__: ::windows_sys::core::IUnknown,
    pub RegisterSink: unsafe extern "system" fn(this: *mut *mut Self, peventsink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnregisterSink: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITPluggableTerminalSuperclassInfo {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CLSID: unsafe extern "system" fn(this: *mut *mut Self, pclsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CLSID: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITPrivateEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CallHub: unsafe extern "system" fn(this: *mut *mut Self, ppcallhub: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CallHub: usize,
    pub EventCode: unsafe extern "system" fn(this: *mut *mut Self, pleventcode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EventInterface: unsafe extern "system" fn(this: *mut *mut Self, peventinterface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EventInterface: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITQOSEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, pqosevent: *mut QOS_EVENT) -> ::windows_sys::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut *mut Self, plmediatype: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITQueue {
    pub base__: super::super::System::Com::IDispatch,
    pub SetMeasurementPeriod: unsafe extern "system" fn(this: *mut *mut Self, lperiod: i32) -> ::windows_sys::core::HRESULT,
    pub MeasurementPeriod: unsafe extern "system" fn(this: *mut *mut Self, plperiod: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalCallsQueued: unsafe extern "system" fn(this: *mut *mut Self, plcalls: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CurrentCallsQueued: unsafe extern "system" fn(this: *mut *mut Self, plcalls: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalCallsAbandoned: unsafe extern "system" fn(this: *mut *mut Self, plcalls: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalCallsFlowedIn: unsafe extern "system" fn(this: *mut *mut Self, plcalls: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TotalCallsFlowedOut: unsafe extern "system" fn(this: *mut *mut Self, plcalls: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LongestEverWaitTime: unsafe extern "system" fn(this: *mut *mut Self, plwaittime: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CurrentLongestWaitTime: unsafe extern "system" fn(this: *mut *mut Self, plwaittime: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AverageWaitTime: unsafe extern "system" fn(this: *mut *mut Self, plwaittime: *mut i32) -> ::windows_sys::core::HRESULT,
    pub FinalDisposition: unsafe extern "system" fn(this: *mut *mut Self, plcalls: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, ppname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITQueueEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Queue: unsafe extern "system" fn(this: *mut *mut Self, ppqueue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Queue: usize,
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, pevent: *mut ACDQUEUE_EVENT) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITRendezvous {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DefaultDirectories: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DefaultDirectories: usize,
    pub EnumerateDefaultDirectories: unsafe extern "system" fn(this: *mut *mut Self, ppenumdirectory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateDirectory: unsafe extern "system" fn(this: *mut *mut Self, directorytype: DIRECTORY_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdir: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateDirectory: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateDirectoryObject: unsafe extern "system" fn(this: *mut *mut Self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdirectoryobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateDirectoryObject: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITRequest {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub MakeCall: unsafe extern "system" fn(this: *mut *mut Self, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pappname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcalledparty: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MakeCall: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITRequestEvent {
    pub base__: super::super::System::Com::IDispatch,
    pub RegistrationInstance: unsafe extern "system" fn(this: *mut *mut Self, plregistrationinstance: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RequestMode: unsafe extern "system" fn(this: *mut *mut Self, plrequestmode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DestAddress: unsafe extern "system" fn(this: *mut *mut Self, ppdestaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DestAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AppName: unsafe extern "system" fn(this: *mut *mut Self, ppappname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AppName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CalledParty: unsafe extern "system" fn(this: *mut *mut Self, ppcalledparty: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CalledParty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Comment: unsafe extern "system" fn(this: *mut *mut Self, ppcomment: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Comment: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITScriptableAudioFormat {
    pub base__: super::super::System::Com::IDispatch,
    pub Channels: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetChannels: unsafe extern "system" fn(this: *mut *mut Self, nnewval: i32) -> ::windows_sys::core::HRESULT,
    pub SamplesPerSec: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSamplesPerSec: unsafe extern "system" fn(this: *mut *mut Self, nnewval: i32) -> ::windows_sys::core::HRESULT,
    pub AvgBytesPerSec: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAvgBytesPerSec: unsafe extern "system" fn(this: *mut *mut Self, nnewval: i32) -> ::windows_sys::core::HRESULT,
    pub BlockAlign: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBlockAlign: unsafe extern "system" fn(this: *mut *mut Self, nnewval: i32) -> ::windows_sys::core::HRESULT,
    pub BitsPerSample: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBitsPerSample: unsafe extern "system" fn(this: *mut *mut Self, nnewval: i32) -> ::windows_sys::core::HRESULT,
    pub FormatTag: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetFormatTag: unsafe extern "system" fn(this: *mut *mut Self, nnewval: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITStaticAudioTerminal {
    pub base__: super::super::System::Com::IDispatch,
    pub WaveId: unsafe extern "system" fn(this: *mut *mut Self, plwaveid: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITStream {
    pub base__: super::super::System::Com::IDispatch,
    pub MediaType: unsafe extern "system" fn(this: *mut *mut Self, plmediatype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, ptd: *mut TERMINAL_DIRECTION) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, ppname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub StartStream: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PauseStream: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StopStream: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectTerminal: unsafe extern "system" fn(this: *mut *mut Self, pterminal: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UnselectTerminal: unsafe extern "system" fn(this: *mut *mut Self, pterminal: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UnselectTerminal: usize,
    pub EnumerateTerminals: unsafe extern "system" fn(this: *mut *mut Self, ppenumterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Terminals: unsafe extern "system" fn(this: *mut *mut Self, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Terminals: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITStreamControl {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStream: unsafe extern "system" fn(this: *mut *mut Self, lmediatype: i32, td: TERMINAL_DIRECTION, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveStream: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveStream: usize,
    pub EnumerateStreams: unsafe extern "system" fn(this: *mut *mut Self, ppenumstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Streams: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Streams: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITSubStream {
    pub base__: super::super::System::Com::IDispatch,
    pub StartSubStream: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PauseSubStream: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StopSubStream: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectTerminal: unsafe extern "system" fn(this: *mut *mut Self, pterminal: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UnselectTerminal: unsafe extern "system" fn(this: *mut *mut Self, pterminal: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UnselectTerminal: usize,
    pub EnumerateTerminals: unsafe extern "system" fn(this: *mut *mut Self, ppenumterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Terminals: unsafe extern "system" fn(this: *mut *mut Self, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Terminals: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Stream: unsafe extern "system" fn(this: *mut *mut Self, ppitstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Stream: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITSubStreamControl {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSubStream: unsafe extern "system" fn(this: *mut *mut Self, ppsubstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSubStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveSubStream: unsafe extern "system" fn(this: *mut *mut Self, psubstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveSubStream: usize,
    pub EnumerateSubStreams: unsafe extern "system" fn(this: *mut *mut Self, ppenumsubstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SubStreams: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SubStreams: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTAPI {
    pub base__: super::super::System::Com::IDispatch,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Addresses: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Addresses: usize,
    pub EnumerateAddresses: unsafe extern "system" fn(this: *mut *mut Self, ppenumaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RegisterCallNotifications: unsafe extern "system" fn(this: *mut *mut Self, paddress: *mut ::core::ffi::c_void, fmonitor: i16, fowner: i16, lmediatypes: i32, lcallbackinstance: i32, plregister: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegisterCallNotifications: usize,
    pub UnregisterNotifications: unsafe extern "system" fn(this: *mut *mut Self, lregister: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CallHubs: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CallHubs: usize,
    pub EnumerateCallHubs: unsafe extern "system" fn(this: *mut *mut Self, ppenumcallhub: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCallHubTracking: unsafe extern "system" fn(this: *mut *mut Self, paddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, btracking: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCallHubTracking: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumeratePrivateTAPIObjects: unsafe extern "system" fn(this: *mut *mut Self, ppenumunknown: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumeratePrivateTAPIObjects: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PrivateTAPIObjects: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PrivateTAPIObjects: usize,
    pub RegisterRequestRecipient: unsafe extern "system" fn(this: *mut *mut Self, lregistrationinstance: i32, lrequestmode: i32, fenable: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAssistedTelephonyPriority: unsafe extern "system" fn(this: *mut *mut Self, pappfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpriority: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAssistedTelephonyPriority: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetApplicationPriority: unsafe extern "system" fn(this: *mut *mut Self, pappfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, fpriority: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetApplicationPriority: usize,
    pub SetEventFilter: unsafe extern "system" fn(this: *mut *mut Self, lfiltermask: i32) -> ::windows_sys::core::HRESULT,
    pub EventFilter: unsafe extern "system" fn(this: *mut *mut Self, plfiltermask: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTAPI2 {
    pub base__: ITTAPI,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Phones: unsafe extern "system" fn(this: *mut *mut Self, pphones: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Phones: usize,
    pub EnumeratePhones: unsafe extern "system" fn(this: *mut *mut Self, ppenumphone: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEmptyCollectionObject: unsafe extern "system" fn(this: *mut *mut Self, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEmptyCollectionObject: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTAPICallCenter {
    pub base__: super::super::System::Com::IDispatch,
    pub EnumerateAgentHandlers: unsafe extern "system" fn(this: *mut *mut Self, ppenumhandler: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AgentHandlers: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AgentHandlers: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTAPIDispatchEventNotification {
    pub base__: super::super::System::Com::IDispatch,
}
#[repr(C)]
pub struct ITTAPIEventNotification {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, tapievent: TAPI_EVENT, pevent: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Event: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTAPIObjectEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub TAPIObject: unsafe extern "system" fn(this: *mut *mut Self, pptapiobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TAPIObject: usize,
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, pevent: *mut TAPIOBJECT_EVENT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut *mut Self, plcallbackinstance: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTAPIObjectEvent2 {
    pub base__: ITTAPIObjectEvent,
    #[cfg(feature = "Win32_System_Com")]
    pub Phone: unsafe extern "system" fn(this: *mut *mut Self, ppphone: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Phone: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTTSTerminalEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(this: *mut *mut Self, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, phrerrorcode: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTerminal {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, ppname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, pterminalstate: *mut TERMINAL_STATE) -> ::windows_sys::core::HRESULT,
    pub TerminalType: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut TERMINAL_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TerminalClass: unsafe extern "system" fn(this: *mut *mut Self, ppterminalclass: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TerminalClass: usize,
    pub MediaType: unsafe extern "system" fn(this: *mut *mut Self, plmediatype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, pdirection: *mut TERMINAL_DIRECTION) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTerminalSupport {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StaticTerminals: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StaticTerminals: usize,
    pub EnumerateStaticTerminals: unsafe extern "system" fn(this: *mut *mut Self, ppterminalenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DynamicTerminalClasses: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DynamicTerminalClasses: usize,
    pub EnumerateDynamicTerminalClasses: unsafe extern "system" fn(this: *mut *mut Self, ppterminalclassenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateTerminal: unsafe extern "system" fn(this: *mut *mut Self, pterminalclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDefaultStaticTerminal: unsafe extern "system" fn(this: *mut *mut Self, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDefaultStaticTerminal: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTerminalSupport2 {
    pub base__: ITTerminalSupport,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PluggableSuperclasses: unsafe extern "system" fn(this: *mut *mut Self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PluggableSuperclasses: usize,
    pub EnumeratePluggableSuperclasses: unsafe extern "system" fn(this: *mut *mut Self, ppsuperclassenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_PluggableTerminalClasses: unsafe extern "system" fn(this: *mut *mut Self, bstrterminalsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_PluggableTerminalClasses: usize,
    pub EnumeratePluggableTerminalClasses: unsafe extern "system" fn(this: *mut *mut Self, iidterminalsuperclass: ::windows_sys::core::GUID, lmediatype: i32, ppclassenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITToneDetectionEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut *mut Self, plappspecific: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TickCount: unsafe extern "system" fn(this: *mut *mut Self, pltickcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut *mut Self, plcallbackinstance: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITToneTerminalEvent {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(this: *mut *mut Self, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, phrerrorcode: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITnef {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_AddressBook")]
    pub AddProps: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, ulelemid: u32, lpvdata: *mut ::core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_AddressBook"))]
    AddProps: usize,
    #[cfg(feature = "Win32_System_AddressBook")]
    pub ExtractProps: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_AddressBook"))]
    ExtractProps: usize,
    pub Finish: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub OpenTaggedBody: unsafe extern "system" fn(this: *mut *mut Self, lpmessage: *mut ::core::ffi::c_void, ulflags: u32, lppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com")))]
    OpenTaggedBody: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub SetProps: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com")))]
    SetProps: usize,
    #[cfg(feature = "Win32_System_AddressBook")]
    pub EncodeRecips: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lprecipienttable: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_AddressBook"))]
    EncodeRecips: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub FinishComponent: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com")))]
    FinishComponent: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LAST_LINEMEDIAMODE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LAST_LINEREQUESTMODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_ACCEPTTOALERT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_ACDGROUP: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_AUTORECONNECT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_BLOCKIDDEFAULT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_BLOCKIDOVERRIDE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_COMPLETIONID: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_CONFDROP: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_CONFERENCEHELD: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_CONFERENCEMAKE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_DESTOFFHOOK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_DIALED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_FWDBUSYNAADDR: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_FWDCONSULT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_FWDINTEXTADDR: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_FWDNUMRINGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_FWDSTATUSVALID: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_HOLDMAKESNEW: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_NOEXTERNALCALLS: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_NOINTERNALCALLS: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_NOPSTNADDRESSTRANSLATION: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_ORIGOFFHOOK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_PARTIALDIAL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_PICKUPCALLWAIT: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_PICKUPGROUPID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_PREDICTIVEDIALER: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_QUEUE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_ROUTEPOINT: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_SECURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_SETCALLINGID: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_SETUPCONFNULL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_TRANSFERHELD: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_TRANSFERMAKE: u32 = 8192u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEADDRESSCAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwLineDeviceID: u32,
    pub dwAddressSize: u32,
    pub dwAddressOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwAddressSharing: u32,
    pub dwAddressStates: u32,
    pub dwCallInfoStates: u32,
    pub dwCallerIDFlags: u32,
    pub dwCalledIDFlags: u32,
    pub dwConnectedIDFlags: u32,
    pub dwRedirectionIDFlags: u32,
    pub dwRedirectingIDFlags: u32,
    pub dwCallStates: u32,
    pub dwDialToneModes: u32,
    pub dwBusyModes: u32,
    pub dwSpecialInfo: u32,
    pub dwDisconnectModes: u32,
    pub dwMaxNumActiveCalls: u32,
    pub dwMaxNumOnHoldCalls: u32,
    pub dwMaxNumOnHoldPendingCalls: u32,
    pub dwMaxNumConference: u32,
    pub dwMaxNumTransConf: u32,
    pub dwAddrCapFlags: u32,
    pub dwCallFeatures: u32,
    pub dwRemoveFromConfCaps: u32,
    pub dwRemoveFromConfState: u32,
    pub dwTransferModes: u32,
    pub dwParkModes: u32,
    pub dwForwardModes: u32,
    pub dwMaxForwardEntries: u32,
    pub dwMaxSpecificEntries: u32,
    pub dwMinFwdNumRings: u32,
    pub dwMaxFwdNumRings: u32,
    pub dwMaxCallCompletions: u32,
    pub dwCallCompletionConds: u32,
    pub dwCallCompletionModes: u32,
    pub dwNumCompletionMessages: u32,
    pub dwCompletionMsgTextEntrySize: u32,
    pub dwCompletionMsgTextSize: u32,
    pub dwCompletionMsgTextOffset: u32,
    pub dwAddressFeatures: u32,
    pub dwPredictiveAutoTransferStates: u32,
    pub dwNumCallTreatments: u32,
    pub dwCallTreatmentListSize: u32,
    pub dwCallTreatmentListOffset: u32,
    pub dwDeviceClassesSize: u32,
    pub dwDeviceClassesOffset: u32,
    pub dwMaxCallDataSize: u32,
    pub dwCallFeatures2: u32,
    pub dwMaxNoAnswerTimeout: u32,
    pub dwConnectedModes: u32,
    pub dwOfferingModes: u32,
    pub dwAvailableMediaModes: u32,
}
impl ::core::marker::Copy for LINEADDRESSCAPS {}
impl ::core::clone::Clone for LINEADDRESSCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSMODE_ADDRESSID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSMODE_DIALABLEADDR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSHARING_BRIDGEDEXCL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSHARING_BRIDGEDNEW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSHARING_BRIDGEDSHARED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSHARING_MONITORED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSHARING_PRIVATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_CAPSCHANGE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_DEVSPECIFIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_FORWARD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_INUSEMANY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_INUSEONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_INUSEZERO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_NUMCALLS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_OTHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_TERMINALS: u32 = 128u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEADDRESSSTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumInUse: u32,
    pub dwNumActiveCalls: u32,
    pub dwNumOnHoldCalls: u32,
    pub dwNumOnHoldPendCalls: u32,
    pub dwAddressFeatures: u32,
    pub dwNumRingsNoAnswer: u32,
    pub dwForwardNumEntries: u32,
    pub dwForwardSize: u32,
    pub dwForwardOffset: u32,
    pub dwTerminalModesSize: u32,
    pub dwTerminalModesOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
}
impl ::core::marker::Copy for LINEADDRESSSTATUS {}
impl ::core::clone::Clone for LINEADDRESSSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSTYPE_DOMAINNAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSTYPE_EMAILNAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSTYPE_IPADDRESS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSTYPE_PHONENUMBER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSTYPE_SDP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_FORWARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_FORWARDDND: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_FORWARDFWD: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_MAKECALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_PICKUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_PICKUPDIRECT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_PICKUPGROUP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_PICKUPHELD: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_PICKUPWAITING: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_SETMEDIACONTROL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_SETTERMINAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_SETUPCONF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_UNCOMPLETECALL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_UNPARK: u32 = 128u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTACTIVITYENTRY {
    pub dwID: u32,
    pub dwNameSize: u32,
    pub dwNameOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTACTIVITYENTRY {}
impl ::core::clone::Clone for LINEAGENTACTIVITYENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTACTIVITYLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTACTIVITYLIST {}
impl ::core::clone::Clone for LINEAGENTACTIVITYLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTCAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwAgentHandlerInfoSize: u32,
    pub dwAgentHandlerInfoOffset: u32,
    pub dwCapsVersion: u32,
    pub dwFeatures: u32,
    pub dwStates: u32,
    pub dwNextStates: u32,
    pub dwMaxNumGroupEntries: u32,
    pub dwAgentStatusMessages: u32,
    pub dwNumAgentExtensionIDs: u32,
    pub dwAgentExtensionIDListSize: u32,
    pub dwAgentExtensionIDListOffset: u32,
    pub ProxyGUID: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for LINEAGENTCAPS {}
impl ::core::clone::Clone for LINEAGENTCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTENTRY {
    pub hAgent: u32,
    pub dwNameSize: u32,
    pub dwNameOffset: u32,
    pub dwIDSize: u32,
    pub dwIDOffset: u32,
    pub dwPINSize: u32,
    pub dwPINOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTENTRY {}
impl ::core::clone::Clone for LINEAGENTENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTFEATURE_AGENTSPECIFIC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTFEATURE_GETAGENTACTIVITYLIST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTFEATURE_GETAGENTGROUP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTFEATURE_SETAGENTACTIVITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTFEATURE_SETAGENTGROUP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTFEATURE_SETAGENTSTATE: u32 = 2u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTGROUPENTRY {
    pub GroupID: LINEAGENTGROUPENTRY_0,
    pub dwNameSize: u32,
    pub dwNameOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTGROUPENTRY {}
impl ::core::clone::Clone for LINEAGENTGROUPENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTGROUPENTRY_0 {
    pub dwGroupID1: u32,
    pub dwGroupID2: u32,
    pub dwGroupID3: u32,
    pub dwGroupID4: u32,
}
impl ::core::marker::Copy for LINEAGENTGROUPENTRY_0 {}
impl ::core::clone::Clone for LINEAGENTGROUPENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTGROUPLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTGROUPLIST {}
impl ::core::clone::Clone for LINEAGENTGROUPLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEAGENTINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwAgentState: u32,
    pub dwNextAgentState: u32,
    pub dwMeasurementPeriod: u32,
    pub cyOverallCallRate: super::super::System::Com::CY,
    pub dwNumberOfACDCalls: u32,
    pub dwNumberOfIncomingCalls: u32,
    pub dwNumberOfOutgoingCalls: u32,
    pub dwTotalACDTalkTime: u32,
    pub dwTotalACDCallTime: u32,
    pub dwTotalACDWrapUpTime: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEAGENTINFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEAGENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTLIST {}
impl ::core::clone::Clone for LINEAGENTLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTSESSIONENTRY {
    pub hAgentSession: u32,
    pub hAgent: u32,
    pub GroupID: ::windows_sys::core::GUID,
    pub dwWorkingAddressID: u32,
}
impl ::core::marker::Copy for LINEAGENTSESSIONENTRY {}
impl ::core::clone::Clone for LINEAGENTSESSIONENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEAGENTSESSIONINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwAgentSessionState: u32,
    pub dwNextAgentSessionState: u32,
    pub dateSessionStartTime: f64,
    pub dwSessionDuration: u32,
    pub dwNumberOfCalls: u32,
    pub dwTotalTalkTime: u32,
    pub dwAverageTalkTime: u32,
    pub dwTotalCallTime: u32,
    pub dwAverageCallTime: u32,
    pub dwTotalWrapUpTime: u32,
    pub dwAverageWrapUpTime: u32,
    pub cyACDCallRate: super::super::System::Com::CY,
    pub dwLongestTimeToAnswer: u32,
    pub dwAverageTimeToAnswer: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEAGENTSESSIONINFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEAGENTSESSIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTSESSIONLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTSESSIONLIST {}
impl ::core::clone::Clone for LINEAGENTSESSIONLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATE_BUSYONCALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATE_BUSYWRAPUP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATE_ENDED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATE_NOTREADY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATE_READY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATE_RELEASED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATUS_NEWSESSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATUS_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATUS_UPDATEINFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_BUSYACD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_BUSYINCOMING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_BUSYOUTGOING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_NOTREADY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_READY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_RELEASED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_UNKNOWN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_BUSYACD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_BUSYINCOMING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_BUSYOTHER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_BUSYOUTBOUND: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_LOGGEDOFF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_NOTREADY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_READY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_UNAVAIL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_UNKNOWN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_WORKINGAFTERCALL: u32 = 128u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTSTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwGroupListSize: u32,
    pub dwGroupListOffset: u32,
    pub dwState: u32,
    pub dwNextState: u32,
    pub dwActivityID: u32,
    pub dwActivitySize: u32,
    pub dwActivityOffset: u32,
    pub dwAgentFeatures: u32,
    pub dwValidStates: u32,
    pub dwValidNextStates: u32,
}
impl ::core::marker::Copy for LINEAGENTSTATUS {}
impl ::core::clone::Clone for LINEAGENTSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUSEX_NEWAGENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUSEX_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUSEX_UPDATEINFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_ACTIVITY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_ACTIVITYLIST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_CAPSCHANGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_GROUP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_GROUPLIST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_NEXTSTATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_VALIDNEXTSTATES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_VALIDSTATES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEANSWERMODE_DROP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEANSWERMODE_HOLD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEANSWERMODE_NONE: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAPPINFO {
    pub dwMachineNameSize: u32,
    pub dwMachineNameOffset: u32,
    pub dwUserNameSize: u32,
    pub dwUserNameOffset: u32,
    pub dwModuleFilenameSize: u32,
    pub dwModuleFilenameOffset: u32,
    pub dwFriendlyNameSize: u32,
    pub dwFriendlyNameOffset: u32,
    pub dwMediaModes: u32,
    pub dwAddressID: u32,
}
impl ::core::marker::Copy for LINEAPPINFO {}
impl ::core::clone::Clone for LINEAPPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_ALTSPEECHDATA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_DATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_MULTIUSE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_NONCALLSIGNALING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_PASSTHROUGH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_RESTRICTEDDATA: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_SPEECH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_VOICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBUSYMODE_STATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBUSYMODE_TRUNK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBUSYMODE_UNAVAIL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBUSYMODE_UNKNOWN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type LINECALLBACK = ::core::option::Option<unsafe extern "system" fn(hdevice: u32, dwmessage: u32, dwinstance: usize, dwparam1: usize, dwparam2: usize, dwparam3: usize)>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLCOMPLCOND_BUSY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLCOMPLCOND_NOANSWER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLCOMPLMODE_CALLBACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLCOMPLMODE_CAMPON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLCOMPLMODE_INTRUDE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLCOMPLMODE_MESSAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_COMPLCALLBACK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_COMPLCAMPON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_COMPLINTRUDE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_COMPLMESSAGE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_NOHOLDCONFERENCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_ONESTEPTRANSFER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_PARKDIRECT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_PARKNONDIRECT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_TRANSFERCONF: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_TRANSFERNORM: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_ACCEPT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_ADDTOCONF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_ANSWER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_BLINDTRANSFER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_COMPLETECALL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_COMPLETETRANSF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_DIAL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_DROP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_GATHERDIGITS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_GENERATEDIGITS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_GENERATETONE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_HOLD: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_MONITORDIGITS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_MONITORMEDIA: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_MONITORTONES: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_PARK: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_PREPAREADDCONF: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_REDIRECT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_RELEASEUSERUSERINFO: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_REMOVEFROMCONF: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SECURECALL: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SENDUSERUSER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETCALLDATA: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETCALLPARAMS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETMEDIACONTROL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETQOS: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETTERMINAL: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETTREATMENT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETUPCONF: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETUPTRANSFER: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SWAPHOLD: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_UNHOLD: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLHUBTRACKING_ALLCALLS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLHUBTRACKING_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLHUBTRACKING_PROVIDERLEVEL: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECALLINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub hLine: u32,
    pub dwLineDeviceID: u32,
    pub dwAddressID: u32,
    pub dwBearerMode: u32,
    pub dwRate: u32,
    pub dwMediaMode: u32,
    pub dwAppSpecific: u32,
    pub dwCallID: u32,
    pub dwRelatedCallID: u32,
    pub dwCallParamFlags: u32,
    pub dwCallStates: u32,
    pub dwMonitorDigitModes: u32,
    pub dwMonitorMediaModes: u32,
    pub DialParams: LINEDIALPARAMS,
    pub dwOrigin: u32,
    pub dwReason: u32,
    pub dwCompletionID: u32,
    pub dwNumOwners: u32,
    pub dwNumMonitors: u32,
    pub dwCountryCode: u32,
    pub dwTrunk: u32,
    pub dwCallerIDFlags: u32,
    pub dwCallerIDSize: u32,
    pub dwCallerIDOffset: u32,
    pub dwCallerIDNameSize: u32,
    pub dwCallerIDNameOffset: u32,
    pub dwCalledIDFlags: u32,
    pub dwCalledIDSize: u32,
    pub dwCalledIDOffset: u32,
    pub dwCalledIDNameSize: u32,
    pub dwCalledIDNameOffset: u32,
    pub dwConnectedIDFlags: u32,
    pub dwConnectedIDSize: u32,
    pub dwConnectedIDOffset: u32,
    pub dwConnectedIDNameSize: u32,
    pub dwConnectedIDNameOffset: u32,
    pub dwRedirectionIDFlags: u32,
    pub dwRedirectionIDSize: u32,
    pub dwRedirectionIDOffset: u32,
    pub dwRedirectionIDNameSize: u32,
    pub dwRedirectionIDNameOffset: u32,
    pub dwRedirectingIDFlags: u32,
    pub dwRedirectingIDSize: u32,
    pub dwRedirectingIDOffset: u32,
    pub dwRedirectingIDNameSize: u32,
    pub dwRedirectingIDNameOffset: u32,
    pub dwAppNameSize: u32,
    pub dwAppNameOffset: u32,
    pub dwDisplayableAddressSize: u32,
    pub dwDisplayableAddressOffset: u32,
    pub dwCalledPartySize: u32,
    pub dwCalledPartyOffset: u32,
    pub dwCommentSize: u32,
    pub dwCommentOffset: u32,
    pub dwDisplaySize: u32,
    pub dwDisplayOffset: u32,
    pub dwUserUserInfoSize: u32,
    pub dwUserUserInfoOffset: u32,
    pub dwHighLevelCompSize: u32,
    pub dwHighLevelCompOffset: u32,
    pub dwLowLevelCompSize: u32,
    pub dwLowLevelCompOffset: u32,
    pub dwChargingInfoSize: u32,
    pub dwChargingInfoOffset: u32,
    pub dwTerminalModesSize: u32,
    pub dwTerminalModesOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwCallTreatment: u32,
    pub dwCallDataSize: u32,
    pub dwCallDataOffset: u32,
    pub dwSendingFlowspecSize: u32,
    pub dwSendingFlowspecOffset: u32,
    pub dwReceivingFlowspecSize: u32,
    pub dwReceivingFlowspecOffset: u32,
}
impl ::core::marker::Copy for LINECALLINFO {}
impl ::core::clone::Clone for LINECALLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_APPSPECIFIC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_BEARERMODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_CALLDATA: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_CALLEDID: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_CALLERID: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_CALLID: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_CHARGINGINFO: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_COMPLETIONID: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_CONNECTEDID: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_DEVSPECIFIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_DIALPARAMS: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_DISPLAY: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_HIGHLEVELCOMP: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_LOWLEVELCOMP: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_MEDIAMODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_MONITORMODES: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_NUMMONITORS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_NUMOWNERDECR: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_NUMOWNERINCR: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_ORIGIN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_OTHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_QOS: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_RATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_REASON: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_REDIRECTINGID: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_REDIRECTIONID: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_RELATEDCALLID: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_TERMINAL: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_TREATMENT: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_TRUNK: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_USERUSERINFO: u32 = 2097152u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECALLLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwCallsNumEntries: u32,
    pub dwCallsSize: u32,
    pub dwCallsOffset: u32,
}
impl ::core::marker::Copy for LINECALLLIST {}
impl ::core::clone::Clone for LINECALLLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_CONFERENCE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_EXTERNAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_INBOUND: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_INTERNAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_OUTBOUND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_UNAVAIL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_UNKNOWN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_BLOCKID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_DESTOFFHOOK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_IDLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_NOHOLDCONFERENCE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_ONESTEPTRANSFER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_ORIGOFFHOOK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_PREDICTIVEDIAL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_SECURE: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECALLPARAMS {
    pub dwTotalSize: u32,
    pub dwBearerMode: u32,
    pub dwMinRate: u32,
    pub dwMaxRate: u32,
    pub dwMediaMode: u32,
    pub dwCallParamFlags: u32,
    pub dwAddressMode: u32,
    pub dwAddressID: u32,
    pub DialParams: LINEDIALPARAMS,
    pub dwOrigAddressSize: u32,
    pub dwOrigAddressOffset: u32,
    pub dwDisplayableAddressSize: u32,
    pub dwDisplayableAddressOffset: u32,
    pub dwCalledPartySize: u32,
    pub dwCalledPartyOffset: u32,
    pub dwCommentSize: u32,
    pub dwCommentOffset: u32,
    pub dwUserUserInfoSize: u32,
    pub dwUserUserInfoOffset: u32,
    pub dwHighLevelCompSize: u32,
    pub dwHighLevelCompOffset: u32,
    pub dwLowLevelCompSize: u32,
    pub dwLowLevelCompOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwPredictiveAutoTransferStates: u32,
    pub dwTargetAddressSize: u32,
    pub dwTargetAddressOffset: u32,
    pub dwSendingFlowspecSize: u32,
    pub dwSendingFlowspecOffset: u32,
    pub dwReceivingFlowspecSize: u32,
    pub dwReceivingFlowspecOffset: u32,
    pub dwDeviceClassSize: u32,
    pub dwDeviceClassOffset: u32,
    pub dwDeviceConfigSize: u32,
    pub dwDeviceConfigOffset: u32,
    pub dwCallDataSize: u32,
    pub dwCallDataOffset: u32,
    pub dwNoAnswerTimeout: u32,
    pub dwCallingPartyIDSize: u32,
    pub dwCallingPartyIDOffset: u32,
}
impl ::core::marker::Copy for LINECALLPARAMS {}
impl ::core::clone::Clone for LINECALLPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_ADDRESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_BLOCKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_NAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_OUTOFAREA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_PARTIAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_UNAVAIL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_UNKNOWN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPRIVILEGE_MONITOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPRIVILEGE_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPRIVILEGE_OWNER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_CALLCOMPLETION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_CAMPEDON: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_DIRECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_FWDBUSY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_FWDNOANSWER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_FWDUNCOND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_INTRUDE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_PARKED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_PICKUP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_REDIRECT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_REMINDER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_ROUTEREQUEST: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_TRANSFER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_UNAVAIL: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_UNKNOWN: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_UNPARK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSELECT_ADDRESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSELECT_CALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSELECT_CALLID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSELECT_DEVICEID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSELECT_LINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_ACCEPTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_BUSY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_CONFERENCED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_CONNECTED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_DIALING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_DIALTONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_DISCONNECTED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_IDLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_OFFERING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_ONHOLD: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_ONHOLDPENDCONF: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_ONHOLDPENDTRANSFER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_PROCEEDING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_RINGBACK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_SPECIALINFO: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_UNKNOWN: u32 = 32768u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LINECALLSTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwCallState: u32,
    pub dwCallStateMode: u32,
    pub dwCallPrivilege: u32,
    pub dwCallFeatures: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwCallFeatures2: u32,
    pub tStateEntryTime: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LINECALLSTATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LINECALLSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECALLTREATMENTENTRY {
    pub dwCallTreatmentID: u32,
    pub dwCallTreatmentNameSize: u32,
    pub dwCallTreatmentNameOffset: u32,
}
impl ::core::marker::Copy for LINECALLTREATMENTENTRY {}
impl ::core::clone::Clone for LINECALLTREATMENTENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLTREATMENT_BUSY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLTREATMENT_MUSIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLTREATMENT_RINGBACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLTREATMENT_SILENCE: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECARDENTRY {
    pub dwPermanentCardID: u32,
    pub dwCardNameSize: u32,
    pub dwCardNameOffset: u32,
    pub dwCardNumberDigits: u32,
    pub dwSameAreaRuleSize: u32,
    pub dwSameAreaRuleOffset: u32,
    pub dwLongDistanceRuleSize: u32,
    pub dwLongDistanceRuleOffset: u32,
    pub dwInternationalRuleSize: u32,
    pub dwInternationalRuleOffset: u32,
    pub dwOptions: u32,
}
impl ::core::marker::Copy for LINECARDENTRY {}
impl ::core::clone::Clone for LINECARDENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECARDOPTION_HIDDEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECARDOPTION_PREDEFINED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECONNECTEDMODE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECONNECTEDMODE_ACTIVEHELD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECONNECTEDMODE_CONFIRMED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECONNECTEDMODE_INACTIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECONNECTEDMODE_INACTIVEHELD: u32 = 8u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECOUNTRYENTRY {
    pub dwCountryID: u32,
    pub dwCountryCode: u32,
    pub dwNextCountryID: u32,
    pub dwCountryNameSize: u32,
    pub dwCountryNameOffset: u32,
    pub dwSameAreaRuleSize: u32,
    pub dwSameAreaRuleOffset: u32,
    pub dwLongDistanceRuleSize: u32,
    pub dwLongDistanceRuleOffset: u32,
    pub dwInternationalRuleSize: u32,
    pub dwInternationalRuleOffset: u32,
}
impl ::core::marker::Copy for LINECOUNTRYENTRY {}
impl ::core::clone::Clone for LINECOUNTRYENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECOUNTRYLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumCountries: u32,
    pub dwCountryListSize: u32,
    pub dwCountryListOffset: u32,
}
impl ::core::marker::Copy for LINECOUNTRYLIST {}
impl ::core::clone::Clone for LINECOUNTRYLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_CALLHUB: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_CALLHUBTRACKING: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_CLOSEDROP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_CROSSADDRCONF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_DIALBILLING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_DIALDIALTONE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_DIALQUIET: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_HIGHLEVCOMP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_LOCAL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_LOWLEVCOMP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_MEDIACONTROL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_MSP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_MULTIPLEADDR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_PRIVATEOBJECTS: u32 = 4096u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEDEVCAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwProviderInfoSize: u32,
    pub dwProviderInfoOffset: u32,
    pub dwSwitchInfoSize: u32,
    pub dwSwitchInfoOffset: u32,
    pub dwPermanentLineID: u32,
    pub dwLineNameSize: u32,
    pub dwLineNameOffset: u32,
    pub dwStringFormat: u32,
    pub dwAddressModes: u32,
    pub dwNumAddresses: u32,
    pub dwBearerModes: u32,
    pub dwMaxRate: u32,
    pub dwMediaModes: u32,
    pub dwGenerateToneModes: u32,
    pub dwGenerateToneMaxNumFreq: u32,
    pub dwGenerateDigitModes: u32,
    pub dwMonitorToneMaxNumFreq: u32,
    pub dwMonitorToneMaxNumEntries: u32,
    pub dwMonitorDigitModes: u32,
    pub dwGatherDigitsMinTimeout: u32,
    pub dwGatherDigitsMaxTimeout: u32,
    pub dwMedCtlDigitMaxListSize: u32,
    pub dwMedCtlMediaMaxListSize: u32,
    pub dwMedCtlToneMaxListSize: u32,
    pub dwMedCtlCallStateMaxListSize: u32,
    pub dwDevCapFlags: u32,
    pub dwMaxNumActiveCalls: u32,
    pub dwAnswerMode: u32,
    pub dwRingModes: u32,
    pub dwLineStates: u32,
    pub dwUUIAcceptSize: u32,
    pub dwUUIAnswerSize: u32,
    pub dwUUIMakeCallSize: u32,
    pub dwUUIDropSize: u32,
    pub dwUUISendUserUserInfoSize: u32,
    pub dwUUICallInfoSize: u32,
    pub MinDialParams: LINEDIALPARAMS,
    pub MaxDialParams: LINEDIALPARAMS,
    pub DefaultDialParams: LINEDIALPARAMS,
    pub dwNumTerminals: u32,
    pub dwTerminalCapsSize: u32,
    pub dwTerminalCapsOffset: u32,
    pub dwTerminalTextEntrySize: u32,
    pub dwTerminalTextSize: u32,
    pub dwTerminalTextOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwLineFeatures: u32,
    pub dwSettableDevStatus: u32,
    pub dwDeviceClassesSize: u32,
    pub dwDeviceClassesOffset: u32,
    pub PermanentLineGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for LINEDEVCAPS {}
impl ::core::clone::Clone for LINEDEVCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_BATTERY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_CAPSCHANGE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_CLOSE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_COMPLCANCEL: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_CONFIGCHANGE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_CONNECTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_DEVSPECIFIC: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_DISCONNECTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_INSERVICE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_LOCK: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_MAINTENANCE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_MSGWAITOFF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_MSGWAITON: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_NUMCALLS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_NUMCOMPLETIONS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_OPEN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_OTHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_OUTOFSERVICE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_REINIT: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_REMOVED: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_RINGING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_ROAMMODE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_SIGNAL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_TERMINALS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_TRANSLATECHANGE: u32 = 4194304u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEDEVSTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumOpens: u32,
    pub dwOpenMediaModes: u32,
    pub dwNumActiveCalls: u32,
    pub dwNumOnHoldCalls: u32,
    pub dwNumOnHoldPendCalls: u32,
    pub dwLineFeatures: u32,
    pub dwNumCallCompletions: u32,
    pub dwRingMode: u32,
    pub dwSignalLevel: u32,
    pub dwBatteryLevel: u32,
    pub dwRoamMode: u32,
    pub dwDevStatusFlags: u32,
    pub dwTerminalModesSize: u32,
    pub dwTerminalModesOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwAvailableMediaModes: u32,
    pub dwAppInfoSize: u32,
    pub dwAppInfoOffset: u32,
}
impl ::core::marker::Copy for LINEDEVSTATUS {}
impl ::core::clone::Clone for LINEDEVSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATUSFLAGS_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATUSFLAGS_INSERVICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATUSFLAGS_LOCKED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATUSFLAGS_MSGWAIT: u32 = 2u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEDIALPARAMS {
    pub dwDialPause: u32,
    pub dwDialSpeed: u32,
    pub dwDigitDuration: u32,
    pub dwWaitForDialtone: u32,
}
impl ::core::marker::Copy for LINEDIALPARAMS {}
impl ::core::clone::Clone for LINEDIALPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIALTONEMODE_EXTERNAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIALTONEMODE_INTERNAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIALTONEMODE_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIALTONEMODE_SPECIAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIALTONEMODE_UNAVAIL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIALTONEMODE_UNKNOWN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIGITMODE_DTMF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIGITMODE_DTMFEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIGITMODE_PULSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_BADADDRESS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_BLOCKED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_BUSY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_CANCELLED: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_CONGESTION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_DESTINATIONBARRED: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_DONOTDISTURB: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_FDNRESTRICT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_FORWARDED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_INCOMPATIBLE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_NOANSWER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_NODIALTONE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_NUMBERCHANGED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_OUTOFORDER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_PICKUP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_QOSUNAVAIL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_REJECT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_TEMPFAILURE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_UNAVAIL: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_UNKNOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_UNREACHABLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEEQOSINFO_ADMISSIONFAILURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEEQOSINFO_GENERICERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEEQOSINFO_NOQOS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEEQOSINFO_POLICYFAILURE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_ADDRESSBLOCKED: u32 = 2147483731u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_ALLOCATED: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_BADDEVICEID: u32 = 2147483650u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_BEARERMODEUNAVAIL: u32 = 2147483651u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_BILLINGREJECTED: u32 = 2147483732u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_CALLUNAVAIL: u32 = 2147483653u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_COMPLETIONOVERRUN: u32 = 2147483654u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_CONFERENCEFULL: u32 = 2147483655u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_DIALBILLING: u32 = 2147483656u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_DIALDIALTONE: u32 = 2147483657u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_DIALPROMPT: u32 = 2147483658u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_DIALQUIET: u32 = 2147483659u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_DIALVOICEDETECT: u32 = 2147483740u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_DISCONNECTED: u32 = 2147483744u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INCOMPATIBLEAPIVERSION: u32 = 2147483660u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INCOMPATIBLEEXTVERSION: u32 = 2147483661u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INIFILECORRUPT: u32 = 2147483662u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INUSE: u32 = 2147483663u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALADDRESS: u32 = 2147483664u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALADDRESSID: u32 = 2147483665u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALADDRESSMODE: u32 = 2147483666u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALADDRESSSTATE: u32 = 2147483667u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALADDRESSTYPE: u32 = 2147483742u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAGENTACTIVITY: u32 = 2147483739u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAGENTGROUP: u32 = 2147483736u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAGENTID: u32 = 2147483735u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAGENTSESSIONSTATE: u32 = 2147483743u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAGENTSTATE: u32 = 2147483738u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAPPHANDLE: u32 = 2147483668u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAPPNAME: u32 = 2147483669u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALBEARERMODE: u32 = 2147483670u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLCOMPLMODE: u32 = 2147483671u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLHANDLE: u32 = 2147483672u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLPARAMS: u32 = 2147483673u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLPRIVILEGE: u32 = 2147483674u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLSELECT: u32 = 2147483675u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLSTATE: u32 = 2147483676u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLSTATELIST: u32 = 2147483677u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCARD: u32 = 2147483678u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCOMPLETIONID: u32 = 2147483679u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCONFCALLHANDLE: u32 = 2147483680u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCONSULTCALLHANDLE: u32 = 2147483681u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCOUNTRYCODE: u32 = 2147483682u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALDEVICECLASS: u32 = 2147483683u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALDEVICEHANDLE: u32 = 2147483684u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALDIALPARAMS: u32 = 2147483685u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALDIGITLIST: u32 = 2147483686u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALDIGITMODE: u32 = 2147483687u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALDIGITS: u32 = 2147483688u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALEXTVERSION: u32 = 2147483689u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALFEATURE: u32 = 2147483733u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALGROUPID: u32 = 2147483690u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALLINEHANDLE: u32 = 2147483691u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALLINESTATE: u32 = 2147483692u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALLOCATION: u32 = 2147483693u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALMEDIALIST: u32 = 2147483694u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALMEDIAMODE: u32 = 2147483695u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALMESSAGEID: u32 = 2147483696u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALPARAM: u32 = 2147483698u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALPARKID: u32 = 2147483699u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALPARKMODE: u32 = 2147483700u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALPASSWORD: u32 = 2147483737u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALPOINTER: u32 = 2147483701u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALPRIVSELECT: u32 = 2147483702u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALRATE: u32 = 2147483703u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALREQUESTMODE: u32 = 2147483704u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTERMINALID: u32 = 2147483705u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTERMINALMODE: u32 = 2147483706u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTIMEOUT: u32 = 2147483707u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTONE: u32 = 2147483708u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTONELIST: u32 = 2147483709u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTONEMODE: u32 = 2147483710u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTRANSFERMODE: u32 = 2147483711u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_LINEMAPPERFAILED: u32 = 2147483712u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NOCONFERENCE: u32 = 2147483713u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NODEVICE: u32 = 2147483714u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NODRIVER: u32 = 2147483715u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NOMEM: u32 = 2147483716u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NOMULTIPLEINSTANCE: u32 = 2147483734u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NOREQUEST: u32 = 2147483717u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NOTOWNER: u32 = 2147483718u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NOTREGISTERED: u32 = 2147483719u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_OPERATIONFAILED: u32 = 2147483720u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_OPERATIONUNAVAIL: u32 = 2147483721u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_RATEUNAVAIL: u32 = 2147483722u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_REINIT: u32 = 2147483730u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_REQUESTOVERRUN: u32 = 2147483724u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_RESOURCEUNAVAIL: u32 = 2147483723u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_SERVICE_NOT_RUNNING: u32 = 2147483745u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_STRUCTURETOOSMALL: u32 = 2147483725u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_TARGETNOTFOUND: u32 = 2147483726u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_TARGETSELF: u32 = 2147483727u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_UNINITIALIZED: u32 = 2147483728u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_USERCANCELLED: u32 = 2147483741u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_USERUSERINFOTOOBIG: u32 = 2147483729u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type LINEEVENT = ::core::option::Option<unsafe extern "system" fn(htline: *mut HTAPILINE__, htcall: *mut HTAPICALL__, dwmsg: u32, dwparam1: usize, dwparam2: usize, dwparam3: usize)>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEEXTENSIONID {
    pub dwExtensionID0: u32,
    pub dwExtensionID1: u32,
    pub dwExtensionID2: u32,
    pub dwExtensionID3: u32,
}
impl ::core::marker::Copy for LINEEXTENSIONID {}
impl ::core::clone::Clone for LINEEXTENSIONID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_DEVSPECIFIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_DEVSPECIFICFEAT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_FORWARD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_FORWARDDND: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_FORWARDFWD: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_MAKECALL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_SETDEVSTATUS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_SETMEDIACONTROL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_SETTERMINAL: u32 = 32u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEFORWARD {
    pub dwForwardMode: u32,
    pub dwCallerAddressSize: u32,
    pub dwCallerAddressOffset: u32,
    pub dwDestCountryCode: u32,
    pub dwDestAddressSize: u32,
    pub dwDestAddressOffset: u32,
}
impl ::core::marker::Copy for LINEFORWARD {}
impl ::core::clone::Clone for LINEFORWARD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEFORWARDLIST {
    pub dwTotalSize: u32,
    pub dwNumEntries: u32,
    pub ForwardList: [LINEFORWARD; 1],
}
impl ::core::marker::Copy for LINEFORWARDLIST {}
impl ::core::clone::Clone for LINEFORWARDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYEXTERNAL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYINTERNAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYNA: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYNAEXTERNAL: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYNAINTERNAL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYNASPECIFIC: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYSPECIFIC: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_NOANSW: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_NOANSWEXTERNAL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_NOANSWINTERNAL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_NOANSWSPECIFIC: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_UNAVAIL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_UNCOND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_UNCONDEXTERNAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_UNCONDINTERNAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_UNCONDSPECIFIC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_UNKNOWN: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGATHERTERM_BUFFERFULL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGATHERTERM_CANCEL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGATHERTERM_FIRSTTIMEOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGATHERTERM_INTERTIMEOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGATHERTERM_TERMDIGIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGENERATETERM_CANCEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGENERATETERM_DONE: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEGENERATETONE {
    pub dwFrequency: u32,
    pub dwCadenceOn: u32,
    pub dwCadenceOff: u32,
    pub dwVolume: u32,
}
impl ::core::marker::Copy for LINEGENERATETONE {}
impl ::core::clone::Clone for LINEGENERATETONE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGROUPSTATUS_GROUPREMOVED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGROUPSTATUS_NEWGROUP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEINITIALIZEEXOPTION_CALLHUBTRACKING: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEINITIALIZEEXOPTION_USECOMPLETIONPORT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEINITIALIZEEXOPTION_USEEVENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEINITIALIZEEXOPTION_USEHIDDENWINDOW: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LINEINITIALIZEEXPARAMS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwOptions: u32,
    pub Handles: LINEINITIALIZEEXPARAMS_0,
    pub dwCompletionKey: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LINEINITIALIZEEXPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LINEINITIALIZEEXPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union LINEINITIALIZEEXPARAMS_0 {
    pub hEvent: super::super::Foundation::HANDLE,
    pub hCompletionPort: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LINEINITIALIZEEXPARAMS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LINEINITIALIZEEXPARAMS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINELOCATIONENTRY {
    pub dwPermanentLocationID: u32,
    pub dwLocationNameSize: u32,
    pub dwLocationNameOffset: u32,
    pub dwCountryCode: u32,
    pub dwCityCodeSize: u32,
    pub dwCityCodeOffset: u32,
    pub dwPreferredCardID: u32,
    pub dwLocalAccessCodeSize: u32,
    pub dwLocalAccessCodeOffset: u32,
    pub dwLongDistanceAccessCodeSize: u32,
    pub dwLongDistanceAccessCodeOffset: u32,
    pub dwTollPrefixListSize: u32,
    pub dwTollPrefixListOffset: u32,
    pub dwCountryID: u32,
    pub dwOptions: u32,
    pub dwCancelCallWaitingSize: u32,
    pub dwCancelCallWaitingOffset: u32,
}
impl ::core::marker::Copy for LINELOCATIONENTRY {}
impl ::core::clone::Clone for LINELOCATIONENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINELOCATIONOPTION_PULSEDIAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMAPPER: u32 = 4294967295u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEMEDIACONTROLCALLSTATE {
    pub dwCallStates: u32,
    pub dwMediaControl: u32,
}
impl ::core::marker::Copy for LINEMEDIACONTROLCALLSTATE {}
impl ::core::clone::Clone for LINEMEDIACONTROLCALLSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEMEDIACONTROLDIGIT {
    pub dwDigit: u32,
    pub dwDigitModes: u32,
    pub dwMediaControl: u32,
}
impl ::core::marker::Copy for LINEMEDIACONTROLDIGIT {}
impl ::core::clone::Clone for LINEMEDIACONTROLDIGIT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEMEDIACONTROLMEDIA {
    pub dwMediaModes: u32,
    pub dwDuration: u32,
    pub dwMediaControl: u32,
}
impl ::core::marker::Copy for LINEMEDIACONTROLMEDIA {}
impl ::core::clone::Clone for LINEMEDIACONTROLMEDIA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEMEDIACONTROLTONE {
    pub dwAppSpecific: u32,
    pub dwDuration: u32,
    pub dwFrequency1: u32,
    pub dwFrequency2: u32,
    pub dwFrequency3: u32,
    pub dwMediaControl: u32,
}
impl ::core::marker::Copy for LINEMEDIACONTROLTONE {}
impl ::core::clone::Clone for LINEMEDIACONTROLTONE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_PAUSE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_RATEDOWN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_RATENORMAL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_RATEUP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_RESET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_RESUME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_START: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_VOLUMEDOWN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_VOLUMENORMAL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_VOLUMEUP: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_ADSI: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_AUTOMATEDVOICE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_DATAMODEM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_DIGITALDATA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_G3FAX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_G4FAX: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_INTERACTIVEVOICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_MIXED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_TDD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_TELETEX: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_TELEX: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_UNKNOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_VIDEO: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_VIDEOTEX: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_VOICEVIEW: u32 = 16384u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEMESSAGE {
    pub hDevice: u32,
    pub dwMessageID: u32,
    pub dwCallbackInstance: usize,
    pub dwParam1: usize,
    pub dwParam2: usize,
    pub dwParam3: usize,
}
impl ::core::marker::Copy for LINEMESSAGE {}
impl ::core::clone::Clone for LINEMESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEMONITORTONE {
    pub dwAppSpecific: u32,
    pub dwDuration: u32,
    pub dwFrequency1: u32,
    pub dwFrequency2: u32,
    pub dwFrequency3: u32,
}
impl ::core::marker::Copy for LINEMONITORTONE {}
impl ::core::clone::Clone for LINEMONITORTONE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEOFFERINGMODE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEOFFERINGMODE_INACTIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEOPENOPTION_PROXY: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEOPENOPTION_SINGLEADDRESS: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPARKMODE_DIRECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPARKMODE_NONDIRECTED: u32 = 2u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEPROVIDERENTRY {
    pub dwPermanentProviderID: u32,
    pub dwProviderFilenameSize: u32,
    pub dwProviderFilenameOffset: u32,
}
impl ::core::marker::Copy for LINEPROVIDERENTRY {}
impl ::core::clone::Clone for LINEPROVIDERENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEPROVIDERLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumProviders: u32,
    pub dwProviderListSize: u32,
    pub dwProviderListOffset: u32,
}
impl ::core::marker::Copy for LINEPROVIDERLIST {}
impl ::core::clone::Clone for LINEPROVIDERLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST {
    pub dwSize: u32,
    pub dwClientMachineNameSize: u32,
    pub dwClientMachineNameOffset: u32,
    pub dwClientUserNameSize: u32,
    pub dwClientUserNameOffset: u32,
    pub dwClientAppAPIVersion: u32,
    pub dwRequestType: u32,
    pub Anonymous: LINEPROXYREQUEST_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union LINEPROXYREQUEST_0 {
    pub SetAgentGroup: LINEPROXYREQUEST_0_14,
    pub SetAgentState: LINEPROXYREQUEST_0_18,
    pub SetAgentActivity: LINEPROXYREQUEST_0_13,
    pub GetAgentCaps: LINEPROXYREQUEST_0_4,
    pub GetAgentStatus: LINEPROXYREQUEST_0_9,
    pub AgentSpecific: LINEPROXYREQUEST_0_0,
    pub GetAgentActivityList: LINEPROXYREQUEST_0_3,
    pub GetAgentGroupList: LINEPROXYREQUEST_0_5,
    pub CreateAgent: LINEPROXYREQUEST_0_2,
    pub SetAgentStateEx: LINEPROXYREQUEST_0_17,
    pub SetAgentMeasurementPeriod: LINEPROXYREQUEST_0_15,
    pub GetAgentInfo: LINEPROXYREQUEST_0_6,
    pub CreateAgentSession: LINEPROXYREQUEST_0_1,
    pub GetAgentSessionList: LINEPROXYREQUEST_0_8,
    pub GetAgentSessionInfo: LINEPROXYREQUEST_0_7,
    pub SetAgentSessionState: LINEPROXYREQUEST_0_16,
    pub GetQueueList: LINEPROXYREQUEST_0_12,
    pub SetQueueMeasurementPeriod: LINEPROXYREQUEST_0_19,
    pub GetQueueInfo: LINEPROXYREQUEST_0_11,
    pub GetGroupList: LINEPROXYREQUEST_0_10,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_0 {
    pub dwAddressID: u32,
    pub dwAgentExtensionIDIndex: u32,
    pub dwSize: u32,
    pub Params: [u8; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_1 {
    pub hAgentSession: u32,
    pub dwAgentPINSize: u32,
    pub dwAgentPINOffset: u32,
    pub hAgent: u32,
    pub GroupID: ::windows_sys::core::GUID,
    pub dwWorkingAddressID: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_1 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_2 {
    pub hAgent: u32,
    pub dwAgentIDSize: u32,
    pub dwAgentIDOffset: u32,
    pub dwAgentPINSize: u32,
    pub dwAgentPINOffset: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_3 {
    pub dwAddressID: u32,
    pub ActivityList: LINEAGENTACTIVITYLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_4 {
    pub dwAddressID: u32,
    pub AgentCaps: LINEAGENTCAPS,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_5 {
    pub dwAddressID: u32,
    pub GroupList: LINEAGENTGROUPLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_5 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_6 {
    pub hAgent: u32,
    pub AgentInfo: LINEAGENTINFO,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_6 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_7 {
    pub hAgentSession: u32,
    pub SessionInfo: LINEAGENTSESSIONINFO,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_7 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_8 {
    pub hAgent: u32,
    pub SessionList: LINEAGENTSESSIONLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_8 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_9 {
    pub dwAddressID: u32,
    pub AgentStatus: LINEAGENTSTATUS,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_9 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_10 {
    pub GroupList: LINEAGENTGROUPLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_10 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_11 {
    pub dwQueueID: u32,
    pub QueueInfo: LINEQUEUEINFO,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_11 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_11 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_12 {
    pub GroupID: ::windows_sys::core::GUID,
    pub QueueList: LINEQUEUELIST,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_12 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_12 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_13 {
    pub dwAddressID: u32,
    pub dwActivityID: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_13 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_13 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_14 {
    pub dwAddressID: u32,
    pub GroupList: LINEAGENTGROUPLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_14 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_14 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_15 {
    pub hAgent: u32,
    pub dwMeasurementPeriod: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_15 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_15 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_16 {
    pub hAgentSession: u32,
    pub dwAgentSessionState: u32,
    pub dwNextAgentSessionState: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_16 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_17 {
    pub hAgent: u32,
    pub dwAgentState: u32,
    pub dwNextAgentState: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_17 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_17 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_18 {
    pub dwAddressID: u32,
    pub dwAgentState: u32,
    pub dwNextAgentState: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_18 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_18 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_19 {
    pub dwQueueID: u32,
    pub dwMeasurementPeriod: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_19 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_19 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEPROXYREQUESTLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl ::core::marker::Copy for LINEPROXYREQUESTLIST {}
impl ::core::clone::Clone for LINEPROXYREQUESTLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_AGENTSPECIFIC: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_CREATEAGENT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_CREATEAGENTSESSION: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTACTIVITYLIST: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTCAPS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTGROUPLIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTINFO: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTSESSIONINFO: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTSESSIONLIST: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTSTATUS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETGROUPLIST: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETQUEUEINFO: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETQUEUELIST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETAGENTACTIVITY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETAGENTGROUP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETAGENTMEASUREMENTPERIOD: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETAGENTSESSIONSTATE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETAGENTSTATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETAGENTSTATEEX: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETQUEUEMEASUREMENTPERIOD: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYSTATUS_ALLOPENFORACD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYSTATUS_CLOSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYSTATUS_OPEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQOSREQUESTTYPE_SERVICELEVEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQOSSERVICELEVEL_BESTEFFORT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQOSSERVICELEVEL_IFAVAILABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQOSSERVICELEVEL_NEEDED: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEQUEUEENTRY {
    pub dwQueueID: u32,
    pub dwNameSize: u32,
    pub dwNameOffset: u32,
}
impl ::core::marker::Copy for LINEQUEUEENTRY {}
impl ::core::clone::Clone for LINEQUEUEENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEQUEUEINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwMeasurementPeriod: u32,
    pub dwTotalCallsQueued: u32,
    pub dwCurrentCallsQueued: u32,
    pub dwTotalCallsAbandoned: u32,
    pub dwTotalCallsFlowedIn: u32,
    pub dwTotalCallsFlowedOut: u32,
    pub dwLongestEverWaitTime: u32,
    pub dwCurrentLongestWaitTime: u32,
    pub dwAverageWaitTime: u32,
    pub dwFinalDisposition: u32,
}
impl ::core::marker::Copy for LINEQUEUEINFO {}
impl ::core::clone::Clone for LINEQUEUEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEQUEUELIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl ::core::marker::Copy for LINEQUEUELIST {}
impl ::core::clone::Clone for LINEQUEUELIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQUEUESTATUS_NEWQUEUE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQUEUESTATUS_QUEUEREMOVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQUEUESTATUS_UPDATEINFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEREMOVEFROMCONF_ANY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEREMOVEFROMCONF_LAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEREMOVEFROMCONF_NONE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LINEREQMAKECALL {
    pub szDestAddress: [super::super::Foundation::CHAR; 80],
    pub szAppName: [super::super::Foundation::CHAR; 40],
    pub szCalledParty: [super::super::Foundation::CHAR; 40],
    pub szComment: [super::super::Foundation::CHAR; 80],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LINEREQMAKECALL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LINEREQMAKECALL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LINEREQMEDIACALL {
    pub hWnd: super::super::Foundation::HWND,
    pub wRequestID: super::super::Foundation::WPARAM,
    pub szDeviceClass: [super::super::Foundation::CHAR; 40],
    pub ucDeviceID: [u8; 40],
    pub dwSize: u32,
    pub dwSecure: u32,
    pub szDestAddress: [super::super::Foundation::CHAR; 80],
    pub szAppName: [super::super::Foundation::CHAR; 40],
    pub szCalledParty: [super::super::Foundation::CHAR; 40],
    pub szComment: [super::super::Foundation::CHAR; 80],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LINEREQMEDIACALL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LINEREQMEDIACALL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEREQUESTMODE_DROP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEREQUESTMODE_MAKECALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEREQUESTMODE_MEDIACALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEROAMMODE_HOME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEROAMMODE_ROAMA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEROAMMODE_ROAMB: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEROAMMODE_UNAVAIL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEROAMMODE_UNKNOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINESPECIALINFO_CUSTIRREG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINESPECIALINFO_NOCIRCUIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINESPECIALINFO_REORDER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINESPECIALINFO_UNAVAIL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINESPECIALINFO_UNKNOWN: u32 = 8u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINETERMCAPS {
    pub dwTermDev: u32,
    pub dwTermModes: u32,
    pub dwTermSharing: u32,
}
impl ::core::marker::Copy for LINETERMCAPS {}
impl ::core::clone::Clone for LINETERMCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMDEV_HEADSET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMDEV_PHONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMDEV_SPEAKER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_BUTTONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_DISPLAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_HOOKSWITCH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_LAMPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_MEDIABIDIRECT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_MEDIAFROMLINE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_MEDIATOLINE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_RINGER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMSHARING_PRIVATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMSHARING_SHAREDCONF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMSHARING_SHAREDEXCL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETOLLLISTOPTION_ADD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETOLLLISTOPTION_REMOVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETONEMODE_BEEP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETONEMODE_BILLING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETONEMODE_BUSY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETONEMODE_CUSTOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETONEMODE_RINGBACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSFERMODE_CONFERENCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSFERMODE_TRANSFER: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINETRANSLATECAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumLocations: u32,
    pub dwLocationListSize: u32,
    pub dwLocationListOffset: u32,
    pub dwCurrentLocationID: u32,
    pub dwNumCards: u32,
    pub dwCardListSize: u32,
    pub dwCardListOffset: u32,
    pub dwCurrentPreferredCardID: u32,
}
impl ::core::marker::Copy for LINETRANSLATECAPS {}
impl ::core::clone::Clone for LINETRANSLATECAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATEOPTION_CANCELCALLWAITING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATEOPTION_CARDOVERRIDE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATEOPTION_FORCELD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATEOPTION_FORCELOCAL: u32 = 4u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINETRANSLATEOUTPUT {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwDialableStringSize: u32,
    pub dwDialableStringOffset: u32,
    pub dwDisplayableStringSize: u32,
    pub dwDisplayableStringOffset: u32,
    pub dwCurrentCountry: u32,
    pub dwDestCountry: u32,
    pub dwTranslateResults: u32,
}
impl ::core::marker::Copy for LINETRANSLATEOUTPUT {}
impl ::core::clone::Clone for LINETRANSLATEOUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_CANONICAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_DIALBILLING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_DIALDIALTONE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_DIALPROMPT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_DIALQUIET: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_INTERNATIONAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_INTOLLLIST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_LOCAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_LONGDISTANCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_NOTINTOLLLIST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_NOTRANSLATION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_VOICEDETECT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETSPIOPTION_NONREENTRANT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_ADDRESSSTATE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_AGENTSESSIONSTATUS: i32 = 27i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_AGENTSPECIFIC: i32 = 21i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_AGENTSTATUS: i32 = 22i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_AGENTSTATUSEX: i32 = 29i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_APPNEWCALL: i32 = 23i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_APPNEWCALLHUB: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_CALLHUBCLOSE: i32 = 33i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_CALLINFO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_CALLSTATE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_CLOSE: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_CREATE: i32 = 19i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_DEVSPECIFIC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_DEVSPECIFICEX: i32 = 34i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_DEVSPECIFICFEATURE: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_GATHERDIGITS: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_GENERATE: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_GROUPSTATUS: i32 = 30i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_LINEDEVSTATE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_MONITORDIGITS: i32 = 9i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_MONITORMEDIA: i32 = 10i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_MONITORTONE: i32 = 11i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_PROXYREQUEST: i32 = 24i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_PROXYSTATUS: i32 = 31i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_QUEUESTATUS: i32 = 28i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_REMOVE: i32 = 25i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_REPLY: i32 = 12i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_REQUEST: i32 = 13i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type LPGETTNEFSTREAMCODEPAGE = ::core::option::Option<unsafe extern "system" fn(lpstream: *mut *mut super::super::System::Com::IStream, lpulcodepage: *mut u32, lpulsubcodepage: *mut u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
pub type LPOPENTNEFSTREAM = ::core::option::Option<unsafe extern "system" fn(lpvsupport: *mut ::core::ffi::c_void, lpstream: *mut *mut super::super::System::Com::IStream, lpszstreamname: *const i8, ulflags: u32, lpmessage: *mut *mut super::super::System::AddressBook::IMessage, wkeyval: u16, lpptnef: *mut *mut *mut ITnef) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
pub type LPOPENTNEFSTREAMEX = ::core::option::Option<unsafe extern "system" fn(lpvsupport: *mut ::core::ffi::c_void, lpstream: *mut *mut super::super::System::Com::IStream, lpszstreamname: *const i8, ulflags: u32, lpmessage: *mut *mut super::super::System::AddressBook::IMessage, wkeyval: u16, lpadressbook: *mut *mut super::super::System::AddressBook::IAddrBook, lpptnef: *mut *mut *mut ITnef) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type MSP_ADDRESS_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ADDRESS_TERMINAL_AVAILABLE: MSP_ADDRESS_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ADDRESS_TERMINAL_UNAVAILABLE: MSP_ADDRESS_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type MSP_CALL_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_NEW_STREAM: MSP_CALL_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_STREAM_FAIL: MSP_CALL_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_TERMINAL_FAIL: MSP_CALL_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_STREAM_NOT_USED: MSP_CALL_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_STREAM_ACTIVE: MSP_CALL_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_STREAM_INACTIVE: MSP_CALL_EVENT = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type MSP_CALL_EVENT_CAUSE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_UNKNOWN: MSP_CALL_EVENT_CAUSE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_BAD_DEVICE: MSP_CALL_EVENT_CAUSE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_CONNECT_FAIL: MSP_CALL_EVENT_CAUSE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_LOCAL_REQUEST: MSP_CALL_EVENT_CAUSE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_REMOTE_REQUEST: MSP_CALL_EVENT_CAUSE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_MEDIA_TIMEOUT: MSP_CALL_EVENT_CAUSE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_MEDIA_RECOVERED: MSP_CALL_EVENT_CAUSE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_QUALITY_OF_SERVICE: MSP_CALL_EVENT_CAUSE = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type MSP_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_ADDRESS_EVENT: MSP_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_CALL_EVENT: MSP_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_TSP_DATA: MSP_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_PRIVATE_EVENT: MSP_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_ASR_TERMINAL_EVENT: MSP_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_TTS_TERMINAL_EVENT: MSP_EVENT = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_FILE_TERMINAL_EVENT: MSP_EVENT = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_TONE_TERMINAL_EVENT: MSP_EVENT = 7i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO {
    pub dwSize: u32,
    pub Event: MSP_EVENT,
    pub hCall: *mut i32,
    pub Anonymous: MSP_EVENT_INFO_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MSP_EVENT_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union MSP_EVENT_INFO_0 {
    pub MSP_ADDRESS_EVENT_INFO: MSP_EVENT_INFO_0_0,
    pub MSP_CALL_EVENT_INFO: MSP_EVENT_INFO_0_2,
    pub MSP_TSP_DATA: MSP_EVENT_INFO_0_6,
    pub MSP_PRIVATE_EVENT_INFO: MSP_EVENT_INFO_0_4,
    pub MSP_FILE_TERMINAL_EVENT_INFO: MSP_EVENT_INFO_0_3,
    pub MSP_ASR_TERMINAL_EVENT_INFO: MSP_EVENT_INFO_0_1,
    pub MSP_TTS_TERMINAL_EVENT_INFO: MSP_EVENT_INFO_0_7,
    pub MSP_TONE_TERMINAL_EVENT_INFO: MSP_EVENT_INFO_0_5,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MSP_EVENT_INFO_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_0 {
    pub Type: MSP_ADDRESS_EVENT,
    pub pTerminal: *mut *mut *mut *mut ITTerminal,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MSP_EVENT_INFO_0_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_1 {
    pub pASRTerminal: *mut *mut *mut *mut ITTerminal,
    pub hrErrorCode: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MSP_EVENT_INFO_0_1 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_2 {
    pub Type: MSP_CALL_EVENT,
    pub Cause: MSP_CALL_EVENT_CAUSE,
    pub pStream: *mut *mut *mut *mut ITStream,
    pub pTerminal: *mut *mut *mut *mut ITTerminal,
    pub hrError: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MSP_EVENT_INFO_0_2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_3 {
    pub pParentFileTerminal: *mut *mut *mut *mut ITTerminal,
    pub pFileTrack: *mut *mut *mut *mut ITFileTrack,
    pub TerminalMediaState: TERMINAL_MEDIA_STATE,
    pub ftecEventCause: FT_STATE_EVENT_CAUSE,
    pub hrErrorCode: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MSP_EVENT_INFO_0_3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_4 {
    pub pEvent: *mut *mut *mut *mut super::super::System::Com::IDispatch,
    pub lEventCode: i32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MSP_EVENT_INFO_0_4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_5 {
    pub pToneTerminal: *mut *mut *mut *mut ITTerminal,
    pub hrErrorCode: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MSP_EVENT_INFO_0_5 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_6 {
    pub dwBufferSize: u32,
    pub pBuffer: [u8; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MSP_EVENT_INFO_0_6 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_7 {
    pub pTTSTerminal: *mut *mut *mut *mut ITTerminal,
    pub hrErrorCode: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MSP_EVENT_INFO_0_7 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const McastAddressAllocation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3742215922, data2: 41609, data3: 4561, data4: [134, 151, 0, 96, 8, 176, 229, 210] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NSID {
    pub dwSize: u32,
    pub uchType: [u8; 16],
    pub xtype: u32,
    pub lTime: i32,
    pub address: NSID_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NSID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NSID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union NSID_0 {
    pub alias: _ADDR_ALIAS,
    pub rgchInterNet: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NSID_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NSID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const OPENTNEFSTREAM: &str = "OpenTnefStream";
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const OPENTNEFSTREAMEX: &str = "OpenTnefStreamEx";
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_ABBREVDIAL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_BRIDGEDAPP: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_BUSY: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_CALLAPP: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_CALLID: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_CAMPON: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_CONFERENCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_CONNECT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_COVER: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DATAOFF: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DATAON: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DATETIME: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DIRECTORY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DISCONNECT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DONOTDISTURB: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DROP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_FLASH: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_FORWARD: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_HOLD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_INTERCOM: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_LASTNUM: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_MSGINDICATOR: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_MSGWAITOFF: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_MSGWAITON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_MUTE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_NIGHTSRV: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_NONE: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_PARK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_PICKUP: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_QUEUECALL: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_RECALL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_REDIRECT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_REJECT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_REPDIAL: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_RINGAGAIN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SAVEREPEAT: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SELECTRING: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SEND: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SENDCALLS: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SETREPDIAL: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SPEAKEROFF: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SPEAKERON: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_STATIONSPEED: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SYSTEMSPEED: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_TRANSFER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_VOLUMEDOWN: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_VOLUMEUP: u32 = 19u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct PHONEBUTTONINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwButtonMode: u32,
    pub dwButtonFunction: u32,
    pub dwButtonTextSize: u32,
    pub dwButtonTextOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwButtonState: u32,
}
impl ::core::marker::Copy for PHONEBUTTONINFO {}
impl ::core::clone::Clone for PHONEBUTTONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONMODE_CALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONMODE_DISPLAY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONMODE_DUMMY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONMODE_FEATURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONMODE_KEYPAD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONMODE_LOCAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONSTATE_DOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONSTATE_UNAVAIL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONSTATE_UNKNOWN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONSTATE_UP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONECALLBACK = ::core::option::Option<unsafe extern "system" fn(hdevice: u32, dwmessage: u32, dwinstance: usize, dwparam1: usize, dwparam2: usize, dwparam3: usize)>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct PHONECAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwProviderInfoSize: u32,
    pub dwProviderInfoOffset: u32,
    pub dwPhoneInfoSize: u32,
    pub dwPhoneInfoOffset: u32,
    pub dwPermanentPhoneID: u32,
    pub dwPhoneNameSize: u32,
    pub dwPhoneNameOffset: u32,
    pub dwStringFormat: u32,
    pub dwPhoneStates: u32,
    pub dwHookSwitchDevs: u32,
    pub dwHandsetHookSwitchModes: u32,
    pub dwSpeakerHookSwitchModes: u32,
    pub dwHeadsetHookSwitchModes: u32,
    pub dwVolumeFlags: u32,
    pub dwGainFlags: u32,
    pub dwDisplayNumRows: u32,
    pub dwDisplayNumColumns: u32,
    pub dwNumRingModes: u32,
    pub dwNumButtonLamps: u32,
    pub dwButtonModesSize: u32,
    pub dwButtonModesOffset: u32,
    pub dwButtonFunctionsSize: u32,
    pub dwButtonFunctionsOffset: u32,
    pub dwLampModesSize: u32,
    pub dwLampModesOffset: u32,
    pub dwNumSetData: u32,
    pub dwSetDataSize: u32,
    pub dwSetDataOffset: u32,
    pub dwNumGetData: u32,
    pub dwGetDataSize: u32,
    pub dwGetDataOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwDeviceClassesSize: u32,
    pub dwDeviceClassesOffset: u32,
    pub dwPhoneFeatures: u32,
    pub dwSettableHandsetHookSwitchModes: u32,
    pub dwSettableSpeakerHookSwitchModes: u32,
    pub dwSettableHeadsetHookSwitchModes: u32,
    pub dwMonitoredHandsetHookSwitchModes: u32,
    pub dwMonitoredSpeakerHookSwitchModes: u32,
    pub dwMonitoredHeadsetHookSwitchModes: u32,
    pub PermanentPhoneGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for PHONECAPS {}
impl ::core::clone::Clone for PHONECAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONECAPS_BUFFER = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCB_DEVSPECIFICBUFFER: PHONECAPS_BUFFER = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONECAPS_LONG = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_HOOKSWITCHES: PHONECAPS_LONG = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_HANDSETHOOKSWITCHMODES: PHONECAPS_LONG = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_HEADSETHOOKSWITCHMODES: PHONECAPS_LONG = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_SPEAKERPHONEHOOKSWITCHMODES: PHONECAPS_LONG = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_DISPLAYNUMROWS: PHONECAPS_LONG = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_DISPLAYNUMCOLUMNS: PHONECAPS_LONG = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_NUMRINGMODES: PHONECAPS_LONG = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_NUMBUTTONLAMPS: PHONECAPS_LONG = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_GENERICPHONE: PHONECAPS_LONG = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONECAPS_STRING = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCS_PHONENAME: PHONECAPS_STRING = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCS_PHONEINFO: PHONECAPS_STRING = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCS_PROVIDERINFO: PHONECAPS_STRING = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_ALLOCATED: u32 = 2415919105u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_BADDEVICEID: u32 = 2415919106u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_DISCONNECTED: u32 = 2415919140u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INCOMPATIBLEAPIVERSION: u32 = 2415919107u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INCOMPATIBLEEXTVERSION: u32 = 2415919108u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INIFILECORRUPT: u32 = 2415919109u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INUSE: u32 = 2415919110u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALAPPHANDLE: u32 = 2415919111u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALAPPNAME: u32 = 2415919112u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALBUTTONLAMPID: u32 = 2415919113u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALBUTTONMODE: u32 = 2415919114u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALBUTTONSTATE: u32 = 2415919115u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALDATAID: u32 = 2415919116u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALDEVICECLASS: u32 = 2415919117u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALEXTVERSION: u32 = 2415919118u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALHOOKSWITCHDEV: u32 = 2415919119u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALHOOKSWITCHMODE: u32 = 2415919120u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALLAMPMODE: u32 = 2415919121u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALPARAM: u32 = 2415919122u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALPHONEHANDLE: u32 = 2415919123u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALPHONESTATE: u32 = 2415919124u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALPOINTER: u32 = 2415919125u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALPRIVILEGE: u32 = 2415919126u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALRINGMODE: u32 = 2415919127u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_NODEVICE: u32 = 2415919128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_NODRIVER: u32 = 2415919129u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_NOMEM: u32 = 2415919130u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_NOTOWNER: u32 = 2415919131u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_OPERATIONFAILED: u32 = 2415919132u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_OPERATIONUNAVAIL: u32 = 2415919133u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_REINIT: u32 = 2415919139u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_REQUESTOVERRUN: u32 = 2415919136u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_RESOURCEUNAVAIL: u32 = 2415919135u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_SERVICE_NOT_RUNNING: u32 = 2415919141u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_STRUCTURETOOSMALL: u32 = 2415919137u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_UNINITIALIZED: u32 = 2415919138u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONEEVENT = ::core::option::Option<unsafe extern "system" fn(htphone: *mut HTAPIPHONE__, dwmsg: u32, dwparam1: usize, dwparam2: usize, dwparam3: usize)>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct PHONEEXTENSIONID {
    pub dwExtensionID0: u32,
    pub dwExtensionID1: u32,
    pub dwExtensionID2: u32,
    pub dwExtensionID3: u32,
}
impl ::core::marker::Copy for PHONEEXTENSIONID {}
impl ::core::clone::Clone for PHONEEXTENSIONID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GENERICPHONE: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETBUTTONINFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETDATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETDISPLAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETGAINHANDSET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETGAINHEADSET: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETGAINSPEAKER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETHOOKSWITCHHANDSET: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETHOOKSWITCHHEADSET: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETHOOKSWITCHSPEAKER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETLAMP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETRING: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETVOLUMEHANDSET: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETVOLUMEHEADSET: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETVOLUMESPEAKER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETBUTTONINFO: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETDATA: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETDISPLAY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETGAINHANDSET: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETGAINHEADSET: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETGAINSPEAKER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETHOOKSWITCHHANDSET: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETHOOKSWITCHHEADSET: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETHOOKSWITCHSPEAKER: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETLAMP: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETRING: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETVOLUMEHANDSET: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETVOLUMEHEADSET: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETVOLUMESPEAKER: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHDEV_HANDSET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHDEV_HEADSET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHDEV_SPEAKER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHMODE_MIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHMODE_MICSPEAKER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHMODE_ONHOOK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHMODE_SPEAKER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHMODE_UNKNOWN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEINITIALIZEEXOPTION_USECOMPLETIONPORT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEINITIALIZEEXOPTION_USEEVENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEINITIALIZEEXOPTION_USEHIDDENWINDOW: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PHONEINITIALIZEEXPARAMS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwOptions: u32,
    pub Handles: PHONEINITIALIZEEXPARAMS_0,
    pub dwCompletionKey: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PHONEINITIALIZEEXPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PHONEINITIALIZEEXPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PHONEINITIALIZEEXPARAMS_0 {
    pub hEvent: super::super::Foundation::HANDLE,
    pub hCompletionPort: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PHONEINITIALIZEEXPARAMS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PHONEINITIALIZEEXPARAMS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_BROKENFLUTTER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_DUMMY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_FLASH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_FLUTTER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_OFF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_STEADY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_UNKNOWN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_WINK: u32 = 8u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct PHONEMESSAGE {
    pub hDevice: u32,
    pub dwMessageID: u32,
    pub dwCallbackInstance: usize,
    pub dwParam1: usize,
    pub dwParam2: usize,
    pub dwParam3: usize,
}
impl ::core::marker::Copy for PHONEMESSAGE {}
impl ::core::clone::Clone for PHONEMESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEPRIVILEGE_MONITOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEPRIVILEGE_OWNER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_CAPSCHANGE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_CONNECTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_DEVSPECIFIC: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_DISCONNECTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_DISPLAY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_HANDSETGAIN: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_HANDSETHOOKSWITCH: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_HANDSETVOLUME: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_HEADSETGAIN: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_HEADSETHOOKSWITCH: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_HEADSETVOLUME: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_LAMP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_MONITORS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_OTHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_OWNER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_REINIT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_REMOVED: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_RESUME: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_RINGMODE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_RINGVOLUME: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_SPEAKERGAIN: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_SPEAKERHOOKSWITCH: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_SPEAKERVOLUME: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_SUSPEND: u32 = 262144u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct PHONESTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwStatusFlags: u32,
    pub dwNumOwners: u32,
    pub dwNumMonitors: u32,
    pub dwRingMode: u32,
    pub dwRingVolume: u32,
    pub dwHandsetHookSwitchMode: u32,
    pub dwHandsetVolume: u32,
    pub dwHandsetGain: u32,
    pub dwSpeakerHookSwitchMode: u32,
    pub dwSpeakerVolume: u32,
    pub dwSpeakerGain: u32,
    pub dwHeadsetHookSwitchMode: u32,
    pub dwHeadsetVolume: u32,
    pub dwHeadsetGain: u32,
    pub dwDisplaySize: u32,
    pub dwDisplayOffset: u32,
    pub dwLampModesSize: u32,
    pub dwLampModesOffset: u32,
    pub dwOwnerNameSize: u32,
    pub dwOwnerNameOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwPhoneFeatures: u32,
}
impl ::core::marker::Copy for PHONESTATUS {}
impl ::core::clone::Clone for PHONESTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATUSFLAGS_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATUSFLAGS_SUSPENDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_BUTTON: i32 = 14i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONE_BUTTON_FUNCTION = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_UNKNOWN: PHONE_BUTTON_FUNCTION = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_CONFERENCE: PHONE_BUTTON_FUNCTION = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_TRANSFER: PHONE_BUTTON_FUNCTION = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DROP: PHONE_BUTTON_FUNCTION = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_HOLD: PHONE_BUTTON_FUNCTION = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_RECALL: PHONE_BUTTON_FUNCTION = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DISCONNECT: PHONE_BUTTON_FUNCTION = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_CONNECT: PHONE_BUTTON_FUNCTION = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_MSGWAITON: PHONE_BUTTON_FUNCTION = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_MSGWAITOFF: PHONE_BUTTON_FUNCTION = 9i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SELECTRING: PHONE_BUTTON_FUNCTION = 10i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_ABBREVDIAL: PHONE_BUTTON_FUNCTION = 11i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_FORWARD: PHONE_BUTTON_FUNCTION = 12i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_PICKUP: PHONE_BUTTON_FUNCTION = 13i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_RINGAGAIN: PHONE_BUTTON_FUNCTION = 14i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_PARK: PHONE_BUTTON_FUNCTION = 15i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_REJECT: PHONE_BUTTON_FUNCTION = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_REDIRECT: PHONE_BUTTON_FUNCTION = 17i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_MUTE: PHONE_BUTTON_FUNCTION = 18i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_VOLUMEUP: PHONE_BUTTON_FUNCTION = 19i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_VOLUMEDOWN: PHONE_BUTTON_FUNCTION = 20i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SPEAKERON: PHONE_BUTTON_FUNCTION = 21i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SPEAKEROFF: PHONE_BUTTON_FUNCTION = 22i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_FLASH: PHONE_BUTTON_FUNCTION = 23i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DATAON: PHONE_BUTTON_FUNCTION = 24i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DATAOFF: PHONE_BUTTON_FUNCTION = 25i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DONOTDISTURB: PHONE_BUTTON_FUNCTION = 26i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_INTERCOM: PHONE_BUTTON_FUNCTION = 27i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_BRIDGEDAPP: PHONE_BUTTON_FUNCTION = 28i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_BUSY: PHONE_BUTTON_FUNCTION = 29i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_CALLAPP: PHONE_BUTTON_FUNCTION = 30i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DATETIME: PHONE_BUTTON_FUNCTION = 31i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DIRECTORY: PHONE_BUTTON_FUNCTION = 32i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_COVER: PHONE_BUTTON_FUNCTION = 33i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_CALLID: PHONE_BUTTON_FUNCTION = 34i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_LASTNUM: PHONE_BUTTON_FUNCTION = 35i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_NIGHTSRV: PHONE_BUTTON_FUNCTION = 36i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SENDCALLS: PHONE_BUTTON_FUNCTION = 37i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_MSGINDICATOR: PHONE_BUTTON_FUNCTION = 38i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_REPDIAL: PHONE_BUTTON_FUNCTION = 39i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SETREPDIAL: PHONE_BUTTON_FUNCTION = 40i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SYSTEMSPEED: PHONE_BUTTON_FUNCTION = 41i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_STATIONSPEED: PHONE_BUTTON_FUNCTION = 42i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_CAMPON: PHONE_BUTTON_FUNCTION = 43i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SAVEREPEAT: PHONE_BUTTON_FUNCTION = 44i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_QUEUECALL: PHONE_BUTTON_FUNCTION = 45i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_NONE: PHONE_BUTTON_FUNCTION = 46i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SEND: PHONE_BUTTON_FUNCTION = 47i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONE_BUTTON_MODE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBM_DUMMY: PHONE_BUTTON_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBM_CALL: PHONE_BUTTON_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBM_FEATURE: PHONE_BUTTON_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBM_KEYPAD: PHONE_BUTTON_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBM_LOCAL: PHONE_BUTTON_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBM_DISPLAY: PHONE_BUTTON_MODE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONE_BUTTON_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBS_UP: PHONE_BUTTON_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBS_DOWN: PHONE_BUTTON_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBS_UNKNOWN: PHONE_BUTTON_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBS_UNAVAIL: PHONE_BUTTON_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_CLOSE: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_CREATE: i32 = 20i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_DEVSPECIFIC: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONE_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_DISPLAY: PHONE_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_LAMPMODE: PHONE_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_RINGMODE: PHONE_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_RINGVOLUME: PHONE_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_HOOKSWITCH: PHONE_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_CAPSCHANGE: PHONE_EVENT = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_BUTTON: PHONE_EVENT = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_CLOSE: PHONE_EVENT = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_NUMBERGATHERED: PHONE_EVENT = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_DIALING: PHONE_EVENT = 9i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_ANSWER: PHONE_EVENT = 10i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_DISCONNECT: PHONE_EVENT = 11i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_LASTITEM: PHONE_EVENT = 11i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONE_HOOK_SWITCH_DEVICE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSD_HANDSET: PHONE_HOOK_SWITCH_DEVICE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSD_SPEAKERPHONE: PHONE_HOOK_SWITCH_DEVICE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSD_HEADSET: PHONE_HOOK_SWITCH_DEVICE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONE_HOOK_SWITCH_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSS_ONHOOK: PHONE_HOOK_SWITCH_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSS_OFFHOOK_MIC_ONLY: PHONE_HOOK_SWITCH_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSS_OFFHOOK_SPEAKER_ONLY: PHONE_HOOK_SWITCH_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSS_OFFHOOK: PHONE_HOOK_SWITCH_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONE_LAMP_MODE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_DUMMY: PHONE_LAMP_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_OFF: PHONE_LAMP_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_STEADY: PHONE_LAMP_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_WINK: PHONE_LAMP_MODE = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_FLASH: PHONE_LAMP_MODE = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_FLUTTER: PHONE_LAMP_MODE = 32i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_BROKENFLUTTER: PHONE_LAMP_MODE = 64i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_UNKNOWN: PHONE_LAMP_MODE = 128i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONE_PRIVILEGE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PP_OWNER: PHONE_PRIVILEGE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PP_MONITOR: PHONE_PRIVILEGE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_REMOVE: i32 = 26i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_REPLY: i32 = 17i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_STATE: i32 = 18i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONE_TONE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADZERO: PHONE_TONE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADONE: PHONE_TONE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADTWO: PHONE_TONE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADTHREE: PHONE_TONE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADFOUR: PHONE_TONE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADFIVE: PHONE_TONE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADSIX: PHONE_TONE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADSEVEN: PHONE_TONE = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADEIGHT: PHONE_TONE = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADNINE: PHONE_TONE = 9i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADSTAR: PHONE_TONE = 10i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADPOUND: PHONE_TONE = 11i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADA: PHONE_TONE = 12i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADB: PHONE_TONE = 13i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADC: PHONE_TONE = 14i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADD: PHONE_TONE = 15i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_NORMALDIALTONE: PHONE_TONE = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_EXTERNALDIALTONE: PHONE_TONE = 17i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_BUSY: PHONE_TONE = 18i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_RINGBACK: PHONE_TONE = 19i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_ERRORTONE: PHONE_TONE = 20i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_SILENCE: PHONE_TONE = 21i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PRIVATEOBJECT_ADDRESS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PRIVATEOBJECT_CALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PRIVATEOBJECT_CALLID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PRIVATEOBJECT_LINE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PRIVATEOBJECT_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PRIVATEOBJECT_PHONE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type QOS_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QE_NOQOS: QOS_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QE_ADMISSIONFAILURE: QOS_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QE_POLICYFAILURE: QOS_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QE_GENERICERROR: QOS_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QE_LASTITEM: QOS_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type QOS_SERVICE_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QSL_NEEDED: QOS_SERVICE_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QSL_IF_AVAILABLE: QOS_SERVICE_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QSL_BEST_EFFORT: QOS_SERVICE_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RENDBIND_AUTHENTICATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RENDBIND_DEFAULTCREDENTIALS: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RENDBIND_DEFAULTDOMAINNAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RENDBIND_DEFAULTPASSWORD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RENDBIND_DEFAULTUSERNAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type RND_ADVERTISING_SCOPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RAS_LOCAL: RND_ADVERTISING_SCOPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RAS_SITE: RND_ADVERTISING_SCOPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RAS_REGION: RND_ADVERTISING_SCOPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RAS_WORLD: RND_ADVERTISING_SCOPE = 4i32;
pub const Rendezvous: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4043480667, data2: 52059, data3: 4560, data4: [141, 89, 0, 192, 79, 217, 26, 192] };
pub const RequestMakeCall: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2890465248, data2: 63684, data3: 4561, data4: [160, 48, 0, 192, 79, 182, 128, 159] };
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRINGFORMAT_ASCII: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRINGFORMAT_BINARY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRINGFORMAT_DBCS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRINGFORMAT_UNICODE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRM_CONFIGURED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRM_INITIAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRM_PAUSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRM_RUNNING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRM_STOPPED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRM_TERMINALSELECTED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct STnefProblem {
    pub ulComponent: u32,
    pub ulAttribute: u32,
    pub ulPropTag: u32,
    pub scode: i32,
}
impl ::core::marker::Copy for STnefProblem {}
impl ::core::clone::Clone for STnefProblem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct STnefProblemArray {
    pub cProblem: u32,
    pub aProblem: [STnefProblem; 1],
}
impl ::core::marker::Copy for STnefProblemArray {}
impl ::core::clone::Clone for STnefProblemArray {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TAPI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 567727246, data2: 43147, data3: 4560, data4: [131, 221, 0, 170, 0, 60, 202, 189] };
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_CONNECTED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DESTBUSY: i32 = -11i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DESTNOANSWER: i32 = -12i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DESTUNAVAIL: i32 = -13i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DEVICECLASSUNAVAIL: i32 = -8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DEVICEIDUNAVAIL: i32 = -9i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DEVICEINUSE: i32 = -10i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DROPPED: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_INVALDESTADDRESS: i32 = -4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_INVALDEVICECLASS: i32 = -6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_INVALDEVICEID: i32 = -7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_INVALPOINTER: i32 = -18i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_INVALWINDOWHANDLE: i32 = -5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_MMCWRITELOCKED: i32 = -20i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_NOREQUESTRECIPIENT: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_NOTADMIN: i32 = -19i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_PROVIDERALREADYINSTALLED: i32 = -21i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_REQUESTCANCELLED: i32 = -17i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_REQUESTFAILED: i32 = -16i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_REQUESTQUEUEFULL: i32 = -3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_SCP_ALREADY_EXISTS: i32 = -22i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_SCP_DOES_NOT_EXIST: i32 = -23i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_UNKNOWNREQUESTID: i32 = -15i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_UNKNOWNWINHANDLE: i32 = -14i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMAXAPPNAMESIZE: i32 = 40i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMAXCALLEDPARTYSIZE: i32 = 40i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMAXCOMMENTSIZE: i32 = 80i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMAXDESTADDRESSSIZE: i32 = 80i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMAXDEVICECLASSSIZE: i32 = 40i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMAXDEVICEIDSIZE: i32 = 40i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMEDIATYPE_AUDIO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMEDIATYPE_DATAMODEM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMEDIATYPE_G3FAX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMEDIATYPE_MULTITRACK: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMEDIATYPE_VIDEO: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type TAPIOBJECT_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ADDRESSCREATE: TAPIOBJECT_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ADDRESSREMOVE: TAPIOBJECT_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_REINIT: TAPIOBJECT_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_TRANSLATECHANGE: TAPIOBJECT_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ADDRESSCLOSE: TAPIOBJECT_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_PHONECREATE: TAPIOBJECT_EVENT = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_PHONEREMOVE: TAPIOBJECT_EVENT = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_CURRENT_VERSION: u32 = 131074u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct TAPI_CUSTOMTONE {
    pub dwFrequency: u32,
    pub dwCadenceOn: u32,
    pub dwCadenceOff: u32,
    pub dwVolume: u32,
}
impl ::core::marker::Copy for TAPI_CUSTOMTONE {}
impl ::core::clone::Clone for TAPI_CUSTOMTONE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct TAPI_DETECTTONE {
    pub dwAppSpecific: u32,
    pub dwDuration: u32,
    pub dwFrequency1: u32,
    pub dwFrequency2: u32,
    pub dwFrequency3: u32,
}
impl ::core::marker::Copy for TAPI_DETECTTONE {}
impl ::core::clone::Clone for TAPI_DETECTTONE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type TAPI_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_TAPIOBJECT: TAPI_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ADDRESS: TAPI_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_CALLNOTIFICATION: TAPI_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_CALLSTATE: TAPI_EVENT = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_CALLMEDIA: TAPI_EVENT = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_CALLHUB: TAPI_EVENT = 32i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_CALLINFOCHANGE: TAPI_EVENT = 64i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_PRIVATE: TAPI_EVENT = 128i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_REQUEST: TAPI_EVENT = 256i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_AGENT: TAPI_EVENT = 512i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_AGENTSESSION: TAPI_EVENT = 1024i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_QOSEVENT: TAPI_EVENT = 2048i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_AGENTHANDLER: TAPI_EVENT = 4096i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ACDGROUP: TAPI_EVENT = 8192i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_QUEUE: TAPI_EVENT = 16384i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_DIGITEVENT: TAPI_EVENT = 32768i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_GENERATEEVENT: TAPI_EVENT = 65536i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ASRTERMINAL: TAPI_EVENT = 131072i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_TTSTERMINAL: TAPI_EVENT = 262144i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_FILETERMINAL: TAPI_EVENT = 524288i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_TONETERMINAL: TAPI_EVENT = 1048576i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_PHONEEVENT: TAPI_EVENT = 2097152i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_TONEEVENT: TAPI_EVENT = 4194304i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_GATHERDIGITS: TAPI_EVENT = 8388608i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ADDRESSDEVSPECIFIC: TAPI_EVENT = 16777216i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_PHONEDEVSPECIFIC: TAPI_EVENT = 33554432i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_ADDRESSBLOCKED: ::windows_sys::core::HRESULT = -2147221462i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_ALLOCATED: ::windows_sys::core::HRESULT = -2147221498i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_BILLINGREJECTED: ::windows_sys::core::HRESULT = -2147221461i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_GROUP_REMOVED: ::windows_sys::core::HRESULT = -2147221435i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_INVALAGENTACTIVITY: ::windows_sys::core::HRESULT = -2147221428i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_INVALAGENTGROUP: ::windows_sys::core::HRESULT = -2147221431i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_INVALAGENTID: ::windows_sys::core::HRESULT = -2147221432i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_INVALAGENTSTATE: ::windows_sys::core::HRESULT = -2147221429i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_INVALPASSWORD: ::windows_sys::core::HRESULT = -2147221430i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_NO_AGENT_ID: ::windows_sys::core::HRESULT = -2147221433i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_QUEUE_REMOVED: ::windows_sys::core::HRESULT = -2147221434i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLNOTSELECTED: ::windows_sys::core::HRESULT = -2147221420i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLUNAVAIL: ::windows_sys::core::HRESULT = -2147221497i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_COMPLETIONOVERRUN: ::windows_sys::core::HRESULT = -2147221496i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CONFERENCEFULL: ::windows_sys::core::HRESULT = -2147221495i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_DESTBUSY: ::windows_sys::core::HRESULT = -2147221452i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_DESTNOANSWER: ::windows_sys::core::HRESULT = -2147221451i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_DESTUNAVAIL: ::windows_sys::core::HRESULT = -2147221450i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_DIALMODIFIERNOTSUPPORTED: ::windows_sys::core::HRESULT = -2147221494i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_DROPPED: ::windows_sys::core::HRESULT = -2147221455i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INUSE: ::windows_sys::core::HRESULT = -2147221493i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALADDRESS: ::windows_sys::core::HRESULT = -2147221492i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALADDRESSSTATE: ::windows_sys::core::HRESULT = -2147221491i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALADDRESSTYPE: ::windows_sys::core::HRESULT = -2147221423i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALBUTTONLAMPID: ::windows_sys::core::HRESULT = -2147221459i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALBUTTONSTATE: ::windows_sys::core::HRESULT = -2147221458i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALCALLPARAMS: ::windows_sys::core::HRESULT = -2147221490i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALCALLPRIVILEGE: ::windows_sys::core::HRESULT = -2147221489i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALCALLSTATE: ::windows_sys::core::HRESULT = -2147221488i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALCARD: ::windows_sys::core::HRESULT = -2147221487i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALCOMPLETIONID: ::windows_sys::core::HRESULT = -2147221486i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALCOUNTRYCODE: ::windows_sys::core::HRESULT = -2147221485i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALDATAID: ::windows_sys::core::HRESULT = -2147221457i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALDEVICECLASS: ::windows_sys::core::HRESULT = -2147221484i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALDIALPARAMS: ::windows_sys::core::HRESULT = -2147221483i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALDIGITS: ::windows_sys::core::HRESULT = -2147221482i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALFEATURE: ::windows_sys::core::HRESULT = -2147221460i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALGROUPID: ::windows_sys::core::HRESULT = -2147221481i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALHOOKSWITCHDEV: ::windows_sys::core::HRESULT = -2147221456i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALIDDIRECTION: ::windows_sys::core::HRESULT = -2147221446i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALIDMEDIATYPE: ::windows_sys::core::HRESULT = -2147221500i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALIDSTREAM: ::windows_sys::core::HRESULT = -2147221437i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALIDSTREAMSTATE: ::windows_sys::core::HRESULT = -2147221417i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALIDTERMINAL: ::windows_sys::core::HRESULT = -2147221445i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALIDTERMINALCLASS: ::windows_sys::core::HRESULT = -2147221444i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALLIST: ::windows_sys::core::HRESULT = -2147221474i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALLOCATION: ::windows_sys::core::HRESULT = -2147221480i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALMESSAGEID: ::windows_sys::core::HRESULT = -2147221479i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALMODE: ::windows_sys::core::HRESULT = -2147221473i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALPARKID: ::windows_sys::core::HRESULT = -2147221478i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALPRIVILEGE: ::windows_sys::core::HRESULT = -2147221447i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALRATE: ::windows_sys::core::HRESULT = -2147221477i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALTIMEOUT: ::windows_sys::core::HRESULT = -2147221476i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALTONE: ::windows_sys::core::HRESULT = -2147221475i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_MAXSTREAMS: ::windows_sys::core::HRESULT = -2147221442i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_MAXTERMINALS: ::windows_sys::core::HRESULT = -2147221438i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOCONFERENCE: ::windows_sys::core::HRESULT = -2147221472i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NODEVICE: ::windows_sys::core::HRESULT = -2147221471i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NODRIVER: ::windows_sys::core::HRESULT = -2147221443i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOEVENT: ::windows_sys::core::HRESULT = -2147221424i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOFORMAT: ::windows_sys::core::HRESULT = -2147221418i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOITEMS: ::windows_sys::core::HRESULT = -2147221502i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOREQUEST: ::windows_sys::core::HRESULT = -2147221470i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOREQUESTRECIPIENT: ::windows_sys::core::HRESULT = -2147221454i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOTENOUGHMEMORY: ::windows_sys::core::HRESULT = -2147221503i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOTERMINALSELECTED: ::windows_sys::core::HRESULT = -2147221441i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOTOWNER: ::windows_sys::core::HRESULT = -2147221469i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOTREGISTERED: ::windows_sys::core::HRESULT = -2147221468i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOTSTOPPED: ::windows_sys::core::HRESULT = -2147221439i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOTSUPPORTED: ::windows_sys::core::HRESULT = -2147221501i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2147221415i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_OPERATIONFAILED: ::windows_sys::core::HRESULT = -2147221499i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_PEER_NOT_SET: ::windows_sys::core::HRESULT = -2147221425i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_PHONENOTOPEN: ::windows_sys::core::HRESULT = -2147221421i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_REGISTRY_SETTING_CORRUPT: ::windows_sys::core::HRESULT = -2147221427i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_REINIT: ::windows_sys::core::HRESULT = -2147221463i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_REQUESTCANCELLED: ::windows_sys::core::HRESULT = -2147221448i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_REQUESTFAILED: ::windows_sys::core::HRESULT = -2147221449i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_REQUESTOVERRUN: ::windows_sys::core::HRESULT = -2147221467i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_REQUESTQUEUEFULL: ::windows_sys::core::HRESULT = -2147221453i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_RESOURCEUNAVAIL: ::windows_sys::core::HRESULT = -2147221422i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_SERVICE_NOT_RUNNING: ::windows_sys::core::HRESULT = -2147221414i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_TARGETNOTFOUND: ::windows_sys::core::HRESULT = -2147221466i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_TARGETSELF: ::windows_sys::core::HRESULT = -2147221465i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_TERMINALINUSE: ::windows_sys::core::HRESULT = -2147221440i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_TERMINAL_PEER: ::windows_sys::core::HRESULT = -2147221426i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_TIMEOUT: ::windows_sys::core::HRESULT = -2147221436i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_USERUSERINFOTOOBIG: ::windows_sys::core::HRESULT = -2147221464i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_WRONGEVENT: ::windows_sys::core::HRESULT = -2147221419i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_WRONG_STATE: ::windows_sys::core::HRESULT = -2147221416i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type TAPI_GATHERTERM = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TGT_BUFFERFULL: TAPI_GATHERTERM = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TGT_TERMDIGIT: TAPI_GATHERTERM = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TGT_FIRSTTIMEOUT: TAPI_GATHERTERM = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TGT_INTERTIMEOUT: TAPI_GATHERTERM = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TGT_CANCEL: TAPI_GATHERTERM = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type TAPI_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_NONE: TAPI_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_TAPI: TAPI_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_ADDRESS: TAPI_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_TERMINAL: TAPI_OBJECT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_CALL: TAPI_OBJECT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_CALLHUB: TAPI_OBJECT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_PHONE: TAPI_OBJECT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_REPLY: u32 = 1123u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type TAPI_TONEMODE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TTM_RINGBACK: TAPI_TONEMODE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TTM_BUSY: TAPI_TONEMODE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TTM_BEEP: TAPI_TONEMODE = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TTM_BILLING: TAPI_TONEMODE = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type TERMINAL_DIRECTION = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TD_CAPTURE: TERMINAL_DIRECTION = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TD_RENDER: TERMINAL_DIRECTION = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TD_BIDIRECTIONAL: TERMINAL_DIRECTION = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TD_MULTITRACK_MIXED: TERMINAL_DIRECTION = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TD_NONE: TERMINAL_DIRECTION = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type TERMINAL_MEDIA_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TMS_IDLE: TERMINAL_MEDIA_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TMS_ACTIVE: TERMINAL_MEDIA_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TMS_PAUSED: TERMINAL_MEDIA_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TMS_LASTITEM: TERMINAL_MEDIA_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type TERMINAL_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TS_INUSE: TERMINAL_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TS_NOTINUSE: TERMINAL_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type TERMINAL_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TT_STATIC: TERMINAL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TT_DYNAMIC: TERMINAL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEACCEPT: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEADDTOCONFERENCE: u32 = 501u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEANSWER: u32 = 502u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEBLINDTRANSFER: u32 = 503u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECLOSE: u32 = 504u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECLOSECALL: u32 = 505u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECLOSEMSPINSTANCE: u32 = 609u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECOMPLETECALL: u32 = 506u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECOMPLETETRANSFER: u32 = 507u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECONDITIONALMEDIADETECTION: u32 = 508u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECONFIGDIALOG: u32 = 509u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECONFIGDIALOGEDIT: u32 = 601u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECREATEMSPINSTANCE: u32 = 608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEDEVSPECIFIC: u32 = 510u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEDEVSPECIFICFEATURE: u32 = 511u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEDIAL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEDROP: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEDROPNOOWNER: u32 = 597u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEDROPONCLOSE: u32 = 596u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEFORWARD: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGATHERDIGITS: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGENERATEDIGITS: u32 = 516u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGENERATETONE: u32 = 517u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETADDRESSCAPS: u32 = 518u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETADDRESSID: u32 = 519u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETADDRESSSTATUS: u32 = 520u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETCALLADDRESSID: u32 = 521u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETCALLHUBTRACKING: u32 = 604u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETCALLID: u32 = 603u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETCALLINFO: u32 = 522u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETCALLSTATUS: u32 = 523u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETDEVCAPS: u32 = 524u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETDEVCONFIG: u32 = 525u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETEXTENSIONID: u32 = 526u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETICON: u32 = 527u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETID: u32 = 528u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETLINEDEVSTATUS: u32 = 529u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETNUMADDRESSIDS: u32 = 530u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEHOLD: u32 = 531u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEMAKECALL: u32 = 532u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEMONITORDIGITS: u32 = 533u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEMONITORMEDIA: u32 = 534u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEMONITORTONES: u32 = 535u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEMSPIDENTIFY: u32 = 607u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINENEGOTIATEEXTVERSION: u32 = 536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINENEGOTIATETSPIVERSION: u32 = 537u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEOPEN: u32 = 538u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEPARK: u32 = 539u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEPICKUP: u32 = 540u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEPREPAREADDTOCONFERENCE: u32 = 541u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINERECEIVEMSPDATA: u32 = 606u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEREDIRECT: u32 = 542u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINERELEASEUSERUSERINFO: u32 = 602u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEREMOVEFROMCONFERENCE: u32 = 543u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESECURECALL: u32 = 544u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESELECTEXTVERSION: u32 = 545u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESENDUSERUSERINFO: u32 = 546u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETAPPSPECIFIC: u32 = 547u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETCALLHUBTRACKING: u32 = 605u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETCALLPARAMS: u32 = 548u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETCURRENTLOCATION: u32 = 600u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETDEFAULTMEDIADETECTION: u32 = 549u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETDEVCONFIG: u32 = 550u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETMEDIACONTROL: u32 = 551u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETMEDIAMODE: u32 = 552u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETSTATUSMESSAGES: u32 = 553u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETTERMINAL: u32 = 554u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETUPCONFERENCE: u32 = 555u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETUPTRANSFER: u32 = 556u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESWAPHOLD: u32 = 557u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEUNCOMPLETECALL: u32 = 558u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEUNHOLD: u32 = 559u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEUNPARK: u32 = 560u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_MESSAGE_BASE: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONECLOSE: u32 = 561u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONECONFIGDIALOG: u32 = 562u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEDEVSPECIFIC: u32 = 563u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETBUTTONINFO: u32 = 564u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETDATA: u32 = 565u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETDEVCAPS: u32 = 566u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETDISPLAY: u32 = 567u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETEXTENSIONID: u32 = 568u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETGAIN: u32 = 569u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETHOOKSWITCH: u32 = 570u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETICON: u32 = 571u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETID: u32 = 572u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETLAMP: u32 = 573u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETRING: u32 = 574u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETSTATUS: u32 = 575u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETVOLUME: u32 = 576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONENEGOTIATEEXTVERSION: u32 = 577u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONENEGOTIATETSPIVERSION: u32 = 578u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEOPEN: u32 = 579u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESELECTEXTVERSION: u32 = 580u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETBUTTONINFO: u32 = 581u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETDATA: u32 = 582u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETDISPLAY: u32 = 583u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETGAIN: u32 = 584u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETHOOKSWITCH: u32 = 585u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETLAMP: u32 = 586u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETRING: u32 = 587u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETSTATUSMESSAGES: u32 = 588u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETVOLUME: u32 = 589u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROC_BASE: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERCONFIG: u32 = 590u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERCREATELINEDEVICE: u32 = 598u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERCREATEPHONEDEVICE: u32 = 599u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERENUMDEVICES: u32 = 595u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERINIT: u32 = 591u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERINSTALL: u32 = 592u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERREMOVE: u32 = 593u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERSHUTDOWN: u32 = 594u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct TUISPICREATEDIALOGINSTANCEPARAMS {
    pub dwRequestID: u32,
    pub hdDlgInst: *mut HDRVDIALOGINSTANCE__,
    pub htDlgInst: u32,
    pub lpszUIDLLName: ::windows_sys::core::PCWSTR,
    pub lpParams: *mut ::core::ffi::c_void,
    pub dwSize: u32,
}
impl ::core::marker::Copy for TUISPICREATEDIALOGINSTANCEPARAMS {}
impl ::core::clone::Clone for TUISPICREATEDIALOGINSTANCEPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type TUISPIDLLCALLBACK = ::core::option::Option<unsafe extern "system" fn(dwobjectid: usize, dwobjecttype: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TUISPIDLL_OBJECT_DIALOGINSTANCE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TUISPIDLL_OBJECT_LINEID: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TUISPIDLL_OBJECT_PHONEID: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TUISPIDLL_OBJECT_PROVIDERID: i32 = 3i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct VARSTRING {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwStringFormat: u32,
    pub dwStringSize: u32,
    pub dwStringOffset: u32,
}
impl ::core::marker::Copy for VARSTRING {}
impl ::core::clone::Clone for VARSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct _ADDR_ALIAS {
    pub rgchName: [super::super::Foundation::CHAR; 41],
    pub rgchEName: [super::super::Foundation::CHAR; 11],
    pub rgchSrvr: [super::super::Foundation::CHAR; 12],
    pub dibDetail: u32,
    pub r#type: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _ADDR_ALIAS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _ADDR_ALIAS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct _dtr {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wDayOfWeek: u16,
}
impl ::core::marker::Copy for _dtr {}
impl ::core::clone::Clone for _dtr {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct _renddata {
    pub atyp: u16,
    pub ulPosition: u32,
    pub dxWidth: u16,
    pub dyHeight: u16,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for _renddata {}
impl ::core::clone::Clone for _renddata {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct _trp {
    pub trpid: u16,
    pub cbgrtrp: u16,
    pub cch: u16,
    pub cbRgb: u16,
}
impl ::core::marker::Copy for _trp {}
impl ::core::clone::Clone for _trp {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const atypFile: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const atypMax: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const atypNull: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const atypOle: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const atypPicture: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const cbDisplayName: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const cbEmailName: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const cbMaxIdData: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const cbSeverName: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const cbTYPE: u32 = 16u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct linereqmakecallW_tag {
    pub szDestAddress: [u16; 80],
    pub szAppName: [u16; 40],
    pub szCalledParty: [u16; 40],
    pub szComment: [u16; 80],
}
impl ::core::marker::Copy for linereqmakecallW_tag {}
impl ::core::clone::Clone for linereqmakecallW_tag {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct linereqmediacallW_tag {
    pub hWnd: super::super::Foundation::HWND,
    pub wRequestID: super::super::Foundation::WPARAM,
    pub szDeviceClass: [u16; 40],
    pub ucDeviceID: [u8; 40],
    pub dwSize: u32,
    pub dwSecure: u32,
    pub szDestAddress: [u16; 80],
    pub szAppName: [u16; 40],
    pub szCalledParty: [u16; 40],
    pub szComment: [u16; 80],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for linereqmediacallW_tag {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for linereqmediacallW_tag {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const prioHigh: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const prioLow: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const prioNorm: u32 = 2u32;
