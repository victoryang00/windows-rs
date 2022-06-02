#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct CommonFileQuery(pub i32);
impl CommonFileQuery {
    pub const DefaultQuery: Self = Self(0i32);
    pub const OrderByName: Self = Self(1i32);
    pub const OrderByTitle: Self = Self(2i32);
    pub const OrderByMusicProperties: Self = Self(3i32);
    pub const OrderBySearchRank: Self = Self(4i32);
    pub const OrderByDate: Self = Self(5i32);
}
impl ::core::marker::Copy for CommonFileQuery {}
impl ::core::clone::Clone for CommonFileQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct CommonFolderQuery(pub i32);
impl CommonFolderQuery {
    pub const DefaultQuery: Self = Self(0i32);
    pub const GroupByYear: Self = Self(100i32);
    pub const GroupByMonth: Self = Self(101i32);
    pub const GroupByArtist: Self = Self(102i32);
    pub const GroupByAlbum: Self = Self(103i32);
    pub const GroupByAlbumArtist: Self = Self(104i32);
    pub const GroupByComposer: Self = Self(105i32);
    pub const GroupByGenre: Self = Self(106i32);
    pub const GroupByPublishedYear: Self = Self(107i32);
    pub const GroupByRating: Self = Self(108i32);
    pub const GroupByTag: Self = Self(109i32);
    pub const GroupByAuthor: Self = Self(110i32);
    pub const GroupByType: Self = Self(111i32);
}
impl ::core::marker::Copy for CommonFolderQuery {}
impl ::core::clone::Clone for CommonFolderQuery {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContentIndexer = *mut ::core::ffi::c_void;
pub type ContentIndexerQuery = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct DateStackOption(pub i32);
impl DateStackOption {
    pub const None: Self = Self(0i32);
    pub const Year: Self = Self(1i32);
    pub const Month: Self = Self(2i32);
}
impl ::core::marker::Copy for DateStackOption {}
impl ::core::clone::Clone for DateStackOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct FolderDepth(pub i32);
impl FolderDepth {
    pub const Shallow: Self = Self(0i32);
    pub const Deep: Self = Self(1i32);
}
impl ::core::marker::Copy for FolderDepth {}
impl ::core::clone::Clone for FolderDepth {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IContentIndexer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AddAsync: unsafe extern "system" fn(this: *mut *mut Self, indexablecontent: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAsync: unsafe extern "system" fn(this: *mut *mut Self, indexablecontent: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, contentid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DeleteMultipleAsync: unsafe extern "system" fn(this: *mut *mut Self, contentids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeleteMultipleAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RetrievePropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, contentid: ::windows_sys::core::HSTRING, propertiestoretrieve: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RetrievePropertiesAsync: usize,
    pub Revision: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentIndexerQuery {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetCountAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesRangeAsync: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, maxitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesRangeAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRangeAsync: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, maxitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRangeAsync: usize,
    pub QueryFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentIndexerQueryOperations {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateQueryWithSortOrderAndLanguage: unsafe extern "system" fn(this: *mut *mut Self, searchfilter: ::windows_sys::core::HSTRING, propertiestoretrieve: *mut ::core::ffi::c_void, sortorder: *mut ::core::ffi::c_void, searchfilterlanguage: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateQueryWithSortOrderAndLanguage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateQueryWithSortOrder: unsafe extern "system" fn(this: *mut *mut Self, searchfilter: ::windows_sys::core::HSTRING, propertiestoretrieve: *mut ::core::ffi::c_void, sortorder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateQueryWithSortOrder: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateQuery: unsafe extern "system" fn(this: *mut *mut Self, searchfilter: ::windows_sys::core::HSTRING, propertiestoretrieve: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateQuery: usize,
}
#[repr(C)]
pub struct IContentIndexerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetIndexerWithName: unsafe extern "system" fn(this: *mut *mut Self, indexname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIndexer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIndexableContent {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStream: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStream: usize,
    pub StreamContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetStreamContentType: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IQueryOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypeFilter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypeFilter: usize,
    pub FolderDepth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FolderDepth) -> ::windows_sys::core::HRESULT,
    pub SetFolderDepth: unsafe extern "system" fn(this: *mut *mut Self, value: FolderDepth) -> ::windows_sys::core::HRESULT,
    pub ApplicationSearchFilter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetApplicationSearchFilter: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserSearchFilter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetUserSearchFilter: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IndexerOption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IndexerOption) -> ::windows_sys::core::HRESULT,
    pub SetIndexerOption: unsafe extern "system" fn(this: *mut *mut Self, value: IndexerOption) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SortOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SortOrder: usize,
    pub GroupPropertyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DateStackOption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DateStackOption) -> ::windows_sys::core::HRESULT,
    pub SaveToString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LoadFromString: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_FileProperties")]
    pub SetThumbnailPrefetch: unsafe extern "system" fn(this: *mut *mut Self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    SetThumbnailPrefetch: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
    pub SetPropertyPrefetch: unsafe extern "system" fn(this: *mut *mut Self, options: super::FileProperties::PropertyPrefetchOptions, propertiestoretrieve: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_FileProperties")))]
    SetPropertyPrefetch: usize,
}
#[repr(C)]
pub struct IQueryOptionsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCommonFileQuery: unsafe extern "system" fn(this: *mut *mut Self, query: CommonFileQuery, filetypefilter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCommonFileQuery: usize,
    pub CreateCommonFolderQuery: unsafe extern "system" fn(this: *mut *mut Self, query: CommonFolderQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IQueryOptionsWithProviderFilter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub StorageProviderIdFilter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorageProviderIdFilter: usize,
}
#[repr(C)]
pub struct IStorageFileQueryResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsync: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, maxnumberofitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncDefaultStartAndCount: usize,
}
#[repr(C)]
pub struct IStorageFileQueryResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))]
    pub GetMatchingPropertiesWithRanges: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Data_Text", feature = "Foundation_Collections")))]
    GetMatchingPropertiesWithRanges: usize,
}
#[repr(C)]
pub struct IStorageFolderQueryOperations {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetIndexedStateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIndexedStateAsync: usize,
    pub CreateFileQueryOverloadDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFileQuery: unsafe extern "system" fn(this: *mut *mut Self, query: CommonFileQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFileQueryWithOptions: unsafe extern "system" fn(this: *mut *mut Self, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFolderQueryOverloadDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFolderQuery: unsafe extern "system" fn(this: *mut *mut Self, query: CommonFolderQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFolderQueryWithOptions: unsafe extern "system" fn(this: *mut *mut Self, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateItemQuery: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateItemQueryWithOptions: unsafe extern "system" fn(this: *mut *mut Self, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsync: unsafe extern "system" fn(this: *mut *mut Self, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncOverloadDefaultStartAndCount: unsafe extern "system" fn(this: *mut *mut Self, query: CommonFileQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncOverloadDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsync: unsafe extern "system" fn(this: *mut *mut Self, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncOverloadDefaultStartAndCount: unsafe extern "system" fn(this: *mut *mut Self, query: CommonFolderQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncOverloadDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    pub AreQueryOptionsSupported: unsafe extern "system" fn(this: *mut *mut Self, queryoptions: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCommonFolderQuerySupported: unsafe extern "system" fn(this: *mut *mut Self, query: CommonFolderQuery, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCommonFileQuerySupported: unsafe extern "system" fn(this: *mut *mut Self, query: CommonFileQuery, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageFolderQueryResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsync: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, maxnumberofitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncDefaultStartAndCount: usize,
}
#[repr(C)]
pub struct IStorageItemQueryResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, maxnumberofitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsyncDefaultStartAndCount: usize,
}
#[repr(C)]
pub struct IStorageLibraryChangeTrackerTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Folder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageLibraryContentChangedTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Folder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateModifiedSinceQuery: unsafe extern "system" fn(this: *mut *mut Self, lastquerytime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateModifiedSinceQuery: usize,
}
#[repr(C)]
pub struct IStorageQueryResultBase {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetItemCountAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemCountAsync: usize,
    pub Folder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContentsChanged: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContentsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub OptionsChanged: unsafe extern "system" fn(this: *mut *mut Self, changedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OptionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOptionsChanged: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOptionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub FindStartIndexAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindStartIndexAsync: usize,
    pub GetCurrentQueryOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ApplyNewQueryOptions: unsafe extern "system" fn(this: *mut *mut Self, newqueryoptions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IValueAndLanguage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type IndexableContent = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct IndexedState(pub i32);
impl IndexedState {
    pub const Unknown: Self = Self(0i32);
    pub const NotIndexed: Self = Self(1i32);
    pub const PartiallyIndexed: Self = Self(2i32);
    pub const FullyIndexed: Self = Self(3i32);
}
impl ::core::marker::Copy for IndexedState {}
impl ::core::clone::Clone for IndexedState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct IndexerOption(pub i32);
impl IndexerOption {
    pub const UseIndexerWhenAvailable: Self = Self(0i32);
    pub const OnlyUseIndexer: Self = Self(1i32);
    pub const DoNotUseIndexer: Self = Self(2i32);
    pub const OnlyUseIndexerAndOptimizeForIndexedProperties: Self = Self(3i32);
}
impl ::core::marker::Copy for IndexerOption {}
impl ::core::clone::Clone for IndexerOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub type QueryOptions = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Storage_Search\"`*"]
pub struct SortEntry {
    pub PropertyName: ::windows_sys::core::HSTRING,
    pub AscendingOrder: bool,
}
impl ::core::marker::Copy for SortEntry {}
impl ::core::clone::Clone for SortEntry {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SortEntryVector = *mut ::core::ffi::c_void;
pub type StorageFileQueryResult = *mut ::core::ffi::c_void;
pub type StorageFolderQueryResult = *mut ::core::ffi::c_void;
pub type StorageItemQueryResult = *mut ::core::ffi::c_void;
pub type StorageLibraryChangeTrackerTriggerDetails = *mut ::core::ffi::c_void;
pub type StorageLibraryContentChangedTriggerDetails = *mut ::core::ffi::c_void;
pub type ValueAndLanguage = *mut ::core::ffi::c_void;
