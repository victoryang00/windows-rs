#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateXmlReader(riid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: *mut *mut super::super::super::System::Com::IMalloc) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateXmlReaderInputWithEncodingCodePage(pinputstream: *mut *mut ::windows_sys::core::IUnknown, pmalloc: *mut *mut super::super::super::System::Com::IMalloc, nencodingcodepage: u32, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: ::windows_sys::core::PCWSTR, ppinput: *mut *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateXmlReaderInputWithEncodingName(pinputstream: *mut *mut ::windows_sys::core::IUnknown, pmalloc: *mut *mut super::super::super::System::Com::IMalloc, pwszencodingname: ::windows_sys::core::PCWSTR, fencodinghint: super::super::super::Foundation::BOOL, pwszbaseuri: ::windows_sys::core::PCWSTR, ppinput: *mut *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateXmlWriter(riid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: *mut *mut super::super::super::System::Com::IMalloc) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateXmlWriterOutputWithEncodingCodePage(poutputstream: *mut *mut ::windows_sys::core::IUnknown, pmalloc: *mut *mut super::super::super::System::Com::IMalloc, nencodingcodepage: u32, ppoutput: *mut *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateXmlWriterOutputWithEncodingName(poutputstream: *mut *mut ::windows_sys::core::IUnknown, pmalloc: *mut *mut super::super::super::System::Com::IMalloc, pwszencodingname: ::windows_sys::core::PCWSTR, ppoutput: *mut *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub type DtdProcessing = i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const DtdProcessing_Prohibit: DtdProcessing = 0i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const DtdProcessing_Parse: DtdProcessing = 1i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _DtdProcessing_Last: DtdProcessing = 1i32;
#[repr(C)]
pub struct IXmlReader {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetInput: unsafe extern "system" fn(this: *mut *mut Self, pinput: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, nproperty: u32, ppvalue: *mut isize) -> ::windows_sys::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, nproperty: u32, pvalue: isize) -> ::windows_sys::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, pnodetype: *mut XmlNodeType) -> ::windows_sys::core::HRESULT,
    pub GetNodeType: unsafe extern "system" fn(this: *mut *mut Self, pnodetype: *mut XmlNodeType) -> ::windows_sys::core::HRESULT,
    pub MoveToFirstAttribute: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MoveToNextAttribute: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MoveToAttributeByName: unsafe extern "system" fn(this: *mut *mut Self, pwszlocalname: ::windows_sys::core::PCWSTR, pwsznamespaceuri: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub MoveToElement: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetQualifiedName: unsafe extern "system" fn(this: *mut *mut Self, ppwszqualifiedname: *mut ::windows_sys::core::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetNamespaceUri: unsafe extern "system" fn(this: *mut *mut Self, ppwsznamespaceuri: *mut ::windows_sys::core::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLocalName: unsafe extern "system" fn(this: *mut *mut Self, ppwszlocalname: *mut ::windows_sys::core::PWSTR, pcwchlocalname: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPrefix: unsafe extern "system" fn(this: *mut *mut Self, ppwszprefix: *mut ::windows_sys::core::PWSTR, pcwchprefix: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, ppwszvalue: *mut ::windows_sys::core::PWSTR, pcwchvalue: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ReadValueChunk: unsafe extern "system" fn(this: *mut *mut Self, pwchbuffer: ::windows_sys::core::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetBaseUri: unsafe extern "system" fn(this: *mut *mut Self, ppwszbaseuri: *mut ::windows_sys::core::PWSTR, pcwchbaseuri: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDefault: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDefault: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEmptyElement: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEmptyElement: usize,
    pub GetLineNumber: unsafe extern "system" fn(this: *mut *mut Self, pnlinenumber: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLinePosition: unsafe extern "system" fn(this: *mut *mut Self, pnlineposition: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAttributeCount: unsafe extern "system" fn(this: *mut *mut Self, pnattributecount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDepth: unsafe extern "system" fn(this: *mut *mut Self, pndepth: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEOF: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEOF: usize,
}
impl ::windows_sys::core::Interface for IXmlReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1920597121, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };
}
#[repr(C)]
pub struct IXmlResolver {
    pub base__: ::windows_sys::core::IUnknown,
    pub ResolveUri: unsafe extern "system" fn(this: *mut *mut Self, pwszbaseuri: ::windows_sys::core::PCWSTR, pwszpublicidentifier: ::windows_sys::core::PCWSTR, pwszsystemidentifier: ::windows_sys::core::PCWSTR, ppresolvedinput: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlResolver {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1920597122, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };
}
#[repr(C)]
pub struct IXmlWriter {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetOutput: unsafe extern "system" fn(this: *mut *mut Self, poutput: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, nproperty: u32, ppvalue: *mut isize) -> ::windows_sys::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, nproperty: u32, pvalue: isize) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteAttributes: unsafe extern "system" fn(this: *mut *mut Self, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteAttributes: usize,
    pub WriteAttributeString: unsafe extern "system" fn(this: *mut *mut Self, pwszprefix: ::windows_sys::core::PCWSTR, pwszlocalname: ::windows_sys::core::PCWSTR, pwsznamespaceuri: ::windows_sys::core::PCWSTR, pwszvalue: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteCData: unsafe extern "system" fn(this: *mut *mut Self, pwsztext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteCharEntity: unsafe extern "system" fn(this: *mut *mut Self, wch: u16) -> ::windows_sys::core::HRESULT,
    pub WriteChars: unsafe extern "system" fn(this: *mut *mut Self, pwch: ::windows_sys::core::PCWSTR, cwch: u32) -> ::windows_sys::core::HRESULT,
    pub WriteComment: unsafe extern "system" fn(this: *mut *mut Self, pwszcomment: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteDocType: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR, pwszpublicid: ::windows_sys::core::PCWSTR, pwszsystemid: ::windows_sys::core::PCWSTR, pwszsubset: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteElementString: unsafe extern "system" fn(this: *mut *mut Self, pwszprefix: ::windows_sys::core::PCWSTR, pwszlocalname: ::windows_sys::core::PCWSTR, pwsznamespaceuri: ::windows_sys::core::PCWSTR, pwszvalue: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteEndDocument: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WriteEndElement: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WriteEntityRef: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteFullEndElement: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WriteName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteNmToken: unsafe extern "system" fn(this: *mut *mut Self, pwsznmtoken: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteNode: unsafe extern "system" fn(this: *mut *mut Self, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteNode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteNodeShallow: unsafe extern "system" fn(this: *mut *mut Self, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteNodeShallow: usize,
    pub WriteProcessingInstruction: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR, pwsztext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteQualifiedName: unsafe extern "system" fn(this: *mut *mut Self, pwszlocalname: ::windows_sys::core::PCWSTR, pwsznamespaceuri: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteRaw: unsafe extern "system" fn(this: *mut *mut Self, pwszdata: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteRawChars: unsafe extern "system" fn(this: *mut *mut Self, pwch: ::windows_sys::core::PCWSTR, cwch: u32) -> ::windows_sys::core::HRESULT,
    pub WriteStartDocument: unsafe extern "system" fn(this: *mut *mut Self, standalone: XmlStandalone) -> ::windows_sys::core::HRESULT,
    pub WriteStartElement: unsafe extern "system" fn(this: *mut *mut Self, pwszprefix: ::windows_sys::core::PCWSTR, pwszlocalname: ::windows_sys::core::PCWSTR, pwsznamespaceuri: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteString: unsafe extern "system" fn(this: *mut *mut Self, pwsztext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteSurrogateCharEntity: unsafe extern "system" fn(this: *mut *mut Self, wchlow: u16, wchhigh: u16) -> ::windows_sys::core::HRESULT,
    pub WriteWhitespace: unsafe extern "system" fn(this: *mut *mut Self, pwszwhitespace: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlWriter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1920597128, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };
}
#[repr(C)]
pub struct IXmlWriterLite {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetOutput: unsafe extern "system" fn(this: *mut *mut Self, poutput: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, nproperty: u32, ppvalue: *mut isize) -> ::windows_sys::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, nproperty: u32, pvalue: isize) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteAttributes: unsafe extern "system" fn(this: *mut *mut Self, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteAttributes: usize,
    pub WriteAttributeString: unsafe extern "system" fn(this: *mut *mut Self, pwszqname: ::windows_sys::core::PCWSTR, cwszqname: u32, pwszvalue: ::windows_sys::core::PCWSTR, cwszvalue: u32) -> ::windows_sys::core::HRESULT,
    pub WriteCData: unsafe extern "system" fn(this: *mut *mut Self, pwsztext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteCharEntity: unsafe extern "system" fn(this: *mut *mut Self, wch: u16) -> ::windows_sys::core::HRESULT,
    pub WriteChars: unsafe extern "system" fn(this: *mut *mut Self, pwch: ::windows_sys::core::PCWSTR, cwch: u32) -> ::windows_sys::core::HRESULT,
    pub WriteComment: unsafe extern "system" fn(this: *mut *mut Self, pwszcomment: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteDocType: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR, pwszpublicid: ::windows_sys::core::PCWSTR, pwszsystemid: ::windows_sys::core::PCWSTR, pwszsubset: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteElementString: unsafe extern "system" fn(this: *mut *mut Self, pwszqname: ::windows_sys::core::PCWSTR, cwszqname: u32, pwszvalue: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteEndDocument: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WriteEndElement: unsafe extern "system" fn(this: *mut *mut Self, pwszqname: ::windows_sys::core::PCWSTR, cwszqname: u32) -> ::windows_sys::core::HRESULT,
    pub WriteEntityRef: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteFullEndElement: unsafe extern "system" fn(this: *mut *mut Self, pwszqname: ::windows_sys::core::PCWSTR, cwszqname: u32) -> ::windows_sys::core::HRESULT,
    pub WriteName: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteNmToken: unsafe extern "system" fn(this: *mut *mut Self, pwsznmtoken: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteNode: unsafe extern "system" fn(this: *mut *mut Self, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteNode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteNodeShallow: unsafe extern "system" fn(this: *mut *mut Self, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteNodeShallow: usize,
    pub WriteProcessingInstruction: unsafe extern "system" fn(this: *mut *mut Self, pwszname: ::windows_sys::core::PCWSTR, pwsztext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteRaw: unsafe extern "system" fn(this: *mut *mut Self, pwszdata: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteRawChars: unsafe extern "system" fn(this: *mut *mut Self, pwch: ::windows_sys::core::PCWSTR, cwch: u32) -> ::windows_sys::core::HRESULT,
    pub WriteStartDocument: unsafe extern "system" fn(this: *mut *mut Self, standalone: XmlStandalone) -> ::windows_sys::core::HRESULT,
    pub WriteStartElement: unsafe extern "system" fn(this: *mut *mut Self, pwszqname: ::windows_sys::core::PCWSTR, cwszqname: u32) -> ::windows_sys::core::HRESULT,
    pub WriteString: unsafe extern "system" fn(this: *mut *mut Self, pwsztext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WriteSurrogateCharEntity: unsafe extern "system" fn(this: *mut *mut Self, wchlow: u16, wchhigh: u16) -> ::windows_sys::core::HRESULT,
    pub WriteWhitespace: unsafe extern "system" fn(this: *mut *mut Self, pwszwhitespace: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlWriterLite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2250544326, data2: 4880, data3: 19117, data4: [179, 205, 45, 190, 235, 246, 112, 211] };
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub type XmlConformanceLevel = i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlConformanceLevel_Auto: XmlConformanceLevel = 0i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlConformanceLevel_Fragment: XmlConformanceLevel = 1i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlConformanceLevel_Document: XmlConformanceLevel = 2i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlConformanceLevel_Last: XmlConformanceLevel = 2i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub type XmlError = i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_MX: XmlError = -1072894464i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_INPUTEND: XmlError = -1072894463i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_ENCODING: XmlError = -1072894462i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_ENCODINGSWITCH: XmlError = -1072894461i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_ENCODINGSIGNATURE: XmlError = -1072894460i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_WC: XmlError = -1072894432i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_WHITESPACE: XmlError = -1072894431i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_SEMICOLON: XmlError = -1072894430i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_GREATERTHAN: XmlError = -1072894429i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_QUOTE: XmlError = -1072894428i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_EQUAL: XmlError = -1072894427i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_LESSTHAN: XmlError = -1072894426i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_HEXDIGIT: XmlError = -1072894425i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DIGIT: XmlError = -1072894424i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_LEFTBRACKET: XmlError = -1072894423i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_LEFTPAREN: XmlError = -1072894422i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_XMLCHARACTER: XmlError = -1072894421i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NAMECHARACTER: XmlError = -1072894420i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_SYNTAX: XmlError = -1072894419i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_CDSECT: XmlError = -1072894418i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_COMMENT: XmlError = -1072894417i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_CONDSECT: XmlError = -1072894416i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLATTLIST: XmlError = -1072894415i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLDOCTYPE: XmlError = -1072894414i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLELEMENT: XmlError = -1072894413i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLENTITY: XmlError = -1072894412i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLNOTATION: XmlError = -1072894411i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NDATA: XmlError = -1072894410i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PUBLIC: XmlError = -1072894409i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_SYSTEM: XmlError = -1072894408i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NAME: XmlError = -1072894407i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_ROOTELEMENT: XmlError = -1072894406i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_ELEMENTMATCH: XmlError = -1072894405i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_UNIQUEATTRIBUTE: XmlError = -1072894404i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_TEXTXMLDECL: XmlError = -1072894403i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_LEADINGXML: XmlError = -1072894402i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_TEXTDECL: XmlError = -1072894401i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_XMLDECL: XmlError = -1072894400i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_ENCNAME: XmlError = -1072894399i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PUBLICID: XmlError = -1072894398i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PESINTERNALSUBSET: XmlError = -1072894397i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PESBETWEENDECLS: XmlError = -1072894396i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NORECURSION: XmlError = -1072894395i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_ENTITYCONTENT: XmlError = -1072894394i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_UNDECLAREDENTITY: XmlError = -1072894393i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PARSEDENTITY: XmlError = -1072894392i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NOEXTERNALENTITYREF: XmlError = -1072894391i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PI: XmlError = -1072894390i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_SYSTEMID: XmlError = -1072894389i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_QUESTIONMARK: XmlError = -1072894388i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_CDSECTEND: XmlError = -1072894387i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_MOREDATA: XmlError = -1072894386i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DTDPROHIBITED: XmlError = -1072894385i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_INVALIDXMLSPACE: XmlError = -1072894384i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_NC: XmlError = -1072894368i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_QNAMECHARACTER: XmlError = -1072894367i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_QNAMECOLON: XmlError = -1072894366i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_NAMECOLON: XmlError = -1072894365i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_DECLAREDPREFIX: XmlError = -1072894364i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_UNDECLAREDPREFIX: XmlError = -1072894363i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_EMPTYURI: XmlError = -1072894362i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_XMLPREFIXRESERVED: XmlError = -1072894361i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_XMLNSPREFIXRESERVED: XmlError = -1072894360i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_XMLURIRESERVED: XmlError = -1072894359i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_XMLNSURIRESERVED: XmlError = -1072894358i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const SC_E_SC: XmlError = -1072894336i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const SC_E_MAXELEMENTDEPTH: XmlError = -1072894335i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const SC_E_MAXENTITYEXPANSION: XmlError = -1072894334i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_WR: XmlError = -1072894208i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_NONWHITESPACE: XmlError = -1072894207i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_NSPREFIXDECLARED: XmlError = -1072894206i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_NSPREFIXWITHEMPTYNSURI: XmlError = -1072894205i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_DUPLICATEATTRIBUTE: XmlError = -1072894204i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_XMLNSPREFIXDECLARATION: XmlError = -1072894203i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_XMLPREFIXDECLARATION: XmlError = -1072894202i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_XMLURIDECLARATION: XmlError = -1072894201i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_XMLNSURIDECLARATION: XmlError = -1072894200i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_NAMESPACEUNDECLARED: XmlError = -1072894199i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_INVALIDXMLSPACE: XmlError = -1072894198i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_INVALIDACTION: XmlError = -1072894197i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_INVALIDSURROGATEPAIR: XmlError = -1072894196i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XML_E_INVALID_DECIMAL: XmlError = -1072898019i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XML_E_INVALID_HEXIDECIMAL: XmlError = -1072898018i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XML_E_INVALID_UNICODE: XmlError = -1072898017i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XML_E_INVALIDENCODING: XmlError = -1072897938i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub type XmlNodeType = i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_None: XmlNodeType = 0i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Element: XmlNodeType = 1i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Attribute: XmlNodeType = 2i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Text: XmlNodeType = 3i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_CDATA: XmlNodeType = 4i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_ProcessingInstruction: XmlNodeType = 7i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Comment: XmlNodeType = 8i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_DocumentType: XmlNodeType = 10i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Whitespace: XmlNodeType = 13i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_EndElement: XmlNodeType = 15i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_XmlDeclaration: XmlNodeType = 17i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlNodeType_Last: XmlNodeType = 17i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub type XmlReadState = i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_Initial: XmlReadState = 0i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_Interactive: XmlReadState = 1i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_Error: XmlReadState = 2i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_EndOfFile: XmlReadState = 3i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_Closed: XmlReadState = 4i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub type XmlReaderProperty = i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_MultiLanguage: XmlReaderProperty = 0i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_ConformanceLevel: XmlReaderProperty = 1i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_RandomAccess: XmlReaderProperty = 2i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_XmlResolver: XmlReaderProperty = 3i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_DtdProcessing: XmlReaderProperty = 4i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_ReadState: XmlReaderProperty = 5i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_MaxElementDepth: XmlReaderProperty = 6i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_MaxEntityExpansion: XmlReaderProperty = 7i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlReaderProperty_Last: XmlReaderProperty = 7i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub type XmlStandalone = i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlStandalone_Omit: XmlStandalone = 0i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlStandalone_Yes: XmlStandalone = 1i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlStandalone_No: XmlStandalone = 2i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlStandalone_Last: XmlStandalone = 2i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub type XmlWriterProperty = i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_MultiLanguage: XmlWriterProperty = 0i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_Indent: XmlWriterProperty = 1i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_ByteOrderMark: XmlWriterProperty = 2i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_OmitXmlDeclaration: XmlWriterProperty = 3i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_ConformanceLevel: XmlWriterProperty = 4i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_CompactEmptyElement: XmlWriterProperty = 5i32;
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlWriterProperty_Last: XmlWriterProperty = 5i32;
pub const _IID_IXmlReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1920597121, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };
pub const _IID_IXmlResolver: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1920597122, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };
pub const _IID_IXmlWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1920597128, data2: 28829, data3: 16533, data4: [182, 61, 105, 254, 75, 13, 144, 48] };
