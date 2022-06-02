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
#[repr(C)]
pub struct IResourceCandidate2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetValueAsStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetValueAsStreamAsync: usize,
}
#[repr(C)]
pub struct IResourceCandidate3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ResourceCandidateKind) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IResourceContextStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateMatchingContext: unsafe extern "system" fn(this: *mut *mut Self, result: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateMatchingContext: usize,
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
#[repr(C)]
pub struct IResourceContextStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetGlobalQualifierValueWithPersistence: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, persistence: ResourceQualifierPersistence) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IResourceContextStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI")]
    pub GetForUIContext: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetForUIContext: usize,
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
#[repr(C)]
pub struct IResourceManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsResourceReference: unsafe extern "system" fn(this: *mut *mut Self, resourcereference: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IResourceQualifier {
    pub base__: ::windows_sys::core::IInspectable,
    pub QualifierName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub QualifierValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsMatch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Score: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
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
