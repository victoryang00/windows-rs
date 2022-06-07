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
impl ::windows_sys::core::Interface for IAppDataPaths {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1929500170, data2: 31138, data3: 18633, data4: [158, 192, 63, 218, 9, 47, 121, 225] };
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
impl ::windows_sys::core::Interface for IAppDataPathsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3639290622, data2: 43481, data3: 19220, data4: [185, 153, 227, 146, 19, 121, 217, 3] };
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
impl ::windows_sys::core::Interface for IApplicationData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3285872567, data2: 46916, data3: 19269, data4: [176, 184, 34, 58, 9, 56, 208, 220] };
}
#[repr(C)]
pub struct IApplicationData2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalCacheFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IApplicationData2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2657471849, data2: 2979, data3: 20018, data4: [190, 41, 176, 45, 230, 96, 118, 56] };
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
impl ::windows_sys::core::Interface for IApplicationData3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3693227252, data2: 10098, data3: 19485, data4: [170, 44, 201, 247, 67, 173, 232, 209] };
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
impl ::windows_sys::core::Interface for IApplicationDataContainer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3316579614, data2: 62567, data3: 16570, data4: [133, 102, 171, 100, 10, 68, 30, 29] };
}
#[repr(C)]
pub struct IApplicationDataStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IApplicationDataStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1444025467, data2: 59459, data3: 17891, data4: [148, 216, 6, 22, 158, 60, 142, 23] };
}
#[repr(C)]
pub struct IApplicationDataStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetForUserAsync: usize,
}
impl ::windows_sys::core::Interface for IApplicationDataStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3445645841, data2: 53065, data3: 16548, data4: [164, 124, 199, 240, 219, 186, 129, 7] };
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
impl ::windows_sys::core::Interface for ICachedFileManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2415665738, data2: 59266, data3: 18781, data4: [182, 20, 101, 76, 79, 11, 35, 112] };
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
impl ::windows_sys::core::Interface for IDownloadsFolderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 663105232, data2: 16462, data3: 18399, data4: [161, 226, 227, 115, 8, 190, 123, 55] };
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
impl ::windows_sys::core::Interface for IDownloadsFolderStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3912254909, data2: 36600, data3: 20366, data4: [141, 21, 172, 14, 38, 95, 57, 13] };
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
impl ::windows_sys::core::Interface for IFileIOStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2289308139, data2: 32596, data3: 18226, data4: [165, 240, 94, 67, 227, 184, 194, 245] };
}
#[repr(C)]
pub struct IKnownFoldersCameraRollStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CameraRoll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownFoldersCameraRollStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1561419366, data2: 10216, data3: 18735, data4: [184, 229, 47, 144, 137, 108, 212, 205] };
}
#[repr(C)]
pub struct IKnownFoldersPlaylistsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Playlists: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownFoldersPlaylistsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3671452886, data2: 12399, data3: 19818, data4: [180, 150, 70, 186, 142, 177, 6, 206] };
}
#[repr(C)]
pub struct IKnownFoldersSavedPicturesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SavedPictures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownFoldersSavedPicturesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 89953258, data2: 9533, data3: 18044, data4: [182, 202, 169, 125, 161, 233, 161, 141] };
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
impl ::windows_sys::core::Interface for IKnownFoldersStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1512731936, data2: 18434, data3: 17709, data4: [154, 217, 67, 81, 173, 167, 236, 53] };
}
#[repr(C)]
pub struct IKnownFoldersStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Objects3D: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppCaptures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RecordedCalls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownFoldersStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 424399053, data2: 53102, data3: 19719, data4: [157, 83, 233, 22, 58, 37, 54, 233] };
}
#[repr(C)]
pub struct IKnownFoldersStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetFolderForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, folderid: KnownFolderId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetFolderForUserAsync: usize,
}
impl ::windows_sys::core::Interface for IKnownFoldersStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3306767169, data2: 38722, data3: 20181, data4: [130, 61, 252, 20, 1, 20, 135, 100] };
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
impl ::windows_sys::core::Interface for IKnownFoldersStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 388163263, data2: 40953, data3: 19233, data4: [190, 213, 144, 236, 177, 58, 25, 46] };
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
impl ::windows_sys::core::Interface for IPathIOStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 254752600, data2: 36551, data3: 17281, data4: [146, 43, 143, 108, 7, 210, 136, 243] };
}
#[repr(C)]
pub struct ISetVersionDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISetVersionDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 53807266, data2: 30746, data3: 17274, data4: [176, 120, 63, 50, 186, 220, 254, 71] };
}
#[repr(C)]
pub struct ISetVersionRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DesiredVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISetVersionRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3116854171, data2: 4182, data3: 20073, data4: [131, 48, 22, 38, 25, 149, 111, 155] };
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
impl ::windows_sys::core::Interface for IStorageFile {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4198457734, data2: 16916, data3: 17036, data4: [166, 76, 20, 201, 172, 115, 21, 234] };
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
impl ::windows_sys::core::Interface for IStorageFile2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2504936399, data2: 2679, data3: 17147, data4: [183, 119, 194, 237, 88, 165, 46, 68] };
}
#[repr(C)]
pub struct IStorageFilePropertiesWithAvailability {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageFilePropertiesWithAvailability {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2949365403, data2: 22571, data3: 16691, data4: [150, 72, 228, 76, 164, 110, 228, 145] };
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
impl ::windows_sys::core::Interface for IStorageFileStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1501873936, data2: 56050, data3: 17352, data4: [139, 180, 164, 211, 234, 207, 208, 63] };
}
#[repr(C)]
pub struct IStorageFileStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetFileFromPathForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, path: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetFileFromPathForUserAsync: usize,
}
impl ::windows_sys::core::Interface for IStorageFileStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1551280001, data2: 8494, data3: 19193, data4: [143, 4, 116, 12, 174, 16, 137, 116] };
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
impl ::windows_sys::core::Interface for IStorageFolder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1926351736, data2: 46063, data3: 20341, data4: [168, 11, 111, 217, 218, 226, 148, 75] };
}
#[repr(C)]
pub struct IStorageFolder2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TryGetItemAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetItemAsync: usize,
}
impl ::windows_sys::core::Interface for IStorageFolder2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3894929593, data2: 2265, data3: 19086, data4: [160, 172, 254, 94, 211, 203, 187, 211] };
}
#[repr(C)]
pub struct IStorageFolder3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageFolder3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2673965209, data2: 48609, data3: 16676, data4: [174, 179, 176, 106, 217, 111, 152, 212] };
}
#[repr(C)]
pub struct IStorageFolderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetFolderFromPathAsync: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderFromPathAsync: usize,
}
impl ::windows_sys::core::Interface for IStorageFolderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 150153215, data2: 34261, data3: 18617, data4: [174, 233, 40, 81, 30, 51, 159, 159] };
}
#[repr(C)]
pub struct IStorageFolderStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetFolderFromPathForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, path: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetFolderFromPathForUserAsync: usize,
}
impl ::windows_sys::core::Interface for IStorageFolderStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3026546115, data2: 29138, data3: 18045, data4: [139, 41, 55, 31, 15, 98, 191, 111] };
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
impl ::windows_sys::core::Interface for IStorageItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1107798422, data2: 51759, data3: 17143, data4: [189, 232, 139, 16, 69, 122, 127, 48] };
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
impl ::windows_sys::core::Interface for IStorageItem2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1408837330, data2: 2108, data3: 17027, data4: [180, 91, 129, 192, 7, 35, 126, 68] };
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
impl ::windows_sys::core::Interface for IStorageItemProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2254849144, data2: 32809, data3: 18174, data4: [167, 137, 28, 47, 62, 47, 251, 92] };
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
impl ::windows_sys::core::Interface for IStorageItemProperties2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2391189841, data2: 1209, data3: 19410, data4: [146, 157, 254, 243, 247, 22, 33, 208] };
}
#[repr(C)]
pub struct IStorageItemPropertiesWithProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Provider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageItemPropertiesWithProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2249978779, data2: 25448, data3: 19950, data4: [180, 14, 116, 104, 74, 92, 231, 20] };
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
impl ::windows_sys::core::Interface for IStorageLibrary {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 517828867, data2: 3678, data3: 19820, data4: [181, 232, 147, 24, 152, 61, 106, 3] };
}
#[repr(C)]
pub struct IStorageLibrary2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageLibrary2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1527571272, data2: 64691, data3: 16433, data4: [175, 176, 166, 141, 123, 212, 69, 52] };
}
#[repr(C)]
pub struct IStorageLibrary3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AreFolderSuggestionsAvailableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AreFolderSuggestionsAvailableAsync: usize,
}
impl ::windows_sys::core::Interface for IStorageLibrary3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2317882001, data2: 8532, data3: 16897, data4: [129, 19, 210, 192, 92, 225, 173, 35] };
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
impl ::windows_sys::core::Interface for IStorageLibraryChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 9964323, data2: 11234, data3: 18697, data4: [170, 72, 21, 159, 82, 3, 165, 30] };
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
impl ::windows_sys::core::Interface for IStorageLibraryChangeReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4060462211, data2: 64674, data3: 16889, data4: [137, 84, 238, 46, 153, 30, 185, 111] };
}
#[repr(C)]
pub struct IStorageLibraryChangeReader2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetLastChangeId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageLibraryChangeReader2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2884929163, data2: 64460, data3: 19023, data4: [153, 158, 231, 171, 124, 100, 109, 190] };
}
#[repr(C)]
pub struct IStorageLibraryChangeTracker {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageLibraryChangeTracker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2652205846, data2: 24691, data3: 17654, data4: [150, 129, 116, 146, 209, 40, 108, 144] };
}
#[repr(C)]
pub struct IStorageLibraryChangeTracker2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnableWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageLibraryChangeTracker2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3439664187, data2: 3999, data3: 17145, data4: [143, 179, 21, 141, 130, 225, 56, 33] };
}
#[repr(C)]
pub struct IStorageLibraryChangeTrackerOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub TrackChangeDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetTrackChangeDetails: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageLibraryChangeTrackerOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3142761684, data2: 6765, data3: 22976, data4: [173, 42, 130, 58, 32, 83, 36, 131] };
}
#[repr(C)]
pub struct IStorageLibraryLastChangeId {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IStorageLibraryLastChangeId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1384219242, data2: 48097, data3: 21436, data4: [130, 202, 129, 204, 127, 3, 147, 41] };
}
#[repr(C)]
pub struct IStorageLibraryLastChangeIdStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Unknown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageLibraryLastChangeIdStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2175045928, data2: 11427, data3: 21257, data4: [176, 209, 207, 7, 136, 228, 7, 98] };
}
#[repr(C)]
pub struct IStorageLibraryStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetLibraryAsync: unsafe extern "system" fn(this: *mut *mut Self, libraryid: KnownLibraryId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetLibraryAsync: usize,
}
impl ::windows_sys::core::Interface for IStorageLibraryStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1107863259, data2: 26698, data3: 18886, data4: [158, 89, 144, 18, 30, 224, 80, 214] };
}
#[repr(C)]
pub struct IStorageLibraryStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetLibraryForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, libraryid: KnownLibraryId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetLibraryForUserAsync: usize,
}
impl ::windows_sys::core::Interface for IStorageLibraryStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4289760732, data2: 64117, data3: 18069, data4: [185, 209, 127, 129, 249, 120, 50, 227] };
}
#[repr(C)]
pub struct IStorageProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3875925716, data2: 54392, data3: 18390, data4: [186, 70, 26, 142, 190, 17, 74, 32] };
}
#[repr(C)]
pub struct IStorageProvider2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub IsPropertySupportedForPartialFileAsync: unsafe extern "system" fn(this: *mut *mut Self, propertycanonicalname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsPropertySupportedForPartialFileAsync: usize,
}
impl ::windows_sys::core::Interface for IStorageProvider2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 17635607, data2: 13316, data3: 16715, data4: [159, 215, 205, 68, 71, 46, 170, 57] };
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
impl ::windows_sys::core::Interface for IStorageStreamTransaction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4135383907, data2: 42301, data3: 19860, data4: [174, 44, 103, 35, 45, 147, 172, 221] };
}
#[repr(C)]
pub struct IStreamedFileDataRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub FailAndClose: unsafe extern "system" fn(this: *mut *mut Self, failuremode: StreamedFileFailureMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStreamedFileDataRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 376700110, data2: 55997, data3: 19792, data4: [190, 238, 24, 11, 138, 129, 145, 182] };
}
#[repr(C)]
pub struct ISystemAudioProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub EncodingBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemAudioProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1066350775, data2: 12428, data3: 18401, data4: [146, 77, 134, 69, 52, 142, 93, 183] };
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
impl ::windows_sys::core::Interface for ISystemDataPaths {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3811229552, data2: 55546, data3: 17900, data4: [169, 66, 210, 226, 111, 182, 11, 165] };
}
#[repr(C)]
pub struct ISystemDataPathsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemDataPathsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3774443472, data2: 39200, data3: 19402, data4: [179, 121, 249, 111, 223, 124, 170, 216] };
}
#[repr(C)]
pub struct ISystemGPSProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub LatitudeDecimal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LongitudeDecimal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemGPSProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3237244596, data2: 49524, data3: 18458, data4: [188, 37, 146, 25, 134, 246, 166, 243] };
}
#[repr(C)]
pub struct ISystemImageProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VerticalSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemImageProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 18558512, data2: 35641, data3: 17160, data4: [190, 161, 232, 170, 97, 228, 120, 38] };
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
impl ::windows_sys::core::Interface for ISystemMediaProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2754294550, data2: 33813, data3: 16604, data4: [140, 68, 152, 54, 29, 35, 84, 48] };
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
impl ::windows_sys::core::Interface for ISystemMusicProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3027863765, data2: 26543, data3: 19395, data4: [141, 57, 91, 137, 2, 32, 38, 161] };
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
impl ::windows_sys::core::Interface for ISystemPhotoProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1194654781, data2: 43809, data3: 17444, data4: [183, 53, 244, 53, 58, 86, 200, 252] };
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
impl ::windows_sys::core::Interface for ISystemProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2440720833, data2: 34291, data3: 19921, data4: [176, 1, 165, 11, 253, 33, 200, 210] };
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
impl ::windows_sys::core::Interface for ISystemVideoProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 541128469, data2: 26616, data3: 17186, data4: [155, 128, 79, 169, 254, 251, 131, 232] };
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
impl ::windows_sys::core::Interface for IUserDataPaths {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4190451986, data2: 43972, data3: 18175, data4: [138, 43, 220, 157, 127, 166, 229, 47] };
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
impl ::windows_sys::core::Interface for IUserDataPathsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 28483055, data2: 57442, data3: 18593, data4: [139, 12, 242, 199, 169, 202, 86, 192] };
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
