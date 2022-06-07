#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn CoGetInstanceFromFile(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows_sys::core::GUID, punkouter: *mut *mut ::windows_sys::core::IUnknown, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: ::windows_sys::core::PCWSTR, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn CoGetInstanceFromIStorage(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows_sys::core::GUID, punkouter: *mut *mut ::windows_sys::core::IUnknown, dwclsctx: super::CLSCTX, pstg: *mut *mut IStorage, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn CoGetInterfaceAndReleaseStream(pstm: *mut *mut super::IStream, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateILockBytesOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, pplkbyt: *mut *mut *mut ILockBytes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStreamOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, ppstm: *mut *mut *mut super::IStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn FmtIdToPropStgName(pfmtid: *const ::windows_sys::core::GUID, oszname: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreePropVariantArray(cvariants: u32, rgvars: *mut PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn GetConvertStg(pstg: *mut *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn GetHGlobalFromILockBytes(plkbyt: *mut *mut ILockBytes, phglobal: *mut isize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn GetHGlobalFromStream(pstm: *mut *mut super::IStream, phglobal: *mut isize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn OleConvertIStorageToOLESTREAM(pstg: *mut *mut IStorage, lpolestream: *mut OLESTREAM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn OleConvertIStorageToOLESTREAMEx(pstg: *mut *mut IStorage, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *mut super::STGMEDIUM, polestm: *mut OLESTREAM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn OleConvertOLESTREAMToIStorage(lpolestream: *mut OLESTREAM, pstg: *mut *mut IStorage, ptd: *const super::DVTARGETDEVICE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn OleConvertOLESTREAMToIStorageEx(polestm: *mut OLESTREAM, pstg: *mut *mut IStorage, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut super::STGMEDIUM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn PropStgNameToFmtId(oszname: ::windows_sys::core::PCWSTR, pfmtid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PropVariantClear(pvar: *mut PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PropVariantCopy(pvardest: *mut PROPVARIANT, pvarsrc: *const PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn ReadClassStg(pstg: *mut *mut IStorage, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn ReadClassStm(pstm: *mut *mut super::IStream, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn ReadFmtUserTypeStg(pstg: *mut *mut IStorage, pcf: *mut u16, lplpszusertype: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConvertStg(pstg: *mut *mut IStorage, fconvert: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgConvertPropertyToVariant(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut PROPVARIANT, pma: *const PMemoryAllocator) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgConvertVariantToProperty(pvar: *const PROPVARIANT, codepage: u16, pprop: *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32, pid: u32, freserved: super::super::super::Foundation::BOOLEAN, pcindirect: *mut u32) -> *mut SERIALIZEDPROPERTYVALUE;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgCreateDocfile(pwcsname: ::windows_sys::core::PCWSTR, grfmode: STGM, reserved: u32, ppstgopen: *mut *mut *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgCreateDocfileOnILockBytes(plkbyt: *mut *mut ILockBytes, grfmode: STGM, reserved: u32, ppstgopen: *mut *mut *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgCreatePropSetStg(pstorage: *mut *mut IStorage, dwreserved: u32, pppropsetstg: *mut *mut *mut IPropertySetStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgCreatePropStg(punk: *mut *mut ::windows_sys::core::IUnknown, fmtid: *const ::windows_sys::core::GUID, pclsid: *const ::windows_sys::core::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut *mut *mut IPropertyStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn StgCreateStorageEx(pwcsname: ::windows_sys::core::PCWSTR, grfmode: STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: super::super::super::Security::PSECURITY_DESCRIPTOR, riid: *const ::windows_sys::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgDeserializePropVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32, ppropvar: *mut PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgGetIFillLockBytesOnFile(pwcsname: ::windows_sys::core::PCWSTR, ppflb: *mut *mut *mut IFillLockBytes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgGetIFillLockBytesOnILockBytes(pilb: *mut *mut ILockBytes, ppflb: *mut *mut *mut IFillLockBytes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgIsStorageFile(pwcsname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgIsStorageILockBytes(plkbyt: *mut *mut ILockBytes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgOpenAsyncDocfileOnIFillLockBytes(pflb: *mut *mut IFillLockBytes, grfmode: u32, asyncflags: u32, ppstgopen: *mut *mut *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgOpenLayoutDocfile(pwcsdfname: ::windows_sys::core::PCWSTR, grfmode: u32, reserved: u32, ppstgopen: *mut *mut *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgOpenPropStg(punk: *mut *mut ::windows_sys::core::IUnknown, fmtid: *const ::windows_sys::core::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut *mut *mut IPropertyStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgOpenStorage(pwcsname: ::windows_sys::core::PCWSTR, pstgpriority: *mut *mut IStorage, grfmode: STGM, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut *mut *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn StgOpenStorageEx(pwcsname: ::windows_sys::core::PCWSTR, grfmode: STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: super::super::super::Security::PSECURITY_DESCRIPTOR, riid: *const ::windows_sys::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgOpenStorageOnILockBytes(plkbyt: *mut *mut ILockBytes, pstgpriority: *mut *mut IStorage, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut *mut *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgPropertyLengthAsVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: u8) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgSerializePropVariant(ppropvar: *const PROPVARIANT, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgSetTimes(lpszname: ::windows_sys::core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn WriteClassStg(pstg: *mut *mut IStorage, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn WriteClassStm(pstm: *mut *mut super::IStream, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn WriteFmtUserTypeStg(pstg: *mut *mut IStorage, cf: u16, lpszusertype: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct BSTRBLOB {
    pub cbSize: u32,
    pub pData: *mut u8,
}
impl ::core::marker::Copy for BSTRBLOB {}
impl ::core::clone::Clone for BSTRBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl ::core::marker::Copy for CABOOL {}
impl ::core::clone::Clone for CABOOL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CABSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CABSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut BSTRBLOB,
}
impl ::core::marker::Copy for CABSTRBLOB {}
impl ::core::clone::Clone for CABSTRBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAC {
    pub cElems: u32,
    pub pElems: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for CAC {}
impl ::core::clone::Clone for CAC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut CLIPDATA,
}
impl ::core::marker::Copy for CACLIPDATA {}
impl ::core::clone::Clone for CACLIPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CACLSID {}
impl ::core::clone::Clone for CACLSID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut super::CY,
}
impl ::core::marker::Copy for CACY {}
impl ::core::clone::Clone for CACY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl ::core::marker::Copy for CADATE {}
impl ::core::clone::Clone for CADATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl ::core::marker::Copy for CADBL {}
impl ::core::clone::Clone for CADBL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CAFILETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CAFILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
impl ::core::marker::Copy for CAFLT {}
impl ::core::clone::Clone for CAFLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
impl ::core::marker::Copy for CAH {}
impl ::core::clone::Clone for CAH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl ::core::marker::Copy for CAI {}
impl ::core::clone::Clone for CAI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl ::core::marker::Copy for CAL {}
impl ::core::clone::Clone for CAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for CALPSTR {}
impl ::core::clone::Clone for CALPSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CALPWSTR {}
impl ::core::clone::Clone for CALPWSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CAPROPVARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CAPROPVARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl ::core::marker::Copy for CASCODE {}
impl ::core::clone::Clone for CASCODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
impl ::core::marker::Copy for CAUB {}
impl ::core::clone::Clone for CAUB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
impl ::core::marker::Copy for CAUH {}
impl ::core::clone::Clone for CAUH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
impl ::core::marker::Copy for CAUI {}
impl ::core::clone::Clone for CAUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl ::core::marker::Copy for CAUL {}
impl ::core::clone::Clone for CAUL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const CCH_MAX_PROPSTG_NAME: u32 = 31u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CLIPDATA {
    pub cbSize: u32,
    pub ulClipFmt: i32,
    pub pClipData: *mut u8,
}
impl ::core::marker::Copy for CLIPDATA {}
impl ::core::clone::Clone for CLIPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const CWCSTORAGENAME: u32 = 32u32;
#[repr(C)]
pub struct IDirectWriterLock {
    pub base__: ::windows_sys::core::IUnknown,
    pub WaitForWriteAccess: unsafe extern "system" fn(this: *mut *mut Self, dwtimeout: u32) -> ::windows_sys::core::HRESULT,
    pub ReleaseWriteAccess: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub HaveWriteAccess: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectWriterLock {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 242044306, data2: 26424, data3: 4559, data4: [150, 8, 0, 170, 0, 104, 13, 180] };
}
#[repr(C)]
pub struct IEnumSTATPROPSETSTG {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumSTATPROPSETSTG {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 315, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IEnumSTATPROPSTG {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumSTATPROPSTG {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 313, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IEnumSTATSTG {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumSTATSTG {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 13, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IFillLockBytes {
    pub base__: ::windows_sys::core::IUnknown,
    pub FillAppend: unsafe extern "system" fn(this: *mut *mut Self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_sys::core::HRESULT,
    pub FillAt: unsafe extern "system" fn(this: *mut *mut Self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetFillSize: unsafe extern "system" fn(this: *mut *mut Self, ulsize: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Terminate: unsafe extern "system" fn(this: *mut *mut Self, bcanceled: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Terminate: usize,
}
impl ::windows_sys::core::Interface for IFillLockBytes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2580213776, data2: 16734, data3: 4559, data4: [136, 20, 0, 170, 0, 181, 105, 245] };
}
#[repr(C)]
pub struct ILayoutStorage {
    pub base__: ::windows_sys::core::IUnknown,
    pub LayoutScript: unsafe extern "system" fn(this: *mut *mut Self, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows_sys::core::HRESULT,
    pub BeginMonitor: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EndMonitor: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReLayoutDocfile: unsafe extern "system" fn(this: *mut *mut Self, pwcsnewdfname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub ReLayoutDocfileOnILockBytes: unsafe extern "system" fn(this: *mut *mut Self, pilockbytes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILayoutStorage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 242044304, data2: 26424, data3: 4559, data4: [150, 8, 0, 170, 0, 104, 13, 180] };
}
#[repr(C)]
pub struct ILockBytes {
    pub base__: ::windows_sys::core::IUnknown,
    pub ReadAt: unsafe extern "system" fn(this: *mut *mut Self, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows_sys::core::HRESULT,
    pub WriteAt: unsafe extern "system" fn(this: *mut *mut Self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, cb: u64) -> ::windows_sys::core::HRESULT,
    pub LockRegion: unsafe extern "system" fn(this: *mut *mut Self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_sys::core::HRESULT,
    pub UnlockRegion: unsafe extern "system" fn(this: *mut *mut Self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Stat: unsafe extern "system" fn(this: *mut *mut Self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Stat: usize,
}
impl ::windows_sys::core::Interface for ILockBytes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 10, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IPersistStorage {
    pub base__: super::IPersist,
    pub IsDirty: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub InitNew: unsafe extern "system" fn(this: *mut *mut Self, pstg: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut *mut Self, pstg: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut *mut Self, pstgsave: *mut ::core::ffi::c_void, fsameasload: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub SaveCompleted: unsafe extern "system" fn(this: *mut *mut Self, pstgnew: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HandsOffStorage: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPersistStorage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 266, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IPropertyBag {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, pszpropname: ::windows_sys::core::PCWSTR, pvar: *mut super::VARIANT, perrorlog: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Read: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, pszpropname: ::windows_sys::core::PCWSTR, pvar: *const super::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Write: usize,
}
impl ::windows_sys::core::Interface for IPropertyBag {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1428630016, data2: 17099, data3: 4558, data4: [129, 53, 0, 170, 0, 75, 184, 81] };
}
#[repr(C)]
pub struct IPropertyBag2 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: *mut ::core::ffi::c_void, pvarvalue: *mut super::VARIANT, phrerror: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Read: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Write: usize,
    pub CountProperties: unsafe extern "system" fn(this: *mut *mut Self, pcproperties: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPropertyInfo: unsafe extern "system" fn(this: *mut *mut Self, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LoadObject: unsafe extern "system" fn(this: *mut *mut Self, pstrname: ::windows_sys::core::PCWSTR, dwhint: u32, punkobject: *mut ::core::ffi::c_void, perrlog: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPropertyBag2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 586504322, data2: 10251, data3: 4560, data4: [168, 169, 0, 160, 201, 12, 32, 4] };
}
#[repr(C)]
pub struct IPropertySetStorage {
    pub base__: ::windows_sys::core::IUnknown,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, rfmtid: *const ::windows_sys::core::GUID, pclsid: *const ::windows_sys::core::GUID, grfflags: u32, grfmode: u32, ppprstg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, rfmtid: *const ::windows_sys::core::GUID, grfmode: u32, ppprstg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, rfmtid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Enum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPropertySetStorage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 314, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IPropertyStorage {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub ReadMultiple: unsafe extern "system" fn(this: *mut *mut Self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReadMultiple: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteMultiple: unsafe extern "system" fn(this: *mut *mut Self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteMultiple: usize,
    pub DeleteMultiple: unsafe extern "system" fn(this: *mut *mut Self, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows_sys::core::HRESULT,
    pub ReadPropertyNames: unsafe extern "system" fn(this: *mut *mut Self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub WritePropertyNames: unsafe extern "system" fn(this: *mut *mut Self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub DeletePropertyNames: unsafe extern "system" fn(this: *mut *mut Self, cpropid: u32, rgpropid: *const u32) -> ::windows_sys::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self, grfcommitflags: u32) -> ::windows_sys::core::HRESULT,
    pub Revert: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Enum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTimes: unsafe extern "system" fn(this: *mut *mut Self, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTimes: usize,
    pub SetClass: unsafe extern "system" fn(this: *mut *mut Self, clsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Stat: unsafe extern "system" fn(this: *mut *mut Self, pstatpsstg: *mut STATPROPSETSTG) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Stat: usize,
}
impl ::windows_sys::core::Interface for IPropertyStorage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 312, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IRootStorage {
    pub base__: ::windows_sys::core::IUnknown,
    pub SwitchToFile: unsafe extern "system" fn(this: *mut *mut Self, pszfile: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRootStorage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 18, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IStorage {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateStream: unsafe extern "system" fn(this: *mut *mut Self, pwcsname: ::windows_sys::core::PCWSTR, grfmode: STGM, reserved1: u32, reserved2: u32, ppstm: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenStream: unsafe extern "system" fn(this: *mut *mut Self, pwcsname: ::windows_sys::core::PCWSTR, reserved1: *mut ::core::ffi::c_void, grfmode: STGM, reserved2: u32, ppstm: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateStorage: unsafe extern "system" fn(this: *mut *mut Self, pwcsname: ::windows_sys::core::PCWSTR, grfmode: STGM, reserved1: u32, reserved2: u32, ppstg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenStorage: unsafe extern "system" fn(this: *mut *mut Self, pwcsname: ::windows_sys::core::PCWSTR, pstgpriority: *mut ::core::ffi::c_void, grfmode: STGM, snbexclude: *const *const u16, reserved: u32, ppstg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(this: *mut *mut Self, ciidexclude: u32, rgiidexclude: *const ::windows_sys::core::GUID, snbexclude: *const *const u16, pstgdest: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MoveElementTo: unsafe extern "system" fn(this: *mut *mut Self, pwcsname: ::windows_sys::core::PCWSTR, pstgdest: *mut ::core::ffi::c_void, pwcsnewname: ::windows_sys::core::PCWSTR, grfflags: STGMOVE) -> ::windows_sys::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self, grfcommitflags: super::STGC) -> ::windows_sys::core::HRESULT,
    pub Revert: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnumElements: unsafe extern "system" fn(this: *mut *mut Self, reserved1: u32, reserved2: *mut ::core::ffi::c_void, reserved3: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DestroyElement: unsafe extern "system" fn(this: *mut *mut Self, pwcsname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub RenameElement: unsafe extern "system" fn(this: *mut *mut Self, pwcsoldname: ::windows_sys::core::PCWSTR, pwcsnewname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetElementTimes: unsafe extern "system" fn(this: *mut *mut Self, pwcsname: ::windows_sys::core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetElementTimes: usize,
    pub SetClass: unsafe extern "system" fn(this: *mut *mut Self, clsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetStateBits: unsafe extern "system" fn(this: *mut *mut Self, grfstatebits: u32, grfmask: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Stat: unsafe extern "system" fn(this: *mut *mut Self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Stat: usize,
}
impl ::windows_sys::core::Interface for IStorage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 11, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type LOCKTYPE = i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const LOCK_WRITE: LOCKTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const LOCK_EXCLUSIVE: LOCKTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const LOCK_ONLYONCE: LOCKTYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct OLESTREAM {
    pub lpstbl: *mut OLESTREAMVTBL,
}
impl ::core::marker::Copy for OLESTREAM {}
impl ::core::clone::Clone for OLESTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct OLESTREAMVTBL {
    pub Get: isize,
    pub Put: isize,
}
impl ::core::marker::Copy for OLESTREAMVTBL {}
impl ::core::clone::Clone for OLESTREAMVTBL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDI_THUMBNAIL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_BYTECOUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_CATEGORY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_COMPANY: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_DOCPARTS: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_HEADINGPAIR: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_HIDDENCOUNT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_LINECOUNT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_LINKSDIRTY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_MANAGER: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_MMCLIPCOUNT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_NOTECOUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_PARCOUNT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_PRESFORMAT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_SCALE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_SLIDECOUNT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_COPYRIGHT: i32 = 11i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_EDITOR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_OWNER: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_PRODUCTION: i32 = 10i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_PROJECT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_RATING: i32 = 9i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_SEQUENCE_NO: i32 = 5i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_SOURCE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS: i32 = 7i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type PIDMSI_STATUS_VALUE = i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_NORMAL: PIDMSI_STATUS_VALUE = 0i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_NEW: PIDMSI_STATUS_VALUE = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_PRELIM: PIDMSI_STATUS_VALUE = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_DRAFT: PIDMSI_STATUS_VALUE = 3i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_INPROGRESS: PIDMSI_STATUS_VALUE = 4i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_EDIT: PIDMSI_STATUS_VALUE = 5i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_REVIEW: PIDMSI_STATUS_VALUE = 6i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_PROOF: PIDMSI_STATUS_VALUE = 7i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_FINAL: PIDMSI_STATUS_VALUE = 8i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_OTHER: PIDMSI_STATUS_VALUE = 32767i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_SUPPLIER: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_APPNAME: i32 = 18i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_AUTHOR: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_CHARCOUNT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_COMMENTS: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_CREATE_DTM: i32 = 12i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_DOC_SECURITY: i32 = 19i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_EDITTIME: i32 = 10i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_KEYWORDS: i32 = 5i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_LASTAUTHOR: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_LASTPRINTED: i32 = 11i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_LASTSAVE_DTM: i32 = 13i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_PAGECOUNT: i32 = 14i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_REVNUMBER: i32 = 9i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_SUBJECT: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_TEMPLATE: i32 = 7i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_THUMBNAIL: i32 = 17i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_TITLE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_WORDCOUNT: i32 = 15i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_BEHAVIOR: u32 = 2147483651u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_CODEPAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_DICTIONARY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_FIRST_NAME_DEFAULT: u32 = 4095u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_FIRST_USABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_ILLEGAL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_LOCALE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_MAX_READONLY: u32 = 3221225471u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_MIN_READONLY: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_MODIFY_TIME: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_SECURITY: u32 = 2147483650u32;
#[repr(C)]
pub struct PMemoryAllocator(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct PROPBAG2 {
    pub dwType: u32,
    pub vt: u16,
    pub cfType: u16,
    pub dwHint: u32,
    pub pstrName: ::windows_sys::core::PWSTR,
    pub clsid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for PROPBAG2 {}
impl ::core::clone::Clone for PROPBAG2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_ANSI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_CASE_SENSITIVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_NONSIMPLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_UNBUFFERED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct PROPSPEC {
    pub ulKind: PROPSPEC_KIND,
    pub Anonymous: PROPSPEC_0,
}
impl ::core::marker::Copy for PROPSPEC {}
impl ::core::clone::Clone for PROPSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub union PROPSPEC_0 {
    pub propid: u32,
    pub lpwstr: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PROPSPEC_0 {}
impl ::core::clone::Clone for PROPSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type PROPSPEC_KIND = u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PRSPEC_LPWSTR: PROPSPEC_KIND = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PRSPEC_PROPID: PROPSPEC_KIND = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPVARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PROPVARIANT_0 {
    pub Anonymous: PROPVARIANT_0_0,
    pub decVal: super::super::super::Foundation::DECIMAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPVARIANT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPVARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: PROPVARIANT_0_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPVARIANT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PROPVARIANT_0_0_0 {
    pub cVal: super::super::super::Foundation::CHAR,
    pub bVal: u8,
    pub iVal: i16,
    pub uiVal: u16,
    pub lVal: i32,
    pub ulVal: u32,
    pub intVal: i32,
    pub uintVal: u32,
    pub hVal: i64,
    pub uhVal: u64,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: i16,
    pub __OBSOLETE__VARIANT_BOOL: i16,
    pub scode: i32,
    pub cyVal: super::CY,
    pub date: f64,
    pub filetime: super::super::super::Foundation::FILETIME,
    pub puuid: *mut ::windows_sys::core::GUID,
    pub pclipdata: *mut CLIPDATA,
    pub bstrVal: super::super::super::Foundation::BSTR,
    pub bstrblobVal: BSTRBLOB,
    pub blob: super::BLOB,
    pub pszVal: ::windows_sys::core::PSTR,
    pub pwszVal: ::windows_sys::core::PWSTR,
    pub punkVal: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
    pub pdispVal: *mut *mut *mut *mut super::IDispatch,
    pub pStream: *mut *mut *mut *mut super::IStream,
    pub pStorage: *mut *mut *mut *mut IStorage,
    pub pVersionedStream: *mut VERSIONEDSTREAM,
    pub parray: *mut super::SAFEARRAY,
    pub cac: CAC,
    pub caub: CAUB,
    pub cai: CAI,
    pub caui: CAUI,
    pub cal: CAL,
    pub caul: CAUL,
    pub cah: CAH,
    pub cauh: CAUH,
    pub caflt: CAFLT,
    pub cadbl: CADBL,
    pub cabool: CABOOL,
    pub cascode: CASCODE,
    pub cacy: CACY,
    pub cadate: CADATE,
    pub cafiletime: CAFILETIME,
    pub cauuid: CACLSID,
    pub caclipdata: CACLIPDATA,
    pub cabstr: CABSTR,
    pub cabstrblob: CABSTRBLOB,
    pub calpstr: CALPSTR,
    pub calpwstr: CALPWSTR,
    pub capropvar: CAPROPVARIANT,
    pub pcVal: ::windows_sys::core::PSTR,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub puiVal: *mut u16,
    pub plVal: *mut i32,
    pub pulVal: *mut u32,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut i16,
    pub pdecVal: *mut super::super::super::Foundation::DECIMAL,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut super::super::super::Foundation::BSTR,
    pub ppunkVal: *mut *mut *mut ::windows_sys::core::IUnknown,
    pub ppdispVal: *mut *mut *mut super::IDispatch,
    pub pparray: *mut *mut super::SAFEARRAY,
    pub pvarVal: *mut PROPVARIANT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPVARIANT_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PRSPEC_INVALID: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct RemSNB {
    pub ulCntStr: u32,
    pub ulCntChar: u32,
    pub rgString: [u16; 1],
}
impl ::core::marker::Copy for RemSNB {}
impl ::core::clone::Clone for RemSNB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct SERIALIZEDPROPERTYVALUE {
    pub dwType: u32,
    pub rgb: [u8; 1],
}
impl ::core::marker::Copy for SERIALIZEDPROPERTYVALUE {}
impl ::core::clone::Clone for SERIALIZEDPROPERTYVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type STATFLAG = i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STATFLAG_DEFAULT: STATFLAG = 0i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STATFLAG_NONAME: STATFLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STATFLAG_NOOPEN: STATFLAG = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STATPROPSETSTG {
    pub fmtid: ::windows_sys::core::GUID,
    pub clsid: ::windows_sys::core::GUID,
    pub grfFlags: u32,
    pub mtime: super::super::super::Foundation::FILETIME,
    pub ctime: super::super::super::Foundation::FILETIME,
    pub atime: super::super::super::Foundation::FILETIME,
    pub dwOSVersion: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STATPROPSETSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STATPROPSETSTG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct STATPROPSTG {
    pub lpwstrName: ::windows_sys::core::PWSTR,
    pub propid: u32,
    pub vt: u16,
}
impl ::core::marker::Copy for STATPROPSTG {}
impl ::core::clone::Clone for STATPROPSTG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type STGFMT = u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_STORAGE: STGFMT = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_NATIVE: STGFMT = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_FILE: STGFMT = 3u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_ANY: STGFMT = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_DOCFILE: STGFMT = 5u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_DOCUMENT: STGFMT = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type STGM = u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_DIRECT: STGM = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_TRANSACTED: STGM = 65536u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SIMPLE: STGM = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_READ: STGM = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_WRITE: STGM = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_READWRITE: STGM = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SHARE_DENY_NONE: STGM = 64u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SHARE_DENY_READ: STGM = 48u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SHARE_DENY_WRITE: STGM = 32u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SHARE_EXCLUSIVE: STGM = 16u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_PRIORITY: STGM = 262144u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_DELETEONRELEASE: STGM = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_NOSCRATCH: STGM = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_CREATE: STGM = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_CONVERT: STGM = 131072u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_FAILIFTHERE: STGM = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_NOSNAPSHOT: STGM = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_DIRECT_SWMR: STGM = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type STGMOVE = i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGMOVE_MOVE: STGMOVE = 0i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGMOVE_COPY: STGMOVE = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGMOVE_SHALLOWCOPY: STGMOVE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct STGOPTIONS {
    pub usVersion: u16,
    pub reserved: u16,
    pub ulSectorSize: u32,
    pub pwcsTemplateFile: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for STGOPTIONS {}
impl ::core::clone::Clone for STGOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGOPTIONS_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct VERSIONEDSTREAM {
    pub guidVersion: ::windows_sys::core::GUID,
    pub pStream: *mut *mut *mut *mut super::IStream,
}
impl ::core::marker::Copy for VERSIONEDSTREAM {}
impl ::core::clone::Clone for VERSIONEDSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
