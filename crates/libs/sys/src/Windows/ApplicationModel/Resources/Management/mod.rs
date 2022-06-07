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
impl ::windows_sys::core::Interface for IIndexedResourceCandidate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 241278707, data2: 64236, data3: 17428, data4: [169, 215, 84, 172, 213, 149, 63, 41] };
}
#[repr(C)]
pub struct IIndexedResourceQualifier {
    pub base__: ::windows_sys::core::IInspectable,
    pub QualifierName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub QualifierValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIndexedResourceQualifier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3672357787, data2: 54020, data3: 18815, data4: [161, 104, 163, 64, 4, 44, 138, 219] };
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
impl ::windows_sys::core::Interface for IResourceIndexer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 760019365, data2: 58159, data3: 19122, data4: [135, 72, 150, 53, 10, 1, 109, 163] };
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
impl ::windows_sys::core::Interface for IResourceIndexerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3101572873, data2: 12749, data3: 19863, data4: [189, 48, 141, 57, 247, 66, 188, 97] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IResourceIndexerFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1614868877, data2: 54757, data3: 19296, data4: [146, 1, 205, 39, 156, 188, 254, 217] };
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
