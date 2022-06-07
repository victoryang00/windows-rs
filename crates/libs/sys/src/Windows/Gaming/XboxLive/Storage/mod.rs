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
impl ::windows_sys::core::Interface for IGameSaveBlobGetResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2440200672, data2: 29185, data3: 18771, data4: [170, 44, 64, 8, 240, 58, 239, 69] };
}
#[repr(C)]
pub struct IGameSaveBlobInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameSaveBlobInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2916319284, data2: 47856, data3: 17989, data4: [182, 208, 70, 237, 175, 251, 60, 43] };
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
impl ::windows_sys::core::Interface for IGameSaveBlobInfoGetResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3344401794, data2: 13975, data3: 17087, data4: [152, 156, 102, 93, 146, 59, 82, 49] };
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
impl ::windows_sys::core::Interface for IGameSaveBlobInfoQuery {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2682090674, data2: 61166, data3: 17531, data4: [169, 210, 127, 150, 192, 248, 50, 8] };
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
impl ::windows_sys::core::Interface for IGameSaveContainer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3284176777, data2: 22079, data3: 20173, data4: [156, 111, 51, 253, 14, 50, 61, 16] };
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
impl ::windows_sys::core::Interface for IGameSaveContainerInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3085071104, data2: 5469, data3: 19380, data4: [178, 186, 147, 3, 6, 243, 145, 181] };
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
impl ::windows_sys::core::Interface for IGameSaveContainerInfoGetResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4291104116, data2: 50561, data3: 20381, data4: [158, 57, 48, 161, 12, 30, 76, 80] };
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
impl ::windows_sys::core::Interface for IGameSaveContainerInfoQuery {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1016391779, data2: 28544, data3: 17191, data4: [147, 39, 255, 193, 26, 253, 66, 179] };
}
#[repr(C)]
pub struct IGameSaveOperationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameSaveErrorStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameSaveOperationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3473873413, data2: 9376, data3: 17794, data4: [154, 85, 177, 187, 187, 147, 136, 216] };
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
impl ::windows_sys::core::Interface for IGameSaveProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2426798996, data2: 33022, data3: 16913, data4: [151, 248, 165, 222, 20, 221, 149, 210] };
}
#[repr(C)]
pub struct IGameSaveProviderGetResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameSaveErrorStatus) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameSaveProviderGetResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 985204758, data2: 54163, data3: 19813, data4: [172, 22, 65, 195, 230, 122, 185, 69] };
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
impl ::windows_sys::core::Interface for IGameSaveProviderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3491577552, data2: 31491, data3: 17565, data4: [140, 189, 52, 2, 132, 42, 16, 72] };
}
