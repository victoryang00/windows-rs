#[repr(C)]
pub struct ISyndicationAttribute {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Namespace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNamespace: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyndicationAttribute {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1911093609, data2: 21102, data3: 16385, data4: [154, 145, 232, 79, 131, 22, 26, 177] };
}
#[repr(C)]
pub struct ISyndicationAttributeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateSyndicationAttribute: unsafe extern "system" fn(this: *mut *mut Self, attributename: ::windows_sys::core::HSTRING, attributenamespace: ::windows_sys::core::HSTRING, attributevalue: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyndicationAttributeFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1649350041, data2: 60734, data3: 16911, data4: [190, 134, 100, 4, 20, 136, 110, 75] };
}
#[repr(C)]
pub struct ISyndicationCategory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Scheme: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetScheme: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Term: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTerm: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyndicationCategory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2266325615, data2: 3258, data3: 19071, data4: [137, 255, 236, 181, 40, 20, 35, 182] };
}
#[repr(C)]
pub struct ISyndicationCategoryFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateSyndicationCategory: unsafe extern "system" fn(this: *mut *mut Self, term: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSyndicationCategoryEx: unsafe extern "system" fn(this: *mut *mut Self, term: ::windows_sys::core::HSTRING, scheme: ::windows_sys::core::HSTRING, label: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyndicationCategoryFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2873262127, data2: 18912, data3: 17701, data4: [138, 178, 171, 69, 192, 37, 40, 255] };
}
#[repr(C)]
pub struct ISyndicationClient {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub ServerCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetServerCredential: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ProxyCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetProxyCredential: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetProxyCredential: usize,
    pub MaxResponseBufferSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxResponseBufferSize: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Timeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTimeout: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub BypassCacheOnRetrieve: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetBypassCacheOnRetrieve: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RetrieveFeedAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RetrieveFeedAsync: usize,
}
impl ::windows_sys::core::Interface for ISyndicationClient {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2652416439, data2: 29257, data3: 19269, data4: [178, 41, 125, 248, 149, 165, 161, 245] };
}
#[repr(C)]
pub struct ISyndicationClientFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub CreateSyndicationClient: unsafe extern "system" fn(this: *mut *mut Self, servercredential: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateSyndicationClient: usize,
}
impl ::windows_sys::core::Interface for ISyndicationClientFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 784642860, data2: 42907, data3: 16660, data4: [178, 154, 5, 223, 251, 175, 185, 164] };
}
#[repr(C)]
pub struct ISyndicationContent {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SourceUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetSourceUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSourceUri: usize,
}
impl ::windows_sys::core::Interface for ISyndicationContent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1178730238, data2: 3669, data3: 16592, data4: [184, 208, 106, 44, 203, 169, 252, 124] };
}
#[repr(C)]
pub struct ISyndicationContentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateSyndicationContent: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, r#type: SyndicationTextType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationContentWithSourceUri: unsafe extern "system" fn(this: *mut *mut Self, sourceuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationContentWithSourceUri: usize,
}
impl ::windows_sys::core::Interface for ISyndicationContentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1026538387, data2: 38176, data3: 16755, data4: [147, 136, 126, 45, 243, 36, 168, 160] };
}
#[repr(C)]
pub struct ISyndicationErrorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, hresult: i32, result__: *mut SyndicationErrorStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyndicationErrorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 532357985, data2: 17863, data3: 18483, data4: [138, 160, 190, 95, 59, 88, 167, 244] };
}
#[repr(C)]
pub struct ISyndicationFeed {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Authors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Authors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Categories: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Categories: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Contributors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Contributors: usize,
    pub Generator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGenerator: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IconUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IconUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetIconUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIconUri: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    #[cfg(feature = "Foundation")]
    pub LastUpdatedTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdatedTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastUpdatedTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastUpdatedTime: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Links: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Links: usize,
    #[cfg(feature = "Foundation")]
    pub ImageUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetImageUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetImageUri: usize,
    pub Rights: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRights: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FirstUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FirstUri: usize,
    #[cfg(feature = "Foundation")]
    pub LastUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUri: usize,
    #[cfg(feature = "Foundation")]
    pub NextUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NextUri: usize,
    #[cfg(feature = "Foundation")]
    pub PreviousUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreviousUri: usize,
    pub SourceFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SyndicationFormat) -> ::windows_sys::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut *mut Self, feed: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub LoadFromXml: unsafe extern "system" fn(this: *mut *mut Self, feeddocument: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    LoadFromXml: usize,
}
impl ::windows_sys::core::Interface for ISyndicationFeed {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2147368146, data2: 23398, data3: 19810, data4: [132, 3, 27, 193, 13, 145, 13, 107] };
}
#[repr(C)]
pub struct ISyndicationFeedFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationFeed: unsafe extern "system" fn(this: *mut *mut Self, title: ::windows_sys::core::HSTRING, subtitle: ::windows_sys::core::HSTRING, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationFeed: usize,
}
impl ::windows_sys::core::Interface for ISyndicationFeedFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 591864370, data2: 35817, data3: 18615, data4: [137, 52, 98, 5, 19, 29, 147, 87] };
}
#[repr(C)]
pub struct ISyndicationGenerator {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyndicationGenerator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2540221305, data2: 64299, data3: 20333, data4: [180, 28, 8, 138, 88, 104, 130, 92] };
}
#[repr(C)]
pub struct ISyndicationGeneratorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateSyndicationGenerator: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyndicationGeneratorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2738914275, data2: 7718, data3: 19900, data4: [186, 157, 26, 184, 75, 239, 249, 123] };
}
#[repr(C)]
pub struct ISyndicationItem {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Authors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Authors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Categories: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Categories: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Contributors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Contributors: usize,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastUpdatedTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdatedTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastUpdatedTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastUpdatedTime: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Links: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Links: usize,
    #[cfg(feature = "Foundation")]
    pub PublishedDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PublishedDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetPublishedDate: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPublishedDate: usize,
    pub Rights: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRights: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Summary: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSummary: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CommentsUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommentsUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetCommentsUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCommentsUri: usize,
    #[cfg(feature = "Foundation")]
    pub EditUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EditUri: usize,
    #[cfg(feature = "Foundation")]
    pub EditMediaUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EditMediaUri: usize,
    pub ETag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemUri: usize,
    pub Load: unsafe extern "system" fn(this: *mut *mut Self, item: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub LoadFromXml: unsafe extern "system" fn(this: *mut *mut Self, itemdocument: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    LoadFromXml: usize,
}
impl ::windows_sys::core::Interface for ISyndicationItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1418573955, data2: 50052, data3: 17857, data4: [138, 232, 163, 120, 196, 236, 72, 108] };
}
#[repr(C)]
pub struct ISyndicationItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationItem: unsafe extern "system" fn(this: *mut *mut Self, title: ::windows_sys::core::HSTRING, content: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationItem: usize,
}
impl ::windows_sys::core::Interface for ISyndicationItemFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 622674767, data2: 32184, data3: 18554, data4: [133, 228, 16, 209, 145, 230, 110, 187] };
}
#[repr(C)]
pub struct ISyndicationLink {
    pub base__: ::windows_sys::core::IInspectable,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Relationship: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRelationship: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub ResourceLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetResourceLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyndicationLink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 659897021, data2: 41230, data3: 16821, data4: [134, 189, 151, 89, 8, 110, 176, 197] };
}
#[repr(C)]
pub struct ISyndicationLinkFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationLink: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationLink: usize,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationLinkEx: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, relationship: ::windows_sys::core::HSTRING, title: ::windows_sys::core::HSTRING, mediatype: ::windows_sys::core::HSTRING, length: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationLinkEx: usize,
}
impl ::windows_sys::core::Interface for ISyndicationLinkFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1591239636, data2: 21813, data3: 18604, data4: [152, 212, 193, 144, 153, 80, 128, 179] };
}
#[repr(C)]
pub struct ISyndicationNode {
    pub base__: ::windows_sys::core::IInspectable,
    pub NodeName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNodeName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NodeNamespace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNodeNamespace: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NodeValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNodeValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BaseUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetBaseUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBaseUri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AttributeExtensions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AttributeExtensions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ElementExtensions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ElementExtensions: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetXmlDocument: unsafe extern "system" fn(this: *mut *mut Self, format: SyndicationFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetXmlDocument: usize,
}
impl ::windows_sys::core::Interface for ISyndicationNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1966927736, data2: 20984, data3: 17856, data4: [169, 245, 241, 113, 157, 236, 63, 178] };
}
#[repr(C)]
pub struct ISyndicationNodeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateSyndicationNode: unsafe extern "system" fn(this: *mut *mut Self, nodename: ::windows_sys::core::HSTRING, nodenamespace: ::windows_sys::core::HSTRING, nodevalue: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyndicationNodeFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 311435656, data2: 19147, data3: 18856, data4: [183, 119, 165, 235, 146, 225, 138, 121] };
}
#[repr(C)]
pub struct ISyndicationPerson {
    pub base__: ::windows_sys::core::IInspectable,
    pub Email: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetEmail: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
}
impl ::windows_sys::core::Interface for ISyndicationPerson {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4196328922, data2: 42950, data3: 17687, data4: [160, 150, 1, 67, 250, 242, 147, 39] };
}
#[repr(C)]
pub struct ISyndicationPersonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateSyndicationPerson: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateSyndicationPersonEx: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, email: ::windows_sys::core::HSTRING, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSyndicationPersonEx: usize,
}
impl ::windows_sys::core::Interface for ISyndicationPersonFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3707013229, data2: 8861, data3: 19288, data4: [164, 155, 243, 210, 240, 245, 201, 159] };
}
#[repr(C)]
pub struct ISyndicationText {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Xml: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Xml: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub SetXml: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    SetXml: usize,
}
impl ::windows_sys::core::Interface for ISyndicationText {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3117178496, data2: 12602, data3: 16529, data4: [162, 166, 36, 62, 14, 233, 35, 249] };
}
#[repr(C)]
pub struct ISyndicationTextFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateSyndicationText: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSyndicationTextEx: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, r#type: SyndicationTextType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyndicationTextFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4000531191, data2: 4550, data3: 19237, data4: [171, 98, 229, 150, 189, 22, 41, 70] };
}
#[repr(C)]
#[doc = "*Required features: `\"Web_Syndication\"`*"]
pub struct RetrievalProgress {
    pub BytesRetrieved: u32,
    pub TotalBytesToRetrieve: u32,
}
impl ::core::marker::Copy for RetrievalProgress {}
impl ::core::clone::Clone for RetrievalProgress {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SyndicationAttribute = *mut ::core::ffi::c_void;
pub type SyndicationCategory = *mut ::core::ffi::c_void;
pub type SyndicationClient = *mut ::core::ffi::c_void;
pub type SyndicationContent = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationErrorStatus(pub i32);
impl SyndicationErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const MissingRequiredElement: Self = Self(1i32);
    pub const MissingRequiredAttribute: Self = Self(2i32);
    pub const InvalidXml: Self = Self(3i32);
    pub const UnexpectedContent: Self = Self(4i32);
    pub const UnsupportedFormat: Self = Self(5i32);
}
impl ::core::marker::Copy for SyndicationErrorStatus {}
impl ::core::clone::Clone for SyndicationErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SyndicationFeed = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationFormat(pub i32);
impl SyndicationFormat {
    pub const Atom10: Self = Self(0i32);
    pub const Rss20: Self = Self(1i32);
    pub const Rss10: Self = Self(2i32);
    pub const Rss092: Self = Self(3i32);
    pub const Rss091: Self = Self(4i32);
    pub const Atom03: Self = Self(5i32);
}
impl ::core::marker::Copy for SyndicationFormat {}
impl ::core::clone::Clone for SyndicationFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SyndicationGenerator = *mut ::core::ffi::c_void;
pub type SyndicationItem = *mut ::core::ffi::c_void;
pub type SyndicationLink = *mut ::core::ffi::c_void;
pub type SyndicationNode = *mut ::core::ffi::c_void;
pub type SyndicationPerson = *mut ::core::ffi::c_void;
pub type SyndicationText = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Web_Syndication\"`*"]
#[repr(transparent)]
pub struct SyndicationTextType(pub i32);
impl SyndicationTextType {
    pub const Text: Self = Self(0i32);
    pub const Html: Self = Self(1i32);
    pub const Xhtml: Self = Self(2i32);
}
impl ::core::marker::Copy for SyndicationTextType {}
impl ::core::clone::Clone for SyndicationTextType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Web_Syndication\"`*"]
pub struct TransferProgress {
    pub BytesSent: u32,
    pub TotalBytesToSend: u32,
    pub BytesRetrieved: u32,
    pub TotalBytesToRetrieve: u32,
}
impl ::core::marker::Copy for TransferProgress {}
impl ::core::clone::Clone for TransferProgress {
    fn clone(&self) -> Self {
        *self
    }
}
