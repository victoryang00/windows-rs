pub type GameSaveBlobGetResult = *mut ::core::ffi::c_void;
pub type GameSaveBlobInfo = *mut ::core::ffi::c_void;
pub type GameSaveBlobInfoGetResult = *mut ::core::ffi::c_void;
pub type GameSaveBlobInfoQuery = *mut ::core::ffi::c_void;
pub type GameSaveContainer = *mut ::core::ffi::c_void;
pub type GameSaveContainerInfo = *mut ::core::ffi::c_void;
pub type GameSaveContainerInfoGetResult = *mut ::core::ffi::c_void;
pub type GameSaveContainerInfoQuery = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Gaming_XboxLive_Storage\"`*"]
#[repr(transparent)]
pub struct GameSaveErrorStatus(pub i32);
impl GameSaveErrorStatus {
    pub const Ok: Self = Self(0i32);
    pub const Abort: Self = Self(-2147467260i32);
    pub const InvalidContainerName: Self = Self(-2138898431i32);
    pub const NoAccess: Self = Self(-2138898430i32);
    pub const OutOfLocalStorage: Self = Self(-2138898429i32);
    pub const UserCanceled: Self = Self(-2138898428i32);
    pub const UpdateTooBig: Self = Self(-2138898427i32);
    pub const QuotaExceeded: Self = Self(-2138898426i32);
    pub const ProvidedBufferTooSmall: Self = Self(-2138898425i32);
    pub const BlobNotFound: Self = Self(-2138898424i32);
    pub const NoXboxLiveInfo: Self = Self(-2138898423i32);
    pub const ContainerNotInSync: Self = Self(-2138898422i32);
    pub const ContainerSyncFailed: Self = Self(-2138898421i32);
    pub const UserHasNoXboxLiveInfo: Self = Self(-2138898420i32);
    pub const ObjectExpired: Self = Self(-2138898419i32);
}
impl ::core::marker::Copy for GameSaveErrorStatus {}
impl ::core::clone::Clone for GameSaveErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GameSaveOperationResult = *mut ::core::ffi::c_void;
pub type GameSaveProvider = *mut ::core::ffi::c_void;
pub type GameSaveProviderGetResult = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IGameSaveBlobGetResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameSaveErrorStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    Value: usize,
}
#[repr(C)]
pub struct IGameSaveBlobInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGameSaveBlobInfoGetResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameSaveErrorStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Value: usize,
}
#[repr(C)]
pub struct IGameSaveBlobInfoQuery {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetBlobInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetBlobInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetBlobInfoWithIndexAndMaxAsync: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, maxnumberofitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetBlobInfoWithIndexAndMaxAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetItemCountAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemCountAsync: usize,
}
#[repr(C)]
pub struct IGameSaveContainer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Provider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub SubmitUpdatesAsync: unsafe extern "system" fn(this: *mut *mut Self, blobstowrite: *mut ::core::ffi::c_void, blobstodelete: *mut ::core::ffi::c_void, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    SubmitUpdatesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub ReadAsync: unsafe extern "system" fn(this: *mut *mut Self, blobstoread: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    ReadAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsync: unsafe extern "system" fn(this: *mut *mut Self, blobstoread: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SubmitPropertySetUpdatesAsync: unsafe extern "system" fn(this: *mut *mut Self, blobstowrite: *mut ::core::ffi::c_void, blobstodelete: *mut ::core::ffi::c_void, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SubmitPropertySetUpdatesAsync: usize,
    pub CreateBlobInfoQuery: unsafe extern "system" fn(this: *mut *mut Self, blobnameprefix: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGameSaveContainerInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TotalSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastModifiedTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastModifiedTime: usize,
    pub NeedsSync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGameSaveContainerInfoGetResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameSaveErrorStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Value: usize,
}
#[repr(C)]
pub struct IGameSaveContainerInfoQuery {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetContainerInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContainerInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContainerInfoWithIndexAndMaxAsync: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, maxnumberofitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContainerInfoWithIndexAndMaxAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetItemCountAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemCountAsync: usize,
}
#[repr(C)]
pub struct IGameSaveOperationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameSaveErrorStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGameSaveProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub CreateContainer: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteContainerAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteContainerAsync: usize,
    pub CreateContainerInfoQuery: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateContainerInfoQueryWithName: unsafe extern "system" fn(this: *mut *mut Self, containernameprefix: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetRemainingBytesInQuotaAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRemainingBytesInQuotaAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ContainersChangedSinceLastSync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContainersChangedSinceLastSync: usize,
}
#[repr(C)]
pub struct IGameSaveProviderGetResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameSaveErrorStatus) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGameSaveProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, serviceconfigid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetSyncOnDemandForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, serviceconfigid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetSyncOnDemandForUserAsync: usize,
}
