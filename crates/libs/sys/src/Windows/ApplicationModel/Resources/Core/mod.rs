#[repr(C)]
pub struct INamedResource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Candidates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Candidates: usize,
    pub Resolve: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ResolveForContext: unsafe extern "system" fn(this: *mut *mut Self, resourcecontext: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ResolveAll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResolveAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResolveAllForContext: unsafe extern "system" fn(this: *mut *mut Self, resourcecontext: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResolveAllForContext: usize,
}
impl ::windows_sys::core::Interface for INamedResource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 479773209, data2: 2835, data3: 16960, data4: [137, 165, 212, 149, 220, 24, 154, 0] };
}
#[repr(C)]
pub struct IResourceCandidate {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Qualifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Qualifiers: usize,
    pub IsMatch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsMatchAsDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ValueAsString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetValueAsFileAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetValueAsFileAsync: usize,
    pub GetQualifierValue: unsafe extern "system" fn(this: *mut *mut Self, qualifiername: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceCandidate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2941388761, data2: 50227, data3: 18276, data4: [179, 253, 143, 166, 191, 188, 186, 220] };
}
#[repr(C)]
pub struct IResourceCandidate2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetValueAsStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetValueAsStreamAsync: usize,
}
impl ::windows_sys::core::Interface for IResourceCandidate2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1776661608, data2: 63228, data3: 16403, data4: [170, 162, 213, 63, 23, 87, 211, 181] };
}
#[repr(C)]
pub struct IResourceCandidate3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ResourceCandidateKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceCandidate3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 145659896, data2: 20858, data3: 18036, data4: [149, 140, 74, 60, 124, 210, 204, 107] };
}
#[repr(C)]
pub struct IResourceContext {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub QualifierValues: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QualifierValues: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetQualifierValues: unsafe extern "system" fn(this: *mut *mut Self, qualifiernames: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetQualifierValues: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OverrideToMatch: unsafe extern "system" fn(this: *mut *mut Self, result: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OverrideToMatch: usize,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetLanguages: unsafe extern "system" fn(this: *mut *mut Self, languages: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetLanguages: usize,
}
impl ::windows_sys::core::Interface for IResourceContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 799158091, data2: 28798, data3: 19239, data4: [173, 13, 208, 216, 205, 70, 143, 210] };
}
#[repr(C)]
pub struct IResourceContextStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateMatchingContext: unsafe extern "system" fn(this: *mut *mut Self, result: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateMatchingContext: usize,
}
impl ::windows_sys::core::Interface for IResourceContextStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2562628972, data2: 25400, data3: 19249, data4: [153, 223, 178, 180, 66, 241, 113, 73] };
}
#[repr(C)]
pub struct IResourceContextStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGlobalQualifierValue: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ResetGlobalQualifierValues: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetGlobalQualifierValuesForSpecifiedQualifiers: unsafe extern "system" fn(this: *mut *mut Self, qualifiernames: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetGlobalQualifierValuesForSpecifiedQualifiers: usize,
    pub GetForViewIndependentUse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceContextStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1106727663, data2: 4783, data3: 16825, data4: [171, 54, 177, 235, 75, 81, 36, 96] };
}
#[repr(C)]
pub struct IResourceContextStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetGlobalQualifierValueWithPersistence: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, persistence: ResourceQualifierPersistence) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceContextStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 550455596, data2: 44815, data3: 17675, data4: [157, 166, 16, 109, 208, 194, 154, 57] };
}
#[repr(C)]
pub struct IResourceContextStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI")]
    pub GetForUIContext: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetForUIContext: usize,
}
impl ::windows_sys::core::Interface for IResourceContextStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 585866445, data2: 64305, data3: 19450, data4: [184, 107, 223, 157, 157, 123, 220, 57] };
}
#[repr(C)]
pub struct IResourceManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub MainResourceMap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllResourceMaps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllResourceMaps: usize,
    pub DefaultContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub LoadPriFiles: unsafe extern "system" fn(this: *mut *mut Self, files: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    LoadPriFiles: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub UnloadPriFiles: unsafe extern "system" fn(this: *mut *mut Self, files: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    UnloadPriFiles: usize,
}
impl ::windows_sys::core::Interface for IResourceManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4148484475, data2: 39304, data3: 17659, data4: [171, 214, 83, 120, 132, 76, 250, 139] };
}
#[repr(C)]
pub struct IResourceManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllNamedResourcesForPackage: unsafe extern "system" fn(this: *mut *mut Self, packagename: ::windows_sys::core::HSTRING, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllNamedResourcesForPackage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllSubtreesForPackage: unsafe extern "system" fn(this: *mut *mut Self, packagename: ::windows_sys::core::HSTRING, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllSubtreesForPackage: usize,
}
impl ::windows_sys::core::Interface for IResourceManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2640772716, data2: 42199, data3: 19491, data4: [158, 133, 103, 95, 48, 76, 37, 45] };
}
#[repr(C)]
pub struct IResourceManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsResourceReference: unsafe extern "system" fn(this: *mut *mut Self, resourcereference: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 482409980, data2: 27118, data3: 20035, data4: [153, 1, 71, 241, 38, 135, 186, 247] };
}
#[repr(C)]
pub struct IResourceMap {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, resource: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetValueForContext: unsafe extern "system" fn(this: *mut *mut Self, resource: ::windows_sys::core::HSTRING, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSubtree: unsafe extern "system" fn(this: *mut *mut Self, reference: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceMap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1915242532, data2: 56204, data3: 17144, data4: [176, 140, 83, 255, 53, 125, 173, 130] };
}
#[repr(C)]
pub struct IResourceQualifier {
    pub base__: ::windows_sys::core::IInspectable,
    pub QualifierName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub QualifierValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsMatch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Score: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceQualifier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2019403186, data2: 19197, data3: 17270, data4: [168, 136, 197, 249, 166, 183, 160, 92] };
}
pub type NamedResource = *mut ::core::ffi::c_void;
pub type ResourceCandidate = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Resources_Core\"`*"]
#[repr(transparent)]
pub struct ResourceCandidateKind(pub i32);
impl ResourceCandidateKind {
    pub const String: Self = Self(0i32);
    pub const File: Self = Self(1i32);
    pub const EmbeddedData: Self = Self(2i32);
}
impl ::core::marker::Copy for ResourceCandidateKind {}
impl ::core::clone::Clone for ResourceCandidateKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ResourceCandidateVectorView = *mut ::core::ffi::c_void;
pub type ResourceContext = *mut ::core::ffi::c_void;
pub type ResourceContextLanguagesVectorView = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"ApplicationModel_Resources_Core\"`*"]
pub struct ResourceLayoutInfo {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ResourceSubtreeCount: u32,
    pub NamedResourceCount: u32,
    pub Checksum: i32,
}
impl ::core::marker::Copy for ResourceLayoutInfo {}
impl ::core::clone::Clone for ResourceLayoutInfo {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ResourceManager = *mut ::core::ffi::c_void;
pub type ResourceMap = *mut ::core::ffi::c_void;
pub type ResourceMapIterator = *mut ::core::ffi::c_void;
pub type ResourceMapMapView = *mut ::core::ffi::c_void;
pub type ResourceMapMapViewIterator = *mut ::core::ffi::c_void;
pub type ResourceQualifier = *mut ::core::ffi::c_void;
pub type ResourceQualifierMapView = *mut ::core::ffi::c_void;
pub type ResourceQualifierObservableMap = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Resources_Core\"`*"]
#[repr(transparent)]
pub struct ResourceQualifierPersistence(pub i32);
impl ResourceQualifierPersistence {
    pub const None: Self = Self(0i32);
    pub const LocalMachine: Self = Self(1i32);
}
impl ::core::marker::Copy for ResourceQualifierPersistence {}
impl ::core::clone::Clone for ResourceQualifierPersistence {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ResourceQualifierVectorView = *mut ::core::ffi::c_void;
