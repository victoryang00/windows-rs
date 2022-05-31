#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_System_Com")]
    pub fn BstrFromVector(psa: *const super::Com::SAFEARRAY, pbstr: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn ClearCustData(pcustdata: *mut super::Com::CUSTDATA);
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateDispTypeInfo(pidata: *mut INTERFACEDATA, lcid: u32, pptinfo: *mut super::Com::ITypeInfo) -> ::windows_core_sys::HRESULT;
    pub fn CreateErrorInfo(pperrinfo: *mut ICreateErrorInfo) -> ::windows_core_sys::HRESULT;
    pub fn CreateOleAdviseHolder(ppoaholder: *mut IOleAdviseHolder) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateStdDispatch(punkouter: ::windows_core_sys::IUnknown, pvthis: *mut ::core::ffi::c_void, ptinfo: super::Com::ITypeInfo, ppunkstddisp: *mut ::windows_core_sys::IUnknown) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateTypeLib(syskind: super::Com::SYSKIND, szfile: ::windows_core_sys::PCWSTR, ppctlib: *mut ICreateTypeLib) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateTypeLib2(syskind: super::Com::SYSKIND, szfile: ::windows_core_sys::PCWSTR, ppctlib: *mut ICreateTypeLib2) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn DispCallFunc(pvinstance: *const ::core::ffi::c_void, ovft: usize, cc: super::Com::CALLCONV, vtreturn: u16, cactuals: u32, prgvt: *const u16, prgpvarg: *const *const super::Com::VARIANT, pvargresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn DispGetIDsOfNames(ptinfo: super::Com::ITypeInfo, rgsznames: *const ::windows_core_sys::PWSTR, cnames: u32, rgdispid: *mut i32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn DispGetParam(pdispparams: *const super::Com::DISPPARAMS, position: u32, vttarg: u16, pvarresult: *mut super::Com::VARIANT, puargerr: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn DispInvoke(_this: *mut ::core::ffi::c_void, ptinfo: super::Com::ITypeInfo, dispidmember: i32, wflags: u16, pparams: *mut super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn DoDragDrop(pdataobj: super::Com::IDataObject, pdropsource: IDropSource, dwokeffects: u32, pdweffect: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DosDateTimeToVariantTime(wdosdate: u16, wdostime: u16, pvtime: *mut f64) -> i32;
    pub fn GetActiveObject(rclsid: *const ::windows_core_sys::GUID, pvreserved: *mut ::core::ffi::c_void, ppunk: *mut ::windows_core_sys::IUnknown) -> ::windows_core_sys::HRESULT;
    pub fn GetAltMonthNames(lcid: u32, prgp: *mut *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn GetRecordInfoFromGuids(rguidtypelib: *const ::windows_core_sys::GUID, uvermajor: u32, uverminor: u32, lcid: u32, rguidtypeinfo: *const ::windows_core_sys::GUID, pprecinfo: *mut IRecordInfo) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn GetRecordInfoFromTypeInfo(ptypeinfo: super::Com::ITypeInfo, pprecinfo: *mut IRecordInfo) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserFree(param0: *const u32, param1: *const ::win32_graphics_sys::Gdi::HRGN);
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserFree64(param0: *const u32, param1: *const ::win32_graphics_sys::Gdi::HRGN);
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics_sys::Gdi::HRGN) -> *mut u8;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::win32_graphics_sys::Gdi::HRGN) -> *mut u8;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserSize(param0: *const u32, param1: u32, param2: *const ::win32_graphics_sys::Gdi::HRGN) -> u32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserSize64(param0: *const u32, param1: u32, param2: *const ::win32_graphics_sys::Gdi::HRGN) -> u32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics_sys::Gdi::HRGN) -> *mut u8;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::win32_graphics_sys::Gdi::HRGN) -> *mut u8;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn IsAccelerator(haccel: ::win32_ui_sys::WindowsAndMessaging::HACCEL, caccelentries: i32, lpmsg: *mut ::win32_ui_sys::WindowsAndMessaging::MSG, lpwcmd: *mut u16) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_System_Com")]
    pub fn LHashValOfNameSys(syskind: super::Com::SYSKIND, lcid: u32, szname: ::windows_core_sys::PCWSTR) -> u32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn LHashValOfNameSysA(syskind: super::Com::SYSKIND, lcid: u32, szname: ::windows_core_sys::PCSTR) -> u32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn LoadRegTypeLib(rguid: *const ::windows_core_sys::GUID, wvermajor: u16, wverminor: u16, lcid: u32, pptlib: *mut super::Com::ITypeLib) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn LoadTypeLib(szfile: ::windows_core_sys::PCWSTR, pptlib: *mut super::Com::ITypeLib) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn LoadTypeLibEx(szfile: ::windows_core_sys::PCWSTR, regkind: REGKIND, pptlib: *mut super::Com::ITypeLib) -> ::windows_core_sys::HRESULT;
    pub fn OaBuildVersion() -> u32;
    pub fn OaEnablePerUserTLibRegistration();
    pub fn OleBuildVersion() -> u32;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreate(rclsid: *const ::windows_core_sys::GUID, riid: *const ::windows_core_sys::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn OleCreateDefaultHandler(clsid: *const ::windows_core_sys::GUID, punkouter: ::windows_core_sys::IUnknown, riid: *const ::windows_core_sys::GUID, lplpobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleCreateEmbeddingHelper(clsid: *const ::windows_core_sys::GUID, punkouter: ::windows_core_sys::IUnknown, flags: u32, pcf: super::Com::IClassFactory, riid: *const ::windows_core_sys::GUID, lplpobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateEx(rclsid: *const ::windows_core_sys::GUID, riid: *const ::windows_core_sys::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleCreateFontIndirect(lpfontdesc: *mut FONTDESC, riid: *const ::windows_core_sys::GUID, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateFromData(psrcdataobj: super::Com::IDataObject, riid: *const ::windows_core_sys::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateFromDataEx(psrcdataobj: super::Com::IDataObject, riid: *const ::windows_core_sys::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateFromFile(rclsid: *const ::windows_core_sys::GUID, lpszfilename: ::windows_core_sys::PCWSTR, riid: *const ::windows_core_sys::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateFromFileEx(rclsid: *const ::windows_core_sys::GUID, lpszfilename: ::windows_core_sys::PCWSTR, riid: *const ::windows_core_sys::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateLink(pmklinksrc: super::Com::IMoniker, riid: *const ::windows_core_sys::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateLinkEx(pmklinksrc: super::Com::IMoniker, riid: *const ::windows_core_sys::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateLinkFromData(psrcdataobj: super::Com::IDataObject, riid: *const ::windows_core_sys::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateLinkFromDataEx(psrcdataobj: super::Com::IDataObject, riid: *const ::windows_core_sys::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateLinkToFile(lpszfilename: ::windows_core_sys::PCWSTR, riid: *const ::windows_core_sys::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateLinkToFileEx(lpszfilename: ::windows_core_sys::PCWSTR, riid: *const ::windows_core_sys::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn OleCreateMenuDescriptor(hmenucombined: ::win32_ui_sys::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OleMenuGroupWidths) -> isize;
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleCreatePictureIndirect(lppictdesc: *mut PICTDESC, riid: *const ::windows_core_sys::GUID, fown: ::win32_foundation_sys::BOOL, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn OleCreatePropertyFrame(hwndowner: ::win32_foundation_sys::HWND, x: u32, y: u32, lpszcaption: ::windows_core_sys::PCWSTR, cobjects: u32, ppunk: *mut ::windows_core_sys::IUnknown, cpages: u32, ppageclsid: *mut ::windows_core_sys::GUID, lcid: u32, dwreserved: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn OleCreatePropertyFrameIndirect(lpparams: *mut OCPFIPARAMS) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleCreateStaticFromData(psrcdataobj: super::Com::IDataObject, iid: *const ::windows_core_sys::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn OleDestroyMenuDescriptor(holemenu: isize) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleDoAutoConvert(pstg: super::Com::StructuredStorage::IStorage, pclsidnew: *mut ::windows_core_sys::GUID) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn OleDraw(punknown: ::windows_core_sys::IUnknown, dwaspect: u32, hdcdraw: ::win32_graphics_sys::Gdi::HDC, lprcbounds: *mut ::win32_foundation_sys::RECT) -> ::windows_core_sys::HRESULT;
    pub fn OleDuplicateData(hsrc: ::win32_foundation_sys::HANDLE, cfformat: u16, uiflags: u32) -> ::win32_foundation_sys::HANDLE;
    pub fn OleFlushClipboard() -> ::windows_core_sys::HRESULT;
    pub fn OleGetAutoConvert(clsidold: *const ::windows_core_sys::GUID, pclsidnew: *mut ::windows_core_sys::GUID) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleGetClipboard(ppdataobj: *mut super::Com::IDataObject) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleGetClipboardWithEnterpriseInfo(dataobject: *mut super::Com::IDataObject, dataenterpriseid: *mut ::windows_core_sys::PWSTR, sourcedescription: *mut ::windows_core_sys::PWSTR, targetdescription: *mut ::windows_core_sys::PWSTR, datadescription: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn OleGetIconOfClass(rclsid: *const ::windows_core_sys::GUID, lpszlabel: ::windows_core_sys::PCWSTR, fusetypeaslabel: ::win32_foundation_sys::BOOL) -> isize;
    pub fn OleGetIconOfFile(lpszpath: ::windows_core_sys::PCWSTR, fusefileaslabel: ::win32_foundation_sys::BOOL) -> isize;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn OleIconToCursor(hinstexe: ::win32_foundation_sys::HINSTANCE, hicon: ::win32_ui_sys::WindowsAndMessaging::HICON) -> ::win32_ui_sys::WindowsAndMessaging::HCURSOR;
    pub fn OleInitialize(pvreserved: *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleIsCurrentClipboard(pdataobj: super::Com::IDataObject) -> ::windows_core_sys::HRESULT;
    pub fn OleIsRunning(pobject: IOleObject) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleLoad(pstg: super::Com::StructuredStorage::IStorage, riid: *const ::windows_core_sys::GUID, pclientsite: IOleClientSite, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleLoadFromStream(pstm: super::Com::IStream, iidinterface: *const ::windows_core_sys::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleLoadPicture(lpstream: super::Com::IStream, lsize: i32, frunmode: ::win32_foundation_sys::BOOL, riid: *const ::windows_core_sys::GUID, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleLoadPictureEx(lpstream: super::Com::IStream, lsize: i32, frunmode: ::win32_foundation_sys::BOOL, riid: *const ::windows_core_sys::GUID, xsizedesired: u32, ysizedesired: u32, dwflags: u32, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleLoadPictureFile(varfilename: super::Com::VARIANT, lplpdisppicture: *mut super::Com::IDispatch) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleLoadPictureFileEx(varfilename: super::Com::VARIANT, xsizedesired: u32, ysizedesired: u32, dwflags: u32, lplpdisppicture: *mut super::Com::IDispatch) -> ::windows_core_sys::HRESULT;
    pub fn OleLoadPicturePath(szurlorpath: ::windows_core_sys::PCWSTR, punkcaller: ::windows_core_sys::IUnknown, dwreserved: u32, clrreserved: u32, riid: *const ::windows_core_sys::GUID, ppvret: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn OleLockRunning(punknown: ::windows_core_sys::IUnknown, flock: ::win32_foundation_sys::BOOL, flastunlockcloses: ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn OleMetafilePictFromIconAndLabel(hicon: ::win32_ui_sys::WindowsAndMessaging::HICON, lpszlabel: ::windows_core_sys::PCWSTR, lpszsourcefile: ::windows_core_sys::PCWSTR, iiconindex: u32) -> isize;
    pub fn OleNoteObjectVisible(punknown: ::windows_core_sys::IUnknown, fvisible: ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleQueryCreateFromData(psrcdataobject: super::Com::IDataObject) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleQueryLinkFromData(psrcdataobject: super::Com::IDataObject) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleRegEnumFormatEtc(clsid: *const ::windows_core_sys::GUID, dwdirection: u32, ppenum: *mut super::Com::IEnumFORMATETC) -> ::windows_core_sys::HRESULT;
    pub fn OleRegEnumVerbs(clsid: *const ::windows_core_sys::GUID, ppenum: *mut IEnumOLEVERB) -> ::windows_core_sys::HRESULT;
    pub fn OleRegGetMiscStatus(clsid: *const ::windows_core_sys::GUID, dwaspect: u32, pdwstatus: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn OleRegGetUserType(clsid: *const ::windows_core_sys::GUID, dwformoftype: u32, pszusertype: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn OleRun(punknown: ::windows_core_sys::IUnknown) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleSave(pps: super::Com::StructuredStorage::IPersistStorage, pstg: super::Com::StructuredStorage::IStorage, fsameasload: ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleSavePictureFile(lpdisppicture: super::Com::IDispatch, bstrfilename: ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleSaveToStream(ppstm: super::Com::IPersistStream, pstm: super::Com::IStream) -> ::windows_core_sys::HRESULT;
    pub fn OleSetAutoConvert(clsidold: *const ::windows_core_sys::GUID, clsidnew: *const ::windows_core_sys::GUID) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleSetClipboard(pdataobj: super::Com::IDataObject) -> ::windows_core_sys::HRESULT;
    pub fn OleSetContainedObject(punknown: ::windows_core_sys::IUnknown, fcontained: ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn OleSetMenuDescriptor(holemenu: isize, hwndframe: ::win32_foundation_sys::HWND, hwndactiveobject: ::win32_foundation_sys::HWND, lpframe: IOleInPlaceFrame, lpactiveobj: IOleInPlaceActiveObject) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn OleTranslateAccelerator(lpframe: IOleInPlaceFrame, lpframeinfo: *mut OIFI, lpmsg: *mut ::win32_ui_sys::WindowsAndMessaging::MSG) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn OleTranslateColor(clr: u32, hpal: ::win32_graphics_sys::Gdi::HPALETTE, lpcolorref: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn OleUIAddVerbMenuA(lpoleobj: IOleObject, lpszshorttype: ::windows_core_sys::PCSTR, hmenu: ::win32_ui_sys::WindowsAndMessaging::HMENU, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: ::win32_foundation_sys::BOOL, idconvert: u32, lphmenu: *mut ::win32_ui_sys::WindowsAndMessaging::HMENU) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn OleUIAddVerbMenuW(lpoleobj: IOleObject, lpszshorttype: ::windows_core_sys::PCWSTR, hmenu: ::win32_ui_sys::WindowsAndMessaging::HMENU, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: ::win32_foundation_sys::BOOL, idconvert: u32, lphmenu: *mut ::win32_ui_sys::WindowsAndMessaging::HMENU) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Media")]
    pub fn OleUIBusyA(param0: *const OLEUIBUSYA) -> u32;
    #[cfg(feature = "Win32_Media")]
    pub fn OleUIBusyW(param0: *const OLEUIBUSYW) -> u32;
    pub fn OleUICanConvertOrActivateAs(rclsid: *const ::windows_core_sys::GUID, fislinkedobject: ::win32_foundation_sys::BOOL, wformat: u16) -> ::win32_foundation_sys::BOOL;
    pub fn OleUIChangeIconA(param0: *const OLEUICHANGEICONA) -> u32;
    pub fn OleUIChangeIconW(param0: *const OLEUICHANGEICONW) -> u32;
    #[cfg(feature = "Win32_UI_Controls_Dialogs")]
    pub fn OleUIChangeSourceA(param0: *const OLEUICHANGESOURCEA) -> u32;
    #[cfg(feature = "Win32_UI_Controls_Dialogs")]
    pub fn OleUIChangeSourceW(param0: *const OLEUICHANGESOURCEW) -> u32;
    pub fn OleUIConvertA(param0: *const OLEUICONVERTA) -> u32;
    pub fn OleUIConvertW(param0: *const OLEUICONVERTW) -> u32;
    pub fn OleUIEditLinksA(param0: *const OLEUIEDITLINKSA) -> u32;
    pub fn OleUIEditLinksW(param0: *const OLEUIEDITLINKSW) -> u32;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleUIInsertObjectA(param0: *const OLEUIINSERTOBJECTA) -> u32;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleUIInsertObjectW(param0: *const OLEUIINSERTOBJECTW) -> u32;
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIObjectPropertiesA(param0: *const OLEUIOBJECTPROPSA) -> u32;
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIObjectPropertiesW(param0: *const OLEUIOBJECTPROPSW) -> u32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleUIPasteSpecialA(param0: *const OLEUIPASTESPECIALA) -> u32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleUIPasteSpecialW(param0: *const OLEUIPASTESPECIALW) -> u32;
    pub fn OleUIPromptUserA(ntemplate: i32, hwndparent: ::win32_foundation_sys::HWND) -> i32;
    pub fn OleUIPromptUserW(ntemplate: i32, hwndparent: ::win32_foundation_sys::HWND) -> i32;
    pub fn OleUIUpdateLinksA(lpoleuilinkcntr: IOleUILinkContainerA, hwndparent: ::win32_foundation_sys::HWND, lpsztitle: ::windows_core_sys::PCSTR, clinks: i32) -> ::win32_foundation_sys::BOOL;
    pub fn OleUIUpdateLinksW(lpoleuilinkcntr: IOleUILinkContainerW, hwndparent: ::win32_foundation_sys::HWND, lpsztitle: ::windows_core_sys::PCWSTR, clinks: i32) -> ::win32_foundation_sys::BOOL;
    pub fn OleUninitialize();
    pub fn QueryPathOfRegTypeLib(guid: *const ::windows_core_sys::GUID, wmaj: u16, wmin: u16, lcid: u32, lpbstrpathname: *mut *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn RegisterActiveObject(punk: ::windows_core_sys::IUnknown, rclsid: *const ::windows_core_sys::GUID, dwflags: u32, pdwregister: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn RegisterDragDrop(hwnd: ::win32_foundation_sys::HWND, pdroptarget: IDropTarget) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn RegisterTypeLib(ptlib: super::Com::ITypeLib, szfullpath: ::windows_core_sys::PCWSTR, szhelpdir: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn RegisterTypeLibForUser(ptlib: super::Com::ITypeLib, szfullpath: ::windows_core_sys::PCWSTR, szhelpdir: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ReleaseStgMedium(param0: *mut super::Com::STGMEDIUM);
    pub fn RevokeActiveObject(dwregister: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn RevokeDragDrop(hwnd: ::win32_foundation_sys::HWND) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAccessData(psa: *const super::Com::SAFEARRAY, ppvdata: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAddRef(psa: *const super::Com::SAFEARRAY, ppdatatorelease: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocData(psa: *const super::Com::SAFEARRAY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocDescriptor(cdims: u32, ppsaout: *mut *mut super::Com::SAFEARRAY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocDescriptorEx(vt: u16, cdims: u32, ppsaout: *mut *mut super::Com::SAFEARRAY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCopy(psa: *const super::Com::SAFEARRAY, ppsaout: *mut *mut super::Com::SAFEARRAY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCopyData(psasource: *const super::Com::SAFEARRAY, psatarget: *const super::Com::SAFEARRAY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreate(vt: u16, cdims: u32, rgsabound: *const super::Com::SAFEARRAYBOUND) -> *mut super::Com::SAFEARRAY;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateEx(vt: u16, cdims: u32, rgsabound: *const super::Com::SAFEARRAYBOUND, pvextra: *const ::core::ffi::c_void) -> *mut super::Com::SAFEARRAY;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateVector(vt: u16, llbound: i32, celements: u32) -> *mut super::Com::SAFEARRAY;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateVectorEx(vt: u16, llbound: i32, celements: u32, pvextra: *const ::core::ffi::c_void) -> *mut super::Com::SAFEARRAY;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroy(psa: *const super::Com::SAFEARRAY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroyData(psa: *const super::Com::SAFEARRAY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroyDescriptor(psa: *const super::Com::SAFEARRAY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetDim(psa: *const super::Com::SAFEARRAY) -> u32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetElement(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, pv: *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetElemsize(psa: *const super::Com::SAFEARRAY) -> u32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetIID(psa: *const super::Com::SAFEARRAY, pguid: *mut ::windows_core_sys::GUID) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetLBound(psa: *const super::Com::SAFEARRAY, ndim: u32, pllbound: *mut i32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetRecordInfo(psa: *const super::Com::SAFEARRAY, prinfo: *mut IRecordInfo) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetUBound(psa: *const super::Com::SAFEARRAY, ndim: u32, plubound: *mut i32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetVartype(psa: *const super::Com::SAFEARRAY, pvt: *mut u16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayLock(psa: *const super::Com::SAFEARRAY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayPtrOfIndex(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, ppvdata: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayPutElement(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, pv: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayRedim(psa: *mut super::Com::SAFEARRAY, psaboundnew: *const super::Com::SAFEARRAYBOUND) -> ::windows_core_sys::HRESULT;
    pub fn SafeArrayReleaseData(pdata: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayReleaseDescriptor(psa: *const super::Com::SAFEARRAY);
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArraySetIID(psa: *const super::Com::SAFEARRAY, guid: *const ::windows_core_sys::GUID) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArraySetRecordInfo(psa: *const super::Com::SAFEARRAY, prinfo: IRecordInfo) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayUnaccessData(psa: *const super::Com::SAFEARRAY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayUnlock(psa: *const super::Com::SAFEARRAY) -> ::windows_core_sys::HRESULT;
    pub fn SystemTimeToVariantTime(lpsystemtime: *const ::win32_foundation_sys::SYSTEMTIME, pvtime: *mut f64) -> i32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn UnRegisterTypeLib(libid: *const ::windows_core_sys::GUID, wvermajor: u16, wverminor: u16, lcid: u32, syskind: super::Com::SYSKIND) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn UnRegisterTypeLibForUser(libid: *const ::windows_core_sys::GUID, wmajorvernum: u16, wminorvernum: u16, lcid: u32, syskind: super::Com::SYSKIND) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarAbs(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarAdd(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarAnd(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarBoolFromCy(cyin: super::Com::CY, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBoolFromDate(datein: f64, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBoolFromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarBoolFromDisp(pdispin: super::Com::IDispatch, lcid: u32, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBoolFromI1(cin: ::win32_foundation_sys::CHAR, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBoolFromI2(sin: i16, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBoolFromI4(lin: i32, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBoolFromI8(i64in: i64, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBoolFromR4(fltin: f32, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBoolFromR8(dblin: f64, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBoolFromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBoolFromUI1(bin: u8, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBoolFromUI2(uiin: u16, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBoolFromUI4(ulin: u32, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBoolFromUI8(i64in: u64, pboolout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrCat(bstrleft: ::win32_foundation_sys::BSTR, bstrright: ::win32_foundation_sys::BSTR, pbstrresult: *mut *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrCmp(bstrleft: ::win32_foundation_sys::BSTR, bstrright: ::win32_foundation_sys::BSTR, lcid: u32, dwflags: u32) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrFromBool(boolin: i16, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarBstrFromCy(cyin: super::Com::CY, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrFromDate(datein: f64, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrFromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarBstrFromDisp(pdispin: super::Com::IDispatch, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrFromI1(cin: ::win32_foundation_sys::CHAR, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrFromI2(ival: i16, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrFromI4(lin: i32, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrFromI8(i64in: i64, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrFromR4(fltin: f32, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrFromR8(dblin: f64, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrFromUI1(bval: u8, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrFromUI2(uiin: u16, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrFromUI4(ulin: u32, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarBstrFromUI8(ui64in: u64, lcid: u32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCat(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCmp(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, lcid: u32, dwflags: u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyAbs(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyAdd(cyleft: super::Com::CY, cyright: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyCmp(cyleft: super::Com::CY, cyright: super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyCmpR8(cyleft: super::Com::CY, dblright: f64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFix(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromBool(boolin: i16, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromDate(datein: f64, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromDisp(pdispin: super::Com::IDispatch, lcid: u32, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI1(cin: ::win32_foundation_sys::CHAR, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI2(sin: i16, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI4(lin: i32, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI8(i64in: i64, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromR4(fltin: f32, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromR8(dblin: f64, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI1(bin: u8, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI2(uiin: u16, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI4(ulin: u32, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI8(ui64in: u64, pcyout: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyInt(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMul(cyleft: super::Com::CY, cyright: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMulI4(cyleft: super::Com::CY, lright: i32, pcyresult: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMulI8(cyleft: super::Com::CY, lright: i64, pcyresult: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyNeg(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyRound(cyin: super::Com::CY, cdecimals: i32, pcyresult: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCySub(cyleft: super::Com::CY, cyright: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromBool(boolin: i16, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarDateFromCy(cyin: super::Com::CY, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarDateFromDisp(pdispin: super::Com::IDispatch, lcid: u32, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromI1(cin: ::win32_foundation_sys::CHAR, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromI2(sin: i16, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromI4(lin: i32, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromI8(i64in: i64, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromR4(fltin: f32, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromR8(dblin: f64, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromUI1(bin: u8, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromUI2(uiin: u16, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromUI4(ulin: u32, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromUI8(ui64in: u64, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromUdate(pudatein: *const UDATE, dwflags: u32, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDateFromUdateEx(pudatein: *const UDATE, lcid: u32, dwflags: u32, pdateout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDecAbs(pdecin: *const ::win32_foundation_sys::DECIMAL, pdecresult: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecAdd(pdecleft: *const ::win32_foundation_sys::DECIMAL, pdecright: *const ::win32_foundation_sys::DECIMAL, pdecresult: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecCmp(pdecleft: *const ::win32_foundation_sys::DECIMAL, pdecright: *const ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecCmpR8(pdecleft: *const ::win32_foundation_sys::DECIMAL, dblright: f64) -> ::windows_core_sys::HRESULT;
    pub fn VarDecDiv(pdecleft: *const ::win32_foundation_sys::DECIMAL, pdecright: *const ::win32_foundation_sys::DECIMAL, pdecresult: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFix(pdecin: *const ::win32_foundation_sys::DECIMAL, pdecresult: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFromBool(boolin: i16, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarDecFromCy(cyin: super::Com::CY, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFromDate(datein: f64, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarDecFromDisp(pdispin: super::Com::IDispatch, lcid: u32, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFromI1(cin: ::win32_foundation_sys::CHAR, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFromI2(uiin: i16, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFromI4(lin: i32, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFromI8(i64in: i64, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFromR4(fltin: f32, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFromR8(dblin: f64, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFromUI1(bin: u8, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFromUI2(uiin: u16, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFromUI4(ulin: u32, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecFromUI8(ui64in: u64, pdecout: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecInt(pdecin: *const ::win32_foundation_sys::DECIMAL, pdecresult: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecMul(pdecleft: *const ::win32_foundation_sys::DECIMAL, pdecright: *const ::win32_foundation_sys::DECIMAL, pdecresult: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecNeg(pdecin: *const ::win32_foundation_sys::DECIMAL, pdecresult: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecRound(pdecin: *const ::win32_foundation_sys::DECIMAL, cdecimals: i32, pdecresult: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    pub fn VarDecSub(pdecleft: *const ::win32_foundation_sys::DECIMAL, pdecright: *const ::win32_foundation_sys::DECIMAL, pdecresult: *mut ::win32_foundation_sys::DECIMAL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarDiv(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarEqv(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarFix(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarFormat(pvarin: *const super::Com::VARIANT, pstrformat: ::windows_core_sys::PCWSTR, ifirstday: i32, ifirstweek: i32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarFormatCurrency(pvarin: *const super::Com::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarFormatDateTime(pvarin: *const super::Com::VARIANT, inamedformat: i32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarFormatFromTokens(pvarin: *const super::Com::VARIANT, pstrformat: ::windows_core_sys::PCWSTR, pbtokcur: *const u8, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR, lcid: u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarFormatNumber(pvarin: *const super::Com::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarFormatPercent(pvarin: *const super::Com::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI1FromBool(boolin: i16, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI1FromCy(cyin: super::Com::CY, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI1FromDate(datein: f64, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI1FromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI1FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI1FromI2(uiin: i16, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI1FromI4(lin: i32, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI1FromI8(i64in: i64, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI1FromR4(fltin: f32, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI1FromR8(dblin: f64, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI1FromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI1FromUI1(bin: u8, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI1FromUI2(uiin: u16, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI1FromUI4(ulin: u32, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI1FromUI8(i64in: u64, pcout: ::windows_core_sys::PSTR) -> ::windows_core_sys::HRESULT;
    pub fn VarI2FromBool(boolin: i16, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI2FromCy(cyin: super::Com::CY, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarI2FromDate(datein: f64, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarI2FromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI2FromDisp(pdispin: super::Com::IDispatch, lcid: u32, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarI2FromI1(cin: ::win32_foundation_sys::CHAR, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarI2FromI4(lin: i32, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarI2FromI8(i64in: i64, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarI2FromR4(fltin: f32, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarI2FromR8(dblin: f64, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarI2FromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarI2FromUI1(bin: u8, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarI2FromUI2(uiin: u16, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarI2FromUI4(ulin: u32, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarI2FromUI8(ui64in: u64, psout: *mut i16) -> ::windows_core_sys::HRESULT;
    pub fn VarI4FromBool(boolin: i16, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI4FromCy(cyin: super::Com::CY, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    pub fn VarI4FromDate(datein: f64, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    pub fn VarI4FromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI4FromDisp(pdispin: super::Com::IDispatch, lcid: u32, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    pub fn VarI4FromI1(cin: ::win32_foundation_sys::CHAR, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    pub fn VarI4FromI2(sin: i16, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    pub fn VarI4FromI8(i64in: i64, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    pub fn VarI4FromR4(fltin: f32, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    pub fn VarI4FromR8(dblin: f64, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    pub fn VarI4FromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    pub fn VarI4FromUI1(bin: u8, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    pub fn VarI4FromUI2(uiin: u16, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    pub fn VarI4FromUI4(ulin: u32, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    pub fn VarI4FromUI8(ui64in: u64, plout: *mut i32) -> ::windows_core_sys::HRESULT;
    pub fn VarI8FromBool(boolin: i16, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI8FromCy(cyin: super::Com::CY, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    pub fn VarI8FromDate(datein: f64, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    pub fn VarI8FromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI8FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    pub fn VarI8FromI1(cin: ::win32_foundation_sys::CHAR, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    pub fn VarI8FromI2(sin: i16, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    pub fn VarI8FromR4(fltin: f32, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    pub fn VarI8FromR8(dblin: f64, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    pub fn VarI8FromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    pub fn VarI8FromUI1(bin: u8, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    pub fn VarI8FromUI2(uiin: u16, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    pub fn VarI8FromUI4(ulin: u32, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    pub fn VarI8FromUI8(ui64in: u64, pi64out: *mut i64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarIdiv(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarImp(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarInt(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarMod(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    pub fn VarMonthName(imonth: i32, fabbrev: i32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarMul(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarNeg(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarNot(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarNumFromParseNum(pnumprs: *const NUMPARSE, rgbdig: *const u8, dwvtbits: u32, pvar: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarOr(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    pub fn VarParseNumFromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, pnumprs: *mut NUMPARSE, rgbdig: *mut u8) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarPow(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    pub fn VarR4CmpR8(fltleft: f32, dblright: f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR4FromBool(boolin: i16, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR4FromCy(cyin: super::Com::CY, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    pub fn VarR4FromDate(datein: f64, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    pub fn VarR4FromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR4FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    pub fn VarR4FromI1(cin: ::win32_foundation_sys::CHAR, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    pub fn VarR4FromI2(sin: i16, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    pub fn VarR4FromI4(lin: i32, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    pub fn VarR4FromI8(i64in: i64, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    pub fn VarR4FromR8(dblin: f64, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    pub fn VarR4FromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    pub fn VarR4FromUI1(bin: u8, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    pub fn VarR4FromUI2(uiin: u16, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    pub fn VarR4FromUI4(ulin: u32, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    pub fn VarR4FromUI8(ui64in: u64, pfltout: *mut f32) -> ::windows_core_sys::HRESULT;
    pub fn VarR8FromBool(boolin: i16, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR8FromCy(cyin: super::Com::CY, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8FromDate(datein: f64, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8FromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR8FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8FromI1(cin: ::win32_foundation_sys::CHAR, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8FromI2(sin: i16, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8FromI4(lin: i32, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8FromI8(i64in: i64, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8FromR4(fltin: f32, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8FromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8FromUI1(bin: u8, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8FromUI2(uiin: u16, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8FromUI4(ulin: u32, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8FromUI8(ui64in: u64, pdblout: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8Pow(dblleft: f64, dblright: f64, pdblresult: *mut f64) -> ::windows_core_sys::HRESULT;
    pub fn VarR8Round(dblin: f64, cdecimals: i32, pdblresult: *mut f64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarRound(pvarin: *const super::Com::VARIANT, cdecimals: i32, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarSub(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    pub fn VarTokenizeFormatString(pstrformat: ::windows_core_sys::PCWSTR, rgbtok: *mut u8, cbtok: i32, ifirstday: i32, ifirstweek: i32, lcid: u32, pcbactual: *const i32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI1FromBool(boolin: i16, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI1FromCy(cyin: super::Com::CY, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn VarUI1FromDate(datein: f64, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn VarUI1FromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI1FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn VarUI1FromI1(cin: ::win32_foundation_sys::CHAR, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn VarUI1FromI2(sin: i16, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn VarUI1FromI4(lin: i32, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn VarUI1FromI8(i64in: i64, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn VarUI1FromR4(fltin: f32, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn VarUI1FromR8(dblin: f64, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn VarUI1FromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn VarUI1FromUI2(uiin: u16, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn VarUI1FromUI4(ulin: u32, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn VarUI1FromUI8(ui64in: u64, pbout: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn VarUI2FromBool(boolin: i16, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI2FromCy(cyin: super::Com::CY, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarUI2FromDate(datein: f64, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarUI2FromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI2FromDisp(pdispin: super::Com::IDispatch, lcid: u32, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarUI2FromI1(cin: ::win32_foundation_sys::CHAR, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarUI2FromI2(uiin: i16, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarUI2FromI4(lin: i32, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarUI2FromI8(i64in: i64, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarUI2FromR4(fltin: f32, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarUI2FromR8(dblin: f64, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarUI2FromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarUI2FromUI1(bin: u8, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarUI2FromUI4(ulin: u32, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarUI2FromUI8(i64in: u64, puiout: *mut u16) -> ::windows_core_sys::HRESULT;
    pub fn VarUI4FromBool(boolin: i16, pulout: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI4FromCy(cyin: super::Com::CY, pulout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI4FromDate(datein: f64, pulout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI4FromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, pulout: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI4FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pulout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI4FromI1(cin: ::win32_foundation_sys::CHAR, pulout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI4FromI2(uiin: i16, pulout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI4FromI4(lin: i32, pulout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI4FromI8(i64in: i64, plout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI4FromR4(fltin: f32, pulout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI4FromR8(dblin: f64, pulout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI4FromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, pulout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI4FromUI1(bin: u8, pulout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI4FromUI2(uiin: u16, pulout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI4FromUI8(ui64in: u64, plout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn VarUI8FromBool(boolin: i16, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI8FromCy(cyin: super::Com::CY, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    pub fn VarUI8FromDate(datein: f64, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    pub fn VarUI8FromDec(pdecin: *const ::win32_foundation_sys::DECIMAL, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI8FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    pub fn VarUI8FromI1(cin: ::win32_foundation_sys::CHAR, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    pub fn VarUI8FromI2(sin: i16, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    pub fn VarUI8FromI8(ui64in: i64, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    pub fn VarUI8FromR4(fltin: f32, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    pub fn VarUI8FromR8(dblin: f64, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    pub fn VarUI8FromStr(strin: ::windows_core_sys::PCWSTR, lcid: u32, dwflags: u32, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    pub fn VarUI8FromUI1(bin: u8, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    pub fn VarUI8FromUI2(uiin: u16, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    pub fn VarUI8FromUI4(ulin: u32, pi64out: *mut u64) -> ::windows_core_sys::HRESULT;
    pub fn VarUdateFromDate(datein: f64, dwflags: u32, pudateout: *mut UDATE) -> ::windows_core_sys::HRESULT;
    pub fn VarWeekdayName(iweekday: i32, fabbrev: i32, ifirstday: i32, dwflags: u32, pbstrout: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarXor(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VariantChangeType(pvargdest: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, wflags: u16, vt: u16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VariantChangeTypeEx(pvargdest: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, lcid: u32, wflags: u16, vt: u16) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VariantClear(pvarg: *mut super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VariantCopy(pvargdest: *mut super::Com::VARIANT, pvargsrc: *const super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VariantCopyInd(pvardest: *mut super::Com::VARIANT, pvargsrc: *const super::Com::VARIANT) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VariantInit(pvarg: *mut super::Com::VARIANT);
    pub fn VariantTimeToDosDateTime(vtime: f64, pwdosdate: *mut u16, pwdostime: *mut u16) -> i32;
    pub fn VariantTimeToSystemTime(vtime: f64, lpsystemtime: *mut ::win32_foundation_sys::SYSTEMTIME) -> i32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VectorFromBstr(bstr: ::win32_foundation_sys::BSTR, ppsa: *mut *mut super::Com::SAFEARRAY) -> ::windows_core_sys::HRESULT;
}
pub type ACTIVATEFLAGS = i32;
pub const ACTIVATE_WINDOWLESS: ACTIVATEFLAGS = 1i32;
pub const ACTIVEOBJECT_STRONG: u32 = 0u32;
pub const ACTIVEOBJECT_WEAK: u32 = 1u32;
#[repr(C)]
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
pub type AspectInfoFlag = i32;
pub const DVASPECTINFOFLAG_CANOPTIMIZE: AspectInfoFlag = 1i32;
pub type BINDSPEED = i32;
pub const BINDSPEED_INDEFINITE: BINDSPEED = 1i32;
pub const BINDSPEED_MODERATE: BINDSPEED = 2i32;
pub const BINDSPEED_IMMEDIATE: BINDSPEED = 3i32;
pub const BZ_DISABLECANCELBUTTON: i32 = 1i32;
pub const BZ_DISABLERETRYBUTTON: i32 = 4i32;
pub const BZ_DISABLESWITCHTOBUTTON: i32 = 2i32;
pub const BZ_NOTRESPONDINGDIALOG: i32 = 8i32;
#[repr(C)]
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
pub struct CALPOLESTR {
    pub cElems: u32,
    pub pElems: *mut ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for CALPOLESTR {}
impl ::core::clone::Clone for CALPOLESTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAUUID {
    pub cElems: u32,
    pub pElems: *mut ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for CAUUID {}
impl ::core::clone::Clone for CAUUID {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_CONVERTONLY: i32 = 256i32;
pub const CF_DISABLEACTIVATEAS: i32 = 64i32;
pub const CF_DISABLEDISPLAYASICON: i32 = 32i32;
pub const CF_HIDECHANGEICON: i32 = 128i32;
pub const CF_SELECTACTIVATEAS: i32 = 16i32;
pub const CF_SELECTCONVERTTO: i32 = 8i32;
pub const CF_SETACTIVATEDEFAULT: i32 = 4i32;
pub const CF_SETCONVERTDEFAULT: i32 = 2i32;
pub const CF_SHOWHELPBUTTON: i32 = 1i32;
pub type CHANGEKIND = i32;
pub const CHANGEKIND_ADDMEMBER: CHANGEKIND = 0i32;
pub const CHANGEKIND_DELETEMEMBER: CHANGEKIND = 1i32;
pub const CHANGEKIND_SETNAMES: CHANGEKIND = 2i32;
pub const CHANGEKIND_SETDOCUMENTATION: CHANGEKIND = 3i32;
pub const CHANGEKIND_GENERAL: CHANGEKIND = 4i32;
pub const CHANGEKIND_INVALIDATE: CHANGEKIND = 5i32;
pub const CHANGEKIND_CHANGEFAILED: CHANGEKIND = 6i32;
pub const CHANGEKIND_MAX: CHANGEKIND = 7i32;
pub const CIF_SELECTCURRENT: i32 = 2i32;
pub const CIF_SELECTDEFAULT: i32 = 4i32;
pub const CIF_SELECTFROMFILE: i32 = 8i32;
pub const CIF_SHOWHELP: i32 = 1i32;
pub const CIF_USEICONEXE: i32 = 16i32;
#[repr(C)]
pub struct CLEANLOCALSTORAGE {
    pub pInterface: ::windows_core_sys::IUnknown,
    pub pStorage: *mut ::core::ffi::c_void,
    pub flags: u32,
}
impl ::core::marker::Copy for CLEANLOCALSTORAGE {}
impl ::core::clone::Clone for CLEANLOCALSTORAGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLSID_CColorPropPage: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 199447041, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CLSID_CFontPropPage: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 199447040, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CLSID_CPicturePropPage: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 199447042, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CLSID_ConvertVBX: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 4220454946, data2: 356, data3: 4123, data4: [132, 237, 8, 0, 43, 46, 199, 19] };
pub const CLSID_PersistPropset: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 4220454945, data2: 356, data3: 4123, data4: [132, 237, 8, 0, 43, 46, 199, 19] };
pub const CLSID_StdFont: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 199447043, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CLSID_StdPicture: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 199447044, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CONNECT_E_ADVISELIMIT: ::windows_core_sys::HRESULT = -2147220991i32;
pub const CONNECT_E_CANNOTCONNECT: ::windows_core_sys::HRESULT = -2147220990i32;
pub const CONNECT_E_FIRST: i32 = -2147220992i32;
pub const CONNECT_E_LAST: ::windows_core_sys::HRESULT = -2147220977i32;
pub const CONNECT_E_NOCONNECTION: ::windows_core_sys::HRESULT = -2147220992i32;
pub const CONNECT_E_OVERRIDDEN: ::windows_core_sys::HRESULT = -2147220989i32;
pub const CONNECT_S_FIRST: ::windows_core_sys::HRESULT = 262656i32;
pub const CONNECT_S_LAST: ::windows_core_sys::HRESULT = 262671i32;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct CONTROLINFO {
    pub cb: u32,
    pub hAccel: ::win32_ui_sys::WindowsAndMessaging::HACCEL,
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
pub const CSF_EXPLORER: i32 = 8i32;
pub const CSF_ONLYGETSOURCE: i32 = 4i32;
pub const CSF_SHOWHELP: i32 = 1i32;
pub const CSF_VALIDSOURCE: i32 = 2i32;
pub const CTL_E_ILLEGALFUNCTIONCALL: i32 = -2146828283i32;
pub type CTRLINFO = i32;
pub const CTRLINFO_EATS_RETURN: CTRLINFO = 1i32;
pub const CTRLINFO_EATS_ESCAPE: CTRLINFO = 2i32;
pub const DD_DEFDRAGDELAY: u32 = 200u32;
pub const DD_DEFDRAGMINDIST: u32 = 2u32;
pub const DD_DEFSCROLLDELAY: u32 = 50u32;
pub const DD_DEFSCROLLINSET: u32 = 11u32;
pub const DD_DEFSCROLLINTERVAL: u32 = 50u32;
pub type DISCARDCACHE = i32;
pub const DISCARDCACHE_SAVEIFDIRTY: DISCARDCACHE = 0i32;
pub const DISCARDCACHE_NOSAVE: DISCARDCACHE = 1i32;
pub const DISPATCH_CONSTRUCT: u32 = 16384u32;
pub const DISPATCH_METHOD: u32 = 1u32;
pub const DISPATCH_PROPERTYGET: u32 = 2u32;
pub const DISPATCH_PROPERTYPUT: u32 = 4u32;
pub const DISPATCH_PROPERTYPUTREF: u32 = 8u32;
pub const DISPID_ABOUTBOX: i32 = -552i32;
pub const DISPID_ACCELERATOR: i32 = -543i32;
pub const DISPID_ADDITEM: i32 = -553i32;
pub const DISPID_AMBIENT_APPEARANCE: i32 = -716i32;
pub const DISPID_AMBIENT_AUTOCLIP: i32 = -715i32;
pub const DISPID_AMBIENT_BACKCOLOR: i32 = -701i32;
pub const DISPID_AMBIENT_CHARSET: i32 = -727i32;
pub const DISPID_AMBIENT_CODEPAGE: i32 = -725i32;
pub const DISPID_AMBIENT_DISPLAYASDEFAULT: i32 = -713i32;
pub const DISPID_AMBIENT_DISPLAYNAME: i32 = -702i32;
pub const DISPID_AMBIENT_FONT: i32 = -703i32;
pub const DISPID_AMBIENT_FORECOLOR: i32 = -704i32;
pub const DISPID_AMBIENT_LOCALEID: i32 = -705i32;
pub const DISPID_AMBIENT_MESSAGEREFLECT: i32 = -706i32;
pub const DISPID_AMBIENT_PALETTE: i32 = -726i32;
pub const DISPID_AMBIENT_RIGHTTOLEFT: i32 = -732i32;
pub const DISPID_AMBIENT_SCALEUNITS: i32 = -707i32;
pub const DISPID_AMBIENT_SHOWGRABHANDLES: i32 = -711i32;
pub const DISPID_AMBIENT_SHOWHATCHING: i32 = -712i32;
pub const DISPID_AMBIENT_SUPPORTSMNEMONICS: i32 = -714i32;
pub const DISPID_AMBIENT_TEXTALIGN: i32 = -708i32;
pub const DISPID_AMBIENT_TOPTOBOTTOM: i32 = -733i32;
pub const DISPID_AMBIENT_TRANSFERPRIORITY: i32 = -728i32;
pub const DISPID_AMBIENT_UIDEAD: i32 = -710i32;
pub const DISPID_AMBIENT_USERMODE: i32 = -709i32;
pub const DISPID_APPEARANCE: i32 = -520i32;
pub const DISPID_AUTOSIZE: i32 = -500i32;
pub const DISPID_BACKCOLOR: i32 = -501i32;
pub const DISPID_BACKSTYLE: i32 = -502i32;
pub const DISPID_BORDERCOLOR: i32 = -503i32;
pub const DISPID_BORDERSTYLE: i32 = -504i32;
pub const DISPID_BORDERVISIBLE: i32 = -519i32;
pub const DISPID_BORDERWIDTH: i32 = -505i32;
pub const DISPID_CAPTION: i32 = -518i32;
pub const DISPID_CLEAR: i32 = -554i32;
pub const DISPID_CLICK: i32 = -600i32;
pub const DISPID_CLICK_VALUE: i32 = -610i32;
pub const DISPID_COLLECT: i32 = -8i32;
pub const DISPID_COLUMN: i32 = -529i32;
pub const DISPID_CONSTRUCTOR: i32 = -6i32;
pub const DISPID_DBLCLICK: i32 = -601i32;
pub const DISPID_DESTRUCTOR: i32 = -7i32;
pub const DISPID_DISPLAYSTYLE: i32 = -540i32;
pub const DISPID_DOCLICK: i32 = -551i32;
pub const DISPID_DRAWMODE: i32 = -507i32;
pub const DISPID_DRAWSTYLE: i32 = -508i32;
pub const DISPID_DRAWWIDTH: i32 = -509i32;
pub const DISPID_Delete: i32 = -801i32;
pub const DISPID_ENABLED: i32 = -514i32;
pub const DISPID_ENTERKEYBEHAVIOR: i32 = -544i32;
pub const DISPID_ERROREVENT: i32 = -608i32;
pub const DISPID_EVALUATE: i32 = -5i32;
pub const DISPID_FILLCOLOR: i32 = -510i32;
pub const DISPID_FILLSTYLE: i32 = -511i32;
pub const DISPID_FONT: i32 = -512i32;
pub const DISPID_FONT_BOLD: u32 = 3u32;
pub const DISPID_FONT_CHANGED: u32 = 9u32;
pub const DISPID_FONT_CHARSET: u32 = 8u32;
pub const DISPID_FONT_ITALIC: u32 = 4u32;
pub const DISPID_FONT_NAME: u32 = 0u32;
pub const DISPID_FONT_SIZE: u32 = 2u32;
pub const DISPID_FONT_STRIKE: u32 = 6u32;
pub const DISPID_FONT_UNDER: u32 = 5u32;
pub const DISPID_FONT_WEIGHT: u32 = 7u32;
pub const DISPID_FORECOLOR: i32 = -513i32;
pub const DISPID_GROUPNAME: i32 = -541i32;
pub const DISPID_HWND: i32 = -515i32;
pub const DISPID_IMEMODE: i32 = -542i32;
pub const DISPID_KEYDOWN: i32 = -602i32;
pub const DISPID_KEYPRESS: i32 = -603i32;
pub const DISPID_KEYUP: i32 = -604i32;
pub const DISPID_LIST: i32 = -528i32;
pub const DISPID_LISTCOUNT: i32 = -531i32;
pub const DISPID_LISTINDEX: i32 = -526i32;
pub const DISPID_MAXLENGTH: i32 = -533i32;
pub const DISPID_MOUSEDOWN: i32 = -605i32;
pub const DISPID_MOUSEICON: i32 = -522i32;
pub const DISPID_MOUSEMOVE: i32 = -606i32;
pub const DISPID_MOUSEPOINTER: i32 = -521i32;
pub const DISPID_MOUSEUP: i32 = -607i32;
pub const DISPID_MULTILINE: i32 = -537i32;
pub const DISPID_MULTISELECT: i32 = -532i32;
pub const DISPID_NEWENUM: i32 = -4i32;
pub const DISPID_NUMBEROFCOLUMNS: i32 = -539i32;
pub const DISPID_NUMBEROFROWS: i32 = -538i32;
pub const DISPID_Name: i32 = -800i32;
pub const DISPID_Object: i32 = -802i32;
pub const DISPID_PASSWORDCHAR: i32 = -534i32;
pub const DISPID_PICTURE: i32 = -523i32;
pub const DISPID_PICT_HANDLE: u32 = 0u32;
pub const DISPID_PICT_HEIGHT: u32 = 5u32;
pub const DISPID_PICT_HPAL: u32 = 2u32;
pub const DISPID_PICT_RENDER: u32 = 6u32;
pub const DISPID_PICT_TYPE: u32 = 3u32;
pub const DISPID_PICT_WIDTH: u32 = 4u32;
pub const DISPID_PROPERTYPUT: i32 = -3i32;
pub const DISPID_Parent: i32 = -803i32;
pub const DISPID_READYSTATE: i32 = -525i32;
pub const DISPID_READYSTATECHANGE: i32 = -609i32;
pub const DISPID_REFRESH: i32 = -550i32;
pub const DISPID_REMOVEITEM: i32 = -555i32;
pub const DISPID_RIGHTTOLEFT: i32 = -611i32;
pub const DISPID_SCROLLBARS: i32 = -535i32;
pub const DISPID_SELECTED: i32 = -527i32;
pub const DISPID_SELLENGTH: i32 = -548i32;
pub const DISPID_SELSTART: i32 = -547i32;
pub const DISPID_SELTEXT: i32 = -546i32;
pub const DISPID_STARTENUM: i32 = -1i32;
pub const DISPID_TABKEYBEHAVIOR: i32 = -545i32;
pub const DISPID_TABSTOP: i32 = -516i32;
pub const DISPID_TEXT: i32 = -517i32;
pub const DISPID_THIS: i32 = -613i32;
pub const DISPID_TOPTOBOTTOM: i32 = -612i32;
pub const DISPID_UNKNOWN: i32 = -1i32;
pub const DISPID_VALID: i32 = -524i32;
pub const DISPID_VALUE: u32 = 0u32;
pub const DISPID_WORDWRAP: i32 = -536i32;
pub type DOCMISC = i32;
pub const DOCMISC_CANCREATEMULTIPLEVIEWS: DOCMISC = 1i32;
pub const DOCMISC_SUPPORTCOMPLEXRECTANGLES: DOCMISC = 2i32;
pub const DOCMISC_CANTOPENEDIT: DOCMISC = 4i32;
pub const DOCMISC_NOFILESUPPORT: DOCMISC = 8i32;
pub const DROPEFFECT_COPY: u32 = 1u32;
pub const DROPEFFECT_LINK: u32 = 4u32;
pub const DROPEFFECT_MOVE: u32 = 2u32;
pub const DROPEFFECT_NONE: u32 = 0u32;
pub const DROPEFFECT_SCROLL: u32 = 2147483648u32;
pub type DVASPECT2 = i32;
pub const DVASPECT_OPAQUE: DVASPECT2 = 16i32;
pub const DVASPECT_TRANSPARENT: DVASPECT2 = 32i32;
pub const ELF_DISABLECANCELLINK: i32 = 16i32;
pub const ELF_DISABLECHANGESOURCE: i32 = 8i32;
pub const ELF_DISABLEOPENSOURCE: i32 = 4i32;
pub const ELF_DISABLEUPDATENOW: i32 = 2i32;
pub const ELF_SHOWHELP: i32 = 1i32;
pub const EMBDHLP_CREATENOW: i32 = 0i32;
pub const EMBDHLP_DELAYCREATE: i32 = 65536i32;
pub const EMBDHLP_INPROC_HANDLER: i32 = 0i32;
pub const EMBDHLP_INPROC_SERVER: i32 = 1i32;
pub type ENUM_CONTROLS_WHICH_FLAGS = u32;
pub const GCW_WCH_SIBLING: ENUM_CONTROLS_WHICH_FLAGS = 1u32;
pub const GC_WCH_CONTAINER: ENUM_CONTROLS_WHICH_FLAGS = 2u32;
pub const GC_WCH_CONTAINED: ENUM_CONTROLS_WHICH_FLAGS = 3u32;
pub const GC_WCH_ALL: ENUM_CONTROLS_WHICH_FLAGS = 4u32;
pub const GC_WCH_FREVERSEDIR: ENUM_CONTROLS_WHICH_FLAGS = 134217728u32;
pub const GC_WCH_FONLYAFTER: ENUM_CONTROLS_WHICH_FLAGS = 268435456u32;
pub const GC_WCH_FONLYBEFORE: ENUM_CONTROLS_WHICH_FLAGS = 536870912u32;
pub const GC_WCH_FSELECTED: ENUM_CONTROLS_WHICH_FLAGS = 1073741824u32;
#[repr(C)]
pub struct ExtentInfo {
    pub cb: u32,
    pub dwExtentMode: u32,
    pub sizelProposed: ::win32_foundation_sys::SIZE,
}
impl ::core::marker::Copy for ExtentInfo {}
impl ::core::clone::Clone for ExtentInfo {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ExtentMode = i32;
pub const DVEXTENT_CONTENT: ExtentMode = 0i32;
pub const DVEXTENT_INTEGRAL: ExtentMode = 1i32;
pub const FADF_AUTO: u32 = 1u32;
pub const FADF_BSTR: u32 = 256u32;
pub const FADF_DISPATCH: u32 = 1024u32;
pub const FADF_EMBEDDED: u32 = 4u32;
pub const FADF_FIXEDSIZE: u32 = 16u32;
pub const FADF_HAVEIID: u32 = 64u32;
pub const FADF_HAVEVARTYPE: u32 = 128u32;
pub const FADF_RECORD: u32 = 32u32;
pub const FADF_RESERVED: u32 = 61448u32;
pub const FADF_STATIC: u32 = 2u32;
pub const FADF_UNKNOWN: u32 = 512u32;
pub const FADF_VARIANT: u32 = 2048u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct FONTDESC {
    pub cbSizeofstruct: u32,
    pub lpstrName: ::windows_core_sys::PWSTR,
    pub cySize: super::Com::CY,
    pub sWeight: i16,
    pub sCharset: i16,
    pub fItalic: ::win32_foundation_sys::BOOL,
    pub fUnderline: ::win32_foundation_sys::BOOL,
    pub fStrikethrough: ::win32_foundation_sys::BOOL,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for FONTDESC {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for FONTDESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FUNCFLAGS = i32;
pub const FUNCFLAG_FRESTRICTED: FUNCFLAGS = 1i32;
pub const FUNCFLAG_FSOURCE: FUNCFLAGS = 2i32;
pub const FUNCFLAG_FBINDABLE: FUNCFLAGS = 4i32;
pub const FUNCFLAG_FREQUESTEDIT: FUNCFLAGS = 8i32;
pub const FUNCFLAG_FDISPLAYBIND: FUNCFLAGS = 16i32;
pub const FUNCFLAG_FDEFAULTBIND: FUNCFLAGS = 32i32;
pub const FUNCFLAG_FHIDDEN: FUNCFLAGS = 64i32;
pub const FUNCFLAG_FUSESGETLASTERROR: FUNCFLAGS = 128i32;
pub const FUNCFLAG_FDEFAULTCOLLELEM: FUNCFLAGS = 256i32;
pub const FUNCFLAG_FUIDEFAULT: FUNCFLAGS = 512i32;
pub const FUNCFLAG_FNONBROWSABLE: FUNCFLAGS = 1024i32;
pub const FUNCFLAG_FREPLACEABLE: FUNCFLAGS = 2048i32;
pub const FUNCFLAG_FIMMEDIATEBIND: FUNCFLAGS = 4096i32;
pub const GC_WCH_SIBLING: i32 = 1i32;
pub type GUIDKIND = i32;
pub const GUIDKIND_DEFAULT_SOURCE_DISP_IID: GUIDKIND = 1i32;
pub const GUID_CHECKVALUEEXCLUSIVE: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536076, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_COLOR: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536065, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTBOLD: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536079, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTITALIC: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536080, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTNAME: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536077, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTSIZE: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536078, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTSTRIKETHROUGH: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536082, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTUNDERSCORE: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536081, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_HANDLE: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536083, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_HIMETRIC: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536064, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_OPTIONVALUEEXCLUSIVE: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536075, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_TRISTATE: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536074, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_XPOS: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536070, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_XPOSPIXEL: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536066, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_XSIZE: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536072, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_XSIZEPIXEL: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536068, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_YPOS: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536071, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_YPOSPIXEL: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536067, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_YSIZE: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536073, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_YSIZEPIXEL: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1716536069, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub type HITRESULT = i32;
pub const HITRESULT_OUTSIDE: HITRESULT = 0i32;
pub const HITRESULT_TRANSPARENT: HITRESULT = 1i32;
pub const HITRESULT_CLOSE: HITRESULT = 2i32;
pub const HITRESULT_HIT: HITRESULT = 3i32;
pub type IAdviseSinkEx = *mut ::core::ffi::c_void;
pub type ICanHandleException = *mut ::core::ffi::c_void;
pub type IClassFactory2 = *mut ::core::ffi::c_void;
pub type IContinue = *mut ::core::ffi::c_void;
pub type IContinueCallback = *mut ::core::ffi::c_void;
pub type ICreateErrorInfo = *mut ::core::ffi::c_void;
pub type ICreateTypeInfo = *mut ::core::ffi::c_void;
pub type ICreateTypeInfo2 = *mut ::core::ffi::c_void;
pub type ICreateTypeLib = *mut ::core::ffi::c_void;
pub type ICreateTypeLib2 = *mut ::core::ffi::c_void;
pub const IDC_BZ_ICON: u32 = 601u32;
pub const IDC_BZ_MESSAGE1: u32 = 602u32;
pub const IDC_BZ_RETRY: u32 = 600u32;
pub const IDC_BZ_SWITCHTO: u32 = 604u32;
pub const IDC_CI_BROWSE: u32 = 130u32;
pub const IDC_CI_CURRENT: u32 = 121u32;
pub const IDC_CI_CURRENTICON: u32 = 122u32;
pub const IDC_CI_DEFAULT: u32 = 123u32;
pub const IDC_CI_DEFAULTICON: u32 = 124u32;
pub const IDC_CI_FROMFILE: u32 = 125u32;
pub const IDC_CI_FROMFILEEDIT: u32 = 126u32;
pub const IDC_CI_GROUP: u32 = 120u32;
pub const IDC_CI_ICONDISPLAY: u32 = 131u32;
pub const IDC_CI_ICONLIST: u32 = 127u32;
pub const IDC_CI_LABEL: u32 = 128u32;
pub const IDC_CI_LABELEDIT: u32 = 129u32;
pub const IDC_CV_ACTIVATEAS: u32 = 156u32;
pub const IDC_CV_ACTIVATELIST: u32 = 154u32;
pub const IDC_CV_CHANGEICON: u32 = 153u32;
pub const IDC_CV_CONVERTLIST: u32 = 158u32;
pub const IDC_CV_CONVERTTO: u32 = 155u32;
pub const IDC_CV_DISPLAYASICON: u32 = 152u32;
pub const IDC_CV_ICONDISPLAY: u32 = 165u32;
pub const IDC_CV_OBJECTTYPE: u32 = 150u32;
pub const IDC_CV_RESULTTEXT: u32 = 157u32;
pub const IDC_EL_AUTOMATIC: u32 = 202u32;
pub const IDC_EL_CANCELLINK: u32 = 209u32;
pub const IDC_EL_CHANGESOURCE: u32 = 201u32;
pub const IDC_EL_COL1: u32 = 220u32;
pub const IDC_EL_COL2: u32 = 221u32;
pub const IDC_EL_COL3: u32 = 222u32;
pub const IDC_EL_LINKSLISTBOX: u32 = 206u32;
pub const IDC_EL_LINKSOURCE: u32 = 216u32;
pub const IDC_EL_LINKTYPE: u32 = 217u32;
pub const IDC_EL_MANUAL: u32 = 212u32;
pub const IDC_EL_OPENSOURCE: u32 = 211u32;
pub const IDC_EL_UPDATENOW: u32 = 210u32;
pub const IDC_GP_CONVERT: u32 = 1013u32;
pub const IDC_GP_OBJECTICON: u32 = 1014u32;
pub const IDC_GP_OBJECTLOCATION: u32 = 1022u32;
pub const IDC_GP_OBJECTNAME: u32 = 1009u32;
pub const IDC_GP_OBJECTSIZE: u32 = 1011u32;
pub const IDC_GP_OBJECTTYPE: u32 = 1010u32;
pub const IDC_IO_ADDCONTROL: u32 = 2115u32;
pub const IDC_IO_CHANGEICON: u32 = 2105u32;
pub const IDC_IO_CONTROLTYPELIST: u32 = 2116u32;
pub const IDC_IO_CREATEFROMFILE: u32 = 2101u32;
pub const IDC_IO_CREATENEW: u32 = 2100u32;
pub const IDC_IO_DISPLAYASICON: u32 = 2104u32;
pub const IDC_IO_FILE: u32 = 2106u32;
pub const IDC_IO_FILEDISPLAY: u32 = 2107u32;
pub const IDC_IO_FILETEXT: u32 = 2112u32;
pub const IDC_IO_FILETYPE: u32 = 2113u32;
pub const IDC_IO_ICONDISPLAY: u32 = 2110u32;
pub const IDC_IO_INSERTCONTROL: u32 = 2114u32;
pub const IDC_IO_LINKFILE: u32 = 2102u32;
pub const IDC_IO_OBJECTTYPELIST: u32 = 2103u32;
pub const IDC_IO_OBJECTTYPETEXT: u32 = 2111u32;
pub const IDC_IO_RESULTIMAGE: u32 = 2108u32;
pub const IDC_IO_RESULTTEXT: u32 = 2109u32;
pub const IDC_LP_AUTOMATIC: u32 = 1016u32;
pub const IDC_LP_BREAKLINK: u32 = 1008u32;
pub const IDC_LP_CHANGESOURCE: u32 = 1015u32;
pub const IDC_LP_DATE: u32 = 1018u32;
pub const IDC_LP_LINKSOURCE: u32 = 1012u32;
pub const IDC_LP_MANUAL: u32 = 1017u32;
pub const IDC_LP_OPENSOURCE: u32 = 1006u32;
pub const IDC_LP_TIME: u32 = 1019u32;
pub const IDC_LP_UPDATENOW: u32 = 1007u32;
pub const IDC_OLEUIHELP: u32 = 99u32;
pub const IDC_PS_CHANGEICON: u32 = 508u32;
pub const IDC_PS_DISPLAYASICON: u32 = 506u32;
pub const IDC_PS_DISPLAYLIST: u32 = 505u32;
pub const IDC_PS_ICONDISPLAY: u32 = 507u32;
pub const IDC_PS_PASTE: u32 = 500u32;
pub const IDC_PS_PASTELINK: u32 = 501u32;
pub const IDC_PS_PASTELINKLIST: u32 = 504u32;
pub const IDC_PS_PASTELIST: u32 = 503u32;
pub const IDC_PS_RESULTIMAGE: u32 = 509u32;
pub const IDC_PS_RESULTTEXT: u32 = 510u32;
pub const IDC_PS_SOURCETEXT: u32 = 502u32;
pub const IDC_PU_CONVERT: u32 = 902u32;
pub const IDC_PU_ICON: u32 = 908u32;
pub const IDC_PU_LINKS: u32 = 900u32;
pub const IDC_PU_TEXT: u32 = 901u32;
pub const IDC_UL_METER: u32 = 1029u32;
pub const IDC_UL_PERCENT: u32 = 1031u32;
pub const IDC_UL_PROGRESS: u32 = 1032u32;
pub const IDC_UL_STOP: u32 = 1030u32;
pub const IDC_VP_ASICON: u32 = 1003u32;
pub const IDC_VP_CHANGEICON: u32 = 1001u32;
pub const IDC_VP_EDITABLE: u32 = 1002u32;
pub const IDC_VP_ICONDISPLAY: u32 = 1021u32;
pub const IDC_VP_PERCENT: u32 = 1000u32;
pub const IDC_VP_RELATIVE: u32 = 1005u32;
pub const IDC_VP_RESULTIMAGE: u32 = 1033u32;
pub const IDC_VP_SCALETXT: u32 = 1034u32;
pub const IDC_VP_SPIN: u32 = 1006u32;
pub const IDD_BUSY: u32 = 1006u32;
pub const IDD_CANNOTUPDATELINK: u32 = 1008u32;
pub const IDD_CHANGEICON: u32 = 1001u32;
pub const IDD_CHANGEICONBROWSE: u32 = 1011u32;
pub const IDD_CHANGESOURCE: u32 = 1009u32;
pub const IDD_CHANGESOURCE4: u32 = 1013u32;
pub const IDD_CONVERT: u32 = 1002u32;
pub const IDD_CONVERT4: u32 = 1103u32;
pub const IDD_CONVERTONLY: u32 = 1012u32;
pub const IDD_CONVERTONLY4: u32 = 1104u32;
pub const IDD_EDITLINKS: u32 = 1004u32;
pub const IDD_EDITLINKS4: u32 = 1105u32;
pub const IDD_GNRLPROPS: u32 = 1100u32;
pub const IDD_GNRLPROPS4: u32 = 1106u32;
pub const IDD_INSERTFILEBROWSE: u32 = 1010u32;
pub const IDD_INSERTOBJECT: u32 = 1000u32;
pub const IDD_LINKPROPS: u32 = 1102u32;
pub const IDD_LINKPROPS4: u32 = 1107u32;
pub const IDD_LINKSOURCEUNAVAILABLE: u32 = 1020u32;
pub const IDD_LINKTYPECHANGED: u32 = 1022u32;
pub const IDD_LINKTYPECHANGEDA: u32 = 1026u32;
pub const IDD_LINKTYPECHANGEDW: u32 = 1022u32;
pub const IDD_OUTOFMEMORY: u32 = 1024u32;
pub const IDD_PASTESPECIAL: u32 = 1003u32;
pub const IDD_PASTESPECIAL4: u32 = 1108u32;
pub const IDD_SERVERNOTFOUND: u32 = 1023u32;
pub const IDD_SERVERNOTREG: u32 = 1021u32;
pub const IDD_SERVERNOTREGA: u32 = 1025u32;
pub const IDD_SERVERNOTREGW: u32 = 1021u32;
pub const IDD_UPDATELINKS: u32 = 1007u32;
pub const IDD_VIEWPROPS: u32 = 1101u32;
pub const IDLFLAG_FIN: u32 = 1u32;
pub const IDLFLAG_FLCID: u32 = 4u32;
pub const IDLFLAG_FOUT: u32 = 2u32;
pub const IDLFLAG_FRETVAL: u32 = 8u32;
pub const IDLFLAG_NONE: u32 = 0u32;
pub const ID_BROWSE_ADDCONTROL: u32 = 3u32;
pub const ID_BROWSE_CHANGEICON: u32 = 1u32;
pub const ID_BROWSE_CHANGESOURCE: u32 = 4u32;
pub const ID_BROWSE_INSERTFILE: u32 = 2u32;
pub const ID_DEFAULTINST: i32 = -2i32;
pub type IDispError = *mut ::core::ffi::c_void;
pub type IDispatchEx = *mut ::core::ffi::c_void;
pub type IDropSource = *mut ::core::ffi::c_void;
pub type IDropSourceNotify = *mut ::core::ffi::c_void;
pub type IDropTarget = *mut ::core::ffi::c_void;
pub type IEnterpriseDropTarget = *mut ::core::ffi::c_void;
pub type IEnumOLEVERB = *mut ::core::ffi::c_void;
pub type IEnumOleDocumentViews = *mut ::core::ffi::c_void;
pub type IEnumOleUndoUnits = *mut ::core::ffi::c_void;
pub type IEnumVARIANT = *mut ::core::ffi::c_void;
pub type IFont = *mut ::core::ffi::c_void;
pub type IFontDisp = *mut ::core::ffi::c_void;
pub type IFontEventsDisp = *mut ::core::ffi::c_void;
pub type IGNOREMIME = i32;
pub const IGNOREMIME_PROMPT: IGNOREMIME = 1i32;
pub const IGNOREMIME_TEXT: IGNOREMIME = 2i32;
pub type IGetOleObject = *mut ::core::ffi::c_void;
pub type IGetVBAObject = *mut ::core::ffi::c_void;
pub const IMPLTYPEFLAG_FDEFAULT: u32 = 1u32;
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: u32 = 8u32;
pub const IMPLTYPEFLAG_FRESTRICTED: u32 = 4u32;
pub const IMPLTYPEFLAG_FSOURCE: u32 = 2u32;
pub const INSTALL_SCOPE_INVALID: u32 = 0u32;
pub const INSTALL_SCOPE_MACHINE: u32 = 1u32;
pub const INSTALL_SCOPE_USER: u32 = 2u32;
#[repr(C)]
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
pub const IOF_CHECKDISPLAYASICON: i32 = 16i32;
pub const IOF_CHECKLINK: i32 = 8i32;
pub const IOF_CREATEFILEOBJECT: i32 = 64i32;
pub const IOF_CREATELINKOBJECT: i32 = 128i32;
pub const IOF_CREATENEWOBJECT: i32 = 32i32;
pub const IOF_DISABLEDISPLAYASICON: i32 = 1024i32;
pub const IOF_DISABLELINK: i32 = 256i32;
pub const IOF_HIDECHANGEICON: i32 = 2048i32;
pub const IOF_SELECTCREATECONTROL: i32 = 8192i32;
pub const IOF_SELECTCREATEFROMFILE: i32 = 4i32;
pub const IOF_SELECTCREATENEW: i32 = 2i32;
pub const IOF_SHOWHELP: i32 = 1i32;
pub const IOF_SHOWINSERTCONTROL: i32 = 4096i32;
pub const IOF_VERIFYSERVERSEXIST: i32 = 512i32;
pub type IObjectIdentity = *mut ::core::ffi::c_void;
pub type IObjectWithSite = *mut ::core::ffi::c_void;
pub type IOleAdviseHolder = *mut ::core::ffi::c_void;
pub type IOleCache = *mut ::core::ffi::c_void;
pub type IOleCache2 = *mut ::core::ffi::c_void;
pub type IOleCacheControl = *mut ::core::ffi::c_void;
pub type IOleClientSite = *mut ::core::ffi::c_void;
pub type IOleCommandTarget = *mut ::core::ffi::c_void;
pub type IOleContainer = *mut ::core::ffi::c_void;
pub type IOleControl = *mut ::core::ffi::c_void;
pub type IOleControlSite = *mut ::core::ffi::c_void;
pub type IOleDocument = *mut ::core::ffi::c_void;
pub type IOleDocumentSite = *mut ::core::ffi::c_void;
pub type IOleDocumentView = *mut ::core::ffi::c_void;
pub type IOleInPlaceActiveObject = *mut ::core::ffi::c_void;
pub type IOleInPlaceFrame = *mut ::core::ffi::c_void;
pub type IOleInPlaceObject = *mut ::core::ffi::c_void;
pub type IOleInPlaceObjectWindowless = *mut ::core::ffi::c_void;
pub type IOleInPlaceSite = *mut ::core::ffi::c_void;
pub type IOleInPlaceSiteEx = *mut ::core::ffi::c_void;
pub type IOleInPlaceSiteWindowless = *mut ::core::ffi::c_void;
pub type IOleInPlaceUIWindow = *mut ::core::ffi::c_void;
pub type IOleItemContainer = *mut ::core::ffi::c_void;
pub type IOleLink = *mut ::core::ffi::c_void;
pub type IOleObject = *mut ::core::ffi::c_void;
pub type IOleParentUndoUnit = *mut ::core::ffi::c_void;
pub type IOleUILinkContainerA = *mut ::core::ffi::c_void;
pub type IOleUILinkContainerW = *mut ::core::ffi::c_void;
pub type IOleUILinkInfoA = *mut ::core::ffi::c_void;
pub type IOleUILinkInfoW = *mut ::core::ffi::c_void;
pub type IOleUIObjInfoA = *mut ::core::ffi::c_void;
pub type IOleUIObjInfoW = *mut ::core::ffi::c_void;
pub type IOleUndoManager = *mut ::core::ffi::c_void;
pub type IOleUndoUnit = *mut ::core::ffi::c_void;
pub type IOleWindow = *mut ::core::ffi::c_void;
pub type IParseDisplayName = *mut ::core::ffi::c_void;
pub type IPerPropertyBrowsing = *mut ::core::ffi::c_void;
pub type IPersistPropertyBag = *mut ::core::ffi::c_void;
pub type IPersistPropertyBag2 = *mut ::core::ffi::c_void;
pub type IPicture = *mut ::core::ffi::c_void;
pub type IPicture2 = *mut ::core::ffi::c_void;
pub type IPictureDisp = *mut ::core::ffi::c_void;
pub type IPointerInactive = *mut ::core::ffi::c_void;
pub type IPrint = *mut ::core::ffi::c_void;
pub type IPropertyNotifySink = *mut ::core::ffi::c_void;
pub type IPropertyPage = *mut ::core::ffi::c_void;
pub type IPropertyPage2 = *mut ::core::ffi::c_void;
pub type IPropertyPageSite = *mut ::core::ffi::c_void;
pub type IProtectFocus = *mut ::core::ffi::c_void;
pub type IProtectedModeMenuServices = *mut ::core::ffi::c_void;
pub type IProvideClassInfo = *mut ::core::ffi::c_void;
pub type IProvideClassInfo2 = *mut ::core::ffi::c_void;
pub type IProvideMultipleClassInfo = *mut ::core::ffi::c_void;
pub type IProvideRuntimeContext = *mut ::core::ffi::c_void;
pub type IQuickActivate = *mut ::core::ffi::c_void;
pub type IRecordInfo = *mut ::core::ffi::c_void;
pub type ISimpleFrameSite = *mut ::core::ffi::c_void;
pub type ISpecifyPropertyPages = *mut ::core::ffi::c_void;
pub type ITypeChangeEvents = *mut ::core::ffi::c_void;
pub type ITypeFactory = *mut ::core::ffi::c_void;
pub type ITypeMarshal = *mut ::core::ffi::c_void;
pub type IVBFormat = *mut ::core::ffi::c_void;
pub type IVBGetControl = *mut ::core::ffi::c_void;
pub type IVariantChangeType = *mut ::core::ffi::c_void;
pub type IViewObject = *mut ::core::ffi::c_void;
pub type IViewObject2 = *mut ::core::ffi::c_void;
pub type IViewObjectEx = *mut ::core::ffi::c_void;
pub type IZoomEvents = *mut ::core::ffi::c_void;
pub type LIBFLAGS = i32;
pub const LIBFLAG_FRESTRICTED: LIBFLAGS = 1i32;
pub const LIBFLAG_FCONTROL: LIBFLAGS = 2i32;
pub const LIBFLAG_FHIDDEN: LIBFLAGS = 4i32;
pub const LIBFLAG_FHASDISKIMAGE: LIBFLAGS = 8i32;
#[repr(C)]
pub struct LICINFO {
    pub cbLicInfo: i32,
    pub fRuntimeKeyAvail: ::win32_foundation_sys::BOOL,
    pub fLicVerified: ::win32_foundation_sys::BOOL,
}
impl ::core::marker::Copy for LICINFO {}
impl ::core::clone::Clone for LICINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LOAD_TLB_AS_32BIT: u32 = 32u32;
pub const LOAD_TLB_AS_64BIT: u32 = 64u32;
pub const LOCALE_USE_NLS: u32 = 268435456u32;
pub type LPFNOLEUIHOOK = ::core::option::Option<unsafe extern "system" fn(param0: ::win32_foundation_sys::HWND, param1: u32, param2: ::win32_foundation_sys::WPARAM, param3: ::win32_foundation_sys::LPARAM) -> u32>;
pub const LP_COLOR: u32 = 4u32;
pub const LP_DEFAULT: u32 = 0u32;
pub const LP_MONOCHROME: u32 = 1u32;
pub const LP_VGACOLOR: u32 = 2u32;
pub type MEDIAPLAYBACK_STATE = i32;
pub const MEDIAPLAYBACK_RESUME: MEDIAPLAYBACK_STATE = 0i32;
pub const MEDIAPLAYBACK_PAUSE: MEDIAPLAYBACK_STATE = 1i32;
pub const MEDIAPLAYBACK_PAUSE_AND_SUSPEND: MEDIAPLAYBACK_STATE = 2i32;
pub const MEDIAPLAYBACK_RESUME_FROM_SUSPEND: MEDIAPLAYBACK_STATE = 3i32;
pub const MEMBERID_NIL: i32 = -1i32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct METHODDATA {
    pub szName: ::windows_core_sys::PWSTR,
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
pub const MK_ALT: u32 = 32u32;
pub const MSOCMDERR_E_CANCELED: i32 = -2147221245i32;
pub const MSOCMDERR_E_DISABLED: i32 = -2147221247i32;
pub const MSOCMDERR_E_FIRST: i32 = -2147221248i32;
pub const MSOCMDERR_E_NOHELP: i32 = -2147221246i32;
pub const MSOCMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
pub const MSOCMDERR_E_UNKNOWNGROUP: i32 = -2147221244i32;
pub type MULTICLASSINFO_FLAGS = u32;
pub const MULTICLASSINFO_GETTYPEINFO: MULTICLASSINFO_FLAGS = 1u32;
pub const MULTICLASSINFO_GETNUMRESERVEDDISPIDS: MULTICLASSINFO_FLAGS = 2u32;
pub const MULTICLASSINFO_GETIIDPRIMARY: MULTICLASSINFO_FLAGS = 4u32;
pub const MULTICLASSINFO_GETIIDSOURCE: MULTICLASSINFO_FLAGS = 8u32;
#[repr(C)]
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
pub const NUMPRS_CURRENCY: u32 = 1024u32;
pub const NUMPRS_DECIMAL: u32 = 256u32;
pub const NUMPRS_EXPONENT: u32 = 2048u32;
pub const NUMPRS_HEX_OCT: u32 = 64u32;
pub const NUMPRS_INEXACT: u32 = 131072u32;
pub const NUMPRS_LEADING_MINUS: u32 = 16u32;
pub const NUMPRS_LEADING_PLUS: u32 = 4u32;
pub const NUMPRS_LEADING_WHITE: u32 = 1u32;
pub const NUMPRS_NEG: u32 = 65536u32;
pub const NUMPRS_PARENS: u32 = 128u32;
pub const NUMPRS_STD: u32 = 8191u32;
pub const NUMPRS_THOUSANDS: u32 = 512u32;
pub const NUMPRS_TRAILING_MINUS: u32 = 32u32;
pub const NUMPRS_TRAILING_PLUS: u32 = 8u32;
pub const NUMPRS_TRAILING_WHITE: u32 = 2u32;
pub const NUMPRS_USE_ALL: u32 = 4096u32;
#[repr(C)]
pub struct OBJECTDESCRIPTOR {
    pub cbSize: u32,
    pub clsid: ::windows_core_sys::GUID,
    pub dwDrawAspect: u32,
    pub sizel: ::win32_foundation_sys::SIZE,
    pub pointl: ::win32_foundation_sys::POINTL,
    pub dwStatus: u32,
    pub dwFullUserTypeName: u32,
    pub dwSrcOfCopy: u32,
}
impl ::core::marker::Copy for OBJECTDESCRIPTOR {}
impl ::core::clone::Clone for OBJECTDESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
pub const OCM__BASE: u32 = 8192u32;
#[repr(C)]
pub struct OCPFIPARAMS {
    pub cbStructSize: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub x: i32,
    pub y: i32,
    pub lpszCaption: ::windows_core_sys::PCWSTR,
    pub cObjects: u32,
    pub lplpUnk: *mut ::windows_core_sys::IUnknown,
    pub cPages: u32,
    pub lpPages: *mut ::windows_core_sys::GUID,
    pub lcid: u32,
    pub dispidInitialProperty: i32,
}
impl ::core::marker::Copy for OCPFIPARAMS {}
impl ::core::clone::Clone for OCPFIPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const OF_GET: u32 = 2u32;
pub const OF_HANDLER: u32 = 4u32;
pub const OF_SET: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct OIFI {
    pub cb: u32,
    pub fMDIApp: ::win32_foundation_sys::BOOL,
    pub hwndFrame: ::win32_foundation_sys::HWND,
    pub haccel: ::win32_ui_sys::WindowsAndMessaging::HACCEL,
    pub cAccelEntries: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for OIFI {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for OIFI {
    fn clone(&self) -> Self {
        *self
    }
}
pub type OLECLOSE = i32;
pub const OLECLOSE_SAVEIFDIRTY: OLECLOSE = 0i32;
pub const OLECLOSE_NOSAVE: OLECLOSE = 1i32;
pub const OLECLOSE_PROMPTSAVE: OLECLOSE = 2i32;
#[repr(C)]
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
pub const OLECMDARGINDEX_ACTIVEXINSTALL_CLSID: u32 = 2u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_DISPLAYNAME: u32 = 1u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_INSTALLSCOPE: u32 = 3u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_PUBLISHER: u32 = 0u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_SOURCEURL: u32 = 4u32;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_HWND: u32 = 0u32;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_X: u32 = 1u32;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_Y: u32 = 2u32;
pub const OLECMDERR_E_CANCELED: ::windows_core_sys::HRESULT = -2147221245i32;
pub const OLECMDERR_E_DISABLED: ::windows_core_sys::HRESULT = -2147221247i32;
pub const OLECMDERR_E_FIRST: ::windows_core_sys::HRESULT = -2147221248i32;
pub const OLECMDERR_E_NOHELP: ::windows_core_sys::HRESULT = -2147221246i32;
pub const OLECMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
pub const OLECMDERR_E_UNKNOWNGROUP: ::windows_core_sys::HRESULT = -2147221244i32;
pub type OLECMDEXECOPT = i32;
pub const OLECMDEXECOPT_DODEFAULT: OLECMDEXECOPT = 0i32;
pub const OLECMDEXECOPT_PROMPTUSER: OLECMDEXECOPT = 1i32;
pub const OLECMDEXECOPT_DONTPROMPTUSER: OLECMDEXECOPT = 2i32;
pub const OLECMDEXECOPT_SHOWHELP: OLECMDEXECOPT = 3i32;
pub type OLECMDF = i32;
pub const OLECMDF_SUPPORTED: OLECMDF = 1i32;
pub const OLECMDF_ENABLED: OLECMDF = 2i32;
pub const OLECMDF_LATCHED: OLECMDF = 4i32;
pub const OLECMDF_NINCHED: OLECMDF = 8i32;
pub const OLECMDF_INVISIBLE: OLECMDF = 16i32;
pub const OLECMDF_DEFHIDEONCTXTMENU: OLECMDF = 32i32;
pub type OLECMDID = i32;
pub const OLECMDID_OPEN: OLECMDID = 1i32;
pub const OLECMDID_NEW: OLECMDID = 2i32;
pub const OLECMDID_SAVE: OLECMDID = 3i32;
pub const OLECMDID_SAVEAS: OLECMDID = 4i32;
pub const OLECMDID_SAVECOPYAS: OLECMDID = 5i32;
pub const OLECMDID_PRINT: OLECMDID = 6i32;
pub const OLECMDID_PRINTPREVIEW: OLECMDID = 7i32;
pub const OLECMDID_PAGESETUP: OLECMDID = 8i32;
pub const OLECMDID_SPELL: OLECMDID = 9i32;
pub const OLECMDID_PROPERTIES: OLECMDID = 10i32;
pub const OLECMDID_CUT: OLECMDID = 11i32;
pub const OLECMDID_COPY: OLECMDID = 12i32;
pub const OLECMDID_PASTE: OLECMDID = 13i32;
pub const OLECMDID_PASTESPECIAL: OLECMDID = 14i32;
pub const OLECMDID_UNDO: OLECMDID = 15i32;
pub const OLECMDID_REDO: OLECMDID = 16i32;
pub const OLECMDID_SELECTALL: OLECMDID = 17i32;
pub const OLECMDID_CLEARSELECTION: OLECMDID = 18i32;
pub const OLECMDID_ZOOM: OLECMDID = 19i32;
pub const OLECMDID_GETZOOMRANGE: OLECMDID = 20i32;
pub const OLECMDID_UPDATECOMMANDS: OLECMDID = 21i32;
pub const OLECMDID_REFRESH: OLECMDID = 22i32;
pub const OLECMDID_STOP: OLECMDID = 23i32;
pub const OLECMDID_HIDETOOLBARS: OLECMDID = 24i32;
pub const OLECMDID_SETPROGRESSMAX: OLECMDID = 25i32;
pub const OLECMDID_SETPROGRESSPOS: OLECMDID = 26i32;
pub const OLECMDID_SETPROGRESSTEXT: OLECMDID = 27i32;
pub const OLECMDID_SETTITLE: OLECMDID = 28i32;
pub const OLECMDID_SETDOWNLOADSTATE: OLECMDID = 29i32;
pub const OLECMDID_STOPDOWNLOAD: OLECMDID = 30i32;
pub const OLECMDID_ONTOOLBARACTIVATED: OLECMDID = 31i32;
pub const OLECMDID_FIND: OLECMDID = 32i32;
pub const OLECMDID_DELETE: OLECMDID = 33i32;
pub const OLECMDID_HTTPEQUIV: OLECMDID = 34i32;
pub const OLECMDID_HTTPEQUIV_DONE: OLECMDID = 35i32;
pub const OLECMDID_ENABLE_INTERACTION: OLECMDID = 36i32;
pub const OLECMDID_ONUNLOAD: OLECMDID = 37i32;
pub const OLECMDID_PROPERTYBAG2: OLECMDID = 38i32;
pub const OLECMDID_PREREFRESH: OLECMDID = 39i32;
pub const OLECMDID_SHOWSCRIPTERROR: OLECMDID = 40i32;
pub const OLECMDID_SHOWMESSAGE: OLECMDID = 41i32;
pub const OLECMDID_SHOWFIND: OLECMDID = 42i32;
pub const OLECMDID_SHOWPAGESETUP: OLECMDID = 43i32;
pub const OLECMDID_SHOWPRINT: OLECMDID = 44i32;
pub const OLECMDID_CLOSE: OLECMDID = 45i32;
pub const OLECMDID_ALLOWUILESSSAVEAS: OLECMDID = 46i32;
pub const OLECMDID_DONTDOWNLOADCSS: OLECMDID = 47i32;
pub const OLECMDID_UPDATEPAGESTATUS: OLECMDID = 48i32;
pub const OLECMDID_PRINT2: OLECMDID = 49i32;
pub const OLECMDID_PRINTPREVIEW2: OLECMDID = 50i32;
pub const OLECMDID_SETPRINTTEMPLATE: OLECMDID = 51i32;
pub const OLECMDID_GETPRINTTEMPLATE: OLECMDID = 52i32;
pub const OLECMDID_PAGEACTIONBLOCKED: OLECMDID = 55i32;
pub const OLECMDID_PAGEACTIONUIQUERY: OLECMDID = 56i32;
pub const OLECMDID_FOCUSVIEWCONTROLS: OLECMDID = 57i32;
pub const OLECMDID_FOCUSVIEWCONTROLSQUERY: OLECMDID = 58i32;
pub const OLECMDID_SHOWPAGEACTIONMENU: OLECMDID = 59i32;
pub const OLECMDID_ADDTRAVELENTRY: OLECMDID = 60i32;
pub const OLECMDID_UPDATETRAVELENTRY: OLECMDID = 61i32;
pub const OLECMDID_UPDATEBACKFORWARDSTATE: OLECMDID = 62i32;
pub const OLECMDID_OPTICAL_ZOOM: OLECMDID = 63i32;
pub const OLECMDID_OPTICAL_GETZOOMRANGE: OLECMDID = 64i32;
pub const OLECMDID_WINDOWSTATECHANGED: OLECMDID = 65i32;
pub const OLECMDID_ACTIVEXINSTALLSCOPE: OLECMDID = 66i32;
pub const OLECMDID_UPDATETRAVELENTRY_DATARECOVERY: OLECMDID = 67i32;
pub const OLECMDID_SHOWTASKDLG: OLECMDID = 68i32;
pub const OLECMDID_POPSTATEEVENT: OLECMDID = 69i32;
pub const OLECMDID_VIEWPORT_MODE: OLECMDID = 70i32;
pub const OLECMDID_LAYOUT_VIEWPORT_WIDTH: OLECMDID = 71i32;
pub const OLECMDID_VISUAL_VIEWPORT_EXCLUDE_BOTTOM: OLECMDID = 72i32;
pub const OLECMDID_USER_OPTICAL_ZOOM: OLECMDID = 73i32;
pub const OLECMDID_PAGEAVAILABLE: OLECMDID = 74i32;
pub const OLECMDID_GETUSERSCALABLE: OLECMDID = 75i32;
pub const OLECMDID_UPDATE_CARET: OLECMDID = 76i32;
pub const OLECMDID_ENABLE_VISIBILITY: OLECMDID = 77i32;
pub const OLECMDID_MEDIA_PLAYBACK: OLECMDID = 78i32;
pub const OLECMDID_SETFAVICON: OLECMDID = 79i32;
pub const OLECMDID_SET_HOST_FULLSCREENMODE: OLECMDID = 80i32;
pub const OLECMDID_EXITFULLSCREEN: OLECMDID = 81i32;
pub const OLECMDID_SCROLLCOMPLETE: OLECMDID = 82i32;
pub const OLECMDID_ONBEFOREUNLOAD: OLECMDID = 83i32;
pub const OLECMDID_SHOWMESSAGE_BLOCKABLE: OLECMDID = 84i32;
pub const OLECMDID_SHOWTASKDLG_BLOCKABLE: OLECMDID = 85i32;
pub type OLECMDID_BROWSERSTATEFLAG = i32;
pub const OLECMDIDF_BROWSERSTATE_EXTENSIONSOFF: OLECMDID_BROWSERSTATEFLAG = 1i32;
pub const OLECMDIDF_BROWSERSTATE_IESECURITY: OLECMDID_BROWSERSTATEFLAG = 2i32;
pub const OLECMDIDF_BROWSERSTATE_PROTECTEDMODE_OFF: OLECMDID_BROWSERSTATEFLAG = 4i32;
pub const OLECMDIDF_BROWSERSTATE_RESET: OLECMDID_BROWSERSTATEFLAG = 8i32;
pub const OLECMDIDF_BROWSERSTATE_REQUIRESACTIVEX: OLECMDID_BROWSERSTATEFLAG = 16i32;
pub const OLECMDIDF_BROWSERSTATE_DESKTOPHTMLDIALOG: OLECMDID_BROWSERSTATEFLAG = 32i32;
pub const OLECMDIDF_BROWSERSTATE_BLOCKEDVERSION: OLECMDID_BROWSERSTATEFLAG = 64i32;
pub type OLECMDID_OPTICAL_ZOOMFLAG = i32;
pub const OLECMDIDF_OPTICAL_ZOOM_NOPERSIST: OLECMDID_OPTICAL_ZOOMFLAG = 1i32;
pub const OLECMDIDF_OPTICAL_ZOOM_NOLAYOUT: OLECMDID_OPTICAL_ZOOMFLAG = 16i32;
pub const OLECMDIDF_OPTICAL_ZOOM_NOTRANSIENT: OLECMDID_OPTICAL_ZOOMFLAG = 32i32;
pub const OLECMDIDF_OPTICAL_ZOOM_RELOADFORNEWTAB: OLECMDID_OPTICAL_ZOOMFLAG = 64i32;
pub type OLECMDID_PAGEACTIONFLAG = i32;
pub const OLECMDIDF_PAGEACTION_FILEDOWNLOAD: OLECMDID_PAGEACTIONFLAG = 1i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = 2i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEXTRUSTFAIL: OLECMDID_PAGEACTIONFLAG = 4i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERDISABLE: OLECMDID_PAGEACTIONFLAG = 8i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEXDISALLOW: OLECMDID_PAGEACTIONFLAG = 16i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEXUNSAFE: OLECMDID_PAGEACTIONFLAG = 32i32;
pub const OLECMDIDF_PAGEACTION_POPUPWINDOW: OLECMDID_PAGEACTIONFLAG = 64i32;
pub const OLECMDIDF_PAGEACTION_LOCALMACHINE: OLECMDID_PAGEACTIONFLAG = 128i32;
pub const OLECMDIDF_PAGEACTION_MIMETEXTPLAIN: OLECMDID_PAGEACTIONFLAG = 256i32;
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE: OLECMDID_PAGEACTIONFLAG = 512i32;
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = 512i32;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_PAGEACTIONFLAG = 1024i32;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_PAGEACTIONFLAG = 2048i32;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_PAGEACTIONFLAG = 4096i32;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_PAGEACTIONFLAG = 8192i32;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_PAGEACTIONFLAG = 16384i32;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNDENY: OLECMDID_PAGEACTIONFLAG = 32768i32;
pub const OLECMDIDF_PAGEACTION_POPUPALLOWED: OLECMDID_PAGEACTIONFLAG = 65536i32;
pub const OLECMDIDF_PAGEACTION_SCRIPTPROMPT: OLECMDID_PAGEACTIONFLAG = 131072i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = 262144i32;
pub const OLECMDIDF_PAGEACTION_MIXEDCONTENT: OLECMDID_PAGEACTIONFLAG = 524288i32;
pub const OLECMDIDF_PAGEACTION_INVALID_CERT: OLECMDID_PAGEACTIONFLAG = 1048576i32;
pub const OLECMDIDF_PAGEACTION_INTRANETZONEREQUEST: OLECMDID_PAGEACTIONFLAG = 2097152i32;
pub const OLECMDIDF_PAGEACTION_XSSFILTERED: OLECMDID_PAGEACTIONFLAG = 4194304i32;
pub const OLECMDIDF_PAGEACTION_SPOOFABLEIDNHOST: OLECMDID_PAGEACTIONFLAG = 8388608i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEX_EPM_INCOMPATIBLE: OLECMDID_PAGEACTIONFLAG = 16777216i32;
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = 33554432i32;
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED: OLECMDID_PAGEACTIONFLAG = 67108864i32;
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED_ACTIVEX: OLECMDID_PAGEACTIONFLAG = 134217728i32;
pub const OLECMDIDF_PAGEACTION_EXTENSION_COMPAT_BLOCKED: OLECMDID_PAGEACTIONFLAG = 268435456i32;
pub const OLECMDIDF_PAGEACTION_NORESETACTIVEX: OLECMDID_PAGEACTIONFLAG = 536870912i32;
pub const OLECMDIDF_PAGEACTION_GENERIC_STATE: OLECMDID_PAGEACTIONFLAG = 1073741824i32;
pub const OLECMDIDF_PAGEACTION_RESET: OLECMDID_PAGEACTIONFLAG = -2147483648i32;
pub type OLECMDID_REFRESHFLAG = i32;
pub const OLECMDIDF_REFRESH_NORMAL: OLECMDID_REFRESHFLAG = 0i32;
pub const OLECMDIDF_REFRESH_IFEXPIRED: OLECMDID_REFRESHFLAG = 1i32;
pub const OLECMDIDF_REFRESH_CONTINUE: OLECMDID_REFRESHFLAG = 2i32;
pub const OLECMDIDF_REFRESH_COMPLETELY: OLECMDID_REFRESHFLAG = 3i32;
pub const OLECMDIDF_REFRESH_NO_CACHE: OLECMDID_REFRESHFLAG = 4i32;
pub const OLECMDIDF_REFRESH_RELOAD: OLECMDID_REFRESHFLAG = 5i32;
pub const OLECMDIDF_REFRESH_LEVELMASK: OLECMDID_REFRESHFLAG = 255i32;
pub const OLECMDIDF_REFRESH_CLEARUSERINPUT: OLECMDID_REFRESHFLAG = 4096i32;
pub const OLECMDIDF_REFRESH_PROMPTIFOFFLINE: OLECMDID_REFRESHFLAG = 8192i32;
pub const OLECMDIDF_REFRESH_THROUGHSCRIPT: OLECMDID_REFRESHFLAG = 16384i32;
pub const OLECMDIDF_REFRESH_SKIPBEFOREUNLOADEVENT: OLECMDID_REFRESHFLAG = 32768i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_ACTIVEXINSTALL: OLECMDID_REFRESHFLAG = 65536i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_FILEDOWNLOAD: OLECMDID_REFRESHFLAG = 131072i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_LOCALMACHINE: OLECMDID_REFRESHFLAG = 262144i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_POPUPWINDOW: OLECMDID_REFRESHFLAG = 524288i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_REFRESHFLAG = 1048576i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_REFRESHFLAG = 2097152i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_REFRESHFLAG = 4194304i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_REFRESHFLAG = 8388608i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_REFRESHFLAG = 16777216i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_MIXEDCONTENT: OLECMDID_REFRESHFLAG = 33554432i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_INVALID_CERT: OLECMDID_REFRESHFLAG = 67108864i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_ALLOW_VERSION: OLECMDID_REFRESHFLAG = 134217728i32;
pub type OLECMDID_VIEWPORT_MODE_FLAG = i32;
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH: OLECMDID_VIEWPORT_MODE_FLAG = 1i32;
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM: OLECMDID_VIEWPORT_MODE_FLAG = 2i32;
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH_VALID: OLECMDID_VIEWPORT_MODE_FLAG = 65536i32;
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM_VALID: OLECMDID_VIEWPORT_MODE_FLAG = 131072i32;
pub type OLECMDID_WINDOWSTATE_FLAG = i32;
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE: OLECMDID_WINDOWSTATE_FLAG = 1i32;
pub const OLECMDIDF_WINDOWSTATE_ENABLED: OLECMDID_WINDOWSTATE_FLAG = 2i32;
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE_VALID: OLECMDID_WINDOWSTATE_FLAG = 65536i32;
pub const OLECMDIDF_WINDOWSTATE_ENABLED_VALID: OLECMDID_WINDOWSTATE_FLAG = 131072i32;
#[repr(C)]
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
pub type OLECMDTEXTF = i32;
pub const OLECMDTEXTF_NONE: OLECMDTEXTF = 0i32;
pub const OLECMDTEXTF_NAME: OLECMDTEXTF = 1i32;
pub const OLECMDTEXTF_STATUS: OLECMDTEXTF = 2i32;
pub const OLECMD_TASKDLGID_ONBEFOREUNLOAD: u32 = 1u32;
pub type OLECONTF = i32;
pub const OLECONTF_EMBEDDINGS: OLECONTF = 1i32;
pub const OLECONTF_LINKS: OLECONTF = 2i32;
pub const OLECONTF_OTHERS: OLECONTF = 4i32;
pub const OLECONTF_ONLYUSER: OLECONTF = 8i32;
pub const OLECONTF_ONLYIFRUNNING: OLECONTF = 16i32;
pub const OLECREATE_LEAVERUNNING: u32 = 1u32;
pub type OLEDCFLAGS = i32;
pub const OLEDC_NODRAW: OLEDCFLAGS = 1i32;
pub const OLEDC_PAINTBKGND: OLEDCFLAGS = 2i32;
pub const OLEDC_OFFSCREEN: OLEDCFLAGS = 4i32;
pub type OLEGETMONIKER = i32;
pub const OLEGETMONIKER_ONLYIFTHERE: OLEGETMONIKER = 1i32;
pub const OLEGETMONIKER_FORCEASSIGN: OLEGETMONIKER = 2i32;
pub const OLEGETMONIKER_UNASSIGN: OLEGETMONIKER = 3i32;
pub const OLEGETMONIKER_TEMPFORUSER: OLEGETMONIKER = 4i32;
pub const OLEIVERB_DISCARDUNDOSTATE: i32 = -6i32;
pub const OLEIVERB_HIDE: i32 = -3i32;
pub const OLEIVERB_INPLACEACTIVATE: i32 = -5i32;
pub const OLEIVERB_OPEN: i32 = -2i32;
pub const OLEIVERB_PRIMARY: i32 = 0i32;
pub const OLEIVERB_PROPERTIES: i32 = -7i32;
pub const OLEIVERB_SHOW: i32 = -1i32;
pub const OLEIVERB_UIACTIVATE: i32 = -4i32;
pub type OLELINKBIND = i32;
pub const OLELINKBIND_EVENIFCLASSDIFF: OLELINKBIND = 1i32;
pub type OLEMISC = i32;
pub const OLEMISC_RECOMPOSEONRESIZE: OLEMISC = 1i32;
pub const OLEMISC_ONLYICONIC: OLEMISC = 2i32;
pub const OLEMISC_INSERTNOTREPLACE: OLEMISC = 4i32;
pub const OLEMISC_STATIC: OLEMISC = 8i32;
pub const OLEMISC_CANTLINKINSIDE: OLEMISC = 16i32;
pub const OLEMISC_CANLINKBYOLE1: OLEMISC = 32i32;
pub const OLEMISC_ISLINKOBJECT: OLEMISC = 64i32;
pub const OLEMISC_INSIDEOUT: OLEMISC = 128i32;
pub const OLEMISC_ACTIVATEWHENVISIBLE: OLEMISC = 256i32;
pub const OLEMISC_RENDERINGISDEVICEINDEPENDENT: OLEMISC = 512i32;
pub const OLEMISC_INVISIBLEATRUNTIME: OLEMISC = 1024i32;
pub const OLEMISC_ALWAYSRUN: OLEMISC = 2048i32;
pub const OLEMISC_ACTSLIKEBUTTON: OLEMISC = 4096i32;
pub const OLEMISC_ACTSLIKELABEL: OLEMISC = 8192i32;
pub const OLEMISC_NOUIACTIVATE: OLEMISC = 16384i32;
pub const OLEMISC_ALIGNABLE: OLEMISC = 32768i32;
pub const OLEMISC_SIMPLEFRAME: OLEMISC = 65536i32;
pub const OLEMISC_SETCLIENTSITEFIRST: OLEMISC = 131072i32;
pub const OLEMISC_IMEMODE: OLEMISC = 262144i32;
pub const OLEMISC_IGNOREACTIVATEWHENVISIBLE: OLEMISC = 524288i32;
pub const OLEMISC_WANTSTOMENUMERGE: OLEMISC = 1048576i32;
pub const OLEMISC_SUPPORTSMULTILEVELUNDO: OLEMISC = 2097152i32;
pub type OLERENDER = i32;
pub const OLERENDER_NONE: OLERENDER = 0i32;
pub const OLERENDER_DRAW: OLERENDER = 1i32;
pub const OLERENDER_FORMAT: OLERENDER = 2i32;
pub const OLERENDER_ASIS: OLERENDER = 3i32;
pub const OLESTDDELIM: &str = "\\";
#[repr(C)]
#[cfg(feature = "Win32_Media")]
pub struct OLEUIBUSYA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub hTask: ::win32_media_sys::HTASK,
    pub lphWndDialog: *mut ::win32_foundation_sys::HWND,
}
#[cfg(feature = "Win32_Media")]
impl ::core::marker::Copy for OLEUIBUSYA {}
#[cfg(feature = "Win32_Media")]
impl ::core::clone::Clone for OLEUIBUSYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Media")]
pub struct OLEUIBUSYW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCWSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub hTask: ::win32_media_sys::HTASK,
    pub lphWndDialog: *mut ::win32_foundation_sys::HWND,
}
#[cfg(feature = "Win32_Media")]
impl ::core::marker::Copy for OLEUIBUSYW {}
#[cfg(feature = "Win32_Media")]
impl ::core::clone::Clone for OLEUIBUSYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OLEUICHANGEICONA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub hMetaPict: isize,
    pub clsid: ::windows_core_sys::GUID,
    pub szIconExe: [::win32_foundation_sys::CHAR; 260],
    pub cchIconExe: i32,
}
impl ::core::marker::Copy for OLEUICHANGEICONA {}
impl ::core::clone::Clone for OLEUICHANGEICONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OLEUICHANGEICONW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCWSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub hMetaPict: isize,
    pub clsid: ::windows_core_sys::GUID,
    pub szIconExe: [u16; 260],
    pub cchIconExe: i32,
}
impl ::core::marker::Copy for OLEUICHANGEICONW {}
impl ::core::clone::Clone for OLEUICHANGEICONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
pub struct OLEUICHANGESOURCEA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub lpOFN: *mut ::win32_ui_sys::Controls::Dialogs::OPENFILENAMEA,
    pub dwReserved1: [u32; 4],
    pub lpOleUILinkContainer: IOleUILinkContainerA,
    pub dwLink: u32,
    pub lpszDisplayName: ::windows_core_sys::PSTR,
    pub nFileLength: u32,
    pub lpszFrom: ::windows_core_sys::PSTR,
    pub lpszTo: ::windows_core_sys::PSTR,
}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
impl ::core::marker::Copy for OLEUICHANGESOURCEA {}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
impl ::core::clone::Clone for OLEUICHANGESOURCEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
pub struct OLEUICHANGESOURCEW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCWSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub lpOFN: *mut ::win32_ui_sys::Controls::Dialogs::OPENFILENAMEW,
    pub dwReserved1: [u32; 4],
    pub lpOleUILinkContainer: IOleUILinkContainerW,
    pub dwLink: u32,
    pub lpszDisplayName: ::windows_core_sys::PWSTR,
    pub nFileLength: u32,
    pub lpszFrom: ::windows_core_sys::PWSTR,
    pub lpszTo: ::windows_core_sys::PWSTR,
}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
impl ::core::marker::Copy for OLEUICHANGESOURCEW {}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
impl ::core::clone::Clone for OLEUICHANGESOURCEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OLEUICONVERTA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub clsid: ::windows_core_sys::GUID,
    pub clsidConvertDefault: ::windows_core_sys::GUID,
    pub clsidActivateDefault: ::windows_core_sys::GUID,
    pub clsidNew: ::windows_core_sys::GUID,
    pub dvAspect: u32,
    pub wFormat: u16,
    pub fIsLinkedObject: ::win32_foundation_sys::BOOL,
    pub hMetaPict: isize,
    pub lpszUserType: ::windows_core_sys::PSTR,
    pub fObjectsIconChanged: ::win32_foundation_sys::BOOL,
    pub lpszDefLabel: ::windows_core_sys::PSTR,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for OLEUICONVERTA {}
impl ::core::clone::Clone for OLEUICONVERTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OLEUICONVERTW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCWSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub clsid: ::windows_core_sys::GUID,
    pub clsidConvertDefault: ::windows_core_sys::GUID,
    pub clsidActivateDefault: ::windows_core_sys::GUID,
    pub clsidNew: ::windows_core_sys::GUID,
    pub dvAspect: u32,
    pub wFormat: u16,
    pub fIsLinkedObject: ::win32_foundation_sys::BOOL,
    pub hMetaPict: isize,
    pub lpszUserType: ::windows_core_sys::PWSTR,
    pub fObjectsIconChanged: ::win32_foundation_sys::BOOL,
    pub lpszDefLabel: ::windows_core_sys::PWSTR,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for OLEUICONVERTW {}
impl ::core::clone::Clone for OLEUICONVERTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OLEUIEDITLINKSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub lpOleUILinkContainer: IOleUILinkContainerA,
}
impl ::core::marker::Copy for OLEUIEDITLINKSA {}
impl ::core::clone::Clone for OLEUIEDITLINKSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OLEUIEDITLINKSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCWSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub lpOleUILinkContainer: IOleUILinkContainerW,
}
impl ::core::marker::Copy for OLEUIEDITLINKSW {}
impl ::core::clone::Clone for OLEUIEDITLINKSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIGNRLPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIGNRLPROPSA {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIGNRLPROPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIGNRLPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIGNRLPROPSW {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIGNRLPROPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub struct OLEUIINSERTOBJECTA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub clsid: ::windows_core_sys::GUID,
    pub lpszFile: ::windows_core_sys::PSTR,
    pub cchFile: u32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows_core_sys::GUID,
    pub iid: ::windows_core_sys::GUID,
    pub oleRender: u32,
    pub lpFormatEtc: *mut super::Com::FORMATETC,
    pub lpIOleClientSite: IOleClientSite,
    pub lpIStorage: super::Com::StructuredStorage::IStorage,
    pub ppvObj: *mut *mut ::core::ffi::c_void,
    pub sc: i32,
    pub hMetaPict: isize,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::marker::Copy for OLEUIINSERTOBJECTA {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::clone::Clone for OLEUIINSERTOBJECTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub struct OLEUIINSERTOBJECTW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCWSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub clsid: ::windows_core_sys::GUID,
    pub lpszFile: ::windows_core_sys::PWSTR,
    pub cchFile: u32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows_core_sys::GUID,
    pub iid: ::windows_core_sys::GUID,
    pub oleRender: u32,
    pub lpFormatEtc: *mut super::Com::FORMATETC,
    pub lpIOleClientSite: IOleClientSite,
    pub lpIStorage: super::Com::StructuredStorage::IStorage,
    pub ppvObj: *mut *mut ::core::ffi::c_void,
    pub sc: i32,
    pub hMetaPict: isize,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::marker::Copy for OLEUIINSERTOBJECTW {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::clone::Clone for OLEUIINSERTOBJECTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUILINKPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUILINKPROPSA {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUILINKPROPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUILINKPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUILINKPROPSW {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUILINKPROPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIOBJECTPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub lpPS: *mut ::win32_ui_sys::Controls::PROPSHEETHEADERA_V2,
    pub dwObject: u32,
    pub lpObjInfo: IOleUIObjInfoA,
    pub dwLink: u32,
    pub lpLinkInfo: IOleUILinkInfoA,
    pub lpGP: *mut OLEUIGNRLPROPSA,
    pub lpVP: *mut OLEUIVIEWPROPSA,
    pub lpLP: *mut OLEUILINKPROPSA,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIOBJECTPROPSA {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIOBJECTPROPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIOBJECTPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub lpPS: *mut ::win32_ui_sys::Controls::PROPSHEETHEADERW_V2,
    pub dwObject: u32,
    pub lpObjInfo: IOleUIObjInfoW,
    pub dwLink: u32,
    pub lpLinkInfo: IOleUILinkInfoW,
    pub lpGP: *mut OLEUIGNRLPROPSW,
    pub lpVP: *mut OLEUIVIEWPROPSW,
    pub lpLP: *mut OLEUILINKPROPSW,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIOBJECTPROPSW {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIOBJECTPROPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct OLEUIPASTEENTRYA {
    pub fmtetc: super::Com::FORMATETC,
    pub lpstrFormatName: ::windows_core_sys::PCSTR,
    pub lpstrResultText: ::windows_core_sys::PCSTR,
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
#[cfg(feature = "Win32_System_Com")]
pub struct OLEUIPASTEENTRYW {
    pub fmtetc: super::Com::FORMATETC,
    pub lpstrFormatName: ::windows_core_sys::PCWSTR,
    pub lpstrResultText: ::windows_core_sys::PCWSTR,
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
pub type OLEUIPASTEFLAG = i32;
pub const OLEUIPASTE_ENABLEICON: OLEUIPASTEFLAG = 2048i32;
pub const OLEUIPASTE_PASTEONLY: OLEUIPASTEFLAG = 0i32;
pub const OLEUIPASTE_PASTE: OLEUIPASTEFLAG = 512i32;
pub const OLEUIPASTE_LINKANYTYPE: OLEUIPASTEFLAG = 1024i32;
pub const OLEUIPASTE_LINKTYPE1: OLEUIPASTEFLAG = 1i32;
pub const OLEUIPASTE_LINKTYPE2: OLEUIPASTEFLAG = 2i32;
pub const OLEUIPASTE_LINKTYPE3: OLEUIPASTEFLAG = 4i32;
pub const OLEUIPASTE_LINKTYPE4: OLEUIPASTEFLAG = 8i32;
pub const OLEUIPASTE_LINKTYPE5: OLEUIPASTEFLAG = 16i32;
pub const OLEUIPASTE_LINKTYPE6: OLEUIPASTEFLAG = 32i32;
pub const OLEUIPASTE_LINKTYPE7: OLEUIPASTEFLAG = 64i32;
pub const OLEUIPASTE_LINKTYPE8: OLEUIPASTEFLAG = 128i32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct OLEUIPASTESPECIALA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub lpSrcDataObj: super::Com::IDataObject,
    pub arrPasteEntries: *mut OLEUIPASTEENTRYA,
    pub cPasteEntries: i32,
    pub arrLinkTypes: *mut u32,
    pub cLinkTypes: i32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows_core_sys::GUID,
    pub nSelectedIndex: i32,
    pub fLink: ::win32_foundation_sys::BOOL,
    pub hMetaPict: isize,
    pub sizel: ::win32_foundation_sys::SIZE,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for OLEUIPASTESPECIALA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for OLEUIPASTESPECIALA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct OLEUIPASTESPECIALW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub lpszCaption: ::windows_core_sys::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub hInstance: ::win32_foundation_sys::HINSTANCE,
    pub lpszTemplate: ::windows_core_sys::PCWSTR,
    pub hResource: ::win32_foundation_sys::HRSRC,
    pub lpSrcDataObj: super::Com::IDataObject,
    pub arrPasteEntries: *mut OLEUIPASTEENTRYW,
    pub cPasteEntries: i32,
    pub arrLinkTypes: *mut u32,
    pub cLinkTypes: i32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows_core_sys::GUID,
    pub nSelectedIndex: i32,
    pub fLink: ::win32_foundation_sys::BOOL,
    pub hMetaPict: isize,
    pub sizel: ::win32_foundation_sys::SIZE,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for OLEUIPASTESPECIALW {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for OLEUIPASTESPECIALW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIVIEWPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
    pub nScaleMin: i32,
    pub nScaleMax: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIVIEWPROPSA {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIVIEWPROPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIVIEWPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: ::win32_foundation_sys::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
    pub nScaleMin: i32,
    pub nScaleMax: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIVIEWPROPSW {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIVIEWPROPSW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const OLEUI_BZERR_HTASKINVALID: u32 = 116u32;
pub const OLEUI_BZ_CALLUNBLOCKED: u32 = 119u32;
pub const OLEUI_BZ_RETRYSELECTED: u32 = 118u32;
pub const OLEUI_BZ_SWITCHTOSELECTED: u32 = 117u32;
pub const OLEUI_CANCEL: u32 = 2u32;
pub const OLEUI_CIERR_MUSTHAVECLSID: u32 = 116u32;
pub const OLEUI_CIERR_MUSTHAVECURRENTMETAFILE: u32 = 117u32;
pub const OLEUI_CIERR_SZICONEXEINVALID: u32 = 118u32;
pub const OLEUI_CSERR_FROMNOTNULL: u32 = 118u32;
pub const OLEUI_CSERR_LINKCNTRINVALID: u32 = 117u32;
pub const OLEUI_CSERR_LINKCNTRNULL: u32 = 116u32;
pub const OLEUI_CSERR_SOURCEINVALID: u32 = 121u32;
pub const OLEUI_CSERR_SOURCENULL: u32 = 120u32;
pub const OLEUI_CSERR_SOURCEPARSEERROR: u32 = 122u32;
pub const OLEUI_CSERR_SOURCEPARSERROR: u32 = 122u32;
pub const OLEUI_CSERR_TONOTNULL: u32 = 119u32;
pub const OLEUI_CTERR_CBFORMATINVALID: u32 = 119u32;
pub const OLEUI_CTERR_CLASSIDINVALID: u32 = 117u32;
pub const OLEUI_CTERR_DVASPECTINVALID: u32 = 118u32;
pub const OLEUI_CTERR_HMETAPICTINVALID: u32 = 120u32;
pub const OLEUI_CTERR_STRINGINVALID: u32 = 121u32;
pub const OLEUI_ELERR_LINKCNTRINVALID: u32 = 117u32;
pub const OLEUI_ELERR_LINKCNTRNULL: u32 = 116u32;
pub const OLEUI_ERR_CBSTRUCTINCORRECT: u32 = 103u32;
pub const OLEUI_ERR_DIALOGFAILURE: u32 = 112u32;
pub const OLEUI_ERR_FINDTEMPLATEFAILURE: u32 = 110u32;
pub const OLEUI_ERR_GLOBALMEMALLOC: u32 = 114u32;
pub const OLEUI_ERR_HINSTANCEINVALID: u32 = 107u32;
pub const OLEUI_ERR_HRESOURCEINVALID: u32 = 109u32;
pub const OLEUI_ERR_HWNDOWNERINVALID: u32 = 104u32;
pub const OLEUI_ERR_LOADSTRING: u32 = 115u32;
pub const OLEUI_ERR_LOADTEMPLATEFAILURE: u32 = 111u32;
pub const OLEUI_ERR_LOCALMEMALLOC: u32 = 113u32;
pub const OLEUI_ERR_LPFNHOOKINVALID: u32 = 106u32;
pub const OLEUI_ERR_LPSZCAPTIONINVALID: u32 = 105u32;
pub const OLEUI_ERR_LPSZTEMPLATEINVALID: u32 = 108u32;
pub const OLEUI_ERR_OLEMEMALLOC: u32 = 100u32;
pub const OLEUI_ERR_STANDARDMAX: u32 = 116u32;
pub const OLEUI_ERR_STANDARDMIN: u32 = 100u32;
pub const OLEUI_ERR_STRUCTUREINVALID: u32 = 102u32;
pub const OLEUI_ERR_STRUCTURENULL: u32 = 101u32;
pub const OLEUI_FALSE: u32 = 0u32;
pub const OLEUI_GPERR_CBFORMATINVALID: u32 = 130u32;
pub const OLEUI_GPERR_CLASSIDINVALID: u32 = 128u32;
pub const OLEUI_GPERR_LPCLSIDEXCLUDEINVALID: u32 = 129u32;
pub const OLEUI_GPERR_STRINGINVALID: u32 = 127u32;
pub const OLEUI_IOERR_ARRLINKTYPESINVALID: u32 = 118u32;
pub const OLEUI_IOERR_ARRPASTEENTRIESINVALID: u32 = 117u32;
pub const OLEUI_IOERR_CCHFILEINVALID: u32 = 125u32;
pub const OLEUI_IOERR_HICONINVALID: u32 = 118u32;
pub const OLEUI_IOERR_LPCLSIDEXCLUDEINVALID: u32 = 124u32;
pub const OLEUI_IOERR_LPFORMATETCINVALID: u32 = 119u32;
pub const OLEUI_IOERR_LPIOLECLIENTSITEINVALID: u32 = 121u32;
pub const OLEUI_IOERR_LPISTORAGEINVALID: u32 = 122u32;
pub const OLEUI_IOERR_LPSZFILEINVALID: u32 = 116u32;
pub const OLEUI_IOERR_LPSZLABELINVALID: u32 = 117u32;
pub const OLEUI_IOERR_PPVOBJINVALID: u32 = 120u32;
pub const OLEUI_IOERR_SCODEHASERROR: u32 = 123u32;
pub const OLEUI_IOERR_SRCDATAOBJECTINVALID: u32 = 116u32;
pub const OLEUI_LPERR_LINKCNTRINVALID: u32 = 134u32;
pub const OLEUI_LPERR_LINKCNTRNULL: u32 = 133u32;
pub const OLEUI_OK: u32 = 1u32;
pub const OLEUI_OPERR_DLGPROCNOTNULL: u32 = 125u32;
pub const OLEUI_OPERR_INVALIDPAGES: u32 = 123u32;
pub const OLEUI_OPERR_LINKINFOINVALID: u32 = 137u32;
pub const OLEUI_OPERR_LPARAMNOTZERO: u32 = 126u32;
pub const OLEUI_OPERR_NOTSUPPORTED: u32 = 124u32;
pub const OLEUI_OPERR_OBJINFOINVALID: u32 = 136u32;
pub const OLEUI_OPERR_PAGESINCORRECT: u32 = 122u32;
pub const OLEUI_OPERR_PROPERTYSHEET: u32 = 135u32;
pub const OLEUI_OPERR_PROPSHEETINVALID: u32 = 119u32;
pub const OLEUI_OPERR_PROPSHEETNULL: u32 = 118u32;
pub const OLEUI_OPERR_PROPSINVALID: u32 = 121u32;
pub const OLEUI_OPERR_SUBPROPINVALID: u32 = 117u32;
pub const OLEUI_OPERR_SUBPROPNULL: u32 = 116u32;
pub const OLEUI_OPERR_SUPPROP: u32 = 120u32;
pub const OLEUI_PSERR_CLIPBOARDCHANGED: u32 = 119u32;
pub const OLEUI_PSERR_GETCLIPBOARDFAILED: u32 = 120u32;
pub const OLEUI_QUERY_GETCLASSID: u32 = 65280u32;
pub const OLEUI_QUERY_LINKBROKEN: u32 = 65281u32;
pub const OLEUI_SUCCESS: u32 = 1u32;
pub const OLEUI_VPERR_DVASPECTINVALID: u32 = 132u32;
pub const OLEUI_VPERR_METAPICTINVALID: u32 = 131u32;
pub type OLEUPDATE = i32;
pub const OLEUPDATE_ALWAYS: OLEUPDATE = 1i32;
pub const OLEUPDATE_ONCALL: OLEUPDATE = 3i32;
#[repr(C)]
pub struct OLEVERB {
    pub lVerb: i32,
    pub lpszVerbName: ::windows_core_sys::PWSTR,
    pub fuFlags: u32,
    pub grfAttribs: u32,
}
impl ::core::marker::Copy for OLEVERB {}
impl ::core::clone::Clone for OLEVERB {
    fn clone(&self) -> Self {
        *self
    }
}
pub type OLEVERBATTRIB = i32;
pub const OLEVERBATTRIB_NEVERDIRTIES: OLEVERBATTRIB = 1i32;
pub const OLEVERBATTRIB_ONCONTAINERMENU: OLEVERBATTRIB = 2i32;
pub const OLEVERB_PRIMARY: u32 = 0u32;
pub type OLEWHICHMK = i32;
pub const OLEWHICHMK_CONTAINER: OLEWHICHMK = 1i32;
pub const OLEWHICHMK_OBJREL: OLEWHICHMK = 2i32;
pub const OLEWHICHMK_OBJFULL: OLEWHICHMK = 3i32;
pub type OLE_TRISTATE = i32;
pub const triUnchecked: OLE_TRISTATE = 0i32;
pub const triChecked: OLE_TRISTATE = 1i32;
pub const triGray: OLE_TRISTATE = 2i32;
pub const OPF_DISABLECONVERT: i32 = 8i32;
pub const OPF_NOFILLDEFAULT: i32 = 2i32;
pub const OPF_OBJECTISLINK: i32 = 1i32;
pub const OPF_SHOWHELP: i32 = 4i32;
pub const OT_EMBEDDED: i32 = 2i32;
pub const OT_LINK: i32 = 1i32;
pub const OT_STATIC: i32 = 3i32;
#[repr(C)]
pub struct OleMenuGroupWidths {
    pub width: [i32; 6],
}
impl ::core::marker::Copy for OleMenuGroupWidths {}
impl ::core::clone::Clone for OleMenuGroupWidths {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PAGEACTION_UI = i32;
pub const PAGEACTION_UI_DEFAULT: PAGEACTION_UI = 0i32;
pub const PAGEACTION_UI_MODAL: PAGEACTION_UI = 1i32;
pub const PAGEACTION_UI_MODELESS: PAGEACTION_UI = 2i32;
pub const PAGEACTION_UI_SILENT: PAGEACTION_UI = 3i32;
#[repr(C)]
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
pub struct PAGESET {
    pub cbStruct: u32,
    pub fOddPages: ::win32_foundation_sys::BOOL,
    pub fEvenPages: ::win32_foundation_sys::BOOL,
    pub cPageRange: u32,
    pub rgPages: [PAGERANGE; 1],
}
impl ::core::marker::Copy for PAGESET {}
impl ::core::clone::Clone for PAGESET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PARAMDATA {
    pub szName: ::windows_core_sys::PWSTR,
    pub vt: u16,
}
impl ::core::marker::Copy for PARAMDATA {}
impl ::core::clone::Clone for PARAMDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct PARAMDESC {
    pub pparamdescex: *mut PARAMDESCEX,
    pub wParamFlags: u16,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for PARAMDESC {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for PARAMDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct PARAMDESCEX {
    pub cBytes: u32,
    pub varDefaultValue: super::Com::VARIANT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for PARAMDESCEX {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for PARAMDESCEX {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PARAMFLAG_FHASCUSTDATA: u32 = 64u32;
pub const PARAMFLAG_FHASDEFAULT: u32 = 32u32;
pub const PARAMFLAG_FIN: u32 = 1u32;
pub const PARAMFLAG_FLCID: u32 = 4u32;
pub const PARAMFLAG_FOPT: u32 = 16u32;
pub const PARAMFLAG_FOUT: u32 = 2u32;
pub const PARAMFLAG_FRETVAL: u32 = 8u32;
pub const PARAMFLAG_NONE: u32 = 0u32;
pub const PERPROP_E_FIRST: i32 = -2147220992i32;
pub const PERPROP_E_LAST: ::windows_core_sys::HRESULT = -2147220977i32;
pub const PERPROP_E_NOPAGEAVAILABLE: ::windows_core_sys::HRESULT = -2147220992i32;
pub const PERPROP_S_FIRST: ::windows_core_sys::HRESULT = 262656i32;
pub const PERPROP_S_LAST: ::windows_core_sys::HRESULT = 262671i32;
#[repr(C)]
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_0 {
    pub hbitmap: ::win32_graphics_sys::Gdi::HBITMAP,
    pub hpal: ::win32_graphics_sys::Gdi::HPALETTE,
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_1 {
    pub hemf: ::win32_graphics_sys::Gdi::HENHMETAFILE,
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_2 {
    pub hicon: ::win32_ui_sys::WindowsAndMessaging::HICON,
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_3 {
    pub hmeta: ::win32_graphics_sys::Gdi::HMETAFILE,
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
pub const PICTYPE_BITMAP: u32 = 1u32;
pub const PICTYPE_ENHMETAFILE: u32 = 4u32;
pub const PICTYPE_ICON: u32 = 3u32;
pub const PICTYPE_METAFILE: u32 = 2u32;
pub const PICTYPE_NONE: u32 = 0u32;
pub const PICTYPE_UNINITIALIZED: i32 = -1i32;
pub type POINTERINACTIVE = i32;
pub const POINTERINACTIVE_ACTIVATEONENTRY: POINTERINACTIVE = 1i32;
pub const POINTERINACTIVE_DEACTIVATEONLEAVE: POINTERINACTIVE = 2i32;
pub const POINTERINACTIVE_ACTIVATEONDRAG: POINTERINACTIVE = 4i32;
#[repr(C)]
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
pub type PRINTFLAG = u32;
pub const PRINTFLAG_MAYBOTHERUSER: PRINTFLAG = 1u32;
pub const PRINTFLAG_PROMPTUSER: PRINTFLAG = 2u32;
pub const PRINTFLAG_USERMAYCHANGEPRINTER: PRINTFLAG = 4u32;
pub const PRINTFLAG_RECOMPOSETODEVICE: PRINTFLAG = 8u32;
pub const PRINTFLAG_DONTACTUALLYPRINT: PRINTFLAG = 16u32;
pub const PRINTFLAG_FORCEPROPERTIES: PRINTFLAG = 32u32;
pub const PRINTFLAG_PRINTTOFILE: PRINTFLAG = 64u32;
pub type PROPBAG2_TYPE = i32;
pub const PROPBAG2_TYPE_UNDEFINED: PROPBAG2_TYPE = 0i32;
pub const PROPBAG2_TYPE_DATA: PROPBAG2_TYPE = 1i32;
pub const PROPBAG2_TYPE_URL: PROPBAG2_TYPE = 2i32;
pub const PROPBAG2_TYPE_OBJECT: PROPBAG2_TYPE = 3i32;
pub const PROPBAG2_TYPE_STREAM: PROPBAG2_TYPE = 4i32;
pub const PROPBAG2_TYPE_STORAGE: PROPBAG2_TYPE = 5i32;
pub const PROPBAG2_TYPE_MONIKER: PROPBAG2_TYPE = 6i32;
#[repr(C)]
pub struct PROPPAGEINFO {
    pub cb: u32,
    pub pszTitle: ::windows_core_sys::PWSTR,
    pub size: ::win32_foundation_sys::SIZE,
    pub pszDocString: ::windows_core_sys::PWSTR,
    pub pszHelpFile: ::windows_core_sys::PWSTR,
    pub dwHelpContext: u32,
}
impl ::core::marker::Copy for PROPPAGEINFO {}
impl ::core::clone::Clone for PROPPAGEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROPPAGESTATUS = i32;
pub const PROPPAGESTATUS_DIRTY: PROPPAGESTATUS = 1i32;
pub const PROPPAGESTATUS_VALIDATE: PROPPAGESTATUS = 2i32;
pub const PROPPAGESTATUS_CLEAN: PROPPAGESTATUS = 4i32;
pub const PROP_HWND_CHGICONDLG: &str = "HWND_CIDLG";
pub const PSF_CHECKDISPLAYASICON: i32 = 8i32;
pub const PSF_DISABLEDISPLAYASICON: i32 = 16i32;
pub const PSF_HIDECHANGEICON: i32 = 32i32;
pub const PSF_NOREFRESHDATAOBJECT: i32 = 128i32;
pub const PSF_SELECTPASTE: i32 = 2i32;
pub const PSF_SELECTPASTELINK: i32 = 4i32;
pub const PSF_SHOWHELP: i32 = 1i32;
pub const PSF_STAYONCLIPBOARDCHANGE: i32 = 64i32;
pub const PS_MAXLINKTYPES: u32 = 8u32;
pub type PictureAttributes = i32;
pub const PICTURE_SCALABLE: PictureAttributes = 1i32;
pub const PICTURE_TRANSPARENT: PictureAttributes = 2i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub struct QACONTAINER {
    pub cbSize: u32,
    pub pClientSite: IOleClientSite,
    pub pAdviseSink: IAdviseSinkEx,
    pub pPropertyNotifySink: IPropertyNotifySink,
    pub pUnkEventSink: ::windows_core_sys::IUnknown,
    pub dwAmbientFlags: u32,
    pub colorFore: u32,
    pub colorBack: u32,
    pub pFont: IFont,
    pub pUndoMgr: IOleUndoManager,
    pub dwAppearance: u32,
    pub lcid: i32,
    pub hpal: ::win32_graphics_sys::Gdi::HPALETTE,
    pub pBindHost: super::Com::IBindHost,
    pub pOleControlSite: IOleControlSite,
    pub pServiceProvider: super::Com::IServiceProvider,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for QACONTAINER {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for QACONTAINER {
    fn clone(&self) -> Self {
        *self
    }
}
pub type QACONTAINERFLAGS = i32;
pub const QACONTAINER_SHOWHATCHING: QACONTAINERFLAGS = 1i32;
pub const QACONTAINER_SHOWGRABHANDLES: QACONTAINERFLAGS = 2i32;
pub const QACONTAINER_USERMODE: QACONTAINERFLAGS = 4i32;
pub const QACONTAINER_DISPLAYASDEFAULT: QACONTAINERFLAGS = 8i32;
pub const QACONTAINER_UIDEAD: QACONTAINERFLAGS = 16i32;
pub const QACONTAINER_AUTOCLIP: QACONTAINERFLAGS = 32i32;
pub const QACONTAINER_MESSAGEREFLECT: QACONTAINERFLAGS = 64i32;
pub const QACONTAINER_SUPPORTSMNEMONICS: QACONTAINERFLAGS = 128i32;
#[repr(C)]
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
pub type READYSTATE = i32;
pub const READYSTATE_UNINITIALIZED: READYSTATE = 0i32;
pub const READYSTATE_LOADING: READYSTATE = 1i32;
pub const READYSTATE_LOADED: READYSTATE = 2i32;
pub const READYSTATE_INTERACTIVE: READYSTATE = 3i32;
pub const READYSTATE_COMPLETE: READYSTATE = 4i32;
pub type REGKIND = i32;
pub const REGKIND_DEFAULT: REGKIND = 0i32;
pub const REGKIND_REGISTER: REGKIND = 1i32;
pub const REGKIND_NONE: REGKIND = 2i32;
pub const SELFREG_E_CLASS: ::windows_core_sys::HRESULT = -2147220991i32;
pub const SELFREG_E_FIRST: i32 = -2147220992i32;
pub const SELFREG_E_LAST: ::windows_core_sys::HRESULT = -2147220977i32;
pub const SELFREG_E_TYPELIB: ::windows_core_sys::HRESULT = -2147220992i32;
pub const SELFREG_S_FIRST: ::windows_core_sys::HRESULT = 262656i32;
pub const SELFREG_S_LAST: ::windows_core_sys::HRESULT = 262671i32;
pub type SF_TYPE = i32;
pub const SF_ERROR: SF_TYPE = 10i32;
pub const SF_I1: SF_TYPE = 16i32;
pub const SF_I2: SF_TYPE = 2i32;
pub const SF_I4: SF_TYPE = 3i32;
pub const SF_I8: SF_TYPE = 20i32;
pub const SF_BSTR: SF_TYPE = 8i32;
pub const SF_UNKNOWN: SF_TYPE = 13i32;
pub const SF_DISPATCH: SF_TYPE = 9i32;
pub const SF_VARIANT: SF_TYPE = 12i32;
pub const SF_RECORD: SF_TYPE = 36i32;
pub const SF_HAVEIID: SF_TYPE = 32781i32;
pub const SID_GetCaller: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1192741952, data2: 48313, data3: 4560, data4: [147, 54, 0, 160, 201, 13, 202, 169] };
pub const SID_ProvideRuntimeContext: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1956971532, data2: 56588, data3: 18672, data4: [172, 133, 25, 76, 50, 89, 24, 10] };
pub const SID_VariantConversion: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 521147521, data2: 48333, data3: 4560, data4: [147, 54, 0, 160, 201, 13, 202, 169] };
pub const STDOLE2_LCID: u32 = 0u32;
pub const STDOLE2_MAJORVERNUM: u32 = 2u32;
pub const STDOLE2_MINORVERNUM: u32 = 0u32;
pub const STDOLE_LCID: u32 = 0u32;
pub const STDOLE_MAJORVERNUM: u32 = 1u32;
pub const STDOLE_MINORVERNUM: u32 = 0u32;
pub const STDOLE_TLB: &str = "stdole2.tlb";
pub const STDTYPE_TLB: &str = "stdole2.tlb";
pub const SZOLEUI_MSG_ADDCONTROL: &str = "OLEUI_MSG_ADDCONTROL";
pub const SZOLEUI_MSG_BROWSE: &str = "OLEUI_MSG_BROWSE";
pub const SZOLEUI_MSG_BROWSE_OFN: &str = "OLEUI_MSG_BROWSE_OFN";
pub const SZOLEUI_MSG_CHANGEICON: &str = "OLEUI_MSG_CHANGEICON";
pub const SZOLEUI_MSG_CHANGESOURCE: &str = "OLEUI_MSG_CHANGESOURCE";
pub const SZOLEUI_MSG_CLOSEBUSYDIALOG: &str = "OLEUI_MSG_CLOSEBUSYDIALOG";
pub const SZOLEUI_MSG_CONVERT: &str = "OLEUI_MSG_CONVERT";
pub const SZOLEUI_MSG_ENDDIALOG: &str = "OLEUI_MSG_ENDDIALOG";
pub const SZOLEUI_MSG_HELP: &str = "OLEUI_MSG_HELP";
pub const TIFLAGS_EXTENDDISPATCHONLY: u32 = 1u32;
pub type TYPEFLAGS = i32;
pub const TYPEFLAG_FAPPOBJECT: TYPEFLAGS = 1i32;
pub const TYPEFLAG_FCANCREATE: TYPEFLAGS = 2i32;
pub const TYPEFLAG_FLICENSED: TYPEFLAGS = 4i32;
pub const TYPEFLAG_FPREDECLID: TYPEFLAGS = 8i32;
pub const TYPEFLAG_FHIDDEN: TYPEFLAGS = 16i32;
pub const TYPEFLAG_FCONTROL: TYPEFLAGS = 32i32;
pub const TYPEFLAG_FDUAL: TYPEFLAGS = 64i32;
pub const TYPEFLAG_FNONEXTENSIBLE: TYPEFLAGS = 128i32;
pub const TYPEFLAG_FOLEAUTOMATION: TYPEFLAGS = 256i32;
pub const TYPEFLAG_FRESTRICTED: TYPEFLAGS = 512i32;
pub const TYPEFLAG_FAGGREGATABLE: TYPEFLAGS = 1024i32;
pub const TYPEFLAG_FREPLACEABLE: TYPEFLAGS = 2048i32;
pub const TYPEFLAG_FDISPATCHABLE: TYPEFLAGS = 4096i32;
pub const TYPEFLAG_FREVERSEBIND: TYPEFLAGS = 8192i32;
pub const TYPEFLAG_FPROXY: TYPEFLAGS = 16384i32;
pub type UASFLAGS = i32;
pub const UAS_NORMAL: UASFLAGS = 0i32;
pub const UAS_BLOCKED: UASFLAGS = 1i32;
pub const UAS_NOPARENTENABLE: UASFLAGS = 2i32;
pub const UAS_MASK: UASFLAGS = 3i32;
#[repr(C)]
pub struct UDATE {
    pub st: ::win32_foundation_sys::SYSTEMTIME,
    pub wDayOfYear: u16,
}
impl ::core::marker::Copy for UDATE {}
impl ::core::clone::Clone for UDATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UPDFCACHE_FLAGS = u32;
pub const UPDFCACHE_ALL: UPDFCACHE_FLAGS = 2147483647u32;
pub const UPDFCACHE_ALLBUTNODATACACHE: UPDFCACHE_FLAGS = 2147483646u32;
pub const UPDFCACHE_NORMALCACHE: UPDFCACHE_FLAGS = 8u32;
pub const UPDFCACHE_IFBLANK: UPDFCACHE_FLAGS = 16u32;
pub const UPDFCACHE_ONLYIFBLANK: UPDFCACHE_FLAGS = 2147483648u32;
pub const UPDFCACHE_NODATACACHE: UPDFCACHE_FLAGS = 1u32;
pub const UPDFCACHE_ONSAVECACHE: UPDFCACHE_FLAGS = 2u32;
pub const UPDFCACHE_ONSTOPCACHE: UPDFCACHE_FLAGS = 4u32;
pub const UPDFCACHE_IFBLANKORONSAVECACHE: UPDFCACHE_FLAGS = 18u32;
pub type USERCLASSTYPE = i32;
pub const USERCLASSTYPE_FULL: USERCLASSTYPE = 1i32;
pub const USERCLASSTYPE_SHORT: USERCLASSTYPE = 2i32;
pub const USERCLASSTYPE_APPNAME: USERCLASSTYPE = 3i32;
pub const VARCMP_EQ: u32 = 1u32;
pub const VARCMP_GT: u32 = 2u32;
pub const VARCMP_LT: u32 = 0u32;
pub const VARCMP_NULL: u32 = 3u32;
pub type VARENUM = i32;
pub const VT_EMPTY: VARENUM = 0i32;
pub const VT_NULL: VARENUM = 1i32;
pub const VT_I2: VARENUM = 2i32;
pub const VT_I4: VARENUM = 3i32;
pub const VT_R4: VARENUM = 4i32;
pub const VT_R8: VARENUM = 5i32;
pub const VT_CY: VARENUM = 6i32;
pub const VT_DATE: VARENUM = 7i32;
pub const VT_BSTR: VARENUM = 8i32;
pub const VT_DISPATCH: VARENUM = 9i32;
pub const VT_ERROR: VARENUM = 10i32;
pub const VT_BOOL: VARENUM = 11i32;
pub const VT_VARIANT: VARENUM = 12i32;
pub const VT_UNKNOWN: VARENUM = 13i32;
pub const VT_DECIMAL: VARENUM = 14i32;
pub const VT_I1: VARENUM = 16i32;
pub const VT_UI1: VARENUM = 17i32;
pub const VT_UI2: VARENUM = 18i32;
pub const VT_UI4: VARENUM = 19i32;
pub const VT_I8: VARENUM = 20i32;
pub const VT_UI8: VARENUM = 21i32;
pub const VT_INT: VARENUM = 22i32;
pub const VT_UINT: VARENUM = 23i32;
pub const VT_VOID: VARENUM = 24i32;
pub const VT_HRESULT: VARENUM = 25i32;
pub const VT_PTR: VARENUM = 26i32;
pub const VT_SAFEARRAY: VARENUM = 27i32;
pub const VT_CARRAY: VARENUM = 28i32;
pub const VT_USERDEFINED: VARENUM = 29i32;
pub const VT_LPSTR: VARENUM = 30i32;
pub const VT_LPWSTR: VARENUM = 31i32;
pub const VT_RECORD: VARENUM = 36i32;
pub const VT_INT_PTR: VARENUM = 37i32;
pub const VT_UINT_PTR: VARENUM = 38i32;
pub const VT_FILETIME: VARENUM = 64i32;
pub const VT_BLOB: VARENUM = 65i32;
pub const VT_STREAM: VARENUM = 66i32;
pub const VT_STORAGE: VARENUM = 67i32;
pub const VT_STREAMED_OBJECT: VARENUM = 68i32;
pub const VT_STORED_OBJECT: VARENUM = 69i32;
pub const VT_BLOB_OBJECT: VARENUM = 70i32;
pub const VT_CF: VARENUM = 71i32;
pub const VT_CLSID: VARENUM = 72i32;
pub const VT_VERSIONED_STREAM: VARENUM = 73i32;
pub const VT_BSTR_BLOB: VARENUM = 4095i32;
pub const VT_VECTOR: VARENUM = 4096i32;
pub const VT_ARRAY: VARENUM = 8192i32;
pub const VT_BYREF: VARENUM = 16384i32;
pub const VT_RESERVED: VARENUM = 32768i32;
pub const VT_ILLEGAL: VARENUM = 65535i32;
pub const VT_ILLEGALMASKED: VARENUM = 4095i32;
pub const VT_TYPEMASK: VARENUM = 4095i32;
pub type VARFLAGS = i32;
pub const VARFLAG_FREADONLY: VARFLAGS = 1i32;
pub const VARFLAG_FSOURCE: VARFLAGS = 2i32;
pub const VARFLAG_FBINDABLE: VARFLAGS = 4i32;
pub const VARFLAG_FREQUESTEDIT: VARFLAGS = 8i32;
pub const VARFLAG_FDISPLAYBIND: VARFLAGS = 16i32;
pub const VARFLAG_FDEFAULTBIND: VARFLAGS = 32i32;
pub const VARFLAG_FHIDDEN: VARFLAGS = 64i32;
pub const VARFLAG_FRESTRICTED: VARFLAGS = 128i32;
pub const VARFLAG_FDEFAULTCOLLELEM: VARFLAGS = 256i32;
pub const VARFLAG_FUIDEFAULT: VARFLAGS = 512i32;
pub const VARFLAG_FNONBROWSABLE: VARFLAGS = 1024i32;
pub const VARFLAG_FREPLACEABLE: VARFLAGS = 2048i32;
pub const VARFLAG_FIMMEDIATEBIND: VARFLAGS = 4096i32;
pub const VARIANT_ALPHABOOL: u32 = 2u32;
pub const VARIANT_CALENDAR_GREGORIAN: u32 = 64u32;
pub const VARIANT_CALENDAR_HIJRI: u32 = 8u32;
pub const VARIANT_CALENDAR_THAI: u32 = 32u32;
pub const VARIANT_LOCALBOOL: u32 = 16u32;
pub const VARIANT_NOUSEROVERRIDE: u32 = 4u32;
pub const VARIANT_NOVALUEPROP: u32 = 1u32;
pub const VARIANT_USE_NLS: u32 = 128u32;
pub type VIEWSTATUS = i32;
pub const VIEWSTATUS_OPAQUE: VIEWSTATUS = 1i32;
pub const VIEWSTATUS_SOLIDBKGND: VIEWSTATUS = 2i32;
pub const VIEWSTATUS_DVASPECTOPAQUE: VIEWSTATUS = 4i32;
pub const VIEWSTATUS_DVASPECTTRANSPARENT: VIEWSTATUS = 8i32;
pub const VIEWSTATUS_SURFACE: VIEWSTATUS = 16i32;
pub const VIEWSTATUS_3DSURFACE: VIEWSTATUS = 32i32;
pub const VPF_DISABLERELATIVE: i32 = 2i32;
pub const VPF_DISABLESCALE: i32 = 4i32;
pub const VPF_SELECTRELATIVE: i32 = 1i32;
pub const VTDATEGRE_MAX: u32 = 2958465u32;
pub const VTDATEGRE_MIN: i32 = -657434i32;
pub const VT_BLOB_PROPSET: u32 = 75u32;
pub const VT_STORED_PROPSET: u32 = 74u32;
pub const VT_STREAMED_PROPSET: u32 = 73u32;
pub const VT_VERBOSE_ENUM: u32 = 76u32;
pub const WIN32: u32 = 100u32;
pub type WPCSETTING = i32;
pub const WPCSETTING_LOGGING_ENABLED: WPCSETTING = 1i32;
pub const WPCSETTING_FILEDOWNLOAD_BLOCKED: WPCSETTING = 2i32;
pub type XFORMCOORDS = i32;
pub const XFORMCOORDS_POSITION: XFORMCOORDS = 1i32;
pub const XFORMCOORDS_SIZE: XFORMCOORDS = 2i32;
pub const XFORMCOORDS_HIMETRICTOCONTAINER: XFORMCOORDS = 4i32;
pub const XFORMCOORDS_CONTAINERTOHIMETRIC: XFORMCOORDS = 8i32;
pub const XFORMCOORDS_EVENTCOMPAT: XFORMCOORDS = 16i32;
#[repr(C)]
pub struct _wireBRECORD {
    pub fFlags: u32,
    pub clSize: u32,
    pub pRecInfo: IRecordInfo,
    pub pRecord: *mut u8,
}
impl ::core::marker::Copy for _wireBRECORD {}
impl ::core::clone::Clone for _wireBRECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct _wireSAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub uArrayStructs: _wireSAFEARRAY_UNION,
    pub rgsabound: [super::Com::SAFEARRAYBOUND; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for _wireSAFEARRAY {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _wireSAFEARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct _wireSAFEARRAY_UNION {
    pub sfType: u32,
    pub u: _wireSAFEARRAY_UNION_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for _wireSAFEARRAY_UNION {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _wireSAFEARRAY_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for _wireSAFEARRAY_UNION_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _wireSAFEARRAY_UNION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[cfg(feature = "Win32_System_Com")]
pub struct _wireSAFEARR_DISPATCH {
    pub Size: u32,
    pub apDispatch: *mut super::Com::IDispatch,
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
pub struct _wireSAFEARR_HAVEIID {
    pub Size: u32,
    pub apUnknown: *mut ::windows_core_sys::IUnknown,
    pub iid: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for _wireSAFEARR_HAVEIID {}
impl ::core::clone::Clone for _wireSAFEARR_HAVEIID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct _wireSAFEARR_UNKNOWN {
    pub Size: u32,
    pub apUnknown: *mut ::windows_core_sys::IUnknown,
}
impl ::core::marker::Copy for _wireSAFEARR_UNKNOWN {}
impl ::core::clone::Clone for _wireSAFEARR_UNKNOWN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct _wireSAFEARR_VARIANT {
    pub Size: u32,
    pub aVariant: *mut *mut _wireVARIANT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for _wireSAFEARR_VARIANT {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _wireSAFEARR_VARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct _wireVARIANT {
    pub clSize: u32,
    pub rpcReserved: u32,
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: _wireVARIANT_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for _wireVARIANT {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _wireVARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
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
    pub punkVal: ::windows_core_sys::IUnknown,
    pub pdispVal: super::Com::IDispatch,
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
    pub ppunkVal: *mut ::windows_core_sys::IUnknown,
    pub ppdispVal: *mut super::Com::IDispatch,
    pub pparray: *mut *mut *mut _wireSAFEARRAY,
    pub pvarVal: *mut *mut _wireVARIANT,
    pub cVal: ::win32_foundation_sys::CHAR,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub decVal: ::win32_foundation_sys::DECIMAL,
    pub pdecVal: *mut ::win32_foundation_sys::DECIMAL,
    pub pcVal: ::windows_core_sys::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for _wireVARIANT_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _wireVARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const fdexEnumAll: i32 = 2i32;
pub const fdexEnumDefault: i32 = 1i32;
pub const fdexNameCaseInsensitive: i32 = 8i32;
pub const fdexNameCaseSensitive: i32 = 1i32;
pub const fdexNameEnsure: i32 = 2i32;
pub const fdexNameImplicit: i32 = 4i32;
pub const fdexNameInternal: i32 = 16i32;
pub const fdexNameNoDynamicProperties: i32 = 32i32;
pub const fdexPropCanCall: i32 = 256i32;
pub const fdexPropCanConstruct: i32 = 1024i32;
pub const fdexPropCanGet: i32 = 1i32;
pub const fdexPropCanPut: i32 = 4i32;
pub const fdexPropCanPutRef: i32 = 16i32;
pub const fdexPropCanSourceEvents: i32 = 4096i32;
pub const fdexPropCannotCall: i32 = 512i32;
pub const fdexPropCannotConstruct: i32 = 2048i32;
pub const fdexPropCannotGet: i32 = 2i32;
pub const fdexPropCannotPut: i32 = 8i32;
pub const fdexPropCannotPutRef: i32 = 32i32;
pub const fdexPropCannotSourceEvents: i32 = 8192i32;
pub const fdexPropDynamicType: i32 = 128i32;
pub const fdexPropNoSideEffects: i32 = 64i32;
