#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn BstrFromVector(psa: *const super::Com::SAFEARRAY, pbstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ClearCustData(pcustdata: *mut super::Com::CUSTDATA);
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateDispTypeInfo(pidata: *mut INTERFACEDATA, lcid: u32, pptinfo: *mut *mut *mut super::Com::ITypeInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn CreateErrorInfo(pperrinfo: *mut *mut *mut ICreateErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn CreateOleAdviseHolder(ppoaholder: *mut *mut *mut IOleAdviseHolder) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateStdDispatch(punkouter: *mut *mut ::windows_sys::core::IUnknown, pvthis: *mut ::core::ffi::c_void, ptinfo: *mut *mut super::Com::ITypeInfo, ppunkstddisp: *mut *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateTypeLib(syskind: super::Com::SYSKIND, szfile: ::windows_sys::core::PCWSTR, ppctlib: *mut *mut *mut ICreateTypeLib) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateTypeLib2(syskind: super::Com::SYSKIND, szfile: ::windows_sys::core::PCWSTR, ppctlib: *mut *mut *mut ICreateTypeLib2) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispCallFunc(pvinstance: *const ::core::ffi::c_void, ovft: usize, cc: super::Com::CALLCONV, vtreturn: u16, cactuals: u32, prgvt: *const u16, prgpvarg: *const *const super::Com::VARIANT, pvargresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DispGetIDsOfNames(ptinfo: *mut *mut super::Com::ITypeInfo, rgsznames: *const ::windows_sys::core::PWSTR, cnames: u32, rgdispid: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispGetParam(pdispparams: *const super::Com::DISPPARAMS, position: u32, vttarg: u16, pvarresult: *mut super::Com::VARIANT, puargerr: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispInvoke(_this: *mut ::core::ffi::c_void, ptinfo: *mut *mut super::Com::ITypeInfo, dispidmember: i32, wflags: u16, pparams: *mut super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DoDragDrop(pdataobj: *mut *mut super::Com::IDataObject, pdropsource: *mut *mut IDropSource, dwokeffects: u32, pdweffect: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn DosDateTimeToVariantTime(wdosdate: u16, wdostime: u16, pvtime: *mut f64) -> i32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn GetActiveObject(rclsid: *const ::windows_sys::core::GUID, pvreserved: *mut ::core::ffi::c_void, ppunk: *mut *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn GetAltMonthNames(lcid: u32, prgp: *mut *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn GetRecordInfoFromGuids(rguidtypelib: *const ::windows_sys::core::GUID, uvermajor: u32, uverminor: u32, lcid: u32, rguidtypeinfo: *const ::windows_sys::core::GUID, pprecinfo: *mut *mut *mut IRecordInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn GetRecordInfoFromTypeInfo(ptypeinfo: *mut *mut super::Com::ITypeInfo, pprecinfo: *mut *mut *mut IRecordInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserFree(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN);
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserFree64(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN);
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserSize(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn IsAccelerator(haccel: super::super::UI::WindowsAndMessaging::HACCEL, caccelentries: i32, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG, lpwcmd: *mut u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn LHashValOfNameSys(syskind: super::Com::SYSKIND, lcid: u32, szname: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn LHashValOfNameSysA(syskind: super::Com::SYSKIND, lcid: u32, szname: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn LoadRegTypeLib(rguid: *const ::windows_sys::core::GUID, wvermajor: u16, wverminor: u16, lcid: u32, pptlib: *mut *mut *mut super::Com::ITypeLib) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn LoadTypeLib(szfile: ::windows_sys::core::PCWSTR, pptlib: *mut *mut *mut super::Com::ITypeLib) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn LoadTypeLibEx(szfile: ::windows_sys::core::PCWSTR, regkind: REGKIND, pptlib: *mut *mut *mut super::Com::ITypeLib) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OaBuildVersion() -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OaEnablePerUserTLibRegistration();
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OleBuildVersion() -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreate(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: *mut *mut IOleClientSite, pstg: *mut *mut super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OleCreateDefaultHandler(clsid: *const ::windows_sys::core::GUID, punkouter: *mut *mut ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, lplpobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleCreateEmbeddingHelper(clsid: *const ::windows_sys::core::GUID, punkouter: *mut *mut ::windows_sys::core::IUnknown, flags: u32, pcf: *mut *mut super::Com::IClassFactory, riid: *const ::windows_sys::core::GUID, lplpobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateEx(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: *mut *mut super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: *mut *mut IOleClientSite, pstg: *mut *mut super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleCreateFontIndirect(lpfontdesc: *mut FONTDESC, riid: *const ::windows_sys::core::GUID, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateFromData(psrcdataobj: *mut *mut super::Com::IDataObject, riid: *const ::windows_sys::core::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: *mut *mut IOleClientSite, pstg: *mut *mut super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateFromDataEx(psrcdataobj: *mut *mut super::Com::IDataObject, riid: *const ::windows_sys::core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: *mut *mut super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: *mut *mut IOleClientSite, pstg: *mut *mut super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateFromFile(rclsid: *const ::windows_sys::core::GUID, lpszfilename: ::windows_sys::core::PCWSTR, riid: *const ::windows_sys::core::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: *mut *mut IOleClientSite, pstg: *mut *mut super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateFromFileEx(rclsid: *const ::windows_sys::core::GUID, lpszfilename: ::windows_sys::core::PCWSTR, riid: *const ::windows_sys::core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: *mut *mut super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: *mut *mut IOleClientSite, pstg: *mut *mut super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateLink(pmklinksrc: *mut *mut super::Com::IMoniker, riid: *const ::windows_sys::core::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: *mut *mut IOleClientSite, pstg: *mut *mut super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateLinkEx(pmklinksrc: *mut *mut super::Com::IMoniker, riid: *const ::windows_sys::core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: *mut *mut super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: *mut *mut IOleClientSite, pstg: *mut *mut super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateLinkFromData(psrcdataobj: *mut *mut super::Com::IDataObject, riid: *const ::windows_sys::core::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: *mut *mut IOleClientSite, pstg: *mut *mut super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateLinkFromDataEx(psrcdataobj: *mut *mut super::Com::IDataObject, riid: *const ::windows_sys::core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: *mut *mut super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: *mut *mut IOleClientSite, pstg: *mut *mut super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateLinkToFile(lpszfilename: ::windows_sys::core::PCWSTR, riid: *const ::windows_sys::core::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: *mut *mut IOleClientSite, pstg: *mut *mut super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateLinkToFileEx(lpszfilename: ::windows_sys::core::PCWSTR, riid: *const ::windows_sys::core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: *mut *mut super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: *mut *mut IOleClientSite, pstg: *mut *mut super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn OleCreateMenuDescriptor(hmenucombined: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OleMenuGroupWidths) -> isize;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleCreatePictureIndirect(lppictdesc: *mut PICTDESC, riid: *const ::windows_sys::core::GUID, fown: super::super::Foundation::BOOL, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleCreatePropertyFrame(hwndowner: super::super::Foundation::HWND, x: u32, y: u32, lpszcaption: ::windows_sys::core::PCWSTR, cobjects: u32, ppunk: *mut *mut *mut ::windows_sys::core::IUnknown, cpages: u32, ppageclsid: *mut ::windows_sys::core::GUID, lcid: u32, dwreserved: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleCreatePropertyFrameIndirect(lpparams: *mut OCPFIPARAMS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateStaticFromData(psrcdataobj: *mut *mut super::Com::IDataObject, iid: *const ::windows_sys::core::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: *mut *mut IOleClientSite, pstg: *mut *mut super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OleDestroyMenuDescriptor(holemenu: isize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleDoAutoConvert(pstg: *mut *mut super::Com::StructuredStorage::IStorage, pclsidnew: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OleDraw(punknown: *mut *mut ::windows_sys::core::IUnknown, dwaspect: u32, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleDuplicateData(hsrc: super::super::Foundation::HANDLE, cfformat: u16, uiflags: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OleFlushClipboard() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OleGetAutoConvert(clsidold: *const ::windows_sys::core::GUID, pclsidnew: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleGetClipboard(ppdataobj: *mut *mut *mut super::Com::IDataObject) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleGetClipboardWithEnterpriseInfo(dataobject: *mut *mut *mut super::Com::IDataObject, dataenterpriseid: *mut ::windows_sys::core::PWSTR, sourcedescription: *mut ::windows_sys::core::PWSTR, targetdescription: *mut ::windows_sys::core::PWSTR, datadescription: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleGetIconOfClass(rclsid: *const ::windows_sys::core::GUID, lpszlabel: ::windows_sys::core::PCWSTR, fusetypeaslabel: super::super::Foundation::BOOL) -> isize;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleGetIconOfFile(lpszpath: ::windows_sys::core::PCWSTR, fusefileaslabel: super::super::Foundation::BOOL) -> isize;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleIconToCursor(hinstexe: super::super::Foundation::HINSTANCE, hicon: super::super::UI::WindowsAndMessaging::HICON) -> super::super::UI::WindowsAndMessaging::HCURSOR;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OleInitialize(pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleIsCurrentClipboard(pdataobj: *mut *mut super::Com::IDataObject) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleIsRunning(pobject: *mut *mut IOleObject) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleLoad(pstg: *mut *mut super::Com::StructuredStorage::IStorage, riid: *const ::windows_sys::core::GUID, pclientsite: *mut *mut IOleClientSite, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleLoadFromStream(pstm: *mut *mut super::Com::IStream, iidinterface: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPicture(lpstream: *mut *mut super::Com::IStream, lsize: i32, frunmode: super::super::Foundation::BOOL, riid: *const ::windows_sys::core::GUID, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPictureEx(lpstream: *mut *mut super::Com::IStream, lsize: i32, frunmode: super::super::Foundation::BOOL, riid: *const ::windows_sys::core::GUID, xsizedesired: u32, ysizedesired: u32, dwflags: u32, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPictureFile(varfilename: super::Com::VARIANT, lplpdisppicture: *mut *mut *mut super::Com::IDispatch) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPictureFileEx(varfilename: super::Com::VARIANT, xsizedesired: u32, ysizedesired: u32, dwflags: u32, lplpdisppicture: *mut *mut *mut super::Com::IDispatch) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OleLoadPicturePath(szurlorpath: ::windows_sys::core::PCWSTR, punkcaller: *mut *mut ::windows_sys::core::IUnknown, dwreserved: u32, clrreserved: u32, riid: *const ::windows_sys::core::GUID, ppvret: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleLockRunning(punknown: *mut *mut ::windows_sys::core::IUnknown, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn OleMetafilePictFromIconAndLabel(hicon: super::super::UI::WindowsAndMessaging::HICON, lpszlabel: ::windows_sys::core::PCWSTR, lpszsourcefile: ::windows_sys::core::PCWSTR, iiconindex: u32) -> isize;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleNoteObjectVisible(punknown: *mut *mut ::windows_sys::core::IUnknown, fvisible: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleQueryCreateFromData(psrcdataobject: *mut *mut super::Com::IDataObject) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleQueryLinkFromData(psrcdataobject: *mut *mut super::Com::IDataObject) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleRegEnumFormatEtc(clsid: *const ::windows_sys::core::GUID, dwdirection: u32, ppenum: *mut *mut *mut super::Com::IEnumFORMATETC) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OleRegEnumVerbs(clsid: *const ::windows_sys::core::GUID, ppenum: *mut *mut *mut IEnumOLEVERB) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OleRegGetMiscStatus(clsid: *const ::windows_sys::core::GUID, dwaspect: u32, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OleRegGetUserType(clsid: *const ::windows_sys::core::GUID, dwformoftype: u32, pszusertype: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OleRun(punknown: *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleSave(pps: *mut *mut super::Com::StructuredStorage::IPersistStorage, pstg: *mut *mut super::Com::StructuredStorage::IStorage, fsameasload: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleSavePictureFile(lpdisppicture: *mut *mut super::Com::IDispatch, bstrfilename: super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleSaveToStream(ppstm: *mut *mut super::Com::IPersistStream, pstm: *mut *mut super::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OleSetAutoConvert(clsidold: *const ::windows_sys::core::GUID, clsidnew: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleSetClipboard(pdataobj: *mut *mut super::Com::IDataObject) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleSetContainedObject(punknown: *mut *mut ::windows_sys::core::IUnknown, fcontained: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleSetMenuDescriptor(holemenu: isize, hwndframe: super::super::Foundation::HWND, hwndactiveobject: super::super::Foundation::HWND, lpframe: *mut *mut IOleInPlaceFrame, lpactiveobj: *mut *mut IOleInPlaceActiveObject) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleTranslateAccelerator(lpframe: *mut *mut IOleInPlaceFrame, lpframeinfo: *mut OIFI, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn OleTranslateColor(clr: u32, hpal: super::super::Graphics::Gdi::HPALETTE, lpcolorref: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIAddVerbMenuA(lpoleobj: *mut *mut IOleObject, lpszshorttype: ::windows_sys::core::PCSTR, hmenu: super::super::UI::WindowsAndMessaging::HMENU, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: super::super::Foundation::BOOL, idconvert: u32, lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIAddVerbMenuW(lpoleobj: *mut *mut IOleObject, lpszshorttype: ::windows_sys::core::PCWSTR, hmenu: super::super::UI::WindowsAndMessaging::HMENU, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: super::super::Foundation::BOOL, idconvert: u32, lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Media\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
    pub fn OleUIBusyA(param0: *const OLEUIBUSYA) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Media\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
    pub fn OleUIBusyW(param0: *const OLEUIBUSYW) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUICanConvertOrActivateAs(rclsid: *const ::windows_sys::core::GUID, fislinkedobject: super::super::Foundation::BOOL, wformat: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIChangeIconA(param0: *const OLEUICHANGEICONA) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIChangeIconW(param0: *const OLEUICHANGEICONW) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls_Dialogs\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn OleUIChangeSourceA(param0: *const OLEUICHANGESOURCEA) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls_Dialogs\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn OleUIChangeSourceW(param0: *const OLEUICHANGESOURCEW) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIConvertA(param0: *const OLEUICONVERTA) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIConvertW(param0: *const OLEUICONVERTW) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIEditLinksA(param0: *const OLEUIEDITLINKSA) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIEditLinksW(param0: *const OLEUIEDITLINKSW) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleUIInsertObjectA(param0: *const OLEUIINSERTOBJECTA) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleUIInsertObjectW(param0: *const OLEUIINSERTOBJECTW) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIObjectPropertiesA(param0: *const OLEUIOBJECTPROPSA) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIObjectPropertiesW(param0: *const OLEUIOBJECTPROPSW) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleUIPasteSpecialA(param0: *const OLEUIPASTESPECIALA) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleUIPasteSpecialW(param0: *const OLEUIPASTESPECIALW) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIPromptUserA(ntemplate: i32, hwndparent: super::super::Foundation::HWND) -> i32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIPromptUserW(ntemplate: i32, hwndparent: super::super::Foundation::HWND) -> i32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIUpdateLinksA(lpoleuilinkcntr: *mut *mut IOleUILinkContainerA, hwndparent: super::super::Foundation::HWND, lpsztitle: ::windows_sys::core::PCSTR, clinks: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIUpdateLinksW(lpoleuilinkcntr: *mut *mut IOleUILinkContainerW, hwndparent: super::super::Foundation::HWND, lpsztitle: ::windows_sys::core::PCWSTR, clinks: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn OleUninitialize();
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn QueryPathOfRegTypeLib(guid: *const ::windows_sys::core::GUID, wmaj: u16, wmin: u16, lcid: u32, lpbstrpathname: *mut *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn RegisterActiveObject(punk: *mut *mut ::windows_sys::core::IUnknown, rclsid: *const ::windows_sys::core::GUID, dwflags: u32, pdwregister: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterDragDrop(hwnd: super::super::Foundation::HWND, pdroptarget: *mut *mut IDropTarget) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RegisterTypeLib(ptlib: *mut *mut super::Com::ITypeLib, szfullpath: ::windows_sys::core::PCWSTR, szhelpdir: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RegisterTypeLibForUser(ptlib: *mut *mut super::Com::ITypeLib, szfullpath: ::windows_sys::core::PCWSTR, szhelpdir: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ReleaseStgMedium(param0: *mut super::Com::STGMEDIUM);
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn RevokeActiveObject(dwregister: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RevokeDragDrop(hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAccessData(psa: *const super::Com::SAFEARRAY, ppvdata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAddRef(psa: *const super::Com::SAFEARRAY, ppdatatorelease: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocData(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocDescriptor(cdims: u32, ppsaout: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocDescriptorEx(vt: u16, cdims: u32, ppsaout: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCopy(psa: *const super::Com::SAFEARRAY, ppsaout: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCopyData(psasource: *const super::Com::SAFEARRAY, psatarget: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreate(vt: u16, cdims: u32, rgsabound: *const super::Com::SAFEARRAYBOUND) -> *mut super::Com::SAFEARRAY;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateEx(vt: u16, cdims: u32, rgsabound: *const super::Com::SAFEARRAYBOUND, pvextra: *const ::core::ffi::c_void) -> *mut super::Com::SAFEARRAY;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateVector(vt: u16, llbound: i32, celements: u32) -> *mut super::Com::SAFEARRAY;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateVectorEx(vt: u16, llbound: i32, celements: u32, pvextra: *const ::core::ffi::c_void) -> *mut super::Com::SAFEARRAY;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroy(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroyData(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroyDescriptor(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetDim(psa: *const super::Com::SAFEARRAY) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetElement(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, pv: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetElemsize(psa: *const super::Com::SAFEARRAY) -> u32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetIID(psa: *const super::Com::SAFEARRAY, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetLBound(psa: *const super::Com::SAFEARRAY, ndim: u32, pllbound: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetRecordInfo(psa: *const super::Com::SAFEARRAY, prinfo: *mut *mut *mut IRecordInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetUBound(psa: *const super::Com::SAFEARRAY, ndim: u32, plubound: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetVartype(psa: *const super::Com::SAFEARRAY, pvt: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayLock(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayPtrOfIndex(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, ppvdata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayPutElement(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, pv: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayRedim(psa: *mut super::Com::SAFEARRAY, psaboundnew: *const super::Com::SAFEARRAYBOUND) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn SafeArrayReleaseData(pdata: *const ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayReleaseDescriptor(psa: *const super::Com::SAFEARRAY);
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArraySetIID(psa: *const super::Com::SAFEARRAY, guid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArraySetRecordInfo(psa: *const super::Com::SAFEARRAY, prinfo: *mut *mut IRecordInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayUnaccessData(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayUnlock(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemTimeToVariantTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME, pvtime: *mut f64) -> i32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn UnRegisterTypeLib(libid: *const ::windows_sys::core::GUID, wvermajor: u16, wverminor: u16, lcid: u32, syskind: super::Com::SYSKIND) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn UnRegisterTypeLibForUser(libid: *const ::windows_sys::core::GUID, wmajorvernum: u16, wminorvernum: u16, lcid: u32, syskind: super::Com::SYSKIND) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarAbs(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarAdd(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarAnd(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarBoolFromCy(cyin: super::Com::CY, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarBoolFromDate(datein: f64, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBoolFromDec(pdecin: *const super::super::Foundation::DECIMAL, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarBoolFromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBoolFromI1(cin: super::super::Foundation::CHAR, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarBoolFromI2(sin: i16, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarBoolFromI4(lin: i32, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarBoolFromI8(i64in: i64, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarBoolFromR4(fltin: f32, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarBoolFromR8(dblin: f64, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarBoolFromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarBoolFromUI1(bin: u8, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarBoolFromUI2(uiin: u16, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarBoolFromUI4(ulin: u32, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarBoolFromUI8(i64in: u64, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrCat(bstrleft: super::super::Foundation::BSTR, bstrright: super::super::Foundation::BSTR, pbstrresult: *mut *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrCmp(bstrleft: super::super::Foundation::BSTR, bstrright: super::super::Foundation::BSTR, lcid: u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromBool(boolin: i16, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarBstrFromCy(cyin: super::Com::CY, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromDate(datein: f64, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromDec(pdecin: *const super::super::Foundation::DECIMAL, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarBstrFromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI1(cin: super::super::Foundation::CHAR, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI2(ival: i16, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI4(lin: i32, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI8(i64in: i64, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromR4(fltin: f32, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromR8(dblin: f64, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI1(bval: u8, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI2(uiin: u16, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI4(ulin: u32, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI8(ui64in: u64, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCat(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCmp(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, lcid: u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyAbs(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyAdd(cyleft: super::Com::CY, cyright: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyCmp(cyleft: super::Com::CY, cyright: super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyCmpR8(cyleft: super::Com::CY, dblright: f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFix(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromBool(boolin: i16, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromDate(datein: f64, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCyFromDec(pdecin: *const super::super::Foundation::DECIMAL, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCyFromI1(cin: super::super::Foundation::CHAR, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI2(sin: i16, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI4(lin: i32, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI8(i64in: i64, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromR4(fltin: f32, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromR8(dblin: f64, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI1(bin: u8, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI2(uiin: u16, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI4(ulin: u32, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI8(ui64in: u64, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyInt(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMul(cyleft: super::Com::CY, cyright: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMulI4(cyleft: super::Com::CY, lright: i32, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMulI8(cyleft: super::Com::CY, lright: i64, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyNeg(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyRound(cyin: super::Com::CY, cdecimals: i32, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCySub(cyleft: super::Com::CY, cyright: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarDateFromBool(boolin: i16, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarDateFromCy(cyin: super::Com::CY, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromDec(pdecin: *const super::super::Foundation::DECIMAL, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarDateFromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromI1(cin: super::super::Foundation::CHAR, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarDateFromI2(sin: i16, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarDateFromI4(lin: i32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarDateFromI8(i64in: i64, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarDateFromR4(fltin: f32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarDateFromR8(dblin: f64, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarDateFromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarDateFromUI1(bin: u8, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarDateFromUI2(uiin: u16, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarDateFromUI4(ulin: u32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarDateFromUI8(ui64in: u64, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromUdate(pudatein: *const UDATE, dwflags: u32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromUdateEx(pudatein: *const UDATE, lcid: u32, dwflags: u32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecAbs(pdecin: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecAdd(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecCmp(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecCmpR8(pdecleft: *const super::super::Foundation::DECIMAL, dblright: f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecDiv(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFix(pdecin: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromBool(boolin: i16, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarDecFromCy(cyin: super::Com::CY, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromDate(datein: f64, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarDecFromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI1(cin: super::super::Foundation::CHAR, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI2(uiin: i16, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI4(lin: i32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI8(i64in: i64, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromR4(fltin: f32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromR8(dblin: f64, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI1(bin: u8, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI2(uiin: u16, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI4(ulin: u32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI8(ui64in: u64, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecInt(pdecin: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecMul(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecNeg(pdecin: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecRound(pdecin: *const super::super::Foundation::DECIMAL, cdecimals: i32, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecSub(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarDiv(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarEqv(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFix(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormat(pvarin: *const super::Com::VARIANT, pstrformat: ::windows_sys::core::PCWSTR, ifirstday: i32, ifirstweek: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatCurrency(pvarin: *const super::Com::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatDateTime(pvarin: *const super::Com::VARIANT, inamedformat: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatFromTokens(pvarin: *const super::Com::VARIANT, pstrformat: ::windows_sys::core::PCWSTR, pbtokcur: *const u8, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR, lcid: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatNumber(pvarin: *const super::Com::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatPercent(pvarin: *const super::Com::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI1FromBool(boolin: i16, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI1FromCy(cyin: super::Com::CY, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI1FromDate(datein: f64, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromDec(pdecin: *const super::super::Foundation::DECIMAL, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI1FromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI1FromI2(uiin: i16, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI1FromI4(lin: i32, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI1FromI8(i64in: i64, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI1FromR4(fltin: f32, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI1FromR8(dblin: f64, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI1FromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI1FromUI1(bin: u8, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI1FromUI2(uiin: u16, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI1FromUI4(ulin: u32, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI1FromUI8(i64in: u64, pcout: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI2FromBool(boolin: i16, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI2FromCy(cyin: super::Com::CY, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI2FromDate(datein: f64, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI2FromDec(pdecin: *const super::super::Foundation::DECIMAL, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI2FromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI2FromI1(cin: super::super::Foundation::CHAR, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI2FromI4(lin: i32, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI2FromI8(i64in: i64, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI2FromR4(fltin: f32, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI2FromR8(dblin: f64, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI2FromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI2FromUI1(bin: u8, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI2FromUI2(uiin: u16, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI2FromUI4(ulin: u32, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI2FromUI8(ui64in: u64, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI4FromBool(boolin: i16, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI4FromCy(cyin: super::Com::CY, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI4FromDate(datein: f64, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI4FromDec(pdecin: *const super::super::Foundation::DECIMAL, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI4FromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI4FromI1(cin: super::super::Foundation::CHAR, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI4FromI2(sin: i16, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI4FromI8(i64in: i64, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI4FromR4(fltin: f32, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI4FromR8(dblin: f64, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI4FromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI4FromUI1(bin: u8, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI4FromUI2(uiin: u16, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI4FromUI4(ulin: u32, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI4FromUI8(ui64in: u64, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI8FromBool(boolin: i16, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI8FromCy(cyin: super::Com::CY, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI8FromDate(datein: f64, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI8FromDec(pdecin: *const super::super::Foundation::DECIMAL, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI8FromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI8FromI1(cin: super::super::Foundation::CHAR, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI8FromI2(sin: i16, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI8FromR4(fltin: f32, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI8FromR8(dblin: f64, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI8FromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI8FromUI1(bin: u8, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI8FromUI2(uiin: u16, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI8FromUI4(ulin: u32, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarI8FromUI8(ui64in: u64, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarIdiv(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarImp(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarInt(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarMod(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarMonthName(imonth: i32, fabbrev: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarMul(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarNeg(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarNot(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarNumFromParseNum(pnumprs: *const NUMPARSE, rgbdig: *const u8, dwvtbits: u32, pvar: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarOr(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarParseNumFromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, pnumprs: *mut NUMPARSE, rgbdig: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarPow(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR4CmpR8(fltleft: f32, dblright: f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR4FromBool(boolin: i16, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR4FromCy(cyin: super::Com::CY, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR4FromDate(datein: f64, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR4FromDec(pdecin: *const super::super::Foundation::DECIMAL, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR4FromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR4FromI1(cin: super::super::Foundation::CHAR, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR4FromI2(sin: i16, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR4FromI4(lin: i32, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR4FromI8(i64in: i64, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR4FromR8(dblin: f64, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR4FromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR4FromUI1(bin: u8, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR4FromUI2(uiin: u16, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR4FromUI4(ulin: u32, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR4FromUI8(ui64in: u64, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR8FromBool(boolin: i16, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR8FromCy(cyin: super::Com::CY, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR8FromDate(datein: f64, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR8FromDec(pdecin: *const super::super::Foundation::DECIMAL, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR8FromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR8FromI1(cin: super::super::Foundation::CHAR, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR8FromI2(sin: i16, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR8FromI4(lin: i32, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR8FromI8(i64in: i64, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR8FromR4(fltin: f32, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR8FromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR8FromUI1(bin: u8, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR8FromUI2(uiin: u16, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR8FromUI4(ulin: u32, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR8FromUI8(ui64in: u64, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR8Pow(dblleft: f64, dblright: f64, pdblresult: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarR8Round(dblin: f64, cdecimals: i32, pdblresult: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarRound(pvarin: *const super::Com::VARIANT, cdecimals: i32, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarSub(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarTokenizeFormatString(pstrformat: ::windows_sys::core::PCWSTR, rgbtok: *mut u8, cbtok: i32, ifirstday: i32, ifirstweek: i32, lcid: u32, pcbactual: *const i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI1FromBool(boolin: i16, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI1FromCy(cyin: super::Com::CY, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI1FromDate(datein: f64, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI1FromDec(pdecin: *const super::super::Foundation::DECIMAL, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI1FromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI1FromI1(cin: super::super::Foundation::CHAR, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI1FromI2(sin: i16, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI1FromI4(lin: i32, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI1FromI8(i64in: i64, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI1FromR4(fltin: f32, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI1FromR8(dblin: f64, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI1FromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI1FromUI2(uiin: u16, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI1FromUI4(ulin: u32, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI1FromUI8(ui64in: u64, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI2FromBool(boolin: i16, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI2FromCy(cyin: super::Com::CY, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI2FromDate(datein: f64, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI2FromDec(pdecin: *const super::super::Foundation::DECIMAL, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI2FromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI2FromI1(cin: super::super::Foundation::CHAR, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI2FromI2(uiin: i16, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI2FromI4(lin: i32, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI2FromI8(i64in: i64, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI2FromR4(fltin: f32, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI2FromR8(dblin: f64, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI2FromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI2FromUI1(bin: u8, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI2FromUI4(ulin: u32, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI2FromUI8(i64in: u64, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI4FromBool(boolin: i16, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI4FromCy(cyin: super::Com::CY, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI4FromDate(datein: f64, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI4FromDec(pdecin: *const super::super::Foundation::DECIMAL, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI4FromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI4FromI1(cin: super::super::Foundation::CHAR, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI4FromI2(uiin: i16, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI4FromI4(lin: i32, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI4FromI8(i64in: i64, plout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI4FromR4(fltin: f32, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI4FromR8(dblin: f64, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI4FromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI4FromUI1(bin: u8, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI4FromUI2(uiin: u16, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI4FromUI8(ui64in: u64, plout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI8FromBool(boolin: i16, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI8FromCy(cyin: super::Com::CY, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI8FromDate(datein: f64, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI8FromDec(pdecin: *const super::super::Foundation::DECIMAL, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI8FromDisp(pdispin: *mut *mut super::Com::IDispatch, lcid: u32, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI8FromI1(cin: super::super::Foundation::CHAR, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI8FromI2(sin: i16, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI8FromI8(ui64in: i64, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI8FromR4(fltin: f32, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI8FromR8(dblin: f64, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI8FromStr(strin: ::windows_sys::core::PCWSTR, lcid: u32, dwflags: u32, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI8FromUI1(bin: u8, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI8FromUI2(uiin: u16, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VarUI8FromUI4(ulin: u32, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUdateFromDate(datein: f64, dwflags: u32, pudateout: *mut UDATE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarWeekdayName(iweekday: i32, fabbrev: i32, ifirstday: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarXor(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantChangeType(pvargdest: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, wflags: u16, vt: u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantChangeTypeEx(pvargdest: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, lcid: u32, wflags: u16, vt: u16) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantClear(pvarg: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantCopy(pvargdest: *mut super::Com::VARIANT, pvargsrc: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantCopyInd(pvardest: *mut super::Com::VARIANT, pvargsrc: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantInit(pvarg: *mut super::Com::VARIANT);
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    pub fn VariantTimeToDosDateTime(vtime: f64, pwdosdate: *mut u16, pwdostime: *mut u16) -> i32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VariantTimeToSystemTime(vtime: f64, lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> i32;
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VectorFromBstr(bstr: super::super::Foundation::BSTR, ppsa: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type ACTIVATEFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ACTIVATE_WINDOWLESS: ACTIVATEFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ACTIVEOBJECT_STRONG: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ACTIVEOBJECT_WEAK: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct ARRAYDESC {
    pub tdescElem: super::Com::TYPEDESC,
    pub cDims: u16,
    pub rgbounds: [super::Com::SAFEARRAYBOUND; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for ARRAYDESC {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ARRAYDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct AspectInfo {
    pub cb: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for AspectInfo {}
impl ::core::clone::Clone for AspectInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type AspectInfoFlag = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DVASPECTINFOFLAG_CANOPTIMIZE: AspectInfoFlag = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type BINDSPEED = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BINDSPEED_INDEFINITE: BINDSPEED = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BINDSPEED_MODERATE: BINDSPEED = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BINDSPEED_IMMEDIATE: BINDSPEED = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BZ_DISABLECANCELBUTTON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BZ_DISABLERETRYBUTTON: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BZ_DISABLESWITCHTOBUTTON: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BZ_NOTRESPONDINGDIALOG: i32 = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct CADWORD {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl ::core::marker::Copy for CADWORD {}
impl ::core::clone::Clone for CADWORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct CALPOLESTR {
    pub cElems: u32,
    pub pElems: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CALPOLESTR {}
impl ::core::clone::Clone for CALPOLESTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct CAUUID {
    pub cElems: u32,
    pub pElems: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CAUUID {}
impl ::core::clone::Clone for CAUUID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_CONVERTONLY: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_DISABLEACTIVATEAS: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_DISABLEDISPLAYASICON: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_HIDECHANGEICON: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_SELECTACTIVATEAS: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_SELECTCONVERTTO: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_SETACTIVATEDEFAULT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_SETCONVERTDEFAULT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_SHOWHELPBUTTON: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type CHANGEKIND = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_ADDMEMBER: CHANGEKIND = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_DELETEMEMBER: CHANGEKIND = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_SETNAMES: CHANGEKIND = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_SETDOCUMENTATION: CHANGEKIND = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_GENERAL: CHANGEKIND = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_INVALIDATE: CHANGEKIND = 5i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_CHANGEFAILED: CHANGEKIND = 6i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_MAX: CHANGEKIND = 7i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CIF_SELECTCURRENT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CIF_SELECTDEFAULT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CIF_SELECTFROMFILE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CIF_SHOWHELP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CIF_USEICONEXE: i32 = 16i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct CLEANLOCALSTORAGE {
    pub pInterface: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
    pub pStorage: *mut ::core::ffi::c_void,
    pub flags: u32,
}
impl ::core::marker::Copy for CLEANLOCALSTORAGE {}
impl ::core::clone::Clone for CLEANLOCALSTORAGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLSID_CColorPropPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 199447041, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CLSID_CFontPropPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 199447040, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CLSID_CPicturePropPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 199447042, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CLSID_ConvertVBX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4220454946, data2: 356, data3: 4123, data4: [132, 237, 8, 0, 43, 46, 199, 19] };
pub const CLSID_PersistPropset: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4220454945, data2: 356, data3: 4123, data4: [132, 237, 8, 0, 43, 46, 199, 19] };
pub const CLSID_StdFont: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 199447043, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CLSID_StdPicture: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 199447044, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_E_ADVISELIMIT: ::windows_sys::core::HRESULT = -2147220991i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_E_CANNOTCONNECT: ::windows_sys::core::HRESULT = -2147220990i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_E_FIRST: i32 = -2147220992i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_E_LAST: ::windows_sys::core::HRESULT = -2147220977i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_E_NOCONNECTION: ::windows_sys::core::HRESULT = -2147220992i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_E_OVERRIDDEN: ::windows_sys::core::HRESULT = -2147220989i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_S_FIRST: ::windows_sys::core::HRESULT = 262656i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_S_LAST: ::windows_sys::core::HRESULT = 262671i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct CONTROLINFO {
    pub cb: u32,
    pub hAccel: super::super::UI::WindowsAndMessaging::HACCEL,
    pub cAccel: u16,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for CONTROLINFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for CONTROLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CSF_EXPLORER: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CSF_ONLYGETSOURCE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CSF_SHOWHELP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CSF_VALIDSOURCE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CTL_E_ILLEGALFUNCTIONCALL: i32 = -2146828283i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type CTRLINFO = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CTRLINFO_EATS_RETURN: CTRLINFO = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CTRLINFO_EATS_ESCAPE: CTRLINFO = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DD_DEFDRAGDELAY: u32 = 200u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DD_DEFDRAGMINDIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DD_DEFSCROLLDELAY: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DD_DEFSCROLLINSET: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DD_DEFSCROLLINTERVAL: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type DISCARDCACHE = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISCARDCACHE_SAVEIFDIRTY: DISCARDCACHE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISCARDCACHE_NOSAVE: DISCARDCACHE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPATCH_CONSTRUCT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPATCH_METHOD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPATCH_PROPERTYGET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPATCH_PROPERTYPUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPATCH_PROPERTYPUTREF: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_ABOUTBOX: i32 = -552i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_ACCELERATOR: i32 = -543i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_ADDITEM: i32 = -553i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_APPEARANCE: i32 = -716i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_AUTOCLIP: i32 = -715i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_BACKCOLOR: i32 = -701i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_CHARSET: i32 = -727i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_CODEPAGE: i32 = -725i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_DISPLAYASDEFAULT: i32 = -713i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_DISPLAYNAME: i32 = -702i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_FONT: i32 = -703i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_FORECOLOR: i32 = -704i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_LOCALEID: i32 = -705i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_MESSAGEREFLECT: i32 = -706i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_PALETTE: i32 = -726i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_RIGHTTOLEFT: i32 = -732i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_SCALEUNITS: i32 = -707i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_SHOWGRABHANDLES: i32 = -711i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_SHOWHATCHING: i32 = -712i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_SUPPORTSMNEMONICS: i32 = -714i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_TEXTALIGN: i32 = -708i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_TOPTOBOTTOM: i32 = -733i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_TRANSFERPRIORITY: i32 = -728i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_UIDEAD: i32 = -710i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_USERMODE: i32 = -709i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_APPEARANCE: i32 = -520i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AUTOSIZE: i32 = -500i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_BACKCOLOR: i32 = -501i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_BACKSTYLE: i32 = -502i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_BORDERCOLOR: i32 = -503i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_BORDERSTYLE: i32 = -504i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_BORDERVISIBLE: i32 = -519i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_BORDERWIDTH: i32 = -505i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_CAPTION: i32 = -518i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_CLEAR: i32 = -554i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_CLICK: i32 = -600i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_CLICK_VALUE: i32 = -610i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_COLLECT: i32 = -8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_COLUMN: i32 = -529i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_CONSTRUCTOR: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DBLCLICK: i32 = -601i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DESTRUCTOR: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DISPLAYSTYLE: i32 = -540i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DOCLICK: i32 = -551i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DRAWMODE: i32 = -507i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DRAWSTYLE: i32 = -508i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DRAWWIDTH: i32 = -509i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_Delete: i32 = -801i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_ENABLED: i32 = -514i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_ENTERKEYBEHAVIOR: i32 = -544i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_ERROREVENT: i32 = -608i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_EVALUATE: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FILLCOLOR: i32 = -510i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FILLSTYLE: i32 = -511i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT: i32 = -512i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_BOLD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_CHANGED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_CHARSET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_ITALIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_NAME: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_SIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_STRIKE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_UNDER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_WEIGHT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FORECOLOR: i32 = -513i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_GROUPNAME: i32 = -541i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_HWND: i32 = -515i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_IMEMODE: i32 = -542i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_KEYDOWN: i32 = -602i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_KEYPRESS: i32 = -603i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_KEYUP: i32 = -604i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_LIST: i32 = -528i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_LISTCOUNT: i32 = -531i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_LISTINDEX: i32 = -526i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MAXLENGTH: i32 = -533i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MOUSEDOWN: i32 = -605i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MOUSEICON: i32 = -522i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MOUSEMOVE: i32 = -606i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MOUSEPOINTER: i32 = -521i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MOUSEUP: i32 = -607i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MULTILINE: i32 = -537i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MULTISELECT: i32 = -532i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_NEWENUM: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_NUMBEROFCOLUMNS: i32 = -539i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_NUMBEROFROWS: i32 = -538i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_Name: i32 = -800i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_Object: i32 = -802i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PASSWORDCHAR: i32 = -534i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICTURE: i32 = -523i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICT_HANDLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICT_HEIGHT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICT_HPAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICT_RENDER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICT_TYPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICT_WIDTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PROPERTYPUT: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_Parent: i32 = -803i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_READYSTATE: i32 = -525i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_READYSTATECHANGE: i32 = -609i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_REFRESH: i32 = -550i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_REMOVEITEM: i32 = -555i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_RIGHTTOLEFT: i32 = -611i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_SCROLLBARS: i32 = -535i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_SELECTED: i32 = -527i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_SELLENGTH: i32 = -548i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_SELSTART: i32 = -547i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_SELTEXT: i32 = -546i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_STARTENUM: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_TABKEYBEHAVIOR: i32 = -545i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_TABSTOP: i32 = -516i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_TEXT: i32 = -517i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_THIS: i32 = -613i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_TOPTOBOTTOM: i32 = -612i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_UNKNOWN: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_VALID: i32 = -524i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_VALUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_WORDWRAP: i32 = -536i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type DOCMISC = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DOCMISC_CANCREATEMULTIPLEVIEWS: DOCMISC = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DOCMISC_SUPPORTCOMPLEXRECTANGLES: DOCMISC = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DOCMISC_CANTOPENEDIT: DOCMISC = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DOCMISC_NOFILESUPPORT: DOCMISC = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DROPEFFECT_COPY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DROPEFFECT_LINK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DROPEFFECT_MOVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DROPEFFECT_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DROPEFFECT_SCROLL: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type DVASPECT2 = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DVASPECT_OPAQUE: DVASPECT2 = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DVASPECT_TRANSPARENT: DVASPECT2 = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ELF_DISABLECANCELLINK: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ELF_DISABLECHANGESOURCE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ELF_DISABLEOPENSOURCE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ELF_DISABLEUPDATENOW: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ELF_SHOWHELP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const EMBDHLP_CREATENOW: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const EMBDHLP_DELAYCREATE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const EMBDHLP_INPROC_HANDLER: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const EMBDHLP_INPROC_SERVER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type ENUM_CONTROLS_WHICH_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GCW_WCH_SIBLING: ENUM_CONTROLS_WHICH_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_CONTAINER: ENUM_CONTROLS_WHICH_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_CONTAINED: ENUM_CONTROLS_WHICH_FLAGS = 3u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_ALL: ENUM_CONTROLS_WHICH_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_FREVERSEDIR: ENUM_CONTROLS_WHICH_FLAGS = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_FONLYAFTER: ENUM_CONTROLS_WHICH_FLAGS = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_FONLYBEFORE: ENUM_CONTROLS_WHICH_FLAGS = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_FSELECTED: ENUM_CONTROLS_WHICH_FLAGS = 1073741824u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ExtentInfo {
    pub cb: u32,
    pub dwExtentMode: u32,
    pub sizelProposed: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ExtentInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ExtentInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type ExtentMode = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DVEXTENT_CONTENT: ExtentMode = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DVEXTENT_INTEGRAL: ExtentMode = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FADF_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FADF_BSTR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FADF_DISPATCH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FADF_EMBEDDED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FADF_FIXEDSIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FADF_HAVEIID: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FADF_HAVEVARTYPE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FADF_RECORD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FADF_RESERVED: u32 = 61448u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FADF_STATIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FADF_UNKNOWN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FADF_VARIANT: u32 = 2048u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct FONTDESC {
    pub cbSizeofstruct: u32,
    pub lpstrName: ::windows_sys::core::PWSTR,
    pub cySize: super::Com::CY,
    pub sWeight: i16,
    pub sCharset: i16,
    pub fItalic: super::super::Foundation::BOOL,
    pub fUnderline: super::super::Foundation::BOOL,
    pub fStrikethrough: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for FONTDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for FONTDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type FUNCFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FUNCFLAG_FRESTRICTED: FUNCFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FUNCFLAG_FSOURCE: FUNCFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FUNCFLAG_FBINDABLE: FUNCFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FUNCFLAG_FREQUESTEDIT: FUNCFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FUNCFLAG_FDISPLAYBIND: FUNCFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FUNCFLAG_FDEFAULTBIND: FUNCFLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FUNCFLAG_FHIDDEN: FUNCFLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FUNCFLAG_FUSESGETLASTERROR: FUNCFLAGS = 128i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FUNCFLAG_FDEFAULTCOLLELEM: FUNCFLAGS = 256i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FUNCFLAG_FUIDEFAULT: FUNCFLAGS = 512i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FUNCFLAG_FNONBROWSABLE: FUNCFLAGS = 1024i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FUNCFLAG_FREPLACEABLE: FUNCFLAGS = 2048i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const FUNCFLAG_FIMMEDIATEBIND: FUNCFLAGS = 4096i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_SIBLING: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type GUIDKIND = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GUIDKIND_DEFAULT_SOURCE_DISP_IID: GUIDKIND = 1i32;
pub const GUID_CHECKVALUEEXCLUSIVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536076, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_COLOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536065, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTBOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536079, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTITALIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536080, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTNAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536077, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTSIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536078, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTSTRIKETHROUGH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536082, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTUNDERSCORE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536081, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_HANDLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536083, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_HIMETRIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536064, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_OPTIONVALUEEXCLUSIVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536075, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_TRISTATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536074, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_XPOS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536070, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_XPOSPIXEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536066, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_XSIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536072, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_XSIZEPIXEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536068, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_YPOS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536071, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_YPOSPIXEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536067, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_YSIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536073, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_YSIZEPIXEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716536069, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type HITRESULT = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const HITRESULT_OUTSIDE: HITRESULT = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const HITRESULT_TRANSPARENT: HITRESULT = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const HITRESULT_CLOSE: HITRESULT = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const HITRESULT_HIT: HITRESULT = 3i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAdviseSinkEx {
    pub base__: super::Com::IAdviseSink,
    pub OnViewStatusChange: unsafe extern "system" fn(this: *mut *mut Self, dwviewstatus: u32),
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAdviseSinkEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 988955280, data2: 3222, data3: 4558, data4: [160, 207, 0, 170, 0, 96, 10, 184] };
}
#[repr(C)]
pub struct ICanHandleException {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CanHandleException: unsafe extern "system" fn(this: *mut *mut Self, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CanHandleException: usize,
}
impl ::windows_sys::core::Interface for ICanHandleException {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3310980704, data2: 45831, data3: 4561, data4: [178, 125, 0, 96, 8, 195, 251, 251] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IClassFactory2 {
    pub base__: super::Com::IClassFactory,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLicInfo: unsafe extern "system" fn(this: *mut *mut Self, plicinfo: *mut LICINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLicInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestLicKey: unsafe extern "system" fn(this: *mut *mut Self, dwreserved: u32, pbstrkey: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestLicKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateInstanceLic: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, bstrkey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateInstanceLic: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IClassFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2979443343, data2: 47796, data3: 4122, data4: [182, 156, 0, 170, 0, 52, 29, 7] };
}
#[repr(C)]
pub struct IContinue {
    pub base__: ::windows_sys::core::IUnknown,
    pub FContinue: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContinue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 298, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IContinueCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub FContinue: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub FContinuePrinting: unsafe extern "system" fn(this: *mut *mut Self, ncntprinted: i32, ncurpage: i32, pwszprintstatus: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContinueCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3072507082, data2: 20072, data3: 4123, data4: [162, 188, 0, 170, 0, 64, 71, 112] };
}
#[repr(C)]
pub struct ICreateErrorInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetGUID: unsafe extern "system" fn(this: *mut *mut Self, rguid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, szsource: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, szdescription: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetHelpFile: unsafe extern "system" fn(this: *mut *mut Self, szhelpfile: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetHelpContext: unsafe extern "system" fn(this: *mut *mut Self, dwhelpcontext: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateErrorInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 586167104, data2: 21629, data3: 4123, data4: [142, 101, 8, 0, 43, 43, 209, 25] };
}
#[repr(C)]
pub struct ICreateTypeInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetGuid: unsafe extern "system" fn(this: *mut *mut Self, guid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetTypeFlags: unsafe extern "system" fn(this: *mut *mut Self, utypeflags: u32) -> ::windows_sys::core::HRESULT,
    pub SetDocString: unsafe extern "system" fn(this: *mut *mut Self, pstrdoc: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetHelpContext: unsafe extern "system" fn(this: *mut *mut Self, dwhelpcontext: u32) -> ::windows_sys::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut *mut Self, wmajorvernum: u16, wminorvernum: u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRefTypeInfo: unsafe extern "system" fn(this: *mut *mut Self, ptinfo: *mut ::core::ffi::c_void, phreftype: *const u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRefTypeInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddFuncDesc: unsafe extern "system" fn(this: *mut *mut Self, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddFuncDesc: usize,
    pub AddImplType: unsafe extern "system" fn(this: *mut *mut Self, index: u32, hreftype: u32) -> ::windows_sys::core::HRESULT,
    pub SetImplTypeFlags: unsafe extern "system" fn(this: *mut *mut Self, index: u32, impltypeflags: i32) -> ::windows_sys::core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(this: *mut *mut Self, cbalignment: u16) -> ::windows_sys::core::HRESULT,
    pub SetSchema: unsafe extern "system" fn(this: *mut *mut Self, pstrschema: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddVarDesc: unsafe extern "system" fn(this: *mut *mut Self, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddVarDesc: usize,
    pub SetFuncAndParamNames: unsafe extern "system" fn(this: *mut *mut Self, index: u32, rgsznames: *const ::windows_sys::core::PWSTR, cnames: u32) -> ::windows_sys::core::HRESULT,
    pub SetVarName: unsafe extern "system" fn(this: *mut *mut Self, index: u32, szname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetTypeDescAlias: unsafe extern "system" fn(this: *mut *mut Self, ptdescalias: *const super::Com::TYPEDESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetTypeDescAlias: usize,
    pub DefineFuncAsDllEntry: unsafe extern "system" fn(this: *mut *mut Self, index: u32, szdllname: ::windows_sys::core::PCWSTR, szprocname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetFuncDocString: unsafe extern "system" fn(this: *mut *mut Self, index: u32, szdocstring: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetVarDocString: unsafe extern "system" fn(this: *mut *mut Self, index: u32, szdocstring: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetFuncHelpContext: unsafe extern "system" fn(this: *mut *mut Self, index: u32, dwhelpcontext: u32) -> ::windows_sys::core::HRESULT,
    pub SetVarHelpContext: unsafe extern "system" fn(this: *mut *mut Self, index: u32, dwhelpcontext: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMops: unsafe extern "system" fn(this: *mut *mut Self, index: u32, bstrmops: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMops: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetTypeIdldesc: unsafe extern "system" fn(this: *mut *mut Self, pidldesc: *const super::Com::IDLDESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetTypeIdldesc: usize,
    pub LayOut: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateTypeInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 132101, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct ICreateTypeInfo2 {
    pub base__: ICreateTypeInfo,
    pub DeleteFuncDesc: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteFuncDescByMemId: unsafe extern "system" fn(this: *mut *mut Self, memid: i32, invkind: super::Com::INVOKEKIND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteFuncDescByMemId: usize,
    pub DeleteVarDesc: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub DeleteVarDescByMemId: unsafe extern "system" fn(this: *mut *mut Self, memid: i32) -> ::windows_sys::core::HRESULT,
    pub DeleteImplType: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetCustData: unsafe extern "system" fn(this: *mut *mut Self, guid: *const ::windows_sys::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetFuncCustData: unsafe extern "system" fn(this: *mut *mut Self, index: u32, guid: *const ::windows_sys::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetFuncCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetParamCustData: unsafe extern "system" fn(this: *mut *mut Self, indexfunc: u32, indexparam: u32, guid: *const ::windows_sys::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetParamCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetVarCustData: unsafe extern "system" fn(this: *mut *mut Self, index: u32, guid: *const ::windows_sys::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetVarCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetImplTypeCustData: unsafe extern "system" fn(this: *mut *mut Self, index: u32, guid: *const ::windows_sys::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetImplTypeCustData: usize,
    pub SetHelpStringContext: unsafe extern "system" fn(this: *mut *mut Self, dwhelpstringcontext: u32) -> ::windows_sys::core::HRESULT,
    pub SetFuncHelpStringContext: unsafe extern "system" fn(this: *mut *mut Self, index: u32, dwhelpstringcontext: u32) -> ::windows_sys::core::HRESULT,
    pub SetVarHelpStringContext: unsafe extern "system" fn(this: *mut *mut Self, index: u32, dwhelpstringcontext: u32) -> ::windows_sys::core::HRESULT,
    pub Invalidate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, szname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateTypeInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 132110, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct ICreateTypeLib {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTypeInfo: unsafe extern "system" fn(this: *mut *mut Self, szname: ::windows_sys::core::PCWSTR, tkind: super::Com::TYPEKIND, ppctinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTypeInfo: usize,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, szname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut *mut Self, wmajorvernum: u16, wminorvernum: u16) -> ::windows_sys::core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(this: *mut *mut Self, guid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetDocString: unsafe extern "system" fn(this: *mut *mut Self, szdoc: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetHelpFileName: unsafe extern "system" fn(this: *mut *mut Self, szhelpfilename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetHelpContext: unsafe extern "system" fn(this: *mut *mut Self, dwhelpcontext: u32) -> ::windows_sys::core::HRESULT,
    pub SetLcid: unsafe extern "system" fn(this: *mut *mut Self, lcid: u32) -> ::windows_sys::core::HRESULT,
    pub SetLibFlags: unsafe extern "system" fn(this: *mut *mut Self, ulibflags: u32) -> ::windows_sys::core::HRESULT,
    pub SaveAllChanges: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateTypeLib {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 132102, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct ICreateTypeLib2 {
    pub base__: ICreateTypeLib,
    pub DeleteTypeInfo: unsafe extern "system" fn(this: *mut *mut Self, szname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetCustData: unsafe extern "system" fn(this: *mut *mut Self, guid: *const ::windows_sys::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetCustData: usize,
    pub SetHelpStringContext: unsafe extern "system" fn(this: *mut *mut Self, dwhelpstringcontext: u32) -> ::windows_sys::core::HRESULT,
    pub SetHelpStringDll: unsafe extern "system" fn(this: *mut *mut Self, szfilename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateTypeLib2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 132111, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_BZ_ICON: u32 = 601u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_BZ_MESSAGE1: u32 = 602u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_BZ_RETRY: u32 = 600u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_BZ_SWITCHTO: u32 = 604u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_BROWSE: u32 = 130u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_CURRENT: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_CURRENTICON: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_DEFAULT: u32 = 123u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_DEFAULTICON: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_FROMFILE: u32 = 125u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_FROMFILEEDIT: u32 = 126u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_GROUP: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_ICONDISPLAY: u32 = 131u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_ICONLIST: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_LABEL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_LABELEDIT: u32 = 129u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_ACTIVATEAS: u32 = 156u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_ACTIVATELIST: u32 = 154u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_CHANGEICON: u32 = 153u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_CONVERTLIST: u32 = 158u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_CONVERTTO: u32 = 155u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_DISPLAYASICON: u32 = 152u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_ICONDISPLAY: u32 = 165u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_OBJECTTYPE: u32 = 150u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_RESULTTEXT: u32 = 157u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_AUTOMATIC: u32 = 202u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_CANCELLINK: u32 = 209u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_CHANGESOURCE: u32 = 201u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_COL1: u32 = 220u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_COL2: u32 = 221u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_COL3: u32 = 222u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_LINKSLISTBOX: u32 = 206u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_LINKSOURCE: u32 = 216u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_LINKTYPE: u32 = 217u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_MANUAL: u32 = 212u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_OPENSOURCE: u32 = 211u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_UPDATENOW: u32 = 210u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_GP_CONVERT: u32 = 1013u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_GP_OBJECTICON: u32 = 1014u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_GP_OBJECTLOCATION: u32 = 1022u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_GP_OBJECTNAME: u32 = 1009u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_GP_OBJECTSIZE: u32 = 1011u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_GP_OBJECTTYPE: u32 = 1010u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_ADDCONTROL: u32 = 2115u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_CHANGEICON: u32 = 2105u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_CONTROLTYPELIST: u32 = 2116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_CREATEFROMFILE: u32 = 2101u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_CREATENEW: u32 = 2100u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_DISPLAYASICON: u32 = 2104u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_FILE: u32 = 2106u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_FILEDISPLAY: u32 = 2107u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_FILETEXT: u32 = 2112u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_FILETYPE: u32 = 2113u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_ICONDISPLAY: u32 = 2110u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_INSERTCONTROL: u32 = 2114u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_LINKFILE: u32 = 2102u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_OBJECTTYPELIST: u32 = 2103u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_OBJECTTYPETEXT: u32 = 2111u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_RESULTIMAGE: u32 = 2108u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_RESULTTEXT: u32 = 2109u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_AUTOMATIC: u32 = 1016u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_BREAKLINK: u32 = 1008u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_CHANGESOURCE: u32 = 1015u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_DATE: u32 = 1018u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_LINKSOURCE: u32 = 1012u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_MANUAL: u32 = 1017u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_OPENSOURCE: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_TIME: u32 = 1019u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_UPDATENOW: u32 = 1007u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_OLEUIHELP: u32 = 99u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_CHANGEICON: u32 = 508u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_DISPLAYASICON: u32 = 506u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_DISPLAYLIST: u32 = 505u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_ICONDISPLAY: u32 = 507u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_PASTE: u32 = 500u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_PASTELINK: u32 = 501u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_PASTELINKLIST: u32 = 504u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_PASTELIST: u32 = 503u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_RESULTIMAGE: u32 = 509u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_RESULTTEXT: u32 = 510u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_SOURCETEXT: u32 = 502u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PU_CONVERT: u32 = 902u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PU_ICON: u32 = 908u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PU_LINKS: u32 = 900u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PU_TEXT: u32 = 901u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_UL_METER: u32 = 1029u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_UL_PERCENT: u32 = 1031u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_UL_PROGRESS: u32 = 1032u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_UL_STOP: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_ASICON: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_CHANGEICON: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_EDITABLE: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_ICONDISPLAY: u32 = 1021u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_PERCENT: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_RELATIVE: u32 = 1005u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_RESULTIMAGE: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_SCALETXT: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_SPIN: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_BUSY: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CANNOTUPDATELINK: u32 = 1008u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CHANGEICON: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CHANGEICONBROWSE: u32 = 1011u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CHANGESOURCE: u32 = 1009u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CHANGESOURCE4: u32 = 1013u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CONVERT: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CONVERT4: u32 = 1103u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CONVERTONLY: u32 = 1012u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CONVERTONLY4: u32 = 1104u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_EDITLINKS: u32 = 1004u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_EDITLINKS4: u32 = 1105u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_GNRLPROPS: u32 = 1100u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_GNRLPROPS4: u32 = 1106u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_INSERTFILEBROWSE: u32 = 1010u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_INSERTOBJECT: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_LINKPROPS: u32 = 1102u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_LINKPROPS4: u32 = 1107u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_LINKSOURCEUNAVAILABLE: u32 = 1020u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_LINKTYPECHANGED: u32 = 1022u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_LINKTYPECHANGEDA: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_LINKTYPECHANGEDW: u32 = 1022u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_OUTOFMEMORY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_PASTESPECIAL: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_PASTESPECIAL4: u32 = 1108u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_SERVERNOTFOUND: u32 = 1023u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_SERVERNOTREG: u32 = 1021u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_SERVERNOTREGA: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_SERVERNOTREGW: u32 = 1021u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_UPDATELINKS: u32 = 1007u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_VIEWPROPS: u32 = 1101u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDLFLAG_FIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDLFLAG_FLCID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDLFLAG_FOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDLFLAG_FRETVAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDLFLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ID_BROWSE_ADDCONTROL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ID_BROWSE_CHANGEICON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ID_BROWSE_CHANGESOURCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ID_BROWSE_INSERTFILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ID_DEFAULTINST: i32 = -2i32;
#[repr(C)]
pub struct IDispError {
    pub base__: ::windows_sys::core::IUnknown,
    pub QueryErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, guiderrortype: ::windows_sys::core::GUID, ppde: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNext: unsafe extern "system" fn(this: *mut *mut Self, ppde: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetHresult: unsafe extern "system" fn(this: *mut *mut Self, phr: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSource: unsafe extern "system" fn(this: *mut *mut Self, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHelpInfo: unsafe extern "system" fn(this: *mut *mut Self, pbstrfilename: *mut super::super::Foundation::BSTR, pdwcontext: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHelpInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
}
impl ::windows_sys::core::Interface for IDispError {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2800719969, data2: 50976, data3: 4560, data4: [147, 55, 0, 160, 201, 13, 202, 169] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDispatchEx {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDispID: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grfdex: u32, pid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDispID: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InvokeEx: unsafe extern "system" fn(this: *mut *mut Self, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: *mut super::Com::VARIANT, pei: *mut super::Com::EXCEPINFO, pspcaller: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InvokeEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteMemberByName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grfdex: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteMemberByName: usize,
    pub DeleteMemberByDispID: unsafe extern "system" fn(this: *mut *mut Self, id: i32) -> ::windows_sys::core::HRESULT,
    pub GetMemberProperties: unsafe extern "system" fn(this: *mut *mut Self, id: i32, grfdexfetch: u32, pgrfdex: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMemberName: unsafe extern "system" fn(this: *mut *mut Self, id: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMemberName: usize,
    pub GetNextDispID: unsafe extern "system" fn(this: *mut *mut Self, grfdex: u32, id: i32, pid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetNameSpaceParent: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDispatchEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2800719968, data2: 50976, data3: 4560, data4: [147, 55, 0, 160, 201, 13, 202, 169] };
}
#[repr(C)]
pub struct IDropSource {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryContinueDrag: unsafe extern "system" fn(this: *mut *mut Self, fescapepressed: super::super::Foundation::BOOL, grfkeystate: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryContinueDrag: usize,
    pub GiveFeedback: unsafe extern "system" fn(this: *mut *mut Self, dweffect: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDropSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 289, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IDropSourceNotify {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub DragEnterTarget: unsafe extern "system" fn(this: *mut *mut Self, hwndtarget: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DragEnterTarget: usize,
    pub DragLeaveTarget: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDropSourceNotify {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 299, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IDropTarget {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub DragEnter: unsafe extern "system" fn(this: *mut *mut Self, pdataobj: *mut ::core::ffi::c_void, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    DragEnter: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DragOver: unsafe extern "system" fn(this: *mut *mut Self, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DragOver: usize,
    pub DragLeave: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Drop: unsafe extern "system" fn(this: *mut *mut Self, pdataobj: *mut ::core::ffi::c_void, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Drop: usize,
}
impl ::windows_sys::core::Interface for IDropTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 290, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IEnterpriseDropTarget {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetDropSourceEnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEvaluatingEdpPolicy: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEvaluatingEdpPolicy: usize,
}
impl ::windows_sys::core::Interface for IEnterpriseDropTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 957233272, data2: 64853, data3: 19992, data4: [129, 157, 70, 130, 8, 28, 12, 253] };
}
#[repr(C)]
pub struct IEnumOLEVERB {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumOLEVERB {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 260, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IEnumOleDocumentViews {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, cviews: u32, rgpview: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, cviews: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumOleDocumentViews {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3072507080, data2: 20072, data3: 4123, data4: [162, 188, 0, 170, 0, 64, 71, 112] };
}
#[repr(C)]
pub struct IEnumOleUndoUnits {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumOleUndoUnits {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3018310464, data2: 61335, data3: 4558, data4: [155, 201, 0, 170, 0, 96, 142, 1] };
}
#[repr(C)]
pub struct IEnumVARIANT {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgvar: *mut super::Com::VARIANT, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumVARIANT {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 132100, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IFont {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, psize: *mut super::Com::CY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Size: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, size: super::Com::CY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Bold: unsafe extern "system" fn(this: *mut *mut Self, pbold: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Bold: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBold: unsafe extern "system" fn(this: *mut *mut Self, bold: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBold: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Italic: unsafe extern "system" fn(this: *mut *mut Self, pitalic: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Italic: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetItalic: unsafe extern "system" fn(this: *mut *mut Self, italic: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetItalic: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Underline: unsafe extern "system" fn(this: *mut *mut Self, punderline: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Underline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUnderline: unsafe extern "system" fn(this: *mut *mut Self, underline: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUnderline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Strikethrough: unsafe extern "system" fn(this: *mut *mut Self, pstrikethrough: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Strikethrough: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStrikethrough: unsafe extern "system" fn(this: *mut *mut Self, strikethrough: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStrikethrough: usize,
    pub Weight: unsafe extern "system" fn(this: *mut *mut Self, pweight: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(this: *mut *mut Self, weight: i16) -> ::windows_sys::core::HRESULT,
    pub Charset: unsafe extern "system" fn(this: *mut *mut Self, pcharset: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetCharset: unsafe extern "system" fn(this: *mut *mut Self, charset: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub hFont: unsafe extern "system" fn(this: *mut *mut Self, phfont: *mut super::super::Graphics::Gdi::HFONT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    hFont: usize,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppfont: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut *mut Self, pfontother: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRatio: unsafe extern "system" fn(this: *mut *mut Self, cylogical: i32, cyhimetric: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub QueryTextMetrics: unsafe extern "system" fn(this: *mut *mut Self, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    QueryTextMetrics: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AddRefHfont: unsafe extern "system" fn(this: *mut *mut Self, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AddRefHfont: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseHfont: unsafe extern "system" fn(this: *mut *mut Self, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseHfont: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetHdc: unsafe extern "system" fn(this: *mut *mut Self, hdc: super::super::Graphics::Gdi::HDC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetHdc: usize,
}
impl ::windows_sys::core::Interface for IFont {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3203850242, data2: 43124, data3: 4122, data4: [139, 186, 0, 170, 0, 48, 12, 171] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFontDisp {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFontDisp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3203850243, data2: 43124, data3: 4122, data4: [139, 186, 0, 170, 0, 48, 12, 171] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFontEventsDisp {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFontEventsDisp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1324748810, data2: 44936, data3: 4560, data4: [152, 70, 0, 192, 79, 194, 153, 147] };
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type IGNOREMIME = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IGNOREMIME_PROMPT: IGNOREMIME = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IGNOREMIME_TEXT: IGNOREMIME = 2i32;
#[repr(C)]
pub struct IGetOleObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOleObject: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGetOleObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2322603424, data2: 20459, data3: 4123, data4: [168, 46, 8, 0, 43, 43, 35, 55] };
}
#[repr(C)]
pub struct IGetVBAObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetObject: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGetVBAObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2440247904, data2: 16204, data3: 4123, data4: [163, 246, 0, 170, 0, 52, 228, 233] };
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IMPLTYPEFLAG_FDEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IMPLTYPEFLAG_FRESTRICTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IMPLTYPEFLAG_FSOURCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const INSTALL_SCOPE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const INSTALL_SCOPE_MACHINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const INSTALL_SCOPE_USER: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct INTERFACEDATA {
    pub pmethdata: *mut METHODDATA,
    pub cMembers: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for INTERFACEDATA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for INTERFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_CHECKDISPLAYASICON: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_CHECKLINK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_CREATEFILEOBJECT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_CREATELINKOBJECT: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_CREATENEWOBJECT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_DISABLEDISPLAYASICON: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_DISABLELINK: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_HIDECHANGEICON: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_SELECTCREATECONTROL: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_SELECTCREATEFROMFILE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_SELECTCREATENEW: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_SHOWHELP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_SHOWINSERTCONTROL: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_VERIFYSERVERSEXIST: i32 = 512i32;
#[repr(C)]
pub struct IObjectIdentity {
    pub base__: ::windows_sys::core::IUnknown,
    pub IsEqualObject: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IObjectIdentity {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3389306854, data2: 3361, data3: 4561, data4: [140, 197, 0, 192, 79, 194, 176, 133] };
}
#[repr(C)]
pub struct IObjectWithSite {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetSite: unsafe extern "system" fn(this: *mut *mut Self, punksite: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSite: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppvsite: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IObjectWithSite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4232577443, data2: 11177, data3: 4559, data4: [162, 41, 0, 170, 0, 61, 115, 82] };
}
#[repr(C)]
pub struct IOleAdviseHolder {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Advise: unsafe extern "system" fn(this: *mut *mut Self, padvise: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(this: *mut *mut Self, dwconnection: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumAdvise: unsafe extern "system" fn(this: *mut *mut Self, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumAdvise: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SendOnRename: unsafe extern "system" fn(this: *mut *mut Self, pmk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SendOnRename: usize,
    pub SendOnSave: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SendOnClose: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleAdviseHolder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 273, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleCache {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Cache: unsafe extern "system" fn(this: *mut *mut Self, pformatetc: *const super::Com::FORMATETC, advf: u32, pdwconnection: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cache: usize,
    pub Uncache: unsafe extern "system" fn(this: *mut *mut Self, dwconnection: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCache: unsafe extern "system" fn(this: *mut *mut Self, ppenumstatdata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCache: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitCache: unsafe extern "system" fn(this: *mut *mut Self, pdataobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitCache: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    SetData: usize,
}
impl ::windows_sys::core::Interface for IOleCache {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 286, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleCache2 {
    pub base__: IOleCache,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateCache: unsafe extern "system" fn(this: *mut *mut Self, pdataobject: *mut ::core::ffi::c_void, grfupdf: UPDFCACHE_FLAGS, preserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateCache: usize,
    pub DiscardCache: unsafe extern "system" fn(this: *mut *mut Self, dwdiscardoptions: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleCache2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 296, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleCacheControl {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub OnRun: unsafe extern "system" fn(this: *mut *mut Self, pdataobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnRun: usize,
    pub OnStop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleCacheControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 297, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleClientSite {
    pub base__: ::windows_sys::core::IUnknown,
    pub SaveObject: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMoniker: unsafe extern "system" fn(this: *mut *mut Self, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMoniker: usize,
    pub GetContainer: unsafe extern "system" fn(this: *mut *mut Self, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShowObject: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnShowWindow: unsafe extern "system" fn(this: *mut *mut Self, fshow: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnShowWindow: usize,
    pub RequestNewObjectLayout: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleClientSite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 280, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleCommandTarget {
    pub base__: ::windows_sys::core::IUnknown,
    pub QueryStatus: unsafe extern "system" fn(this: *mut *mut Self, pguidcmdgroup: *const ::windows_sys::core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Exec: unsafe extern "system" fn(this: *mut *mut Self, pguidcmdgroup: *const ::windows_sys::core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::Com::VARIANT, pvaout: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Exec: usize,
}
impl ::windows_sys::core::Interface for IOleCommandTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3072507083, data2: 20072, data3: 4123, data4: [162, 188, 0, 170, 0, 64, 71, 112] };
}
#[repr(C)]
pub struct IOleContainer {
    pub base__: IParseDisplayName,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumObjects: unsafe extern "system" fn(this: *mut *mut Self, grfflags: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumObjects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LockContainer: unsafe extern "system" fn(this: *mut *mut Self, flock: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LockContainer: usize,
}
impl ::windows_sys::core::Interface for IOleContainer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 283, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleControl {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetControlInfo: unsafe extern "system" fn(this: *mut *mut Self, pci: *mut CONTROLINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetControlInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub OnMnemonic: unsafe extern "system" fn(this: *mut *mut Self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    OnMnemonic: usize,
    pub OnAmbientPropertyChange: unsafe extern "system" fn(this: *mut *mut Self, dispid: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FreezeEvents: unsafe extern "system" fn(this: *mut *mut Self, bfreeze: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FreezeEvents: usize,
}
impl ::windows_sys::core::Interface for IOleControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2979443336, data2: 47796, data3: 4122, data4: [182, 156, 0, 170, 0, 52, 29, 7] };
}
#[repr(C)]
pub struct IOleControlSite {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnControlInfoChanged: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LockInPlaceActive: unsafe extern "system" fn(this: *mut *mut Self, flock: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LockInPlaceActive: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetExtendedControl: unsafe extern "system" fn(this: *mut *mut Self, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetExtendedControl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TransformCoords: unsafe extern "system" fn(this: *mut *mut Self, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: XFORMCOORDS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransformCoords: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TranslateAccelerator: unsafe extern "system" fn(this: *mut *mut Self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TranslateAccelerator: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnFocus: unsafe extern "system" fn(this: *mut *mut Self, fgotfocus: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnFocus: usize,
    pub ShowPropertyFrame: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleControlSite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2979443337, data2: 47796, data3: 4122, data4: [182, 156, 0, 170, 0, 52, 29, 7] };
}
#[repr(C)]
pub struct IOleDocument {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateView: unsafe extern "system" fn(this: *mut *mut Self, pipsite: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, dwreserved: u32, ppview: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateView: usize,
    pub GetDocMiscStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EnumViews: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void, ppview: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleDocument {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3072507077, data2: 20072, data3: 4123, data4: [162, 188, 0, 170, 0, 64, 71, 112] };
}
#[repr(C)]
pub struct IOleDocumentSite {
    pub base__: ::windows_sys::core::IUnknown,
    pub ActivateMe: unsafe extern "system" fn(this: *mut *mut Self, pviewtoactivate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleDocumentSite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3072507079, data2: 20072, data3: 4123, data4: [162, 188, 0, 170, 0, 64, 71, 112] };
}
#[repr(C)]
pub struct IOleDocumentView {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetInPlaceSite: unsafe extern "system" fn(this: *mut *mut Self, pipsite: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetInPlaceSite: unsafe extern "system" fn(this: *mut *mut Self, ppipsite: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDocument: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRect: unsafe extern "system" fn(this: *mut *mut Self, prcview: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRect: unsafe extern "system" fn(this: *mut *mut Self, prcview: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRectComplex: unsafe extern "system" fn(this: *mut *mut Self, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRectComplex: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, fshow: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UIActivate: unsafe extern "system" fn(this: *mut *mut Self, fuiactivate: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UIActivate: usize,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CloseView: unsafe extern "system" fn(this: *mut *mut Self, dwreserved: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveViewState: unsafe extern "system" fn(this: *mut *mut Self, pstm: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveViewState: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplyViewState: unsafe extern "system" fn(this: *mut *mut Self, pstm: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplyViewState: usize,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, pipsitenew: *mut ::core::ffi::c_void, ppviewnew: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleDocumentView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3072507078, data2: 20072, data3: 4123, data4: [162, 188, 0, 170, 0, 64, 71, 112] };
}
#[repr(C)]
pub struct IOleInPlaceActiveObject {
    pub base__: IOleWindow,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TranslateAccelerator: unsafe extern "system" fn(this: *mut *mut Self, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TranslateAccelerator: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnFrameWindowActivate: unsafe extern "system" fn(this: *mut *mut Self, factivate: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnFrameWindowActivate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDocWindowActivate: unsafe extern "system" fn(this: *mut *mut Self, factivate: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDocWindowActivate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ResizeBorder: unsafe extern "system" fn(this: *mut *mut Self, prcborder: *const super::super::Foundation::RECT, puiwindow: *mut ::core::ffi::c_void, fframewindow: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ResizeBorder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableModeless: unsafe extern "system" fn(this: *mut *mut Self, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableModeless: usize,
}
impl ::windows_sys::core::Interface for IOleInPlaceActiveObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 279, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleInPlaceFrame {
    pub base__: IOleInPlaceUIWindow,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub InsertMenus: unsafe extern "system" fn(this: *mut *mut Self, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OleMenuGroupWidths) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    InsertMenus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub SetMenu: unsafe extern "system" fn(this: *mut *mut Self, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    SetMenu: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub RemoveMenus: unsafe extern "system" fn(this: *mut *mut Self, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    RemoveMenus: usize,
    pub SetStatusText: unsafe extern "system" fn(this: *mut *mut Self, pszstatustext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableModeless: unsafe extern "system" fn(this: *mut *mut Self, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableModeless: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TranslateAccelerator: unsafe extern "system" fn(this: *mut *mut Self, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TranslateAccelerator: usize,
}
impl ::windows_sys::core::Interface for IOleInPlaceFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 278, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleInPlaceObject {
    pub base__: IOleWindow,
    pub InPlaceDeactivate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub UIDeactivate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetObjectRects: unsafe extern "system" fn(this: *mut *mut Self, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetObjectRects: usize,
    pub ReactivateAndUndo: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleInPlaceObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 275, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleInPlaceObjectWindowless {
    pub base__: IOleInPlaceObject,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowMessage: unsafe extern "system" fn(this: *mut *mut Self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowMessage: usize,
    pub GetDropTarget: unsafe extern "system" fn(this: *mut *mut Self, ppdroptarget: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleInPlaceObjectWindowless {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 471881420, data2: 24308, data3: 4123, data4: [139, 200, 0, 170, 0, 62, 59, 41] };
}
#[repr(C)]
pub struct IOleInPlaceSite {
    pub base__: IOleWindow,
    pub CanInPlaceActivate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnInPlaceActivate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnUIActivate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetWindowContext: unsafe extern "system" fn(this: *mut *mut Self, ppframe: *mut *mut ::core::ffi::c_void, ppdoc: *mut *mut ::core::ffi::c_void, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OIFI) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    GetWindowContext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Scroll: unsafe extern "system" fn(this: *mut *mut Self, scrollextant: super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Scroll: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnUIDeactivate: unsafe extern "system" fn(this: *mut *mut Self, fundoable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnUIDeactivate: usize,
    pub OnInPlaceDeactivate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DiscardUndoState: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DeactivateAndUndo: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnPosRectChange: unsafe extern "system" fn(this: *mut *mut Self, lprcposrect: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnPosRectChange: usize,
}
impl ::windows_sys::core::Interface for IOleInPlaceSite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 281, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleInPlaceSiteEx {
    pub base__: IOleInPlaceSite,
    #[cfg(feature = "Win32_Foundation")]
    pub OnInPlaceActivateEx: unsafe extern "system" fn(this: *mut *mut Self, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnInPlaceActivateEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnInPlaceDeactivateEx: unsafe extern "system" fn(this: *mut *mut Self, fnoredraw: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnInPlaceDeactivateEx: usize,
    pub RequestUIActivate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleInPlaceSiteEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2620173696, data2: 13348, data3: 4559, data4: [182, 112, 0, 170, 0, 76, 214, 216] };
}
#[repr(C)]
pub struct IOleInPlaceSiteWindowless {
    pub base__: IOleInPlaceSiteEx,
    pub CanWindowlessActivate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetCapture: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCapture: unsafe extern "system" fn(this: *mut *mut Self, fcapture: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCapture: usize,
    pub GetFocus: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFocus: unsafe extern "system" fn(this: *mut *mut Self, ffocus: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFocus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetDC: unsafe extern "system" fn(this: *mut *mut Self, prect: *const super::super::Foundation::RECT, grfflags: u32, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetDC: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseDC: unsafe extern "system" fn(this: *mut *mut Self, hdc: super::super::Graphics::Gdi::HDC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseDC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InvalidateRect: unsafe extern "system" fn(this: *mut *mut Self, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InvalidateRect: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub InvalidateRgn: unsafe extern "system" fn(this: *mut *mut Self, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    InvalidateRgn: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ScrollRect: unsafe extern "system" fn(this: *mut *mut Self, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ScrollRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AdjustRect: unsafe extern "system" fn(this: *mut *mut Self, prc: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AdjustRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDefWindowMessage: unsafe extern "system" fn(this: *mut *mut Self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDefWindowMessage: usize,
}
impl ::windows_sys::core::Interface for IOleInPlaceSiteWindowless {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2452532640, data2: 13348, data3: 4559, data4: [182, 112, 0, 170, 0, 76, 214, 216] };
}
#[repr(C)]
pub struct IOleInPlaceUIWindow {
    pub base__: IOleWindow,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBorder: unsafe extern "system" fn(this: *mut *mut Self, lprectborder: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBorder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestBorderSpace: unsafe extern "system" fn(this: *mut *mut Self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestBorderSpace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBorderSpace: unsafe extern "system" fn(this: *mut *mut Self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBorderSpace: usize,
    pub SetActiveObject: unsafe extern "system" fn(this: *mut *mut Self, pactiveobject: *mut ::core::ffi::c_void, pszobjname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleInPlaceUIWindow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 277, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleItemContainer {
    pub base__: IOleContainer,
    #[cfg(feature = "Win32_System_Com")]
    pub GetObject: unsafe extern "system" fn(this: *mut *mut Self, pszitem: ::windows_sys::core::PCWSTR, dwspeedneeded: u32, pbc: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetObjectStorage: unsafe extern "system" fn(this: *mut *mut Self, pszitem: ::windows_sys::core::PCWSTR, pbc: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppvstorage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetObjectStorage: usize,
    pub IsRunning: unsafe extern "system" fn(this: *mut *mut Self, pszitem: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleItemContainer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 284, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleLink {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetUpdateOptions: unsafe extern "system" fn(this: *mut *mut Self, dwupdateopt: u32) -> ::windows_sys::core::HRESULT,
    pub GetUpdateOptions: unsafe extern "system" fn(this: *mut *mut Self, pdwupdateopt: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSourceMoniker: unsafe extern "system" fn(this: *mut *mut Self, pmk: *mut ::core::ffi::c_void, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSourceMoniker: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceMoniker: unsafe extern "system" fn(this: *mut *mut Self, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceMoniker: usize,
    pub SetSourceDisplayName: unsafe extern "system" fn(this: *mut *mut Self, pszstatustext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSourceDisplayName: unsafe extern "system" fn(this: *mut *mut Self, ppszdisplayname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub BindToSource: unsafe extern "system" fn(this: *mut *mut Self, bindflags: u32, pbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BindToSource: usize,
    pub BindIfRunning: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetBoundSource: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnbindSource: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, pbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
impl ::windows_sys::core::Interface for IOleLink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 285, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetClientSite: unsafe extern "system" fn(this: *mut *mut Self, pclientsite: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetClientSite: unsafe extern "system" fn(this: *mut *mut Self, ppclientsite: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetHostNames: unsafe extern "system" fn(this: *mut *mut Self, szcontainerapp: ::windows_sys::core::PCWSTR, szcontainerobj: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self, dwsaveoption: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMoniker: unsafe extern "system" fn(this: *mut *mut Self, dwwhichmoniker: u32, pmk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMoniker: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMoniker: unsafe extern "system" fn(this: *mut *mut Self, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMoniker: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InitFromData: unsafe extern "system" fn(this: *mut *mut Self, pdataobject: *mut ::core::ffi::c_void, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InitFromData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClipboardData: unsafe extern "system" fn(this: *mut *mut Self, dwreserved: u32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClipboardData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub DoVerb: unsafe extern "system" fn(this: *mut *mut Self, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: *mut ::core::ffi::c_void, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    DoVerb: usize,
    pub EnumVerbs: unsafe extern "system" fn(this: *mut *mut Self, ppenumoleverb: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub IsUpToDate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetUserClassID: unsafe extern "system" fn(this: *mut *mut Self, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetUserType: unsafe extern "system" fn(this: *mut *mut Self, dwformoftype: u32, pszusertype: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExtent: unsafe extern "system" fn(this: *mut *mut Self, dwdrawaspect: u32, psizel: *const super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExtent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetExtent: unsafe extern "system" fn(this: *mut *mut Self, dwdrawaspect: u32, psizel: *mut super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetExtent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Advise: unsafe extern "system" fn(this: *mut *mut Self, padvsink: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(this: *mut *mut Self, dwconnection: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumAdvise: unsafe extern "system" fn(this: *mut *mut Self, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumAdvise: usize,
    pub GetMiscStatus: unsafe extern "system" fn(this: *mut *mut Self, dwaspect: u32, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetColorScheme: unsafe extern "system" fn(this: *mut *mut Self, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetColorScheme: usize,
}
impl ::windows_sys::core::Interface for IOleObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 274, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IOleParentUndoUnit {
    pub base__: IOleUndoUnit,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, ppuu: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Close: unsafe extern "system" fn(this: *mut *mut Self, ppuu: *mut ::core::ffi::c_void, fcommit: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Close: usize,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, puu: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindUnit: unsafe extern "system" fn(this: *mut *mut Self, puu: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetParentState: unsafe extern "system" fn(this: *mut *mut Self, pdwstate: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleParentUndoUnit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2717578032, data2: 61335, data3: 4558, data4: [155, 201, 0, 170, 0, 96, 142, 1] };
}
#[repr(C)]
pub struct IOleUILinkContainerA {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetNextLink: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32) -> u32,
    pub SetLinkUpdateOptions: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32, dwupdateopt: u32) -> ::windows_sys::core::HRESULT,
    pub GetLinkUpdateOptions: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLinkSource: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32, lpszdisplayname: ::windows_sys::core::PCSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLinkSource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLinkSource: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32, lplpszdisplayname: *mut ::windows_sys::core::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut ::windows_sys::core::PSTR, lplpszshortlinktype: *mut ::windows_sys::core::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLinkSource: usize,
    pub OpenLinkSource: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateLink: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateLink: usize,
    pub CancelLink: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleUILinkContainerA {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IOleUILinkContainerW {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetNextLink: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32) -> u32,
    pub SetLinkUpdateOptions: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32, dwupdateopt: u32) -> ::windows_sys::core::HRESULT,
    pub GetLinkUpdateOptions: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLinkSource: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32, lpszdisplayname: ::windows_sys::core::PCWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLinkSource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLinkSource: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32, lplpszdisplayname: *mut ::windows_sys::core::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut ::windows_sys::core::PWSTR, lplpszshortlinktype: *mut ::windows_sys::core::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLinkSource: usize,
    pub OpenLinkSource: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateLink: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateLink: usize,
    pub CancelLink: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleUILinkContainerW {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IOleUILinkInfoA {
    pub base__: IOleUILinkContainerA,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastUpdate: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastUpdate: usize,
}
impl ::windows_sys::core::Interface for IOleUILinkInfoA {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IOleUILinkInfoW {
    pub base__: IOleUILinkContainerW,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastUpdate: unsafe extern "system" fn(this: *mut *mut Self, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastUpdate: usize,
}
impl ::windows_sys::core::Interface for IOleUILinkInfoW {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IOleUIObjInfoA {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetObjectInfo: unsafe extern "system" fn(this: *mut *mut Self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut ::windows_sys::core::PSTR, lplpsztype: *mut ::windows_sys::core::PSTR, lplpszshorttype: *mut ::windows_sys::core::PSTR, lplpszlocation: *mut ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT,
    pub GetConvertInfo: unsafe extern "system" fn(this: *mut *mut Self, dwobject: u32, lpclassid: *mut ::windows_sys::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows_sys::core::GUID, lplpclsidexclude: *mut *mut ::windows_sys::core::GUID, lpcclsidexclude: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ConvertObject: unsafe extern "system" fn(this: *mut *mut Self, dwobject: u32, clsidnew: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetViewInfo: unsafe extern "system" fn(this: *mut *mut Self, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetViewInfo: unsafe extern "system" fn(this: *mut *mut Self, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetViewInfo: usize,
}
impl ::windows_sys::core::Interface for IOleUIObjInfoA {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IOleUIObjInfoW {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetObjectInfo: unsafe extern "system" fn(this: *mut *mut Self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut ::windows_sys::core::PWSTR, lplpsztype: *mut ::windows_sys::core::PWSTR, lplpszshorttype: *mut ::windows_sys::core::PWSTR, lplpszlocation: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetConvertInfo: unsafe extern "system" fn(this: *mut *mut Self, dwobject: u32, lpclassid: *mut ::windows_sys::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows_sys::core::GUID, lplpclsidexclude: *mut *mut ::windows_sys::core::GUID, lpcclsidexclude: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ConvertObject: unsafe extern "system" fn(this: *mut *mut Self, dwobject: u32, clsidnew: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetViewInfo: unsafe extern "system" fn(this: *mut *mut Self, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetViewInfo: unsafe extern "system" fn(this: *mut *mut Self, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetViewInfo: usize,
}
impl ::windows_sys::core::Interface for IOleUIObjInfoW {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IOleUndoManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, ppuu: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Close: unsafe extern "system" fn(this: *mut *mut Self, ppuu: *mut ::core::ffi::c_void, fcommit: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Close: usize,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, puu: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOpenParentState: unsafe extern "system" fn(this: *mut *mut Self, pdwstate: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DiscardFrom: unsafe extern "system" fn(this: *mut *mut Self, puu: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UndoTo: unsafe extern "system" fn(this: *mut *mut Self, puu: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RedoTo: unsafe extern "system" fn(this: *mut *mut Self, puu: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumUndoable: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumRedoable: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastUndoDescription: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastUndoDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastRedoDescription: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastRedoDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enable: usize,
}
impl ::windows_sys::core::Interface for IOleUndoManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3489788416, data2: 61335, data3: 4558, data4: [155, 201, 0, 170, 0, 96, 142, 1] };
}
#[repr(C)]
pub struct IOleUndoUnit {
    pub base__: ::windows_sys::core::IUnknown,
    pub Do: unsafe extern "system" fn(this: *mut *mut Self, pundomanager: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
    pub GetUnitType: unsafe extern "system" fn(this: *mut *mut Self, pclsid: *mut ::windows_sys::core::GUID, plid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub OnNextAdd: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOleUndoUnit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2303382448, data2: 61335, data3: 4558, data4: [155, 201, 0, 170, 0, 96, 142, 1] };
}
#[repr(C)]
pub struct IOleWindow {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindow: unsafe extern "system" fn(this: *mut *mut Self, phwnd: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ContextSensitiveHelp: unsafe extern "system" fn(this: *mut *mut Self, fentermode: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ContextSensitiveHelp: usize,
}
impl ::windows_sys::core::Interface for IOleWindow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 276, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IParseDisplayName {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub ParseDisplayName: unsafe extern "system" fn(this: *mut *mut Self, pbc: *mut ::core::ffi::c_void, pszdisplayname: ::windows_sys::core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParseDisplayName: usize,
}
impl ::windows_sys::core::Interface for IParseDisplayName {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 282, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IPerPropertyBrowsing {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDisplayString: unsafe extern "system" fn(this: *mut *mut Self, dispid: i32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDisplayString: usize,
    pub MapPropertyToPage: unsafe extern "system" fn(this: *mut *mut Self, dispid: i32, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetPredefinedStrings: unsafe extern "system" fn(this: *mut *mut Self, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetPredefinedValue: unsafe extern "system" fn(this: *mut *mut Self, dispid: i32, dwcookie: u32, pvarout: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetPredefinedValue: usize,
}
impl ::windows_sys::core::Interface for IPerPropertyBrowsing {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 929813418, data2: 14405, data3: 4123, data4: [132, 237, 8, 0, 43, 46, 199, 19] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPersistPropertyBag {
    pub base__: super::Com::IPersist,
    pub InitNew: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Load: unsafe extern "system" fn(this: *mut *mut Self, ppropbag: *mut ::core::ffi::c_void, perrorlog: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Load: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Save: unsafe extern "system" fn(this: *mut *mut Self, ppropbag: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Save: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IPersistPropertyBag {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 936922976, data2: 17099, data3: 4558, data4: [129, 53, 0, 170, 0, 75, 184, 81] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPersistPropertyBag2 {
    pub base__: super::Com::IPersist,
    pub InitNew: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Load: unsafe extern "system" fn(this: *mut *mut Self, ppropbag: *mut ::core::ffi::c_void, perrlog: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Load: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Save: unsafe extern "system" fn(this: *mut *mut Self, ppropbag: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Save: usize,
    pub IsDirty: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IPersistPropertyBag2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 586504321, data2: 10251, data3: 4560, data4: [168, 169, 0, 160, 201, 12, 32, 4] };
}
#[repr(C)]
pub struct IPicture {
    pub base__: ::windows_sys::core::IUnknown,
    pub Handle: unsafe extern "system" fn(this: *mut *mut Self, phandle: *mut u32) -> ::windows_sys::core::HRESULT,
    pub hPal: unsafe extern "system" fn(this: *mut *mut Self, phpal: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, pwidth: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, pheight: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub Render: unsafe extern "system" fn(this: *mut *mut Self, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    Render: usize,
    pub set_hPal: unsafe extern "system" fn(this: *mut *mut Self, hpal: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CurDC: unsafe extern "system" fn(this: *mut *mut Self, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CurDC: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SelectPicture: unsafe extern "system" fn(this: *mut *mut Self, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SelectPicture: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub KeepOriginalFormat: unsafe extern "system" fn(this: *mut *mut Self, pkeep: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    KeepOriginalFormat: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetKeepOriginalFormat: unsafe extern "system" fn(this: *mut *mut Self, keep: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetKeepOriginalFormat: usize,
    pub PictureChanged: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SaveAsFile: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SaveAsFile: usize,
    pub Attributes: unsafe extern "system" fn(this: *mut *mut Self, pdwattr: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPicture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2079852928, data2: 48946, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
}
#[repr(C)]
pub struct IPicture2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub Handle: unsafe extern "system" fn(this: *mut *mut Self, phandle: *mut usize) -> ::windows_sys::core::HRESULT,
    pub hPal: unsafe extern "system" fn(this: *mut *mut Self, phpal: *mut usize) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, pwidth: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, pheight: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub Render: unsafe extern "system" fn(this: *mut *mut Self, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    Render: usize,
    pub set_hPal: unsafe extern "system" fn(this: *mut *mut Self, hpal: usize) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CurDC: unsafe extern "system" fn(this: *mut *mut Self, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CurDC: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SelectPicture: unsafe extern "system" fn(this: *mut *mut Self, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SelectPicture: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub KeepOriginalFormat: unsafe extern "system" fn(this: *mut *mut Self, pkeep: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    KeepOriginalFormat: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetKeepOriginalFormat: unsafe extern "system" fn(this: *mut *mut Self, keep: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetKeepOriginalFormat: usize,
    pub PictureChanged: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SaveAsFile: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SaveAsFile: usize,
    pub Attributes: unsafe extern "system" fn(this: *mut *mut Self, pdwattr: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPicture2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4112014808, data2: 8210, data3: 19211, data4: [170, 217, 240, 82, 198, 189, 72, 43] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPictureDisp {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IPictureDisp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2079852929, data2: 48946, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
}
#[repr(C)]
pub struct IPointerInactive {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetActivationPolicy: unsafe extern "system" fn(this: *mut *mut Self, pdwpolicy: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnInactiveMouseMove: unsafe extern "system" fn(this: *mut *mut Self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnInactiveMouseMove: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnInactiveSetCursor: unsafe extern "system" fn(this: *mut *mut Self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnInactiveSetCursor: usize,
}
impl ::windows_sys::core::Interface for IPointerInactive {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1436027808, data2: 13738, data3: 4559, data4: [182, 113, 0, 170, 0, 76, 214, 216] };
}
#[repr(C)]
pub struct IPrint {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetInitialPageNum: unsafe extern "system" fn(this: *mut *mut Self, nfirstpage: i32) -> ::windows_sys::core::HRESULT,
    pub GetPageInfo: unsafe extern "system" fn(this: *mut *mut Self, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub Print: unsafe extern "system" fn(this: *mut *mut Self, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: *mut ::core::ffi::c_void, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    Print: usize,
}
impl ::windows_sys::core::Interface for IPrint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3072507081, data2: 20072, data3: 4123, data4: [162, 188, 0, 170, 0, 64, 71, 112] };
}
#[repr(C)]
pub struct IPropertyNotifySink {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnChanged: unsafe extern "system" fn(this: *mut *mut Self, dispid: i32) -> ::windows_sys::core::HRESULT,
    pub OnRequestEdit: unsafe extern "system" fn(this: *mut *mut Self, dispid: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPropertyNotifySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2616966146, data2: 61425, data3: 4122, data4: [132, 237, 0, 170, 0, 52, 29, 7] };
}
#[repr(C)]
pub struct IPropertyPage {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetPageSite: unsafe extern "system" fn(this: *mut *mut Self, ppagesite: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Activate: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Activate: usize,
    pub Deactivate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPageInfo: unsafe extern "system" fn(this: *mut *mut Self, ppageinfo: *mut PROPPAGEINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPageInfo: usize,
    pub SetObjects: unsafe extern "system" fn(this: *mut *mut Self, cobjects: u32, ppunk: *const *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, ncmdshow: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Move: unsafe extern "system" fn(this: *mut *mut Self, prect: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Move: usize,
    pub IsPageDirty: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Apply: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Help: unsafe extern "system" fn(this: *mut *mut Self, pszhelpdir: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TranslateAccelerator: unsafe extern "system" fn(this: *mut *mut Self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TranslateAccelerator: usize,
}
impl ::windows_sys::core::Interface for IPropertyPage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2979443341, data2: 47796, data3: 4122, data4: [182, 156, 0, 170, 0, 52, 29, 7] };
}
#[repr(C)]
pub struct IPropertyPage2 {
    pub base__: IPropertyPage,
    pub EditProperty: unsafe extern "system" fn(this: *mut *mut Self, dispid: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPropertyPage2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 31737445, data2: 9388, data3: 4123, data4: [132, 237, 8, 0, 43, 46, 199, 19] };
}
#[repr(C)]
pub struct IPropertyPageSite {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnStatusChange: unsafe extern "system" fn(this: *mut *mut Self, dwflags: PROPPAGESTATUS) -> ::windows_sys::core::HRESULT,
    pub GetLocaleID: unsafe extern "system" fn(this: *mut *mut Self, plocaleid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPageContainer: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TranslateAccelerator: unsafe extern "system" fn(this: *mut *mut Self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TranslateAccelerator: usize,
}
impl ::windows_sys::core::Interface for IPropertyPageSite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2979443340, data2: 47796, data3: 4122, data4: [182, 156, 0, 170, 0, 52, 29, 7] };
}
#[repr(C)]
pub struct IProtectFocus {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowFocusChange: unsafe extern "system" fn(this: *mut *mut Self, pfallow: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowFocusChange: usize,
}
impl ::windows_sys::core::Interface for IProtectFocus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3625947299, data2: 33110, data3: 17655, data4: [173, 40, 90, 187, 135, 0, 50, 116] };
}
#[repr(C)]
pub struct IProtectedModeMenuServices {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub CreateMenu: unsafe extern "system" fn(this: *mut *mut Self, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    CreateMenu: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub LoadMenu: unsafe extern "system" fn(this: *mut *mut Self, pszmodulename: ::windows_sys::core::PCWSTR, pszmenuname: ::windows_sys::core::PCWSTR, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    LoadMenu: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub LoadMenuID: unsafe extern "system" fn(this: *mut *mut Self, pszmodulename: ::windows_sys::core::PCWSTR, wresourceid: u16, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    LoadMenuID: usize,
}
impl ::windows_sys::core::Interface for IProtectedModeMenuServices {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1942029806, data2: 40447, data3: 18951, data4: [184, 60, 126, 255, 41, 12, 38, 110] };
}
#[repr(C)]
pub struct IProvideClassInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClassInfo: unsafe extern "system" fn(this: *mut *mut Self, ppti: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClassInfo: usize,
}
impl ::windows_sys::core::Interface for IProvideClassInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2979443331, data2: 47796, data3: 4122, data4: [182, 156, 0, 170, 0, 52, 29, 7] };
}
#[repr(C)]
pub struct IProvideClassInfo2 {
    pub base__: IProvideClassInfo,
    pub GetGUID: unsafe extern "system" fn(this: *mut *mut Self, dwguidkind: u32, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProvideClassInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2797353664, data2: 56234, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
}
#[repr(C)]
pub struct IProvideMultipleClassInfo {
    pub base__: IProvideClassInfo2,
    pub GetMultiTypeInfoCount: unsafe extern "system" fn(this: *mut *mut Self, pcti: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInfoOfIndex: unsafe extern "system" fn(this: *mut *mut Self, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut *mut ::core::ffi::c_void, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows_sys::core::GUID, piidsource: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInfoOfIndex: usize,
}
impl ::windows_sys::core::Interface for IProvideMultipleClassInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2813045185, data2: 35203, data3: 4559, data4: [143, 32, 0, 128, 95, 44, 208, 100] };
}
#[repr(C)]
pub struct IProvideRuntimeContext {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrentSourceContext: unsafe extern "system" fn(this: *mut *mut Self, pdwcontext: *mut usize, pfexecutingglobalcode: *mut i16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProvideRuntimeContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 283263306, data2: 60505, data3: 18898, data4: [188, 81, 90, 221, 44, 54, 254, 188] };
}
#[repr(C)]
pub struct IQuickActivate {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub QuickActivate: unsafe extern "system" fn(this: *mut *mut Self, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    QuickActivate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContentExtent: unsafe extern "system" fn(this: *mut *mut Self, psizel: *const super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContentExtent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetContentExtent: unsafe extern "system" fn(this: *mut *mut Self, psizel: *mut super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetContentExtent: usize,
}
impl ::windows_sys::core::Interface for IQuickActivate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3478252816, data2: 25342, data3: 4559, data4: [191, 134, 0, 160, 201, 3, 72, 54] };
}
#[repr(C)]
pub struct IRecordInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub RecordInit: unsafe extern "system" fn(this: *mut *mut Self, pvnew: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RecordClear: unsafe extern "system" fn(this: *mut *mut Self, pvexisting: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RecordCopy: unsafe extern "system" fn(this: *mut *mut Self, pvexisting: *const ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetGuid: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetName: usize,
    pub GetSize: unsafe extern "system" fn(this: *mut *mut Self, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTypeInfo: unsafe extern "system" fn(this: *mut *mut Self, pptypeinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTypeInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetField: unsafe extern "system" fn(this: *mut *mut Self, pvdata: *const ::core::ffi::c_void, szfieldname: ::windows_sys::core::PCWSTR, pvarfield: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetField: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFieldNoCopy: unsafe extern "system" fn(this: *mut *mut Self, pvdata: *const ::core::ffi::c_void, szfieldname: ::windows_sys::core::PCWSTR, pvarfield: *mut super::Com::VARIANT, ppvdatacarray: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFieldNoCopy: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub PutField: unsafe extern "system" fn(this: *mut *mut Self, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: ::windows_sys::core::PCWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    PutField: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub PutFieldNoCopy: unsafe extern "system" fn(this: *mut *mut Self, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: ::windows_sys::core::PCWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    PutFieldNoCopy: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFieldNames: unsafe extern "system" fn(this: *mut *mut Self, pcnames: *mut u32, rgbstrnames: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFieldNames: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMatchingType: unsafe extern "system" fn(this: *mut *mut Self, precordinfo: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMatchingType: usize,
    pub RecordCreate: unsafe extern "system" fn(this: *mut *mut Self) -> *mut ::core::ffi::c_void,
    pub RecordCreateCopy: unsafe extern "system" fn(this: *mut *mut Self, pvsource: *const ::core::ffi::c_void, ppvdest: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RecordDestroy: unsafe extern "system" fn(this: *mut *mut Self, pvrecord: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRecordInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 47, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct ISimpleFrameSite {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub PreMessageFilter: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreMessageFilter: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostMessageFilter: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PostMessageFilter: usize,
}
impl ::windows_sys::core::Interface for ISimpleFrameSite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1948978689, data2: 5350, data3: 4123, data4: [145, 78, 0, 170, 0, 48, 12, 171] };
}
#[repr(C)]
pub struct ISpecifyPropertyPages {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPages: unsafe extern "system" fn(this: *mut *mut Self, ppages: *mut CAUUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpecifyPropertyPages {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2979443339, data2: 47796, data3: 4122, data4: [182, 156, 0, 170, 0, 52, 29, 7] };
}
#[repr(C)]
pub struct ITypeChangeEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub RequestTypeChange: unsafe extern "system" fn(this: *mut *mut Self, changekind: CHANGEKIND, ptinfobefore: *mut ::core::ffi::c_void, pstrname: ::windows_sys::core::PCWSTR, pfcancel: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RequestTypeChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AfterTypeChange: unsafe extern "system" fn(this: *mut *mut Self, changekind: CHANGEKIND, ptinfoafter: *mut ::core::ffi::c_void, pstrname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AfterTypeChange: usize,
}
impl ::windows_sys::core::Interface for ITypeChangeEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 132112, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct ITypeFactory {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFromTypeInfo: unsafe extern "system" fn(this: *mut *mut Self, ptypeinfo: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFromTypeInfo: usize,
}
impl ::windows_sys::core::Interface for ITypeFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 46, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct ITypeMarshal {
    pub base__: ::windows_sys::core::IUnknown,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, psize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Marshal: unsafe extern "system" fn(this: *mut *mut Self, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Unmarshal: unsafe extern "system" fn(this: *mut *mut Self, pvtype: *mut ::core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Free: unsafe extern "system" fn(this: *mut *mut Self, pvtype: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITypeMarshal {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 45, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IVBFormat {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, vdata: *mut super::Com::VARIANT, bstrformat: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpbuffer: *mut ::core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Format: usize,
}
impl ::windows_sys::core::Interface for IVBFormat {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2554985824, data2: 14184, data3: 4123, data4: [141, 114, 174, 97, 100, 255, 227, 207] };
}
#[repr(C)]
pub struct IVBGetControl {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumControls: unsafe extern "system" fn(this: *mut *mut Self, dwolecontf: OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS, ppenumunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumControls: usize,
}
impl ::windows_sys::core::Interface for IVBGetControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1084248224, data2: 15409, data3: 4123, data4: [168, 46, 8, 0, 43, 43, 35, 55] };
}
#[repr(C)]
pub struct IVariantChangeType {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ChangeType: unsafe extern "system" fn(this: *mut *mut Self, pvardst: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, lcid: u32, vtnew: u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ChangeType: usize,
}
impl ::windows_sys::core::Interface for IVariantChangeType {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2800719970, data2: 50976, data3: 4560, data4: [147, 55, 0, 160, 201, 13, 202, 169] };
}
#[repr(C)]
pub struct IViewObject {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub Draw: unsafe extern "system" fn(this: *mut *mut Self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    Draw: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub GetColorSet: unsafe extern "system" fn(this: *mut *mut Self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    GetColorSet: usize,
    pub Freeze: unsafe extern "system" fn(this: *mut *mut Self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Unfreeze: unsafe extern "system" fn(this: *mut *mut Self, dwfreeze: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAdvise: unsafe extern "system" fn(this: *mut *mut Self, aspects: u32, advf: u32, padvsink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAdvise: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAdvise: unsafe extern "system" fn(this: *mut *mut Self, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAdvise: usize,
}
impl ::windows_sys::core::Interface for IViewObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 269, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IViewObject2 {
    pub base__: IViewObject,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetExtent: unsafe extern "system" fn(this: *mut *mut Self, dwdrawaspect: u32, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, lpsizel: *mut super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetExtent: usize,
}
impl ::windows_sys::core::Interface for IViewObject2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 295, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IViewObjectEx {
    pub base__: IViewObject2,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRect: unsafe extern "system" fn(this: *mut *mut Self, dwaspect: u32, prect: *mut super::super::Foundation::RECTL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRect: usize,
    pub GetViewStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryHitPoint: unsafe extern "system" fn(this: *mut *mut Self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: super::super::Foundation::POINT, lclosehint: i32, phitresult: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryHitPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryHitRect: unsafe extern "system" fn(this: *mut *mut Self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32, phitresult: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryHitRect: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub GetNaturalExtent: unsafe extern "system" fn(this: *mut *mut Self, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const ExtentInfo, psizel: *mut super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    GetNaturalExtent: usize,
}
impl ::windows_sys::core::Interface for IViewObjectEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 988955282, data2: 3222, data3: 4558, data4: [160, 207, 0, 170, 0, 96, 10, 184] };
}
#[repr(C)]
pub struct IZoomEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnZoomPercentChanged: unsafe extern "system" fn(this: *mut *mut Self, ulzoompercent: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IZoomEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1102479696, data2: 36940, data3: 19991, data4: [160, 186, 164, 56, 24, 46, 53, 157] };
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type LIBFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LIBFLAG_FRESTRICTED: LIBFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LIBFLAG_FCONTROL: LIBFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LIBFLAG_FHIDDEN: LIBFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LIBFLAG_FHASDISKIMAGE: LIBFLAGS = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LICINFO {
    pub cbLicInfo: i32,
    pub fRuntimeKeyAvail: super::super::Foundation::BOOL,
    pub fLicVerified: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LICINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LICINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LOAD_TLB_AS_32BIT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LOAD_TLB_AS_64BIT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LOCALE_USE_NLS: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNOLEUIHOOK = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::WPARAM, param3: super::super::Foundation::LPARAM) -> u32>;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LP_COLOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LP_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LP_MONOCHROME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LP_VGACOLOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type MEDIAPLAYBACK_STATE = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MEDIAPLAYBACK_RESUME: MEDIAPLAYBACK_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MEDIAPLAYBACK_PAUSE: MEDIAPLAYBACK_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MEDIAPLAYBACK_PAUSE_AND_SUSPEND: MEDIAPLAYBACK_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MEDIAPLAYBACK_RESUME_FROM_SUSPEND: MEDIAPLAYBACK_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MEMBERID_NIL: i32 = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct METHODDATA {
    pub szName: ::windows_sys::core::PWSTR,
    pub ppdata: *mut PARAMDATA,
    pub dispid: i32,
    pub iMeth: u32,
    pub cc: super::Com::CALLCONV,
    pub cArgs: u32,
    pub wFlags: u16,
    pub vtReturn: u16,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for METHODDATA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for METHODDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MK_ALT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MSOCMDERR_E_CANCELED: i32 = -2147221245i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MSOCMDERR_E_DISABLED: i32 = -2147221247i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MSOCMDERR_E_FIRST: i32 = -2147221248i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MSOCMDERR_E_NOHELP: i32 = -2147221246i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MSOCMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MSOCMDERR_E_UNKNOWNGROUP: i32 = -2147221244i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type MULTICLASSINFO_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MULTICLASSINFO_GETTYPEINFO: MULTICLASSINFO_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MULTICLASSINFO_GETNUMRESERVEDDISPIDS: MULTICLASSINFO_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MULTICLASSINFO_GETIIDPRIMARY: MULTICLASSINFO_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MULTICLASSINFO_GETIIDSOURCE: MULTICLASSINFO_FLAGS = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct NUMPARSE {
    pub cDig: i32,
    pub dwInFlags: u32,
    pub dwOutFlags: u32,
    pub cchUsed: i32,
    pub nBaseShift: i32,
    pub nPwr10: i32,
}
impl ::core::marker::Copy for NUMPARSE {}
impl ::core::clone::Clone for NUMPARSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_CURRENCY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_DECIMAL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_EXPONENT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_HEX_OCT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_INEXACT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_LEADING_MINUS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_LEADING_PLUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_LEADING_WHITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_NEG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_PARENS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_STD: u32 = 8191u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_THOUSANDS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_TRAILING_MINUS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_TRAILING_PLUS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_TRAILING_WHITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_USE_ALL: u32 = 4096u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OBJECTDESCRIPTOR {
    pub cbSize: u32,
    pub clsid: ::windows_sys::core::GUID,
    pub dwDrawAspect: u32,
    pub sizel: super::super::Foundation::SIZE,
    pub pointl: super::super::Foundation::POINTL,
    pub dwStatus: u32,
    pub dwFullUserTypeName: u32,
    pub dwSrcOfCopy: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OBJECTDESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OBJECTDESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OCM__BASE: u32 = 8192u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OCPFIPARAMS {
    pub cbStructSize: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub x: i32,
    pub y: i32,
    pub lpszCaption: ::windows_sys::core::PCWSTR,
    pub cObjects: u32,
    pub lplpUnk: *mut *mut *mut ::windows_sys::core::IUnknown,
    pub cPages: u32,
    pub lpPages: *mut ::windows_sys::core::GUID,
    pub lcid: u32,
    pub dispidInitialProperty: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OCPFIPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OCPFIPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OF_GET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OF_HANDLER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OF_SET: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OIFI {
    pub cb: u32,
    pub fMDIApp: super::super::Foundation::BOOL,
    pub hwndFrame: super::super::Foundation::HWND,
    pub haccel: super::super::UI::WindowsAndMessaging::HACCEL,
    pub cAccelEntries: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OIFI {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OIFI {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLECLOSE = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECLOSE_SAVEIFDIRTY: OLECLOSE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECLOSE_NOSAVE: OLECLOSE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECLOSE_PROMPTSAVE: OLECLOSE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct OLECMD {
    pub cmdID: u32,
    pub cmdf: u32,
}
impl ::core::marker::Copy for OLECMD {}
impl ::core::clone::Clone for OLECMD {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_CLSID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_DISPLAYNAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_INSTALLSCOPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_PUBLISHER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_SOURCEURL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_HWND: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_X: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_Y: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDERR_E_CANCELED: ::windows_sys::core::HRESULT = -2147221245i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDERR_E_DISABLED: ::windows_sys::core::HRESULT = -2147221247i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDERR_E_FIRST: ::windows_sys::core::HRESULT = -2147221248i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDERR_E_NOHELP: ::windows_sys::core::HRESULT = -2147221246i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDERR_E_UNKNOWNGROUP: ::windows_sys::core::HRESULT = -2147221244i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLECMDEXECOPT = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDEXECOPT_DODEFAULT: OLECMDEXECOPT = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDEXECOPT_PROMPTUSER: OLECMDEXECOPT = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDEXECOPT_DONTPROMPTUSER: OLECMDEXECOPT = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDEXECOPT_SHOWHELP: OLECMDEXECOPT = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLECMDF = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDF_SUPPORTED: OLECMDF = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDF_ENABLED: OLECMDF = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDF_LATCHED: OLECMDF = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDF_NINCHED: OLECMDF = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDF_INVISIBLE: OLECMDF = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDF_DEFHIDEONCTXTMENU: OLECMDF = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLECMDID = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_OPEN: OLECMDID = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_NEW: OLECMDID = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SAVE: OLECMDID = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SAVEAS: OLECMDID = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SAVECOPYAS: OLECMDID = 5i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PRINT: OLECMDID = 6i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PRINTPREVIEW: OLECMDID = 7i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PAGESETUP: OLECMDID = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SPELL: OLECMDID = 9i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PROPERTIES: OLECMDID = 10i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_CUT: OLECMDID = 11i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_COPY: OLECMDID = 12i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PASTE: OLECMDID = 13i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PASTESPECIAL: OLECMDID = 14i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UNDO: OLECMDID = 15i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_REDO: OLECMDID = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SELECTALL: OLECMDID = 17i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_CLEARSELECTION: OLECMDID = 18i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ZOOM: OLECMDID = 19i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_GETZOOMRANGE: OLECMDID = 20i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UPDATECOMMANDS: OLECMDID = 21i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_REFRESH: OLECMDID = 22i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_STOP: OLECMDID = 23i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_HIDETOOLBARS: OLECMDID = 24i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETPROGRESSMAX: OLECMDID = 25i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETPROGRESSPOS: OLECMDID = 26i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETPROGRESSTEXT: OLECMDID = 27i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETTITLE: OLECMDID = 28i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETDOWNLOADSTATE: OLECMDID = 29i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_STOPDOWNLOAD: OLECMDID = 30i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ONTOOLBARACTIVATED: OLECMDID = 31i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_FIND: OLECMDID = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_DELETE: OLECMDID = 33i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_HTTPEQUIV: OLECMDID = 34i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_HTTPEQUIV_DONE: OLECMDID = 35i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ENABLE_INTERACTION: OLECMDID = 36i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ONUNLOAD: OLECMDID = 37i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PROPERTYBAG2: OLECMDID = 38i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PREREFRESH: OLECMDID = 39i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWSCRIPTERROR: OLECMDID = 40i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWMESSAGE: OLECMDID = 41i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWFIND: OLECMDID = 42i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWPAGESETUP: OLECMDID = 43i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWPRINT: OLECMDID = 44i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_CLOSE: OLECMDID = 45i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ALLOWUILESSSAVEAS: OLECMDID = 46i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_DONTDOWNLOADCSS: OLECMDID = 47i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UPDATEPAGESTATUS: OLECMDID = 48i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PRINT2: OLECMDID = 49i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PRINTPREVIEW2: OLECMDID = 50i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETPRINTTEMPLATE: OLECMDID = 51i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_GETPRINTTEMPLATE: OLECMDID = 52i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PAGEACTIONBLOCKED: OLECMDID = 55i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PAGEACTIONUIQUERY: OLECMDID = 56i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_FOCUSVIEWCONTROLS: OLECMDID = 57i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_FOCUSVIEWCONTROLSQUERY: OLECMDID = 58i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWPAGEACTIONMENU: OLECMDID = 59i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ADDTRAVELENTRY: OLECMDID = 60i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UPDATETRAVELENTRY: OLECMDID = 61i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UPDATEBACKFORWARDSTATE: OLECMDID = 62i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_OPTICAL_ZOOM: OLECMDID = 63i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_OPTICAL_GETZOOMRANGE: OLECMDID = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_WINDOWSTATECHANGED: OLECMDID = 65i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ACTIVEXINSTALLSCOPE: OLECMDID = 66i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UPDATETRAVELENTRY_DATARECOVERY: OLECMDID = 67i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWTASKDLG: OLECMDID = 68i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_POPSTATEEVENT: OLECMDID = 69i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_VIEWPORT_MODE: OLECMDID = 70i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_LAYOUT_VIEWPORT_WIDTH: OLECMDID = 71i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_VISUAL_VIEWPORT_EXCLUDE_BOTTOM: OLECMDID = 72i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_USER_OPTICAL_ZOOM: OLECMDID = 73i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PAGEAVAILABLE: OLECMDID = 74i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_GETUSERSCALABLE: OLECMDID = 75i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UPDATE_CARET: OLECMDID = 76i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ENABLE_VISIBILITY: OLECMDID = 77i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_MEDIA_PLAYBACK: OLECMDID = 78i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETFAVICON: OLECMDID = 79i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SET_HOST_FULLSCREENMODE: OLECMDID = 80i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_EXITFULLSCREEN: OLECMDID = 81i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SCROLLCOMPLETE: OLECMDID = 82i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ONBEFOREUNLOAD: OLECMDID = 83i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWMESSAGE_BLOCKABLE: OLECMDID = 84i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWTASKDLG_BLOCKABLE: OLECMDID = 85i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLECMDID_BROWSERSTATEFLAG = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_EXTENSIONSOFF: OLECMDID_BROWSERSTATEFLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_IESECURITY: OLECMDID_BROWSERSTATEFLAG = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_PROTECTEDMODE_OFF: OLECMDID_BROWSERSTATEFLAG = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_RESET: OLECMDID_BROWSERSTATEFLAG = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_REQUIRESACTIVEX: OLECMDID_BROWSERSTATEFLAG = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_DESKTOPHTMLDIALOG: OLECMDID_BROWSERSTATEFLAG = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_BLOCKEDVERSION: OLECMDID_BROWSERSTATEFLAG = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLECMDID_OPTICAL_ZOOMFLAG = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_OPTICAL_ZOOM_NOPERSIST: OLECMDID_OPTICAL_ZOOMFLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_OPTICAL_ZOOM_NOLAYOUT: OLECMDID_OPTICAL_ZOOMFLAG = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_OPTICAL_ZOOM_NOTRANSIENT: OLECMDID_OPTICAL_ZOOMFLAG = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_OPTICAL_ZOOM_RELOADFORNEWTAB: OLECMDID_OPTICAL_ZOOMFLAG = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLECMDID_PAGEACTIONFLAG = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_FILEDOWNLOAD: OLECMDID_PAGEACTIONFLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEXTRUSTFAIL: OLECMDID_PAGEACTIONFLAG = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERDISABLE: OLECMDID_PAGEACTIONFLAG = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEXDISALLOW: OLECMDID_PAGEACTIONFLAG = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEXUNSAFE: OLECMDID_PAGEACTIONFLAG = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_POPUPWINDOW: OLECMDID_PAGEACTIONFLAG = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_LOCALMACHINE: OLECMDID_PAGEACTIONFLAG = 128i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_MIMETEXTPLAIN: OLECMDID_PAGEACTIONFLAG = 256i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE: OLECMDID_PAGEACTIONFLAG = 512i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = 512i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_PAGEACTIONFLAG = 1024i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_PAGEACTIONFLAG = 2048i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_PAGEACTIONFLAG = 4096i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_PAGEACTIONFLAG = 8192i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_PAGEACTIONFLAG = 16384i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNDENY: OLECMDID_PAGEACTIONFLAG = 32768i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_POPUPALLOWED: OLECMDID_PAGEACTIONFLAG = 65536i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_SCRIPTPROMPT: OLECMDID_PAGEACTIONFLAG = 131072i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = 262144i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_MIXEDCONTENT: OLECMDID_PAGEACTIONFLAG = 524288i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_INVALID_CERT: OLECMDID_PAGEACTIONFLAG = 1048576i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_INTRANETZONEREQUEST: OLECMDID_PAGEACTIONFLAG = 2097152i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_XSSFILTERED: OLECMDID_PAGEACTIONFLAG = 4194304i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_SPOOFABLEIDNHOST: OLECMDID_PAGEACTIONFLAG = 8388608i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEX_EPM_INCOMPATIBLE: OLECMDID_PAGEACTIONFLAG = 16777216i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = 33554432i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED: OLECMDID_PAGEACTIONFLAG = 67108864i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED_ACTIVEX: OLECMDID_PAGEACTIONFLAG = 134217728i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_EXTENSION_COMPAT_BLOCKED: OLECMDID_PAGEACTIONFLAG = 268435456i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_NORESETACTIVEX: OLECMDID_PAGEACTIONFLAG = 536870912i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_GENERIC_STATE: OLECMDID_PAGEACTIONFLAG = 1073741824i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_RESET: OLECMDID_PAGEACTIONFLAG = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLECMDID_REFRESHFLAG = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_NORMAL: OLECMDID_REFRESHFLAG = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_IFEXPIRED: OLECMDID_REFRESHFLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_CONTINUE: OLECMDID_REFRESHFLAG = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_COMPLETELY: OLECMDID_REFRESHFLAG = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_NO_CACHE: OLECMDID_REFRESHFLAG = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_RELOAD: OLECMDID_REFRESHFLAG = 5i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_LEVELMASK: OLECMDID_REFRESHFLAG = 255i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_CLEARUSERINPUT: OLECMDID_REFRESHFLAG = 4096i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PROMPTIFOFFLINE: OLECMDID_REFRESHFLAG = 8192i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_THROUGHSCRIPT: OLECMDID_REFRESHFLAG = 16384i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_SKIPBEFOREUNLOADEVENT: OLECMDID_REFRESHFLAG = 32768i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_ACTIVEXINSTALL: OLECMDID_REFRESHFLAG = 65536i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_FILEDOWNLOAD: OLECMDID_REFRESHFLAG = 131072i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_LOCALMACHINE: OLECMDID_REFRESHFLAG = 262144i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_POPUPWINDOW: OLECMDID_REFRESHFLAG = 524288i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_REFRESHFLAG = 1048576i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_REFRESHFLAG = 2097152i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_REFRESHFLAG = 4194304i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_REFRESHFLAG = 8388608i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_REFRESHFLAG = 16777216i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_MIXEDCONTENT: OLECMDID_REFRESHFLAG = 33554432i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_INVALID_CERT: OLECMDID_REFRESHFLAG = 67108864i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_ALLOW_VERSION: OLECMDID_REFRESHFLAG = 134217728i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLECMDID_VIEWPORT_MODE_FLAG = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH: OLECMDID_VIEWPORT_MODE_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM: OLECMDID_VIEWPORT_MODE_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH_VALID: OLECMDID_VIEWPORT_MODE_FLAG = 65536i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM_VALID: OLECMDID_VIEWPORT_MODE_FLAG = 131072i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLECMDID_WINDOWSTATE_FLAG = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE: OLECMDID_WINDOWSTATE_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_WINDOWSTATE_ENABLED: OLECMDID_WINDOWSTATE_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE_VALID: OLECMDID_WINDOWSTATE_FLAG = 65536i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_WINDOWSTATE_ENABLED_VALID: OLECMDID_WINDOWSTATE_FLAG = 131072i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct OLECMDTEXT {
    pub cmdtextf: u32,
    pub cwActual: u32,
    pub cwBuf: u32,
    pub rgwz: [u16; 1],
}
impl ::core::marker::Copy for OLECMDTEXT {}
impl ::core::clone::Clone for OLECMDTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLECMDTEXTF = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDTEXTF_NONE: OLECMDTEXTF = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDTEXTF_NAME: OLECMDTEXTF = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDTEXTF_STATUS: OLECMDTEXTF = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMD_TASKDLGID_ONBEFOREUNLOAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLECONTF = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECONTF_EMBEDDINGS: OLECONTF = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECONTF_LINKS: OLECONTF = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECONTF_OTHERS: OLECONTF = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECONTF_ONLYUSER: OLECONTF = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECONTF_ONLYIFRUNNING: OLECONTF = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECREATE_LEAVERUNNING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLEDCFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEDC_NODRAW: OLEDCFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEDC_PAINTBKGND: OLEDCFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEDC_OFFSCREEN: OLEDCFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLEGETMONIKER = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEGETMONIKER_ONLYIFTHERE: OLEGETMONIKER = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEGETMONIKER_FORCEASSIGN: OLEGETMONIKER = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEGETMONIKER_UNASSIGN: OLEGETMONIKER = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEGETMONIKER_TEMPFORUSER: OLEGETMONIKER = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_DISCARDUNDOSTATE: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_HIDE: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_INPLACEACTIVATE: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_OPEN: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_PRIMARY: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_PROPERTIES: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_SHOW: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_UIACTIVATE: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLELINKBIND = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLELINKBIND_EVENIFCLASSDIFF: OLELINKBIND = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLEMISC = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_RECOMPOSEONRESIZE: OLEMISC = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ONLYICONIC: OLEMISC = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_INSERTNOTREPLACE: OLEMISC = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_STATIC: OLEMISC = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_CANTLINKINSIDE: OLEMISC = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_CANLINKBYOLE1: OLEMISC = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ISLINKOBJECT: OLEMISC = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_INSIDEOUT: OLEMISC = 128i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ACTIVATEWHENVISIBLE: OLEMISC = 256i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_RENDERINGISDEVICEINDEPENDENT: OLEMISC = 512i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_INVISIBLEATRUNTIME: OLEMISC = 1024i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ALWAYSRUN: OLEMISC = 2048i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ACTSLIKEBUTTON: OLEMISC = 4096i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ACTSLIKELABEL: OLEMISC = 8192i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_NOUIACTIVATE: OLEMISC = 16384i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ALIGNABLE: OLEMISC = 32768i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_SIMPLEFRAME: OLEMISC = 65536i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_SETCLIENTSITEFIRST: OLEMISC = 131072i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_IMEMODE: OLEMISC = 262144i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_IGNOREACTIVATEWHENVISIBLE: OLEMISC = 524288i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_WANTSTOMENUMERGE: OLEMISC = 1048576i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_SUPPORTSMULTILEVELUNDO: OLEMISC = 2097152i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLERENDER = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLERENDER_NONE: OLERENDER = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLERENDER_DRAW: OLERENDER = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLERENDER_FORMAT: OLERENDER = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLERENDER_ASIS: OLERENDER = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLESTDDELIM: &str = "\\";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Media\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
pub struct OLEUIBUSYA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hTask: super::super::Media::HTASK,
    pub lphWndDialog: *mut super::super::Foundation::HWND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::marker::Copy for OLEUIBUSYA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::clone::Clone for OLEUIBUSYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Media\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
pub struct OLEUIBUSYW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hTask: super::super::Media::HTASK,
    pub lphWndDialog: *mut super::super::Foundation::HWND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::marker::Copy for OLEUIBUSYW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::clone::Clone for OLEUIBUSYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OLEUICHANGEICONA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hMetaPict: isize,
    pub clsid: ::windows_sys::core::GUID,
    pub szIconExe: [super::super::Foundation::CHAR; 260],
    pub cchIconExe: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OLEUICHANGEICONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OLEUICHANGEICONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OLEUICHANGEICONW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hMetaPict: isize,
    pub clsid: ::windows_sys::core::GUID,
    pub szIconExe: [u16; 260],
    pub cchIconExe: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OLEUICHANGEICONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OLEUICHANGEICONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
pub struct OLEUICHANGESOURCEA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOFN: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEA,
    pub dwReserved1: [u32; 4],
    pub lpOleUILinkContainer: *mut *mut *mut *mut IOleUILinkContainerA,
    pub dwLink: u32,
    pub lpszDisplayName: ::windows_sys::core::PSTR,
    pub nFileLength: u32,
    pub lpszFrom: ::windows_sys::core::PSTR,
    pub lpszTo: ::windows_sys::core::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
impl ::core::marker::Copy for OLEUICHANGESOURCEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
impl ::core::clone::Clone for OLEUICHANGESOURCEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
pub struct OLEUICHANGESOURCEW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOFN: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEW,
    pub dwReserved1: [u32; 4],
    pub lpOleUILinkContainer: *mut *mut *mut *mut IOleUILinkContainerW,
    pub dwLink: u32,
    pub lpszDisplayName: ::windows_sys::core::PWSTR,
    pub nFileLength: u32,
    pub lpszFrom: ::windows_sys::core::PWSTR,
    pub lpszTo: ::windows_sys::core::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
impl ::core::marker::Copy for OLEUICHANGESOURCEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
impl ::core::clone::Clone for OLEUICHANGESOURCEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OLEUICONVERTA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: ::windows_sys::core::GUID,
    pub clsidConvertDefault: ::windows_sys::core::GUID,
    pub clsidActivateDefault: ::windows_sys::core::GUID,
    pub clsidNew: ::windows_sys::core::GUID,
    pub dvAspect: u32,
    pub wFormat: u16,
    pub fIsLinkedObject: super::super::Foundation::BOOL,
    pub hMetaPict: isize,
    pub lpszUserType: ::windows_sys::core::PSTR,
    pub fObjectsIconChanged: super::super::Foundation::BOOL,
    pub lpszDefLabel: ::windows_sys::core::PSTR,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OLEUICONVERTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OLEUICONVERTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OLEUICONVERTW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: ::windows_sys::core::GUID,
    pub clsidConvertDefault: ::windows_sys::core::GUID,
    pub clsidActivateDefault: ::windows_sys::core::GUID,
    pub clsidNew: ::windows_sys::core::GUID,
    pub dvAspect: u32,
    pub wFormat: u16,
    pub fIsLinkedObject: super::super::Foundation::BOOL,
    pub hMetaPict: isize,
    pub lpszUserType: ::windows_sys::core::PWSTR,
    pub fObjectsIconChanged: super::super::Foundation::BOOL,
    pub lpszDefLabel: ::windows_sys::core::PWSTR,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OLEUICONVERTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OLEUICONVERTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OLEUIEDITLINKSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOleUILinkContainer: *mut *mut *mut *mut IOleUILinkContainerA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OLEUIEDITLINKSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OLEUIEDITLINKSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OLEUIEDITLINKSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOleUILinkContainer: *mut *mut *mut *mut IOleUILinkContainerW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OLEUIEDITLINKSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OLEUIEDITLINKSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIGNRLPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIGNRLPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIGNRLPROPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIGNRLPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIGNRLPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIGNRLPROPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct OLEUIINSERTOBJECTA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: ::windows_sys::core::GUID,
    pub lpszFile: ::windows_sys::core::PSTR,
    pub cchFile: u32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows_sys::core::GUID,
    pub iid: ::windows_sys::core::GUID,
    pub oleRender: u32,
    pub lpFormatEtc: *mut super::Com::FORMATETC,
    pub lpIOleClientSite: *mut *mut *mut *mut IOleClientSite,
    pub lpIStorage: *mut *mut *mut *mut super::Com::StructuredStorage::IStorage,
    pub ppvObj: *mut *mut ::core::ffi::c_void,
    pub sc: i32,
    pub hMetaPict: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for OLEUIINSERTOBJECTA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for OLEUIINSERTOBJECTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct OLEUIINSERTOBJECTW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: ::windows_sys::core::GUID,
    pub lpszFile: ::windows_sys::core::PWSTR,
    pub cchFile: u32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows_sys::core::GUID,
    pub iid: ::windows_sys::core::GUID,
    pub oleRender: u32,
    pub lpFormatEtc: *mut super::Com::FORMATETC,
    pub lpIOleClientSite: *mut *mut *mut *mut IOleClientSite,
    pub lpIStorage: *mut *mut *mut *mut super::Com::StructuredStorage::IStorage,
    pub ppvObj: *mut *mut ::core::ffi::c_void,
    pub sc: i32,
    pub hMetaPict: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for OLEUIINSERTOBJECTW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for OLEUIINSERTOBJECTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUILINKPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUILINKPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUILINKPROPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUILINKPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUILINKPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUILINKPROPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIOBJECTPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub lpPS: *mut super::super::UI::Controls::PROPSHEETHEADERA_V2,
    pub dwObject: u32,
    pub lpObjInfo: *mut *mut *mut *mut IOleUIObjInfoA,
    pub dwLink: u32,
    pub lpLinkInfo: *mut *mut *mut *mut IOleUILinkInfoA,
    pub lpGP: *mut OLEUIGNRLPROPSA,
    pub lpVP: *mut OLEUIVIEWPROPSA,
    pub lpLP: *mut OLEUILINKPROPSA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIOBJECTPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIOBJECTPROPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIOBJECTPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub lpPS: *mut super::super::UI::Controls::PROPSHEETHEADERW_V2,
    pub dwObject: u32,
    pub lpObjInfo: *mut *mut *mut *mut IOleUIObjInfoW,
    pub dwLink: u32,
    pub lpLinkInfo: *mut *mut *mut *mut IOleUILinkInfoW,
    pub lpGP: *mut OLEUIGNRLPROPSW,
    pub lpVP: *mut OLEUIVIEWPROPSW,
    pub lpLP: *mut OLEUILINKPROPSW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIOBJECTPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIOBJECTPROPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct OLEUIPASTEENTRYA {
    pub fmtetc: super::Com::FORMATETC,
    pub lpstrFormatName: ::windows_sys::core::PCSTR,
    pub lpstrResultText: ::windows_sys::core::PCSTR,
    pub dwFlags: u32,
    pub dwScratchSpace: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for OLEUIPASTEENTRYA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for OLEUIPASTEENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct OLEUIPASTEENTRYW {
    pub fmtetc: super::Com::FORMATETC,
    pub lpstrFormatName: ::windows_sys::core::PCWSTR,
    pub lpstrResultText: ::windows_sys::core::PCWSTR,
    pub dwFlags: u32,
    pub dwScratchSpace: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for OLEUIPASTEENTRYW {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for OLEUIPASTEENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLEUIPASTEFLAG = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_ENABLEICON: OLEUIPASTEFLAG = 2048i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_PASTEONLY: OLEUIPASTEFLAG = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_PASTE: OLEUIPASTEFLAG = 512i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKANYTYPE: OLEUIPASTEFLAG = 1024i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE1: OLEUIPASTEFLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE2: OLEUIPASTEFLAG = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE3: OLEUIPASTEFLAG = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE4: OLEUIPASTEFLAG = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE5: OLEUIPASTEFLAG = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE6: OLEUIPASTEFLAG = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE7: OLEUIPASTEFLAG = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE8: OLEUIPASTEFLAG = 128i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct OLEUIPASTESPECIALA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpSrcDataObj: *mut *mut *mut *mut super::Com::IDataObject,
    pub arrPasteEntries: *mut OLEUIPASTEENTRYA,
    pub cPasteEntries: i32,
    pub arrLinkTypes: *mut u32,
    pub cLinkTypes: i32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows_sys::core::GUID,
    pub nSelectedIndex: i32,
    pub fLink: super::super::Foundation::BOOL,
    pub hMetaPict: isize,
    pub sizel: super::super::Foundation::SIZE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for OLEUIPASTESPECIALA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for OLEUIPASTESPECIALA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct OLEUIPASTESPECIALW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpSrcDataObj: *mut *mut *mut *mut super::Com::IDataObject,
    pub arrPasteEntries: *mut OLEUIPASTEENTRYW,
    pub cPasteEntries: i32,
    pub arrLinkTypes: *mut u32,
    pub cLinkTypes: i32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows_sys::core::GUID,
    pub nSelectedIndex: i32,
    pub fLink: super::super::Foundation::BOOL,
    pub hMetaPict: isize,
    pub sizel: super::super::Foundation::SIZE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for OLEUIPASTESPECIALW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for OLEUIPASTESPECIALW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIVIEWPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
    pub nScaleMin: i32,
    pub nScaleMax: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIVIEWPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIVIEWPROPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIVIEWPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
    pub nScaleMin: i32,
    pub nScaleMax: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIVIEWPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIVIEWPROPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_BZERR_HTASKINVALID: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_BZ_CALLUNBLOCKED: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_BZ_RETRYSELECTED: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_BZ_SWITCHTOSELECTED: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CANCEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CIERR_MUSTHAVECLSID: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CIERR_MUSTHAVECURRENTMETAFILE: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CIERR_SZICONEXEINVALID: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_FROMNOTNULL: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_LINKCNTRINVALID: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_LINKCNTRNULL: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_SOURCEINVALID: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_SOURCENULL: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_SOURCEPARSEERROR: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_SOURCEPARSERROR: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_TONOTNULL: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CTERR_CBFORMATINVALID: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CTERR_CLASSIDINVALID: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CTERR_DVASPECTINVALID: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CTERR_HMETAPICTINVALID: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CTERR_STRINGINVALID: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ELERR_LINKCNTRINVALID: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ELERR_LINKCNTRNULL: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_CBSTRUCTINCORRECT: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_DIALOGFAILURE: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_FINDTEMPLATEFAILURE: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_GLOBALMEMALLOC: u32 = 114u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_HINSTANCEINVALID: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_HRESOURCEINVALID: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_HWNDOWNERINVALID: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_LOADSTRING: u32 = 115u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_LOADTEMPLATEFAILURE: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_LOCALMEMALLOC: u32 = 113u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_LPFNHOOKINVALID: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_LPSZCAPTIONINVALID: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_LPSZTEMPLATEINVALID: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_OLEMEMALLOC: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_STANDARDMAX: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_STANDARDMIN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_STRUCTUREINVALID: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_STRUCTURENULL: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_FALSE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_GPERR_CBFORMATINVALID: u32 = 130u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_GPERR_CLASSIDINVALID: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_GPERR_LPCLSIDEXCLUDEINVALID: u32 = 129u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_GPERR_STRINGINVALID: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_ARRLINKTYPESINVALID: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_ARRPASTEENTRIESINVALID: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_CCHFILEINVALID: u32 = 125u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_HICONINVALID: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_LPCLSIDEXCLUDEINVALID: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_LPFORMATETCINVALID: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_LPIOLECLIENTSITEINVALID: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_LPISTORAGEINVALID: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_LPSZFILEINVALID: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_LPSZLABELINVALID: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_PPVOBJINVALID: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_SCODEHASERROR: u32 = 123u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_SRCDATAOBJECTINVALID: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_LPERR_LINKCNTRINVALID: u32 = 134u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_LPERR_LINKCNTRNULL: u32 = 133u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_DLGPROCNOTNULL: u32 = 125u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_INVALIDPAGES: u32 = 123u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_LINKINFOINVALID: u32 = 137u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_LPARAMNOTZERO: u32 = 126u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_NOTSUPPORTED: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_OBJINFOINVALID: u32 = 136u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_PAGESINCORRECT: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_PROPERTYSHEET: u32 = 135u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_PROPSHEETINVALID: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_PROPSHEETNULL: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_PROPSINVALID: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_SUBPROPINVALID: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_SUBPROPNULL: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_SUPPROP: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_PSERR_CLIPBOARDCHANGED: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_PSERR_GETCLIPBOARDFAILED: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_QUERY_GETCLASSID: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_QUERY_LINKBROKEN: u32 = 65281u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_VPERR_DVASPECTINVALID: u32 = 132u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_VPERR_METAPICTINVALID: u32 = 131u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLEUPDATE = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUPDATE_ALWAYS: OLEUPDATE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUPDATE_ONCALL: OLEUPDATE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct OLEVERB {
    pub lVerb: i32,
    pub lpszVerbName: ::windows_sys::core::PWSTR,
    pub fuFlags: u32,
    pub grfAttribs: u32,
}
impl ::core::marker::Copy for OLEVERB {}
impl ::core::clone::Clone for OLEVERB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLEVERBATTRIB = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEVERBATTRIB_NEVERDIRTIES: OLEVERBATTRIB = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEVERBATTRIB_ONCONTAINERMENU: OLEVERBATTRIB = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEVERB_PRIMARY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLEWHICHMK = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEWHICHMK_CONTAINER: OLEWHICHMK = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEWHICHMK_OBJREL: OLEWHICHMK = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEWHICHMK_OBJFULL: OLEWHICHMK = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type OLE_TRISTATE = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const triUnchecked: OLE_TRISTATE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const triChecked: OLE_TRISTATE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const triGray: OLE_TRISTATE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OPF_DISABLECONVERT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OPF_NOFILLDEFAULT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OPF_OBJECTISLINK: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OPF_SHOWHELP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OT_EMBEDDED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OT_LINK: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OT_STATIC: i32 = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct OleMenuGroupWidths {
    pub width: [i32; 6],
}
impl ::core::marker::Copy for OleMenuGroupWidths {}
impl ::core::clone::Clone for OleMenuGroupWidths {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type PAGEACTION_UI = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PAGEACTION_UI_DEFAULT: PAGEACTION_UI = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PAGEACTION_UI_MODAL: PAGEACTION_UI = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PAGEACTION_UI_MODELESS: PAGEACTION_UI = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PAGEACTION_UI_SILENT: PAGEACTION_UI = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct PAGERANGE {
    pub nFromPage: i32,
    pub nToPage: i32,
}
impl ::core::marker::Copy for PAGERANGE {}
impl ::core::clone::Clone for PAGERANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESET {
    pub cbStruct: u32,
    pub fOddPages: super::super::Foundation::BOOL,
    pub fEvenPages: super::super::Foundation::BOOL,
    pub cPageRange: u32,
    pub rgPages: [PAGERANGE; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct PARAMDATA {
    pub szName: ::windows_sys::core::PWSTR,
    pub vt: u16,
}
impl ::core::marker::Copy for PARAMDATA {}
impl ::core::clone::Clone for PARAMDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct PARAMDESC {
    pub pparamdescex: *mut PARAMDESCEX,
    pub wParamFlags: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for PARAMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for PARAMDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct PARAMDESCEX {
    pub cBytes: u32,
    pub varDefaultValue: super::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for PARAMDESCEX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for PARAMDESCEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FHASCUSTDATA: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FHASDEFAULT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FLCID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FOPT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FRETVAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PERPROP_E_FIRST: i32 = -2147220992i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PERPROP_E_LAST: ::windows_sys::core::HRESULT = -2147220977i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PERPROP_E_NOPAGEAVAILABLE: ::windows_sys::core::HRESULT = -2147220992i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PERPROP_S_FIRST: ::windows_sys::core::HRESULT = 262656i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PERPROP_S_LAST: ::windows_sys::core::HRESULT = 262671i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC {
    pub cbSizeofstruct: u32,
    pub picType: u32,
    pub Anonymous: PICTDESC_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PICTDESC {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PICTDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PICTDESC_0 {
    pub bmp: PICTDESC_0_0,
    pub wmf: PICTDESC_0_3,
    pub icon: PICTDESC_0_2,
    pub emf: PICTDESC_0_1,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PICTDESC_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PICTDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_0 {
    pub hbitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PICTDESC_0_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PICTDESC_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_1 {
    pub hemf: super::super::Graphics::Gdi::HENHMETAFILE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PICTDESC_0_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PICTDESC_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_2 {
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PICTDESC_0_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PICTDESC_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_3 {
    pub hmeta: super::super::Graphics::Gdi::HMETAFILE,
    pub xExt: i32,
    pub yExt: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PICTDESC_0_3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PICTDESC_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTYPE_BITMAP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTYPE_ENHMETAFILE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTYPE_ICON: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTYPE_METAFILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTYPE_UNINITIALIZED: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type POINTERINACTIVE = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const POINTERINACTIVE_ACTIVATEONENTRY: POINTERINACTIVE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const POINTERINACTIVE_DEACTIVATEONLEAVE: POINTERINACTIVE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const POINTERINACTIVE_ACTIVATEONDRAG: POINTERINACTIVE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct POINTF {
    pub x: f32,
    pub y: f32,
}
impl ::core::marker::Copy for POINTF {}
impl ::core::clone::Clone for POINTF {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type PRINTFLAG = u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_MAYBOTHERUSER: PRINTFLAG = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_PROMPTUSER: PRINTFLAG = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_USERMAYCHANGEPRINTER: PRINTFLAG = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_RECOMPOSETODEVICE: PRINTFLAG = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_DONTACTUALLYPRINT: PRINTFLAG = 16u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_FORCEPROPERTIES: PRINTFLAG = 32u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_PRINTTOFILE: PRINTFLAG = 64u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type PROPBAG2_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_UNDEFINED: PROPBAG2_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_DATA: PROPBAG2_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_URL: PROPBAG2_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_OBJECT: PROPBAG2_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_STREAM: PROPBAG2_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_STORAGE: PROPBAG2_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_MONIKER: PROPBAG2_TYPE = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPPAGEINFO {
    pub cb: u32,
    pub pszTitle: ::windows_sys::core::PWSTR,
    pub size: super::super::Foundation::SIZE,
    pub pszDocString: ::windows_sys::core::PWSTR,
    pub pszHelpFile: ::windows_sys::core::PWSTR,
    pub dwHelpContext: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPPAGEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPPAGEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type PROPPAGESTATUS = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPPAGESTATUS_DIRTY: PROPPAGESTATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPPAGESTATUS_VALIDATE: PROPPAGESTATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPPAGESTATUS_CLEAN: PROPPAGESTATUS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROP_HWND_CHGICONDLG: &str = "HWND_CIDLG";
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_CHECKDISPLAYASICON: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_DISABLEDISPLAYASICON: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_HIDECHANGEICON: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_NOREFRESHDATAOBJECT: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_SELECTPASTE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_SELECTPASTELINK: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_SHOWHELP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_STAYONCLIPBOARDCHANGE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PS_MAXLINKTYPES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type PictureAttributes = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTURE_SCALABLE: PictureAttributes = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTURE_TRANSPARENT: PictureAttributes = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub struct QACONTAINER {
    pub cbSize: u32,
    pub pClientSite: *mut *mut *mut *mut IOleClientSite,
    pub pAdviseSink: *mut *mut *mut *mut IAdviseSinkEx,
    pub pPropertyNotifySink: *mut *mut *mut *mut IPropertyNotifySink,
    pub pUnkEventSink: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
    pub dwAmbientFlags: u32,
    pub colorFore: u32,
    pub colorBack: u32,
    pub pFont: *mut *mut *mut *mut IFont,
    pub pUndoMgr: *mut *mut *mut *mut IOleUndoManager,
    pub dwAppearance: u32,
    pub lcid: i32,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
    pub pBindHost: *mut *mut *mut *mut super::Com::IBindHost,
    pub pOleControlSite: *mut *mut *mut *mut IOleControlSite,
    pub pServiceProvider: *mut *mut *mut *mut super::Com::IServiceProvider,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for QACONTAINER {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for QACONTAINER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type QACONTAINERFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_SHOWHATCHING: QACONTAINERFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_SHOWGRABHANDLES: QACONTAINERFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_USERMODE: QACONTAINERFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_DISPLAYASDEFAULT: QACONTAINERFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_UIDEAD: QACONTAINERFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_AUTOCLIP: QACONTAINERFLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_MESSAGEREFLECT: QACONTAINERFLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_SUPPORTSMNEMONICS: QACONTAINERFLAGS = 128i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct QACONTROL {
    pub cbSize: u32,
    pub dwMiscStatus: u32,
    pub dwViewStatus: u32,
    pub dwEventCookie: u32,
    pub dwPropNotifyCookie: u32,
    pub dwPointerActivationPolicy: u32,
}
impl ::core::marker::Copy for QACONTROL {}
impl ::core::clone::Clone for QACONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type READYSTATE = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const READYSTATE_UNINITIALIZED: READYSTATE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const READYSTATE_LOADING: READYSTATE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const READYSTATE_LOADED: READYSTATE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const READYSTATE_INTERACTIVE: READYSTATE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const READYSTATE_COMPLETE: READYSTATE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type REGKIND = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const REGKIND_DEFAULT: REGKIND = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const REGKIND_REGISTER: REGKIND = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const REGKIND_NONE: REGKIND = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SELFREG_E_CLASS: ::windows_sys::core::HRESULT = -2147220991i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SELFREG_E_FIRST: i32 = -2147220992i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SELFREG_E_LAST: ::windows_sys::core::HRESULT = -2147220977i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SELFREG_E_TYPELIB: ::windows_sys::core::HRESULT = -2147220992i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SELFREG_S_FIRST: ::windows_sys::core::HRESULT = 262656i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SELFREG_S_LAST: ::windows_sys::core::HRESULT = 262671i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type SF_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_ERROR: SF_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_I1: SF_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_I2: SF_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_I4: SF_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_I8: SF_TYPE = 20i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_BSTR: SF_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_UNKNOWN: SF_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_DISPATCH: SF_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_VARIANT: SF_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_RECORD: SF_TYPE = 36i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_HAVEIID: SF_TYPE = 32781i32;
pub const SID_GetCaller: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1192741952, data2: 48313, data3: 4560, data4: [147, 54, 0, 160, 201, 13, 202, 169] };
pub const SID_ProvideRuntimeContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1956971532, data2: 56588, data3: 18672, data4: [172, 133, 25, 76, 50, 89, 24, 10] };
pub const SID_VariantConversion: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 521147521, data2: 48333, data3: 4560, data4: [147, 54, 0, 160, 201, 13, 202, 169] };
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE2_LCID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE2_MAJORVERNUM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE2_MINORVERNUM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE_LCID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE_MAJORVERNUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE_MINORVERNUM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE_TLB: &str = "stdole2.tlb";
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDTYPE_TLB: &str = "stdole2.tlb";
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_ADDCONTROL: &str = "OLEUI_MSG_ADDCONTROL";
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_BROWSE: &str = "OLEUI_MSG_BROWSE";
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_BROWSE_OFN: &str = "OLEUI_MSG_BROWSE_OFN";
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_CHANGEICON: &str = "OLEUI_MSG_CHANGEICON";
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_CHANGESOURCE: &str = "OLEUI_MSG_CHANGESOURCE";
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_CLOSEBUSYDIALOG: &str = "OLEUI_MSG_CLOSEBUSYDIALOG";
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_CONVERT: &str = "OLEUI_MSG_CONVERT";
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_ENDDIALOG: &str = "OLEUI_MSG_ENDDIALOG";
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_HELP: &str = "OLEUI_MSG_HELP";
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TIFLAGS_EXTENDDISPATCHONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type TYPEFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FAPPOBJECT: TYPEFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FCANCREATE: TYPEFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FLICENSED: TYPEFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FPREDECLID: TYPEFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FHIDDEN: TYPEFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FCONTROL: TYPEFLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FDUAL: TYPEFLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FNONEXTENSIBLE: TYPEFLAGS = 128i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FOLEAUTOMATION: TYPEFLAGS = 256i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FRESTRICTED: TYPEFLAGS = 512i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FAGGREGATABLE: TYPEFLAGS = 1024i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FREPLACEABLE: TYPEFLAGS = 2048i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FDISPATCHABLE: TYPEFLAGS = 4096i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FREVERSEBIND: TYPEFLAGS = 8192i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FPROXY: TYPEFLAGS = 16384i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type UASFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UAS_NORMAL: UASFLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UAS_BLOCKED: UASFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UAS_NOPARENTENABLE: UASFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UAS_MASK: UASFLAGS = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct UDATE {
    pub st: super::super::Foundation::SYSTEMTIME,
    pub wDayOfYear: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UDATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UDATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type UPDFCACHE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_ALL: UPDFCACHE_FLAGS = 2147483647u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_ALLBUTNODATACACHE: UPDFCACHE_FLAGS = 2147483646u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_NORMALCACHE: UPDFCACHE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_IFBLANK: UPDFCACHE_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_ONLYIFBLANK: UPDFCACHE_FLAGS = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_NODATACACHE: UPDFCACHE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_ONSAVECACHE: UPDFCACHE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_ONSTOPCACHE: UPDFCACHE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_IFBLANKORONSAVECACHE: UPDFCACHE_FLAGS = 18u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type USERCLASSTYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const USERCLASSTYPE_FULL: USERCLASSTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const USERCLASSTYPE_SHORT: USERCLASSTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const USERCLASSTYPE_APPNAME: USERCLASSTYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARCMP_EQ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARCMP_GT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARCMP_LT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARCMP_NULL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type VARENUM = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_EMPTY: VARENUM = 0i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_NULL: VARENUM = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_I2: VARENUM = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_I4: VARENUM = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_R4: VARENUM = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_R8: VARENUM = 5i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_CY: VARENUM = 6i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_DATE: VARENUM = 7i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_BSTR: VARENUM = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_DISPATCH: VARENUM = 9i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_ERROR: VARENUM = 10i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_BOOL: VARENUM = 11i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_VARIANT: VARENUM = 12i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_UNKNOWN: VARENUM = 13i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_DECIMAL: VARENUM = 14i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_I1: VARENUM = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_UI1: VARENUM = 17i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_UI2: VARENUM = 18i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_UI4: VARENUM = 19i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_I8: VARENUM = 20i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_UI8: VARENUM = 21i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_INT: VARENUM = 22i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_UINT: VARENUM = 23i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_VOID: VARENUM = 24i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_HRESULT: VARENUM = 25i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_PTR: VARENUM = 26i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_SAFEARRAY: VARENUM = 27i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_CARRAY: VARENUM = 28i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_USERDEFINED: VARENUM = 29i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_LPSTR: VARENUM = 30i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_LPWSTR: VARENUM = 31i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_RECORD: VARENUM = 36i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_INT_PTR: VARENUM = 37i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_UINT_PTR: VARENUM = 38i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_FILETIME: VARENUM = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_BLOB: VARENUM = 65i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_STREAM: VARENUM = 66i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_STORAGE: VARENUM = 67i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_STREAMED_OBJECT: VARENUM = 68i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_STORED_OBJECT: VARENUM = 69i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_BLOB_OBJECT: VARENUM = 70i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_CF: VARENUM = 71i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_CLSID: VARENUM = 72i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_VERSIONED_STREAM: VARENUM = 73i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_BSTR_BLOB: VARENUM = 4095i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_VECTOR: VARENUM = 4096i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_ARRAY: VARENUM = 8192i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_BYREF: VARENUM = 16384i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_RESERVED: VARENUM = 32768i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_ILLEGAL: VARENUM = 65535i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_ILLEGALMASKED: VARENUM = 4095i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_TYPEMASK: VARENUM = 4095i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type VARFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFLAG_FREADONLY: VARFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFLAG_FSOURCE: VARFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFLAG_FBINDABLE: VARFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFLAG_FREQUESTEDIT: VARFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFLAG_FDISPLAYBIND: VARFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFLAG_FDEFAULTBIND: VARFLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFLAG_FHIDDEN: VARFLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFLAG_FRESTRICTED: VARFLAGS = 128i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFLAG_FDEFAULTCOLLELEM: VARFLAGS = 256i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFLAG_FUIDEFAULT: VARFLAGS = 512i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFLAG_FNONBROWSABLE: VARFLAGS = 1024i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFLAG_FREPLACEABLE: VARFLAGS = 2048i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFLAG_FIMMEDIATEBIND: VARFLAGS = 4096i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_ALPHABOOL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_CALENDAR_GREGORIAN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_CALENDAR_HIJRI: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_CALENDAR_THAI: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_LOCALBOOL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_NOUSEROVERRIDE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_NOVALUEPROP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_USE_NLS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type VIEWSTATUS = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VIEWSTATUS_OPAQUE: VIEWSTATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VIEWSTATUS_SOLIDBKGND: VIEWSTATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VIEWSTATUS_DVASPECTOPAQUE: VIEWSTATUS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VIEWSTATUS_DVASPECTTRANSPARENT: VIEWSTATUS = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VIEWSTATUS_SURFACE: VIEWSTATUS = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VIEWSTATUS_3DSURFACE: VIEWSTATUS = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VPF_DISABLERELATIVE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VPF_DISABLESCALE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VPF_SELECTRELATIVE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VTDATEGRE_MAX: u32 = 2958465u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VTDATEGRE_MIN: i32 = -657434i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_BLOB_PROPSET: u32 = 75u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_STORED_PROPSET: u32 = 74u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_STREAMED_PROPSET: u32 = 73u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_VERBOSE_ENUM: u32 = 76u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const WIN32: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type WPCSETTING = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const WPCSETTING_LOGGING_ENABLED: WPCSETTING = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const WPCSETTING_FILEDOWNLOAD_BLOCKED: WPCSETTING = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub type XFORMCOORDS = i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const XFORMCOORDS_POSITION: XFORMCOORDS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const XFORMCOORDS_SIZE: XFORMCOORDS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const XFORMCOORDS_HIMETRICTOCONTAINER: XFORMCOORDS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const XFORMCOORDS_CONTAINERTOHIMETRIC: XFORMCOORDS = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const XFORMCOORDS_EVENTCOMPAT: XFORMCOORDS = 16i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct _wireBRECORD {
    pub fFlags: u32,
    pub clSize: u32,
    pub pRecInfo: *mut *mut *mut *mut IRecordInfo,
    pub pRecord: *mut u8,
}
impl ::core::marker::Copy for _wireBRECORD {}
impl ::core::clone::Clone for _wireBRECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct _wireSAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub uArrayStructs: _wireSAFEARRAY_UNION,
    pub rgsabound: [super::Com::SAFEARRAYBOUND; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for _wireSAFEARRAY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for _wireSAFEARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct _wireSAFEARRAY_UNION {
    pub sfType: u32,
    pub u: _wireSAFEARRAY_UNION_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for _wireSAFEARRAY_UNION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for _wireSAFEARRAY_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub union _wireSAFEARRAY_UNION_0 {
    pub BstrStr: _wireSAFEARR_BSTR,
    pub UnknownStr: _wireSAFEARR_UNKNOWN,
    pub DispatchStr: _wireSAFEARR_DISPATCH,
    pub VariantStr: _wireSAFEARR_VARIANT,
    pub RecordStr: _wireSAFEARR_BRECORD,
    pub HaveIidStr: _wireSAFEARR_HAVEIID,
    pub ByteStr: super::Com::BYTE_SIZEDARR,
    pub WordStr: super::Com::SHORT_SIZEDARR,
    pub LongStr: super::Com::LONG_SIZEDARR,
    pub HyperStr: super::Com::HYPER_SIZEDARR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for _wireSAFEARRAY_UNION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for _wireSAFEARRAY_UNION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct _wireSAFEARR_BRECORD {
    pub Size: u32,
    pub aRecord: *mut *mut _wireBRECORD,
}
impl ::core::marker::Copy for _wireSAFEARR_BRECORD {}
impl ::core::clone::Clone for _wireSAFEARR_BRECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct _wireSAFEARR_BSTR {
    pub Size: u32,
    pub aBstr: *mut *mut super::Com::FLAGGED_WORD_BLOB,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for _wireSAFEARR_BSTR {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _wireSAFEARR_BSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct _wireSAFEARR_DISPATCH {
    pub Size: u32,
    pub apDispatch: *mut *mut *mut super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for _wireSAFEARR_DISPATCH {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _wireSAFEARR_DISPATCH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct _wireSAFEARR_HAVEIID {
    pub Size: u32,
    pub apUnknown: *mut *mut *mut ::windows_sys::core::IUnknown,
    pub iid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for _wireSAFEARR_HAVEIID {}
impl ::core::clone::Clone for _wireSAFEARR_HAVEIID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct _wireSAFEARR_UNKNOWN {
    pub Size: u32,
    pub apUnknown: *mut *mut *mut ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for _wireSAFEARR_UNKNOWN {}
impl ::core::clone::Clone for _wireSAFEARR_UNKNOWN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct _wireSAFEARR_VARIANT {
    pub Size: u32,
    pub aVariant: *mut *mut _wireVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for _wireSAFEARR_VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for _wireSAFEARR_VARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct _wireVARIANT {
    pub clSize: u32,
    pub rpcReserved: u32,
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: _wireVARIANT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for _wireVARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for _wireVARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub union _wireVARIANT_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: i16,
    pub scode: i32,
    pub cyVal: super::Com::CY,
    pub date: f64,
    pub bstrVal: *mut super::Com::FLAGGED_WORD_BLOB,
    pub punkVal: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
    pub pdispVal: *mut *mut *mut *mut super::Com::IDispatch,
    pub parray: *mut *mut _wireSAFEARRAY,
    pub brecVal: *mut _wireBRECORD,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut i16,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::Com::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut *mut super::Com::FLAGGED_WORD_BLOB,
    pub ppunkVal: *mut *mut *mut ::windows_sys::core::IUnknown,
    pub ppdispVal: *mut *mut *mut super::Com::IDispatch,
    pub pparray: *mut *mut *mut _wireSAFEARRAY,
    pub pvarVal: *mut *mut _wireVARIANT,
    pub cVal: super::super::Foundation::CHAR,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub decVal: super::super::Foundation::DECIMAL,
    pub pdecVal: *mut super::super::Foundation::DECIMAL,
    pub pcVal: ::windows_sys::core::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for _wireVARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for _wireVARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexEnumAll: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexEnumDefault: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexNameCaseInsensitive: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexNameCaseSensitive: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexNameEnsure: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexNameImplicit: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexNameInternal: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexNameNoDynamicProperties: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCanCall: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCanConstruct: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCanGet: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCanPut: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCanPutRef: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCanSourceEvents: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCannotCall: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCannotConstruct: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCannotGet: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCannotPut: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCannotPutRef: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCannotSourceEvents: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropDynamicType: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropNoSideEffects: i32 = 64i32;
