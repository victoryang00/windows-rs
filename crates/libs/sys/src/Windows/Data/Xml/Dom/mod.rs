pub type DtdEntity = *mut ::core::ffi::c_void;
pub type DtdNotation = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDtdEntity {
    pub base__: ::windows_sys::core::IInspectable,
    pub PublicId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NotationName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDtdNotation {
    pub base__: ::windows_sys::core::IInspectable,
    pub PublicId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXmlAttribute {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Specified: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXmlCDataSection {
    pub base__: ::windows_sys::core::IInspectable,
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
#[repr(C)]
pub struct IXmlComment {
    pub base__: ::windows_sys::core::IInspectable,
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
#[repr(C)]
pub struct IXmlDocumentFragment {
    pub base__: ::windows_sys::core::IInspectable,
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
#[repr(C)]
pub struct IXmlDocumentType {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Entities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Notations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXmlDomImplementation {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasFeature: unsafe extern "system" fn(this: *mut *mut Self, feature: ::windows_sys::core::HSTRING, version: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IXmlEntityReference {
    pub base__: ::windows_sys::core::IInspectable,
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
#[repr(C)]
pub struct IXmlNodeList {
    pub base__: ::windows_sys::core::IInspectable,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXmlNodeSelector {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectSingleNode: unsafe extern "system" fn(this: *mut *mut Self, xpath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectNodes: unsafe extern "system" fn(this: *mut *mut Self, xpath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectSingleNodeNS: unsafe extern "system" fn(this: *mut *mut Self, xpath: ::windows_sys::core::HSTRING, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectNodesNS: unsafe extern "system" fn(this: *mut *mut Self, xpath: ::windows_sys::core::HSTRING, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXmlNodeSerializer {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetXml: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InnerText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetInnerText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXmlProcessingInstruction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Target: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXmlText {
    pub base__: ::windows_sys::core::IInspectable,
    pub SplitText: unsafe extern "system" fn(this: *mut *mut Self, offset: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
