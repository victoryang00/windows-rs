#[repr(C)]
pub struct IIndexedResourceCandidate {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IndexedResourceType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Metadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metadata: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Qualifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Qualifiers: usize,
    pub ValueAsString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetQualifierValue: unsafe extern "system" fn(this: *mut *mut Self, qualifiername: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIndexedResourceQualifier {
    pub base__: ::windows_sys::core::IInspectable,
    pub QualifierName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub QualifierValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IResourceIndexer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub IndexFilePath: unsafe extern "system" fn(this: *mut *mut Self, filepath: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    IndexFilePath: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub IndexFileContentsAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    IndexFileContentsAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IResourceIndexerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateResourceIndexer: unsafe extern "system" fn(this: *mut *mut Self, projectroot: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateResourceIndexer: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IResourceIndexerFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateResourceIndexerWithExtension: unsafe extern "system" fn(this: *mut *mut Self, projectroot: *mut ::core::ffi::c_void, extensiondllpath: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateResourceIndexerWithExtension: usize,
}
pub type IndexedResourceCandidate = *mut ::core::ffi::c_void;
pub type IndexedResourceQualifier = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`*"]
#[repr(transparent)]
pub struct IndexedResourceType(pub i32);
impl IndexedResourceType {
    pub const String: Self = Self(0i32);
    pub const Path: Self = Self(1i32);
    pub const EmbeddedData: Self = Self(2i32);
}
impl ::core::marker::Copy for IndexedResourceType {}
impl ::core::clone::Clone for IndexedResourceType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ResourceIndexer = *mut ::core::ffi::c_void;
