pub type DtdEntity = *mut ::core::ffi::c_void;
pub type DtdNotation = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDtdEntity {
    pub base__: ::windows_sys::core::IInspectable,
    pub PublicId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NotationName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtdEntity {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1779130364, data2: 25524, data3: 18447, data4: [158, 106, 138, 146, 129, 106, 173, 228] };
}
#[repr(C)]
pub struct IDtdNotation {
    pub base__: ::windows_sys::core::IInspectable,
    pub PublicId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtdNotation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2360664141, data2: 27974, data3: 20187, data4: [171, 115, 223, 131, 197, 26, 211, 151] };
}
#[repr(C)]
pub struct IXmlAttribute {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Specified: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlAttribute {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2887010980, data2: 46321, data3: 19894, data4: [178, 6, 138, 34, 195, 8, 219, 10] };
}
#[repr(C)]
pub struct IXmlCDataSection {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IXmlCDataSection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1292153967, data2: 51389, data3: 17844, data4: [136, 153, 4, 0, 215, 194, 198, 15] };
}
#[repr(C)]
pub struct IXmlCharacterData {
    pub base__: ::windows_sys::core::IInspectable,
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SubstringData: unsafe extern "system" fn(this: *mut *mut Self, offset: u32, count: u32, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AppendData: unsafe extern "system" fn(this: *mut *mut Self, data: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InsertData: unsafe extern "system" fn(this: *mut *mut Self, offset: u32, data: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DeleteData: unsafe extern "system" fn(this: *mut *mut Self, offset: u32, count: u32) -> ::windows_sys::core::HRESULT,
    pub ReplaceData: unsafe extern "system" fn(this: *mut *mut Self, offset: u32, count: u32, data: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlCharacterData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 321798827, data2: 20022, data3: 19958, data4: [177, 200, 12, 230, 47, 216, 139, 38] };
}
#[repr(C)]
pub struct IXmlComment {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IXmlComment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3164894421, data2: 46623, data3: 17937, data4: [156, 172, 46, 146, 227, 71, 109, 71] };
}
#[repr(C)]
pub struct IXmlDocument {
    pub base__: ::windows_sys::core::IInspectable,
    pub Doctype: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Implementation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateElement: unsafe extern "system" fn(this: *mut *mut Self, tagname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDocumentFragment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTextNode: unsafe extern "system" fn(this: *mut *mut Self, data: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateComment: unsafe extern "system" fn(this: *mut *mut Self, data: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateProcessingInstruction: unsafe extern "system" fn(this: *mut *mut Self, target: ::windows_sys::core::HSTRING, data: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAttribute: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateEntityReference: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(this: *mut *mut Self, tagname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateCDataSection: unsafe extern "system" fn(this: *mut *mut Self, data: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CreateAttributeNS: unsafe extern "system" fn(this: *mut *mut Self, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateElementNS: unsafe extern "system" fn(this: *mut *mut Self, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetElementById: unsafe extern "system" fn(this: *mut *mut Self, elementid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ImportNode: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void, deep: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlDocument {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4159939846, data2: 7815, data3: 17110, data4: [188, 251, 184, 200, 9, 250, 84, 148] };
}
#[repr(C)]
pub struct IXmlDocumentFragment {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IXmlDocumentFragment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3807013526, data2: 3105, data3: 17573, data4: [139, 201, 158, 74, 38, 39, 8, 236] };
}
#[repr(C)]
pub struct IXmlDocumentIO {
    pub base__: ::windows_sys::core::IInspectable,
    pub LoadXml: unsafe extern "system" fn(this: *mut *mut Self, xml: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LoadXmlWithSettings: unsafe extern "system" fn(this: *mut *mut Self, xml: ::windows_sys::core::HSTRING, loadsettings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveToFileAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveToFileAsync: usize,
}
impl ::windows_sys::core::Interface for IXmlDocumentIO {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1825630030, data2: 61029, data3: 17545, data4: [158, 191, 202, 67, 232, 123, 166, 55] };
}
#[repr(C)]
pub struct IXmlDocumentIO2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub LoadXmlFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadXmlFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LoadXmlFromBufferWithSettings: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void, loadsettings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadXmlFromBufferWithSettings: usize,
}
impl ::windows_sys::core::Interface for IXmlDocumentIO2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1560495713, data2: 31704, data3: 19157, data4: [158, 191, 129, 230, 52, 114, 99, 177] };
}
#[repr(C)]
pub struct IXmlDocumentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LoadFromUriAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadFromUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LoadFromUriWithSettingsAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, loadsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadFromUriWithSettingsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromFileAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromFileWithSettingsAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, loadsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromFileWithSettingsAsync: usize,
}
impl ::windows_sys::core::Interface for IXmlDocumentStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1430508116, data2: 55127, data3: 19321, data4: [149, 57, 35, 43, 24, 245, 11, 241] };
}
#[repr(C)]
pub struct IXmlDocumentType {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Entities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Notations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlDocumentType {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4147389477, data2: 38785, data3: 18788, data4: [142, 148, 155, 28, 109, 252, 155, 199] };
}
#[repr(C)]
pub struct IXmlDomImplementation {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasFeature: unsafe extern "system" fn(this: *mut *mut Self, feature: ::windows_sys::core::HSTRING, version: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlDomImplementation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1843757362, data2: 61725, data3: 20411, data4: [140, 198, 88, 60, 186, 147, 17, 47] };
}
#[repr(C)]
pub struct IXmlElement {
    pub base__: ::windows_sys::core::IInspectable,
    pub TagName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(this: *mut *mut Self, attributename: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(this: *mut *mut Self, attributename: ::windows_sys::core::HSTRING, attributevalue: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoveAttribute: unsafe extern "system" fn(this: *mut *mut Self, attributename: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetAttributeNode: unsafe extern "system" fn(this: *mut *mut Self, attributename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAttributeNode: unsafe extern "system" fn(this: *mut *mut Self, newattribute: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAttributeNode: unsafe extern "system" fn(this: *mut *mut Self, attributenode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(this: *mut *mut Self, tagname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAttributeNS: unsafe extern "system" fn(this: *mut *mut Self, namespaceuri: *mut ::core::ffi::c_void, qualifiedname: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetAttributeNS: unsafe extern "system" fn(this: *mut *mut Self, namespaceuri: *mut ::core::ffi::c_void, localname: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoveAttributeNS: unsafe extern "system" fn(this: *mut *mut Self, namespaceuri: *mut ::core::ffi::c_void, localname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAttributeNodeNS: unsafe extern "system" fn(this: *mut *mut Self, newattribute: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAttributeNodeNS: unsafe extern "system" fn(this: *mut *mut Self, namespaceuri: *mut ::core::ffi::c_void, localname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlElement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 771459615, data2: 27408, data3: 20216, data4: [159, 131, 239, 204, 232, 250, 236, 55] };
}
#[repr(C)]
pub struct IXmlEntityReference {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IXmlEntityReference {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 774850492, data2: 50128, data3: 19663, data4: [187, 134, 10, 184, 195, 106, 97, 207] };
}
#[repr(C)]
pub struct IXmlLoadSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxElementDepth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxElementDepth: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ProhibitDtd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetProhibitDtd: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ResolveExternals: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetResolveExternals: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ValidateOnParse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetValidateOnParse: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ElementContentWhiteSpace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetElementContentWhiteSpace: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlLoadSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1487538088, data2: 65238, data3: 18167, data4: [180, 197, 251, 27, 167, 33, 8, 214] };
}
#[repr(C)]
pub struct IXmlNamedNodeMap {
    pub base__: ::windows_sys::core::IInspectable,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNamedItem: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNamedItem: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveNamedItem: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNamedItemNS: unsafe extern "system" fn(this: *mut *mut Self, namespaceuri: *mut ::core::ffi::c_void, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveNamedItemNS: unsafe extern "system" fn(this: *mut *mut Self, namespaceuri: *mut ::core::ffi::c_void, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNamedItemNS: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlNamedNodeMap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3014041264, data2: 43696, data3: 19330, data4: [166, 250, 177, 69, 63, 124, 2, 27] };
}
#[repr(C)]
pub struct IXmlNode {
    pub base__: ::windows_sys::core::IInspectable,
    pub NodeValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNodeValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NodeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NodeType) -> ::windows_sys::core::HRESULT,
    pub NodeName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ParentNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ChildNodes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FirstChild: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LastChild: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PreviousSibling: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NextSibling: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HasChildNodes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub OwnerDocument: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertBefore: unsafe extern "system" fn(this: *mut *mut Self, newchild: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReplaceChild: unsafe extern "system" fn(this: *mut *mut Self, newchild: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(this: *mut *mut Self, childnode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppendChild: unsafe extern "system" fn(this: *mut *mut Self, newchild: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CloneNode: unsafe extern "system" fn(this: *mut *mut Self, deep: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NamespaceUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LocalName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Prefix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Normalize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetPrefix: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 477371737, data2: 8482, data3: 18389, data4: [168, 86, 131, 243, 212, 33, 72, 117] };
}
#[repr(C)]
pub struct IXmlNodeList {
    pub base__: ::windows_sys::core::IInspectable,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlNodeList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2355146103, data2: 33700, data3: 20161, data4: [156, 84, 123, 164, 41, 225, 61, 166] };
}
#[repr(C)]
pub struct IXmlNodeSelector {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectSingleNode: unsafe extern "system" fn(this: *mut *mut Self, xpath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectNodes: unsafe extern "system" fn(this: *mut *mut Self, xpath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectSingleNodeNS: unsafe extern "system" fn(this: *mut *mut Self, xpath: ::windows_sys::core::HSTRING, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectNodesNS: unsafe extern "system" fn(this: *mut *mut Self, xpath: ::windows_sys::core::HSTRING, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlNodeSelector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1675344523, data2: 53467, data3: 20449, data4: [183, 69, 249, 67, 58, 253, 194, 91] };
}
#[repr(C)]
pub struct IXmlNodeSerializer {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetXml: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InnerText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetInnerText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlNodeSerializer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1556460418, data2: 59101, data3: 18833, data4: [171, 239, 6, 216, 210, 231, 189, 12] };
}
#[repr(C)]
pub struct IXmlProcessingInstruction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Target: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlProcessingInstruction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 654834974, data2: 7826, data3: 20174, data4: [182, 244, 38, 240, 105, 7, 141, 220] };
}
#[repr(C)]
pub struct IXmlText {
    pub base__: ::windows_sys::core::IInspectable,
    pub SplitText: unsafe extern "system" fn(this: *mut *mut Self, offset: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXmlText {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4180780235, data2: 12429, data3: 18272, data4: [161, 213, 67, 182, 116, 80, 172, 126] };
}
#[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
#[repr(transparent)]
pub struct NodeType(pub i32);
impl NodeType {
    pub const Invalid: Self = Self(0i32);
    pub const ElementNode: Self = Self(1i32);
    pub const AttributeNode: Self = Self(2i32);
    pub const TextNode: Self = Self(3i32);
    pub const DataSectionNode: Self = Self(4i32);
    pub const EntityReferenceNode: Self = Self(5i32);
    pub const EntityNode: Self = Self(6i32);
    pub const ProcessingInstructionNode: Self = Self(7i32);
    pub const CommentNode: Self = Self(8i32);
    pub const DocumentNode: Self = Self(9i32);
    pub const DocumentTypeNode: Self = Self(10i32);
    pub const DocumentFragmentNode: Self = Self(11i32);
    pub const NotationNode: Self = Self(12i32);
}
impl ::core::marker::Copy for NodeType {}
impl ::core::clone::Clone for NodeType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type XmlAttribute = *mut ::core::ffi::c_void;
pub type XmlCDataSection = *mut ::core::ffi::c_void;
pub type XmlComment = *mut ::core::ffi::c_void;
pub type XmlDocument = *mut ::core::ffi::c_void;
pub type XmlDocumentFragment = *mut ::core::ffi::c_void;
pub type XmlDocumentType = *mut ::core::ffi::c_void;
pub type XmlDomImplementation = *mut ::core::ffi::c_void;
pub type XmlElement = *mut ::core::ffi::c_void;
pub type XmlEntityReference = *mut ::core::ffi::c_void;
pub type XmlLoadSettings = *mut ::core::ffi::c_void;
pub type XmlNamedNodeMap = *mut ::core::ffi::c_void;
pub type XmlNodeList = *mut ::core::ffi::c_void;
pub type XmlProcessingInstruction = *mut ::core::ffi::c_void;
pub type XmlText = *mut ::core::ffi::c_void;
