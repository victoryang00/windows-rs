#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn BuildDisplayTable(lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpmalloc: *mut *mut super::Com::IMalloc, hinstance: super::super::Foundation::HINSTANCE, cpages: u32, lppage: *mut DTPAGE, ulflags: u32, lpptable: *mut *mut *mut IMAPITable, lpptbldata: *mut *mut *mut ITableData) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeIdleRoutine(ftg: *mut ::core::ffi::c_void, lpfnidle: PFNIDLE, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16, ircidle: u16);
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn CreateIProp(lpinterface: *mut ::windows_sys::core::GUID, lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpvreserved: *mut ::core::ffi::c_void, lpppropdata: *mut *mut *mut IPropData) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn CreateTable(lpinterface: *mut ::windows_sys::core::GUID, lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpvreserved: *mut ::core::ffi::c_void, ultabletype: u32, ulproptagindexcolumn: u32, lpsproptagarraycolumns: *mut SPropTagArray, lpptabledata: *mut *mut *mut ITableData) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn DeinitMapiUtil();
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn DeregisterIdleRoutine(ftg: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableIdleRoutine(ftg: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL);
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FEqualNames(lpname1: *mut MAPINAMEID, lpname2: *mut MAPINAMEID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FPropCompareProp(lpspropvalue1: *mut SPropValue, ulrelop: u32, lpspropvalue2: *mut SPropValue) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FPropContainsProp(lpspropvaluedst: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, ulfuzzylevel: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FPropExists(lpmapiprop: *mut *mut IMAPIProp, ulproptag: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FreePadrlist(lpadrlist: *mut ADRLIST);
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FreeProws(lprows: *mut SRowSet);
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtAddFt(ftaddend1: super::super::Foundation::FILETIME, ftaddend2: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtMulDw(ftmultiplier: u32, ftmultiplicand: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtMulDwDw(ftmultiplicand: u32, ftmultiplier: u32) -> super::super::Foundation::FILETIME;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtNegFt(ft: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtSubFt(ftminuend: super::super::Foundation::FILETIME, ftsubtrahend: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtgRegisterIdleRoutine(lpfnidle: PFNIDLE, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn HrAddColumns(lptbl: *mut *mut IMAPITable, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn HrAddColumnsEx(lptbl: *mut *mut IMAPITable, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, lpfnfiltercolumns: isize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrAllocAdviseSink(lpfncallback: LPNOTIFCALLBACK, lpvcontext: *mut ::core::ffi::c_void, lppadvisesink: *mut *mut *mut IMAPIAdviseSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn HrDispatchNotifications(ulflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrGetOneProp(lpmapiprop: *mut *mut IMAPIProp, ulproptag: u32, lppprop: *mut *mut SPropValue) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn HrIStorageFromStream(lpunkin: *mut *mut ::windows_sys::core::IUnknown, lpinterface: *mut ::windows_sys::core::GUID, ulflags: u32, lppstorageout: *mut *mut *mut super::Com::StructuredStorage::IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrQueryAllRows(lptable: *mut *mut IMAPITable, lpproptags: *mut SPropTagArray, lprestriction: *mut SRestriction, lpsortorderset: *mut SSortOrderSet, crowsmax: i32, lpprows: *mut *mut SRowSet) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrSetOneProp(lpmapiprop: *mut *mut IMAPIProp, lpprop: *mut SPropValue) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn HrThisThreadAdviseSink(lpadvisesink: *mut *mut IMAPIAdviseSink, lppadvisesink: *mut *mut *mut IMAPIAdviseSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LPropCompareProp(lpspropvaluea: *mut SPropValue, lpspropvalueb: *mut SPropValue) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LpValFindProp(ulproptag: u32, cvalues: u32, lpproparray: *mut SPropValue) -> *mut SPropValue;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn MAPIDeinitIdle();
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MAPIGetDefaultMalloc() -> *mut *mut super::Com::IMalloc;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn MAPIInitIdle(lpvreserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OpenStreamOnFile(lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8, lppstream: *mut *mut *mut super::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn PpropFindProp(lpproparray: *mut SPropValue, cvalues: u32, ulproptag: u32) -> *mut SPropValue;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn PropCopyMore(lpspropvaluedest: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, lpfallocmore: LPALLOCATEMORE, lpvobject: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RTFSync(lpmessage: *mut *mut IMessage, ulflags: u32, lpfmessageupdated: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCopyNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCopyProps(cvalues: i32, lpproparray: *mut SPropValue, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCountNotifications(cnotifications: i32, lpnotifications: *mut NOTIFICATION, lpcb: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCountProps(cvalues: i32, lpproparray: *mut SPropValue, lpcb: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn ScCreateConversationIndex(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScDupPropset(cvalues: i32, lpproparray: *mut SPropValue, lpallocatebuffer: LPALLOCATEBUFFER, lppproparray: *mut *mut SPropValue) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn ScInitMapiUtil(ulflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn ScLocalPathFromUNC(lpszunc: ::windows_sys::core::PCSTR, lpszlocal: ::windows_sys::core::PCSTR, cchlocal: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScRelocNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScRelocProps(cvalues: i32, lpproparray: *mut SPropValue, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn ScUNCFromLocalPath(lpszlocal: ::windows_sys::core::PCSTR, lpszunc: ::windows_sys::core::PCSTR, cchunc: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn SzFindCh(lpsz: *mut i8, ch: u16) -> *mut i8;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn SzFindLastCh(lpsz: *mut i8, ch: u16) -> *mut i8;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn SzFindSz(lpsz: *mut i8, lpszkey: *mut i8) -> *mut i8;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn UFromSz(lpsz: *mut i8) -> u32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn UlAddRef(lpunk: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UlPropSize(lpspropvalue: *mut SPropValue) -> u32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn UlRelease(lpunk: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn WrapCompressedRTFStream(lpcompressedrtfstream: *mut *mut super::Com::IStream, ulflags: u32, lpuncompressedrtfstream: *mut *mut *mut super::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    pub fn WrapStoreEntryID(ulflags: u32, lpszdllname: *const i8, cborigentry: u32, lporigentry: *const ENTRYID, lpcbwrappedentry: *mut u32, lppwrappedentry: *mut *mut ENTRYID) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct ADRENTRY {
    pub ulReserved1: u32,
    pub cValues: u32,
    pub rgPropVals: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for ADRENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for ADRENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct ADRLIST {
    pub cEntries: u32,
    pub aEntries: [ADRENTRY; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for ADRLIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for ADRLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct ADRPARM {
    pub cbABContEntryID: u32,
    pub lpABContEntryID: *mut ENTRYID,
    pub ulFlags: u32,
    pub lpReserved: *mut ::core::ffi::c_void,
    pub ulHelpContext: u32,
    pub lpszHelpFileName: *mut i8,
    pub lpfnABSDI: LPFNABSDI,
    pub lpfnDismiss: LPFNDISMISS,
    pub lpvDismissContext: *mut ::core::ffi::c_void,
    pub lpszCaption: *mut i8,
    pub lpszNewEntryTitle: *mut i8,
    pub lpszDestWellsTitle: *mut i8,
    pub cDestFields: u32,
    pub nDestFieldFocus: u32,
    pub lppszDestTitles: *mut *mut i8,
    pub lpulDestComps: *mut u32,
    pub lpContRestriction: *mut SRestriction,
    pub lpHierRestriction: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for ADRPARM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for ADRPARM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type CALLERRELEASE = ::core::option::Option<unsafe extern "system" fn(ulcallerdata: u32, lptbldata: *mut *mut ITableData, lpvue: *mut *mut IMAPITable)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLBUTTON {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulPRControl: u32,
}
impl ::core::marker::Copy for DTBLBUTTON {}
impl ::core::clone::Clone for DTBLBUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLCHECKBOX {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulPRPropertyName: u32,
}
impl ::core::marker::Copy for DTBLCHECKBOX {}
impl ::core::clone::Clone for DTBLCHECKBOX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLCOMBOBOX {
    pub ulbLpszCharsAllowed: u32,
    pub ulFlags: u32,
    pub ulNumCharsAllowed: u32,
    pub ulPRPropertyName: u32,
    pub ulPRTableName: u32,
}
impl ::core::marker::Copy for DTBLCOMBOBOX {}
impl ::core::clone::Clone for DTBLCOMBOBOX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLDDLBX {
    pub ulFlags: u32,
    pub ulPRDisplayProperty: u32,
    pub ulPRSetProperty: u32,
    pub ulPRTableName: u32,
}
impl ::core::marker::Copy for DTBLDDLBX {}
impl ::core::clone::Clone for DTBLDDLBX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLEDIT {
    pub ulbLpszCharsAllowed: u32,
    pub ulFlags: u32,
    pub ulNumCharsAllowed: u32,
    pub ulPropTag: u32,
}
impl ::core::marker::Copy for DTBLEDIT {}
impl ::core::clone::Clone for DTBLEDIT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLGROUPBOX {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
}
impl ::core::marker::Copy for DTBLGROUPBOX {}
impl ::core::clone::Clone for DTBLGROUPBOX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLLABEL {
    pub ulbLpszLabelName: u32,
    pub ulFlags: u32,
}
impl ::core::marker::Copy for DTBLLABEL {}
impl ::core::clone::Clone for DTBLLABEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLLBX {
    pub ulFlags: u32,
    pub ulPRSetProperty: u32,
    pub ulPRTableName: u32,
}
impl ::core::marker::Copy for DTBLLBX {}
impl ::core::clone::Clone for DTBLLBX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLMVDDLBX {
    pub ulFlags: u32,
    pub ulMVPropTag: u32,
}
impl ::core::marker::Copy for DTBLMVDDLBX {}
impl ::core::clone::Clone for DTBLMVDDLBX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLMVLISTBOX {
    pub ulFlags: u32,
    pub ulMVPropTag: u32,
}
impl ::core::marker::Copy for DTBLMVLISTBOX {}
impl ::core::clone::Clone for DTBLMVLISTBOX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLPAGE {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulbLpszComponent: u32,
    pub ulContext: u32,
}
impl ::core::marker::Copy for DTBLPAGE {}
impl ::core::clone::Clone for DTBLPAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLRADIOBUTTON {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulcButtons: u32,
    pub ulPropTag: u32,
    pub lReturnValue: i32,
}
impl ::core::marker::Copy for DTBLRADIOBUTTON {}
impl ::core::clone::Clone for DTBLRADIOBUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTCTL {
    pub ulCtlType: u32,
    pub ulCtlFlags: u32,
    pub lpbNotif: *mut u8,
    pub cbNotif: u32,
    pub lpszFilter: *mut i8,
    pub ulItemID: u32,
    pub ctl: DTCTL_0,
}
impl ::core::marker::Copy for DTCTL {}
impl ::core::clone::Clone for DTCTL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub union DTCTL_0 {
    pub lpv: *mut ::core::ffi::c_void,
    pub lplabel: *mut DTBLLABEL,
    pub lpedit: *mut DTBLEDIT,
    pub lplbx: *mut DTBLLBX,
    pub lpcombobox: *mut DTBLCOMBOBOX,
    pub lpddlbx: *mut DTBLDDLBX,
    pub lpcheckbox: *mut DTBLCHECKBOX,
    pub lpgroupbox: *mut DTBLGROUPBOX,
    pub lpbutton: *mut DTBLBUTTON,
    pub lpradiobutton: *mut DTBLRADIOBUTTON,
    pub lpmvlbx: *mut DTBLMVLISTBOX,
    pub lpmvddlbx: *mut DTBLMVDDLBX,
    pub lppage: *mut DTBLPAGE,
}
impl ::core::marker::Copy for DTCTL_0 {}
impl ::core::clone::Clone for DTCTL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTPAGE {
    pub cctl: u32,
    pub lpszResourceName: *mut i8,
    pub Anonymous: DTPAGE_0,
    pub lpctl: *mut DTCTL,
}
impl ::core::marker::Copy for DTPAGE {}
impl ::core::clone::Clone for DTPAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub union DTPAGE_0 {
    pub lpszComponent: *mut i8,
    pub ulItemID: u32,
}
impl ::core::marker::Copy for DTPAGE_0 {}
impl ::core::clone::Clone for DTPAGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct ENTRYID {
    pub abFlags: [u8; 4],
    pub ab: [u8; 1],
}
impl ::core::marker::Copy for ENTRYID {}
impl ::core::clone::Clone for ENTRYID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct ERROR_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub scode: i32,
    pub ulFlags: u32,
    pub lpMAPIError: *mut MAPIERROR,
}
impl ::core::marker::Copy for ERROR_NOTIFICATION {}
impl ::core::clone::Clone for ERROR_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct EXTENDED_NOTIFICATION {
    pub ulEvent: u32,
    pub cb: u32,
    pub pbEventParameters: *mut u8,
}
impl ::core::marker::Copy for EXTENDED_NOTIFICATION {}
impl ::core::clone::Clone for EXTENDED_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_BURN_VERIFICATION_FAILED: ::windows_sys::core::HRESULT = -1062600697i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = -1062599672i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_INVALID_MEDIA_STATE: ::windows_sys::core::HRESULT = -1062599678i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_MEDIA_IS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062599674i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_MEDIA_NOT_BLANK: ::windows_sys::core::HRESULT = -1062599675i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_RECORDER_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062599673i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_STREAM_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062599677i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_STREAM_TOO_LARGE_FOR_CURRENT_MEDIA: ::windows_sys::core::HRESULT = -1062599676i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_WRITE_IN_PROGRESS: ::windows_sys::core::HRESULT = -1062599680i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_WRITE_NOT_IN_PROGRESS: ::windows_sys::core::HRESULT = -1062599679i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = -1062599164i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_DATA_BLOCK_TYPE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062599154i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_BLANK: ::windows_sys::core::HRESULT = -1062599162i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_PREPARED: ::windows_sys::core::HRESULT = -1062599166i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062599161i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_MEDIA_IS_PREPARED: ::windows_sys::core::HRESULT = -1062599165i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_NOT_ENOUGH_SPACE: ::windows_sys::core::HRESULT = -1062599159i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_NO_RECORDER_SPECIFIED: ::windows_sys::core::HRESULT = -1062599158i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_RECORDER_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062599152i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_STREAM_LEADIN_TOO_SHORT: ::windows_sys::core::HRESULT = -1062599153i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_STREAM_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062599155i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_WRITE_IN_PROGRESS: ::windows_sys::core::HRESULT = -1062599168i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_WRITE_NOT_IN_PROGRESS: ::windows_sys::core::HRESULT = -1062599167i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = -1062599409i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_INVALID_ISRC: ::windows_sys::core::HRESULT = -1062599413i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_INVALID_MCN: ::windows_sys::core::HRESULT = -1062599412i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_BLANK: ::windows_sys::core::HRESULT = -1062599418i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_PREPARED: ::windows_sys::core::HRESULT = -1062599422i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062599417i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_MEDIA_IS_PREPARED: ::windows_sys::core::HRESULT = -1062599421i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_NOT_ENOUGH_SPACE: ::windows_sys::core::HRESULT = -1062599415i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_NO_RECORDER_SPECIFIED: ::windows_sys::core::HRESULT = -1062599414i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_PROPERTY_FOR_BLANK_MEDIA_ONLY: ::windows_sys::core::HRESULT = -1062599420i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_RECORDER_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062599410i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_STREAM_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062599411i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_TABLE_OF_CONTENTS_EMPTY_DISC: ::windows_sys::core::HRESULT = -1062599419i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_TRACK_LIMIT_REACHED: ::windows_sys::core::HRESULT = -1062599416i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_WRITE_IN_PROGRESS: ::windows_sys::core::HRESULT = -1062599424i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_WRITE_NOT_IN_PROGRESS: ::windows_sys::core::HRESULT = -1062599423i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = -1062598389i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_DISC_INFORMATION_TOO_SMALL: ::windows_sys::core::HRESULT = -2136340222i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_DRIVE_FAILED_ERASE_COMMAND: ::windows_sys::core::HRESULT = -2136340219i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_DRIVE_FAILED_SPINUP_COMMAND: ::windows_sys::core::HRESULT = -2136340216i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_ERASABLE: ::windows_sys::core::HRESULT = -2136340220i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062598391i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_MODE_PAGE_2A_TOO_SMALL: ::windows_sys::core::HRESULT = -2136340221i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_ONLY_ONE_RECORDER_SUPPORTED: ::windows_sys::core::HRESULT = -2136340223i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_RECORDER_IN_USE: ::windows_sys::core::HRESULT = -2136340224i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_RECORDER_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062598390i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_TOOK_LONGER_THAN_ONE_HOUR: ::windows_sys::core::HRESULT = -2136340218i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_UNEXPECTED_DRIVE_RESPONSE_DURING_ERASE: ::windows_sys::core::HRESULT = -2136340217i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_LOSS_OF_STREAMING: ::windows_sys::core::HRESULT = -1062599936i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_INSUFFICIENT_SPACE: ::windows_sys::core::HRESULT = -2136339963i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_IS_READ_ONLY: ::windows_sys::core::HRESULT = -2136339968i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_NO_TRACKS: ::windows_sys::core::HRESULT = -2136339965i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_SECTOR_TYPE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2136339966i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACKS: ::windows_sys::core::HRESULT = -2136339967i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACK_INDEXES: ::windows_sys::core::HRESULT = -2136339962i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_TRACKS_ALREADY_ADDED: ::windows_sys::core::HRESULT = -2136339964i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_NOT_FOUND: ::windows_sys::core::HRESULT = -2136339961i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_OFFSET_ZERO_CANNOT_BE_CLEARED: ::windows_sys::core::HRESULT = -2136339959i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_TOO_CLOSE_TO_OTHER_INDEX: ::windows_sys::core::HRESULT = -2136339958i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = -1062600175i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_COMMAND_TIMEOUT: ::windows_sys::core::HRESULT = -1062600179i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_DVD_STRUCTURE_NOT_PRESENT: ::windows_sys::core::HRESULT = -1062600178i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_FEATURE_IS_NOT_CURRENT: ::windows_sys::core::HRESULT = -1062600181i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_GET_CONFIGURATION_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062600180i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_INVALID_MODE_PARAMETERS: ::windows_sys::core::HRESULT = -1062600184i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_INVALID_RESPONSE_FROM_DEVICE: ::windows_sys::core::HRESULT = -1062599937i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_LOCKED: ::windows_sys::core::HRESULT = -1062600176i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_BECOMING_READY: ::windows_sys::core::HRESULT = -1062600187i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_BUSY: ::windows_sys::core::HRESULT = -1062600185i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_FORMAT_IN_PROGRESS: ::windows_sys::core::HRESULT = -1062600186i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_INCOMPATIBLE: ::windows_sys::core::HRESULT = -1062600189i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_NOT_FORMATTED: ::windows_sys::core::HRESULT = -1062600174i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_NO_MEDIA: ::windows_sys::core::HRESULT = -1062600190i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_SPEED_MISMATCH: ::windows_sys::core::HRESULT = -1062600177i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_UPSIDE_DOWN: ::windows_sys::core::HRESULT = -1062600188i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_WRITE_PROTECTED: ::windows_sys::core::HRESULT = -1062600183i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_NO_SUCH_FEATURE: ::windows_sys::core::HRESULT = -1062600182i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_NO_SUCH_MODE_PAGE: ::windows_sys::core::HRESULT = -1062600191i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_REQUIRED: ::windows_sys::core::HRESULT = -1062600701i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_REQUEST_CANCELLED: ::windows_sys::core::HRESULT = -1062600702i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_UNEXPECTED_RESPONSE_FROM_DEVICE: ::windows_sys::core::HRESULT = -1062599935i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const FACILITY_IMAPI2: u32 = 170u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct FLATENTRY {
    pub cb: u32,
    pub abEntry: [u8; 1],
}
impl ::core::marker::Copy for FLATENTRY {}
impl ::core::clone::Clone for FLATENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct FLATENTRYLIST {
    pub cEntries: u32,
    pub cbEntries: u32,
    pub abEntries: [u8; 1],
}
impl ::core::marker::Copy for FLATENTRYLIST {}
impl ::core::clone::Clone for FLATENTRYLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct FLATMTSIDLIST {
    pub cMTSIDs: u32,
    pub cbMTSIDs: u32,
    pub abMTSIDs: [u8; 1],
}
impl ::core::marker::Copy for FLATMTSIDLIST {}
impl ::core::clone::Clone for FLATMTSIDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type Gender = i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const genderUnspecified: Gender = 0i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const genderFemale: Gender = 1i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const genderMale: Gender = 2i32;
#[repr(C)]
pub struct IABContainer {
    pub base__: IMAPIContainer,
    pub CreateEntry: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CopyEntries: unsafe extern "system" fn(this: *mut *mut Self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub DeleteEntries: unsafe extern "system" fn(this: *mut *mut Self, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ResolveNames: unsafe extern "system" fn(this: *mut *mut Self, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut _flaglist) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ResolveNames: usize,
}
impl ::windows_sys::core::Interface for IABContainer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IAddrBook {
    pub base__: IMAPIProp,
    pub OpenEntry: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut ::windows_sys::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CompareEntryIDs: unsafe extern "system" fn(this: *mut *mut Self, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: *mut ::core::ffi::c_void, lpulconnection: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut *mut Self, ulconnection: u32) -> ::windows_sys::core::HRESULT,
    pub CreateOneOff: unsafe extern "system" fn(this: *mut *mut Self, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows_sys::core::HRESULT,
    pub NewEntry: unsafe extern "system" fn(this: *mut *mut Self, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ResolveName: unsafe extern "system" fn(this: *mut *mut Self, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ResolveName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Address: usize,
    pub Details: unsafe extern "system" fn(this: *mut *mut Self, lpuluiparam: *mut usize, lpfndismiss: *mut ::core::ffi::c_void, lpvdismisscontext: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: *mut ::core::ffi::c_void, lpvbuttoncontext: *mut ::core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RecipOptions: unsafe extern "system" fn(this: *mut *mut Self, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RecipOptions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryDefaultRecipOpt: unsafe extern "system" fn(this: *mut *mut Self, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryDefaultRecipOpt: usize,
    pub GetPAB: unsafe extern "system" fn(this: *mut *mut Self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows_sys::core::HRESULT,
    pub SetPAB: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows_sys::core::HRESULT,
    pub GetDefaultDir: unsafe extern "system" fn(this: *mut *mut Self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows_sys::core::HRESULT,
    pub SetDefaultDir: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetSearchPath: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetSearchPath: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetSearchPath: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpsearchpath: *mut SRowSet) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetSearchPath: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub PrepareRecips: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    PrepareRecips: usize,
}
impl ::windows_sys::core::Interface for IAddrBook {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IAttach {
    pub base__: IMAPIProp,
}
impl ::windows_sys::core::Interface for IAttach {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IDistList {
    pub base__: IMAPIContainer,
    pub CreateEntry: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CopyEntries: unsafe extern "system" fn(this: *mut *mut Self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub DeleteEntries: unsafe extern "system" fn(this: *mut *mut Self, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ResolveNames: unsafe extern "system" fn(this: *mut *mut Self, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut _flaglist) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ResolveNames: usize,
}
impl ::windows_sys::core::Interface for IDistList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IMAPIAdviseSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnNotify: unsafe extern "system" fn(this: *mut *mut Self, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnNotify: usize,
}
impl ::windows_sys::core::Interface for IMAPIAdviseSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IMAPIContainer {
    pub base__: IMAPIProp,
    pub GetContentsTable: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetHierarchyTable: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenEntry: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows_sys::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetSearchCriteria: unsafe extern "system" fn(this: *mut *mut Self, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetSearchCriteria: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetSearchCriteria: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetSearchCriteria: usize,
}
impl ::windows_sys::core::Interface for IMAPIContainer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IMAPIControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLastError: unsafe extern "system" fn(this: *mut *mut Self, hresult: ::windows_sys::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_sys::core::HRESULT,
    pub Activate: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, uluiparam: usize) -> ::windows_sys::core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpulstate: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMAPIControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IMAPIFolder {
    pub base__: IMAPIContainer,
    pub CreateMessage: unsafe extern "system" fn(this: *mut *mut Self, lpinterface: *mut ::windows_sys::core::GUID, ulflags: u32, lppmessage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CopyMessages: unsafe extern "system" fn(this: *mut *mut Self, lpmsglist: *const SBinaryArray, lpinterface: *const ::windows_sys::core::GUID, lpdestfolder: *const ::core::ffi::c_void, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub DeleteMessages: unsafe extern "system" fn(this: *mut *mut Self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub CreateFolder: unsafe extern "system" fn(this: *mut *mut Self, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: *const ::windows_sys::core::GUID, ulflags: u32, lppfolder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CopyFolder: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows_sys::core::GUID, lpdestfolder: *const ::core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub DeleteFolder: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub SetReadFlags: unsafe extern "system" fn(this: *mut *mut Self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetMessageStatus: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32, lpulmessagestatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMessageStatus: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32, lpuloldstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SaveContentsSort: unsafe extern "system" fn(this: *mut *mut Self, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub EmptyFolder: unsafe extern "system" fn(this: *mut *mut Self, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMAPIFolder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IMAPIProgress {
    pub base__: ::windows_sys::core::IUnknown,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, ulvalue: u32, ulcount: u32, ultotal: u32) -> ::windows_sys::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut *mut Self, lpulflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMax: unsafe extern "system" fn(this: *mut *mut Self, lpulmax: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMin: unsafe extern "system" fn(this: *mut *mut Self, lpulmin: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetLimits: unsafe extern "system" fn(this: *mut *mut Self, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMAPIProgress {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IMAPIProp {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLastError: unsafe extern "system" fn(this: *mut *mut Self, hresult: ::windows_sys::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_sys::core::HRESULT,
    pub SaveChanges: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetProps: unsafe extern "system" fn(this: *mut *mut Self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetProps: usize,
    pub GetPropList: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows_sys::core::HRESULT,
    pub OpenProperty: unsafe extern "system" fn(this: *mut *mut Self, ulproptag: u32, lpiid: *mut ::windows_sys::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetProps: unsafe extern "system" fn(this: *mut *mut Self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetProps: usize,
    pub DeleteProps: unsafe extern "system" fn(this: *mut *mut Self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows_sys::core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(this: *mut *mut Self, ciidexclude: u32, rgiidexclude: *mut ::windows_sys::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, lpinterface: *mut ::windows_sys::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows_sys::core::HRESULT,
    pub CopyProps: unsafe extern "system" fn(this: *mut *mut Self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, lpinterface: *mut ::windows_sys::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows_sys::core::HRESULT,
    pub GetNamesFromIDs: unsafe extern "system" fn(this: *mut *mut Self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows_sys::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows_sys::core::HRESULT,
    pub GetIDsFromNames: unsafe extern "system" fn(this: *mut *mut Self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMAPIProp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IMAPIStatus {
    pub base__: IMAPIProp,
    pub ValidateState: unsafe extern "system" fn(this: *mut *mut Self, uluiparam: usize, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub SettingsDialog: unsafe extern "system" fn(this: *mut *mut Self, uluiparam: usize, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub ChangePassword: unsafe extern "system" fn(this: *mut *mut Self, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub FlushQueues: unsafe extern "system" fn(this: *mut *mut Self, uluiparam: usize, cbtargettransport: u32, lptargettransport: *const ENTRYID, ulflags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMAPIStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IMAPITable {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLastError: unsafe extern "system" fn(this: *mut *mut Self, hresult: ::windows_sys::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_sys::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut *mut Self, uleventmask: u32, lpadvisesink: *mut ::core::ffi::c_void, lpulconnection: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut *mut Self, ulconnection: u32) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetColumns: unsafe extern "system" fn(this: *mut *mut Self, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub QueryColumns: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> ::windows_sys::core::HRESULT,
    pub GetRowCount: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpulcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SeekRow: unsafe extern "system" fn(this: *mut *mut Self, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SeekRowApprox: unsafe extern "system" fn(this: *mut *mut Self, ulnumerator: u32, uldenominator: u32) -> ::windows_sys::core::HRESULT,
    pub QueryPosition: unsafe extern "system" fn(this: *mut *mut Self, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub FindRow: unsafe extern "system" fn(this: *mut *mut Self, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    FindRow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Restrict: unsafe extern "system" fn(this: *mut *mut Self, lprestriction: *mut SRestriction, ulflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Restrict: usize,
    pub CreateBookmark: unsafe extern "system" fn(this: *mut *mut Self, lpbkposition: *mut u32) -> ::windows_sys::core::HRESULT,
    pub FreeBookmark: unsafe extern "system" fn(this: *mut *mut Self, bkposition: u32) -> ::windows_sys::core::HRESULT,
    pub SortTable: unsafe extern "system" fn(this: *mut *mut Self, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub QuerySortOrder: unsafe extern "system" fn(this: *mut *mut Self, lppsortcriteria: *mut *mut SSortOrderSet) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryRows: unsafe extern "system" fn(this: *mut *mut Self, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryRows: usize,
    pub Abort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ExpandRow: unsafe extern "system" fn(this: *mut *mut Self, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ExpandRow: usize,
    pub CollapseRow: unsafe extern "system" fn(this: *mut *mut Self, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetCollapseState: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetCollapseState: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMAPITable {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_BAD_MULTISESSION_PARAMETER: ::windows_sys::core::HRESULT = -1062555294i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_BOOT_EMULATION_IMAGE_SIZE_MISMATCH: ::windows_sys::core::HRESULT = -1062555318i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_BOOT_IMAGE_DATA: ::windows_sys::core::HRESULT = -1062555320i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_BOOT_OBJECT_CONFLICT: ::windows_sys::core::HRESULT = -1062555319i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DATA_STREAM_CREATE_FAILURE: ::windows_sys::core::HRESULT = -1062555350i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DATA_STREAM_INCONSISTENCY: ::windows_sys::core::HRESULT = -1062555352i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DATA_STREAM_READ_FAILURE: ::windows_sys::core::HRESULT = -1062555351i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DATA_TOO_BIG: ::windows_sys::core::HRESULT = -1062555342i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DIRECTORY_READ_FAILURE: ::windows_sys::core::HRESULT = -1062555349i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DIR_NOT_EMPTY: ::windows_sys::core::HRESULT = -1062555382i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DIR_NOT_FOUND: ::windows_sys::core::HRESULT = -1062555366i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DISC_MISMATCH: ::windows_sys::core::HRESULT = -1062555304i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DUP_NAME: ::windows_sys::core::HRESULT = -1062555374i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_EMPTY_DISC: ::windows_sys::core::HRESULT = -1062555312i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FILE_NOT_FOUND: ::windows_sys::core::HRESULT = -1062555367i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FILE_SYSTEM_CHANGE_NOT_ALLOWED: ::windows_sys::core::HRESULT = -1062555293i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FILE_SYSTEM_FEATURE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1062555308i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FILE_SYSTEM_NOT_EMPTY: ::windows_sys::core::HRESULT = -1062555386i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FILE_SYSTEM_NOT_FOUND: ::windows_sys::core::HRESULT = -1062555310i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FILE_SYSTEM_READ_CONSISTENCY_ERROR: ::windows_sys::core::HRESULT = -1062555309i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FSI_INTERNAL_ERROR: ::windows_sys::core::HRESULT = -1062555392i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMAGEMANAGER_IMAGE_NOT_ALIGNED: ::windows_sys::core::HRESULT = -1062555136i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMAGEMANAGER_IMAGE_TOO_BIG: ::windows_sys::core::HRESULT = -1062555133i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMAGEMANAGER_NO_IMAGE: ::windows_sys::core::HRESULT = -1062555134i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMAGEMANAGER_NO_VALID_VD_FOUND: ::windows_sys::core::HRESULT = -1062555135i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMAGE_SIZE_LIMIT: ::windows_sys::core::HRESULT = -1062555360i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMAGE_TOO_BIG: ::windows_sys::core::HRESULT = -1062555359i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMPORT_MEDIA_NOT_ALLOWED: ::windows_sys::core::HRESULT = -1062555303i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMPORT_READ_FAILURE: ::windows_sys::core::HRESULT = -1062555305i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMPORT_SEEK_FAILURE: ::windows_sys::core::HRESULT = -1062555306i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMPORT_TYPE_COLLISION_DIRECTORY_EXISTS_AS_FILE: ::windows_sys::core::HRESULT = -1062555298i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMPORT_TYPE_COLLISION_FILE_EXISTS_AS_DIRECTORY: ::windows_sys::core::HRESULT = -1062555307i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INCOMPATIBLE_MULTISESSION_TYPE: ::windows_sys::core::HRESULT = -1062555301i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INCOMPATIBLE_PREVIOUS_SESSION: ::windows_sys::core::HRESULT = -1062555341i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INVALID_DATE: ::windows_sys::core::HRESULT = -1062555387i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INVALID_PARAM: ::windows_sys::core::HRESULT = -1062555391i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INVALID_PATH: ::windows_sys::core::HRESULT = -1062555376i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INVALID_VOLUME_NAME: ::windows_sys::core::HRESULT = -1062555388i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INVALID_WORKING_DIRECTORY: ::windows_sys::core::HRESULT = -1062555328i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_ISO9660_LEVELS: ::windows_sys::core::HRESULT = -1062555343i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_ITEM_NOT_FOUND: ::windows_sys::core::HRESULT = -1062555368i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_MULTISESSION_NOT_SET: ::windows_sys::core::HRESULT = -1062555299i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NOT_DIR: ::windows_sys::core::HRESULT = -1062555383i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NOT_FILE: ::windows_sys::core::HRESULT = -1062555384i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NOT_IN_FILE_SYSTEM: ::windows_sys::core::HRESULT = -1062555381i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NO_COMPATIBLE_MULTISESSION_TYPE: ::windows_sys::core::HRESULT = -1062555300i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NO_OUTPUT: ::windows_sys::core::HRESULT = -1062555389i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NO_SUPPORTED_FILE_SYSTEM: ::windows_sys::core::HRESULT = -1062555311i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NO_UNIQUE_NAME: ::windows_sys::core::HRESULT = -1062555373i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_PROPERTY_NOT_ACCESSIBLE: ::windows_sys::core::HRESULT = -1062555296i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_READONLY: ::windows_sys::core::HRESULT = -1062555390i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_RESTRICTED_NAME_VIOLATION: ::windows_sys::core::HRESULT = -1062555375i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_STASHFILE_MOVE: ::windows_sys::core::HRESULT = -1062555326i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_STASHFILE_OPEN_FAILURE: ::windows_sys::core::HRESULT = -1062555336i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_STASHFILE_READ_FAILURE: ::windows_sys::core::HRESULT = -1062555333i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_STASHFILE_SEEK_FAILURE: ::windows_sys::core::HRESULT = -1062555335i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_STASHFILE_WRITE_FAILURE: ::windows_sys::core::HRESULT = -1062555334i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_TOO_MANY_DIRS: ::windows_sys::core::HRESULT = -1062555344i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_UDF_NOT_WRITE_COMPATIBLE: ::windows_sys::core::HRESULT = -1062555302i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_UDF_REVISION_CHANGE_NOT_ALLOWED: ::windows_sys::core::HRESULT = -1062555295i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_WORKING_DIRECTORY_SPACE: ::windows_sys::core::HRESULT = -1062555327i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_S_IMAGE_FEATURE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = 11186527i32;
#[repr(C)]
pub struct IMailUser {
    pub base__: IMAPIProp,
}
impl ::windows_sys::core::Interface for IMailUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IMessage {
    pub base__: IMAPIProp,
    pub GetAttachmentTable: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenAttach: unsafe extern "system" fn(this: *mut *mut Self, ulattachmentnum: u32, lpinterface: *const ::windows_sys::core::GUID, ulflags: u32, lppattach: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAttach: unsafe extern "system" fn(this: *mut *mut Self, lpinterface: *const ::windows_sys::core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeleteAttach: unsafe extern "system" fn(this: *mut *mut Self, ulattachmentnum: u32, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetRecipientTable: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ModifyRecipients: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpmods: *const ADRLIST) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ModifyRecipients: usize,
    pub SubmitMessage: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub SetReadFlag: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IMsgStore {
    pub base__: IMAPIProp,
    pub Advise: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: *mut ::core::ffi::c_void, lpulconnection: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut *mut Self, ulconnection: u32) -> ::windows_sys::core::HRESULT,
    pub CompareEntryIDs: unsafe extern "system" fn(this: *mut *mut Self, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows_sys::core::HRESULT,
    pub OpenEntry: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows_sys::core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetReceiveFolder: unsafe extern "system" fn(this: *mut *mut Self, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows_sys::core::HRESULT,
    pub GetReceiveFolder: unsafe extern "system" fn(this: *mut *mut Self, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> ::windows_sys::core::HRESULT,
    pub GetReceiveFolderTable: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StoreLogoff: unsafe extern "system" fn(this: *mut *mut Self, lpulflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AbortSubmit: unsafe extern "system" fn(this: *mut *mut Self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetOutgoingQueue: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLockState: unsafe extern "system" fn(this: *mut *mut Self, lpmessage: *mut ::core::ffi::c_void, ullockstate: u32) -> ::windows_sys::core::HRESULT,
    pub FinishedMsg: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub NotifyNewMail: unsafe extern "system" fn(this: *mut *mut Self, lpnotification: *const NOTIFICATION) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    NotifyNewMail: usize,
}
impl ::windows_sys::core::Interface for IMsgStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IProfSect {
    pub base__: IMAPIProp,
}
impl ::windows_sys::core::Interface for IProfSect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IPropData {
    pub base__: IMAPIProp,
    pub HrSetObjAccess: unsafe extern "system" fn(this: *mut *mut Self, ulaccess: u32) -> ::windows_sys::core::HRESULT,
    pub HrSetPropAccess: unsafe extern "system" fn(this: *mut *mut Self, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> ::windows_sys::core::HRESULT,
    pub HrGetPropAccess: unsafe extern "system" fn(this: *mut *mut Self, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> ::windows_sys::core::HRESULT,
    pub HrAddObjProps: unsafe extern "system" fn(this: *mut *mut Self, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPropData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IProviderAdmin {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLastError: unsafe extern "system" fn(this: *mut *mut Self, hresult: ::windows_sys::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_sys::core::HRESULT,
    pub GetProviderTable: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateProvider: unsafe extern "system" fn(this: *mut *mut Self, lpszprovider: *const i8, cvalues: u32, lpprops: *const SPropValue, uluiparam: usize, ulflags: u32, lpuid: *mut MAPIUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateProvider: usize,
    pub DeleteProvider: unsafe extern "system" fn(this: *mut *mut Self, lpuid: *const MAPIUID) -> ::windows_sys::core::HRESULT,
    pub OpenProfileSection: unsafe extern "system" fn(this: *mut *mut Self, lpuid: *const MAPIUID, lpinterface: *const ::windows_sys::core::GUID, ulflags: u32, lppprofsect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProviderAdmin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct ITableData {
    pub base__: ::windows_sys::core::IUnknown,
    pub HrGetView: unsafe extern "system" fn(this: *mut *mut Self, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut *mut ::core::ffi::c_void, ulcallerdata: u32, lppmapitable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrModifyRow: unsafe extern "system" fn(this: *mut *mut Self, param0: *mut SRow) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrModifyRow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrDeleteRow: unsafe extern "system" fn(this: *mut *mut Self, lpspropvalue: *mut SPropValue) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrDeleteRow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrQueryRow: unsafe extern "system" fn(this: *mut *mut Self, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrQueryRow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrEnumRow: unsafe extern "system" fn(this: *mut *mut Self, ulrownumber: u32, lppsrow: *mut *mut SRow) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrEnumRow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrNotify: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrNotify: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrInsertRow: unsafe extern "system" fn(this: *mut *mut Self, ulirow: u32, lpsrow: *mut SRow) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrInsertRow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrModifyRows: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lpsrowset: *mut SRowSet) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrModifyRows: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrDeleteRows: unsafe extern "system" fn(this: *mut *mut Self, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrDeleteRows: usize,
}
impl ::windows_sys::core::Interface for ITableData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IWABExtInit {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, lpwabextdisplay: *mut WABEXTDISPLAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
}
impl ::windows_sys::core::Interface for IWABExtInit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3928157168, data2: 34724, data3: 4561, data4: [154, 207, 0, 160, 201, 31, 156, 139] };
}
#[repr(C)]
pub struct IWABOBJECT_ {
    pub QueryInterface: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetLastError: unsafe extern "system" fn(this: *mut *mut Self, hresult: ::windows_sys::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_sys::core::HRESULT,
    pub AllocateBuffer: unsafe extern "system" fn(this: *mut *mut Self, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AllocateMore: unsafe extern "system" fn(this: *mut *mut Self, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Backup: unsafe extern "system" fn(this: *mut *mut Self, lpfilename: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT,
    pub Import: unsafe extern "system" fn(this: *mut *mut Self, lpwip: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Find: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Find: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VCardDisplay: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, lpszfilename: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VCardDisplay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LDAPUrl: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: ::windows_sys::core::PCSTR, lppmailuser: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LDAPUrl: usize,
    pub VCardCreate: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpszvcard: ::windows_sys::core::PCSTR, lpmailuser: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VCardRetrieve: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpszvcard: ::windows_sys::core::PCSTR, lppmailuser: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMe: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMe: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMe: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMe: usize,
}
impl ::windows_sys::core::Interface for IWABOBJECT_ {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_AddRef_METHOD = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_AllocateBuffer_METHOD = ::core::option::Option<unsafe extern "system" fn(cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_AllocateMore_METHOD = ::core::option::Option<unsafe extern "system" fn(cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_Backup_METHOD = ::core::option::Option<unsafe extern "system" fn(lpfilename: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_Find_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: *mut *mut IAddrBook, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_FreeBuffer_METHOD = ::core::option::Option<unsafe extern "system" fn(lpbuffer: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_GetLastError_METHOD = ::core::option::Option<unsafe extern "system" fn(hresult: ::windows_sys::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_GetMe_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: *mut *mut IAddrBook, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_Import_METHOD = ::core::option::Option<unsafe extern "system" fn(lpwip: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_LDAPUrl_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: *mut *mut IAddrBook, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: ::windows_sys::core::PCSTR, lppmailuser: *mut *mut *mut IMailUser) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_QueryInterface_METHOD = ::core::option::Option<unsafe extern "system" fn(riid: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_Release_METHOD = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_SetMe_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: *mut *mut IAddrBook, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_VCardCreate_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: *mut *mut IAddrBook, ulflags: u32, lpszvcard: ::windows_sys::core::PCSTR, lpmailuser: *mut *mut IMailUser) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_VCardDisplay_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: *mut *mut IAddrBook, hwnd: super::super::Foundation::HWND, lpszfilename: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_VCardRetrieve_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: *mut *mut IAddrBook, ulflags: u32, lpszvcard: ::windows_sys::core::PCSTR, lppmailuser: *mut *mut *mut IMailUser) -> ::windows_sys::core::HRESULT>;
#[repr(C)]
pub struct IWABObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLastError: unsafe extern "system" fn(this: *mut *mut Self, hresult: ::windows_sys::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_sys::core::HRESULT,
    pub AllocateBuffer: unsafe extern "system" fn(this: *mut *mut Self, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AllocateMore: unsafe extern "system" fn(this: *mut *mut Self, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut *mut Self, lpbuffer: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Backup: unsafe extern "system" fn(this: *mut *mut Self, lpfilename: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT,
    pub Import: unsafe extern "system" fn(this: *mut *mut Self, lpwip: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Find: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Find: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VCardDisplay: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, lpszfilename: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VCardDisplay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LDAPUrl: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: ::windows_sys::core::PCSTR, lppmailuser: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LDAPUrl: usize,
    pub VCardCreate: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpszvcard: ::windows_sys::core::PCSTR, lpmailuser: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VCardRetrieve: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpszvcard: ::windows_sys::core::PCSTR, lppmailuser: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMe: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMe: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMe: unsafe extern "system" fn(this: *mut *mut Self, lpiab: *mut ::core::ffi::c_void, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMe: usize,
}
impl ::windows_sys::core::Interface for IWABObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPALLOCATEBUFFER = ::core::option::Option<unsafe extern "system" fn(cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPALLOCATEMORE = ::core::option::Option<unsafe extern "system" fn(cbsize: u32, lpobject: *mut ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPCREATECONVERSATIONINDEX = ::core::option::Option<unsafe extern "system" fn(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPDISPATCHNOTIFICATIONS = ::core::option::Option<unsafe extern "system" fn(ulflags: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNABSDI = ::core::option::Option<unsafe extern "system" fn(uluiparam: usize, lpvmsg: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPFNBUTTON = ::core::option::Option<unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut ::core::ffi::c_void, cbentryid: u32, lpselection: *mut ENTRYID, ulflags: u32) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPFNDISMISS = ::core::option::Option<unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPFREEBUFFER = ::core::option::Option<unsafe extern "system" fn(lpbuffer: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNOTIFCALLBACK = ::core::option::Option<unsafe extern "system" fn(lpvcontext: *mut ::core::ffi::c_void, cnotification: u32, lpnotifications: *mut NOTIFICATION) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type LPOPENSTREAMONFILE = ::core::option::Option<unsafe extern "system" fn(lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8, lppstream: *mut *mut *mut super::Com::IStream) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPWABALLOCATEBUFFER = ::core::option::Option<unsafe extern "system" fn(lpwabobject: *mut *mut IWABObject, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPWABALLOCATEMORE = ::core::option::Option<unsafe extern "system" fn(lpwabobject: *mut *mut IWABObject, cbsize: u32, lpobject: *mut ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPWABFREEBUFFER = ::core::option::Option<unsafe extern "system" fn(lpwabobject: *mut *mut IWABObject, lpbuffer: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPWABOPEN = ::core::option::Option<unsafe extern "system" fn(lppadrbook: *mut *mut *mut IAddrBook, lppwabobject: *mut *mut *mut IWABObject, lpwp: *mut WAB_PARAM, reserved2: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPWABOPENEX = ::core::option::Option<unsafe extern "system" fn(lppadrbook: *mut *mut *mut IAddrBook, lppwabobject: *mut *mut *mut IWABObject, lpwp: *mut WAB_PARAM, reserved: u32, fnallocatebuffer: LPALLOCATEBUFFER, fnallocatemore: LPALLOCATEMORE, fnfreebuffer: LPFREEBUFFER) -> ::windows_sys::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct MAPIERROR {
    pub ulVersion: u32,
    pub lpszError: *mut i8,
    pub lpszComponent: *mut i8,
    pub ulLowLevelError: u32,
    pub ulContext: u32,
}
impl ::core::marker::Copy for MAPIERROR {}
impl ::core::clone::Clone for MAPIERROR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct MAPINAMEID {
    pub lpguid: *mut ::windows_sys::core::GUID,
    pub ulKind: u32,
    pub Kind: MAPINAMEID_0,
}
impl ::core::marker::Copy for MAPINAMEID {}
impl ::core::clone::Clone for MAPINAMEID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub union MAPINAMEID_0 {
    pub lID: i32,
    pub lpwstrName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for MAPINAMEID_0 {}
impl ::core::clone::Clone for MAPINAMEID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct MAPIUID {
    pub ab: [u8; 16],
}
impl ::core::marker::Copy for MAPIUID {}
impl ::core::clone::Clone for MAPIUID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_COMPOUND: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_DIM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_ERROR_VERSION: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_E_CALL_FAILED: i32 = -2147467259i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_E_INTERFACE_NOT_SUPPORTED: i32 = -2147467262i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_E_INVALID_PARAMETER: i32 = -2147024809i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_E_NOT_ENOUGH_MEMORY: i32 = -2147024882i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_E_NO_ACCESS: i32 = -2147024891i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_NOTRECIP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_NOTRESERVED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_NOW: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_ONE_OFF_NO_RICH_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_P1: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_SHORTTERM: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_SUBMITTED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_THISSESSION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_USE_DEFAULT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MNID_ID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MNID_STRING: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct MTSID {
    pub cb: u32,
    pub ab: [u8; 1],
}
impl ::core::marker::Copy for MTSID {}
impl ::core::clone::Clone for MTSID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MV_FLAG: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MV_INSTANCE: u32 = 8192u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct NEWMAIL_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub cbParentID: u32,
    pub lpParentID: *mut ENTRYID,
    pub ulFlags: u32,
    pub lpszMessageClass: *mut i8,
    pub ulMessageFlags: u32,
}
impl ::core::marker::Copy for NEWMAIL_NOTIFICATION {}
impl ::core::clone::Clone for NEWMAIL_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct NOTIFICATION {
    pub ulEventType: u32,
    pub ulAlignPad: u32,
    pub info: NOTIFICATION_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub union NOTIFICATION_0 {
    pub err: ERROR_NOTIFICATION,
    pub newmail: NEWMAIL_NOTIFICATION,
    pub obj: OBJECT_NOTIFICATION,
    pub tab: TABLE_NOTIFICATION,
    pub ext: EXTENDED_NOTIFICATION,
    pub statobj: STATUS_OBJECT_NOTIFICATION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for NOTIFICATION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for NOTIFICATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct NOTIFKEY {
    pub cb: u32,
    pub ab: [u8; 1],
}
impl ::core::marker::Copy for NOTIFKEY {}
impl ::core::clone::Clone for NOTIFKEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct OBJECT_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub ulObjType: u32,
    pub cbParentID: u32,
    pub lpParentID: *mut ENTRYID,
    pub cbOldID: u32,
    pub lpOldID: *mut ENTRYID,
    pub cbOldParentID: u32,
    pub lpOldParentID: *mut ENTRYID,
    pub lpPropTagArray: *mut SPropTagArray,
}
impl ::core::marker::Copy for OBJECT_NOTIFICATION {}
impl ::core::clone::Clone for OBJECT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const OPENSTREAMONFILE: &str = "OpenStreamOnFile";
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNIDLE = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PRIHIGHEST: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PRILOWEST: i32 = -32768i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PRIUSER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PROP_ID_INVALID: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PROP_ID_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PROP_ID_SECURE_MAX: u32 = 26623u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PROP_ID_SECURE_MIN: u32 = 26608u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SAndRestriction {
    pub cRes: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SAndRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SAndRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SAppTimeArray {
    pub cValues: u32,
    pub lpat: *mut f64,
}
impl ::core::marker::Copy for SAppTimeArray {}
impl ::core::clone::Clone for SAppTimeArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SBinary {
    pub cb: u32,
    pub lpb: *mut u8,
}
impl ::core::marker::Copy for SBinary {}
impl ::core::clone::Clone for SBinary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SBinaryArray {
    pub cValues: u32,
    pub lpbin: *mut SBinary,
}
impl ::core::marker::Copy for SBinaryArray {}
impl ::core::clone::Clone for SBinaryArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SBitMaskRestriction {
    pub relBMR: u32,
    pub ulPropTag: u32,
    pub ulMask: u32,
}
impl ::core::marker::Copy for SBitMaskRestriction {}
impl ::core::clone::Clone for SBitMaskRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SCommentRestriction {
    pub cValues: u32,
    pub lpRes: *mut SRestriction,
    pub lpProp: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SCommentRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SCommentRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SComparePropsRestriction {
    pub relop: u32,
    pub ulPropTag1: u32,
    pub ulPropTag2: u32,
}
impl ::core::marker::Copy for SComparePropsRestriction {}
impl ::core::clone::Clone for SComparePropsRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SContentRestriction {
    pub ulFuzzyLevel: u32,
    pub ulPropTag: u32,
    pub lpProp: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SContentRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SContentRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SCurrencyArray {
    pub cValues: u32,
    pub lpcur: *mut super::Com::CY,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SCurrencyArray {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SCurrencyArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SDateTimeArray {
    pub cValues: u32,
    pub lpft: *mut super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SDateTimeArray {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SDateTimeArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SDoubleArray {
    pub cValues: u32,
    pub lpdbl: *mut f64,
}
impl ::core::marker::Copy for SDoubleArray {}
impl ::core::clone::Clone for SDoubleArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const SERVICE_UI_ALLOWED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const SERVICE_UI_ALWAYS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SExistRestriction {
    pub ulReserved1: u32,
    pub ulPropTag: u32,
    pub ulReserved2: u32,
}
impl ::core::marker::Copy for SExistRestriction {}
impl ::core::clone::Clone for SExistRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SGuidArray {
    pub cValues: u32,
    pub lpguid: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for SGuidArray {}
impl ::core::clone::Clone for SGuidArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SLPSTRArray {
    pub cValues: u32,
    pub lppszA: *mut ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for SLPSTRArray {}
impl ::core::clone::Clone for SLPSTRArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SLargeIntegerArray {
    pub cValues: u32,
    pub lpli: *mut i64,
}
impl ::core::marker::Copy for SLargeIntegerArray {}
impl ::core::clone::Clone for SLargeIntegerArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SLongArray {
    pub cValues: u32,
    pub lpl: *mut i32,
}
impl ::core::marker::Copy for SLongArray {}
impl ::core::clone::Clone for SLongArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SNotRestriction {
    pub ulReserved: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SNotRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SNotRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SOrRestriction {
    pub cRes: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SOrRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SOrRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SPropProblem {
    pub ulIndex: u32,
    pub ulPropTag: u32,
    pub scode: i32,
}
impl ::core::marker::Copy for SPropProblem {}
impl ::core::clone::Clone for SPropProblem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SPropProblemArray {
    pub cProblem: u32,
    pub aProblem: [SPropProblem; 1],
}
impl ::core::marker::Copy for SPropProblemArray {}
impl ::core::clone::Clone for SPropProblemArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SPropTagArray {
    pub cValues: u32,
    pub aulPropTag: [u32; 1],
}
impl ::core::marker::Copy for SPropTagArray {}
impl ::core::clone::Clone for SPropTagArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SPropValue {
    pub ulPropTag: u32,
    pub dwAlignPad: u32,
    pub Value: _PV,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SPropValue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SPropValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SPropertyRestriction {
    pub relop: u32,
    pub ulPropTag: u32,
    pub lpProp: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SPropertyRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SPropertyRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SRealArray {
    pub cValues: u32,
    pub lpflt: *mut f32,
}
impl ::core::marker::Copy for SRealArray {}
impl ::core::clone::Clone for SRealArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SRestriction {
    pub rt: u32,
    pub res: SRestriction_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub union SRestriction_0 {
    pub resCompareProps: SComparePropsRestriction,
    pub resAnd: SAndRestriction,
    pub resOr: SOrRestriction,
    pub resNot: SNotRestriction,
    pub resContent: SContentRestriction,
    pub resProperty: SPropertyRestriction,
    pub resBitMask: SBitMaskRestriction,
    pub resSize: SSizeRestriction,
    pub resExist: SExistRestriction,
    pub resSub: SSubRestriction,
    pub resComment: SCommentRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SRestriction_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SRestriction_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SRow {
    pub ulAdrEntryPad: u32,
    pub cValues: u32,
    pub lpProps: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SRow {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SRow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SRowSet {
    pub cRows: u32,
    pub aRow: [SRow; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SRowSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SRowSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SShortArray {
    pub cValues: u32,
    pub lpi: *mut i16,
}
impl ::core::marker::Copy for SShortArray {}
impl ::core::clone::Clone for SShortArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SSizeRestriction {
    pub relop: u32,
    pub ulPropTag: u32,
    pub cb: u32,
}
impl ::core::marker::Copy for SSizeRestriction {}
impl ::core::clone::Clone for SSizeRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SSortOrder {
    pub ulPropTag: u32,
    pub ulOrder: u32,
}
impl ::core::marker::Copy for SSortOrder {}
impl ::core::clone::Clone for SSortOrder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SSortOrderSet {
    pub cSorts: u32,
    pub cCategories: u32,
    pub cExpanded: u32,
    pub aSort: [SSortOrder; 1],
}
impl ::core::marker::Copy for SSortOrderSet {}
impl ::core::clone::Clone for SSortOrderSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SSubRestriction {
    pub ulSubObject: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SSubRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SSubRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct STATUS_OBJECT_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub cValues: u32,
    pub lpPropVals: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for STATUS_OBJECT_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for STATUS_OBJECT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SWStringArray {
    pub cValues: u32,
    pub lppszW: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for SWStringArray {}
impl ::core::clone::Clone for SWStringArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const S_IMAPI_BOTHADJUSTED: ::windows_sys::core::HRESULT = 11141126i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const S_IMAPI_COMMAND_HAS_SENSE_DATA: ::windows_sys::core::HRESULT = 11141632i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const S_IMAPI_RAW_IMAGE_TRACK_INDEX_ALREADY_EXISTS: ::windows_sys::core::HRESULT = 11143688i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const S_IMAPI_ROTATIONADJUSTED: ::windows_sys::core::HRESULT = 11141125i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const S_IMAPI_SPEEDADJUSTED: ::windows_sys::core::HRESULT = 11141124i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const S_IMAPI_WRITE_NOT_IN_PROGRESS: ::windows_sys::core::HRESULT = 11141890i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_CHANGED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_ERROR: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct TABLE_NOTIFICATION {
    pub ulTableEvent: u32,
    pub hResult: ::windows_sys::core::HRESULT,
    pub propIndex: SPropValue,
    pub propPrior: SPropValue,
    pub row: SRow,
    pub ulPad: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for TABLE_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for TABLE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_RELOAD: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_RESTRICT_DONE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_ROW_ADDED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_ROW_DELETED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_ROW_MODIFIED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_SETCOL_DONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_SORT_DONE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TAD_ALL_ROWS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const UI_CURRENT_PROVIDER_FIRST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const UI_SERVICE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WABEXTDISPLAY {
    pub cbSize: u32,
    pub lpWABObject: *mut *mut *mut *mut IWABObject,
    pub lpAdrBook: *mut *mut *mut *mut IAddrBook,
    pub lpPropObj: *mut *mut *mut *mut IMAPIProp,
    pub fReadOnly: super::super::Foundation::BOOL,
    pub fDataChanged: super::super::Foundation::BOOL,
    pub ulFlags: u32,
    pub lpv: *mut ::core::ffi::c_void,
    pub lpsz: *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WABEXTDISPLAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WABEXTDISPLAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WABIMPORTPARAM {
    pub cbSize: u32,
    pub lpAdrBook: *mut *mut *mut *mut IAddrBook,
    pub hWnd: super::super::Foundation::HWND,
    pub ulFlags: u32,
    pub lpszFileName: ::windows_sys::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WABIMPORTPARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WABIMPORTPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WABOBJECT_LDAPURL_RETURN_MAILUSER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WABOBJECT_ME_NEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WABOBJECT_ME_NOCREATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_CONTEXT_ADRLIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_DISPLAY_ISNTDS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_DISPLAY_LDAPURL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_DLL_NAME: &str = "WAB32.DLL";
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_DLL_PATH_KEY: &str = "Software\\Microsoft\\WAB\\DLLPath";
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_ENABLE_PROFILES: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_IGNORE_PROFILES: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_LOCAL_CONTAINERS: u32 = 1048576u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WAB_PARAM {
    pub cbSize: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub szFileName: ::windows_sys::core::PSTR,
    pub ulFlags: u32,
    pub guidPSExt: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WAB_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAB_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_PROFILE_CONTENTS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_USE_OE_SENDMAIL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_VCARD_FILE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_VCARD_STREAM: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub union _PV {
    pub i: i16,
    pub l: i32,
    pub ul: u32,
    pub flt: f32,
    pub dbl: f64,
    pub b: u16,
    pub cur: super::Com::CY,
    pub at: f64,
    pub ft: super::super::Foundation::FILETIME,
    pub lpszA: ::windows_sys::core::PSTR,
    pub bin: SBinary,
    pub lpszW: ::windows_sys::core::PWSTR,
    pub lpguid: *mut ::windows_sys::core::GUID,
    pub li: i64,
    pub MVi: SShortArray,
    pub MVl: SLongArray,
    pub MVflt: SRealArray,
    pub MVdbl: SDoubleArray,
    pub MVcur: SCurrencyArray,
    pub MVat: SAppTimeArray,
    pub MVft: SDateTimeArray,
    pub MVbin: SBinaryArray,
    pub MVszA: SLPSTRArray,
    pub MVszW: SWStringArray,
    pub MVguid: SGuidArray,
    pub MVli: SLargeIntegerArray,
    pub err: i32,
    pub x: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for _PV {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for _PV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct _WABACTIONITEM(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct _flaglist {
    pub cFlags: u32,
    pub ulFlag: [u32; 1],
}
impl ::core::marker::Copy for _flaglist {}
impl ::core::clone::Clone for _flaglist {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const cchProfileNameMax: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const cchProfilePassMax: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const fMapiUnicode: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const hrSuccess: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const szHrDispatchNotifications: &str = "HrDispatchNotifications";
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const szMAPINotificationMsg: &str = "MAPI Notify window message";
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const szScCreateConversationIndex: &str = "ScCreateConversationIndex";
