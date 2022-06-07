pub type AtomPubClient = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAtomPubClient {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub RetrieveServiceDocumentAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    RetrieveServiceDocumentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub RetrieveMediaResourceAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))]
    RetrieveMediaResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub RetrieveResourceAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    RetrieveResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub CreateResourceAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, description: ::windows_sys::core::HSTRING, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    CreateResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub CreateMediaResourceAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, mediatype: ::windows_sys::core::HSTRING, description: ::windows_sys::core::HSTRING, mediastream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))]
    CreateMediaResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication"))]
    pub UpdateMediaResourceAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, mediatype: ::windows_sys::core::HSTRING, mediastream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Syndication")))]
    UpdateMediaResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub UpdateResourceAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    UpdateResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub UpdateResourceItemAsync: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    UpdateResourceItemAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub DeleteResourceAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    DeleteResourceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Syndication"))]
    pub DeleteResourceItemAsync: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Syndication")))]
    DeleteResourceItemAsync: usize,
    pub CancelAsyncOperations: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAtomPubClient {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 892939320, data2: 52717, data3: 19788, data4: [150, 55, 5, 241, 92, 28, 148, 6] };
}
#[repr(C)]
pub struct IAtomPubClientFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub CreateAtomPubClientWithCredentials: unsafe extern "system" fn(this: *mut *mut Self, servercredential: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateAtomPubClientWithCredentials: usize,
}
impl ::windows_sys::core::Interface for IAtomPubClientFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1238716434, data2: 22475, data3: 19422, data4: [171, 159, 38, 16, 177, 114, 119, 123] };
}
#[repr(C)]
pub struct IResourceCollection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Web_Syndication")]
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    Title: usize,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Syndication"))]
    pub Categories: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Syndication")))]
    Categories: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Accepts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Accepts: usize,
}
impl ::windows_sys::core::Interface for IResourceCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2136987145, data2: 48264, data3: 16852, data4: [136, 250, 61, 230, 112, 77, 66, 142] };
}
#[repr(C)]
pub struct IServiceDocument {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Workspaces: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Workspaces: usize,
}
impl ::windows_sys::core::Interface for IServiceDocument {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2340341617, data2: 10931, data3: 19902, data4: [139, 204, 119, 143, 146, 183, 94, 81] };
}
#[repr(C)]
pub struct IWorkspace {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Web_Syndication")]
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Syndication"))]
    Title: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Collections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Collections: usize,
}
impl ::windows_sys::core::Interface for IWorkspace {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3021841979, data2: 42168, data3: 16438, data4: [137, 197, 131, 195, 18, 102, 186, 73] };
}
pub type ResourceCollection = *mut ::core::ffi::c_void;
pub type ServiceDocument = *mut ::core::ffi::c_void;
pub type Workspace = *mut ::core::ffi::c_void;
