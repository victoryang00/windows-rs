#[cfg(feature = "Storage_AccessCache")]
pub mod AccessCache;
#[cfg(feature = "Storage_BulkAccess")]
pub mod BulkAccess;
#[cfg(feature = "Storage_Compression")]
pub mod Compression;
#[cfg(feature = "Storage_FileProperties")]
pub mod FileProperties;
#[cfg(feature = "Storage_Pickers")]
pub mod Pickers;
#[cfg(feature = "Storage_Provider")]
pub mod Provider;
#[cfg(feature = "Storage_Search")]
pub mod Search;
#[cfg(feature = "Storage_Streams")]
pub mod Streams;
pub type AppDataPaths = *mut ::core::ffi::c_void;
pub type ApplicationData = *mut ::core::ffi::c_void;
pub type ApplicationDataCompositeValue = *mut ::core::ffi::c_void;
pub type ApplicationDataContainer = *mut ::core::ffi::c_void;
pub type ApplicationDataContainerSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct ApplicationDataCreateDisposition(pub i32);
impl ApplicationDataCreateDisposition {
    pub const Always: Self = Self(0i32);
    pub const Existing: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationDataCreateDisposition {}
impl ::core::clone::Clone for ApplicationDataCreateDisposition {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct ApplicationDataLocality(pub i32);
impl ApplicationDataLocality {
    pub const Local: Self = Self(0i32);
    pub const Roaming: Self = Self(1i32);
    pub const Temporary: Self = Self(2i32);
    pub const LocalCache: Self = Self(3i32);
    pub const SharedLocal: Self = Self(4i32);
}
impl ::core::marker::Copy for ApplicationDataLocality {}
impl ::core::clone::Clone for ApplicationDataLocality {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ApplicationDataSetVersionHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct CreationCollisionOption(pub i32);
impl CreationCollisionOption {
    pub const GenerateUniqueName: Self = Self(0i32);
    pub const ReplaceExisting: Self = Self(1i32);
    pub const FailIfExists: Self = Self(2i32);
    pub const OpenIfExists: Self = Self(3i32);
}
impl ::core::marker::Copy for CreationCollisionOption {}
impl ::core::clone::Clone for CreationCollisionOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct FileAccessMode(pub i32);
impl FileAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for FileAccessMode {}
impl ::core::clone::Clone for FileAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct FileAttributes(pub u32);
impl FileAttributes {
    pub const Normal: Self = Self(0u32);
    pub const ReadOnly: Self = Self(1u32);
    pub const Directory: Self = Self(16u32);
    pub const Archive: Self = Self(32u32);
    pub const Temporary: Self = Self(256u32);
    pub const LocallyIncomplete: Self = Self(512u32);
}
impl ::core::marker::Copy for FileAttributes {}
impl ::core::clone::Clone for FileAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAppDataPaths {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cookies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Desktop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Documents: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Favorites: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub History: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InternetCache: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LocalAppData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ProgramData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RoamingAppData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppDataPathsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationData {
    pub base__: ::windows_sys::core::IInspectable,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetVersionAsync: unsafe extern "system" fn(this: *mut *mut Self, desiredversion: u32, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetVersionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearAllAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearAsync: unsafe extern "system" fn(this: *mut *mut Self, locality: ApplicationDataLocality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearAsync: usize,
    pub LocalSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RoamingSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LocalFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RoamingFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TemporaryFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DataChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataChanged: usize,
    pub SignalDataChanged: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RoamingStorageQuota: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationData2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalCacheFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationData3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetPublisherCacheFolder: unsafe extern "system" fn(this: *mut *mut Self, foldername: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClearPublisherCacheFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, foldername: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearPublisherCacheFolderAsync: usize,
    pub SharedLocalFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationDataContainer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Locality: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ApplicationDataLocality) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Values: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Values: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Containers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Containers: usize,
    pub CreateContainer: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, disposition: ApplicationDataCreateDisposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeleteContainer: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationDataStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationDataStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetForUserAsync: usize,
}
#[repr(C)]
pub struct ICachedFileManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeferUpdates: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Provider"))]
    pub CompleteUpdatesAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Provider")))]
    CompleteUpdatesAsync: usize,
}
#[repr(C)]
pub struct IDownloadsFolderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateFileAsync: unsafe extern "system" fn(this: *mut *mut Self, desiredname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, desiredname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFileWithCollisionOptionAsync: unsafe extern "system" fn(this: *mut *mut Self, desiredname: ::windows_sys::core::HSTRING, option: CreationCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFileWithCollisionOptionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFolderWithCollisionOptionAsync: unsafe extern "system" fn(this: *mut *mut Self, desiredname: ::windows_sys::core::HSTRING, option: CreationCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFolderWithCollisionOptionAsync: usize,
}
#[repr(C)]
pub struct IDownloadsFolderStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub CreateFileForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, desiredname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    CreateFileForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub CreateFolderForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, desiredname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    CreateFolderForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub CreateFileForUserWithCollisionOptionAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, desiredname: ::windows_sys::core::HSTRING, option: CreationCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    CreateFileForUserWithCollisionOptionAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub CreateFolderForUserWithCollisionOptionAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, desiredname: ::windows_sys::core::HSTRING, option: CreationCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    CreateFolderForUserWithCollisionOptionAsync: usize,
}
#[repr(C)]
pub struct IFileIOStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReadTextAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReadTextWithEncodingAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, encoding: Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReadTextWithEncodingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub WriteTextAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, contents: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteTextWithEncodingAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, contents: ::windows_sys::core::HSTRING, encoding: Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteTextWithEncodingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AppendTextAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, contents: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppendTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub AppendTextWithEncodingAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, contents: ::windows_sys::core::HSTRING, encoding: Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    AppendTextWithEncodingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadLinesAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadLinesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub ReadLinesWithEncodingAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, encoding: Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    ReadLinesWithEncodingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WriteLinesAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, lines: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WriteLinesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub WriteLinesWithEncodingAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, lines: *mut ::core::ffi::c_void, encoding: Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    WriteLinesWithEncodingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AppendLinesAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, lines: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppendLinesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub AppendLinesWithEncodingAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, lines: *mut ::core::ffi::c_void, encoding: Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    AppendLinesWithEncodingAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReadBufferAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReadBufferAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteBufferAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteBufferAsync: usize,
    #[cfg(feature = "Foundation")]
    pub WriteBytesAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteBytesAsync: usize,
}
#[repr(C)]
pub struct IKnownFoldersCameraRollStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CameraRoll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKnownFoldersPlaylistsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Playlists: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKnownFoldersSavedPicturesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SavedPictures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKnownFoldersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MusicLibrary: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PicturesLibrary: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VideosLibrary: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentsLibrary: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HomeGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemovableDevices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MediaServerDevices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKnownFoldersStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Objects3D: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppCaptures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RecordedCalls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKnownFoldersStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetFolderForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, folderid: KnownFolderId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetFolderForUserAsync: usize,
}
#[repr(C)]
pub struct IKnownFoldersStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, folderid: KnownFolderId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub RequestAccessForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, folderid: KnownFolderId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    RequestAccessForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, folderid: KnownFolderId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderAsync: usize,
}
#[repr(C)]
pub struct IPathIOStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReadTextAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReadTextWithEncodingAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, encoding: Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReadTextWithEncodingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub WriteTextAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, contents: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteTextWithEncodingAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, contents: ::windows_sys::core::HSTRING, encoding: Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteTextWithEncodingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AppendTextAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, contents: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppendTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub AppendTextWithEncodingAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, contents: ::windows_sys::core::HSTRING, encoding: Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    AppendTextWithEncodingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadLinesAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadLinesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub ReadLinesWithEncodingAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, encoding: Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    ReadLinesWithEncodingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WriteLinesAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, lines: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WriteLinesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub WriteLinesWithEncodingAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, lines: *mut ::core::ffi::c_void, encoding: Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    WriteLinesWithEncodingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AppendLinesAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, lines: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppendLinesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub AppendLinesWithEncodingAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, lines: *mut ::core::ffi::c_void, encoding: Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    AppendLinesWithEncodingAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReadBufferAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReadBufferAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteBufferAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteBufferAsync: usize,
    #[cfg(feature = "Foundation")]
    pub WriteBytesAsync: unsafe extern "system" fn(this: *mut *mut Self, absolutepath: ::windows_sys::core::HSTRING, buffer_array_size: u32, buffer: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteBytesAsync: usize,
}
#[repr(C)]
pub struct ISetVersionDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISetVersionRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DesiredVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageFile {
    pub base__: ::windows_sys::core::IInspectable,
    pub FileType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub OpenAsync: unsafe extern "system" fn(this: *mut *mut Self, accessmode: FileAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenTransactedWriteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenTransactedWriteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CopyOverloadDefaultNameAndOptions: unsafe extern "system" fn(this: *mut *mut Self, destinationfolder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyOverloadDefaultNameAndOptions: usize,
    #[cfg(feature = "Foundation")]
    pub CopyOverloadDefaultOptions: unsafe extern "system" fn(this: *mut *mut Self, destinationfolder: *mut ::core::ffi::c_void, desirednewname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyOverloadDefaultOptions: usize,
    #[cfg(feature = "Foundation")]
    pub CopyOverload: unsafe extern "system" fn(this: *mut *mut Self, destinationfolder: *mut ::core::ffi::c_void, desirednewname: ::windows_sys::core::HSTRING, option: NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyOverload: usize,
    #[cfg(feature = "Foundation")]
    pub CopyAndReplaceAsync: unsafe extern "system" fn(this: *mut *mut Self, filetoreplace: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyAndReplaceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MoveOverloadDefaultNameAndOptions: unsafe extern "system" fn(this: *mut *mut Self, destinationfolder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveOverloadDefaultNameAndOptions: usize,
    #[cfg(feature = "Foundation")]
    pub MoveOverloadDefaultOptions: unsafe extern "system" fn(this: *mut *mut Self, destinationfolder: *mut ::core::ffi::c_void, desirednewname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveOverloadDefaultOptions: usize,
    #[cfg(feature = "Foundation")]
    pub MoveOverload: unsafe extern "system" fn(this: *mut *mut Self, destinationfolder: *mut ::core::ffi::c_void, desirednewname: ::windows_sys::core::HSTRING, option: NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveOverload: usize,
    #[cfg(feature = "Foundation")]
    pub MoveAndReplaceAsync: unsafe extern "system" fn(this: *mut *mut Self, filetoreplace: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveAndReplaceAsync: usize,
}
#[repr(C)]
pub struct IStorageFile2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub OpenWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, accessmode: FileAccessMode, options: StorageOpenOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    OpenWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenTransactedWriteWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, options: StorageOpenOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenTransactedWriteWithOptionsAsync: usize,
}
#[repr(C)]
pub struct IStorageFilePropertiesWithAvailability {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageFileStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetFileFromPathAsync: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFileFromPathAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFileFromApplicationUriAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFileFromApplicationUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateStreamedFileAsync: unsafe extern "system" fn(this: *mut *mut Self, displaynamewithextension: ::windows_sys::core::HSTRING, datarequested: *mut ::core::ffi::c_void, thumbnail: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateStreamedFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReplaceWithStreamedFileAsync: unsafe extern "system" fn(this: *mut *mut Self, filetoreplace: *mut ::core::ffi::c_void, datarequested: *mut ::core::ffi::c_void, thumbnail: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReplaceWithStreamedFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateStreamedFileFromUriAsync: unsafe extern "system" fn(this: *mut *mut Self, displaynamewithextension: ::windows_sys::core::HSTRING, uri: *mut ::core::ffi::c_void, thumbnail: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateStreamedFileFromUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReplaceWithStreamedFileFromUriAsync: unsafe extern "system" fn(this: *mut *mut Self, filetoreplace: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, thumbnail: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReplaceWithStreamedFileFromUriAsync: usize,
}
#[repr(C)]
pub struct IStorageFileStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetFileFromPathForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, path: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetFileFromPathForUserAsync: usize,
}
#[repr(C)]
pub struct IStorageFolder {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateFileAsyncOverloadDefaultOptions: unsafe extern "system" fn(this: *mut *mut Self, desiredname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFileAsyncOverloadDefaultOptions: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFileAsync: unsafe extern "system" fn(this: *mut *mut Self, desiredname: ::windows_sys::core::HSTRING, options: CreationCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFolderAsyncOverloadDefaultOptions: unsafe extern "system" fn(this: *mut *mut Self, desiredname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFolderAsyncOverloadDefaultOptions: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, desiredname: ::windows_sys::core::HSTRING, options: CreationCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFileAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetItemAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncOverloadDefaultOptionsStartAndCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncOverloadDefaultOptionsStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncOverloadDefaultOptionsStartAndCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncOverloadDefaultOptionsStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsyncOverloadDefaultStartAndCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsyncOverloadDefaultStartAndCount: usize,
}
#[repr(C)]
pub struct IStorageFolder2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TryGetItemAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetItemAsync: usize,
}
#[repr(C)]
pub struct IStorageFolder3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageFolderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetFolderFromPathAsync: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderFromPathAsync: usize,
}
#[repr(C)]
pub struct IStorageFolderStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetFolderFromPathForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, path: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetFolderFromPathForUserAsync: usize,
}
#[repr(C)]
pub struct IStorageItem {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RenameAsyncOverloadDefaultOptions: unsafe extern "system" fn(this: *mut *mut Self, desiredname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenameAsyncOverloadDefaultOptions: usize,
    #[cfg(feature = "Foundation")]
    pub RenameAsync: unsafe extern "system" fn(this: *mut *mut Self, desiredname: ::windows_sys::core::HSTRING, option: NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsyncOverloadDefaultOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsyncOverloadDefaultOptions: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, option: StorageDeleteOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub GetBasicPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties")))]
    GetBasicPropertiesAsync: usize,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FileAttributes) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DateCreated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateCreated: usize,
    pub IsOfType: unsafe extern "system" fn(this: *mut *mut Self, r#type: StorageItemTypes, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageItem2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetParentAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetParentAsync: usize,
    pub IsEqual: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageItemProperties {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub GetThumbnailAsyncOverloadDefaultSizeDefaultOptions: unsafe extern "system" fn(this: *mut *mut Self, mode: FileProperties::ThumbnailMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    GetThumbnailAsyncOverloadDefaultSizeDefaultOptions: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub GetThumbnailAsyncOverloadDefaultOptions: unsafe extern "system" fn(this: *mut *mut Self, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    GetThumbnailAsyncOverloadDefaultOptions: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut *mut Self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    GetThumbnailAsync: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FolderRelativeId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_FileProperties")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    Properties: usize,
}
#[repr(C)]
pub struct IStorageItemProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions: unsafe extern "system" fn(this: *mut *mut Self, mode: FileProperties::ThumbnailMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub GetScaledImageAsThumbnailAsyncOverloadDefaultOptions: unsafe extern "system" fn(this: *mut *mut Self, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    GetScaledImageAsThumbnailAsyncOverloadDefaultOptions: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub GetScaledImageAsThumbnailAsync: unsafe extern "system" fn(this: *mut *mut Self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    GetScaledImageAsThumbnailAsync: usize,
}
#[repr(C)]
pub struct IStorageItemPropertiesWithProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Provider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageLibrary {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAddFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAddFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestRemoveFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, folder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRemoveFolderAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Folders: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Folders: usize,
    pub SaveFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DefinitionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefinitionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDefinitionChanged: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDefinitionChanged: usize,
}
#[repr(C)]
pub struct IStorageLibrary2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageLibrary3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AreFolderSuggestionsAvailableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AreFolderSuggestionsAvailableAsync: usize,
}
#[repr(C)]
pub struct IStorageLibraryChange {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StorageLibraryChangeType) -> ::windows_sys::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PreviousPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsOfType: unsafe extern "system" fn(this: *mut *mut Self, r#type: StorageItemTypes, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetStorageItemAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStorageItemAsync: usize,
}
#[repr(C)]
pub struct IStorageLibraryChangeReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AcceptChangesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcceptChangesAsync: usize,
}
#[repr(C)]
pub struct IStorageLibraryChangeReader2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetLastChangeId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageLibraryChangeTracker {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageLibraryChangeTracker2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnableWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageLibraryChangeTrackerOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub TrackChangeDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetTrackChangeDetails: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageLibraryLastChangeId {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IStorageLibraryLastChangeIdStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Unknown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageLibraryStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetLibraryAsync: unsafe extern "system" fn(this: *mut *mut Self, libraryid: KnownLibraryId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetLibraryAsync: usize,
}
#[repr(C)]
pub struct IStorageLibraryStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetLibraryForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, libraryid: KnownLibraryId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetLibraryForUserAsync: usize,
}
#[repr(C)]
pub struct IStorageProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageProvider2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub IsPropertySupportedForPartialFileAsync: unsafe extern "system" fn(this: *mut *mut Self, propertycanonicalname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsPropertySupportedForPartialFileAsync: usize,
}
#[repr(C)]
pub struct IStorageStreamTransaction {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    #[cfg(feature = "Foundation")]
    pub CommitAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommitAsync: usize,
}
#[repr(C)]
pub struct IStreamedFileDataRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub FailAndClose: unsafe extern "system" fn(this: *mut *mut Self, failuremode: StreamedFileFailureMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemAudioProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub EncodingBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemDataPaths {
    pub base__: ::windows_sys::core::IInspectable,
    pub Fonts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ProgramData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Public: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PublicDesktop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PublicDocuments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PublicDownloads: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PublicMusic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PublicPictures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PublicVideos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub System: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SystemHost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SystemX86: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SystemX64: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SystemArm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Windows: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemDataPathsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemGPSProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub LatitudeDecimal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LongitudeDecimal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemImageProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VerticalSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemMediaProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Producer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Publisher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SubTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Writer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Year: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemMusicProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlbumArtist: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AlbumTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Composer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Conductor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayArtist: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Genre: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemPhotoProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub CameraManufacturer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CameraModel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DateTaken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PeopleNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Author: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ItemNameDisplay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Keywords: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Rating: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Audio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GPS: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Media: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Music: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Photo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Video: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Image: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemVideoProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Director: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FrameHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FrameWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TotalBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserDataPaths {
    pub base__: ::windows_sys::core::IInspectable,
    pub CameraRoll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Cookies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Desktop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Documents: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Downloads: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Favorites: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub History: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InternetCache: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LocalAppData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LocalAppDataLow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Music: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Pictures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Profile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Recent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RoamingAppData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SavedPictures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Screenshots: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Templates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Videos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserDataPathsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct KnownFolderId(pub i32);
impl KnownFolderId {
    pub const AppCaptures: Self = Self(0i32);
    pub const CameraRoll: Self = Self(1i32);
    pub const DocumentsLibrary: Self = Self(2i32);
    pub const HomeGroup: Self = Self(3i32);
    pub const MediaServerDevices: Self = Self(4i32);
    pub const MusicLibrary: Self = Self(5i32);
    pub const Objects3D: Self = Self(6i32);
    pub const PicturesLibrary: Self = Self(7i32);
    pub const Playlists: Self = Self(8i32);
    pub const RecordedCalls: Self = Self(9i32);
    pub const RemovableDevices: Self = Self(10i32);
    pub const SavedPictures: Self = Self(11i32);
    pub const Screenshots: Self = Self(12i32);
    pub const VideosLibrary: Self = Self(13i32);
    pub const AllAppMods: Self = Self(14i32);
    pub const CurrentAppMods: Self = Self(15i32);
    pub const DownloadsFolder: Self = Self(16i32);
}
impl ::core::marker::Copy for KnownFolderId {}
impl ::core::clone::Clone for KnownFolderId {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct KnownFoldersAccessStatus(pub i32);
impl KnownFoldersAccessStatus {
    pub const DeniedBySystem: Self = Self(0i32);
    pub const NotDeclaredByApp: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const UserPromptRequired: Self = Self(3i32);
    pub const Allowed: Self = Self(4i32);
    pub const AllowedPerAppFolder: Self = Self(5i32);
}
impl ::core::marker::Copy for KnownFoldersAccessStatus {}
impl ::core::clone::Clone for KnownFoldersAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct KnownLibraryId(pub i32);
impl KnownLibraryId {
    pub const Music: Self = Self(0i32);
    pub const Pictures: Self = Self(1i32);
    pub const Videos: Self = Self(2i32);
    pub const Documents: Self = Self(3i32);
}
impl ::core::marker::Copy for KnownLibraryId {}
impl ::core::clone::Clone for KnownLibraryId {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct NameCollisionOption(pub i32);
impl NameCollisionOption {
    pub const GenerateUniqueName: Self = Self(0i32);
    pub const ReplaceExisting: Self = Self(1i32);
    pub const FailIfExists: Self = Self(2i32);
}
impl ::core::marker::Copy for NameCollisionOption {}
impl ::core::clone::Clone for NameCollisionOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SetVersionDeferral = *mut ::core::ffi::c_void;
pub type SetVersionRequest = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageDeleteOption(pub i32);
impl StorageDeleteOption {
    pub const Default: Self = Self(0i32);
    pub const PermanentDelete: Self = Self(1i32);
}
impl ::core::marker::Copy for StorageDeleteOption {}
impl ::core::clone::Clone for StorageDeleteOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StorageFile = *mut ::core::ffi::c_void;
pub type StorageFolder = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageItemTypes(pub u32);
impl StorageItemTypes {
    pub const None: Self = Self(0u32);
    pub const File: Self = Self(1u32);
    pub const Folder: Self = Self(2u32);
}
impl ::core::marker::Copy for StorageItemTypes {}
impl ::core::clone::Clone for StorageItemTypes {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StorageLibrary = *mut ::core::ffi::c_void;
pub type StorageLibraryChange = *mut ::core::ffi::c_void;
pub type StorageLibraryChangeReader = *mut ::core::ffi::c_void;
pub type StorageLibraryChangeTracker = *mut ::core::ffi::c_void;
pub type StorageLibraryChangeTrackerOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageLibraryChangeType(pub i32);
impl StorageLibraryChangeType {
    pub const Created: Self = Self(0i32);
    pub const Deleted: Self = Self(1i32);
    pub const MovedOrRenamed: Self = Self(2i32);
    pub const ContentsChanged: Self = Self(3i32);
    pub const MovedOutOfLibrary: Self = Self(4i32);
    pub const MovedIntoLibrary: Self = Self(5i32);
    pub const ContentsReplaced: Self = Self(6i32);
    pub const IndexingStatusChanged: Self = Self(7i32);
    pub const EncryptionChanged: Self = Self(8i32);
    pub const ChangeTrackingLost: Self = Self(9i32);
}
impl ::core::marker::Copy for StorageLibraryChangeType {}
impl ::core::clone::Clone for StorageLibraryChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StorageLibraryLastChangeId = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageOpenOptions(pub u32);
impl StorageOpenOptions {
    pub const None: Self = Self(0u32);
    pub const AllowOnlyReaders: Self = Self(1u32);
    pub const AllowReadersAndWriters: Self = Self(2u32);
}
impl ::core::marker::Copy for StorageOpenOptions {}
impl ::core::clone::Clone for StorageOpenOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StorageProvider = *mut ::core::ffi::c_void;
pub type StorageStreamTransaction = *mut ::core::ffi::c_void;
pub type StreamedFileDataRequest = *mut ::core::ffi::c_void;
pub type StreamedFileDataRequestedHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StreamedFileFailureMode(pub i32);
impl StreamedFileFailureMode {
    pub const Failed: Self = Self(0i32);
    pub const CurrentlyUnavailable: Self = Self(1i32);
    pub const Incomplete: Self = Self(2i32);
}
impl ::core::marker::Copy for StreamedFileFailureMode {}
impl ::core::clone::Clone for StreamedFileFailureMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SystemAudioProperties = *mut ::core::ffi::c_void;
pub type SystemDataPaths = *mut ::core::ffi::c_void;
pub type SystemGPSProperties = *mut ::core::ffi::c_void;
pub type SystemImageProperties = *mut ::core::ffi::c_void;
pub type SystemMediaProperties = *mut ::core::ffi::c_void;
pub type SystemMusicProperties = *mut ::core::ffi::c_void;
pub type SystemPhotoProperties = *mut ::core::ffi::c_void;
pub type SystemVideoProperties = *mut ::core::ffi::c_void;
pub type UserDataPaths = *mut ::core::ffi::c_void;
