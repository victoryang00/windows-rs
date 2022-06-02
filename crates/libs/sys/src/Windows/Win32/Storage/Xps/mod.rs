#[cfg(feature = "Win32_Storage_Xps_Printing")]
pub mod Printing;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn AbortDoc(hdc: super::super::Graphics::Gdi::HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DeviceCapabilitiesA(pdevice: ::windows_sys::core::PCSTR, pport: ::windows_sys::core::PCSTR, fwcapability: DEVICE_CAPABILITIES, poutput: ::windows_sys::core::PSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEA) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DeviceCapabilitiesW(pdevice: ::windows_sys::core::PCWSTR, pport: ::windows_sys::core::PCWSTR, fwcapability: DEVICE_CAPABILITIES, poutput: ::windows_sys::core::PWSTR, pdevmode: *const super::super::Graphics::Gdi::DEVMODEW) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn EndDoc(hdc: super::super::Graphics::Gdi::HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn EndPage(hdc: super::super::Graphics::Gdi::HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn Escape(hdc: super::super::Graphics::Gdi::HDC, iescape: i32, cjin: i32, pvin: ::windows_sys::core::PCSTR, pvout: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ExtEscape(hdc: super::super::Graphics::Gdi::HDC, iescape: i32, cjinput: i32, lpindata: ::windows_sys::core::PCSTR, cjoutput: i32, lpoutdata: ::windows_sys::core::PSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintWindow(hwnd: super::super::Foundation::HWND, hdcblt: super::super::Graphics::Gdi::HDC, nflags: PRINT_WINDOW_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetAbortProc(hdc: super::super::Graphics::Gdi::HDC, proc: ABORTPROC) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn StartDocA(hdc: super::super::Graphics::Gdi::HDC, lpdi: *const DOCINFOA) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn StartDocW(hdc: super::super::Graphics::Gdi::HDC, lpdi: *const DOCINFOW) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn StartPage(hdc: super::super::Graphics::Gdi::HDC) -> i32;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type ABORTPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Graphics::Gdi::HDC, param1: i32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type DEVICE_CAPABILITIES = u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_BINNAMES: DEVICE_CAPABILITIES = 12u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_BINS: DEVICE_CAPABILITIES = 6u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_COLLATE: DEVICE_CAPABILITIES = 22u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_COLORDEVICE: DEVICE_CAPABILITIES = 32u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_COPIES: DEVICE_CAPABILITIES = 18u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_DRIVER: DEVICE_CAPABILITIES = 11u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_DUPLEX: DEVICE_CAPABILITIES = 7u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_ENUMRESOLUTIONS: DEVICE_CAPABILITIES = 13u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_EXTRA: DEVICE_CAPABILITIES = 9u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_FIELDS: DEVICE_CAPABILITIES = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_FILEDEPENDENCIES: DEVICE_CAPABILITIES = 14u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MAXEXTENT: DEVICE_CAPABILITIES = 5u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MEDIAREADY: DEVICE_CAPABILITIES = 29u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MEDIATYPENAMES: DEVICE_CAPABILITIES = 34u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MEDIATYPES: DEVICE_CAPABILITIES = 35u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MINEXTENT: DEVICE_CAPABILITIES = 4u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_ORIENTATION: DEVICE_CAPABILITIES = 17u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_NUP: DEVICE_CAPABILITIES = 33u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PAPERNAMES: DEVICE_CAPABILITIES = 16u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PAPERS: DEVICE_CAPABILITIES = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PAPERSIZE: DEVICE_CAPABILITIES = 3u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PERSONALITY: DEVICE_CAPABILITIES = 25u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PRINTERMEM: DEVICE_CAPABILITIES = 28u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PRINTRATE: DEVICE_CAPABILITIES = 26u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PRINTRATEPPM: DEVICE_CAPABILITIES = 31u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PRINTRATEUNIT: DEVICE_CAPABILITIES = 27u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_SIZE: DEVICE_CAPABILITIES = 8u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_STAPLE: DEVICE_CAPABILITIES = 30u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_TRUETYPE: DEVICE_CAPABILITIES = 15u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_VERSION: DEVICE_CAPABILITIES = 10u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct DOCINFOA {
    pub cbSize: i32,
    pub lpszDocName: ::windows_sys::core::PCSTR,
    pub lpszOutput: ::windows_sys::core::PCSTR,
    pub lpszDatatype: ::windows_sys::core::PCSTR,
    pub fwType: u32,
}
impl ::core::marker::Copy for DOCINFOA {}
impl ::core::clone::Clone for DOCINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct DOCINFOW {
    pub cbSize: i32,
    pub lpszDocName: ::windows_sys::core::PCWSTR,
    pub lpszOutput: ::windows_sys::core::PCWSTR,
    pub lpszDatatype: ::windows_sys::core::PCWSTR,
    pub fwType: u32,
}
impl ::core::marker::Copy for DOCINFOW {}
impl ::core::clone::Clone for DOCINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRAWPATRECT {
    pub ptPosition: super::super::Foundation::POINT,
    pub ptSize: super::super::Foundation::POINT,
    pub wStyle: u16,
    pub wPattern: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRAWPATRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRAWPATRECT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HPTPROVIDER = isize;
#[repr(C)]
pub struct IXpsDocumentPackageTarget {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetXpsOMPackageWriter: unsafe extern "system" fn(this: *mut *mut Self, documentsequencepartname: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetXpsOMPackageWriter: usize,
    pub GetXpsOMFactory: unsafe extern "system" fn(this: *mut *mut Self, xpsfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetXpsType: unsafe extern "system" fn(this: *mut *mut Self, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsDocumentPackageTarget3D {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetXpsOMPackageWriter3D: unsafe extern "system" fn(this: *mut *mut Self, documentsequencepartname: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, modelpartname: *mut ::core::ffi::c_void, modeldata: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetXpsOMPackageWriter3D: usize,
    pub GetXpsOMFactory: unsafe extern "system" fn(this: *mut *mut Self, xpsfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMBrush {
    pub base__: IXpsOMShareable,
    pub GetOpacity: unsafe extern "system" fn(this: *mut *mut Self, opacity: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut *mut Self, opacity: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMCanvas {
    pub base__: IXpsOMVisual,
    pub GetVisuals: unsafe extern "system" fn(this: *mut *mut Self, visuals: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUseAliasedEdgeMode: unsafe extern "system" fn(this: *mut *mut Self, usealiasededgemode: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUseAliasedEdgeMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseAliasedEdgeMode: unsafe extern "system" fn(this: *mut *mut Self, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseAliasedEdgeMode: usize,
    pub GetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut *mut Self, shortdescription: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut *mut Self, shortdescription: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut *mut Self, longdescription: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut *mut Self, longdescription: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDictionary: unsafe extern "system" fn(this: *mut *mut Self, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDictionaryLocal: unsafe extern "system" fn(this: *mut *mut Self, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDictionaryLocal: unsafe extern "system" fn(this: *mut *mut Self, resourcedictionary: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDictionaryResource: unsafe extern "system" fn(this: *mut *mut Self, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDictionaryResource: unsafe extern "system" fn(this: *mut *mut Self, remotedictionaryresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, canvas: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMColorProfileResource {
    pub base__: IXpsOMResource,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[repr(C)]
pub struct IXpsOMColorProfileResourceCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut *mut Self, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[repr(C)]
pub struct IXpsOMCoreProperties {
    pub base__: IXpsOMPart,
    pub GetOwner: unsafe extern "system" fn(this: *mut *mut Self, package: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(this: *mut *mut Self, category: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(this: *mut *mut Self, category: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetContentStatus: unsafe extern "system" fn(this: *mut *mut Self, contentstatus: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetContentStatus: unsafe extern "system" fn(this: *mut *mut Self, contentstatus: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetContentType: unsafe extern "system" fn(this: *mut *mut Self, contenttype: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut *mut Self, contenttype: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCreated: unsafe extern "system" fn(this: *mut *mut Self, created: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCreated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCreated: unsafe extern "system" fn(this: *mut *mut Self, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCreated: usize,
    pub GetCreator: unsafe extern "system" fn(this: *mut *mut Self, creator: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetCreator: unsafe extern "system" fn(this: *mut *mut Self, creator: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, description: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, description: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetIdentifier: unsafe extern "system" fn(this: *mut *mut Self, identifier: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetIdentifier: unsafe extern "system" fn(this: *mut *mut Self, identifier: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetKeywords: unsafe extern "system" fn(this: *mut *mut Self, keywords: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetKeywords: unsafe extern "system" fn(this: *mut *mut Self, keywords: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(this: *mut *mut Self, language: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut *mut Self, language: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetLastModifiedBy: unsafe extern "system" fn(this: *mut *mut Self, lastmodifiedby: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetLastModifiedBy: unsafe extern "system" fn(this: *mut *mut Self, lastmodifiedby: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastPrinted: unsafe extern "system" fn(this: *mut *mut Self, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastPrinted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLastPrinted: unsafe extern "system" fn(this: *mut *mut Self, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLastPrinted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetModified: unsafe extern "system" fn(this: *mut *mut Self, modified: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetModified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetModified: unsafe extern "system" fn(this: *mut *mut Self, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetModified: usize,
    pub GetRevision: unsafe extern "system" fn(this: *mut *mut Self, revision: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetRevision: unsafe extern "system" fn(this: *mut *mut Self, revision: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSubject: unsafe extern "system" fn(this: *mut *mut Self, subject: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut *mut Self, subject: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetTitle: unsafe extern "system" fn(this: *mut *mut Self, title: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, title: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut *mut Self, version: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut *mut Self, version: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMDashCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, dash: *mut XPS_DASH) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, dash: *const XPS_DASH) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, dash: *const XPS_DASH) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, dash: *const XPS_DASH) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMDictionary {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, key: *mut ::windows_sys::core::PWSTR, entry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetByKey: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::PCWSTR, beforeentry: *mut ::core::ffi::c_void, entry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(this: *mut *mut Self, entry: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, key: ::windows_sys::core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, key: ::windows_sys::core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMDocument {
    pub base__: IXpsOMPart,
    pub GetOwner: unsafe extern "system" fn(this: *mut *mut Self, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPageReferences: unsafe extern "system" fn(this: *mut *mut Self, pagereferences: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(this: *mut *mut Self, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(this: *mut *mut Self, printticketresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDocumentStructureResource: unsafe extern "system" fn(this: *mut *mut Self, documentstructureresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDocumentStructureResource: unsafe extern "system" fn(this: *mut *mut Self, documentstructureresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSignatureBlockResources: unsafe extern "system" fn(this: *mut *mut Self, signatureblockresources: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, document: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMDocumentCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, document: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, document: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, document: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, document: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMDocumentSequence {
    pub base__: IXpsOMPart,
    pub GetOwner: unsafe extern "system" fn(this: *mut *mut Self, package: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDocuments: unsafe extern "system" fn(this: *mut *mut Self, documents: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(this: *mut *mut Self, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(this: *mut *mut Self, printticketresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMDocumentStructureResource {
    pub base__: IXpsOMResource,
    pub GetOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[repr(C)]
pub struct IXpsOMFontResource {
    pub base__: IXpsOMResource,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, readerstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, sourcestream: *mut ::core::ffi::c_void, embeddingoption: XPS_FONT_EMBEDDING, partname: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
    pub GetEmbeddingOption: unsafe extern "system" fn(this: *mut *mut Self, embeddingoption: *mut XPS_FONT_EMBEDDING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMFontResourceCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut *mut Self, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[repr(C)]
pub struct IXpsOMGeometry {
    pub base__: IXpsOMShareable,
    pub GetFigures: unsafe extern "system" fn(this: *mut *mut Self, figures: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFillRule: unsafe extern "system" fn(this: *mut *mut Self, fillrule: *mut XPS_FILL_RULE) -> ::windows_sys::core::HRESULT,
    pub SetFillRule: unsafe extern "system" fn(this: *mut *mut Self, fillrule: XPS_FILL_RULE) -> ::windows_sys::core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut *mut Self, lookup: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut *mut Self, lookup: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMGeometryFigure {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSegmentData: unsafe extern "system" fn(this: *mut *mut Self, datacount: *mut u32, segmentdata: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetSegmentTypes: unsafe extern "system" fn(this: *mut *mut Self, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSegmentStrokes: unsafe extern "system" fn(this: *mut *mut Self, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSegmentStrokes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSegments: unsafe extern "system" fn(this: *mut *mut Self, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSegments: usize,
    pub GetStartPoint: unsafe extern "system" fn(this: *mut *mut Self, startpoint: *mut XPS_POINT) -> ::windows_sys::core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(this: *mut *mut Self, startpoint: *const XPS_POINT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsClosed: unsafe extern "system" fn(this: *mut *mut Self, isclosed: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsClosed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsClosed: unsafe extern "system" fn(this: *mut *mut Self, isclosed: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsClosed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsFilled: unsafe extern "system" fn(this: *mut *mut Self, isfilled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsFilled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsFilled: unsafe extern "system" fn(this: *mut *mut Self, isfilled: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsFilled: usize,
    pub GetSegmentCount: unsafe extern "system" fn(this: *mut *mut Self, segmentcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSegmentDataCount: unsafe extern "system" fn(this: *mut *mut Self, segmentdatacount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSegmentStrokePattern: unsafe extern "system" fn(this: *mut *mut Self, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, geometryfigure: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMGeometryFigureCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, geometryfigure: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, geometryfigure: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, geometryfigure: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, geometryfigure: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMGlyphs {
    pub base__: IXpsOMVisual,
    pub GetUnicodeString: unsafe extern "system" fn(this: *mut *mut Self, unicodestring: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetGlyphIndexCount: unsafe extern "system" fn(this: *mut *mut Self, indexcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(this: *mut *mut Self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_sys::core::HRESULT,
    pub GetGlyphMappingCount: unsafe extern "system" fn(this: *mut *mut Self, glyphmappingcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetGlyphMappings: unsafe extern "system" fn(this: *mut *mut Self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_sys::core::HRESULT,
    pub GetProhibitedCaretStopCount: unsafe extern "system" fn(this: *mut *mut Self, prohibitedcaretstopcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetProhibitedCaretStops: unsafe extern "system" fn(this: *mut *mut Self, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetBidiLevel: unsafe extern "system" fn(this: *mut *mut Self, bidilevel: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsSideways: unsafe extern "system" fn(this: *mut *mut Self, issideways: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsSideways: usize,
    pub GetDeviceFontName: unsafe extern "system" fn(this: *mut *mut Self, devicefontname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetStyleSimulations: unsafe extern "system" fn(this: *mut *mut Self, stylesimulations: *mut XPS_STYLE_SIMULATION) -> ::windows_sys::core::HRESULT,
    pub SetStyleSimulations: unsafe extern "system" fn(this: *mut *mut Self, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows_sys::core::HRESULT,
    pub GetOrigin: unsafe extern "system" fn(this: *mut *mut Self, origin: *mut XPS_POINT) -> ::windows_sys::core::HRESULT,
    pub SetOrigin: unsafe extern "system" fn(this: *mut *mut Self, origin: *const XPS_POINT) -> ::windows_sys::core::HRESULT,
    pub GetFontRenderingEmSize: unsafe extern "system" fn(this: *mut *mut Self, fontrenderingemsize: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetFontRenderingEmSize: unsafe extern "system" fn(this: *mut *mut Self, fontrenderingemsize: f32) -> ::windows_sys::core::HRESULT,
    pub GetFontResource: unsafe extern "system" fn(this: *mut *mut Self, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFontResource: unsafe extern "system" fn(this: *mut *mut Self, fontresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFontFaceIndex: unsafe extern "system" fn(this: *mut *mut Self, fontfaceindex: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetFontFaceIndex: unsafe extern "system" fn(this: *mut *mut Self, fontfaceindex: i16) -> ::windows_sys::core::HRESULT,
    pub GetFillBrush: unsafe extern "system" fn(this: *mut *mut Self, fillbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFillBrushLocal: unsafe extern "system" fn(this: *mut *mut Self, fillbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFillBrushLocal: unsafe extern "system" fn(this: *mut *mut Self, fillbrush: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFillBrushLookup: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetFillBrushLookup: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetGlyphsEditor: unsafe extern "system" fn(this: *mut *mut Self, editor: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, glyphs: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMGlyphsEditor {
    pub base__: ::windows_sys::core::IUnknown,
    pub ApplyEdits: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetUnicodeString: unsafe extern "system" fn(this: *mut *mut Self, unicodestring: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetUnicodeString: unsafe extern "system" fn(this: *mut *mut Self, unicodestring: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetGlyphIndexCount: unsafe extern "system" fn(this: *mut *mut Self, indexcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(this: *mut *mut Self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_sys::core::HRESULT,
    pub SetGlyphIndices: unsafe extern "system" fn(this: *mut *mut Self, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows_sys::core::HRESULT,
    pub GetGlyphMappingCount: unsafe extern "system" fn(this: *mut *mut Self, glyphmappingcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetGlyphMappings: unsafe extern "system" fn(this: *mut *mut Self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_sys::core::HRESULT,
    pub SetGlyphMappings: unsafe extern "system" fn(this: *mut *mut Self, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows_sys::core::HRESULT,
    pub GetProhibitedCaretStopCount: unsafe extern "system" fn(this: *mut *mut Self, prohibitedcaretstopcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetProhibitedCaretStops: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetProhibitedCaretStops: unsafe extern "system" fn(this: *mut *mut Self, count: u32, prohibitedcaretstops: *const u32) -> ::windows_sys::core::HRESULT,
    pub GetBidiLevel: unsafe extern "system" fn(this: *mut *mut Self, bidilevel: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBidiLevel: unsafe extern "system" fn(this: *mut *mut Self, bidilevel: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsSideways: unsafe extern "system" fn(this: *mut *mut Self, issideways: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsSideways: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsSideways: unsafe extern "system" fn(this: *mut *mut Self, issideways: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsSideways: usize,
    pub GetDeviceFontName: unsafe extern "system" fn(this: *mut *mut Self, devicefontname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetDeviceFontName: unsafe extern "system" fn(this: *mut *mut Self, devicefontname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMGradientBrush {
    pub base__: IXpsOMBrush,
    pub GetGradientStops: unsafe extern "system" fn(this: *mut *mut Self, gradientstops: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSpreadMethod: unsafe extern "system" fn(this: *mut *mut Self, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows_sys::core::HRESULT,
    pub SetSpreadMethod: unsafe extern "system" fn(this: *mut *mut Self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows_sys::core::HRESULT,
    pub GetColorInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows_sys::core::HRESULT,
    pub SetColorInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMGradientStop {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: f32) -> ::windows_sys::core::HRESULT,
    pub GetColor: unsafe extern "system" fn(this: *mut *mut Self, color: *mut XPS_COLOR, colorprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, gradientstop: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMGradientStopCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, stop: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, stop: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, stop: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, stop: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMImageBrush {
    pub base__: IXpsOMTileBrush,
    pub GetImageResource: unsafe extern "system" fn(this: *mut *mut Self, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetImageResource: unsafe extern "system" fn(this: *mut *mut Self, imageresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetColorProfileResource: unsafe extern "system" fn(this: *mut *mut Self, colorprofileresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetColorProfileResource: unsafe extern "system" fn(this: *mut *mut Self, colorprofileresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, imagebrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMImageResource {
    pub base__: IXpsOMResource,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, readerstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, sourcestream: *mut ::core::ffi::c_void, imagetype: XPS_IMAGE_TYPE, partname: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
    pub GetImageType: unsafe extern "system" fn(this: *mut *mut Self, imagetype: *mut XPS_IMAGE_TYPE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMImageResourceCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut *mut Self, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[repr(C)]
pub struct IXpsOMLinearGradientBrush {
    pub base__: IXpsOMGradientBrush,
    pub GetStartPoint: unsafe extern "system" fn(this: *mut *mut Self, startpoint: *mut XPS_POINT) -> ::windows_sys::core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(this: *mut *mut Self, startpoint: *const XPS_POINT) -> ::windows_sys::core::HRESULT,
    pub GetEndPoint: unsafe extern "system" fn(this: *mut *mut Self, endpoint: *mut XPS_POINT) -> ::windows_sys::core::HRESULT,
    pub SetEndPoint: unsafe extern "system" fn(this: *mut *mut Self, endpoint: *const XPS_POINT) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, lineargradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMMatrixTransform {
    pub base__: IXpsOMShareable,
    pub GetMatrix: unsafe extern "system" fn(this: *mut *mut Self, matrix: *mut XPS_MATRIX) -> ::windows_sys::core::HRESULT,
    pub SetMatrix: unsafe extern "system" fn(this: *mut *mut Self, matrix: *const XPS_MATRIX) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMNameCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, name: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMObjectFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreatePackage: unsafe extern "system" fn(this: *mut *mut Self, package: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePackageFromFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePackageFromFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePackageFromStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePackageFromStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateStoryFragmentsResource: unsafe extern "system" fn(this: *mut *mut Self, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, storyfragmentsresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateStoryFragmentsResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocumentStructureResource: unsafe extern "system" fn(this: *mut *mut Self, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, documentstructureresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocumentStructureResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateSignatureBlockResource: unsafe extern "system" fn(this: *mut *mut Self, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateSignatureBlockResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResource: unsafe extern "system" fn(this: *mut *mut Self, dictionary: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResourceFromStream: unsafe extern "system" fn(this: *mut *mut Self, dictionarymarkupstream: *mut ::core::ffi::c_void, dictionaryparturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, dictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResourceFromStream: usize,
    pub CreatePartResources: unsafe extern "system" fn(this: *mut *mut Self, partresources: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocumentSequence: unsafe extern "system" fn(this: *mut *mut Self, parturi: *mut ::core::ffi::c_void, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocumentSequence: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocument: unsafe extern "system" fn(this: *mut *mut Self, parturi: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocument: usize,
    pub CreatePageReference: unsafe extern "system" fn(this: *mut *mut Self, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePage: unsafe extern "system" fn(this: *mut *mut Self, pagedimensions: *const XPS_SIZE, language: ::windows_sys::core::PCWSTR, parturi: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePageFromStream: unsafe extern "system" fn(this: *mut *mut Self, pagemarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePageFromStream: usize,
    pub CreateCanvas: unsafe extern "system" fn(this: *mut *mut Self, canvas: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateGlyphs: unsafe extern "system" fn(this: *mut *mut Self, fontresource: *mut ::core::ffi::c_void, glyphs: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePath: unsafe extern "system" fn(this: *mut *mut Self, path: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateGeometry: unsafe extern "system" fn(this: *mut *mut Self, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateGeometryFigure: unsafe extern "system" fn(this: *mut *mut Self, startpoint: *const XPS_POINT, figure: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut *mut Self, matrix: *const XPS_MATRIX, transform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSolidColorBrush: unsafe extern "system" fn(this: *mut *mut Self, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void, solidcolorbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateColorProfileResource: unsafe extern "system" fn(this: *mut *mut Self, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, colorprofileresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateColorProfileResource: usize,
    pub CreateImageBrush: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateVisualBrush: unsafe extern "system" fn(this: *mut *mut Self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateImageResource: unsafe extern "system" fn(this: *mut *mut Self, acquiredstream: *mut ::core::ffi::c_void, contenttype: XPS_IMAGE_TYPE, parturi: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateImageResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePrintTicketResource: unsafe extern "system" fn(this: *mut *mut Self, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePrintTicketResource: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateFontResource: unsafe extern "system" fn(this: *mut *mut Self, acquiredstream: *mut ::core::ffi::c_void, fontembedding: XPS_FONT_EMBEDDING, parturi: *mut ::core::ffi::c_void, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateFontResource: usize,
    pub CreateGradientStop: unsafe extern "system" fn(this: *mut *mut Self, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void, offset: f32, gradientstop: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLinearGradientBrush: unsafe extern "system" fn(this: *mut *mut Self, gradstop1: *mut ::core::ffi::c_void, gradstop2: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRadialGradientBrush: unsafe extern "system" fn(this: *mut *mut Self, gradstop1: *mut ::core::ffi::c_void, gradstop2: *mut ::core::ffi::c_void, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateCoreProperties: unsafe extern "system" fn(this: *mut *mut Self, parturi: *mut ::core::ffi::c_void, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateCoreProperties: usize,
    pub CreateDictionary: unsafe extern "system" fn(this: *mut *mut Self, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePartUriCollection: unsafe extern "system" fn(this: *mut *mut Self, parturicollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnStream: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePartUri: unsafe extern "system" fn(this: *mut *mut Self, uri: ::windows_sys::core::PCWSTR, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePartUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReadOnlyStreamOnFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, stream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReadOnlyStreamOnFile: usize,
}
#[repr(C)]
pub struct IXpsOMObjectFactory1 {
    pub base__: IXpsOMObjectFactory,
    pub GetDocumentTypeFromFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocumentTypeFromStream: unsafe extern "system" fn(this: *mut *mut Self, xpsdocumentstream: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocumentTypeFromStream: usize,
    pub ConvertHDPhotoToJpegXR: unsafe extern "system" fn(this: *mut *mut Self, imageresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConvertJpegXRToHDPhoto: unsafe extern "system" fn(this: *mut *mut Self, imageresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnFile1: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnFile1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnStream1: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnStream1: usize,
    pub CreatePackage1: unsafe extern "system" fn(this: *mut *mut Self, package: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePackageFromStream1: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePackageFromStream1: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePackageFromFile1: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePackageFromFile1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePage1: unsafe extern "system" fn(this: *mut *mut Self, pagedimensions: *const XPS_SIZE, language: ::windows_sys::core::PCWSTR, parturi: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePage1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePageFromStream1: unsafe extern "system" fn(this: *mut *mut Self, pagemarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePageFromStream1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResourceFromStream1: unsafe extern "system" fn(this: *mut *mut Self, dictionarymarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, dictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResourceFromStream1: usize,
}
#[repr(C)]
pub struct IXpsOMPackage {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDocumentSequence: unsafe extern "system" fn(this: *mut *mut Self, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDocumentSequence: unsafe extern "system" fn(this: *mut *mut Self, documentsequence: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCoreProperties: unsafe extern "system" fn(this: *mut *mut Self, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCoreProperties: unsafe extern "system" fn(this: *mut *mut Self, coreproperties: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetDiscardControlPartName: unsafe extern "system" fn(this: *mut *mut Self, discardcontrolparturi: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetDiscardControlPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetDiscardControlPartName: unsafe extern "system" fn(this: *mut *mut Self, discardcontrolparturi: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetDiscardControlPartName: usize,
    pub GetThumbnailResource: unsafe extern "system" fn(this: *mut *mut Self, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetThumbnailResource: unsafe extern "system" fn(this: *mut *mut Self, imageresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub WriteToFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    WriteToFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub WriteToStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    WriteToStream: usize,
}
#[repr(C)]
pub struct IXpsOMPackage1 {
    pub base__: IXpsOMPackage,
    pub GetDocumentType: unsafe extern "system" fn(this: *mut *mut Self, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub WriteToFile1: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    WriteToFile1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub WriteToStream1: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    WriteToStream1: usize,
}
#[repr(C)]
pub struct IXpsOMPackageTarget {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateXpsOMPackageWriter: unsafe extern "system" fn(this: *mut *mut Self, documentsequencepartname: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateXpsOMPackageWriter: usize,
}
#[repr(C)]
pub struct IXpsOMPackageWriter {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub StartNewDocument: unsafe extern "system" fn(this: *mut *mut Self, documentpartname: *mut ::core::ffi::c_void, documentprintticket: *mut ::core::ffi::c_void, documentstructure: *mut ::core::ffi::c_void, signatureblockresources: *mut ::core::ffi::c_void, restrictedfonts: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    StartNewDocument: usize,
    pub AddPage: unsafe extern "system" fn(this: *mut *mut Self, page: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: *mut ::core::ffi::c_void, storyfragments: *mut ::core::ffi::c_void, pageprintticket: *mut ::core::ffi::c_void, pagethumbnail: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddResource: unsafe extern "system" fn(this: *mut *mut Self, resource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsClosed: unsafe extern "system" fn(this: *mut *mut Self, isclosed: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsClosed: usize,
}
#[repr(C)]
pub struct IXpsOMPackageWriter3D {
    pub base__: IXpsOMPackageWriter,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub AddModelTexture: unsafe extern "system" fn(this: *mut *mut Self, texturepartname: *mut ::core::ffi::c_void, texturedata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    AddModelTexture: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetModelPrintTicket: unsafe extern "system" fn(this: *mut *mut Self, printticketpartname: *mut ::core::ffi::c_void, printticketdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetModelPrintTicket: usize,
}
#[repr(C)]
pub struct IXpsOMPage {
    pub base__: IXpsOMPart,
    pub GetOwner: unsafe extern "system" fn(this: *mut *mut Self, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetVisuals: unsafe extern "system" fn(this: *mut *mut Self, visuals: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPageDimensions: unsafe extern "system" fn(this: *mut *mut Self, pagedimensions: *mut XPS_SIZE) -> ::windows_sys::core::HRESULT,
    pub SetPageDimensions: unsafe extern "system" fn(this: *mut *mut Self, pagedimensions: *const XPS_SIZE) -> ::windows_sys::core::HRESULT,
    pub GetContentBox: unsafe extern "system" fn(this: *mut *mut Self, contentbox: *mut XPS_RECT) -> ::windows_sys::core::HRESULT,
    pub SetContentBox: unsafe extern "system" fn(this: *mut *mut Self, contentbox: *const XPS_RECT) -> ::windows_sys::core::HRESULT,
    pub GetBleedBox: unsafe extern "system" fn(this: *mut *mut Self, bleedbox: *mut XPS_RECT) -> ::windows_sys::core::HRESULT,
    pub SetBleedBox: unsafe extern "system" fn(this: *mut *mut Self, bleedbox: *const XPS_RECT) -> ::windows_sys::core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(this: *mut *mut Self, language: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut *mut Self, language: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut *mut Self, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsHyperlinkTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut *mut Self, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsHyperlinkTarget: usize,
    pub GetDictionary: unsafe extern "system" fn(this: *mut *mut Self, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDictionaryLocal: unsafe extern "system" fn(this: *mut *mut Self, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDictionaryLocal: unsafe extern "system" fn(this: *mut *mut Self, resourcedictionary: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDictionaryResource: unsafe extern "system" fn(this: *mut *mut Self, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDictionaryResource: unsafe extern "system" fn(this: *mut *mut Self, remotedictionaryresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Write: usize,
    pub GenerateUnusedLookupKey: unsafe extern "system" fn(this: *mut *mut Self, r#type: XPS_OBJECT_TYPE, key: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, page: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMPage1 {
    pub base__: IXpsOMPage,
    pub GetDocumentType: unsafe extern "system" fn(this: *mut *mut Self, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Write1: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Write1: usize,
}
#[repr(C)]
pub struct IXpsOMPageReference {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOwner: unsafe extern "system" fn(this: *mut *mut Self, document: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPage: unsafe extern "system" fn(this: *mut *mut Self, page: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPage: unsafe extern "system" fn(this: *mut *mut Self, page: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DiscardPage: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPageLoaded: unsafe extern "system" fn(this: *mut *mut Self, ispageloaded: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPageLoaded: usize,
    pub GetAdvisoryPageDimensions: unsafe extern "system" fn(this: *mut *mut Self, pagedimensions: *mut XPS_SIZE) -> ::windows_sys::core::HRESULT,
    pub SetAdvisoryPageDimensions: unsafe extern "system" fn(this: *mut *mut Self, pagedimensions: *const XPS_SIZE) -> ::windows_sys::core::HRESULT,
    pub GetStoryFragmentsResource: unsafe extern "system" fn(this: *mut *mut Self, storyfragmentsresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetStoryFragmentsResource: unsafe extern "system" fn(this: *mut *mut Self, storyfragmentsresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(this: *mut *mut Self, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(this: *mut *mut Self, printticketresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetThumbnailResource: unsafe extern "system" fn(this: *mut *mut Self, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetThumbnailResource: unsafe extern "system" fn(this: *mut *mut Self, imageresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CollectLinkTargets: unsafe extern "system" fn(this: *mut *mut Self, linktargets: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CollectPartResources: unsafe extern "system" fn(this: *mut *mut Self, partresources: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasRestrictedFonts: unsafe extern "system" fn(this: *mut *mut Self, restrictedfonts: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasRestrictedFonts: usize,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMPageReferenceCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, pagereference: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, pagereference: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, pagereference: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMPart {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetPartName: unsafe extern "system" fn(this: *mut *mut Self, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetPartName: unsafe extern "system" fn(this: *mut *mut Self, parturi: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetPartName: usize,
}
#[repr(C)]
pub struct IXpsOMPartResources {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFontResources: unsafe extern "system" fn(this: *mut *mut Self, fontresources: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetImageResources: unsafe extern "system" fn(this: *mut *mut Self, imageresources: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetColorProfileResources: unsafe extern "system" fn(this: *mut *mut Self, colorprofileresources: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRemoteDictionaryResources: unsafe extern "system" fn(this: *mut *mut Self, dictionaryresources: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMPartUriCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, parturi: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    InsertAt: usize,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, parturi: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetAt: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, parturi: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    Append: usize,
}
#[repr(C)]
pub struct IXpsOMPath {
    pub base__: IXpsOMVisual,
    pub GetGeometry: unsafe extern "system" fn(this: *mut *mut Self, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetGeometryLocal: unsafe extern "system" fn(this: *mut *mut Self, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGeometryLocal: unsafe extern "system" fn(this: *mut *mut Self, geometry: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetGeometryLookup: unsafe extern "system" fn(this: *mut *mut Self, lookup: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetGeometryLookup: unsafe extern "system" fn(this: *mut *mut Self, lookup: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut *mut Self, shortdescription: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut *mut Self, shortdescription: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut *mut Self, longdescription: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut *mut Self, longdescription: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSnapsToPixels: unsafe extern "system" fn(this: *mut *mut Self, snapstopixels: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSnapsToPixels: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSnapsToPixels: unsafe extern "system" fn(this: *mut *mut Self, snapstopixels: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSnapsToPixels: usize,
    pub GetStrokeBrush: unsafe extern "system" fn(this: *mut *mut Self, brush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStrokeBrushLocal: unsafe extern "system" fn(this: *mut *mut Self, brush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetStrokeBrushLocal: unsafe extern "system" fn(this: *mut *mut Self, brush: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStrokeBrushLookup: unsafe extern "system" fn(this: *mut *mut Self, lookup: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetStrokeBrushLookup: unsafe extern "system" fn(this: *mut *mut Self, lookup: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetStrokeDashes: unsafe extern "system" fn(this: *mut *mut Self, strokedashes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStrokeDashCap: unsafe extern "system" fn(this: *mut *mut Self, strokedashcap: *mut XPS_DASH_CAP) -> ::windows_sys::core::HRESULT,
    pub SetStrokeDashCap: unsafe extern "system" fn(this: *mut *mut Self, strokedashcap: XPS_DASH_CAP) -> ::windows_sys::core::HRESULT,
    pub GetStrokeDashOffset: unsafe extern "system" fn(this: *mut *mut Self, strokedashoffset: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetStrokeDashOffset: unsafe extern "system" fn(this: *mut *mut Self, strokedashoffset: f32) -> ::windows_sys::core::HRESULT,
    pub GetStrokeStartLineCap: unsafe extern "system" fn(this: *mut *mut Self, strokestartlinecap: *mut XPS_LINE_CAP) -> ::windows_sys::core::HRESULT,
    pub SetStrokeStartLineCap: unsafe extern "system" fn(this: *mut *mut Self, strokestartlinecap: XPS_LINE_CAP) -> ::windows_sys::core::HRESULT,
    pub GetStrokeEndLineCap: unsafe extern "system" fn(this: *mut *mut Self, strokeendlinecap: *mut XPS_LINE_CAP) -> ::windows_sys::core::HRESULT,
    pub SetStrokeEndLineCap: unsafe extern "system" fn(this: *mut *mut Self, strokeendlinecap: XPS_LINE_CAP) -> ::windows_sys::core::HRESULT,
    pub GetStrokeLineJoin: unsafe extern "system" fn(this: *mut *mut Self, strokelinejoin: *mut XPS_LINE_JOIN) -> ::windows_sys::core::HRESULT,
    pub SetStrokeLineJoin: unsafe extern "system" fn(this: *mut *mut Self, strokelinejoin: XPS_LINE_JOIN) -> ::windows_sys::core::HRESULT,
    pub GetStrokeMiterLimit: unsafe extern "system" fn(this: *mut *mut Self, strokemiterlimit: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetStrokeMiterLimit: unsafe extern "system" fn(this: *mut *mut Self, strokemiterlimit: f32) -> ::windows_sys::core::HRESULT,
    pub GetStrokeThickness: unsafe extern "system" fn(this: *mut *mut Self, strokethickness: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetStrokeThickness: unsafe extern "system" fn(this: *mut *mut Self, strokethickness: f32) -> ::windows_sys::core::HRESULT,
    pub GetFillBrush: unsafe extern "system" fn(this: *mut *mut Self, brush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFillBrushLocal: unsafe extern "system" fn(this: *mut *mut Self, brush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFillBrushLocal: unsafe extern "system" fn(this: *mut *mut Self, brush: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFillBrushLookup: unsafe extern "system" fn(this: *mut *mut Self, lookup: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetFillBrushLookup: unsafe extern "system" fn(this: *mut *mut Self, lookup: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, path: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMPrintTicketResource {
    pub base__: IXpsOMResource,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[repr(C)]
pub struct IXpsOMRadialGradientBrush {
    pub base__: IXpsOMGradientBrush,
    pub GetCenter: unsafe extern "system" fn(this: *mut *mut Self, center: *mut XPS_POINT) -> ::windows_sys::core::HRESULT,
    pub SetCenter: unsafe extern "system" fn(this: *mut *mut Self, center: *const XPS_POINT) -> ::windows_sys::core::HRESULT,
    pub GetRadiiSizes: unsafe extern "system" fn(this: *mut *mut Self, radiisizes: *mut XPS_SIZE) -> ::windows_sys::core::HRESULT,
    pub SetRadiiSizes: unsafe extern "system" fn(this: *mut *mut Self, radiisizes: *const XPS_SIZE) -> ::windows_sys::core::HRESULT,
    pub GetGradientOrigin: unsafe extern "system" fn(this: *mut *mut Self, origin: *mut XPS_POINT) -> ::windows_sys::core::HRESULT,
    pub SetGradientOrigin: unsafe extern "system" fn(this: *mut *mut Self, origin: *const XPS_POINT) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, radialgradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMRemoteDictionaryResource {
    pub base__: IXpsOMResource,
    pub GetDictionary: unsafe extern "system" fn(this: *mut *mut Self, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDictionary: unsafe extern "system" fn(this: *mut *mut Self, dictionary: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMRemoteDictionaryResource1 {
    pub base__: IXpsOMRemoteDictionaryResource,
    pub GetDocumentType: unsafe extern "system" fn(this: *mut *mut Self, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Write1: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write1: usize,
}
#[repr(C)]
pub struct IXpsOMRemoteDictionaryResourceCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut *mut Self, partname: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[repr(C)]
pub struct IXpsOMResource {
    pub base__: IXpsOMPart,
}
#[repr(C)]
pub struct IXpsOMShareable {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut XPS_OBJECT_TYPE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMSignatureBlockResource {
    pub base__: IXpsOMResource,
    pub GetOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[repr(C)]
pub struct IXpsOMSignatureBlockResourceCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut *mut Self, partname: *mut ::core::ffi::c_void, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[repr(C)]
pub struct IXpsOMSolidColorBrush {
    pub base__: IXpsOMBrush,
    pub GetColor: unsafe extern "system" fn(this: *mut *mut Self, color: *mut XPS_COLOR, colorprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, solidcolorbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMStoryFragmentsResource {
    pub base__: IXpsOMResource,
    pub GetOwner: unsafe extern "system" fn(this: *mut *mut Self, owner: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[repr(C)]
pub struct IXpsOMThumbnailGenerator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GenerateThumbnail: unsafe extern "system" fn(this: *mut *mut Self, page: *mut ::core::ffi::c_void, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GenerateThumbnail: usize,
}
#[repr(C)]
pub struct IXpsOMTileBrush {
    pub base__: IXpsOMBrush,
    pub GetTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetViewbox: unsafe extern "system" fn(this: *mut *mut Self, viewbox: *mut XPS_RECT) -> ::windows_sys::core::HRESULT,
    pub SetViewbox: unsafe extern "system" fn(this: *mut *mut Self, viewbox: *const XPS_RECT) -> ::windows_sys::core::HRESULT,
    pub GetViewport: unsafe extern "system" fn(this: *mut *mut Self, viewport: *mut XPS_RECT) -> ::windows_sys::core::HRESULT,
    pub SetViewport: unsafe extern "system" fn(this: *mut *mut Self, viewport: *const XPS_RECT) -> ::windows_sys::core::HRESULT,
    pub GetTileMode: unsafe extern "system" fn(this: *mut *mut Self, tilemode: *mut XPS_TILE_MODE) -> ::windows_sys::core::HRESULT,
    pub SetTileMode: unsafe extern "system" fn(this: *mut *mut Self, tilemode: XPS_TILE_MODE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMVisual {
    pub base__: IXpsOMShareable,
    pub GetTransform: unsafe extern "system" fn(this: *mut *mut Self, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut *mut Self, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut *mut Self, matrixtransform: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetClipGeometry: unsafe extern "system" fn(this: *mut *mut Self, clipgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetClipGeometryLocal: unsafe extern "system" fn(this: *mut *mut Self, clipgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetClipGeometryLocal: unsafe extern "system" fn(this: *mut *mut Self, clipgeometry: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetClipGeometryLookup: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetClipGeometryLookup: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetOpacity: unsafe extern "system" fn(this: *mut *mut Self, opacity: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut *mut Self, opacity: f32) -> ::windows_sys::core::HRESULT,
    pub GetOpacityMaskBrush: unsafe extern "system" fn(this: *mut *mut Self, opacitymaskbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOpacityMaskBrushLocal: unsafe extern "system" fn(this: *mut *mut Self, opacitymaskbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOpacityMaskBrushLocal: unsafe extern "system" fn(this: *mut *mut Self, opacitymaskbrush: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOpacityMaskBrushLookup: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetOpacityMaskBrushLookup: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut *mut Self, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsHyperlinkTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut *mut Self, ishyperlink: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsHyperlinkTarget: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetHyperlinkNavigateUri: unsafe extern "system" fn(this: *mut *mut Self, hyperlinkuri: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetHyperlinkNavigateUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHyperlinkNavigateUri: unsafe extern "system" fn(this: *mut *mut Self, hyperlinkuri: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHyperlinkNavigateUri: usize,
    pub GetLanguage: unsafe extern "system" fn(this: *mut *mut Self, language: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut *mut Self, language: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMVisualBrush {
    pub base__: IXpsOMTileBrush,
    pub GetVisual: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetVisualLocal: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetVisualLocal: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetVisualLookup: unsafe extern "system" fn(this: *mut *mut Self, lookup: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetVisualLookup: unsafe extern "system" fn(this: *mut *mut Self, lookup: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, visualbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsOMVisualCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, object: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsSignature {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut *mut Self, sigid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSignatureValue: unsafe extern "system" fn(this: *mut *mut Self, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCertificateEnumerator: unsafe extern "system" fn(this: *mut *mut Self, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCertificateEnumerator: usize,
    pub GetSigningTime: unsafe extern "system" fn(this: *mut *mut Self, sigdatetimestring: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetSigningTimeFormat: unsafe extern "system" fn(this: *mut *mut Self, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetSigningTimeFormat: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut *mut Self, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignaturePartName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Verify: unsafe extern "system" fn(this: *mut *mut Self, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Verify: usize,
    pub GetPolicy: unsafe extern "system" fn(this: *mut *mut Self, policy: *mut XPS_SIGN_POLICY) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomObjectEnumerator: unsafe extern "system" fn(this: *mut *mut Self, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomObjectEnumerator: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomReferenceEnumerator: unsafe extern "system" fn(this: *mut *mut Self, customreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomReferenceEnumerator: usize,
    pub GetSignatureXml: unsafe extern "system" fn(this: *mut *mut Self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSignatureXml: unsafe extern "system" fn(this: *mut *mut Self, signaturexml: *const u8, count: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsSignatureBlock {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRequests: unsafe extern "system" fn(this: *mut *mut Self, requests: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetPartName: unsafe extern "system" fn(this: *mut *mut Self, partname: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetPartName: usize,
    pub GetDocumentIndex: unsafe extern "system" fn(this: *mut *mut Self, fixeddocumentindex: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetDocumentName: unsafe extern "system" fn(this: *mut *mut Self, fixeddocumentname: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetDocumentName: usize,
    pub CreateRequest: unsafe extern "system" fn(this: *mut *mut Self, requestid: ::windows_sys::core::PCWSTR, signaturerequest: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsSignatureBlockCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, signatureblock: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsSignatureCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, signature: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsSignatureManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub LoadPackageFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadPackageStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadPackageStream: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Sign: unsafe extern "system" fn(this: *mut *mut Self, signoptions: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Sign: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignatureOriginPartName: unsafe extern "system" fn(this: *mut *mut Self, signatureoriginpartname: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignatureOriginPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetSignatureOriginPartName: unsafe extern "system" fn(this: *mut *mut Self, signatureoriginpartname: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetSignatureOriginPartName: usize,
    pub GetSignatures: unsafe extern "system" fn(this: *mut *mut Self, signatures: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub AddSignatureBlock: unsafe extern "system" fn(this: *mut *mut Self, partname: *mut ::core::ffi::c_void, fixeddocumentindex: u32, signatureblock: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    AddSignatureBlock: usize,
    pub GetSignatureBlocks: unsafe extern "system" fn(this: *mut *mut Self, signatureblocks: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSigningOptions: unsafe extern "system" fn(this: *mut *mut Self, signingoptions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub SavePackageToFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    SavePackageToFile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SavePackageToStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SavePackageToStream: usize,
}
#[repr(C)]
pub struct IXpsSignatureRequest {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetIntent: unsafe extern "system" fn(this: *mut *mut Self, intent: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetIntent: unsafe extern "system" fn(this: *mut *mut Self, intent: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetRequestedSigner: unsafe extern "system" fn(this: *mut *mut Self, signername: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetRequestedSigner: unsafe extern "system" fn(this: *mut *mut Self, signername: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetRequestSignByDate: unsafe extern "system" fn(this: *mut *mut Self, datestring: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetRequestSignByDate: unsafe extern "system" fn(this: *mut *mut Self, datestring: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSigningLocale: unsafe extern "system" fn(this: *mut *mut Self, place: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetSigningLocale: unsafe extern "system" fn(this: *mut *mut Self, place: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSpotLocation: unsafe extern "system" fn(this: *mut *mut Self, pageindex: *mut i32, pagepartname: *mut *mut ::core::ffi::c_void, x: *mut f32, y: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSpotLocation: usize,
    pub SetSpotLocation: unsafe extern "system" fn(this: *mut *mut Self, pageindex: i32, x: f32, y: f32) -> ::windows_sys::core::HRESULT,
    pub GetRequestId: unsafe extern "system" fn(this: *mut *mut Self, requestid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut *mut Self, signature: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsSignatureRequestCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, signaturerequest: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXpsSigningOptions {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut *mut Self, signatureid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetSignatureId: unsafe extern "system" fn(this: *mut *mut Self, signatureid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSignatureMethod: unsafe extern "system" fn(this: *mut *mut Self, signaturemethod: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetSignatureMethod: unsafe extern "system" fn(this: *mut *mut Self, signaturemethod: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDigestMethod: unsafe extern "system" fn(this: *mut *mut Self, digestmethod: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetDigestMethod: unsafe extern "system" fn(this: *mut *mut Self, digestmethod: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut *mut Self, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignaturePartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetSignaturePartName: unsafe extern "system" fn(this: *mut *mut Self, signaturepartname: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetSignaturePartName: usize,
    pub GetPolicy: unsafe extern "system" fn(this: *mut *mut Self, policy: *mut XPS_SIGN_POLICY) -> ::windows_sys::core::HRESULT,
    pub SetPolicy: unsafe extern "system" fn(this: *mut *mut Self, policy: XPS_SIGN_POLICY) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetSigningTimeFormat: unsafe extern "system" fn(this: *mut *mut Self, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetSigningTimeFormat: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub SetSigningTimeFormat: unsafe extern "system" fn(this: *mut *mut Self, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    SetSigningTimeFormat: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomObjects: unsafe extern "system" fn(this: *mut *mut Self, customobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomObjects: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomReferences: unsafe extern "system" fn(this: *mut *mut Self, customreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomReferences: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCertificateSet: unsafe extern "system" fn(this: *mut *mut Self, certificateset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCertificateSet: usize,
    pub GetFlags: unsafe extern "system" fn(this: *mut *mut Self, flags: *mut XPS_SIGN_FLAGS) -> ::windows_sys::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut *mut Self, flags: XPS_SIGN_FLAGS) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type PRINT_WINDOW_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PW_CLIENTONLY: PRINT_WINDOW_FLAGS = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct PSFEATURE_CUSTPAPER {
    pub lOrientation: i32,
    pub lWidth: i32,
    pub lHeight: i32,
    pub lWidthOffset: i32,
    pub lHeightOffset: i32,
}
impl ::core::marker::Copy for PSFEATURE_CUSTPAPER {}
impl ::core::clone::Clone for PSFEATURE_CUSTPAPER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PSFEATURE_OUTPUT {
    pub bPageIndependent: super::super::Foundation::BOOL,
    pub bSetPageDevice: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSFEATURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSFEATURE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct PSINJECTDATA {
    pub DataBytes: u32,
    pub InjectionPoint: PSINJECT_POINT,
    pub PageNumber: u16,
}
impl ::core::marker::Copy for PSINJECTDATA {}
impl ::core::clone::Clone for PSINJECTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type PSINJECT_POINT = u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINSTREAM: PSINJECT_POINT = 1u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PSADOBE: PSINJECT_POINT = 2u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGESATEND: PSINJECT_POINT = 3u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGES: PSINJECT_POINT = 4u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_DOCNEEDEDRES: PSINJECT_POINT = 5u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_DOCSUPPLIEDRES: PSINJECT_POINT = 6u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGEORDER: PSINJECT_POINT = 7u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ORIENTATION: PSINJECT_POINT = 8u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BOUNDINGBOX: PSINJECT_POINT = 9u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_DOCUMENTPROCESSCOLORS: PSINJECT_POINT = 10u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_COMMENTS: PSINJECT_POINT = 11u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINDEFAULTS: PSINJECT_POINT = 12u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDDEFAULTS: PSINJECT_POINT = 13u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINPROLOG: PSINJECT_POINT = 14u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDPROLOG: PSINJECT_POINT = 15u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINSETUP: PSINJECT_POINT = 16u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDSETUP: PSINJECT_POINT = 17u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_TRAILER: PSINJECT_POINT = 18u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_EOF: PSINJECT_POINT = 19u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDSTREAM: PSINJECT_POINT = 20u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_DOCUMENTPROCESSCOLORSATEND: PSINJECT_POINT = 21u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGENUMBER: PSINJECT_POINT = 100u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINPAGESETUP: PSINJECT_POINT = 101u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDPAGESETUP: PSINJECT_POINT = 102u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGETRAILER: PSINJECT_POINT = 103u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PLATECOLOR: PSINJECT_POINT = 104u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_SHOWPAGE: PSINJECT_POINT = 105u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGEBBOX: PSINJECT_POINT = 106u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDPAGECOMMENTS: PSINJECT_POINT = 107u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_VMSAVE: PSINJECT_POINT = 200u16;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_VMRESTORE: PSINJECT_POINT = 201u16;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_COLOR {
    pub colorType: XPS_COLOR_TYPE,
    pub value: XPS_COLOR_0,
}
impl ::core::marker::Copy for XPS_COLOR {}
impl ::core::clone::Clone for XPS_COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub union XPS_COLOR_0 {
    pub sRGB: XPS_COLOR_0_1,
    pub scRGB: XPS_COLOR_0_2,
    pub context: XPS_COLOR_0_0,
}
impl ::core::marker::Copy for XPS_COLOR_0 {}
impl ::core::clone::Clone for XPS_COLOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_COLOR_0_0 {
    pub channelCount: u8,
    pub channels: [f32; 9],
}
impl ::core::marker::Copy for XPS_COLOR_0_0 {}
impl ::core::clone::Clone for XPS_COLOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_COLOR_0_1 {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl ::core::marker::Copy for XPS_COLOR_0_1 {}
impl ::core::clone::Clone for XPS_COLOR_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_COLOR_0_2 {
    pub alpha: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl ::core::marker::Copy for XPS_COLOR_0_2 {}
impl ::core::clone::Clone for XPS_COLOR_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_COLOR_INTERPOLATION = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_INTERPOLATION_SCRGBLINEAR: XPS_COLOR_INTERPOLATION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_INTERPOLATION_SRGBLINEAR: XPS_COLOR_INTERPOLATION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_COLOR_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_TYPE_SRGB: XPS_COLOR_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_TYPE_SCRGB: XPS_COLOR_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_TYPE_CONTEXT: XPS_COLOR_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_DASH {
    pub length: f32,
    pub gap: f32,
}
impl ::core::marker::Copy for XPS_DASH {}
impl ::core::clone::Clone for XPS_DASH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_DASH_CAP = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DASH_CAP_FLAT: XPS_DASH_CAP = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DASH_CAP_ROUND: XPS_DASH_CAP = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DASH_CAP_SQUARE: XPS_DASH_CAP = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DASH_CAP_TRIANGLE: XPS_DASH_CAP = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_DOCUMENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DOCUMENT_TYPE_UNSPECIFIED: XPS_DOCUMENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DOCUMENT_TYPE_XPS: XPS_DOCUMENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DOCUMENT_TYPE_OPENXPS: XPS_DOCUMENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_ABSOLUTE_REFERENCE: ::windows_sys::core::HRESULT = -2142108159i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_ALREADY_OWNED: ::windows_sys::core::HRESULT = -2142108413i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_BLEED_BOX_PAGE_DIMENSIONS_NOT_IN_SYNC: ::windows_sys::core::HRESULT = -2142108407i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_BOTH_PATHFIGURE_AND_ABBR_SYNTAX_PRESENT: ::windows_sys::core::HRESULT = -2142108409i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_BOTH_RESOURCE_AND_SOURCEATTR_PRESENT: ::windows_sys::core::HRESULT = -2142108408i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_CARET_OUTSIDE_STRING: ::windows_sys::core::HRESULT = -2142108923i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_CARET_OUT_OF_ORDER: ::windows_sys::core::HRESULT = -2142108922i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_COLOR_COMPONENT_OUT_OF_RANGE: ::windows_sys::core::HRESULT = -2142108410i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_DICTIONARY_ITEM_NAMED: ::windows_sys::core::HRESULT = -2142108671i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_DUPLICATE_NAMES: ::windows_sys::core::HRESULT = -2142109175i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_DUPLICATE_RESOURCE_KEYS: ::windows_sys::core::HRESULT = -2142109184i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INDEX_OUT_OF_RANGE: ::windows_sys::core::HRESULT = -2142108416i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_BLEED_BOX: ::windows_sys::core::HRESULT = -2142109692i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_CONTENT_BOX: ::windows_sys::core::HRESULT = -2142109685i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_CONTENT_TYPE: ::windows_sys::core::HRESULT = -2142109682i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_FLOAT: ::windows_sys::core::HRESULT = -2142109689i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_FONT_URI: ::windows_sys::core::HRESULT = -2142109686i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_LANGUAGE: ::windows_sys::core::HRESULT = -2142109696i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_LOOKUP_TYPE: ::windows_sys::core::HRESULT = -2142109690i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_MARKUP: ::windows_sys::core::HRESULT = -2142109684i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_NAME: ::windows_sys::core::HRESULT = -2142109695i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_NUMBER_OF_COLOR_CHANNELS: ::windows_sys::core::HRESULT = -2142108158i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_NUMBER_OF_POINTS_IN_CURVE_SEGMENTS: ::windows_sys::core::HRESULT = -2142108160i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_OBFUSCATED_FONT_URI: ::windows_sys::core::HRESULT = -2142109681i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_PAGE_SIZE: ::windows_sys::core::HRESULT = -2142109693i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_RESOURCE_KEY: ::windows_sys::core::HRESULT = -2142109694i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_SIGNATUREBLOCK_MARKUP: ::windows_sys::core::HRESULT = -2142108789i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_THUMBNAIL_IMAGE_TYPE: ::windows_sys::core::HRESULT = -2142109691i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_XML_ENCODING: ::windows_sys::core::HRESULT = -2142109683i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MAPPING_OUTSIDE_INDICES: ::windows_sys::core::HRESULT = -2142108924i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MAPPING_OUTSIDE_STRING: ::windows_sys::core::HRESULT = -2142108925i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MAPPING_OUT_OF_ORDER: ::windows_sys::core::HRESULT = -2142108926i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MARKUP_COMPATIBILITY_ELEMENTS: ::windows_sys::core::HRESULT = -2142108791i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_COLORPROFILE: ::windows_sys::core::HRESULT = -2142109436i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_DISCARDCONTROL: ::windows_sys::core::HRESULT = -2142109422i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_DOCUMENT: ::windows_sys::core::HRESULT = -2142109431i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_DOCUMENTSEQUENCE_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142109432i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_FONTURI: ::windows_sys::core::HRESULT = -2142109433i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_GLYPHS: ::windows_sys::core::HRESULT = -2142109438i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_IMAGE_IN_IMAGEBRUSH: ::windows_sys::core::HRESULT = -2142109426i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_LOOKUP: ::windows_sys::core::HRESULT = -2142109439i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_NAME: ::windows_sys::core::HRESULT = -2142109440i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_PAGE_IN_DOCUMENT: ::windows_sys::core::HRESULT = -2142109428i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_PAGE_IN_PAGEREFERENCE: ::windows_sys::core::HRESULT = -2142109427i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_PART_REFERENCE: ::windows_sys::core::HRESULT = -2142109424i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_PART_STREAM: ::windows_sys::core::HRESULT = -2142109421i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_REFERRED_DOCUMENT: ::windows_sys::core::HRESULT = -2142109430i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_REFERRED_PAGE: ::windows_sys::core::HRESULT = -2142109429i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_RELATIONSHIP_TARGET: ::windows_sys::core::HRESULT = -2142109435i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_RESOURCE_KEY: ::windows_sys::core::HRESULT = -2142109425i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_RESOURCE_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142109434i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_RESTRICTED_FONT_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142109423i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_SEGMENT_DATA: ::windows_sys::core::HRESULT = -2142109437i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_DOCUMENTSEQUENCE_RELATIONSHIPS: ::windows_sys::core::HRESULT = -2142109182i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENT: ::windows_sys::core::HRESULT = -2142109178i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENTSEQUENCE: ::windows_sys::core::HRESULT = -2142109177i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_PAGE: ::windows_sys::core::HRESULT = -2142109179i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_REFERENCES_TO_PART: ::windows_sys::core::HRESULT = -2142109176i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_RESOURCES: ::windows_sys::core::HRESULT = -2142109183i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PACKAGE: ::windows_sys::core::HRESULT = -2142109180i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PAGE: ::windows_sys::core::HRESULT = -2142109181i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_NEGATIVE_FLOAT: ::windows_sys::core::HRESULT = -2142108918i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_NESTED_REMOTE_DICTIONARY: ::windows_sys::core::HRESULT = -2142108670i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_NOT_ENOUGH_GRADIENT_STOPS: ::windows_sys::core::HRESULT = -2142108405i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_NO_CUSTOM_OBJECTS: ::windows_sys::core::HRESULT = -2142108414i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_OBJECT_DETACHED: ::windows_sys::core::HRESULT = -2142108790i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_ODD_BIDILEVEL: ::windows_sys::core::HRESULT = -2142108921i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_ONE_TO_ONE_MAPPING_EXPECTED: ::windows_sys::core::HRESULT = -2142108920i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_PACKAGE_ALREADY_OPENED: ::windows_sys::core::HRESULT = -2142108793i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_PACKAGE_NOT_OPENED: ::windows_sys::core::HRESULT = -2142108794i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_PACKAGE_WRITER_NOT_CLOSED: ::windows_sys::core::HRESULT = -2142108404i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_RELATIONSHIP_EXTERNAL: ::windows_sys::core::HRESULT = -2142108406i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_RESOURCE_NOT_OWNED: ::windows_sys::core::HRESULT = -2142108412i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_RESTRICTED_FONT_NOT_OBFUSCATED: ::windows_sys::core::HRESULT = -2142108919i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_SIGNATUREID_DUP: ::windows_sys::core::HRESULT = -2142108792i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_SIGREQUESTID_DUP: ::windows_sys::core::HRESULT = -2142108795i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_STRING_TOO_LONG: ::windows_sys::core::HRESULT = -2142108928i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_TOO_MANY_INDICES: ::windows_sys::core::HRESULT = -2142108927i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNAVAILABLE_PACKAGE: ::windows_sys::core::HRESULT = -2142109420i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNEXPECTED_COLORPROFILE: ::windows_sys::core::HRESULT = -2142108411i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNEXPECTED_CONTENT_TYPE: ::windows_sys::core::HRESULT = -2142109688i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNEXPECTED_RELATIONSHIP_TYPE: ::windows_sys::core::HRESULT = -2142109680i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNEXPECTED_RESTRICTED_FONT_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142109679i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_VISUAL_CIRCULAR_REF: ::windows_sys::core::HRESULT = -2142108415i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_XKEY_ATTR_PRESENT_OUTSIDE_RES_DICT: ::windows_sys::core::HRESULT = -2142108672i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_FILL_RULE = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FILL_RULE_EVENODD: XPS_FILL_RULE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FILL_RULE_NONZERO: XPS_FILL_RULE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_FONT_EMBEDDING = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FONT_EMBEDDING_NORMAL: XPS_FONT_EMBEDDING = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FONT_EMBEDDING_OBFUSCATED: XPS_FONT_EMBEDDING = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FONT_EMBEDDING_RESTRICTED: XPS_FONT_EMBEDDING = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FONT_EMBEDDING_RESTRICTED_UNOBFUSCATED: XPS_FONT_EMBEDDING = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_GLYPH_INDEX {
    pub index: i32,
    pub advanceWidth: f32,
    pub horizontalOffset: f32,
    pub verticalOffset: f32,
}
impl ::core::marker::Copy for XPS_GLYPH_INDEX {}
impl ::core::clone::Clone for XPS_GLYPH_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_GLYPH_MAPPING {
    pub unicodeStringStart: u32,
    pub unicodeStringLength: u16,
    pub glyphIndicesStart: u32,
    pub glyphIndicesLength: u16,
}
impl ::core::marker::Copy for XPS_GLYPH_MAPPING {}
impl ::core::clone::Clone for XPS_GLYPH_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_IMAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_JPEG: XPS_IMAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_PNG: XPS_IMAGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_TIFF: XPS_IMAGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_WDP: XPS_IMAGE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_JXR: XPS_IMAGE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_INTERLEAVING = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_INTERLEAVING_OFF: XPS_INTERLEAVING = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_INTERLEAVING_ON: XPS_INTERLEAVING = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_LINE_CAP = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_CAP_FLAT: XPS_LINE_CAP = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_CAP_ROUND: XPS_LINE_CAP = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_CAP_SQUARE: XPS_LINE_CAP = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_CAP_TRIANGLE: XPS_LINE_CAP = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_LINE_JOIN = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_JOIN_MITER: XPS_LINE_JOIN = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_JOIN_BEVEL: XPS_LINE_JOIN = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_JOIN_ROUND: XPS_LINE_JOIN = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub m31: f32,
    pub m32: f32,
}
impl ::core::marker::Copy for XPS_MATRIX {}
impl ::core::clone::Clone for XPS_MATRIX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_CANVAS: XPS_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_GLYPHS: XPS_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_PATH: XPS_OBJECT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_MATRIX_TRANSFORM: XPS_OBJECT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_GEOMETRY: XPS_OBJECT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_SOLID_COLOR_BRUSH: XPS_OBJECT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_IMAGE_BRUSH: XPS_OBJECT_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_LINEAR_GRADIENT_BRUSH: XPS_OBJECT_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_RADIAL_GRADIENT_BRUSH: XPS_OBJECT_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_VISUAL_BRUSH: XPS_OBJECT_TYPE = 10i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_POINT {
    pub x: f32,
    pub y: f32,
}
impl ::core::marker::Copy for XPS_POINT {}
impl ::core::clone::Clone for XPS_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_RECT {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for XPS_RECT {}
impl ::core::clone::Clone for XPS_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_SEGMENT_STROKE_PATTERN = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_STROKE_PATTERN_ALL: XPS_SEGMENT_STROKE_PATTERN = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_STROKE_PATTERN_NONE: XPS_SEGMENT_STROKE_PATTERN = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_STROKE_PATTERN_MIXED: XPS_SEGMENT_STROKE_PATTERN = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_SEGMENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_ARC_LARGE_CLOCKWISE: XPS_SEGMENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_ARC_LARGE_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_ARC_SMALL_CLOCKWISE: XPS_SEGMENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_ARC_SMALL_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_BEZIER: XPS_SEGMENT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_LINE: XPS_SEGMENT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_QUADRATIC_BEZIER: XPS_SEGMENT_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_SIGNATURE_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_INCOMPLIANT: XPS_SIGNATURE_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_INCOMPLETE: XPS_SIGNATURE_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_BROKEN: XPS_SIGNATURE_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_QUESTIONABLE: XPS_SIGNATURE_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_VALID: XPS_SIGNATURE_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_SIGN_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_FLAGS_NONE: XPS_SIGN_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_FLAGS_IGNORE_MARKUP_COMPATIBILITY: XPS_SIGN_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_SIGN_POLICY = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_NONE: XPS_SIGN_POLICY = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_CORE_PROPERTIES: XPS_SIGN_POLICY = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_SIGNATURE_RELATIONSHIPS: XPS_SIGN_POLICY = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_PRINT_TICKET: XPS_SIGN_POLICY = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_DISCARD_CONTROL: XPS_SIGN_POLICY = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_ALL: XPS_SIGN_POLICY = 15i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_SIZE {
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for XPS_SIZE {}
impl ::core::clone::Clone for XPS_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_SPREAD_METHOD = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SPREAD_METHOD_PAD: XPS_SPREAD_METHOD = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SPREAD_METHOD_REFLECT: XPS_SPREAD_METHOD = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SPREAD_METHOD_REPEAT: XPS_SPREAD_METHOD = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_STYLE_SIMULATION = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_STYLE_SIMULATION_NONE: XPS_STYLE_SIMULATION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_STYLE_SIMULATION_ITALIC: XPS_STYLE_SIMULATION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_STYLE_SIMULATION_BOLD: XPS_STYLE_SIMULATION = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_STYLE_SIMULATION_BOLDITALIC: XPS_STYLE_SIMULATION = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_THUMBNAIL_SIZE = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_THUMBNAIL_SIZE_VERYSMALL: XPS_THUMBNAIL_SIZE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_THUMBNAIL_SIZE_SMALL: XPS_THUMBNAIL_SIZE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_THUMBNAIL_SIZE_MEDIUM: XPS_THUMBNAIL_SIZE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_THUMBNAIL_SIZE_LARGE: XPS_THUMBNAIL_SIZE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub type XPS_TILE_MODE = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_NONE: XPS_TILE_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_TILE: XPS_TILE_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_FLIPX: XPS_TILE_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_FLIPY: XPS_TILE_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_FLIPXY: XPS_TILE_MODE = 5i32;
pub const XpsOMObjectFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3916747373, data2: 15771, data3: 19783, data4: [136, 204, 56, 114, 242, 220, 53, 133] };
pub const XpsOMThumbnailGenerator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2118788066, data2: 47465, data3: 18273, data4: [190, 53, 26, 140, 237, 88, 227, 35] };
pub const XpsSignatureManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2965648160, data2: 8981, data3: 17570, data4: [183, 10, 9, 67, 161, 64, 168, 238] };
